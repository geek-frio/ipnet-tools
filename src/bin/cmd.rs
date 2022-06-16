use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::time::Instant;

use anyhow::Error as AnyError;
use clap::Parser;
use fastping_rs::PingResult::*;
use fastping_rs::Pinger;
use iprange::is_ip_fmt;
use iprange::DisplayIp;
use iprange::IpAnalyze;
use iprange::IP;
use tracing::error;
use tracing::info;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    config: String,

    #[clap(short, long)]
    output: String,

    #[clap(short, long, default_value = "300")]
    rtt: u64,
}

fn try_to_start_tcp_conn(ip: &str, rtt: u64) -> (bool, u128) {
    let conn_str = format!("{}:443", ip);
    let current = Instant::now();
    let sock_addr = conn_str
        .to_socket_addrs()
        .unwrap()
        .collect::<Vec<SocketAddr>>();
    let res = TcpStream::connect_timeout(sock_addr.get(0).unwrap(), Duration::from_secs(1));
    match res {
        Ok(s) => {
            println!(
                "Host 对应ip: {} 连接正常, 连接耗时:{}ms",
                ip,
                current.elapsed().as_millis()
            );
            drop(s);
            let elapsed = current.elapsed().as_millis();
            if current.elapsed().as_millis() > rtt as u128 {
                (false, elapsed)
            } else {
                (true, elapsed)
            }
        }
        Err(e) => {
            println!("Have met error in connecting..., e:{:?}", e);
            let elapsed = current.elapsed().as_millis();
            (false, elapsed)
        }
    }
}

fn main() -> Result<(), AnyError> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    if args.config.is_empty() {
        return Err(AnyError::msg("需要提供配置文件地址"));
    }

    let ip_ranges =
        std::fs::read_to_string(args.config.as_str()).expect("读取配置文件中的ip range列表失败");
    let ip_ranges = ip_ranges.split('\n');
    let ip_ranges = ip_ranges
        .into_iter()
        .filter(|ip| is_ip_fmt(*ip))
        .map(|ip| ip.to_string())
        .collect::<Vec<String>>();

    // 每个Range只取100个ip
    let used_ips: Vec<String> = ip_ranges
        .into_iter()
        .map(|ip_range| {
            let ip: IP = ip_range.as_str().try_into().unwrap();
            let (min, max) = ip.compute_ip_range();

            let mut ips = Vec::new();
            let mut step = (max - min) / 100;
            if step == 0 {
                step = 1;
            }
            for ip in (min + 1..max).into_iter().step_by(step as usize) {
                ips.push(ip.display_ip());
            }
            ips
        })
        .flatten()
        .collect();
    let (send, recv) = std::sync::mpsc::channel::<String>();

    let output_file = args.output.as_str().to_string();
    std::thread::spawn(move || {
        let output = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(output_file)
            .expect("创建输出file失败");
        let mut buf_write = BufWriter::new(output);
        while let Ok(ip) = recv.recv() {
            buf_write
                .write_fmt(format_args!("{}\n", ip))
                .expect("写入新的ip失败");
            buf_write.flush().unwrap();
        }
    });

    let timeout = args.rtt;
    for ip_range in used_ips {
        let (pinger, results) = Pinger::new(Some(timeout), Some(16)).unwrap();
        let ip: IP = ip_range.as_str().try_into()?;
        let (min, max) = ip.compute_ip_range();
        info!(
            "最小ip range min:{}, 最大ip range max:{}",
            min.display_ip(),
            max.display_ip()
        );

        for ip in (min + 1..max).into_iter() {
            pinger.add_ipaddr(ip.display_ip().as_str());
        }
        pinger.ping_once();
        loop {
            match results.recv() {
                Ok(result) => match result {
                    Idle { addr } => {
                        info!("无用的ip:{}", addr);
                        continue;
                    }
                    Receive { addr, rtt } => {
                        info!("可能有效的ip地址 {} in {:?}.", addr, rtt);
                        if rtt > Duration::from_millis(timeout) {
                            info!("ip地址延迟过大, 跳过尝试");
                        }
                        info!("尝试对应ip建立tcp连接, 只接收延迟小于{}ms的ip", timeout);
                        if addr.is_ipv4() {
                            info!("ip is:{}", format!("{}", addr));
                            let (stat, elapse) =
                                try_to_start_tcp_conn(format!("{}", addr).as_str(), timeout);
                            if stat && elapse < timeout as u128 {
                                let _ = send.send(addr.to_string()).expect("发送失败");
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("Receive error:{}", e)
                }
            }
        }
    }
    Ok(())
}

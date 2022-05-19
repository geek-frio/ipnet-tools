use anyhow::Error as AnyError;
use fastping_rs::PingResult::*;
use fastping_rs::Pinger;
use iprange::DisplayIp;
use iprange::IpAnalyze;
use iprange::IP;
fn main() -> Result<(), AnyError> {
    let (pinger, results) = Pinger::new(Some(500), Some(16)).unwrap();

    let ip: IP = "103.21.244.0/22".try_into()?;
    let (min, max) = ip.compute_ip_range();
    println!("min:{}, max:{}", min.display_ip(), max.display_ip());

    for (i, ip) in (min + 1..max).into_iter().enumerate() {
        println!("Number {:3} ip: {}", i + 1, ip.display_ip());
        pinger.add_ipaddr(ip.display_ip().as_str());
    }
    pinger.ping_once();
    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    println!("Idle Address {}.", addr);
                }
                Receive { addr, rtt } => {
                    println!("Receive from Address {} in {:?}.", addr, rtt);
                }
            },
            Err(e) => {
                println!("Receive error:{}", e)
            }
        }
    }
}

use anyhow::Error as AnyError;

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + (array[3] as u32)
}

struct IP {
    bytes: [u8; 4],
    subnet: usize,
}

impl<'a> TryFrom<&'a str> for IP {
    type Error = AnyError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split("/").collect();
        if split.len() == 2 {
            let nums: Vec<u8> = split[0]
                .split(".")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|a| a.parse::<u8>().ok())
                .filter_map(|a| a)
                .collect();
            if nums.len() == 4 {
                let s = nums.as_slice();
                let bytes: Result<[u8; 4], _> = s.try_into();
                match bytes {
                    Err(_) => return Err(AnyError::msg("IP format is not correct")),
                    Ok(bytes) => {
                        let subnet = split[1].parse::<usize>()?;
                        if subnet >= 32 || subnet == 0 {
                            return Err(AnyError::msg("Incorrect subnet value"));
                        }
                        return Ok(IP { bytes, subnet });
                    }
                }
            }
        }
        Err(AnyError::msg(
            "Not correct format, format should be ip/subnet(eg: 192.168.32.0/24)",
        ))
    }
}

trait IpAnalyze {
    fn compute_ip_range(&self) -> (u32, u32);
}

trait IntoU32 {
    fn into_u32(&self) -> u32;
}

impl IntoU32 for &[u8; 4] {
    fn into_u32(&self) -> u32 {
        ((self[0] as u32) << 24)
            + ((self[1] as u32) << 16)
            + ((self[2] as u32) << 8)
            + (self[3] as u32)
    }
}

impl IpAnalyze for IP {
    fn compute_ip_range(&self) -> (u32, u32) {
        let i = (1u32 << (32 - self.subnet)) - 1;
        println!(
            "Subnet number binary representation: {:32}",
            format!("{:b}", i)
        );
        let min: u32 = (&self.bytes).into_u32();
        (min & (!i), i ^ min)
    }
}

trait DisplayIp {
    fn display_ip(&self) -> String;
}

impl DisplayIp for u32 {
    fn display_ip(&self) -> String {
        let bytes = self.to_be_bytes();
        let mut ip = String::new();
        for (i, b) in bytes.into_iter().enumerate() {
            ip.push_str(format!("{}", b).as_str());
            if i < 3 {
                ip.push_str(".")
            }
        }
        ip
    }
}

fn main() -> Result<(), AnyError> {
    let ip: IP = "192.168.32.1/24".try_into()?;
    let (min, max) = ip.compute_ip_range();
    println!("min:{}, max:{}", min.display_ip(), max.display_ip());

    for (i, ip) in (min + 1..max).into_iter().enumerate() {
        println!("Number {:3} ip: {}", i + 1, ip.display_ip());
    }
    Ok(())
}

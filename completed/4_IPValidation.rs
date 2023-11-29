/*
Write an algorithm that will identify valid IPv4 addresses in dot-decimal format. 
IPs should be considered valid if they consist of four octets, with values between 0 and 255, inclusive.
*/
fn main() {
}

// Simple
use std::net::Ipv4Addr;
fn ip2(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

// A better version!!!!
fn ip1(ip: &str) -> bool {
    let segments: Vec<&str> = ip.split(".").collect();
    segments
        .iter()
        .all(|s| (!s.starts_with("0") || s.len() == 1) && s.parse::<u8>().is_ok())
        && segments.len() == 4
}

// My version
pub fn is_valid_ip(ip: &str) -> bool {

    print!("{} ",ip.clone());
    
    let bytes = ip.split('.');
    
    if bytes.clone().count() != 4 {return false}
    
    let bad: Vec<bool> = bytes.clone().map(|b| 
        match (b.clone().chars().next().is_none() || b.clone().chars().next().unwrap() == '0') && b.len() > 1 {
        true => true,
        false => false
    }).collect();

    if bad[0] || bad[1] ||bad[2] || bad[3] {
        println!("false");
        return false;
    }

    let res: Vec<bool> = bytes.map(|b| match b.parse::<u8>() {
        Ok(_) => true,
        Err(_) => false
    }).collect();

    let r = res[0] && res[1] && res[2] && res[3];
    println!("{}",r.clone());
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_test() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));
        
        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
    }
}


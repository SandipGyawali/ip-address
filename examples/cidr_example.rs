use std::net::{IpAddr, Ipv4Addr};
use ip_address::cidr::Cidr;

fn main() {
    println!("------Cidr Example------");
    println!("--- IPv4 Example ---");
    let cidr_v4 = Cidr::new("192.168.1.0/24").expect("Invalid CIDR");
    println!("CIDR: 192.168.1.0/24");
    println!("Version: {}", cidr_v4.version());
    println!("Network: {}", cidr_v4.network());
    println!("Broadcast: {}", cidr_v4.broadcast());
    println!("Netmask: {}", cidr_v4.netmask());

    let ip_inside = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
    let ip_outside = IpAddr::V4(Ipv4Addr::new(192, 168, 2, 5));
    println!("Contains 192.168.1.10? {}", cidr_v4.contains(ip_inside));
    println!("Contains 192.168.2.5? {}", cidr_v4.contains(ip_outside));

    println!("\n--- IPv6 Example ---");
    let cidr_v6 = Cidr::new("2001:db8::/32").expect("Invalid CIDR");
    println!("CIDR: 2001:db8::/32");
    println!("Version: {}", cidr_v6.version());
    println!("Network: {}", cidr_v6.network());
    println!("Broadcast: {}", cidr_v6.broadcast());
    println!("Netmask: {}", cidr_v6.netmask());

    let ip_v6_inside: IpAddr = "2001:db8::1".parse().unwrap();
    let ip_v6_outside: IpAddr = "2001:db9::1".parse().unwrap();
    println!("Contains 2001:db8::1? {}", cidr_v6.contains(ip_v6_inside));
    println!("Contains 2001:db9::1? {}", cidr_v6.contains(ip_v6_outside));
}

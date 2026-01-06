mod modules;
mod constants;

use std::net::IpAddr;

use modules::ipv4::Ipv4;

use crate::modules::{cidr::Cidr, ipv6::Ipv6};

fn main() {
    let input = "192.168.1.1";
    
    // 1. Test constructor (new)
    let ip = Ipv4::new(input).expect("Invalid IP provided");
    println!("--- IP Info ---");
    
    // 2. Test get_address
    println!("String format: {}", ip.get_address()); 

    // 3. Test get_version
    println!("IP Version: {}", Ipv4::get_version());

    // 4. Test equals
    let is_equal = ip.equals("192.168.1.1");
    println!("Is equal to '192.168.1.1'?: {}", is_equal);

    // 5. Test is_private
    println!("Is private range?: {}", ip.is_private());

    // 6. Test to_bytes (Instance method)
    let bytes = ip.to_bytes();
    println!("Byte array: {:?}", bytes); // {:?} helps print arrays

    // 7. Test from_bytes (Static/Associated method)
    let new_ip = Ipv4::from_bytes([10, 0, 0, 255]);
    println!("Created from bytes [10, 0, 0, 255]: {}", new_ip.get_address());
    println!("Is the new IP private?: {}", new_ip.is_private());

    // 8. Test is_valid (Static/Associated method)
    let check_valid = Ipv4::is_valid("999.999.999.999");
    println!("Is '999.999.999.999' valid?: {}", check_valid);


    println!("Ip to arpa is: {}", ip.to_arpa());

    // Get the next IP
    let next_ip = ip.next(1);
    println!("Next:     {}", next_ip.get_address()); // 192.168.2.0

    // Get an IP 10 steps back
    let prev_ip = ip.previous(10);
    println!("Prev 10:  {}", prev_ip.get_address()); // 192.168.1.245


    let cidr = Cidr::new("192.168.1.0/24").unwrap();
    println!("Network: {}", cidr.network());   // 192.168.1.0
    println!("Netmask: {}", cidr.netmask());   // 255.255.255.0
    
    let ip: IpAddr = "192.168.1.50".parse().unwrap();
    println!("Contains: {}", cidr.contains(ip)); // true


    let ip6 = Ipv6::new("2001:db8::1").expect("Failed to parse IPv6");
    
    println!("Address:    {}", ip6.get_address());
    println!("Version:    {}", Ipv6::get_version());
    println!("Is Global:  {}", ip6.is_global_unicast());
    println!("ARPA:       {}", ip6.to_arpa());
    println!("Next IP:    {}", ip6.next(1).get_address());

    // Checking equality directly
    let is_same = ip6.equals("2001:db8::1");
    println!("Is Equals:  {}", is_same);

    println!("\n--- Special IPv6 Checks ---");
    
    let loopback = Ipv6::new("::1").expect("Invalid Loopback");
    println!("::1 Loopback: {}", loopback.is_loopback());

    let ula = Ipv6::new("fd00::1").expect("Invalid ULA");
}
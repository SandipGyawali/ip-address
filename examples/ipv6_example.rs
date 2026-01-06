use ip_address::ipv6::Ipv6;


fn main() {

    println!("------Ipv6 Example Running------");
    // Create a new IPv6 address
    let ip = Ipv6::new("2001:0db8:85a3:0000:0000:8a2e:0370:7334")
        .expect("Invalid IPv6 address");
    println!("IPv6 Address: {}", ip.get_address());

    // Check if an IPv6 address is valid
    let valid = Ipv6::is_valid("fe80::1");
    println!("Is fe80::1 valid? {}", valid);

    // Compare two IPv6 addresses
    let other_ip = "2001:db8:85a3::8a2e:370:7334"; // compressed format
    println!("IP equals {}? {}", other_ip, ip.equals(other_ip));

    // Check IP properties
    println!("Is private (ULA)? {}", ip.is_private());
    println!("Is loopback? {}", ip.is_loopback());
    println!("Is multicast? {}", ip.is_multicast());
    println!("Is link-local? {}", ip.is_link_local());
    println!("Is global unicast? {}", ip.is_global_unicast());
    println!("Is unspecified? {}", ip.is_unspecified());

    // Convert IPv6 to bytes
    let bytes = ip.to_bytes();
    println!("IPv6 as bytes: {:?}", bytes);

    // Create IPv6 from bytes
    let ip_from_bytes = Ipv6::from_bytes(bytes);
    println!("IPv6 from bytes: {}", ip_from_bytes.get_address());

    // Next and previous IPv6
    let next_ip = ip.next(5);
    println!("Next IP (+5): {}", next_ip.get_address());

    let prev_ip = ip.previous(5);
    println!("Previous IP (-5): {}", prev_ip.get_address());

    // Reverse DNS (arpa)
    println!("Reverse DNS: {}", ip.to_arpa());

    // IP version
    println!("IP Version: {}", Ipv6::get_version());
}

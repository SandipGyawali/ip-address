use ip_address::ipv4::Ipv4;


fn main() {
    println!("------Ipv4 Example Running------");
    // Create a new IPV4 address.
    let ip = Ipv4::new("192.168.1.10").expect("Invalid IP");
    println!("IP Address: {}", ip.get_address());

    // Check if IP is valid or not.
    let valid = Ipv4::is_valid("10.0.0.1");
    println!("Is 10.0.0.1 valid? {}", valid);

    // Compare two IPs
    let other_ip = "192.168.1.10";
    println!("IP equals {}? {}", other_ip, ip.equals(other_ip));

    // Check IP type
    println!("Is private? {}", ip.is_private());
    println!("Is loopback? {}", ip.is_loopback());
    println!("Is broadcast? {}", ip.is_broadcast());
    println!("Is global unicast? {}", ip.is_global_unicast());
    println!("Is unspecified? {}", ip.is_unspecified());

    // Convert IP to bytes
    let bytes = ip.to_bytes();
    println!("IP as bytes: {:?}", bytes);

    // Create IP from bytes
    let ip_from_bytes = Ipv4::from_bytes([192, 168, 1, 10]);
    println!("IP from bytes: {}", ip_from_bytes.get_address());

    // Next and previous IP
    let next_ip = ip.next(5);
    println!("Next IP (+5): {}", next_ip.get_address());

    let prev_ip = ip.previous(5);
    println!("Previous IP (-5): {}", prev_ip.get_address());

    // Convert to reverse DNS (arpa)
    println!("Reverse DNS: {}", ip.to_arpa());

    // IP version
    println!("IP Version: {}", Ipv4::get_version());
}

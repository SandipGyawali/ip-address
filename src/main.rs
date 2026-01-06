mod modules;
mod constants;

use modules::ipv4::Ipv4;

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
}
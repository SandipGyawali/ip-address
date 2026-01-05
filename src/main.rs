// targets the mod.rs inside the modules
mod modules;
mod constants;

// targets the impl of the Ipv4 
// for the instance.
use modules::ipv4::Ipv4;


fn main() {
  let input = "192.168.1.1";
  
  let ip: Ipv4 = Ipv4::new(input).expect("Invalid IP provided");

  println!("{} is the ip address", ip.get_address().to_string()); 

  println!("{}", ip.equals(input));
}
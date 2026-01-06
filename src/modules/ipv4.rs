use crate::constants::patterns;
use std::{ net::Ipv4Addr, str::FromStr };

pub struct Ipv4 {
  address: u32,
}

impl Ipv4 { 
  pub fn new(addr: &str) -> Result<Self, String> {
      if !Self::is_valid(addr) {
        return Err(format!("Invalid IP: {}", addr));
      }
      let val = Self::ip_to_u32(addr); 
      Ok(Self { address: val })
    }

  /**
   * checks if the provided ipv4 address is 
   * valid or not.
   */
  pub fn is_valid(address: &str) -> bool {
    let regex = patterns::get_ipv4_regex();
    regex.is_match(address) 
  }

  /**
   * return the address in the string parsing it 
   * from unsigned32-bit address
   */
  pub fn get_address(&self) -> String {
    let ip  = Ipv4Addr::from(self.address);
    ip.to_string()
  }

  /**
   * parses the given ip address to unsigned32bit address
   */
  pub fn ip_to_u32(addr: &str) -> u32 {
    let ip = Ipv4Addr::from_str(addr).expect("Invalid IP format");
    u32::from(ip)
  }

  /**
   * get the version of the ip.
   */
  pub fn get_version() -> i32 {
    4
  }

  /**
   * checks the current ip present with the 
   * ip passed in the parameters
   */
  pub fn equals(&self, other_ip: &str) -> bool {
    match Ipv4Addr::from_str(other_ip) {
      Ok(parsed) => self.address == u32::from(parsed),
      Err(_) => false,
    }
  }

  /**
   * checks if the provided ip address is private of not.
   */
  pub fn is_private(&self) -> bool {
    let addr = self.address;
    // 10.0.0.0/8 (167772160 to 184549375)
    if addr >= 167772160 && addr <= 184549375 { return true; }
    // 172.16.0.0/12
    if addr >= 2886729728 && addr <= 2887778303 { return true; }
    // 192.168.0.0/16
    if addr >= 3232235520 && addr <= 3232301055 { return true; }
    false
  }

  pub fn from_bytes(bytes: [u8; 4]) -> Self {
    // input format -> [129, 168, 1, 1] u8 -> [0, 255]
    let value = u32::from_be_bytes(bytes);
    
    Self {
      address: value
    }
  }

  pub fn to_bytes(&self) -> [u8; 4] {
    self.address.to_be_bytes()
  }
}
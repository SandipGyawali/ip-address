use crate::constants::patterns;
use std::{ net::Ipv6Addr, str::FromStr };

pub struct Ipv6 {
  address: u128,
}

impl Ipv6 { 
  pub fn new(addr: &str) -> Result<Self, String> {
      if !Self::is_valid(addr) {
        return Err(format!("Invalid IPv6: {}", addr));
      }
      let val = Self::ip_to_u128(addr); 
      Ok(Self { address: val })
    }

  /**
   * Checks if the provided IPv6 address is valid.
   */
  pub fn is_valid(address: &str) -> bool {
    let regex = patterns::get_ipv6_regex();
    regex.is_match(address) 
  }

  /**
   * Returns the address as a string parsed from the 128-bit value.
   */
  pub fn get_address(&self) -> String {
    let ip = Ipv6Addr::from(self.address);
    ip.to_string()
  }

  /**
   * Parses the given IPv6 string to a 128-bit unsigned integer.
   */
  pub fn ip_to_u128(addr: &str) -> u128 {
    let ip = Ipv6Addr::from_str(addr).expect("Invalid IPv6 format");
    u128::from(ip)
  }

  /**
   * Gets the version of the IP.
   */
  pub fn get_version() -> i32 {
    6
  }

  /**
   * Checks if the current IP matches the provided string.
   */
  pub fn equals(&self, other_ip: &str) -> bool {
    match Ipv6Addr::from_str(other_ip) {
      Ok(parsed) => self.address == u128::from(parsed),
      Err(_) => false,
    }
  }

  /**
   * Checks if the provided IPv6 address is unique local (ULAs).
   * Range: fc00::/7
   */
  pub fn is_private(&self) -> bool {
    (self.address >> 121) == 0b1111110
  }

  pub fn from_bytes(bytes: [u8; 16]) -> Self {
    let value = u128::from_be_bytes(bytes);
    Self { address: value }
  }

  pub fn to_bytes(&self) -> [u8; 16] {
    self.address.to_be_bytes()
  }

  /**
   * Returns the Reverse DNS lookup string.
   * Format: b.a.9.8.7.6.5.4.3.2.1.0.0.0.0.0.0.0.0.0.0.0.0.0.8.b.d.0.1.0.0.2.ip6.arpa
   */
  pub fn to_arpa(&self) -> String {
    let hex_string = format!("{:032x}", self.address);
    let reversed_hex: String = hex_string
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| format!("{}.", c))
        .collect();
    
    format!("{}ip6.arpa", reversed_hex)
  }

  pub fn next(&self, count: u128) -> Self {
    let new_val = self.address.wrapping_add(count);
    Self { address: new_val }
  }

  pub fn previous(&self, count: u128) -> Self {
    let new_val = self.address.wrapping_sub(count);
    Self { address: new_val }
  }

  pub fn is_unspecified(&self) -> bool {
    self.address == 0
  }

  pub fn is_loopback(&self) -> bool {
    self.address == 1
  }

  /**
   * Checks for Multicast addresses (ff00::/8)
   */
  pub fn is_multicast(&self) -> bool {
    (self.address >> 120) == 0xff
  }

  /**
   * Checks for Link-Local addresses (fe80::/10)
   */
  pub fn is_link_local(&self) -> bool {
    (self.address >> 118) == 0b1111111010
  }

  pub fn is_global_unicast(&self) -> bool {
    !self.is_unspecified()
      && !self.is_loopback()
      && !self.is_multicast()
      && !self.is_link_local()
  }
}
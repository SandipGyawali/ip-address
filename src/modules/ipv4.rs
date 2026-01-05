use crate::constants::patterns;

pub struct Ipv4 {
  address: String,
}

impl Ipv4 { 
  pub fn new(addr: &str) -> Result<Self, String> {
    if Self::is_valid(addr) {
      Ok(Self {
        address: addr.to_string(),
      })
    } else {
      Err(format!("'{}' is not a valid IPv4 address", addr))
    }
  }

  /**
   * checks if the provided ipv4 address is 
   * valid or not.
   */
  pub fn is_valid(address: &str) -> bool {
    let regex = patterns::get_ipv4_regex();
    regex.is_match(address) 
  }

  pub fn get_address(&self) -> String {
    return self.address.clone();
  }

  pub fn get_version() -> i32 {
    return 4;
  }

  pub fn equals(&self, other_ip: &str) -> bool {
    self.address == other_ip
  }
}
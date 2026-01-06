use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq, Eq)]
pub enum CidrError {
    InvalidFormat,
    InvalidAddress,
    InvalidPrefix,
}

pub struct Cidr {
    address: IpAddr,
    prefix: u8,
}

impl Cidr {
  /// Creates a new CIDR from a string like "192.168.1.0/24"
  pub fn new(cidr: &str) -> Result<Self, CidrError> {
      let parts: Vec<&str> = cidr.split('/').collect();
      let address: IpAddr = parts[0].parse().map_err(|_| CidrError::InvalidAddress)?;
      
      let max_bits = match address {
          IpAddr::V4(_) => 32,
          IpAddr::V6(_) => 128,
      };  
      let prefix = if parts.len() == 2 {
          parts[1].parse::<u8>().map_err(|_| CidrError::InvalidPrefix)?
      } else {
          max_bits
      };  
      if prefix > max_bits {
          return Err(CidrError::InvalidPrefix);
      }  
      Ok(Self { address, prefix })
  }

  pub fn version(&self) -> u8 {
    match self.address {
        IpAddr::V4(_) => 4,
        IpAddr::V6(_) => 6,
    }
  }

  pub fn netmask(&self) -> IpAddr {
    match self.address {
      IpAddr::V4(_) => {
        let mask = !0u32 << (32 - self.prefix);
        IpAddr::V4(Ipv4Addr::from(mask))
      }
      IpAddr::V6(_) => {
        let mask = !0u128 << (128 - self.prefix);
        IpAddr::V6(Ipv6Addr::from(mask))
      }
    }
  }

  pub fn broadcast(&self) -> IpAddr {
    match self.address {
      IpAddr::V4(addr) => {
        let hostmask = !(!0u32 << (32 - self.prefix));
        let broadcast = u32::from(self.network_u32()) | hostmask;
        IpAddr::V4(Ipv4Addr::from(broadcast))
      }
      IpAddr::V6(addr) => {
        let hostmask = !(!0u128 << (128 - self.prefix));
        let broadcast = u128::from(self.network_u128()) | hostmask;
        IpAddr::V6(Ipv6Addr::from(broadcast))
      }
    }
  }

  pub fn network(&self) -> IpAddr {
    match self.address {
      IpAddr::V4(addr) => {
          let mask = !0u32 << (32 - self.prefix);
          let network = u32::from(addr) & mask;
          IpAddr::V4(Ipv4Addr::from(network))
      }
      IpAddr::V6(addr) => {
          let mask = !0u128 << (128 - self.prefix);
          let network = u128::from(addr) & mask;
          IpAddr::V6(Ipv6Addr::from(network))
      }
    }
  }

  pub fn contains(&self, ip: IpAddr) -> bool {
    if self.version() != (if ip.is_ipv4() { 4 } else { 6 }) {
        return false;
    }
  
    match (self.address, ip) {
      (IpAddr::V4(_), IpAddr::V4(target)) => {
        let start = self.network_u32();
        let end = u32::from(match self.broadcast() { IpAddr::V4(a) => a, _ => unreachable!() });
        let t = u32::from(target);
        t >= start && t <= end
      },
      (IpAddr::V6(_), IpAddr::V6(target)) => {
        let start = self.network_u128();
        let end = u128::from(match self.broadcast() { IpAddr::V6(a) => a, _ => unreachable!() });
        let t = u128::from(target);
        t >= start && t <= end
      },
      _ => false
    }
  }

  fn network_u32(&self) -> u32 {
        if let IpAddr::V4(addr) = self.network() { u32::from(addr) } else { 0 }
  }

  fn network_u128(&self) -> u128 {
      if let IpAddr::V6(addr) = self.network() { u128::from(addr) } else { 0 }
  }
}
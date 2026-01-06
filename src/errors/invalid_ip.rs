use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpAddressInvalidError {
    pub input: String,
}

impl fmt::Display for IpAddressInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid IP address: {}", self.input)
    }
}

impl std::error::Error for IpAddressInvalidError {}

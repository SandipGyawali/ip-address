use regex::Regex;

pub const IPV4_ADDRESS_PATTERN: &str = r"(\d{1,3}\.){3}\d{1,3}";

pub fn get_ipv4_regex() -> Regex {
  let pattern = format!("^{}", IPV4_ADDRESS_PATTERN);
  Regex::new(&pattern).expect("Invalid Regex Pattern")
}
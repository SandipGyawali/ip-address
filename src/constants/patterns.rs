use regex::Regex;
use std::sync::OnceLock;

pub const IPV4_ADDRESS_PATTERN: &str = r"(\d{1,3}\.){3}\d{1,3}";

static IPV6_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_ipv4_regex() -> Regex {
  let pattern = format!("^{}", IPV4_ADDRESS_PATTERN);
  Regex::new(&pattern).expect("Invalid Regex Pattern")
}

pub fn get_ipv6_regex() -> &'static Regex {
  IPV6_REGEX.get_or_init(|| {
      let pattern = r"(?i)^(([0-9a-f]{1,4}:){7,7}[0-9a-f]{1,4}|([0-9a-f]{1,4}:){1,7}:|([0-9a-f]{1,4}:){1,6}:[0-9a-f]{1,4}|([0-9a-f]{1,4}:){1,5}(:[0-9a-f]{1,4}){1,2}|([0-9a-f]{1,4}:){1,4}(:[0-9a-f]{1,4}){1,3}|([0-9a-f]{1,4}:){1,3}(:[0-9a-f]{1,4}){1,4}|([0-9a-f]{1,4}:){1,2}(:[0-9a-f]{1,4}){1,5}|[0-9a-f]{1,4}:((:[0-9a-f]{1,4}){1,6})|:((:[0-9a-f]{1,4}){1,7}|:)|fe80:(:[0-9a-f]{0,4}){0,4}%[0-9a-z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]{1,2})\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]{1,2})))|([0-9a-f]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]{1,2})\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]{1,2}))))$";
      Regex::new(pattern).expect("Invalid IPv6 Regex Pattern")
  })
}
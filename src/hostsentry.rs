use std::net::Ipv4Addr;

use fqdn::{fqdn, FQDN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostsEntry {
    pub ip: Ipv4Addr,
    pub host: FQDN,
}

impl Ord for HostsEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ip
            .cmp(&other.ip)
            .then(self.host.as_c_str().cmp(other.host.as_c_str()))
    }
}

impl PartialOrd for HostsEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ip.partial_cmp(&other.ip) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.host.as_c_str().partial_cmp(other.host.as_c_str())
    }
}

impl HostsEntry {
    pub fn new(ip: Ipv4Addr, host: FQDN) -> Self {
        HostsEntry { ip, host }
    }

    pub fn from(ip: (u8, u8, u8, u8), fqdn: &str) -> Self {
        HostsEntry {
            ip: Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3),
            host: fqdn!(fqdn),
        }
    }

    pub fn parse(ip: &str, host: &str) -> Result<Self, String> {
        let parsed_ip = match ip.parse::<Ipv4Addr>() {
            Ok(ip) => ip,
            Err(error) => return Err(error.to_string()),
        };
        let parsed_host = match host.parse::<FQDN>() {
            Ok(host) => host,
            Err(error) => return Err(error.to_string()),
        };
        Ok(HostsEntry::new(parsed_ip, parsed_host))
    }
}

impl std::fmt::Display for HostsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ip, self.host)
    }
}

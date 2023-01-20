use super::HostsFile;
use crate::hostsentry::HostsEntry;
use fqdn::FQDN;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedLine {
    Entry(HostsEntry),
    Comment(String),
    Invalid(String),
    Empty,
}

impl std::fmt::Display for ParsedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedLine::Entry(entry) => write!(f, "{}", entry),
            ParsedLine::Comment(comment) => write!(f, "{}", comment),
            ParsedLine::Invalid(line) => write!(f, "{}", line),
            ParsedLine::Empty => write!(f, ""),
        }
    }
}

impl HostsFile {
    pub fn parse_line(line: &str) -> ParsedLine {
        if line.is_empty() {
            return ParsedLine::Empty;
        } else if line.starts_with(' ') {
            return ParsedLine::Invalid(String::from(line));
        }
        if line.starts_with('#') {
            return ParsedLine::Comment(String::from(line));
        }
        let mut ip: Option<Ipv4Addr> = None;
        let mut host: Option<FQDN> = None;
        for (index, part) in line.split(' ').enumerate() {
            if index == 0 {
                match part.parse::<Ipv4Addr>() {
                    Ok(ipv4addr) => ip = Some(ipv4addr),
                    Err(_) => return ParsedLine::Invalid(String::from(line)),
                }
            } else {
                if part.is_empty() {
                    continue;
                }
                match host {
                    Some(_) => {
                        if !part.starts_with('#') {
                            return ParsedLine::Invalid(String::from(line));
                        }
                        break;
                    }
                    None => {
                        match part.trim_end().parse::<FQDN>() {
                            Ok(fqdn) => {
                                if fqdn.depth() >= 1 {
                                    host = Some(fqdn)
                                } else {
                                    return ParsedLine::Invalid(String::from(line));
                                }
                            }
                            Err(_) => return ParsedLine::Invalid(String::from(line)),
                        };
                    }
                }
            }
        }
        match (ip, host) {
            (Some(ip), Some(host)) => ParsedLine::Entry(HostsEntry { ip, host }),
            _ => panic!("Guru meditation!"),
        }
    }
}

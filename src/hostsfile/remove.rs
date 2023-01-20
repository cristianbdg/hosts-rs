use super::parse::ParsedLine;
use super::HostsFile;
use fqdn::FQDN;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Debug)]
pub enum RemoveResult {
    RemovedByIp(u8),
    RemovedByHost(u8),
    Invalid,
}

impl HostsFile {
    pub fn remove_from_file(
        &mut self,
        destination_path: &PathBuf,
        entry: &str,
    ) -> Result<RemoveResult, std::io::Error> {
        let action = HostsFile::remove_from_lines(&mut self.lines, entry);
        match action {
            RemoveResult::RemovedByIp(_) => self.write(destination_path)?,
            RemoveResult::RemovedByHost(_) => self.write(destination_path)?,
            RemoveResult::Invalid => {}
        }
        Ok(action)
    }

    pub fn remove_from_lines(lines: &mut Vec<ParsedLine>, entry: &str) -> RemoveResult {
        let parsed_ip = entry.parse::<Ipv4Addr>();
        let parsed_host = entry.parse::<FQDN>();
        if parsed_ip.is_err() && parsed_host.is_err() {
            return RemoveResult::Invalid;
        }

        let mut removed_count = 0;
        lines.retain(|line| {
            let retain = match line {
                ParsedLine::Entry(line_entry) => match parsed_ip {
                    Ok(ref ip) => *ip != line_entry.ip,
                    Err(_) => match parsed_host {
                        Ok(ref host) => *host != line_entry.host,
                        Err(_) => true,
                    },
                },
                ParsedLine::Comment(_) => true,
                ParsedLine::Invalid(_) => true,
                ParsedLine::Empty => true,
            };
            if !retain {
                removed_count += 1;
            }
            retain
        });

        if parsed_ip.is_ok() {
            RemoveResult::RemovedByIp(removed_count)
        } else {
            RemoveResult::RemovedByHost(removed_count)
        }
    }
}

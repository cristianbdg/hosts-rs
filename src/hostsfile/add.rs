use super::parse::ParsedLine;
use super::HostsFile;
use crate::hostsentry::HostsEntry;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AddResult {
    Added,
    Updated(Ipv4Addr),
    Skipped,
}

impl HostsFile {
    pub fn add_to_file(
        &mut self,
        destination_path: &PathBuf,
        entry: &HostsEntry,
    ) -> std::io::Result<AddResult> {
        let action = HostsFile::add_to_lines(&mut self.lines, entry);
        match action {
            AddResult::Added => HostsFile::write(self, destination_path)?,
            AddResult::Updated(_) => HostsFile::write(self, destination_path)?,
            AddResult::Skipped => {}
        }
        Ok(action)
    }

    pub fn add_to_lines(lines: &mut Vec<ParsedLine>, entry: &HostsEntry) -> AddResult {
        let mut action = AddResult::Added;
        for line in lines.iter_mut() {
            match line {
                ParsedLine::Entry(line_entry) => {
                    if line_entry.host == entry.host {
                        if line_entry.ip == entry.ip {
                            action = AddResult::Skipped;
                        } else {
                            action = AddResult::Updated(line_entry.ip.to_owned());
                            line_entry.ip = entry.ip;
                        }
                    }
                }
                ParsedLine::Comment(_) => {}
                ParsedLine::Invalid(_) => {}
                ParsedLine::Empty => {}
            }
        }
        match action {
            AddResult::Added => {
                lines.push(ParsedLine::Entry(entry.clone()));
            }
            AddResult::Updated(_) => {}
            AddResult::Skipped => {}
        }
        action
    }
}

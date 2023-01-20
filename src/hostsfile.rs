use self::parse::ParsedLine;
use crate::hostsentry::HostsEntry;

pub mod add;
pub mod parse;
pub mod read;
pub mod remove;
pub mod write;

#[derive(Debug, PartialEq, Eq)]
pub struct HostsFile {
    pub lines: Vec<ParsedLine>,
    pub ip_width: usize,
}

impl HostsFile {
    pub const PATH: &str = r"C:\WINDOWS\System32\drivers\etc\hosts";

    pub fn entries(&self) -> Vec<&HostsEntry> {
        let mut entries = Vec::new();
        for line in &self.lines {
            match line {
                ParsedLine::Entry(entry) => entries.push(entry),
                ParsedLine::Comment(_) => {}
                ParsedLine::Invalid(_) => {}
                ParsedLine::Empty => {}
            }
        }
        entries
    }

    pub fn invalids(&self) -> Vec<&String> {
        let mut invalids = Vec::new();
        for line in &self.lines {
            match line {
                ParsedLine::Invalid(line) => invalids.push(line),
                ParsedLine::Entry(_) => {}
                ParsedLine::Comment(_) => {}
                ParsedLine::Empty => {}
            }
        }
        invalids
    }

    pub fn entries_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| match line {
                ParsedLine::Entry(_) => true,
                ParsedLine::Comment(_) => false,
                ParsedLine::Invalid(_) => false,
                ParsedLine::Empty => false,
            })
            .count()
    }

    pub fn invalids_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| match line {
                ParsedLine::Invalid(_) => true,
                ParsedLine::Entry(_) => false,
                ParsedLine::Comment(_) => false,
                ParsedLine::Empty => false,
            })
            .count()
    }
}

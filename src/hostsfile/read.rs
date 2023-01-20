use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use super::parse::ParsedLine;
use super::HostsFile;

impl HostsFile {
    pub fn read_from_file(path: &PathBuf) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let read_file = HostsFile::read_from_lines(reader.lines())?;
        Ok(read_file)
    }

    pub fn read_from_lines<T>(lines: T) -> std::io::Result<HostsFile>
    where
        T: Iterator<Item = std::io::Result<String>>,
    {
        let mut parsed_lines = Vec::new();
        let mut ip_width = 0;
        for line in lines {
            match line {
                Ok(line) => {
                    let parsed_line = HostsFile::parse_line(&line);
                    match &parsed_line {
                        ParsedLine::Entry(entry) => {
                            let entry_ip_width = format!("{}", entry.ip).len();
                            if entry_ip_width > ip_width {
                                ip_width = entry_ip_width;
                            }
                        }
                        ParsedLine::Comment(_) => {}
                        ParsedLine::Invalid(_) => {}
                        ParsedLine::Empty => {}
                    }
                    parsed_lines.push(parsed_line);
                }
                Err(error) => return Err(error),
            }
        }
        Ok(HostsFile {
            lines: parsed_lines,
            ip_width,
        })
    }
}

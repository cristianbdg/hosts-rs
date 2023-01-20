use std::fs::OpenOptions;
use std::io::{LineWriter, Write};
use std::path::PathBuf;

use super::HostsFile;

impl HostsFile {
    pub fn write(&self, path: &PathBuf) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        let mut file = LineWriter::new(file);
        for line in self.lines.iter() {
            writeln!(file, "{}", line)?;
        }
        file.flush()?;
        Ok(())
    }
}

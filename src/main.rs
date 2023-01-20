use std::path::PathBuf;

use argh::FromArgs;
use colored::Colorize;

use hosts::hostsentry::HostsEntry;
use hosts::hostsfile::add::AddResult;
use hosts::hostsfile::remove::RemoveResult;
use hosts::hostsfile::HostsFile;

#[derive(FromArgs, PartialEq, Debug)]
/// Manage [IP hostname] entries in your hosts file.
struct TopLevel {
    #[argh(subcommand)]
    subcommands: Option<SubCommands>,
    /// path to hosts file (defaults to system path)
    #[argh(option, default = "String::from(HostsFile::PATH)")]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    List(ListCommand),
    Add(AddCommand),
    Remove(RemoveCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// List the entries in your hosts file.
#[argh(subcommand, name = "list")]
struct ListCommand {
    /// path to hosts file (defaults to system path)
    #[argh(option, default = "String::from(HostsFile::PATH)")]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add an entry to your hosts file.
#[argh(subcommand, name = "add")]
struct AddCommand {
    #[argh(positional)]
    /// IP of the entry to add
    ip: String,
    #[argh(positional)]
    /// hostname of the entry to add
    host: String,
    /// path to hosts file (defaults to system path)
    #[argh(option, default = "String::from(HostsFile::PATH)")]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Remove an entry (or entries) from the hosts file.
#[argh(subcommand, name = "remove")]
struct RemoveCommand {
    #[argh(positional)]
    /// IP or hostname of the entry to remove
    entry: String,
    /// path to hosts file (defaults to system path)
    #[argh(option, default = "String::from(HostsFile::PATH)")]
    path: String,
}

pub fn main() {
    let command: TopLevel = argh::from_env();
    match command.subcommands {
        Some(subcommand) => match subcommand {
            SubCommands::List(parameters) => list_entries(&PathBuf::from(&parameters.path)),
            SubCommands::Add(parameters) => add_entry(
                &PathBuf::from(&parameters.path),
                parameters.ip,
                parameters.host,
            ),
            SubCommands::Remove(parameters) => {
                remove_entry(&PathBuf::from(&parameters.path), parameters.entry)
            }
        },
        None => list_entries(&PathBuf::from(&command.path)),
    }
}

fn list_entries(path: &PathBuf) {
    let file = match HostsFile::read_from_file(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };
    let entries = file.entries_count();
    if entries == 1 {
        println!("Found 1 entry in {}", path.to_string_lossy());
    } else {
        println!("Found {} entries in {}", entries, path.to_string_lossy());
    }
    for entry in file.entries() {
        println!(
            "{} {}",
            format!("{:width$}", entry.ip, width = file.ip_width).bright_yellow(),
            format!("{}", entry.host).bright_cyan()
        );
    }
    let invalids = file.invalids_count();
    if invalids > 0 {
        println!();
        if invalids == 1 {
            println!("{}", "Found 1 invalid entry".bright_white().on_bright_red());
        } else {
            println!(
                "{}",
                format!("Found {} invalid entries", invalids)
                    .bright_white()
                    .on_bright_red()
            );
        }
        for entry in file.invalids() {
            println!("{}", entry);
        }
    };
}

fn add_entry(path: &PathBuf, ip: String, host: String) {
    let mut file = match HostsFile::read_from_file(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };
    let entry = match HostsEntry::parse(&ip, &host) {
        Ok(entry) => entry,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };
    match file.add_to_file(path, &entry) {
        Ok(action) => match action {
            AddResult::Added => {
                println!("Entry added");
                println!(
                    "{} {}",
                    format!("{}", entry.ip).bright_yellow(),
                    format!("{}", entry.host).bright_cyan()
                );
            }
            AddResult::Updated(previous_ip) => {
                println!("Existing entry updated");
                println!(
                    "{} > {} {}",
                    format!("{}", previous_ip).bright_yellow(),
                    format!("{}", entry.ip).bright_yellow(),
                    format!("{}", entry.host).bright_cyan()
                );
            }
            AddResult::Skipped => {
                println!("Entry already exists");
                println!(
                    "{} {}",
                    format!("{}", entry.ip).bright_yellow(),
                    format!("{}", entry.host).bright_cyan()
                );
            }
        },
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn remove_entry(path: &PathBuf, entry: String) {
    let mut file = match HostsFile::read_from_file(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };
    match file.remove_from_file(path, &entry) {
        Ok(action) => match action {
            RemoveResult::RemovedByIp(amount) => {
                if amount == 1 {
                    println!(
                        "Entry with ip {} removed",
                        format!("{}", entry).bright_yellow()
                    );
                } else {
                    println!(
                        "Removed {} entries with ip {}",
                        amount,
                        format!("{}", entry).bright_yellow()
                    );
                }
            }
            RemoveResult::RemovedByHost(amount) => {
                if amount == 1 {
                    println!(
                        "Entry with hostname {} removed",
                        format!("{}", entry).bright_cyan()
                    );
                } else {
                    println!(
                        "Removed {} entries with hostname {}",
                        amount,
                        format!("{}", entry).bright_cyan()
                    );
                }
            }
            RemoveResult::Invalid => {
                println!(
                    "Error: invalid entry {}",
                    format!("{}", entry).bright_white().on_bright_red()
                );
                std::process::exit(0);
            }
        },
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

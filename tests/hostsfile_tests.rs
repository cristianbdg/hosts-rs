use hosts::hostsentry::HostsEntry;
use hosts::hostsfile::add::AddResult;
use hosts::hostsfile::parse::ParsedLine;
use hosts::hostsfile::remove::RemoveResult;
use hosts::hostsfile::HostsFile;

fn get_test_host() -> &'static str {
    "host.domain.com"
}

fn get_test_entry() -> HostsEntry {
    HostsEntry::from((1, 1, 1, 1), get_test_host())
}

fn get_test_entry2() -> HostsEntry {
    HostsEntry::from((2, 2, 2, 2), get_test_host())
}

#[test]
pub fn it_reads_entries() {
    let lines = vec![
        std::io::Result::Ok(format!("1.1.1.1 {}", get_test_host())),
        std::io::Result::Ok(format!("2.2.2.2 {}", get_test_host())),
    ];
    let expected = vec![
        ParsedLine::Entry(get_test_entry()),
        ParsedLine::Entry(get_test_entry2()),
    ];
    match HostsFile::read_from_lines(lines.into_iter()) {
        Ok(hosts_file) => assert_eq!(hosts_file.lines, expected),
        Err(_) => panic!(),
    }
}

#[test]
fn it_adds_entry() {
    let mut lines = vec![ParsedLine::Comment(String::from("# comment"))];
    let expected = vec![
        ParsedLine::Comment(String::from("# comment")),
        ParsedLine::Entry(get_test_entry()),
    ];
    match HostsFile::add_to_lines(&mut lines, &get_test_entry()) {
        AddResult::Added => assert_eq!(lines, expected),
        AddResult::Skipped => panic!("Skipped instead of added"),
        AddResult::Updated(_) => panic!("Updated instead of added"),
    }
}

#[test]
fn it_skips_entry() {
    let mut lines = vec![ParsedLine::Entry(get_test_entry())];
    let expected = vec![ParsedLine::Entry(get_test_entry())];
    match HostsFile::add_to_lines(&mut lines, &get_test_entry()) {
        AddResult::Skipped => assert_eq!(lines, expected),
        AddResult::Added => panic!("Added instead of skipped"),
        AddResult::Updated(_) => panic!("Updated instead of skipped"),
    }
}

#[test]
fn it_updates_entry() {
    let mut lines = vec![ParsedLine::Entry(get_test_entry())];
    let expected = vec![ParsedLine::Entry(get_test_entry2())];
    match HostsFile::add_to_lines(&mut lines, &get_test_entry2()) {
        AddResult::Updated(_) => assert_eq!(lines, expected),
        AddResult::Added => panic!("Added instead of updated"),
        AddResult::Skipped => panic!("Skipped instead of updated"),
    }
}

#[test]
fn it_removes_entry_by_host() {
    let mut lines = vec![
        ParsedLine::Comment(String::from("# comment")),
        ParsedLine::Entry(get_test_entry()),
    ];
    let expected = vec![ParsedLine::Comment(String::from("# comment"))];
    match HostsFile::remove_from_lines(&mut lines, &get_test_host()) {
        RemoveResult::RemovedByHost(amount) if amount == 1 => assert_eq!(lines, expected),
        RemoveResult::RemovedByHost(amount) => panic!("Removed {} hosts instead of 1", amount),
        RemoveResult::RemovedByIp(amount) => panic!("Removed {} IPs instead of 1 host", amount),
        RemoveResult::Invalid => panic!("Entry is invalid"),
    }
}

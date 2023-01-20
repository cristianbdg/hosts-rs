use std::path::PathBuf;

use hosts::hostsfile::HostsFile;

fn get_testfile_path() -> PathBuf {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(r"resources\test\hosts.txt");
    path
}

#[test]
fn it_reads_testfile_correctly() {
    let file = HostsFile::read_from_file(&get_testfile_path()).unwrap();
    assert_eq!(file.entries().len(), 8);
    assert_eq!(file.invalids().len(), 2);
}

#[test]
fn it_reads_hostsfile_correctly() {
    let result = HostsFile::read_from_file(&PathBuf::from(HostsFile::PATH));
    assert!(result.is_ok());
}

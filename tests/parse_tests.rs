use hosts::hostsfile::HostsFile;
use hosts::hostsentry::HostsEntry;
use hosts::hostsfile::parse::ParsedLine;

fn get_test_line() -> ParsedLine {
    ParsedLine::Entry(HostsEntry::from((1, 2, 3, 4), "host.domain.com"))
}

#[test]
fn it_parses_regular_entries() {
    let result = HostsFile::parse_line("1.2.3.4 host.domain.com");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_entries_with_multiple_spaces() {
    let result = HostsFile::parse_line("1.2.3.4   host.domain.com");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_entries_with_multiple_spaces_before_comment() {
    let result = HostsFile::parse_line("1.2.3.4 host.domain.com   # foo");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_entries_with_a_comment() {
    let result = HostsFile::parse_line("1.2.3.4 host.domain.com # foo bar");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_entries_with_a_comment_without_space() {
    let result = HostsFile::parse_line("1.2.3.4 host.domain.com #foobar");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_entries_with_multiple_comments() {
    let result = HostsFile::parse_line("1.2.3.4 host.domain.com # foo # bar");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_entries_with_spaces_at_the_end() {
    let result = HostsFile::parse_line("1.2.3.4 host.domain.com   ");
    let expected = get_test_line();
    assert_eq!(result, expected);
}

#[test]
fn it_parses_comment() {
    let line = "# 1.2.3.4 host.domain.com";
    let result = HostsFile::parse_line(line);
    let expected = ParsedLine::Comment(String::from(line));
    assert_eq!(result, expected);
}

#[test]
fn it_parses_comment_without_space() {
    let line = "#1.2.3.4 host.domain.com";
    let result = HostsFile::parse_line(line);
    let expected = ParsedLine::Comment(String::from(line));
    assert_eq!(result, expected);
}

#[test]
fn it_parses_invalid_ip() {
    let line = "a.b.c.d host.domain.com";
    let result = HostsFile::parse_line(line);
    let expected = ParsedLine::Invalid(String::from(line));
    assert_eq!(result, expected);
}

#[test]
fn it_parses_invalid_host() {
    let line = "1.2.3.4 invalid";
    let result = HostsFile::parse_line(line);
    let expected = ParsedLine::Invalid(String::from(line));
    assert_eq!(result, expected);
}

#[test]
fn it_parses_invalid_starting_with_space() {
    let line = " 1.2.3.4 host.domain.com";
    let result = HostsFile::parse_line(line);
    let expected = ParsedLine::Invalid(String::from(line));
    assert_eq!(result, expected);
}

#[test]
fn it_parses_invalid_with_trailing_text() {
    let line = "1.2.3.4 host.domain.com invalid";
    let result = HostsFile::parse_line(line);
    let expected = ParsedLine::Invalid(String::from(line));
    assert_eq!(result, expected);
}

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_flag_shows_usage() {
    Command::cargo_bin("flood")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Target URL with FUZZ keyword"))
        .stdout(predicate::str::contains("--wordlist"));
}

#[test]
fn test_version_flag() {
    Command::cargo_bin("flood")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("flood"));
}

#[test]
fn test_missing_url_fails() {
    Command::cargo_bin("flood")
        .unwrap()
        .arg("-w")
        .arg("wordlist.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--url"));
}

#[test]
fn test_missing_wordlist_fails() {
    Command::cargo_bin("flood")
        .unwrap()
        .arg("-u")
        .arg("http://example.com/FUZZ")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--wordlist"));
}

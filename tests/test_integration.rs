mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_basic_scan_finds_results() {
    let server = common::setup_mock_server().await;
    let wordlist = common::create_wordlist(&["admin", "api", "secret", "images", ".htaccess"]);
    let output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin("flood").unwrap()
        .args(&["-u", &format!("{}/FUZZ", server.uri()), "-w", wordlist.path().to_str().unwrap(),
            "-t", "5", "-s", "-o", output_file.path().to_str().unwrap(), "--output-format", "json"])
        .assert().success();

    let content = std::fs::read_to_string(output_file.path()).unwrap();
    let results: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();
    assert!(results.len() >= 3, "Expected at least 3 results, got {}", results.len());
    let urls: Vec<String> = results.iter().map(|r| r["url"].as_str().unwrap().to_string()).collect();
    assert!(urls.iter().any(|u| u.contains("/admin")));
    assert!(urls.iter().any(|u| u.contains("/api")));
    assert!(!urls.iter().any(|u| u.contains("/secret")));
}

#[tokio::test]
async fn test_filter_code_excludes_403() {
    let server = common::setup_mock_server().await;
    let wordlist = common::create_wordlist(&["admin", ".htaccess"]);
    let output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin("flood").unwrap()
        .args(&["-u", &format!("{}/FUZZ", server.uri()), "-w", wordlist.path().to_str().unwrap(),
            "-t", "5", "-s", "--filter-code", "403", "-o", output_file.path().to_str().unwrap(), "--output-format", "json"])
        .assert().success();

    let content = std::fs::read_to_string(output_file.path()).unwrap();
    let results: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();
    let urls: Vec<String> = results.iter().map(|r| r["url"].as_str().unwrap().to_string()).collect();
    assert!(!urls.iter().any(|u| u.contains(".htaccess")));
    assert!(urls.iter().any(|u| u.contains("/admin")));
}

#[tokio::test]
async fn test_no_fuzz_keyword_fails() {
    let wordlist = common::create_wordlist(&["admin"]);
    Command::cargo_bin("flood").unwrap()
        .args(&["-u", "http://example.com/test", "-w", wordlist.path().to_str().unwrap()])
        .assert().failure().stderr(predicate::str::contains("No FUZZ keyword found"));
}

#[tokio::test]
async fn test_jsonl_output_format() {
    let server = common::setup_mock_server().await;
    let wordlist = common::create_wordlist(&["admin", "api"]);
    let output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin("flood").unwrap()
        .args(&["-u", &format!("{}/FUZZ", server.uri()), "-w", wordlist.path().to_str().unwrap(),
            "-t", "5", "-s", "-o", output_file.path().to_str().unwrap(), "--output-format", "jsonl"])
        .assert().success();

    let content = std::fs::read_to_string(output_file.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert!(lines.len() >= 2);
    for line in &lines { let _: serde_json::Value = serde_json::from_str(line).unwrap(); }
}

#[tokio::test]
async fn test_csv_output_format() {
    let server = common::setup_mock_server().await;
    let wordlist = common::create_wordlist(&["admin"]);
    let output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin("flood").unwrap()
        .args(&["-u", &format!("{}/FUZZ", server.uri()), "-w", wordlist.path().to_str().unwrap(),
            "-t", "5", "-s", "-o", output_file.path().to_str().unwrap(), "--output-format", "csv"])
        .assert().success();

    let content = std::fs::read_to_string(output_file.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert!(lines.len() >= 2);
    assert!(lines[0].contains("url"));
    assert!(lines[0].contains("status"));
}

use flood::output::{csv_writer, json, text, ScanResult};
use std::io::Read;
use tempfile::NamedTempFile;

fn sample_results() -> Vec<ScanResult> {
    vec![
        ScanResult {
            url: "https://example.com/admin".to_string(),
            status: 200,
            size: 12847,
            words: 1847,
            lines: 142,
            duration_ms: 85,
            redirect_to: None,
            content_type: Some("text/html".to_string()),
            depth: 0,
            input: "admin".to_string(),
        },
        ScanResult {
            url: "https://example.com/images".to_string(),
            status: 301,
            size: 194,
            words: 7,
            lines: 4,
            duration_ms: 32,
            redirect_to: Some("https://example.com/images/".to_string()),
            content_type: Some("text/html".to_string()),
            depth: 0,
            input: "images".to_string(),
        },
    ]
}

#[test]
fn test_json_output() {
    let results = sample_results();
    let f = NamedTempFile::new().unwrap();
    json::write_json(&results, f.path()).unwrap();
    let mut content = String::new();
    std::fs::File::open(f.path())
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let parsed: Vec<ScanResult> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].url, "https://example.com/admin");
    assert_eq!(parsed[0].status, 200);
    assert_eq!(
        parsed[1].redirect_to,
        Some("https://example.com/images/".to_string())
    );
}

#[test]
fn test_jsonl_output() {
    let results = sample_results();
    let f = NamedTempFile::new().unwrap();
    json::write_jsonl(&results, f.path()).unwrap();
    let content = std::fs::read_to_string(f.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);
    let first: ScanResult = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(first.url, "https://example.com/admin");
}

#[test]
fn test_csv_output() {
    let results = sample_results();
    let f = NamedTempFile::new().unwrap();
    csv_writer::write_csv(&results, f.path()).unwrap();
    let content = std::fs::read_to_string(f.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("url"));
    assert!(lines[0].contains("status"));
    assert!(lines[1].contains("admin"));
}

#[test]
fn test_text_output() {
    let results = sample_results();
    let f = NamedTempFile::new().unwrap();
    text::write_text(&results, f.path()).unwrap();
    let content = std::fs::read_to_string(f.path()).unwrap();
    assert!(content.contains("200"));
    assert!(content.contains("/admin"));
    assert!(content.contains("301"));
    assert!(content.contains("/images"));
}

#[test]
fn test_scan_result_human_size() {
    let r = ScanResult {
        url: String::new(),
        status: 200,
        size: 0,
        words: 0,
        lines: 0,
        duration_ms: 0,
        redirect_to: None,
        content_type: None,
        depth: 0,
        input: String::new(),
    };
    assert_eq!(r.human_size(), "0B");
    let r2 = ScanResult {
        size: 1024,
        ..r.clone()
    };
    assert_eq!(r2.human_size(), "1.0 KB");
    let r3 = ScanResult {
        size: 1_048_576,
        ..r.clone()
    };
    assert_eq!(r3.human_size(), "1.0 MB");
    let r4 = ScanResult { size: 500, ..r };
    assert_eq!(r4.human_size(), "500B");
}

use flood::state::{ScanState, save_state, load_state};
use flood::output::ScanResult;
use tempfile::NamedTempFile;

fn sample_state() -> ScanState {
    ScanState {
        url: "https://example.com/FUZZ".to_string(),
        wordlist_paths: vec!["common.txt".to_string()],
        method: "GET".to_string(),
        headers: vec![],
        data: None,
        match_codes: "200,204,301,302,307,401,403".to_string(),
        threads: 100,
        timeout: 7,
        wordlist_position: 5000,
        recursion_pending: vec![("/admin/".to_string(), 1)],
        results: vec![ScanResult {
            url: "https://example.com/admin".to_string(), status: 200, size: 12847,
            words: 1847, lines: 142, duration_ms: 85, redirect_to: None,
            content_type: Some("text/html".to_string()), depth: 0, input: "admin".to_string(),
        }],
        elapsed_secs: 42,
        errors: 3,
    }
}

#[test]
fn test_save_and_load_state() {
    let state = sample_state();
    let f = NamedTempFile::new().unwrap();
    save_state(&state, f.path()).unwrap();
    let loaded = load_state(f.path()).unwrap();
    assert_eq!(loaded.url, state.url);
    assert_eq!(loaded.wordlist_position, 5000);
    assert_eq!(loaded.recursion_pending.len(), 1);
    assert_eq!(loaded.results.len(), 1);
    assert_eq!(loaded.elapsed_secs, 42);
    assert_eq!(loaded.errors, 3);
}

#[test]
fn test_load_nonexistent_state_fails() {
    let result = load_state(std::path::Path::new("/nonexistent/state.json"));
    assert!(result.is_err());
}

#[test]
fn test_state_roundtrip_preserves_all_fields() {
    let state = sample_state();
    let f = NamedTempFile::new().unwrap();
    save_state(&state, f.path()).unwrap();
    let loaded = load_state(f.path()).unwrap();
    assert_eq!(loaded.url, state.url);
    assert_eq!(loaded.wordlist_paths, state.wordlist_paths);
    assert_eq!(loaded.method, state.method);
    assert_eq!(loaded.headers, state.headers);
    assert_eq!(loaded.data, state.data);
    assert_eq!(loaded.match_codes, state.match_codes);
    assert_eq!(loaded.threads, state.threads);
    assert_eq!(loaded.timeout, state.timeout);
    assert_eq!(loaded.wordlist_position, state.wordlist_position);
    assert_eq!(loaded.recursion_pending, state.recursion_pending);
    assert_eq!(loaded.results, state.results);
    assert_eq!(loaded.elapsed_secs, state.elapsed_secs);
    assert_eq!(loaded.errors, state.errors);
}

use flood::fuzz::{detect_fuzz_positions, substitute, FuzzPosition};

#[test]
fn test_detect_single_fuzz_in_url() {
    let positions = detect_fuzz_positions("https://example.com/FUZZ", &[], &None);
    assert_eq!(positions.len(), 1);
    assert!(matches!(positions[0], FuzzPosition::Url(_)));
}

#[test]
fn test_detect_fuzz_in_header() {
    let positions = detect_fuzz_positions(
        "https://example.com",
        &["Host: FUZZ.example.com".to_string()],
        &None,
    );
    assert_eq!(positions.len(), 1);
    assert!(matches!(positions[0], FuzzPosition::Header(_, _)));
}

#[test]
fn test_detect_fuzz_in_data() {
    let positions = detect_fuzz_positions(
        "https://example.com/login",
        &[],
        &Some("user=admin&pass=FUZZ".to_string()),
    );
    assert_eq!(positions.len(), 1);
    assert!(matches!(positions[0], FuzzPosition::Data(_)));
}

#[test]
fn test_detect_multiple_positions() {
    let positions = detect_fuzz_positions("https://example.com/FUZZ/FUZ2Z", &[], &None);
    assert_eq!(positions.len(), 2);
}

#[test]
fn test_detect_no_fuzz_keyword() {
    let positions = detect_fuzz_positions("https://example.com/test", &[], &None);
    assert_eq!(positions.len(), 0);
}

#[test]
fn test_substitute_single() {
    let result = substitute("https://example.com/FUZZ", "admin", "FUZZ");
    assert_eq!(result, "https://example.com/admin");
}

#[test]
fn test_substitute_multiple_same_keyword() {
    let result = substitute("https://FUZZ.example.com/FUZZ", "test", "FUZZ");
    assert_eq!(result, "https://test.example.com/test");
}

#[test]
fn test_substitute_fuz2z() {
    let result = substitute("https://example.com/FUZZ/FUZ2Z", "index.php", "FUZ2Z");
    assert_eq!(result, "https://example.com/FUZZ/index.php");
}

#[test]
fn test_fuzz_keywords_list() {
    let keywords = flood::fuzz::fuzz_keywords(3);
    assert_eq!(keywords, vec!["FUZZ", "FUZ2Z", "FUZ3Z"]);
}

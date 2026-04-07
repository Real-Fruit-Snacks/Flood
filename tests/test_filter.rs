use flood::filter::{FilterConfig, FilterEngine, ResponseData};

fn make_response(status: u16, size: u64, words: u64, lines: u64) -> ResponseData {
    ResponseData {
        status,
        size,
        words,
        lines,
        duration_ms: 100,
        body: String::new(),
        redirect_to: None,
        content_type: None,
    }
}

#[test]
fn test_default_match_codes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200, 204, 301, 302, 307, 401, 403],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(engine.should_display(&make_response(301, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(404, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(500, 100, 10, 5)));
}

#[test]
fn test_filter_code_excludes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200, 301, 403],
        filter_codes: vec![403],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(403, 100, 10, 5)));
}

#[test]
fn test_match_size() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        match_sizes: vec![100],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(200, 200, 10, 5)));
}

#[test]
fn test_filter_size_excludes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        filter_sizes: vec![0],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(200, 0, 10, 5)));
}

#[test]
fn test_match_words() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        match_words: vec![10],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(200, 100, 20, 5)));
}

#[test]
fn test_filter_words_excludes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        filter_words: vec![10],
        ..Default::default()
    });
    assert!(!engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(engine.should_display(&make_response(200, 100, 20, 5)));
}

#[test]
fn test_match_lines() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        match_lines: vec![5],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(!engine.should_display(&make_response(200, 100, 10, 10)));
}

#[test]
fn test_filter_lines_excludes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        filter_lines: vec![5],
        ..Default::default()
    });
    assert!(!engine.should_display(&make_response(200, 100, 10, 5)));
    assert!(engine.should_display(&make_response(200, 100, 10, 10)));
}

#[test]
fn test_match_regex() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        match_regex: Some(regex::Regex::new("admin").unwrap()),
        ..Default::default()
    });
    let mut resp = make_response(200, 100, 10, 5);
    resp.body = "Welcome admin panel".to_string();
    assert!(engine.should_display(&resp));
    resp.body = "Not found".to_string();
    assert!(!engine.should_display(&resp));
}

#[test]
fn test_filter_regex_excludes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        filter_regex: Some(regex::Regex::new("error").unwrap()),
        ..Default::default()
    });
    let mut resp = make_response(200, 100, 10, 5);
    resp.body = "Some error occurred".to_string();
    assert!(!engine.should_display(&resp));
    resp.body = "Success".to_string();
    assert!(engine.should_display(&resp));
}

#[test]
fn test_match_time() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        match_time: Some(500),
        ..Default::default()
    });
    let mut resp = make_response(200, 100, 10, 5);
    resp.duration_ms = 600;
    assert!(engine.should_display(&resp));
    resp.duration_ms = 100;
    assert!(!engine.should_display(&resp));
}

#[test]
fn test_filter_time_excludes() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200],
        filter_time: Some(500),
        ..Default::default()
    });
    let mut resp = make_response(200, 100, 10, 5);
    resp.duration_ms = 600;
    assert!(!engine.should_display(&resp));
    resp.duration_ms = 100;
    assert!(engine.should_display(&resp));
}

#[test]
fn test_combined_match_and_filter() {
    let engine = FilterEngine::new(FilterConfig {
        match_codes: vec![200, 301],
        filter_sizes: vec![0],
        ..Default::default()
    });
    assert!(engine.should_display(&make_response(200, 500, 10, 5)));
    assert!(!engine.should_display(&make_response(200, 0, 10, 5)));
    assert!(!engine.should_display(&make_response(404, 500, 10, 5)));
}

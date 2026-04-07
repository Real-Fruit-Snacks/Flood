use flood::recursion::{is_directory_response, RecursionQueue};

#[test]
fn test_is_directory_redirect_with_trailing_slash() {
    assert!(is_directory_response(
        301,
        Some("https://example.com/images/"),
        "/images"
    ));
}
#[test]
fn test_is_directory_redirect_without_trailing_slash() {
    assert!(!is_directory_response(
        301,
        Some("https://example.com/other"),
        "/images"
    ));
}
#[test]
fn test_is_directory_200_with_trailing_slash() {
    assert!(is_directory_response(200, None, "/images/"));
}
#[test]
fn test_is_not_directory_regular_file() {
    assert!(!is_directory_response(200, None, "/index.html"));
}
#[test]
fn test_queue_add_and_pop() {
    let mut queue = RecursionQueue::new(3, vec![]);
    assert!(queue.add("/images/", 0));
    assert_eq!(queue.pop(), Some(("/images/".to_string(), 1)));
    assert_eq!(queue.pop(), None);
}
#[test]
fn test_queue_respects_max_depth() {
    let mut queue = RecursionQueue::new(2, vec![]);
    assert!(queue.add("/a/", 0));
    assert!(queue.add("/b/", 1));
    assert!(!queue.add("/c/", 2));
}
#[test]
fn test_queue_respects_exclude_patterns() {
    let mut queue = RecursionQueue::new(3, vec!["static".to_string(), "assets".to_string()]);
    assert!(!queue.add("/static/", 0));
    assert!(!queue.add("/assets/", 0));
    assert!(queue.add("/admin/", 0));
}
#[test]
fn test_queue_deduplicates() {
    let mut queue = RecursionQueue::new(3, vec![]);
    assert!(queue.add("/images/", 0));
    assert!(!queue.add("/images/", 0));
    assert_eq!(queue.len(), 1);
}
#[test]
fn test_queue_pending_count() {
    let mut queue = RecursionQueue::new(3, vec![]);
    queue.add("/a/", 0);
    queue.add("/b/", 0);
    queue.add("/c/", 0);
    assert_eq!(queue.len(), 3);
    queue.pop();
    assert_eq!(queue.len(), 2);
}

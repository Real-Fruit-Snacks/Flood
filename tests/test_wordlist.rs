use flood::wordlist::{load_wordlist, cartesian_product};
use std::io::Write;
use tempfile::NamedTempFile;

fn create_wordlist(words: &[&str]) -> NamedTempFile {
    let mut f = NamedTempFile::new().unwrap();
    for word in words {
        writeln!(f, "{}", word).unwrap();
    }
    f
}

#[test]
fn test_load_wordlist_basic() {
    let f = create_wordlist(&["admin", "login", "api", "config"]);
    let words = load_wordlist(f.path()).unwrap();
    assert_eq!(words, vec!["admin", "login", "api", "config"]);
}

#[test]
fn test_load_wordlist_skips_empty_lines() {
    let f = create_wordlist(&["admin", "", "login", "", "api"]);
    let words = load_wordlist(f.path()).unwrap();
    assert_eq!(words, vec!["admin", "login", "api"]);
}

#[test]
fn test_load_wordlist_skips_comments() {
    let f = create_wordlist(&["# this is a comment", "admin", "# another comment", "login"]);
    let words = load_wordlist(f.path()).unwrap();
    assert_eq!(words, vec!["admin", "login"]);
}

#[test]
fn test_load_wordlist_trims_whitespace() {
    let f = create_wordlist(&["  admin  ", "login\t", " api"]);
    let words = load_wordlist(f.path()).unwrap();
    assert_eq!(words, vec!["admin", "login", "api"]);
}

#[test]
fn test_load_wordlist_nonexistent_file() {
    let result = load_wordlist(std::path::Path::new("/nonexistent/wordlist.txt"));
    assert!(result.is_err());
}

#[test]
fn test_cartesian_product_two_lists() {
    let lists = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["1".to_string(), "2".to_string()],
    ];
    let product = cartesian_product(&lists);
    assert_eq!(product.len(), 4);
    assert!(product.contains(&vec!["a".to_string(), "1".to_string()]));
    assert!(product.contains(&vec!["a".to_string(), "2".to_string()]));
    assert!(product.contains(&vec!["b".to_string(), "1".to_string()]));
    assert!(product.contains(&vec!["b".to_string(), "2".to_string()]));
}

#[test]
fn test_cartesian_product_single_list() {
    let lists = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]];
    let product = cartesian_product(&lists);
    assert_eq!(product.len(), 3);
    assert_eq!(product[0], vec!["a".to_string()]);
    assert_eq!(product[1], vec!["b".to_string()]);
    assert_eq!(product[2], vec!["c".to_string()]);
}

#[test]
fn test_cartesian_product_three_lists() {
    let lists = vec![
        vec!["a".to_string()],
        vec!["1".to_string(), "2".to_string()],
        vec!["x".to_string()],
    ];
    let product = cartesian_product(&lists);
    assert_eq!(product.len(), 2);
    assert!(product.contains(&vec!["a".to_string(), "1".to_string(), "x".to_string()]));
    assert!(product.contains(&vec!["a".to_string(), "2".to_string(), "x".to_string()]));
}

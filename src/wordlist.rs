use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Load a wordlist from a file path. Skips empty lines and lines starting with #.
pub fn load_wordlist(path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read wordlist: {}", path.display()))?;

    let words: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    if words.is_empty() {
        anyhow::bail!("Wordlist is empty: {}", path.display());
    }

    Ok(words)
}

/// Compute the cartesian product of multiple wordlists.
pub fn cartesian_product(lists: &[Vec<String>]) -> Vec<Vec<String>> {
    if lists.is_empty() {
        return vec![];
    }

    let mut result: Vec<Vec<String>> = lists[0].iter().map(|item| vec![item.clone()]).collect();

    for list in &lists[1..] {
        let mut new_result = Vec::with_capacity(result.len() * list.len());
        for existing in &result {
            for item in list {
                let mut combo = existing.clone();
                combo.push(item.clone());
                new_result.push(combo);
            }
        }
        result = new_result;
    }

    result
}

/// Expand a word with extensions.
pub fn expand_with_extensions(word: &str, extensions: &[String], no_extension: bool) -> Vec<String> {
    let mut results = Vec::new();
    if extensions.is_empty() || no_extension {
        results.push(word.to_string());
    }
    for ext in extensions {
        results.push(format!("{}{}", word, ext));
    }
    results
}

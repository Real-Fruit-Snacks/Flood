use crate::output::ScanResult;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanState {
    pub url: String,
    pub wordlist_paths: Vec<String>,
    pub method: String,
    pub headers: Vec<String>,
    pub data: Option<String>,
    pub match_codes: String,
    pub threads: usize,
    pub timeout: u64,
    pub wordlist_position: usize,
    pub recursion_pending: Vec<(String, u32)>,
    pub results: Vec<ScanResult>,
    pub elapsed_secs: u64,
    pub errors: u64,
}

pub fn save_state(state: &ScanState, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(state)?;
    std::fs::write(path, json)
        .with_context(|| format!("Failed to save state to {}", path.display()))?;
    Ok(())
}

pub fn load_state(path: &Path) -> Result<ScanState> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read state file: {}", path.display()))?;
    let state: ScanState = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse state file: {}", path.display()))?;
    Ok(state)
}

pub fn default_state_filename() -> String {
    let now = chrono::Local::now();
    format!("flood-state-{}.json", now.format("%Y%m%d-%H%M%S"))
}

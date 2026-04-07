pub mod json;
pub mod csv_writer;
pub mod text;
pub mod terminal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanResult {
    pub url: String,
    pub status: u16,
    pub size: u64,
    pub words: u64,
    pub lines: u64,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    pub depth: u32,
    pub input: String,
}

impl ScanResult {
    pub fn human_size(&self) -> String {
        if self.size >= 1_048_576 {
            format!("{:.1} MB", self.size as f64 / 1_048_576.0)
        } else if self.size >= 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else {
            format!("{}B", self.size)
        }
    }
}

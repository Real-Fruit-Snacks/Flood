use super::ScanResult;
use anyhow::Result;
use std::io::Write;
use std::path::Path;

pub fn write_text(results: &[ScanResult], path: &Path) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    for r in results {
        let redirect_info = match &r.redirect_to {
            Some(loc) => format!(" → {}", loc),
            None => String::new(),
        };
        let depth_info = if r.depth > 0 {
            format!(" (depth: {})", r.depth)
        } else {
            String::new()
        };
        writeln!(
            file,
            "{:>3}  {:>8}  {:>5}w  {}{}{}",
            r.status,
            r.human_size(),
            r.words,
            r.url,
            redirect_info,
            depth_info
        )?;
    }
    Ok(())
}

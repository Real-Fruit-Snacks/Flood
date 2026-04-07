use super::ScanResult;
use anyhow::Result;
use std::path::Path;

pub fn write_json(results: &[ScanResult], path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn write_jsonl(results: &[ScanResult], path: &Path) -> Result<()> {
    let mut output = String::new();
    for result in results {
        output.push_str(&serde_json::to_string(result)?);
        output.push('\n');
    }
    std::fs::write(path, output)?;
    Ok(())
}

pub fn append_jsonl(result: &ScanResult, path: &Path) -> Result<()> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", serde_json::to_string(result)?)?;
    Ok(())
}

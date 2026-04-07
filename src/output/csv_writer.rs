use super::ScanResult;
use anyhow::Result;
use std::path::Path;

pub fn write_csv(results: &[ScanResult], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record([
        "url",
        "status",
        "size",
        "words",
        "lines",
        "duration_ms",
        "redirect_to",
        "content_type",
        "depth",
        "input",
    ])?;
    for r in results {
        wtr.write_record([
            &r.url,
            &r.status.to_string(),
            &r.size.to_string(),
            &r.words.to_string(),
            &r.lines.to_string(),
            &r.duration_ms.to_string(),
            &r.redirect_to.as_deref().unwrap_or("").to_string(),
            &r.content_type.as_deref().unwrap_or("").to_string(),
            &r.depth.to_string(),
            &r.input,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

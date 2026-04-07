use super::ScanResult;
use crate::colors;
use crossterm::style::Stylize;

pub fn format_result(result: &ScanResult, no_color: bool) -> String {
    if no_color { return format_result_plain(result); }
    let status_color = colors::rgb(colors::status_color(result.status));
    let size_color = colors::rgb(colors::YELLOW);
    let words_color = colors::rgb(colors::MAUVE);
    let text_color = colors::rgb(colors::TEXT);
    let overlay_color = colors::rgb(colors::OVERLAY0);
    let redirect_info = match &result.redirect_to {
        Some(loc) => format!(" {} {}", "→".with(overlay_color), loc.clone().with(overlay_color)),
        None => String::new(),
    };
    let depth_info = if result.depth > 0 {
        format!(" {}", format!("(depth: {})", result.depth).with(overlay_color))
    } else { String::new() };
    format!("  {} {:>3}  {:>8}  {:>5}w  {}{}{}", "●".with(status_color), result.status.to_string().with(status_color), result.human_size().with(size_color), result.words.to_string().with(words_color), result.url.clone().with(text_color), redirect_info, depth_info)
}

fn format_result_plain(result: &ScanResult) -> String {
    let redirect_info = match &result.redirect_to { Some(loc) => format!(" → {}", loc), None => String::new() };
    let depth_info = if result.depth > 0 { format!(" (depth: {})", result.depth) } else { String::new() };
    format!("  ● {:>3}  {:>8}  {:>5}w  {}{}{}", result.status, result.human_size(), result.words, result.url, redirect_info, depth_info)
}

pub fn format_recursion_indicator(path: &str, no_color: bool) -> String {
    if no_color { return format!("      ├── scanning recursively: {}", path); }
    let teal = colors::rgb(colors::SKY);
    format!("      {}", format!("├── scanning recursively: {}", path).with(teal))
}

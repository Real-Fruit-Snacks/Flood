use crate::colors;
use crossterm::style::Stylize;

const BANNER: &str = r#"     _____ _                 _
    |  ___| | ___   ___   __| |
    | |_  | |/ _ \ / _ \ / _` |
    |  _| | | (_) | (_) | (_| |
    |_|   |_|\___/ \___/ \__,_|"#;

pub fn print_banner(version: &str, no_color: bool) {
    if no_color { println!("{}  v{}", BANNER, version); }
    else {
        let blue = colors::rgb(colors::BLUE);
        let overlay = colors::rgb(colors::OVERLAY0);
        println!("{}  {}", BANNER.with(blue), format!("v{}", version).with(overlay));
    }
}

pub fn print_config(url: &str, wordlist_name: &str, wordlist_count: usize, threads: usize, match_codes: &str, no_color: bool) {
    let separator = "─".repeat(40);
    if no_color {
        println!("    {}", separator);
        println!("    Target ─── {}", url);
        println!("    Wordlist ─ {} ({}) │ Threads: {}", wordlist_name, wordlist_count, threads);
        println!("    Filters ── Match: {}", match_codes);
        println!("    {}", separator);
    } else {
        let subtext = colors::rgb(colors::SUBTEXT0);
        let overlay = colors::rgb(colors::OVERLAY0);
        println!("    {}", separator.clone().with(overlay));
        println!("    {}", format!("Target ─── {}", url).with(subtext));
        println!("    {}", format!("Wordlist ─ {} ({}) │ Threads: {}", wordlist_name, wordlist_count, threads).with(subtext));
        println!("    {}", format!("Filters ── Match: {}", match_codes).with(subtext));
        println!("    {}", separator.with(overlay));
    }
}

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "flood",
    version,
    about = "Fast web fuzzer with recursive discovery",
    after_help = "EXAMPLES:\n  \
        # Basic directory enumeration\n  \
        flood -u https://example.com/FUZZ -w common.txt\n\n  \
        # Recursive scan with extensions\n  \
        flood -u https://example.com/FUZZ -w common.txt -r -e php,bak,conf\n\n  \
        # VHost discovery\n  \
        flood -u https://example.com -H \"Host: FUZZ.example.com\" -w subs.txt -fc 404\n\n  \
        # Parameter fuzzing with POST data\n  \
        flood -u https://example.com/login -d \"user=admin&pass=FUZZ\" -w passwords.txt -fc 401\n\n  \
        # Multiple wordlists (clusterbomb)\n  \
        flood -u https://example.com/FUZZ/FUZ2Z -w dirs.txt -w files.txt\n\n  \
        # Through Burp Suite proxy, rate limited\n  \
        flood -u https://example.com/FUZZ -w common.txt -x http://127.0.0.1:8080 --rate 50\n\n  \
        # Output results to JSON\n  \
        flood -u https://example.com/FUZZ -w common.txt -o results.json -of json"
)]
pub struct Cli {
    // === Target ===
    /// Target URL with FUZZ keyword
    #[arg(short, long)]
    pub url: String,

    /// Wordlist path (repeatable for multi-position: FUZZ, FUZ2Z, FUZ3Z...)
    #[arg(short, long, required = true, num_args = 1)]
    pub wordlist: Vec<PathBuf>,

    /// HTTP method
    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    /// POST data (enables POST, supports FUZZ keyword)
    #[arg(short, long)]
    pub data: Option<String>,

    /// Custom header "Name: Value" (repeatable, supports FUZZ)
    #[arg(short = 'H', long = "header", num_args = 1)]
    pub headers: Vec<String>,

    /// Cookie data "name=value; name2=value2"
    #[arg(short = 'b', long)]
    pub cookie: Option<String>,

    // === Discovery ===
    /// Enable recursive scanning
    #[arg(short, long = "recurse")]
    pub recurse: bool,

    /// Max recursion depth
    #[arg(long, default_value = "3")]
    pub depth: u32,

    /// Exclude pattern from recursion (repeatable)
    #[arg(short = 'R', long = "recurse-exclude", num_args = 1)]
    pub recurse_exclude: Vec<String>,

    /// File extensions to append, comma-separated (e.g., php,bak,conf)
    #[arg(short, long)]
    pub extensions: Option<String>,

    /// Also fuzz without any extension (when -e is set)
    #[arg(long)]
    pub no_extension: bool,

    /// Append / to each request
    #[arg(short = 'f', long)]
    pub add_slash: bool,

    // === Match filters ===
    /// Match HTTP status codes
    #[arg(
        long = "match-code",
        short = 'M',
        alias = "mc",
        default_value = "200,204,301,302,307,401,403"
    )]
    pub match_code: String,

    /// Match response size (bytes)
    #[arg(long = "match-size", alias = "ms")]
    pub match_size: Option<String>,

    /// Match word count
    #[arg(long = "match-words", alias = "mw")]
    pub match_words: Option<String>,

    /// Match line count
    #[arg(long = "match-lines", alias = "ml")]
    pub match_lines: Option<String>,

    /// Match regex in response body
    #[arg(long = "match-regex", alias = "mr")]
    pub match_regex: Option<String>,

    /// Match response time greater than N ms
    #[arg(long = "match-time", alias = "mt")]
    pub match_time: Option<u64>,

    // === Filter (exclude) ===
    /// Filter out HTTP status codes
    #[arg(long = "filter-code", alias = "fc")]
    pub filter_code: Option<String>,

    /// Filter out response size (bytes)
    #[arg(long = "filter-size", alias = "fs")]
    pub filter_size: Option<String>,

    /// Filter out word count
    #[arg(long = "filter-words", alias = "fw")]
    pub filter_words: Option<String>,

    /// Filter out line count
    #[arg(long = "filter-lines", alias = "fl")]
    pub filter_lines: Option<String>,

    /// Filter out regex match in response body
    #[arg(long = "filter-regex", alias = "fr")]
    pub filter_regex: Option<String>,

    /// Filter out response time greater than N ms
    #[arg(long = "filter-time", alias = "ft")]
    pub filter_time: Option<u64>,

    // === Performance ===
    /// Concurrent requests
    #[arg(short, long = "threads", default_value = "100")]
    pub threads: usize,

    /// Max requests per second (0 = unlimited)
    #[arg(long, default_value = "0")]
    pub rate: u32,

    /// Request timeout in seconds
    #[arg(long, default_value = "7")]
    pub timeout: u64,

    /// Retries on connection error
    #[arg(long, default_value = "3")]
    pub retries: u32,

    /// Disable automatic throttle on 429 responses
    #[arg(long)]
    pub no_auto_throttle: bool,

    // === Authentication ===
    /// HTTP Basic auth (user:pass)
    #[arg(long)]
    pub auth: Option<String>,

    /// Bearer token
    #[arg(long)]
    pub bearer: Option<String>,

    // === Network ===
    /// Proxy URL (HTTP or SOCKS5)
    #[arg(short = 'x', long)]
    pub proxy: Option<String>,

    /// Send only matched results through this proxy (e.g., Burp)
    #[arg(long)]
    pub replay_proxy: Option<String>,

    /// Skip TLS certificate verification
    #[arg(short = 'k', long)]
    pub insecure: bool,

    /// Follow HTTP redirects
    #[arg(long)]
    pub follow_redirects: bool,

    /// Max redirects to follow
    #[arg(long, default_value = "5")]
    pub max_redirects: usize,

    // === Output ===
    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output format: json, jsonl, csv, text
    #[arg(long = "output-format", alias = "of", default_value = "json")]
    pub output_format: String,

    /// Verbose output (show all responses including filtered)
    #[arg(short, long)]
    pub verbose: bool,

    /// Silent mode (results only, no banner or progress bar)
    #[arg(short, long)]
    pub silent: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    // === State ===
    /// Resume scan from state file
    #[arg(long)]
    pub resume: Option<PathBuf>,

    /// Path to save scan state on pause/quit
    #[arg(long)]
    pub state_file: Option<PathBuf>,

    // === Misc ===
    /// Custom User-Agent string
    #[arg(long, default_value = "Flood/0.1.0")]
    pub user_agent: String,

    /// Randomize User-Agent per request
    #[arg(long)]
    pub random_agent: bool,
}

/// Parse CLI arguments and return the validated config.
pub fn parse() -> Cli {
    Cli::parse()
}

/// Parse comma-separated status codes into a Vec<u16>.
pub fn parse_status_codes(input: &str) -> anyhow::Result<Vec<u16>> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u16>()
                .map_err(|e| anyhow::anyhow!("Invalid status code '{}': {}", s.trim(), e))
        })
        .collect()
}

/// Parse comma-separated numeric values into a Vec<u64>.
pub fn parse_numeric_list(input: &str) -> anyhow::Result<Vec<u64>> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u64>()
                .map_err(|e| anyhow::anyhow!("Invalid number '{}': {}", s.trim(), e))
        })
        .collect()
}

/// Parse comma-separated extensions, stripping leading dots.
pub fn parse_extensions(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| {
            let s = s.trim();
            if s.starts_with('.') {
                s.to_string()
            } else {
                format!(".{}", s)
            }
        })
        .filter(|s| s.len() > 1)
        .collect()
}

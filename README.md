<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Real-Fruit-Snacks/Flood/main/docs/assets/logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Real-Fruit-Snacks/Flood/main/docs/assets/logo-light.svg">
  <img alt="Flood" src="https://raw.githubusercontent.com/Real-Fruit-Snacks/Flood/main/docs/assets/logo-dark.svg" width="100%">
</picture>

> [!IMPORTANT]
> **Fast async web fuzzer with recursive discovery.** Directory enumeration, virtual host discovery, and parameter fuzzing with 100 concurrent requests, recursive scanning, auto-throttle on 429s, FUZZ keyword placement, and Catppuccin Mocha terminal UI.

> *A flood is an overwhelming surge of water that covers everything in its path, unstoppable and thorough. Felt fitting for a tool that sends massive concurrent requests across a target to discover every hidden endpoint and path.*

---

## §1 / Premise

Flood is an **async Rust web fuzzer** built for comprehensive discovery through overwhelming concurrent requests. Point it at a target with FUZZ keyword placement and it systematically enumerates directories, discovers virtual hosts, fuzzes parameters, and recursively scans discovered paths.

The async engine fans out 100 concurrent requests across tokio tasks with intelligent auto-throttling that monitors 429 responses and adjusts concurrency transparently. Advanced filtering by status codes, response size, word count, line count, and regex patterns. Interactive controls allow real-time adjustment of threads, pause/resume, and state saving for large scans.

**Authorization Required**: Designed exclusively for authorized security testing with explicit written permission.

---

## §2 / Specs

| KEY        | VALUE                                                                       |
|------------|-----------------------------------------------------------------------------|
| **FUZZING**    | **Directory enum · VHost discovery · parameter fuzzing · clusterbomb mode** |
| **CONCURRENCY** | **100 async requests** with auto-throttle monitoring 429 responses |
| **FILTERING**  | **Status code · response size · word count · line count · regex matching** |
| **RECURSIVE**  | **Auto-discovery and re-queue** with configurable depth limits |
| **INTEGRATION** | **Burp Suite proxy · replay to proxy · JSON/CSV/JSONL output formats** |
| **CONTROLS**   | **Real-time pause/resume · thread adjustment · state save/restore** |
| **KEYWORDS**   | **FUZZ/FUZ2Z placement** in URLs, headers, POST data, and query strings |
| **PLATFORM**   | **Linux · Windows · macOS** with cross-compilation and static linking |
| **UI**         | **Catppuccin Mocha terminal interface** with real-time progress and stats |
| **STACK**      | **Pure Rust async/tokio · HTTP client · filtering pipeline · interactive TUI** |

---

## §3 / Quickstart

**Prerequisites:** Rust toolchain (stable), wordlists

```bash
git clone https://github.com/Real-Fruit-Snacks/Flood.git
cd Flood
make release

# Basic directory enumeration
./target/release/flood -u https://example.com/FUZZ -w common.txt

# Virtual host discovery
./target/release/flood -u https://target.com -H "Host: FUZZ.target.com" -w subdomains.txt

# Parameter fuzzing with filtering
./target/release/flood -u https://target.com/login -d "user=admin&pass=FUZZ" -w passwords.txt --filter-code 401

# Recursive scanning with extensions
./target/release/flood -u https://target.com/FUZZ -w common.txt -r -e php,bak --depth 3
```

---

## §4 / Reference

```bash
# BASIC FUZZING
flood -u https://target.com/FUZZ -w wordlist.txt    # Directory enumeration
flood -u https://target.com/FUZZ/FUZ2Z -w dirs.txt -w files.txt  # Clusterbomb mode

# VIRTUAL HOST DISCOVERY
flood -u https://target.com -H "Host: FUZZ.target.com" -w subs.txt
flood -H "X-Forwarded-Host: FUZZ.target.com" -u https://target.com -w vhosts.txt

# PARAMETER FUZZING
flood -u https://target.com/search?q=FUZZ -w params.txt        # GET parameters
flood -u https://target.com/login -d "user=FUZZ&pass=admin" -w users.txt  # POST data
flood -u https://target.com -H "Authorization: Bearer FUZZ" -w tokens.txt  # Headers

# FILTERING & MATCHING
--match-code 200,301,403        # Match specific status codes
--filter-code 404,429           # Filter out status codes
--match-size 1337-1400          # Match response size range
--filter-size 0                 # Filter empty responses
--match-words 10-50             # Match word count range
--match-lines 5-20              # Match line count range
--match-regex "admin|dashboard" # Match regex in response body
--filter-regex "not found|404"  # Filter regex patterns

# RECURSIVE SCANNING
-r --depth 3                    # Recursive with max depth
-e php,asp,jsp,txt,bak         # File extensions to append
--discover-backup              # Auto-discover backup files (.bak, .old)

# PROXY & OUTPUT
-x http://127.0.0.1:8080       # Route through proxy
--replay-proxy http://127.0.0.1:8080  # Send matches to proxy
-o results.json                # JSON output
-o results.csv                 # CSV format
--format jsonl                 # JSON Lines format

# PERFORMANCE & CONTROL
-t 50                          # Thread count (default: 100)
--delay 100                    # Delay between requests (ms)
--timeout 10                   # Request timeout (seconds)
--auto-throttle                # Enable 429 auto-throttling
--save-state results.state     # Save state for resume
--resume results.state         # Resume from saved state

# INTERACTIVE CONTROLS (during scan)
p          # Pause/resume scanning
+/-        # Increase/decrease threads
q          # Quit and save state
Ctrl+C     # Stop immediately
```

---

## §5 / Architecture

**Three-Layer Design**: CLI config parser → Async fuzzing engine → Filter pipeline

```
src/
├── main.rs          # CLI dispatch and configuration parsing
├── fuzzer/          # Core async fuzzing engine with tokio tasks
├── http/            # HTTP request builder and response parser
├── filter/          # Match/filter pipeline (code, size, regex)
├── output/          # JSON, JSONL, CSV, text output formatters
├── throttle/        # Auto-throttle monitoring and rate limiting
├── recursive/       # Directory discovery and request re-queueing
└── ui/              # Catppuccin Mocha terminal user interface
```

**Async Architecture**: CLI parses flags into FuzzConfig, async engine fans out requests across 100 tokio tasks, filter pipeline processes responses with match/filter criteria, auto-throttle monitors 429 responses and adjusts concurrency transparently.

---

## §6 / Authorization

Flood is designed for **authorized web application security testing** with explicit written permission. Use only against targets you own or have proper authorization to test. The tool generates significant HTTP traffic and server logs.

Security vulnerabilities should be reported via [GitHub Security Advisories](https://github.com/Real-Fruit-Snacks/Flood/security/advisories) with 90-day responsible disclosure.

**Flood does not**: exploit discovered endpoints, inject payloads for SQLi/XSS testing, brute-force authentication without explicit FUZZ placement, or evade WAF/IDS detection systems.

---

**Real-Fruit-Snacks** — [All projects](https://real-fruit-snacks.github.io/) · [Security](https://github.com/Real-Fruit-Snacks/Flood/security/advisories) · [License](LICENSE)
<div align="center">
  <h1>Flood</h1>

  ![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
  ![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20macOS-blue)
  ![License](https://img.shields.io/badge/License-MIT-green)

  **Fast web fuzzer with recursive discovery**

  Async Rust web fuzzer for directory enumeration, virtual host discovery, and parameter fuzzing.
  100 concurrent requests by default, recursive scanning, auto-throttle on 429s, Catppuccin Mocha terminal UI.

  > **Authorization Required**: Only use Flood against systems you have explicit written permission to test.

</div>

---

## Quick Start

### Prerequisites

- Rust toolchain (stable): https://rustup.rs
- A wordlist (e.g., [SecLists](https://github.com/danielmiessler/SecLists))

### Build

```bash
git clone https://github.com/Real-Fruit-Snacks/Flood.git
cd Flood
make release
```

### Run

```bash
# Basic directory enumeration
./target/release/flood -u https://example.com/FUZZ -w /path/to/wordlist.txt

# With extensions and recursion
./target/release/flood -u https://example.com/FUZZ -w wordlist.txt -r -e php,bak,conf

# VHost discovery
./target/release/flood -u https://example.com -H "Host: FUZZ.example.com" -w subdomains.txt -fc 404
```

---

## Features

### Directory Enumeration
```bash
flood -u https://example.com/FUZZ -w common.txt
```

### Recursive Scanning
```bash
# Automatically discover and fuzz into subdirectories
flood -u https://example.com/FUZZ -w common.txt -r -e php,bak --depth 3
```

### Virtual Host Discovery
```bash
flood -u https://example.com -H "Host: FUZZ.example.com" -w subdomains.txt -fs 0
```

### Parameter Fuzzing
```bash
flood -u https://example.com/login -d "user=admin&pass=FUZZ" -w passwords.txt --filter-code 401
```

### Multiple Wordlists (Clusterbomb)
```bash
flood -u https://example.com/FUZZ/FUZ2Z -w dirs.txt -w files.txt
```

### Advanced Filtering
```bash
# Match only specific status codes
flood -u https://target.com/FUZZ -w list.txt --match-code 200,301

# Filter by response size (remove empty responses)
flood -u https://target.com/FUZZ -w list.txt --filter-size 0

# Match by regex in response body
flood -u https://target.com/FUZZ -w list.txt --match-regex "admin|dashboard"
```

### Proxy Integration
```bash
# Route all traffic through Burp Suite
flood -u https://target.com/FUZZ -w list.txt -x http://127.0.0.1:8080

# Replay only matched results to Burp
flood -u https://target.com/FUZZ -w list.txt --replay-proxy http://127.0.0.1:8080
```

### Rate Limiting
```bash
# Cap at 50 requests per second
flood -u https://target.com/FUZZ -w list.txt --rate 50

# Auto-throttle is on by default (backs off on 429s)
# Disable with:
flood -u https://target.com/FUZZ -w list.txt --no-auto-throttle
```

### Output Formats
```bash
flood -u https://target.com/FUZZ -w list.txt -o results.json --output-format json
flood -u https://target.com/FUZZ -w list.txt -o results.jsonl --output-format jsonl
flood -u https://target.com/FUZZ -w list.txt -o results.csv --output-format csv
flood -u https://target.com/FUZZ -w list.txt -o results.txt --output-format text
```

### Interactive Controls

During a scan, use these keyboard shortcuts:

| Key | Action |
|-----|--------|
| `p` | Pause / resume |
| `q` | Quit and save state |
| `+` | Increase threads by 10 |
| `-` | Decrease threads by 10 |
| `Ctrl+C` | Immediate stop |

### Pause / Resume
```bash
# Scan saves state on quit
flood -u https://target.com/FUZZ -w big-wordlist.txt
# Press q to quit...

# Resume later
flood --resume flood-state-20260407-143022.json
```

---

## Usage

```
flood [OPTIONS] -u <URL> -w <WORDLIST>
```

See `flood --help` for the complete flag reference.

---

## Build

```bash
make build              # Development build
make release            # Optimized release build
make test               # Run all tests
make lint               # Clippy lints
make fmt                # Format code
make opsec-check        # Inspect binary for metadata leaks
make release-linux      # Cross-compile for Linux (musl)
make release-windows    # Cross-compile for Windows
make release-macos      # Cross-compile for macOS
```

---

## License

MIT

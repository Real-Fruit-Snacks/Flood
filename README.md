<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Real-Fruit-Snacks/Flood/main/docs/assets/logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Real-Fruit-Snacks/Flood/main/docs/assets/logo-light.svg">
  <img alt="Flood" src="https://raw.githubusercontent.com/Real-Fruit-Snacks/Flood/main/docs/assets/logo-dark.svg" width="520">
</picture>

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

**Fast web fuzzer with recursive discovery.**

Async Rust web fuzzer for directory enumeration, virtual host discovery, and parameter fuzzing. 100 concurrent requests, recursive scanning, auto-throttle on 429s, FUZZ keyword placement, Catppuccin Mocha terminal UI.

> **Authorization Required**: Designed exclusively for authorized security testing with explicit written permission.

</div>

---

## Quick Start

**Prerequisites:** Rust toolchain (stable), a wordlist

```bash
git clone https://github.com/Real-Fruit-Snacks/Flood.git
cd Flood
make release
```

**Verify:**

```bash
./target/release/flood --help
./target/release/flood -u https://example.com/FUZZ -w wordlist.txt
```

---

## Features

### Directory Enumeration

FUZZ keyword placement in any URL position. Extensions, status code filtering, response size filtering.

```bash
flood -u https://target.com/FUZZ -w common.txt
flood -u https://target.com/FUZZ -w common.txt -r -e php,bak --depth 3
```

### Virtual Host Discovery

FUZZ in headers for subdomain enumeration. Filter by response size to remove defaults.

```bash
flood -u https://target.com -H "Host: FUZZ.target.com" -w subdomains.txt -fs 0
```

### Parameter Fuzzing

FUZZ in POST data, query strings, or headers. Filter by status code to find valid credentials.

```bash
flood -u https://target.com/login -d "user=admin&pass=FUZZ" -w passwords.txt --filter-code 401
```

### Clusterbomb Mode

Multiple wordlists with FUZZ/FUZ2Z keyword placement for combinatorial fuzzing.

```bash
flood -u https://target.com/FUZZ/FUZ2Z -w dirs.txt -w files.txt
```

### Advanced Filtering

Match or filter by status code, response size, word count, line count, or regex in response body.

```bash
flood -u https://target.com/FUZZ -w list.txt --match-code 200,301
flood -u https://target.com/FUZZ -w list.txt --match-regex "admin|dashboard"
```

### Proxy Integration

Route traffic through Burp Suite or replay matched results to a proxy.

```bash
flood -u https://target.com/FUZZ -w list.txt -x http://127.0.0.1:8080
flood -u https://target.com/FUZZ -w list.txt --replay-proxy http://127.0.0.1:8080
```

### Interactive Controls

Pause, resume, adjust threads, and save state during scans. Resume interrupted scans from saved state.

```bash
flood -u https://target.com/FUZZ -w big-wordlist.txt
# p: pause/resume | q: quit+save | +/-: threads | Ctrl+C: stop
flood --resume flood-state-20260407-143022.json
```

---

## Architecture

```
src/
├── main.rs          # CLI dispatch and config
├── fuzzer/          # Core async fuzzing engine
├── http/            # Request builder, response parser
├── filter/          # Match/filter by code, size, regex
├── output/          # JSON, JSONL, CSV, text formatters
├── throttle/        # Auto-throttle and rate limiting
├── recursive/       # Directory discovery and re-queue
└── ui/              # Catppuccin Mocha terminal interface
```

Three-layer design: CLI parses flags into a FuzzConfig, the async engine fans out requests across tokio tasks, and the filter pipeline processes responses. Auto-throttle monitors 429s and adjusts concurrency transparently.

---

## Platform Support

| | Linux | Windows | macOS |
|---|---|---|---|
| Directory Fuzzing | Full | Full | Full |
| VHost Discovery | Full | Full | Full |
| Recursive Scan | Full | Full | Full |
| Proxy Integration | Full | Full | Full |
| Interactive Controls | Full | Full | Full |
| Cross-compile | musl | MSVC | Native |

---

## Security

Report vulnerabilities via [GitHub Security Advisories](https://github.com/Real-Fruit-Snacks/Flood/security/advisories). 90-day responsible disclosure.

**Flood does not:**
- Exploit discovered endpoints (not a vulnerability scanner)
- Inject payloads or test for SQLi/XSS (not a web app scanner)
- Brute-force authentication without explicit FUZZ placement
- Evade WAF or IDS detection systems

---

## License

[MIT](LICENSE) — Copyright 2026 Real-Fruit-Snacks

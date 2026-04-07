# Changelog

All notable changes to Flood will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- Initial release
- Directory enumeration with FUZZ keyword
- Virtual host discovery via Host header fuzzing
- Parameter fuzzing (GET/POST)
- Recursive scanning with configurable depth limits
- Match/filter by status code, size, words, lines, regex, response time
- Rate limiting with auto-throttle on 429 responses
- Interactive controls (pause, quit, adjust threads)
- Pause/resume with state serialization
- Multiple wordlist support (clusterbomb mode)
- Output formats: JSON, JSONL, CSV, plain text
- Proxy and replay-proxy support (Burp Suite integration)
- HTTP Basic and Bearer token authentication
- Catppuccin Mocha terminal colors
- File extension fuzzing
- Random User-Agent rotation

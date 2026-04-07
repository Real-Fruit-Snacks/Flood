# Wordlists

Flood works with any wordlist in plain text format (one entry per line).

## Recommended Wordlists

- [SecLists](https://github.com/danielmiessler/SecLists) — The de facto standard
  - `Discovery/Web-Content/common.txt` — Quick scan (~4,700 entries)
  - `Discovery/Web-Content/directory-list-2.3-medium.txt` — Medium scan (~220,000 entries)
  - `Discovery/Web-Content/raft-large-directories.txt` — Large directory list
  - `Discovery/Web-Content/raft-large-files.txt` — Large file list
  - `Discovery/DNS/subdomains-top1million-5000.txt` — VHost discovery

## Format

One entry per line. Empty lines and lines starting with `#` are skipped.

```text
admin
login
api
# This is a comment
config
.htaccess
```

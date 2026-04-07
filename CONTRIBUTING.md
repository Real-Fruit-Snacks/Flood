# Contributing to Flood

## Development Setup

1. Install Rust (stable): https://rustup.rs
2. Clone the repository: `git clone https://github.com/Real-Fruit-Snacks/Flood.git`
3. Build: `make build`
4. Test: `make test`

## Code Style

- Format with `cargo fmt` before committing
- Pass `cargo clippy -- -D warnings` with no warnings
- Follow existing code patterns

## Testing

- All new features require tests
- All tests must pass: `make test`
- Integration tests use `wiremock` for HTTP mocking

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes with tests
4. Run `make fmt && make lint && make test`
5. Commit using Conventional Commits format:
   - `feat(scope): description`
   - `fix(scope): description`
   - `docs: description`
   - `test: description`
   - `build: description`
6. Open a PR against `main`

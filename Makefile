.PHONY: build release test clean fmt lint opsec-check release-linux release-windows release-macos

VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
BINARY := flood

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

test-unit:
	cargo test --lib

test-integration:
	cargo test --test test_integration

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean

opsec-check: release
	@echo "=== Binary size ==="
	@ls -lh target/release/$(BINARY)
	@echo ""
	@echo "=== Checking for metadata leaks ==="
	@strings target/release/$(BINARY) | grep -iE "(home|user|debug|\.rs)" | head -20 || echo "No obvious leaks found"
	@echo ""
	@echo "=== Entropy check ==="
	@file target/release/$(BINARY)

release-linux:
	cross build --release --target x86_64-unknown-linux-musl
	@ls -lh target/x86_64-unknown-linux-musl/release/$(BINARY)

release-windows:
	cross build --release --target x86_64-pc-windows-gnu
	@ls -lh target/x86_64-pc-windows-gnu/release/$(BINARY).exe

release-macos:
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	@ls -lh target/x86_64-apple-darwin/release/$(BINARY)
	@ls -lh target/aarch64-apple-darwin/release/$(BINARY)

version:
	@echo $(VERSION)

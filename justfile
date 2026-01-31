set shell := ["bash", "-uc"]

BIN_DIR := "bin"
BIN := "bin/loopr"

# Show available tasks
default:
	@just --list

# Show available tasks
help:
	@just --list

# Build release binary into bin/loopr
build: _build_release
	mkdir -p {{BIN_DIR}}
	cp target/release/loopr {{BIN}}

# Run the CLI via cargo
run:
	cargo run --

# Format Rust code
fmt:
	cargo fmt --all

# Lint with clippy
clippy:
	cargo clippy --all-targets --all-features

# Run all tests
test:
	cargo test

# Run CLI integration tests
test-cli:
	cargo test --test cli

# Run ops/unit tests
test-ops:
	cargo test --test ops

# Remove build artifacts
clean:
	rm -rf {{BIN_DIR}} target

# Install loopr into $HOME/bin
install: _build_release
	mkdir -p "$HOME/bin"
	cp target/release/loopr "$HOME/bin/loopr"

# Build release binary with version metadata
_build_release:
	@command -v cargo >/dev/null 2>&1 || { echo "Rust and cargo are required to build Loopr. Install Rust (edition 2024) and re-run just build."; exit 1; }
	LOOPR_VERSION="$(git describe --tags --dirty --always 2>/dev/null || echo dev)" \
	  LOOPR_COMMIT="$(git rev-parse --short=7 HEAD 2>/dev/null || echo "")" \
	  LOOPR_DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
	  cargo build --release

# Run CI checks locally
ci:
	just build
	just fmt
	just clippy
	just test

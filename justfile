set shell := ["bash", "-uc"]

BIN_DIR := "bin"
BIN := "{{BIN_DIR}}/loopr"

build:
	@command -v cargo >/dev/null 2>&1 || { echo "Rust and cargo are required to build Loopr. Install Rust (edition 2024) and re-run just build."; exit 1; }
	mkdir -p {{BIN_DIR}}
	LOOPR_VERSION="$(git describe --tags --dirty --always 2>/dev/null || echo dev)" \
	  LOOPR_COMMIT="$(git rev-parse --short=12 HEAD 2>/dev/null || echo "")" \
	  LOOPR_DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
	  cargo build --release
	cp target/release/loopr {{BIN}}

run:
	cargo run --

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features

test:
	cargo test

test-cli:
	cargo test --test cli

test-ops:
	cargo test --test ops

clean:
	rm -rf {{BIN_DIR}} target

ci:
	just build
	just fmt
	just clippy
	just test

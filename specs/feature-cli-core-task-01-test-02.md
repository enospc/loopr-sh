# Test: Version output includes build metadata

## Test ID
02

## Type
Manual

## Purpose
Verify `loopr version` prints version and optional commit/date lines when built with ldflags.

## Preconditions
- Go toolchain available.
- Makefile present.

## Test Data
- Command: `make build`
- Command: `./bin/loopr version`

## Steps
1. Run `make build` to build `bin/loopr` with ldflags.
2. Run `./bin/loopr version`.

## Expected Results
- Output includes `loopr <version>` (non-empty).
- If commit/date were injected, output includes `commit:` and `date:` lines.

## Automation Notes
- A Go test can invoke the built binary and assert output contains `loopr ` prefix.

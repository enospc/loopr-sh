# Test: Usage and unknown command handling

## Test ID
01

## Type
Manual

## Purpose
Verify usage output and exit codes for missing or unknown commands.

## Preconditions
- Go toolchain available.
- Run from repo root.

## Test Data
- Commands:
  - `go run ./cmd/loopr`
  - `go run ./cmd/loopr help`
  - `go run ./cmd/loopr --help`
  - `go run ./cmd/loopr not-a-command`

## Steps
1. Run `go run ./cmd/loopr` with no arguments.
2. Run `go run ./cmd/loopr help`.
3. Run `go run ./cmd/loopr --help`.
4. Run `go run ./cmd/loopr not-a-command`.

## Expected Results
- Step 1 prints usage and exits with code 2.
- Steps 2-3 print usage and exit with code 0.
- Step 4 prints a clear error and usage, exits with code 2.

## Automation Notes
- Capture exit codes in a shell test or Go integration test if desired.

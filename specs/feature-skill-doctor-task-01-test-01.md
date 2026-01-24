# Test: Doctor detects missing and drifted files

## Test ID
01

## Type
Integration

## Purpose
Verify `loopr doctor` reports missing and drifted files and returns a non-zero exit code.

## Preconditions
- Temp directory available for CODEX_HOME.
- `loopr-init` installed in skills root.

## Test Data
- Modify or delete a file under `$CODEX_HOME/skills/loopr-init/`.
- Command: `go run ./cmd/loopr doctor --only loopr-init --verbose`

## Steps
1. Install `loopr-init` into a temp skills root.
2. Delete or modify one file under `loopr-init`.
3. Run `go run ./cmd/loopr doctor --only loopr-init --verbose`.

## Expected Results
- Doctor exits with non-zero status.
- Output reports `drifted` for `loopr-init`.
- Verbose output includes the missing or drifted file path.

## Automation Notes
- Capture exit code and stdout/stderr for assertions.

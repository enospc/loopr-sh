# Test: Install writes skills into an empty skills root

## Test ID
01

## Type
Integration

## Purpose
Verify `loopr install` installs embedded skills into a clean skills root with correct directory structure.

## Preconditions
- Temp directory available for CODEX_HOME.
- Go toolchain available.

## Test Data
- CODEX_HOME set to temp dir.
- Command: `go run ./cmd/loopr install --only loopr-init`

## Steps
1. Create a temp directory and export `CODEX_HOME` to it.
2. Run `go run ./cmd/loopr install --only loopr-init`.
3. Inspect `$CODEX_HOME/skills/loopr-init` for installed files.

## Expected Results
- Install command exits successfully.
- `loopr-init` directory exists under the skills root.
- At least one expected file from embedded skills is present.

## Automation Notes
- For automated tests, read embedded skills index and assert files exist.

# Test: Doctor reports extra skills

## Test ID
02

## Type
Integration

## Purpose
Verify `loopr doctor` reports extra skills that exist locally but not in embedded skills.

## Preconditions
- Temp directory available for CODEX_HOME.

## Test Data
- Create `$CODEX_HOME/skills/loopr-extra/` directory with a dummy file.
- Command: `go run ./cmd/loopr doctor --verbose`

## Steps
1. Create a temp skills root and set CODEX_HOME.
2. Add a directory named `loopr-extra` under `$CODEX_HOME/skills/`.
3. Run `go run ./cmd/loopr doctor --verbose`.

## Expected Results
- Doctor exits with non-zero status.
- Output includes `extra` skills or lists `loopr-extra` as extra.

## Automation Notes
- Ensure `loopr-extra` matches the default `loopr-` filter.

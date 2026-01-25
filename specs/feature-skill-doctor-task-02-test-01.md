# Test: List output matches doctor status

## Test ID
01

## Type
Integration

## Purpose
Ensure `loopr list` reflects the same status as `loopr doctor`.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.

## Test Data
- A temp skills root with a mix of installed and missing skills.

## Steps
1. Run `loopr doctor` for a controlled skills root.
2. Run `loopr list` for the same skills root.
3. Compare statuses for each skill.

## Expected Results
- Status values match between doctor and list outputs.

## Automation Notes
- Can be automated by setting `CODEX_HOME` to a temp directory.

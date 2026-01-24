# Test: Codex wrapper fails without repo-id

## Test ID
02

## Type
Integration

## Purpose
Ensure `loopr codex` fails fast with a clear error when `specs/.loopr/repo-id` is missing.

## Preconditions
- Temp directory that is not a Loopr repo (no `specs/.loopr/repo-id`).

## Test Data
- Command: `go run ./cmd/loopr codex -- --help`

## Steps
1. Create a temp directory without `specs/.loopr/repo-id`.
2. Run `go run ./cmd/loopr codex -- --help` from that directory.

## Expected Results
- Command exits with non-zero status.
- Error output mentions missing `specs/.loopr/repo-id` and suggests running `loopr-init`.

## Automation Notes
- Use a temp working directory to avoid existing repo state.

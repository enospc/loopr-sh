# Test: Codex wrapper fails without repo-id

## Test ID
02

## Type
Integration

## Purpose
Ensure `loopr codex` fails with a clear error when `specs/.loopr/repo-id` is missing.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- A directory without `specs/.loopr/repo-id`.

## Test Data
- Example Codex args such as `--help`.

## Steps
1. Run `loopr codex -- --help` from a directory without `specs/.loopr/repo-id`.

## Expected Results
- Command exits non-zero.
- Error message instructs to run `loopr init`.

## Automation Notes
- Use a temp directory without `specs/.loopr/` for automation.

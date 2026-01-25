# Test: Codex wrapper fails without repo-id

## Test ID
02

## Type
Integration

## Purpose
Ensure `loopr codex` exits with a clear error when no Loopr repo-id is found.

## Preconditions
- No `specs/.loopr/repo-id` exists in the current or parent directories.

## Test Data
- Command: `loopr codex -- --help`.

## Steps
1. Run the command from a directory without a Loopr workspace.

## Expected Results
- Exit code is non-zero.
- Error message mentions missing `specs/.loopr/repo-id` and suggests running `loopr-init`.

## Automation Notes
- Use a temp directory outside any Loopr workspace to avoid false positives.

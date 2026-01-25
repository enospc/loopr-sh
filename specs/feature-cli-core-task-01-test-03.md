# Test: Codex passthrough respects -- delimiter

## Test ID
03

## Type
Integration

## Purpose
Confirm arguments after `--` are passed to Codex unchanged.

## Preconditions
- A stub `codex` binary is available in PATH that records its argv.

## Test Data
- Command: `loopr codex -- --help`.

## Steps
1. Place a stub `codex` executable in PATH that writes its argv to a temp file.
2. Run `loopr codex -- --help`.
3. Inspect the stub's recorded argv.

## Expected Results
- The stub sees `--help` as the first argument.
- No `loopr` flags appear after the `--` delimiter.

## Automation Notes
- A shell script stub can write "$@" to a file for assertions.

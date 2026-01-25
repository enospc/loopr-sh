# Test: Usage and unknown command handling

## Test ID
01

## Type
Integration

## Purpose
Verify usage output and exit codes for missing or unknown commands.

## Preconditions
- `loopr` binary is available (built or `go run`).

## Test Data
- Commands: `loopr`, `loopr help`, `loopr nosuch`.

## Steps
1. Run `loopr` with no arguments.
2. Run `loopr help`.
3. Run `loopr nosuch`.

## Expected Results
- Step 1: usage is printed and exit code is 2.
- Step 2: usage is printed and exit code is 0.
- Step 3: an error plus usage is printed and exit code is 2.

## Automation Notes
- Capture stdout/stderr to assert usage text appears.

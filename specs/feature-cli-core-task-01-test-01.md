# Test: Usage and unknown command handling

## Test ID
01

## Type
Integration

## Purpose
Confirm usage output and exit codes for missing or unknown commands.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.

## Test Data
- None.

## Steps
1. Run `loopr` with no arguments.
2. Run `loopr help`.
3. Run `loopr unknown`.

## Expected Results
- Step 1 prints usage and exits with code 2.
- Step 2 prints usage and exits successfully.
- Step 3 prints an error plus usage and exits with code 2.

## Automation Notes
- Can be automated by invoking the CLI and checking exit codes/output.

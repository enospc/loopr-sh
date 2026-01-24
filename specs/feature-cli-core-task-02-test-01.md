# Test: Only-list parsing trims and ignores empty values

## Test ID
01

## Type
Unit

## Purpose
Ensure `--only` parsing returns a stable list of skill names with whitespace trimmed and empties removed.

## Preconditions
- Unit test harness available in Go.

## Test Data
- Input: "loopr-init, ,loopr-prd,,loopr-specify"
- Expected: ["loopr-init", "loopr-prd", "loopr-specify"]

## Steps
1. Invoke the shared `--only` parsing helper with the input string.
2. Capture the returned slice.

## Expected Results
- Returned slice matches expected order and excludes empty entries.

## Automation Notes
- Prefer a unit test for the helper function to avoid CLI invocation.

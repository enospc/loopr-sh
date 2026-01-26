# Test: Only-list parsing trims and ignores empty values

## Test ID
01

## Type
Unit

## Purpose
Verify `--only` parsing trims whitespace and ignores empty entries.

## Preconditions
- None.

## Test Data
- Example input: "loopr-prd,, loopr-specify , ,loopr-features".

## Steps
1. Call the `--only` parsing helper with the example input.
2. Inspect the returned slice.

## Expected Results
- Output contains `loopr-prd`, `loopr-specify`, and `loopr-features` in order.
- Empty entries are discarded.

## Automation Notes
- Implement as a unit test for the helper function.

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
- Example input: "loopr-init,, loopr-prd , ,loopr-specify".

## Steps
1. Call the `--only` parsing helper with the example input.
2. Inspect the returned slice.

## Expected Results
- Output contains `loopr-init`, `loopr-prd`, and `loopr-specify` in order.
- Empty entries are discarded.

## Automation Notes
- Implement as a unit test for the helper function.

# Test: Doctor reports extra skills

## Test ID
02

## Type
Unit

## Purpose
Verify doctor reports extra installed skills not present in embedded list.

## Preconditions
- None.

## Test Data
- A manually created extra skill directory.

## Steps
1. Create an extra `loopr-*` skill directory in the skills root.
2. Run doctor.

## Expected Results
- Doctor reports the extra skill in the extra list.

## Automation Notes
- Use temp directories and manual skill creation in unit tests.

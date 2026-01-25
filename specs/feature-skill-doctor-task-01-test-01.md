# Test: Doctor detects missing and drifted files

## Test ID
01

## Type
Unit

## Purpose
Ensure doctor reports missing skills and drifted files correctly.

## Preconditions
- None.

## Test Data
- Embedded test skill with a README.

## Steps
1. Run doctor with an empty skills root.
2. Install skills and modify one file.
3. Run doctor again.

## Expected Results
- Step 1 reports the skill as missing.
- Step 3 reports the skill as drifted and includes the modified file.

## Automation Notes
- Use temp directories and a test embedded filesystem.

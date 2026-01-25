# Test: Install writes skills into an empty skills root

## Test ID
01

## Type
Unit

## Purpose
Ensure install writes embedded skills into a fresh skills root and preserves script mode.

## Preconditions
- None.

## Test Data
- Embedded test skill with a README and a script.

## Steps
1. Create a temp skills root.
2. Run install against the temp root.
3. Verify files exist and script is executable.

## Expected Results
- Skill directory exists with expected files.
- Script file has executable permissions.

## Automation Notes
- Use a test embedded filesystem and temp directories.

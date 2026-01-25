# Test: Install backs up modified skills before update

## Test ID
02

## Type
Unit

## Purpose
Verify install backs up modified skills before overwriting.

## Preconditions
- None.

## Test Data
- Embedded test skill with a README.

## Steps
1. Install skills into a temp root.
2. Modify a file in the installed skill.
3. Run install again.
4. Inspect the backup directory.

## Expected Results
- Backup directory is created under `.backup/loopr-<timestamp>/`.
- Modified file contents appear in the backup.

## Automation Notes
- Use temp directories and check backup contents in the unit test.

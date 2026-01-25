# Test: Uninstall removes skills and creates backup

## Test ID
01

## Type
Unit

## Purpose
Verify uninstall removes skills and creates backups by default.

## Preconditions
- Skills installed in a temp root.

## Test Data
- Embedded test skill.

## Steps
1. Install skills into a temp root.
2. Run uninstall without `--force`.
3. Check that skills are removed and backups exist.

## Expected Results
- Skill directory is removed.
- Backup directory exists and contains the removed skill.

## Automation Notes
- Use temp directories and verify filesystem state in unit tests.

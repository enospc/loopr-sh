# Test: Uninstall --force removes skills without backup

## Test ID
02

## Type
Unit

## Purpose
Verify `--force` uninstall removes skills without creating backups.

## Preconditions
- Skills installed in a temp root.

## Test Data
- Embedded test skill.

## Steps
1. Install skills into a temp root.
2. Run uninstall with `--force`.
3. Check that skills are removed and no backup directory exists.

## Expected Results
- Skill directory is removed.
- `.backup/` is not created.

## Automation Notes
- Use temp directories and verify filesystem state in unit tests.

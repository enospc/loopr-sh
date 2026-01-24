# Test: Uninstall removes skills and creates backup

## Test ID
01

## Type
Integration

## Purpose
Verify `loopr uninstall` removes targeted skills and creates a backup by default.

## Preconditions
- Temp directory available for CODEX_HOME.
- `loopr-init` installed in skills root.

## Test Data
- Command: `go run ./cmd/loopr uninstall --only loopr-init`

## Steps
1. Install `loopr-init` into a temp skills root.
2. Run `go run ./cmd/loopr uninstall --only loopr-init`.
3. Check that `$CODEX_HOME/skills/loopr-init` is removed.
4. Check `.backup/loopr-<timestamp>/loopr-init` exists.

## Expected Results
- Uninstall command exits successfully.
- Skill directory is removed.
- Backup directory exists with the removed skill contents.

## Automation Notes
- Ensure backup directory is created even when only one skill is removed.

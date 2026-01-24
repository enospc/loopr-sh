# Test: Uninstall --force removes skills without backup

## Test ID
02

## Type
Integration

## Purpose
Ensure `--force` bypasses backups and still removes the targeted skills.

## Preconditions
- Temp directory available for CODEX_HOME.
- `loopr-init` installed in skills root.

## Test Data
- Command: `go run ./cmd/loopr uninstall --only loopr-init --force`

## Steps
1. Install `loopr-init` into a temp skills root.
2. Run `go run ./cmd/loopr uninstall --only loopr-init --force`.
3. Check that `$CODEX_HOME/skills/loopr-init` is removed.
4. Verify no new backup directory is created for this uninstall.

## Expected Results
- Uninstall command exits successfully.
- Skill directory is removed.
- No backup directory is created for the forced uninstall.

## Automation Notes
- When automating, record the backup directory listing before and after the command.

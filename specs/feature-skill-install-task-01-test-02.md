# Test: Install backs up modified skills before update

## Test ID
02

## Type
Integration

## Purpose
Ensure backups are created when installing over modified skills.

## Preconditions
- Temp directory available for CODEX_HOME.
- Go toolchain available.
- `loopr-init` already installed under the skills root.

## Test Data
- Modify a file under `$CODEX_HOME/skills/loopr-init/`.
- Command: `go run ./cmd/loopr install --only loopr-init`

## Steps
1. Install `loopr-init` into a temp skills root.
2. Modify one file under `$CODEX_HOME/skills/loopr-init/`.
3. Re-run install for `loopr-init`.
4. Inspect `$CODEX_HOME/skills/.backup/` for a `loopr-<timestamp>/loopr-init` backup.

## Expected Results
- Install command exits successfully.
- A backup directory is created containing the modified skill.
- The modified file is restored to the embedded version.

## Automation Notes
- Use filesystem checks to confirm backup and restored content hashes.

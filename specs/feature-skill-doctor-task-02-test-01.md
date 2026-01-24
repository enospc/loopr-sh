# Test: List output matches doctor status

## Test ID
01

## Type
Integration

## Purpose
Ensure `loopr list` reports the same status as `loopr doctor` for the same skills.

## Preconditions
- Temp directory available for CODEX_HOME.
- `loopr-init` installed in skills root.

## Test Data
- Modify a file under `$CODEX_HOME/skills/loopr-init/`.
- Commands:
  - `go run ./cmd/loopr doctor --only loopr-init`
  - `go run ./cmd/loopr list --only loopr-init`

## Steps
1. Install `loopr-init` into a temp skills root.
2. Modify a file under `loopr-init`.
3. Run `go run ./cmd/loopr doctor --only loopr-init` and note the status.
4. Run `go run ./cmd/loopr list --only loopr-init`.

## Expected Results
- Doctor reports `drifted` for `loopr-init`.
- List output shows `loopr-init` with status `drifted`.

## Automation Notes
- Parse both outputs and assert the status strings match.

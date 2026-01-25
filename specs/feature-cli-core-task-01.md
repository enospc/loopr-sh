# Task: CLI command surface and version info / Command routing and usage

## Task ID
01

## Summary
Implement top-level command routing, usage output, and Codex argument passthrough for the `loopr` CLI.

## Goal
Provide a predictable command surface with clear help/usage, correct exit codes, and safe `--` handling for `loopr codex`.

## Scope
- In scope:
  - Command switch for install/doctor/list/uninstall/codex/version/help.
  - Usage output and error messages for unknown commands or missing args.
  - Exit code conventions (usage errors vs success).
  - Respect the `--` delimiter so arguments after it are passed to Codex unchanged.
- Out of scope:
  - The underlying behavior of install/doctor/uninstall/codex operations.
  - Validation of Codex arguments.

## Acceptance Criteria
- Running `loopr` with no arguments prints usage and exits with code 2.
- Running `loopr help`, `loopr -h`, or `loopr --help` prints usage and exits successfully.
- Unknown commands print a clear error plus usage and exit with code 2.
- `loopr version` prints version metadata from `internal/version`.
- `loopr codex -- --help` passes `--help` through to the Codex runner unchanged.

## Implementation Plan
- Define a `usage()` function that prints a concise command list.
- Add a command switch in `cmd/loopr/main.go` with explicit cases and a default error path.
- Split args on `--` and pass the trailing args through to the codex handler without modification.
- Ensure `version` uses the build metadata values (version/commit/date) when available.

## Dependencies
- None.

## Risks
- Inconsistent exit codes could break scripts; keep usage errors on exit code 2.

## Test Plan
- Manual: run `loopr`, `loopr help`, and an unknown command to verify output and exit codes.
- Integration: run `loopr codex -- --help` and confirm Codex sees `--help`.

## Notes
- Keep CLI output concise; avoid noisy logs for normal usage.

## Completion
- Status: Done
- Tests: `go test ./...` and manual CLI checks for usage/help/unknown/version, plus `loopr codex -- --help` passthrough.
- Notes: Verified exit codes for usage/help/unknown.

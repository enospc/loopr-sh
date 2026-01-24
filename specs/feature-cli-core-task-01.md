# Task: CLI command surface and version info / Command routing and usage

## Task ID
01

## Summary
Implement the top-level CLI command routing, usage output, and error handling for the `loopr` binary.

## Goal
Provide a predictable command surface with clear help/usage and correct exit codes for invalid invocations.

## Scope
- In scope:
  - Command switch for install/doctor/list/uninstall/codex/version/help.
  - Usage output and error messages for unknown commands or missing args.
  - Exit code conventions (usage errors vs success).
- Out of scope:
  - The underlying behavior of install/doctor/uninstall/codex operations.

## Acceptance Criteria
- Running `loopr` with no arguments prints usage and exits with code 2.
- Running `loopr help`, `loopr -h`, or `loopr --help` prints usage and exits successfully.
- Unknown commands print a clear error plus usage and exit with code 2.
- `loopr version` prints version metadata from `internal/version`.

## Implementation Plan
- Define a `usage()` function that prints a concise command list.
- Add a command switch in `cmd/loopr/main.go` with explicit cases and a default error path.
- Ensure `version` uses the build metadata values (version/commit/date) when available.

## Dependencies
- None.

## Risks
- Inconsistent exit codes could break scripts; keep usage errors on exit code 2.

## Test Plan
- Manual: run `loopr`, `loopr help`, and an unknown command to verify output and exit codes.
- Unit: optional tests for usage output formatting (if a test harness exists).

## Notes
- Keep CLI output concise; avoid noisy logs for normal usage.

---
order: 2
depends_on: []
---

# Feature: CLI command surface and version info

## Summary
Provide the `loopr` CLI interface with consistent command routing, shared flag parsing, help/usage output, and version reporting.

## Goals
- Make the CLI predictable and stable.
- Provide clear usage and error messaging.
- Expose build metadata via `loopr version`.

## Non-goals
- Implementing install/doctor/uninstall/codex behaviors (handled by other features).

## User Stories
- As a developer, I want a clear CLI command surface so that I can run Loopr tasks reliably.
- As a developer, I want version metadata so that I can debug issues against a specific build.

## Scope
- In scope:
  - Command routing for install/doctor/list/uninstall/codex/version/help.
  - Parsing global flags: `--agent`, `--all`, `--only`, `--force`, `--verbose`.
  - Respecting the `--` delimiter so Codex args are passed through untouched.
  - Usage output and error handling for unknown commands.
- Out of scope:
  - File operations, skill management logic, or transcript capture.

## Requirements
- Parse commands and route to the appropriate operation function.
- Provide usage text when called with `-h`, `--help`, or `help`.
- Exit with non-zero status for unknown commands or flag parsing errors.
- Default `--agent` to `codex`; support `--all` where applicable.
- Parse `--only` as a comma-separated list and drop empty entries.
- Leave arguments after `--` untouched for `loopr codex`.
- Print version, commit, and date when available via build metadata.

## Acceptance Criteria
- Running `loopr` with no args prints usage and exits with code 2.
- Running `loopr help` prints usage and exits successfully.
- Running `loopr version` prints version and includes commit/date when set.
- `loopr codex -- --help` passes `--help` through to Codex.

## UX / Flow
- `loopr <command> [flags]` is the standard invocation.
- Usage text lists available commands and a short description.

## Data / API Impact
- CLI flags: `--agent`, `--all`, `--only`, `--force`, `--verbose`.

## Dependencies
- None.

## Risks & Mitigations
- Risk: inconsistent flags across commands → Mitigation: shared parsing helpers.

## Open Questions
- Should agent discovery expand beyond Codex in the near term?

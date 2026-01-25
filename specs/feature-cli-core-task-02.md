# Task: CLI command surface and version info / Flag parsing helpers

## Task ID
02

## Summary
Add shared helpers for agent resolution and `--only` parsing to keep command flags consistent.

## Goal
Provide reliable parsing for `--agent`, `--all`, and `--only` across commands that operate on skills.

## Scope
- In scope:
  - Parse `--agent` with default `codex`.
  - Support `--all` to target all supported agents.
  - Parse `--only` as a comma-separated list, trimming whitespace.
- Out of scope:
  - Validation of unknown agent names beyond existing resolver logic.

## Acceptance Criteria
- `--only` ignores empty values and returns a stable slice of skill names.
- `--all` returns the full supported agent list; `--agent` returns the named agent.
- Flag parsing failures exit with code 2.

## Implementation Plan
- Introduce or reuse helper functions for agent resolution and list parsing.
- Use shared helpers in install/doctor/list/uninstall command handlers.

## Dependencies
- CLI command routing in Task 01.

## Risks
- Divergent flag behavior across commands; use helpers to keep consistency.

## Test Plan
- Unit: tests for `--only` parsing and agent resolution behaviors.
- Manual: run `loopr install --only loopr-init` to verify parsing.

## Notes
- Keep helpers small and reusable.

## Completion
- Status: Done
- Tests: go test ./...
- Notes: Added unit coverage for `splitList` and `resolveAgents`.

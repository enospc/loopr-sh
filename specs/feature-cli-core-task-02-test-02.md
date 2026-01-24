# Test: Agent resolution default and --all behavior

## Test ID
02

## Type
Unit

## Purpose
Validate agent resolution returns default `codex`, supports `--all`, and rejects unknown agents.

## Preconditions
- Unit test harness available in Go.

## Test Data
- Default: no `--agent` flag set
- All: `--all` set
- Invalid: `--agent not-real`

## Steps
1. Run helper with no `--agent` flag and `--all` false.
2. Run helper with `--all` true.
3. Run helper with `--agent not-real`.

## Expected Results
- Step 1 returns the `codex` agent spec.
- Step 2 returns all supported agent specs (non-empty).
- Step 3 returns an error for unsupported agent.

## Automation Notes
- Use a flagset in tests to simulate CLI parsing.

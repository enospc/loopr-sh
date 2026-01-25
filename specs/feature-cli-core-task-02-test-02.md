# Test: Agent resolution default and --all behavior

## Test ID
02

## Type
Unit

## Purpose
Verify agent resolution defaults to Codex and `--all` returns all supported agents.

## Preconditions
- None.

## Test Data
- Agent name: "codex".
- `--all` flag set to true.

## Steps
1. Resolve agents with default agent name and `--all=false`.
2. Resolve agents with `--all=true`.

## Expected Results
- Step 1 returns a single Codex spec.
- Step 2 returns the full supported agent list.

## Automation Notes
- Implement as unit tests for the resolver helper.

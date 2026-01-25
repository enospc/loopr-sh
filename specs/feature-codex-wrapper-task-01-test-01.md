# Test: Codex wrapper writes transcript and metadata

## Test ID
01

## Type
Integration

## Purpose
Verify that `loopr codex` writes transcript and JSONL metadata under the nearest Loopr workspace.

## Preconditions
- A workspace exists with `specs/.loopr/repo-id`.
- A stub `codex` binary is available in PATH to avoid invoking the real Codex.

## Test Data
- Command: `loopr codex -- --help`.

## Steps
1. Create a temp workspace with `specs/.loopr/repo-id`.
2. Run `loopr codex -- --help` from a nested directory inside the workspace.
3. Locate `specs/.loopr/transcripts/<repo-id>/`.
4. Inspect the latest `session-*.log` and `session-*.jsonl` files.

## Expected Results
- Both transcript and JSONL files exist under the workspace.
- JSONL includes `start` and `end` events with timestamps and an exit code.

## Automation Notes
- The stub codex can print a known string to make log assertions deterministic.

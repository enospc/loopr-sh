# Test: Codex wrapper writes transcript and reproducibility metadata

## Test ID
01

## Type
Integration

## Purpose
Verify `loopr codex` writes transcript and JSONL metadata files under the Loopr workspace with required reproducibility fields.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- A Loopr workspace with `specs/.loopr/repo-id` present.
- Codex CLI installed, or a stub `codex` script on PATH.

## Test Data
- Example Codex args such as `--help`.

## Steps
1. Run `loopr codex -- --help` from within the workspace.
2. Inspect `specs/.loopr/transcripts/<repo-id>/`.
3. Inspect JSONL `start` event for required reproducibility fields.

## Expected Results
- A new `session-*.log` and `session-*.jsonl` are created.
- JSONL includes `start` and `end` events with timestamps and exit code.
- JSONL `start` event includes required reproducibility fields.

## Automation Notes
- Use a stub `codex` binary for deterministic runs in CI.

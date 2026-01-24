# Task: Codex wrapper transcript logging / Transcript capture wrapper

## Task ID
01

## Summary
Wrap Codex runs to capture transcripts and JSONL metadata in the Loopr workspace.

## Goal
Ensure each Codex session is logged with minimal metadata for traceability.

## Scope
- In scope:
  - Locate repo root by finding `specs/.loopr/repo-id`.
  - Create transcript directory under `specs/.loopr/transcripts/<repo-id>/`.
  - Write `session-<timestamp>.log` and `session-<timestamp>.jsonl`.
  - Use `script` when available; fall back to tee stdout/stderr.
  - Preserve Codex exit code.
- Out of scope:
  - Log rotation or remote shipping.

## Acceptance Criteria
- `loopr codex -- <args>` writes a log and metadata file for each run.
- Metadata includes start timestamp, end timestamp, and exit code.
- The wrapper prints transcript and metadata paths on completion.

## Implementation Plan
- Resolve repo root and repo-id; error if missing.
- Create transcript directory and filenames using UTC timestamps.
- Write JSONL start record before running Codex.
- Execute Codex via `script` if present; otherwise tee stdout/stderr to log.
- Write JSONL end record after completion and return exit code.

## Dependencies
- CLI command routing (cli-core task 01).

## Risks
- Missing `script` utility → fallback to tee.
- Missing repo-id → fail fast with clear error.

## Test Plan
- Manual: run `loopr codex -- --help` and verify transcript artifacts.
- Unit: mock process execution to validate metadata records.

## Notes
- Keep metadata minimal and append-only JSONL.

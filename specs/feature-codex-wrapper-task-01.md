# Task: Codex wrapper transcript logging / Transcript capture wrapper

## Task ID
01

## Summary
Wrap Codex runs to capture transcripts and JSONL metadata in the Loopr workspace, including monorepo workspace resolution.

## Goal
Ensure each Codex session is logged with minimal metadata for traceability in the correct Loopr workspace.

## Scope
- In scope:
  - Resolve the Loopr workspace root with `--loopr-root`, `LOOPR_ROOT`, or nearest ancestor search.
  - Locate `specs/.loopr/repo-id` and fail fast if missing.
  - Create transcript directory under `specs/.loopr/transcripts/<repo-id>/`.
  - Write `session-<timestamp>.log` and `session-<timestamp>.jsonl`.
  - Use `script` when available; fall back to tee stdout/stderr.
  - Preserve Codex exit code.
- Out of scope:
  - Log rotation or remote shipping.

## Acceptance Criteria
- `loopr codex -- <args>` writes a log and metadata file for each run under the nearest workspace.
- `loopr codex --loopr-root <path> -- <args>` writes artifacts under the specified workspace.
- `LOOPR_ROOT=<path> loopr codex -- <args>` writes artifacts under the specified workspace.
- Missing `specs/.loopr/repo-id` yields a clear error and non-zero exit.
- Metadata includes start timestamp, end timestamp, and exit code.
- The wrapper prints transcript and metadata paths on completion.

## Implementation Plan
- Resolve workspace root with precedence: `--loopr-root` > `LOOPR_ROOT` > nearest ancestor search.
- Validate `specs/.loopr/repo-id` exists under the chosen root.
- Create transcript directory and filenames using UTC timestamps.
- Write JSONL start record before running Codex.
- Execute Codex via `script` if present; otherwise tee stdout/stderr to log.
- Write JSONL end record after completion and return exit code.

## Dependencies
- CLI command routing (cli-core task 01).

## Risks
- Missing `script` utility → fallback to tee.
- Incorrect workspace selection in monorepos → allow explicit override.

## Test Plan
- Integration: run `loopr codex -- --help` and verify transcript artifacts in nearest workspace.
- Integration: run with `--loopr-root` and `LOOPR_ROOT` to verify override behavior.
- Unit: mock process execution to validate metadata records.

## Notes
- Keep metadata minimal and append-only JSONL.

## Completion
- Status: Done
- Tests: go test ./...
- Notes: Added workspace override resolution with `--loopr-root` and `LOOPR_ROOT`.

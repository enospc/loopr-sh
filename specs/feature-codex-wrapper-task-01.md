# Task: Codex wrapper transcript logging / Transcript capture wrapper

## Task ID
01

## Summary
Wrap Codex runs to capture transcripts and JSONL metadata in the Loopr workspace, including monorepo workspace resolution and reproducibility fields.

## Goal
Ensure each Codex session is logged with reproducibility metadata in the correct Loopr workspace.

## Scope
- In scope:
  - Resolve the Loopr workspace root with `--loopr-root`, `LOOPR_ROOT`, or nearest ancestor search.
  - Locate `specs/.loopr/repo-id` and fail fast if missing.
  - Create transcript directory under `specs/.loopr/transcripts/<repo-id>/`.
  - Write `session-<timestamp>.log` and `session-<timestamp>.jsonl`.
  - Use `script` when available; fall back to tee stdout/stderr.
  - Preserve Codex exit code.
  - Include reproducibility metadata in the JSONL `start` event.
- Out of scope:
  - Log rotation or remote shipping.
  - Capturing prompt data unless explicitly provided via env vars.

## Acceptance Criteria
- `loopr run --codex -- <args>` writes a log and metadata file for each run under the nearest workspace.
- `loopr run --codex --loopr-root <path> -- <args>` writes artifacts under the specified workspace.
- `LOOPR_ROOT=<path> loopr run --codex -- <args>` writes artifacts under the specified workspace.
- Missing `specs/.loopr/repo-id` yields a clear error and non-zero exit.
- Metadata includes start timestamp, end timestamp, and exit code.
- `start` event includes required reproducibility fields and optional fields when available.

## Implementation Plan
- Resolve workspace root with precedence: `--loopr-root` > `LOOPR_ROOT` > nearest ancestor search.
- Validate `specs/.loopr/repo-id` exists under the chosen root.
- Create transcript directory and filenames using UTC timestamps.
- Gather reproducibility metadata:
  - loopr version/commit/date from build metadata.
  - repo root/id and current working directory.
  - command array for codex invocation.
  - embedded skills hash snapshot and optional installed skills hash snapshot.
  - optional `codex_model` and `codex_prompt` from env vars.
- Write JSONL start record before running Codex.
- Execute Codex via `script` if present; otherwise tee stdout/stderr to log.
- Write JSONL end record after completion and return exit code.

## Dependencies
- CLI command routing (cli-core task 01).

## Risks
- Missing `script` utility → fallback to tee.
- Incorrect workspace selection in monorepos → allow explicit override.
- Prompt metadata may contain sensitive content → capture only when explicitly provided.

## Test Plan
- Integration: run `loopr run --codex --step execute -- --help` and verify transcript artifacts and reproducibility fields in JSONL.
- Integration: run with `--loopr-root` and `LOOPR_ROOT` to verify override behavior.
- Unit: verify reproducibility fields are present in start event and optional fields are conditional.

## Notes
- Keep metadata minimal and append-only JSONL.

## Completion
- Status: Done
- Tests: `go test ./...` and manual `loopr run --codex --step execute -- --help` runs verifying reproducibility fields (with and without `LOOPR_CODEX_MODEL`/`LOOPR_CODEX_PROMPT`).
- Notes: Optional fields only appear when env vars are set; git commit/dirty and skills hash fields captured when available.

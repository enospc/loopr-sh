---
order: 6
depends_on:
  - cli-core
---

# Feature: Codex wrapper transcript logging

## Summary
Wrap Codex execution and capture transcripts + reproducibility metadata under the Loopr workspace, with monorepo-aware workspace resolution.

## Goals
- Capture Codex transcripts and reproducibility metadata reliably.
- Resolve the correct Loopr workspace when running inside a monorepo.
- Keep transcript handling local, deterministic, and safe.

## Non-goals
- Running the Loopr workflow beyond Codex wrapping.
- Altering Codex behavior beyond argument passthrough.

## User Stories
- As a developer, I want Codex transcripts saved under my Loopr workspace for traceability.
- As a developer working in a monorepo, I want to choose or auto-detect the correct Loopr workspace.

## Scope
- In scope:
  - `loopr run --codex -- <args>` command that wraps Codex execution.
  - Workspace resolution by searching upward for `specs/.loopr/repo-id`.
  - Optional explicit workspace selection for monorepos.
  - Transcript log and JSONL metadata creation.
  - `script`-based capture when available with a tee fallback.
- Out of scope:
  - Post-processing transcript data.
  - Network or remote storage of transcripts.

## Requirements
- Resolve Loopr workspace root:
  - If `--loopr-root <path>` is provided, use it and require `specs/.loopr/repo-id` under that root.
  - Otherwise, search upward from the current directory for the nearest `specs/.loopr/repo-id`.
  - If not found, exit non-zero with a hint to run `loopr init`.
- Create `specs/.loopr/transcripts/<repo-id>/` if missing.
- Write session artifacts without overwriting existing files:
  - `session-<timestamp>.log` (raw transcript)
  - `session-<timestamp>.jsonl` (start/end metadata)
- JSONL metadata must include `start` and `end` events with timestamp and exit code.
- JSONL `start` event includes reproducibility fields:
  - Required: `loopr_version`, `loopr_commit`, `loopr_date`, `repo_root`, `repo_id`, `cwd`, `cmd`, `skills_embedded_hash`.
  - Optional when available: `git_commit`, `git_dirty`, `skills_installed_hash`.
- If `script` is available, use it to capture the session; otherwise tee stdout/stderr into the log file.
- Pass arguments after `--` directly to `codex` without modification.

## Acceptance Criteria
- Running `loopr run --codex -- <args>` from a nested directory stores transcripts under the nearest workspace.
- Running `loopr run --codex --loopr-root <path> -- <args>` stores transcripts under the specified workspace.
- Missing `specs/.loopr/repo-id` yields a clear error and non-zero exit.
- JSONL includes both `start` and `end` events with timestamps and exit code.
- JSONL `start` event includes the required reproducibility fields and optional fields when available.

## UX / Flow
- `loopr run --codex -- <args>` runs Codex and prints transcript/metadata paths.
- `loopr run --codex --loopr-root <path> -- <args>` targets a specific workspace in a monorepo.

## Data / API Impact
- CLI flag: `--loopr-root` (codex command only).

## Dependencies
- CLI core for command parsing and flag handling.

## Risks & Mitigations
- Risk: ambiguous workspace selection in monorepos → Mitigation: `--loopr-root` override.
- Risk: `script` not available → Mitigation: tee fallback.

## Open Questions
- Should `--loopr-root` be supported by other commands?

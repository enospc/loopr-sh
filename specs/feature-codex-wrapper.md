---
order: 4
depends_on:
  - cli-core
---

# Feature: Codex wrapper transcript logging

## Summary
Wrap Codex runs to capture transcripts and metadata under the Loopr workspace for traceability.

## Goals
- Capture Codex session logs and metadata with minimal overhead.
- Ensure logs are stored under `specs/.loopr/transcripts/<repo-id>/`.

## Non-goals
- Modifying Codex behavior beyond stdout/stderr capture.
- Providing live streaming analytics or remote storage.

## User Stories
- As a developer, I want a transcript of my Codex run so that I can audit and share what happened.
- As a developer, I want metadata about the session for traceability.

## Scope
- In scope:
  - Locate repo root by finding `specs/.loopr/repo-id`.
  - Create transcript directory when missing.
  - Write `session-<timestamp>.log` and `session-<timestamp>.jsonl`.
  - Use `script` when available, otherwise tee stdout/stderr.
- Out of scope:
  - Custom log formats or remote log shipping.

## Requirements
- Fail fast if `specs/.loopr/repo-id` is missing.
- Write JSONL metadata with `start` and `end` events.
- Preserve Codex exit code and return it from the wrapper.
- Print transcript and metadata paths on completion.

## Acceptance Criteria
- `loopr codex -- <args>` creates a log file and metadata file for each run.
- Metadata includes start timestamp, end timestamp, and exit code.
- If `script` is not available, logs still capture stdout/stderr.

## UX / Flow
- `loopr codex -- <args>` prints transcript and metadata paths.

## Data / API Impact
- Writes to `specs/.loopr/transcripts/<repo-id>/`.

## Dependencies
- CLI command parsing and repo-id discovery.

## Risks & Mitigations
- Risk: `script` not installed → Mitigation: fallback to stdout/stderr tee.
- Risk: repo-id missing → Mitigation: explicit error advising loopr-init.

## Open Questions
- Should users be able to override transcript paths?

---
name: loopr-init
description: Initialize a Loopr repo with a persistent repo-id and transcript logging under specs/.loopr. Use as a prerequisite for all loopr-* skills to ensure consistent session logging and correlation across runs.
---

## Overview
Create or reuse a stable repo-id and set up transcript logging under specs/.loopr. This skill is idempotent and safe to run multiple times.

## Scope (Greenfield only)
This workflow is intended for a brand-new, empty repo. Stop and ask for confirmation if the repo already contains application code or build tooling (examples: `package.json`, `go.mod`, `Cargo.toml`, `pyproject.toml`, `pom.xml`, `build.gradle`, `src/`, `app/`, `backend/`, `frontend/`). If `specs/.loopr/repo-id` already exists, treat the repo as Loopr-managed and proceed.

## Workflow
0. Preflight: verify the repo is greenfield per the scope above.
1. Ensure `specs/.loopr/` exists.
2. Check for `specs/.loopr/repo-id`.
   - If present: read and reuse the repo-id.
   - If missing: generate a new 6-character lowercase ID (nanoid-style) and persist it to `specs/.loopr/repo-id`.
3. Ensure transcript directory exists: `specs/.loopr/transcripts/<repo-id>/`.
4. Create a new session log file path (do not overwrite):
   - `specs/.loopr/transcripts/<repo-id>/session-<timestamp>.jsonl`
5. Export environment variables for the current session:
   - `LOOPR_REPO_ID=<repo-id>`
   - `LOOPR_TRANSCRIPT=<session-log-path>`
6. If desired, append a short entry to `specs/.loopr/index.md` indicating the new session log path.

## Output requirements
- Idempotent: re-running does not change an existing repo-id.
- Repo-id must be 6 lowercase alphanumeric characters.
- Transcript logs must live under `specs/.loopr/transcripts/<repo-id>/`.

## Version
- 2026-01-24

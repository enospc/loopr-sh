---
name: loopr-init
description: Initialize a Loopr repo with a persistent repo-id and transcript logging under specs/.loopr. Use as a prerequisite for all loopr-* skills to ensure consistent session logging and correlation across runs.
---

## Overview
Create or reuse a stable repo-id and set up transcript logging under specs/.loopr. This skill is idempotent and safe to run multiple times.

## Scope (Greenfield by default)
This workflow is intended for a brand-new, empty repo. Stop and ask for confirmation if the repo already contains application code or build tooling (examples: `package.json`, `go.mod`, `Cargo.toml`, `pyproject.toml`, `pom.xml`, `build.gradle`, `src/`, `app/`, `backend/`, `frontend/`). If `specs/.loopr/repo-id` already exists, treat the repo as Loopr-managed and proceed. Loopr init writes `specs/.loopr/init-state.json` to record the initialization mode.

Greenfield detection allows only minimal scaffolding at repo root: `.git/`, `.github/`, `.vscode/`, `docs/`, `specs/`, and common metadata files like `README*`, `LICENSE*`, `.gitignore`, `.gitattributes`, `.editorconfig`, `AGENTS.md`. If non-greenfield signals exist, require `--allow-existing` to proceed.

## Workflow
0. Preflight: run `python3 ~/.codex/skills/loopr-init/scripts/loopr-init --specs-dir specs` (add `--allow-existing` if you intend to proceed in an existing repo). This writes `specs/.loopr/init-state.json` and exits non-zero if non-greenfield signals are found without `--allow-existing`.
1. Ensure `specs/.loopr/` exists.
2. Check for `specs/.loopr/repo-id`.
   - If present: read and reuse the repo-id.
   - If missing: generate a new 6-character lowercase ID (nanoid-style) and persist it to `specs/.loopr/repo-id`.
3. Ensure transcript directory exists: `specs/.loopr/transcripts/<repo-id>/`.
4. Define session log paths (do not overwrite):
   - Raw transcript: `specs/.loopr/transcripts/<repo-id>/session-<timestamp>.log`
   - Metadata (JSONL): `specs/.loopr/transcripts/<repo-id>/session-<timestamp>.jsonl`
5. Export environment variables for the current session (optional):
   - `LOOPR_REPO_ID=<repo-id>`
   - `LOOPR_TRANSCRIPT=<session-log-path>`
6. Note: transcripts are captured by the `loopr codex` wrapper. If you launch Codex directly, these files may remain empty.
7. If desired, append a short entry to `specs/.loopr/index.md` indicating the new session log path.

## Output requirements
- Idempotent: re-running does not change an existing repo-id.
- Repo-id must be 6 lowercase alphanumeric characters.
- Transcript logs must live under `specs/.loopr/transcripts/<repo-id>/`.
- Init state must live at `specs/.loopr/init-state.json`.

## Version
- 2026-01-24

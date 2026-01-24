# Task: Skill install/uninstall with backup and filtering / Install embedded skills

## Task ID
01

## Summary
Implement installation of embedded Loopr skills with backups, filtering, and atomic writes.

## Goal
Safely install or update embedded skills into the Codex skills root with predictable behavior and minimal risk of data loss.

## Scope
- In scope:
  - Load embedded skill index and filter by `--only` or `loopr-` prefix.
  - Determine skills root from `CODEX_HOME` or `~/.codex/skills`.
  - Detect changes and back up modified skills before overwrite.
  - Write skill files atomically and preserve script executability.
- Out of scope:
  - Drift reporting (doctor feature).

## Acceptance Criteria
- `loopr install` installs missing skills and updates changed ones.
- Backups are created under `.backup/loopr-<timestamp>/` when changes are detected.
- Unchanged files are skipped to minimize writes.
- `--only` limits the installed skills to the specified set.

## Implementation Plan
- Load the embedded skills index from the embedded filesystem.
- Compare installed files by hash to detect changes and determine backups.
- Copy existing skill directories to backup when needed.
- Write updated files atomically, preserving file mode for scripts.
- Produce an install report with counts of installed/updated/skipped.

## Dependencies
- CLI flag parsing helpers (cli-core task 02).

## Risks
- Partial backups on failure → use atomic file writes and fail fast unless `--force`.

## Test Plan
- Unit: use temp directories to verify backup creation and file writes.
- Manual: run `loopr install --only loopr-init` and verify files under skills root.

## Notes
- Keep file operations local and deterministic.

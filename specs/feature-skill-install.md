---
order: 2
depends_on:
  - cli-core
---

# Feature: Skill install/uninstall with backup and filtering

## Summary
Provide deterministic install and uninstall commands for embedded Loopr skills, with safe backups, filtering, and atomic writes.

## Goals
- Install embedded `loopr-*` skills safely and repeatably.
- Allow clean uninstalls with reversible backups.
- Keep operations local, safe, and deterministic.

## Non-goals
- Drift detection and listing (handled by skill-doctor).

## User Stories
- As a developer, I want to install Loopr skills into my Codex environment safely.
- As a developer, I want to uninstall Loopr skills and keep backups for rollback.

## Scope
- In scope:
  - `loopr install` and `loopr uninstall` commands.
  - Backup creation for changes/removals.
  - Filtering by agent and skill names.
- Out of scope:
  - Drift reporting or per-skill status output.

## Requirements
- Determine Codex skills root:
  - If `CODEX_HOME` is set, use `$CODEX_HOME/skills`.
  - Otherwise, default to `~/.codex/skills`.
- Default skill selection to the embedded skills with prefix `loopr-` unless `--only` is provided.
- `install` behavior:
  - Back up existing skills that would change into `.backup/loopr-<timestamp>/`.
  - Skip unchanged files based on content hash.
  - Write changed files atomically.
  - Preserve executable mode for scripts.
  - Support `--agent`, `--all`, `--only`, `--force`, `--verbose`.
- `uninstall` behavior:
  - Back up removed skills into `.backup/loopr-<timestamp>/` by default.
  - Support `--force` to remove without backup and to proceed if backup fails.
  - Support `--agent`, `--all`, `--only`, `--verbose`.

## Acceptance Criteria
- `loopr install` reports counts of installed/updated/unchanged skills and the backup path.
- `loopr uninstall` reports counts of removed skills and the backup path (unless `--force`).
- Unchanged skills are not rewritten during install.
- Executable bits on scripts are preserved after install.

## UX / Flow
- `loopr install` → installs/updates skills and prints summary counts + backup path.
- `loopr uninstall` → removes skills and prints summary counts + backup path.

## Data / API Impact
- Uses `CODEX_HOME` to resolve the skills root.
- CLI flags: `--agent`, `--all`, `--only`, `--force`, `--verbose`.

## Dependencies
- CLI core for command parsing and flag handling.

## Risks & Mitigations
- Risk: accidental overwrite of local changes → Mitigation: backup by default and atomic writes.
- Risk: backup failure blocks uninstall → Mitigation: `--force` bypasses backup.

## Open Questions
- Should backups be optionally created in a user-specified location?

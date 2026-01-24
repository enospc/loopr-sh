---
order: 2
depends_on:
  - cli-core
---

# Feature: Skill install/uninstall with backup and filtering

## Summary
Install and remove embedded Loopr skills in the local Codex skills directory with safe defaults, backups, and filtering by skill name.

## Goals
- Provide safe, deterministic installation of embedded skills.
- Prevent accidental data loss via backups and atomic writes.
- Support targeted operations via `--only` and agent selection.

## Non-goals
- Detecting skill drift (handled by the doctor feature).
- Managing workflow execution beyond skill files.

## User Stories
- As a developer, I want to install Loopr skills so that Codex can run the workflow.
- As a developer, I want to uninstall Loopr skills so that I can reset or clean up.
- As a developer, I want backups so that I can recover local edits.

## Scope
- In scope:
  - Install embedded skills into `$CODEX_HOME/skills` or `~/.codex/skills`.
  - Backup modified skills before overwrite.
  - Uninstall skills with optional backup.
  - `--only` filters and `--force` behavior.
- Out of scope:
  - Skill drift reporting.
  - Remote or network-based installs.

## Requirements
- Use embedded skills as the source of truth for installation.
- Default filter installs only skills prefixed with `loopr-`.
- Back up any skill that would be overwritten to `.backup/loopr-<timestamp>/`.
- Write changed files atomically; skip unchanged files.
- Preserve executable mode for scripts (`/scripts/` entries).
- `--force` bypasses backup failures and allows uninstall without backup.
- Support agent selection: `--agent` and `--all`.

## Acceptance Criteria
- `loopr install` installs or updates skills and prints summary counts.
- Modified skills are backed up before overwrite; unchanged skills are skipped.
- `loopr uninstall` removes skills and backs them up unless `--force` is set.
- `--only` limits installs/uninstalls to named skills.

## UX / Flow
- `loopr install [--only <list>] [--force]` outputs backup path and counts.
- `loopr uninstall [--only <list>] [--force]` outputs backup path and count.

## Data / API Impact
- Writes to `$CODEX_HOME/skills` or `~/.codex/skills`.
- Backup directory: `$SKILLS_ROOT/.backup/loopr-<timestamp>/`.

## Dependencies
- CLI command parsing and agent resolution.

## Risks & Mitigations
- Risk: data loss during install/uninstall → Mitigation: backups + atomic writes by default.
- Risk: partial backup on failure → Mitigation: `--force` for explicit override.

## Open Questions
- Should the backup retention policy be configurable?

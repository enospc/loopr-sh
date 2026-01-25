# Task: Skill install/uninstall with backup and filtering / Uninstall skills with backup

## Task ID
02

## Summary
Remove installed Loopr skills with optional backups and `--force` behavior.

## Goal
Allow developers to cleanly remove skills while preserving local edits by default.

## Scope
- In scope:
  - Remove selected skills from the skills root.
  - Back up skill directories before removal unless `--force`.
  - Support `--only` filtering and agent selection.
- Out of scope:
  - Restoring backups (manual operation).

## Acceptance Criteria
- `loopr uninstall` removes targeted skills.
- Backups are created under `.backup/loopr-<timestamp>/` unless `--force` is set.
- `--force` bypasses backup creation and proceeds on backup failure.

## Implementation Plan
- Determine skills root and filter skill list.
- If not `--force`, copy skill directories into a timestamped backup.
- Remove skill directories and produce a removal report.

## Dependencies
- CLI flag parsing helpers (cli-core task 02).
- Install logic for shared file helpers (optional).

## Risks
- Removing the wrong paths → keep strict skill root resolution and filtering.

## Test Plan
- Unit: verify backups are created and files removed in temp dirs.
- Manual: run `loopr uninstall --only loopr-init` and confirm removal.

## Notes
- Use consistent backup behavior with install.

## Completion
- Status: Done
- Tests: `go test ./...`
- Notes: None.

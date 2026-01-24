# Task: Skill drift detection and listing / Doctor drift detection

## Task ID
01

## Summary
Implement skill drift detection by comparing installed skills to embedded skill hashes and report missing/drifted/extra skills.

## Goal
Make drift and installation issues visible before running the Loopr workflow.

## Scope
- In scope:
  - Load embedded skill index.
  - Compare installed skill files by hash.
  - Report per-skill status: installed, missing, drifted.
  - Report extra local skills not in the embedded set.
- Out of scope:
  - Auto-remediation (install handles fixes).

## Acceptance Criteria
- `loopr doctor` identifies missing skills and drifted files accurately.
- `loopr doctor` reports extra skills in the skills root.
- Verbose mode prints lists of missing/drifted files.

## Implementation Plan
- Load embedded skills and filter by `--only`.
- Walk the installed skills root and compare file hashes.
- Produce a structured report (skills, missing, drifted, extras).
- Expose a CLI printer that uses the report to render output.

## Dependencies
- CLI flag parsing helpers (cli-core task 02).

## Risks
- False positives from incorrect skills root → rely on shared resolution logic.

## Test Plan
- Unit: verify detection for missing and modified files using temp dirs.
- Manual: edit an installed skill file and run `loopr doctor`.

## Notes
- Keep hash comparison deterministic; ignore hidden files.

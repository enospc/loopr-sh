# Task: Skill drift detection and listing / Doctor drift detection

## Task ID
01

## Summary
Implement skill drift detection against the embedded skills source of truth.

## Goal
Report missing, drifted, and installed skills with clear status output.

## Scope
- In scope:
  - Load embedded skill index.
  - Compare installed skills by hash and report `installed`, `missing`, or `drifted`.
  - Report extra installed skills not present in embedded list.
- Out of scope:
  - Writing or modifying installed skill files.

## Acceptance Criteria
- `loopr doctor` reports correct status for each embedded skill.
- Extra installed skills are reported separately.
- `--only` filters the report to specified skills.

## Implementation Plan
- Load embedded skills and compute expected hashes.
- Resolve installed skills root and scan for skill directories.
- Compare content hashes and generate per-skill status.
- Print a summary and per-skill status list.

## Dependencies
- CLI flag parsing helpers (cli-core task 02).

## Risks
- False drift due to normalization → hash raw bytes as stored.

## Test Plan
- Unit: create temp skill trees and validate drift detection logic.
- Manual: edit an installed skill file and verify `drifted` status.

## Notes
- Keep output stable for scripting.

## Completion
- Status: Done
- Tests: `go test ./...`
- Notes: None.

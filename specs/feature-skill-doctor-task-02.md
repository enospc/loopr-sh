# Task: Skill drift detection and listing / List command output

## Task ID
02

## Summary
Provide a `list` command that prints skill names and statuses using doctor results.

## Goal
Offer a compact status overview for installed Loopr skills.

## Scope
- In scope:
  - Reuse doctor logic to compute status per skill.
  - Print a concise list of skill name + status.
  - Respect `--only`, `--agent`, and `--all` flags.
- Out of scope:
  - Detailed drift output (doctor handles verbose listing).

## Acceptance Criteria
- `loopr list` prints each skill and its status.
- `--only` limits output to the specified skills.
- Status output matches doctor results.

## Implementation Plan
- Call doctor logic to produce a report.
- Render a compact list of skills and statuses.
- Surface extra skills in the list output.

## Dependencies
- Doctor drift detection (skill-doctor task 01).
- CLI flag parsing helpers (cli-core task 02).

## Risks
- Inconsistent status between list and doctor → always reuse doctor report.

## Test Plan
- Unit: verify list output given a mocked doctor report.
- Manual: run `loopr list` and compare to `loopr doctor`.

## Notes
- Keep output stable for easy parsing.

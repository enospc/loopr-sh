# Task: Skill drift detection and listing / List command output

## Task ID
02

## Summary
Add `loopr list` output using doctor results for a concise skill status view.

## Goal
Provide a simple list of skills and their status for quick inspection.

## Scope
- In scope:
  - Reuse doctor logic to determine status.
  - Output skill name + status for selected skills.
- Out of scope:
  - Installing or modifying skills.

## Acceptance Criteria
- `loopr list` prints skill names with status for the selected agent(s).
- `--only` limits the list to specified skills.

## Implementation Plan
- Call the doctor logic and map results to a list view.
- Format output as one skill per line with status.

## Dependencies
- Doctor drift detection (skill-doctor task 01).

## Risks
- Divergent output from doctor → reuse the same status source.

## Test Plan
- Unit: verify list output formatting from a sample status set.
- Manual: run `loopr list` to confirm output matches `loopr doctor` statuses.

## Notes
- Keep the list output stable for scripting.

## Completion
- Status: Done
- Tests: go test ./...
- Notes: Added integration-style list output check in CLI tests.

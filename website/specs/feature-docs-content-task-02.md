# Task: Documentation content / Commands and workflow docs

## Task ID
02

## Summary
Create the commands reference and workflow overview pages grounded in Loopr.md.

## Goal
Explain Loopr commands and the end-to-end workflow in a concise, accurate way.

## Scope
- In scope:
  - Commands overview (install, doctor, codex, etc.)
  - Workflow explanation (PRD -> Spec -> Features -> Tasks -> Tests -> Implementation)
- Out of scope:
  - Versioned reference or exhaustive flags list
  - Copying Loopr.md verbatim

## Acceptance Criteria
- Commands page includes core commands and their purpose.
- Workflow page explains the sequence and when user input is required.
- Content reflects Loopr.md’s themes (verification, outcomes, short loops).

## Implementation Plan
- Write markdown pages with examples and callouts.
- Link between commands and workflow pages and the install/quickstart docs.

## Dependencies
- Task 01 (docs index, install, quickstart).

## Risks
- Inaccurate command descriptions; validate against README.

## Test Plan
- Build site and verify pages exist and links are valid.
- Toggle theme and verify readability.

## Notes
- Highlight safe usage and greenfield requirements.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: Commands/workflow docs aligned with Loopr.md principles.

# Task: Documentation content / FAQ page

## Task ID
03

## Summary
Create the FAQ page covering common questions and edge cases, grounded in Loopr.md.

## Goal
Reduce onboarding friction by answering common questions up front.

## Scope
- In scope:
  - FAQ page with 8-12 common questions
  - Links to relevant docs sections
- Out of scope:
  - Support ticket workflows or community forums
  - Copying Loopr.md verbatim

## Acceptance Criteria
- FAQ page exists and is linked from nav.
- FAQ includes topics on installation, safety, and workflow expectations.
- Content reflects Loopr.md’s principles (verification, reversibility, responsibility).

## Implementation Plan
- Draft FAQ content in markdown.
- Link each answer to relevant docs sections.

## Dependencies
- Task 01 (docs index, install, quickstart).

## Risks
- FAQ out of date; review during releases.

## Test Plan
- Build site and verify FAQ page exists and links resolve.
- Toggle theme and verify readability.

## Notes
- Keep answers short and scannable.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: FAQ aligned with Loopr.md themes.

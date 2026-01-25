# Task: Repository scaffolding and test harness / Validation and test harness

## Task ID
03

## Summary
Create a lightweight validation script that checks required pages, CTAs, and internal links.

## Goal
Provide a fast, deterministic `npm test` that ensures the site meets basic requirements.

## Scope
- In scope:
  - Validate required pages exist in `dist/`
  - Validate primary/secondary CTA links are present
  - Validate internal links are not obviously broken
- Out of scope:
  - Deep HTML correctness or accessibility audits
  - External link validation

## Acceptance Criteria
- `npm test` fails if required pages or CTA links are missing.
- `npm test` fails on broken internal links found in built HTML.

## Implementation Plan
- Implement `scripts/test.js` to parse built HTML output.
- Define a list of required pages and required link targets.
- Parse anchor tags and verify internal link targets exist in `dist/`.

## Dependencies
- Task 02 (static build and dev server).

## Risks
- False positives on link checks; keep rules simple and predictable.

## Test Plan
- Run `npm run build` then `npm test` and confirm passing.
- Manually break a link to confirm `npm test` fails.

## Notes
- Keep the test harness minimal but enforce core requirements.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: Validation script in place and passing.

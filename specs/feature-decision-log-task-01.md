# Task: Decision log scaffolding / Scaffold decision log template

## Task ID
01

## Summary
Add decision log scaffolding so Loopr initializes a standard decision log template under `specs/decisions/`.

## Goal
Ensure every Loopr workspace starts with a decision log template aligned to Loopr guidance.

## Scope
- In scope:
  - Create `specs/decisions/` when missing.
  - Create `specs/decisions/template.md` with required headings.
  - Avoid overwriting existing templates.
- Out of scope:
  - Enforcing decision log usage.

## Acceptance Criteria
- Running `loopr init` creates `specs/decisions/` if missing.
- `specs/decisions/template.md` exists and includes `Title`, `Date`, `Status`, `Context`, `Decision`, `Alternatives`, `Consequences`.
- Existing templates are left unchanged.

## Implementation Plan
- Add `loopr init` CLI behavior to create the decisions directory and template if missing.
- Add a small template with the required headings.
- Update documentation to mention decision log scaffolding via `loopr init`.

## Dependencies
- None.

## Risks
- Risk: overwriting user content → Mitigation: only create files when missing.

## Test Plan
- Integration: run `loopr init` on a temp specs dir and confirm template creation.
- Unit: verify the template includes the required headings.

## Notes
- Keep the template concise to encourage use.

## Completion
- Status: Done
- Tests: `loopr init --root <temp> --specs-dir specs` (verified template headings)
- Notes: Template creation is idempotent and skipped if present.

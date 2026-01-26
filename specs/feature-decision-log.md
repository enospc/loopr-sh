---
order: 3
depends_on: []
---

# Feature: Decision log scaffolding

## Summary
Ensure Loopr scaffolds a decision log directory and template to capture key architectural decisions.

## Goals
- Provide a standard decision log template under `specs/decisions/`.
- Make decision capture easy and consistent across Loopr projects.

## Non-goals
- Enforcing that teams use the decision log.
- Building a decision log UI or automation beyond scaffolding.

## User Stories
- As a developer, I want a decision log template so that I can capture rationale and tradeoffs consistently.
- As a reviewer, I want decisions documented so that I can understand the "why" behind changes.

## Scope
- In scope:
  - Ensure `specs/decisions/` exists.
  - Provide a `specs/decisions/template.md` file with standard headings.
- Out of scope:
  - Enforcing decision log usage or validation.

## Requirements
- `loopr init` (CLI) ensures `specs/decisions/` exists.
- `specs/decisions/template.md` contains the headings: `Title`, `Date`, `Status`, `Context`, `Decision`, `Alternatives`, `Consequences`.
- Do not overwrite an existing template.

## Acceptance Criteria
- Running `loopr init` creates `specs/decisions/` if it does not exist.
- `specs/decisions/template.md` exists and includes the required headings.

## UX / Flow
- `loopr init` scaffolds the decision log directory and template alongside other Loopr metadata.

## Data / API Impact
- Adds `specs/decisions/` and `specs/decisions/template.md` to the repo.

## Dependencies
- None.

## Risks & Mitigations
- Risk: overwriting existing decision logs → Mitigation: only create missing files.

## Open Questions
- Should decision logs be indexed (e.g., `specs/decisions/index.md`) in the future?

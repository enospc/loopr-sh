---
order: 6
depends_on:
  - foundation
  - visual-theme
  - site-structure
  - theme-switcher
---

# Feature: Documentation content

## Summary
Create the documentation section with install, quickstart, commands, workflow, and FAQ content grounded in Loopr.md.

## Goals
- Provide accurate, easy-to-follow docs
- Reduce onboarding friction for new users
- Reflect Loopr.md’s emphasis on verification and intent

## Non-goals
- Full API reference or enterprise docs
- Copying Loopr.md verbatim

## User Stories
- As a developer, I want install instructions that work first try.
- As an operator, I want clarity on workflow safety and recovery.

## Scope
- In scope:
  - Install guide
  - Quickstart
  - Commands overview
  - Workflow explanation
  - FAQ
- Out of scope:
  - Advanced troubleshooting guides
  - Versioned docs or release notes

## Requirements
- Docs align with existing README and CLI behavior.
- Content stored as Markdown for easy updates.
- Docs incorporate Loopr.md themes (verification, reversibility, responsibility).

## Acceptance Criteria
- Docs section includes install, quickstart, commands, workflow, FAQ.
- Docs pages are linked from nav and each other.
- Content reflects Loopr.md’s terminology and principles.

## UX / Flow
- Docs pages use a consistent, content-first layout that aligns with the marketing theme.

## Data / API Impact
- None.

## Dependencies
- foundation
- visual-theme
- site-structure
- theme-switcher

## Risks & Mitigations
- Risk: Docs drift. Mitigation: update docs with releases and Loopr.md changes.

## Open Questions
- Level of detail for each section.

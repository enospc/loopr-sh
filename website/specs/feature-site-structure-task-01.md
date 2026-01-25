# Task: Site structure and navigation / Base layout and navigation

## Task ID
01

## Summary
Create shared layout templates with header/footer navigation and persistent CTAs aligned with the Fly.io-inspired theme.

## Goal
Ensure consistent navigation and calls-to-action across all pages, with room for a theme switcher.

## Scope
- In scope:
  - Shared layout with header, footer, and nav links
  - Primary CTA: Install Loopr
  - Secondary CTA: View Docs
  - Header utility area for theme toggle
- Out of scope:
  - Final visual polish beyond the theme primitives

## Acceptance Criteria
- All pages render with the shared header/footer.
- Nav includes Home, Docs, Codex Power User, FAQ, GitHub.
- Primary/secondary CTAs are visible on all core pages.

## Implementation Plan
- Create base layout template(s) or adjust existing ones.
- Implement nav data structure (config or front matter) consumed by templates.
- Ensure layout aligns with the Fly.io-inspired theme and supports theme toggle placement.

## Dependencies
- Foundation and visual-theme tasks.

## Risks
- Navigation drift between pages; enforce shared layout usage.

## Test Plan
- Build the site and verify shared nav/CTAs exist on all pages.

## Notes
- Keep layout simple and content-first.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: Shared layout and header already implemented.

# Task: Fly.io-inspired visual system / Section layouts and components

## Task ID
02

## Summary
Implement reusable layout patterns (hero, stacked sections, trust/enterprise blocks, CTA bands, and multi-column footer) consistent with the Fly.io-inspired theme.

## Goal
Ensure the site structure visually matches the new theme and scales across pages.

## Scope
- In scope:
  - Hero layout styles and CTA grouping
  - Stacked section rhythm (feature sections, callouts, trust/enterprise blocks)
  - CTA band and multi-column footer styling
- Out of scope:
  - Complex animations or illustration systems

## Acceptance Criteria
- Marketing and docs pages can use shared layout classes for consistent styling.
- Footer is multi-column and aligns with the theme.

## Implementation Plan
- Extend `assets/styles.css` with layout utilities and section classes.
- Adjust layout templates to support section rhythm and CTA band styling.
- Update core content pages to use the shared layout classes.

## Dependencies
- Task 01 (design tokens and typography).

## Risks
- Layout drift between pages. Mitigation: centralize styles in shared classes.

## Test Plan
- Run `npm run build` and visually inspect the home and docs pages.
- Run `npm test` for structural validation.

## Notes
- Keep patterns composable; avoid one-off layout hacks.

## Completion
- Status: Done
- Tests: `npm run build`, `npm test` (pass)
- Notes: Layout styles already align with the Fly.io-inspired theme.

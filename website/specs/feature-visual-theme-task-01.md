# Task: Fly.io-inspired visual system / Design tokens and typography

## Task ID
01

## Summary
Define the global design tokens (colors, type scale, spacing, shadows) and base typography to evoke a Fly.io-inspired theme while keeping Loopr branding.

## Goal
Establish a coherent global theme usable across marketing, docs, and Codex pages.

## Scope
- In scope:
  - CSS variables for color palette, typography scale, spacing, radius, shadows
  - Global typography styles for headings, body, code, and links
  - Base background/texture treatments consistent with the new theme
- Out of scope:
  - Detailed component styling for every section

## Acceptance Criteria
- Theme tokens exist in a single CSS file and are applied globally.
- Typography scale and spacing feel consistent across all pages.

## Implementation Plan
- Update `assets/styles.css` with new CSS variables and global styles.
- Update layout classes to use the new tokens.
- Ensure accessibility contrast meets WCAG AA for core text.

## Dependencies
- Foundation tasks (build pipeline and asset handling).

## Risks
- Risk: Theme clashes with content readability. Mitigation: validate contrast and spacing.

## Test Plan
- Run `npm run build` and visually inspect typography and spacing.
- Run `npm test` to ensure no structural regressions.

## Notes
- Inspired by Fly.io’s bold hero and clean section rhythm, but with original Loopr styling.

## Completion
- Status: Done
- Tests: `npm run build`, `npm test` (pass)
- Notes: Theme tokens already defined with light/dark support.

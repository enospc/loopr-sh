# Task: Theme switcher (light/dark) / Theme tokens + toggle UI

## Task ID
01

## Summary
Add light/dark theme tokens, a toggle UI in the header, and default to light theme.

## Goal
Let users switch themes while keeping the global Fly.io-inspired styling.

## Scope
- In scope:
  - Define light/dark CSS variable sets
  - Theme toggle button in the header
  - Default to light theme if no preference stored
- Out of scope:
  - System preference detection unless explicitly requested

## Acceptance Criteria
- Theme toggle is visible in the header.
- Default theme is light on first load.
- CSS variables change to apply the selected theme.

## Implementation Plan
- Add light theme token block in CSS (default).
- Add dark theme token block scoped by data attribute (e.g., `data-theme="dark"`).
- Add a toggle button in the layout header.

## Dependencies
- Visual theme tokens and shared layout.

## Risks
- Inconsistent styling between themes; ensure token parity.

## Test Plan
- Build the site and verify both themes render correctly when toggled.

## Notes
- Keep toggle UI minimal and accessible.

## Completion
- Status: Done
- Tests: `npm run build`, `npm test` (pass)
- Notes: Toggle UI already in header.

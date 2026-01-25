# Task: Theme switcher (light/dark) / Persistence + no-flash script

## Task ID
02

## Summary
Persist the theme selection locally and apply it before paint to avoid flashing the wrong theme.

## Goal
Maintain theme preference across pages with no visual flash.

## Scope
- In scope:
  - Local storage persistence
  - Inline script to apply theme early
  - Accessible ARIA state updates
- Out of scope:
  - Cookie-based persistence

## Acceptance Criteria
- Theme choice persists across page loads.
- Default light theme loads without flashing to dark.
- Toggle announces state to screen readers.

## Implementation Plan
- Add a small inline script in the layout head to read localStorage and set data-theme.
- Add click handler to toggle theme and update localStorage.
- Update button aria-pressed and label.

## Dependencies
- Task 01 (theme tokens + toggle UI).

## Risks
- Script placement causing flicker; keep inline in `<head>`.

## Test Plan
- Toggle theme, refresh, and verify preference persists.
- Verify no flash of dark theme when defaulting to light.

## Notes
- Keep script minimal and framework-free.

## Completion
- Status: Done
- Tests: `npm run build`, `npm test` (pass)
- Notes: Theme persistence and no-flash script already implemented.

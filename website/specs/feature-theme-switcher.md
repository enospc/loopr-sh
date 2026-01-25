---
order: 4
depends_on:
  - visual-theme
  - site-structure
---

# Feature: Theme switcher (light/dark)

## Summary
Add a global theme switcher that defaults to light theme and lets users toggle between light and dark, with preference persisted locally.

## Goals
- Provide accessible light/dark theme switching
- Default to light theme on first load
- Persist user preference without cookies

## Non-goals
- Following system preference unless explicitly specified
- Complex theming beyond light/dark

## User Stories
- As a user, I want to toggle between light and dark themes.
- As a user, I want my theme choice to persist across pages.

## Scope
- In scope:
  - Theme toggle UI in the global header
  - JavaScript to apply the selected theme without flash
  - Local persistence (e.g., localStorage)
- Out of scope:
  - System preference detection unless explicitly requested

## Requirements
- Default to light theme when no preference is stored.
- Theme toggle must be keyboard accessible and screen-reader friendly.
- Apply theme early to avoid flash of incorrect theme.
- Persist selection locally without cookies.

## Acceptance Criteria
- Theme switcher visible in header on all pages.
- Default theme is light; dark only after explicit toggle.
- Theme preference persists across page loads.

## UX / Flow
- Toggle control near primary navigation or CTAs.

## Data / API Impact
- Local storage only; no external APIs.

## Dependencies
- visual-theme
- site-structure

## Risks & Mitigations
- Risk: Flash of wrong theme. Mitigation: inline script to set theme before paint.

## Open Questions
- Whether to add a small indicator (icon + label) for current theme.

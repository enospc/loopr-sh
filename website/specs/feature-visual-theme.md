---
order: 2
depends_on:
  - foundation
---

# Feature: Fly.io-inspired visual system

## Summary
Define a global visual system inspired by Fly.io (bold hero, strong CTA emphasis, section rhythm) with dual light/dark tokens, while keeping Loopr-specific branding.

## Goals
- Establish a cohesive, Fly.io-inspired theme across all pages
- Create reusable design tokens and layout patterns
- Define light and dark theme tokens as the foundation for theme switching

## Non-goals
- Copying Fly.io assets, copy, or branding
- Building a full design system library

## User Stories
- As a visitor, I want a confident, modern visual style that signals trust.
- As a maintainer, I want consistent typography, spacing, and buttons across pages.

## Scope
- In scope:
  - Typography scale, color palette, spacing system, and button styles
  - CSS variables for both light and dark themes
  - Base background/texture treatments consistent with the new theme
- Out of scope:
  - Theme switcher UI and persistence logic

## Requirements
- Global CSS tokens for type scale, spacing, colors, and shadows.
- Light and dark themes defined via CSS variables.
- Layout patterns for hero, feature sections, trust/enterprise blocks, and footer.

## Acceptance Criteria
- All pages share a cohesive Fly.io-inspired aesthetic.
- Light/dark tokens are present and applied via CSS variables.

## UX / Flow
- Emphasize bold hero headline, tight CTA grouping, and stacked sections with clear hierarchy.

## Data / API Impact
- None.

## Dependencies
- foundation

## Risks & Mitigations
- Risk: Theme clashes with content readability. Mitigation: validate contrast and spacing.

## Open Questions
- Final palette and typography choices that best evoke the Fly.io feel.

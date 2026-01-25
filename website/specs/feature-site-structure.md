---
order: 3
depends_on:
  - foundation
  - visual-theme
---

# Feature: Site structure and navigation

## Summary
Define the core information architecture and navigation for marketing and docs sections, including a custom 404 page and SEO metadata.

## Goals
- Clear separation of marketing vs docs
- Consistent navigation and CTAs across pages
- Basic SEO metadata per page
- Layout slot for global utilities (theme switcher)

## Non-goals
- Final visual design polish beyond the defined theme
- Advanced SEO (sitemap/robots beyond basics)

## User Stories
- As a visitor, I want to quickly understand what Loopr is and how to install it.
- As a visitor, I want to navigate between marketing, docs, and Codex power user content.

## Scope
- In scope:
  - Top-level nav: Home, Docs, Codex Power User, FAQ, GitHub
  - Primary/secondary CTAs in header/footer
  - Custom 404 page
  - Per-page metadata (title, description, OG/Twitter)
  - Header layout that can host a theme toggle
- Out of scope:
  - Visual design beyond theme primitives

## Requirements
- Navigation must be consistent across pages.
- 404 page must provide links back to key sections.
- SEO metadata defined per page.
- Header includes a utility area for the theme toggle.

## Acceptance Criteria
- All pages share header/footer navigation.
- 404 page renders and links to Home and Docs.
- Metadata is included in built HTML.

## UX / Flow
- Persistent CTAs: Install Loopr (primary), View Docs (secondary).

## Data / API Impact
- None.

## Dependencies
- foundation
- visual-theme

## Risks & Mitigations
- Risk: Confusing IA. Mitigation: keep nav minimal and content-first.

## Open Questions
- Exact page list for v1 (can be refined during implementation).

---
order: 8
depends_on:
  - foundation
  - site-structure
---

# Feature: Analytics and SEO instrumentation

## Summary
Integrate Cloudflare Web Analytics and ensure SEO metadata is present on all pages; define conversion proxy tracking.

## Goals
- Enable privacy-first analytics via Cloudflare
- Track conversion proxies (CTA, GitHub, docs pageviews)

## Non-goals
- Cookie-based tracking or advanced marketing automation

## User Stories
- As a maintainer, I want to see whether users are clicking install and docs links.
- As a maintainer, I want basic traffic and referrer insights without compliance burden.

## Scope
- In scope:
  - Cloudflare Web Analytics snippet
  - Trackable conversion proxies via URLs or events
  - Per-page SEO metadata checks
- Out of scope:
  - Advanced analytics dashboards or user-level tracking

## Requirements
- Analytics snippet included on all pages.
- Conversion proxies are trackable in Cloudflare analytics reports.

## Acceptance Criteria
- Analytics snippet present in built HTML.
- CTA/GitHub link clicks are trackable (e.g., `/go/install`, `/go/github`).

## UX / Flow
- Not user-facing; tracking only.

## Data / API Impact
- Cloudflare Web Analytics integration only.

## Dependencies
- foundation
- site-structure

## Risks & Mitigations
- Risk: Cloudflare analytics limitations. Mitigation: use URL-based conversion proxies.

## Open Questions
- Preferred Cloudflare analytics configuration (Pages vs global snippet).

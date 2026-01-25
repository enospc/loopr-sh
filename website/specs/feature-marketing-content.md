---
order: 5
depends_on:
  - foundation
  - visual-theme
  - site-structure
  - theme-switcher
---

# Feature: Marketing landing content

## Summary
Create marketing landing content aligned to the Fly.io-inspired layout, grounded in Loopr.md’s principles and terminology.

## Goals
- Provide compelling, concise marketing content
- Drive users to installation and docs
- Reflect Loopr.md’s core mindset and verification focus

## Non-goals
- A/B testing or advanced growth experiments
- Copying Loopr.md verbatim

## User Stories
- As a developer, I want to understand Loopr’s value quickly.
- As a decision maker, I want to see reliability and risk mitigation signals.

## Scope
- In scope:
  - Hero section with bold headline and primary CTA
  - Benefits/feature highlights tied to verification/outcomes
  - Trust/credibility block informed by Loopr.md themes
  - Enterprise-readiness block grounded in reversibility and observability
  - Closing CTA
- Out of scope:
  - Pricing or enterprise sales funnel

## Requirements
- Content must be accurate relative to Loopr CLI behavior.
- Content must align with Loopr.md’s gist (verification, outcomes, reversibility).
- CTA links to docs/install or conversion proxy route.

## Acceptance Criteria
- Landing page includes hero, benefits, trust/enterprise blocks, and CTA sections.
- CTA links to install docs or proxy route.
- Copy reflects Loopr.md’s themes without contradicting the guide.

## UX / Flow
- Narrative scroll: hero -> features -> trust/enterprise -> closing CTA.

## Data / API Impact
- None.

## Dependencies
- foundation
- visual-theme
- site-structure
- theme-switcher

## Risks & Mitigations
- Risk: Messaging mismatch. Mitigation: align with Loopr.md and README.

## Open Questions
- Availability of social proof (testimonials, adoption stats).

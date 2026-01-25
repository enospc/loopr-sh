# Task: Marketing landing content / Landing page content

## Task ID
01

## Summary
Draft the marketing landing page content to align with the Fly.io-inspired layout and Loopr.md themes.

## Goal
Explain Loopr clearly and drive visitors to install or read docs.

## Scope
- In scope:
  - Hero section with clear value prop grounded in Loopr.md
  - Benefits/feature highlights tied to verification/outcomes
  - Trust/credibility block reflecting reliability and safety
  - Enterprise/readiness block grounded in reversibility and observability
  - Closing CTA band
- Out of scope:
  - A/B testing or growth experiments
  - Copying Loopr.md verbatim

## Acceptance Criteria
- Home page includes hero, benefits, trust/enterprise, and CTA sections.
- Install CTA links to docs/install (or conversion proxy route).
- Copy reflects Loopr.md’s themes without contradicting the guide.

## Implementation Plan
- Draft marketing content in markdown using Loopr.md as baseline.
- Ensure sections align with current CLI capabilities and README.
- Add links to docs and GitHub.

## Dependencies
- Site structure, visual theme, and theme switcher.

## Risks
- Messaging mismatch with product; review against Loopr.md and README.

## Test Plan
- Build site and verify required sections and CTA links exist.
- Toggle theme and ensure contrast remains acceptable.

## Notes
- Keep copy concise and developer-focused.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: Marketing copy aligned with Loopr.md themes.

# Task: Analytics and SEO instrumentation / Cloudflare analytics integration

## Task ID
01

## Summary
Integrate Cloudflare Web Analytics across all pages.

## Goal
Provide privacy-first analytics without cookies or user tracking.

## Scope
- In scope:
  - Add the Cloudflare Web Analytics snippet to the shared layout
  - Allow configuration of the site token (e.g., via environment or config file)
- Out of scope:
  - Third-party analytics or cookie banners

## Acceptance Criteria
- Analytics snippet is present in built HTML on all pages.
- Site token can be set without editing templates directly.

## Implementation Plan
- Add analytics config to site settings.
- Inject snippet in the base layout.
- Document where to set the Cloudflare token.

## Dependencies
- Site structure and shared layout.

## Risks
- Token leakage in repo; keep token out of source if required.

## Test Plan
- Build site and verify snippet is present in HTML output.

## Notes
- Keep integration minimal and privacy-first.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: Analytics snippet unchanged; compatible with updated content.

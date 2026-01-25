# Task: Analytics and SEO instrumentation / Conversion proxy routes

## Task ID
02

## Summary
Add trackable conversion proxy routes for Install and GitHub CTAs.

## Goal
Enable reliable conversion tracking in Cloudflare Analytics.

## Scope
- In scope:
  - Static redirect pages or routes (e.g., /go/install, /go/github)
  - Update CTAs to use the proxy routes
- Out of scope:
  - Client-side click tracking scripts

## Acceptance Criteria
- /go/install and /go/github are built and redirect to targets.
- CTAs and GitHub links use the proxy routes.

## Implementation Plan
- Add simple redirect pages in the content tree.
- Update CTA links and GitHub link to point to proxy routes.
- Ensure test harness validates the proxy routes exist.

## Dependencies
- Analytics integration and shared layout.

## Risks
- Redirect behavior differs between local and Cloudflare; use standard meta refresh or HTTP redirect via hosting config.

## Test Plan
- Build site and verify proxy routes exist.
- Confirm links point to /go/install and /go/github.

## Notes
- Use simple HTML redirect or Cloudflare Pages redirects.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: Conversion proxy routes unchanged.

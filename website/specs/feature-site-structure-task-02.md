# Task: Site structure and navigation / SEO metadata and 404 page

## Task ID
02

## Summary
Add per-page SEO metadata and a custom 404 page with navigation back to key sections.

## Goal
Provide baseline SEO tags and a usable 404 experience.

## Scope
- In scope:
  - Title/description metadata per page
  - Open Graph/Twitter metadata
  - Custom 404 page
- Out of scope:
  - Advanced SEO (sitemaps, robots tuning)

## Acceptance Criteria
- Built HTML includes metadata tags per page.
- 404 page exists and links to Home and Docs.

## Implementation Plan
- Extend front matter schema to include title/description/og fields.
- Render metadata into the layout template.
- Create a `404` content page.

## Dependencies
- Task 01 (base layout and navigation).

## Risks
- Metadata inconsistencies; keep required fields minimal and enforce defaults.

## Test Plan
- Build the site and inspect metadata in HTML output.
- Verify the 404 page is built and accessible.

## Notes
- Provide sensible defaults when metadata is missing.

## Completion
- Status: Done
- Tests: `npm test` (pass)
- Notes: SEO metadata and 404 page already present.

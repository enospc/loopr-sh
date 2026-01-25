# Task: Repository scaffolding and test harness / Static build and dev server

## Task ID
02

## Summary
Implement the home-grown static site build pipeline and local dev server.

## Goal
Generate static HTML from markdown content using shared templates and serve it locally.

## Scope
- In scope:
  - Build script to convert markdown to HTML with templates
  - Asset copy step into `dist/`
  - Local dev server for previewing pages
- Out of scope:
  - Live-reload sophistication beyond basic file watching
  - Production deployment pipeline

## Acceptance Criteria
- `npm run build` produces HTML pages in `dist/`.
- `npm run dev` serves the built site locally.
- Static assets are available in `dist/`.

## Implementation Plan
- Implement `scripts/build.js` to:
  - Read markdown content and front matter
  - Render with a shared layout template
  - Write HTML into `dist/` preserving paths
  - Copy static assets into `dist/`
- Implement `scripts/dev.js` to build and serve the site locally.
- Implement `scripts/preview.js` to serve `dist/` without rebuilding.

## Dependencies
- Task 01 (project scaffold and npm scripts).

## Risks
- Build complexity creep. Keep pipeline minimal and deterministic.

## Test Plan
- Run `npm run build` and verify expected HTML files exist.
- Run `npm run dev` and confirm pages load in a browser.

## Notes
- Prefer simple, transparent scripts over heavy tooling.

## Completion
- Status: Done
- Tests: `npm run build` (pass); dev server not run (long-running)
- Notes: Build pipeline and dev/preview servers already implemented.

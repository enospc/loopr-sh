---
order: 1
depends_on: []
---

# Feature: Repository scaffolding and test harness

## Summary
Create the home-grown static site scaffolding, build tooling, and a minimal test harness to validate required pages and links.

## Goals
- Provide a deterministic local dev/build workflow with npm scripts
- Establish a test harness for content and link validation
- Produce static build output for Cloudflare hosting

## Non-goals
- Implement full site content or visual design
- Add advanced CI/CD automation

## User Stories
- As a developer, I want simple scripts to develop, build, and preview the site.
- As a maintainer, I want basic tests to ensure required pages and links exist.

## Scope
- In scope:
  - Project structure for content, templates, assets, and build output
  - npm scripts: dev, build, preview, test
  - Basic test runner to validate required pages/CTAs/links
- Out of scope:
  - Full content and styling

## Requirements
- Provide a static build output directory (e.g., `dist/`).
- Provide a dev server for local preview with file watching.
- `npm test` validates required pages and internal links.

## Acceptance Criteria
- `npm run build` produces static output directory.
- `npm run dev` serves the site locally.
- `npm test` fails on missing required pages or broken internal links.

## UX / Flow
- Not applicable (foundation only).

## Data / API Impact
- None.

## Dependencies
- None.

## Risks & Mitigations
- Risk: Overbuilding the tooling. Mitigation: keep scripts minimal and deterministic.

## Open Questions
- None.

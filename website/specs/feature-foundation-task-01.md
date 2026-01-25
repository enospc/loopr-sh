# Task: Repository scaffolding and test harness / Project scaffold and npm scripts

## Task ID
01

## Summary
Set up the repo structure and npm tooling for a home-grown static site build.

## Goal
Provide a deterministic local workflow with `dev`, `build`, `preview`, and `test` scripts.

## Scope
- In scope:
  - Create package.json with npm scripts
  - Establish directories for content, templates, assets, and build output
  - Add baseline config files (e.g., .gitignore for dist/node_modules)
- Out of scope:
  - Full site content or styling
  - Production deployment setup

## Acceptance Criteria
- `npm run dev`, `npm run build`, `npm run preview`, and `npm test` are defined.
- Repo contains a clear structure for content, templates, assets, and output.

## Implementation Plan
- Create a minimal directory layout (e.g., `content/`, `templates/`, `assets/`, `scripts/`, `dist/`).
- Initialize `package.json` with required scripts.
- Add minimal dependencies needed for markdown parsing, templating, and local serving.
- Add `.gitignore` entries for build output and node_modules.

## Dependencies
- None.

## Risks
- Over-scoping tooling. Keep dependencies minimal and focused.

## Test Plan
- Run `npm run build` and verify a `dist/` directory is produced.
- Run `npm test` to ensure the harness is wired (even if basic initially).

## Notes
- Favor small libraries over frameworks to keep the build home-grown.

## Completion
- Status: Done
- Tests: `npm run build`, `npm test` (pass)
- Notes: Scaffold and scripts already in place.

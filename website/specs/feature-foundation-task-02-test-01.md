# Test: Static build output and dev server

## Test ID
01

## Type
Manual

## Purpose
Ensure the static build outputs HTML and the dev server serves pages.

## Preconditions
- Dependencies installed via `npm install`.

## Test Data
- None.

## Steps
1. Run `npm run build`.
2. Confirm the output directory (e.g., `dist/`) exists and includes HTML files.
3. Run `npm run dev` and open the served URL.
4. Verify the home page renders.

## Expected Results
- Build produces HTML output in the static directory.
- Dev server serves the site locally.

## Automation Notes
- Can be automated by checking `dist/` and hitting the dev URL.

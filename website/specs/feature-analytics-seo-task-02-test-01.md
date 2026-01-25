# Test: Conversion proxy routes

## Test ID
01

## Type
Manual

## Purpose
Ensure conversion proxy routes exist and are used by CTAs.

## Preconditions
- Site built with `npm run build`.

## Test Data
- None.

## Steps
1. Verify `dist/go/install/index.html` and `dist/go/github/index.html` (or equivalent) exist.
2. Open the home page and ensure the Install CTA points to `/go/install`.
3. Verify GitHub links point to `/go/github`.

## Expected Results
- Proxy routes exist in the build output.
- CTAs use the proxy routes.

## Automation Notes
- Can be automated by checking for route files and hrefs.

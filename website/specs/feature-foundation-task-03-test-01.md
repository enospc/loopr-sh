# Test: Validation harness detects missing pages/links

## Test ID
01

## Type
Manual

## Purpose
Ensure `npm test` fails on missing required pages or broken internal links.

## Preconditions
- A successful build exists in `dist/`.

## Test Data
- None.

## Steps
1. Run `npm test` and confirm it passes.
2. Remove or rename a required HTML page in `dist/`.
3. Run `npm test` again.
4. Restore the missing page.

## Expected Results
- Test passes when required pages/links are intact.
- Test fails when required pages/links are missing or broken.

## Automation Notes
- Can be automated by mutating `dist/` and asserting exit codes.

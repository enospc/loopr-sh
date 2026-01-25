# Test: Commands and workflow docs exist

## Test ID
01

## Type
Manual

## Purpose
Verify commands and workflow documentation pages exist and reflect Loopr.md themes.

## Preconditions
- Site built with `npm run build`.
- Loopr.md available for reference.

## Test Data
- None.

## Steps
1. Open the Commands page in the build output.
2. Confirm it lists core Loopr commands and brief descriptions.
3. Open the Workflow page and verify it explains the PRD -> Spec -> Features -> Tasks -> Tests -> Implementation sequence.
4. Spot-check for Loopr.md themes (verification, outcomes, short loops).

## Expected Results
- Commands and Workflow pages exist and contain expected content.
- Copy aligns with Loopr.md themes.

## Automation Notes
- Can be automated by checking for page files and key headings.

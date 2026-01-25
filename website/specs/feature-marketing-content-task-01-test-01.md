# Test: Landing page sections and Loopr.md alignment

## Test ID
01

## Type
Manual

## Purpose
Verify the landing page includes required sections and reflects Loopr.md themes.

## Preconditions
- Site built with `npm run build`.
- Loopr.md available for reference.

## Test Data
- None.

## Steps
1. Open the built home page HTML.
2. Confirm sections exist for hero/value prop, benefits, trust/enterprise blocks, and closing CTA.
3. Verify the primary Install CTA links to the install docs or proxy route.
4. Spot-check copy for alignment with Loopr.md themes (verification, outcomes, reversibility).

## Expected Results
- Required sections appear on the landing page.
- Install CTA points to the correct destination.
- Copy aligns with Loopr.md themes.

## Automation Notes
- Can be partially automated by checking for section IDs and CTA hrefs.

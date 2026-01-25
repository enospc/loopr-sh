# Test: Shared layout and navigation

## Test ID
01

## Type
Manual

## Purpose
Verify all pages include the shared header/footer navigation and CTAs.

## Preconditions
- Site built with `npm run build`.

## Test Data
- None.

## Steps
1. Open the built HTML for Home, Docs, and Codex Power User pages.
2. Verify each page includes the shared header and footer.
3. Confirm nav links: Home, Docs, Codex Power User, FAQ, GitHub.
4. Confirm primary CTA (Install Loopr) and secondary CTA (View Docs) appear.

## Expected Results
- All core pages share the same nav and CTAs.

## Automation Notes
- Can be automated by parsing `dist/*.html` for nav link anchors and CTA IDs.

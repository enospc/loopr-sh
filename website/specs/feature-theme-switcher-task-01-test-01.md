# Test: Theme toggle defaults to light

## Test ID
01

## Type
Manual

## Purpose
Verify the theme toggle exists and defaults to the light theme.

## Preconditions
- Site built with `npm run build`.
- Local storage cleared for theme key.

## Test Data
- None.

## Steps
1. Clear local storage for the site.
2. Load the home page.
3. Verify the theme toggle is visible in the header.
4. Confirm the theme is light by default.

## Expected Results
- Theme toggle is visible.
- Light theme is active on first load.

## Automation Notes
- Can be automated with a headless browser checking data attributes.

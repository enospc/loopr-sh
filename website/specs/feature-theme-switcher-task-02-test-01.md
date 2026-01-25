# Test: Theme preference persists

## Test ID
01

## Type
Manual

## Purpose
Ensure theme preference persists across page loads without flash.

## Preconditions
- Site built with `npm run build`.

## Test Data
- None.

## Steps
1. Toggle to dark theme.
2. Refresh the page.
3. Confirm dark theme remains active.
4. Toggle back to light theme and refresh again.

## Expected Results
- Theme selection persists across refreshes.
- No visible flash of the wrong theme.

## Automation Notes
- Can be automated with a headless browser and localStorage checks.

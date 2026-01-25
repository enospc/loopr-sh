# Test: Cloudflare analytics snippet present

## Test ID
01

## Type
Manual

## Purpose
Verify Cloudflare Web Analytics snippet is included on all pages.

## Preconditions
- Site built with `npm run build`.

## Test Data
- Cloudflare analytics token configured if required.

## Steps
1. Open the built HTML for Home and Docs pages.
2. Search for the Cloudflare analytics snippet or beacon script.

## Expected Results
- Analytics snippet is present in built HTML across pages.

## Automation Notes
- Can be automated by scanning `dist/` for the snippet string.

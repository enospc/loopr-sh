# Test: SEO metadata and custom 404 page

## Test ID
01

## Type
Manual

## Purpose
Ensure metadata tags exist and the 404 page is present.

## Preconditions
- Site built with `npm run build`.

## Test Data
- None.

## Steps
1. Open the built HTML for the home page.
2. Verify presence of title, description, and OG/Twitter tags.
3. Confirm a `404` page exists in the build output.
4. Open the 404 page and verify it links to Home and Docs.

## Expected Results
- Metadata tags exist in HTML.
- 404 page exists and provides navigation back to key sections.

## Automation Notes
- Can be automated by parsing HTML for meta tags and checking `dist/404.html`.

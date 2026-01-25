# Test: Decision log template includes required headings

## Test ID
02

## Type
Unit

## Purpose
Verify the decision log template contains all required headings.

## Preconditions
- `specs/decisions/template.md` exists.

## Test Data
- None.

## Steps
1. Read `specs/decisions/template.md`.
2. Confirm the headings `Title`, `Date`, `Status`, `Context`, `Decision`, `Alternatives`, `Consequences` are present.

## Expected Results
- All required headings are present in the template.

## Automation Notes
- Implement as a simple file-content check.

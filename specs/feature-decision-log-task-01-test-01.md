# Test: Decision log scaffolding created by loopr init

## Test ID
01

## Type
Integration

## Purpose
Ensure `loopr init` creates `specs/decisions/` and the decision log template.

## Preconditions
- `loopr` binary built and available on PATH or invoked directly.

## Test Data
- A temp directory with an empty `specs/`.

## Steps
1. Run `loopr init --root <temp> --specs-dir specs`.
2. Check for `specs/decisions/` and `specs/decisions/template.md`.

## Expected Results
- The decisions directory exists.
- The template file exists.

## Automation Notes
- Can be automated by invoking `loopr init` and checking filesystem state.

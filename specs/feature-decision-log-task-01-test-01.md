# Test: Decision log scaffolding created by loopr-init

## Test ID
01

## Type
Integration

## Purpose
Ensure `loopr-init` creates `specs/decisions/` and the decision log template.

## Preconditions
- Python 3 available.
- loopr-init script accessible in the installed skills.

## Test Data
- A temp directory with an empty `specs/`.

## Steps
1. Run the loopr-init script with `--specs-dir` pointing at the temp `specs/` directory.
2. Check for `specs/decisions/` and `specs/decisions/template.md`.

## Expected Results
- The decisions directory exists.
- The template file exists.

## Automation Notes
- Can be automated by invoking the loopr-init script and checking filesystem state.

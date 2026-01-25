# Test: Make build produces bin/loopr

## Test ID
01

## Type
Manual

## Purpose
Verify the deterministic build target produces the Loopr binary.

## Preconditions
- Go toolchain installed per repo requirements.

## Test Data
- None.

## Steps
1. Run `make build` at the repo root.
2. Check for `bin/loopr`.

## Expected Results
- `make build` completes without errors.
- `bin/loopr` exists and is executable.

## Automation Notes
- Suitable for CI if build environment is standardized.

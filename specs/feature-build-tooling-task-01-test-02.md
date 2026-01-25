# Test: Make fmt and vet complete

## Test ID
02

## Type
Integration

## Purpose
Ensure formatting and vetting targets run successfully.

## Preconditions
- Go toolchain installed per repo requirements.

## Test Data
- None.

## Steps
1. Run `make fmt` at the repo root.
2. Run `make vet` at the repo root.

## Expected Results
- `make fmt` completes without errors.
- `make vet` completes without errors.

## Automation Notes
- Covered by `scripts/ci/build-tooling-check.sh` in CI or local runs.

# Test: Version output includes build metadata

## Test ID
02

## Type
Integration

## Purpose
Ensure `loopr version` prints version/commit/date when provided via build metadata.

## Preconditions
- Build `loopr` with ldflags that set version, commit, and date.

## Test Data
- Example ldflags: `-X internal/version.Version=1.2.3 -X internal/version.Commit=abc123 -X internal/version.Date=2026-01-25`.

## Steps
1. Build `loopr` with the ldflags above.
2. Run `loopr version`.

## Expected Results
- Output includes the version, commit, and date values that were injected.
- Exit code is 0.

## Automation Notes
- Parse output tokens to verify each metadata field is present.

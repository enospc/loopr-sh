# Test: Version output includes build metadata

## Test ID
02

## Type
Integration

## Purpose
Verify `loopr version` prints version information and optional build metadata.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.

## Test Data
- Optional build metadata set via ldflags.

## Steps
1. Run `loopr version`.

## Expected Results
- Output starts with `loopr <version>`.
- If commit/date metadata is set, it is printed on subsequent lines.

## Automation Notes
- Can be automated with a build that injects version metadata via ldflags.

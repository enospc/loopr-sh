# Test: Codex passthrough respects -- delimiter

## Test ID
03

## Type
Integration

## Purpose
Ensure arguments after `--` are passed to Codex unchanged.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- Codex CLI installed, or a stub `codex` script on PATH to capture args.

## Test Data
- Codex args such as `--help`.

## Steps
1. Run `loopr codex -- --help`.
2. Observe Codex receiving `--help` as an argument.

## Expected Results
- Codex receives `--help` without Loopr parsing it.

## Automation Notes
- Use a stub `codex` binary in PATH for deterministic argument capture.

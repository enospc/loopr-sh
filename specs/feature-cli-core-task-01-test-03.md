# Test: Codex passthrough respects -- delimiter

## Test ID
03

## Type
Integration

## Purpose
Ensure arguments after `--` are passed to Codex unchanged and the Loopr prompt is appended afterward.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- Codex CLI installed, or a stub `codex` script on PATH to capture args.

## Test Data
- Codex args such as `--help`.

## Steps
1. Run `loopr run --codex --step execute -- --help`.
2. Observe Codex receiving `--help` as an argument and the Loopr prompt as the final argument.

## Expected Results
- Codex receives `--help` without Loopr parsing it.
- The final argument includes the Loopr prompt (starts with `Loopr step:`).

## Automation Notes
- Use a stub `codex` binary in PATH for deterministic argument capture.

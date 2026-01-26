# Test: Optional reproducibility fields follow env vars

## Test ID
04

## Type
Integration

## Purpose
Verify optional reproducibility fields are present only when env vars are set.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- A Loopr workspace with `specs/.loopr/repo-id` present.
- Codex CLI installed, or a stub `codex` script on PATH.

## Test Data
- `LOOPR_CODEX_MODEL=example-model`
- `LOOPR_CODEX_PROMPT=example-prompt`

## Steps
1. Run `loopr run --codex --step execute -- --help` without the env vars and inspect JSONL `start` event.
2. Run `loopr run --codex --step execute -- --help` with the env vars set and inspect JSONL `start` event.

## Expected Results
- Step 1: optional fields are absent.
- Step 2: optional fields are present and match the env var values.

## Automation Notes
- Use a stub `codex` binary for deterministic runs in CI.

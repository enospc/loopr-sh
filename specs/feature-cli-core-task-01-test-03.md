# Test: Codex passthrough respects -- delimiter

## Test ID
03

## Type
Integration

## Purpose
Ensure arguments after `--` are passed to Codex unchanged and that the Loopr prompt is not appended when `--help` is supplied.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- Codex CLI installed and available on PATH.

## Test Data
- Codex args such as `--help`.

## Steps
1. Run `loopr run --codex --step execute -- --help`.
2. Inspect the JSONL `start` event `cmd` array in the transcript metadata.

## Expected Results
- `cmd` includes `--help` without Loopr parsing it.
- `cmd` does not include a trailing Loopr prompt entry (no `Loopr step:` payload).

## Automation Notes
- Use the real Codex CLI; this test assumes it is installed and runnable.

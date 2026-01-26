# Test: Codex wrapper honors --loopr-root override

## Test ID
03

## Type
Integration

## Purpose
Verify `--loopr-root` overrides workspace resolution.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- A Loopr workspace with `specs/.loopr/repo-id` present.
- Codex CLI installed and available on PATH.

## Test Data
- Example Codex args such as `--help`.

## Steps
1. Run `loopr run --codex --step execute --loopr-root <workspace-a> -- --help`.
2. Inspect transcript directories for the workspace.

## Expected Results
- Step 1 writes transcripts under workspace A.

## Automation Notes
- Use temp workspaces with repo-id files for deterministic checks.

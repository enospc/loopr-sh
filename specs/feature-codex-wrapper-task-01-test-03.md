# Test: Codex wrapper honors loopr root overrides

## Test ID
03

## Type
Integration

## Purpose
Verify `--loopr-root` and `LOOPR_ROOT` override workspace resolution.

## Preconditions
- `bin/loopr` built and available on PATH or invoked directly.
- Two Loopr workspaces with distinct `specs/.loopr/repo-id` values.
- Codex CLI installed, or a stub `codex` script on PATH.

## Test Data
- Example Codex args such as `--help`.

## Steps
1. Run `loopr run --codex --step execute --loopr-root <workspace-a> -- --help`.
2. Run `LOOPR_ROOT=<workspace-b> loopr run --codex --step execute -- --help`.
3. Inspect transcript directories for each workspace.

## Expected Results
- Step 1 writes transcripts under workspace A.
- Step 2 writes transcripts under workspace B.

## Automation Notes
- Use temp workspaces with repo-id files for deterministic checks.

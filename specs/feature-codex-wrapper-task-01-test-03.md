# Test: Codex wrapper honors loopr root overrides

## Test ID
03

## Type
Integration

## Purpose
Verify `--loopr-root` and `LOOPR_ROOT` select the intended workspace, with flag precedence over env.

## Preconditions
- Two temp workspaces exist, each with its own `specs/.loopr/repo-id`.
- A stub `codex` binary is available in PATH.

## Test Data
- Commands:
  - `LOOPR_ROOT=<pathA> loopr codex -- --help`
  - `LOOPR_ROOT=<pathA> loopr codex --loopr-root <pathB> -- --help`

## Steps
1. Create workspace A and workspace B, each with `specs/.loopr/repo-id`.
2. Set `LOOPR_ROOT` to workspace A and run `loopr codex -- --help`.
3. Confirm transcript artifacts are created under workspace A.
4. With `LOOPR_ROOT` still set to A, run `loopr codex --loopr-root <pathB> -- --help`.
5. Confirm transcript artifacts are created under workspace B.

## Expected Results
- Without `--loopr-root`, `LOOPR_ROOT` determines the workspace.
- With `--loopr-root`, artifacts are created under the flag-specified workspace regardless of `LOOPR_ROOT`.

## Automation Notes
- Ensure each workspace has a distinct repo-id to simplify assertions.

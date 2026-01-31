# Loopr Loop Harness (Spec)

## Goal
Provide a minimal, safe, repeatable autonomous loop for Codex that enforces Loopr.md principles:
verification-first, determinism, observability, and failure containment.

## Scope (MVP)
- Repeated Codex invocations with a structured status block.
- Structured status JSON.
- Simple project config in `loopr/config`.

## Non-goals (MVP)
- Project onboarding wizard, PRD import, or migrations.
- Deep Codex session continuity (future extension).

## Loop Command
`loopr loop [--loopr-root <path>] [--max-iterations <n>] [--] <codex args>`

Behavior:
1. Resolve Loopr root.
2. Load `loopr/config` (env overrides optional).
3. Invoke Codex with a Loopr prompt that requires the status block.
4. Parse status from the transcript.
5. Update `loopr/state/status.json`.
6. Exit on completion, error, or missing status.

## Status Block (Required)
Codex output must include:
```
---LOOPR_STATUS---
STATUS: IN_PROGRESS | COMPLETE | BLOCKED | ERROR
EXIT_SIGNAL: true | false
SUMMARY: <short summary>
---END_LOOPR_STATUS---
```

## Config File
Location: `loopr/config`
Format: `KEY=VALUE` (blank lines and `#` comments ignored)

Defaults:
```
CODEX_TIMEOUT_MINUTES=15
MAX_ITERATIONS=50
MAX_MISSING_STATUS=2
```

## State Files
All under `loopr/state/`:
- `status.json` (public, current loop status)

## Exit Logic (MVP)
- **Complete**: `EXIT_SIGNAL=true` or `STATUS=COMPLETE`.
- **Error/Blocked**: `STATUS=ERROR` or `STATUS=BLOCKED`.
- **Missing status**: `missing_status >= MAX_MISSING_STATUS`.
- **Max iterations**: stop when `iteration >= MAX_ITERATIONS`.

## Observability
`status.json` includes:
- `state` (running | complete | blocked | error)
- `iteration`
- `exit_reason` (if any)
- `last_summary`
- `last_error`

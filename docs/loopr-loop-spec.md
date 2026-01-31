# Loopr Loop Harness (Spec)

## Goal
Provide a minimal, safe, repeatable autonomous loop for Codex that enforces Loopr.md principles:
verification-first, determinism, observability, and failure containment.

## Scope (MVP)
- Repeated Codex invocations with a structured status block.
- Dual-condition exit gate.
- Rate limiting with hourly reset.
- Circuit breaker for no-progress and repeated errors.
- Structured status JSON + append-only loop log.
- Simple project config in `.loopr/config`.

## Non-goals (MVP)
- Project onboarding wizard, PRD import, or migrations.
- Deep Codex session continuity (future extension).

## Loop Command
`loopr loop [--loopr-root <path>] [--max-iterations <n>] [--] <codex args>`

Behavior:
1. Resolve Loopr root.
2. Load `.loopr/config` (env overrides optional).
3. Enforce rate limits.
4. Invoke Codex with a Loopr prompt that requires the status block.
5. Parse status from the transcript.
6. Update `.loopr/status.json` and loop state.
7. Exit on completion or circuit breaker.

## Status Block (Required)
Codex output must include:
```
---LOOPR_STATUS---
STATUS: IN_PROGRESS | COMPLETE | BLOCKED | ERROR
EXIT_SIGNAL: true | false
WORK_TYPE: tests | code | docs | other
FILES_MODIFIED: <int>
ERRORS: <int>
SUMMARY: <short summary>
---END_LOOPR_STATUS---
```

## Config File
Location: `.loopr/config`
Format: `KEY=VALUE` (blank lines and `#` comments ignored)

Defaults:
```
MAX_CALLS_PER_HOUR=100
CODEX_TIMEOUT_MINUTES=15
MAX_ITERATIONS=50
MAX_CONSECUTIVE_DONE_SIGNALS=2
MAX_NO_PROGRESS=3
MAX_SAME_ERROR=5
MAX_CONSECUTIVE_TEST_LOOPS=3
MAX_MISSING_STATUS=2
```

## State Files
All under `.loopr/`:
- `status.json` (public, current loop status)
- `loop-state.json` (internal counters)
- `.call_count` and `.last_reset` (rate limiting)
- `loop.log` (append-only run log)

## Exit Logic (MVP)
- **Complete**: `EXIT_SIGNAL=true` AND `completion_indicators >= MAX_CONSECUTIVE_DONE_SIGNALS`.
- **Circuit breaker**:
  - `no_progress >= MAX_NO_PROGRESS`
  - `same_error >= MAX_SAME_ERROR`
  - `consecutive_test_loops >= MAX_CONSECUTIVE_TEST_LOOPS`
  - `missing_status >= MAX_MISSING_STATUS`
- **Max iterations**: stop when `iteration >= MAX_ITERATIONS`.

## Observability
`status.json` includes:
- `state` (running | waiting | complete | blocked | error | circuit_open)
- `iteration`
- `exit_reason` (if any)
- `last_summary`
- `last_error`
- `call_count` and `next_reset_at`

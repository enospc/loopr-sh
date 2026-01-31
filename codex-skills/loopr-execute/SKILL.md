---
name: loopr-execute
description: Implement every task listed in specs/task-order.yaml in order, stopping on failures. Use when asked to execute the full ordered task list end-to-end in the Loopr workflow.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.
- This skill requires `mode` (read `.loopr/init-state.json`; if missing, assume `existing`).

# Loopr Execute

## Overview
Execute all tasks in specs/task-order.yaml sequentially using a strict tests-first flow. For each task, implement task-scoped unit tests first, make them pass, then mark the task complete only after unit tests are green.

## Inputs
- `specs/task-order.yaml`
- `specs/test-order.yaml`
- `specs/feature-*-task-*.md`
- `specs/feature-*-task-*-test-*.md`
- `.loopr/init-state.json` (for `mode`)
- `specs/spec.md`

## Outputs
- Code changes required by the tasks
- Updated task file completion sections
- `specs/implementation-progress.md`

## Workflow
1. **Preflight (mode-aware):**
   - Verify specs/task-order.yaml exists; if missing, stop and ask to run loopr-tasks.
   - Verify specs/test-order.yaml exists; if missing, stop and ask to run loopr-tests.
   - If `mode=greenfield`, verify the first feature is `foundation`; if missing, stop and ask to re-run loopr-features and loopr-tasks to add it.
2. Read specs/task-order.yaml and extract the ordered list of feature slugs and task IDs.
3. For each task in order:
   - Verify specs/feature-<slug>-task-<id>.md exists; if missing, stop and ask to regenerate tasks.
   - Verify tests for the task exist; if missing, stop and ask to run loopr-testify (or loopr-tests).
   - Use specs/spec.md testing strategy for PBT seed/budget guidance; if missing and PBT tests are present, stop and ask.
   - Implement or update the task-scoped **unit tests** first.
   - Run the task-scoped unit tests and confirm they fail for the expected reason.
   - Implement the task and iterate until task-scoped unit tests pass.
   - Run any remaining task-scoped tests (integration/e2e/manual) required by the test specs.
   - Update the task file with completion notes and test results.
   - If tests fail or implementation errors occur, stop and report the blocking task.
4. Maintain a simple progress log at specs/implementation-progress.md with completed tasks and timestamps.
   - On failure, record the exact failing test slug(s) (e.g., `feature-<slug>-task-<id>-test-<test_id>.md`) and the command that failed.

## Output requirements
- Do not skip tasks or reorder them.
- Stop immediately on failure; do not proceed to the next task.
- Never mark a task complete unless its task-scoped unit tests are green (unless the task file explicitly marks unit tests as not suitable).
- Keep specs/implementation-progress.md updated with completed task IDs and notes, including exact failing test slugs when applicable.
- If property-based tests are involved, record seed/replay information in the task completion notes and implementation-progress.md.
- Append a LOOPR_STATUS block at the end of your response:
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
Guidance:
- Use STATUS=COMPLETE with EXIT_SIGNAL=true only when all tasks are finished and tests are green.
- Use STATUS=BLOCKED when required inputs/tests are missing or a failure prevents progress.
- Use STATUS=ERROR when you encounter unexpected errors or tool failures.

## Version
- 2026-01-24

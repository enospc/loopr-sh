---
name: loopr-execute
description: Implement every task listed in specs/task-order.yaml in order, stopping on failures. Use when asked to execute the full ordered task list end-to-end in the Loopr workflow.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.
- This skill requires `mode` (read `specs/.loopr/init-state.json`; if missing, assume `existing`).

# Loopr Execute

## Overview
Execute all tasks in specs/task-order.yaml sequentially. Stop on first failure and report progress.

## Inputs
- `specs/task-order.yaml`
- `specs/test-order.yaml`
- `specs/feature-*-task-*.md`
- `specs/feature-*-task-*-test-*.md`
- `specs/.loopr/init-state.json` (for `mode`)
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
   - Implement the task and run the tests for that task.
   - Update the task file with completion notes and test results.
   - If tests fail or implementation errors occur, stop and report the blocking task.
4. Maintain a simple progress log at specs/implementation-progress.md with completed tasks and timestamps.
   - On failure, record the exact failing test slug(s) (e.g., `feature-<slug>-task-<id>-test-<test_id>.md`) and the command that failed.

## Output requirements
- Do not skip tasks or reorder them.
- Stop immediately on failure; do not proceed to the next task.
- Keep specs/implementation-progress.md updated with completed task IDs and notes, including exact failing test slugs when applicable.
- If property-based tests are involved, record seed/replay information in the task completion notes and implementation-progress.md.

## Version
- 2026-01-24

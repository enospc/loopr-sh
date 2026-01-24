---
name: loopr-execute
description: Implement every task listed in specs/task-order.yaml in order, using loopr-run-task for each task and stopping on failures. Use when asked to execute the full ordered task list end-to-end in the Loopr workflow.
---

## Prerequisite
- Run loopr-init to ensure repo-id and transcript logging are initialized.
- Ensure the repo is greenfield (empty) or already Loopr-managed; otherwise stop and clarify scope.
- Read `specs/.loopr/init-state.json` to determine `mode` (if missing, assume `existing`).

# Loopr Execute

## Overview
Execute all tasks in specs/task-order.yaml sequentially, using loopr-run-task for each task. Stop on first failure and report progress.

## Workflow
1. **Preflight (mode-aware):**
   - Run **loopr-doctor**; if it fails, stop and fix inputs.
   - Verify specs/task-order.yaml exists; if missing, stop and ask to run loopr-tasks.
   - If `mode=greenfield`, verify the first feature is `foundation`; if missing, stop and ask to re-run loopr-features and loopr-tasks to add it.
   - If specs/test-order.yaml is missing, generate it with loopr-tests before implementation.
2. Read specs/task-order.yaml and extract the ordered list of feature slugs and task IDs.
3. For each task in order:
   - Verify specs/feature-<slug>-task-<id>.md exists.
   - If tests for the task are missing, generate them using loopr-testify (or loopr-tests if none exist).
   - Run loopr-run-task on the task file.
   - If tests fail or implementation errors occur, stop and report the blocking task.
4. Maintain a simple progress log at specs/implementation-progress.md with completed tasks and timestamps.
   - On failure, record the exact failing test slug(s) (e.g., `feature-<slug>-task-<id>-test-<test_id>.md`) and the command that failed.

## Output requirements
- Do not skip tasks or reorder them.
- Stop immediately on failure; do not proceed to the next task.
- Keep specs/implementation-progress.md updated with completed task IDs and notes, including exact failing test slugs when applicable.

## Version
- 2026-01-24

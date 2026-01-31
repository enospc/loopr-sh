---
name: loopr-testify
description: Break a single task into tests and write specs/feature-<slug>-task-<id>-test-<id>.md. Use when asked to derive tests for one task in the Loopr workflow.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

# Loopr Testify

## Overview
Create focused tests that fully cover a task's acceptance criteria and edge cases.

## Inputs
- `specs/feature-<slug>-task-<id>.md`
- `specs/feature-<slug>.md`
- `specs/spec.md`
- `specs/test-order.yaml` (optional; update if present)
- `$CODEX_HOME/skills/loopr-common/test-templates.md` (if CODEX_HOME is set)
- `~/.codex/skills/loopr-common/test-templates.md` (fallback; use the first path that exists, otherwise stop and ask to reinstall Loopr skills)
- `$CODEX_HOME/skills/loopr-common/pbt-guidance.md` (if CODEX_HOME is set)
- `~/.codex/skills/loopr-common/pbt-guidance.md` (fallback; use the first path that exists, otherwise stop and ask to reinstall Loopr skills)

## Outputs
- `specs/feature-<slug>-task-<id>-test-*.md`
- `specs/test-order.yaml` (updated if present)

## Workflow
1. Read the task file and acceptance criteria.
2. Read the feature file, spec testing strategy, and the installed `loopr-common/pbt-guidance.md` (see Inputs) for PBT guidance. Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.
3. Identify test types (unit, integration, e2e, manual, property-based) as needed.
4. Create 1+ tests per acceptance criterion plus key edge cases.
   - Include at least one **Unit** test unless the task file explicitly marks unit tests as not suitable (with rationale).
   - If unit tests are not suitable, update the task file to mark `Unit tests required: No` with a brief rationale.
   - Follow the installed `loopr-common/pbt-guidance.md` (see Inputs) for PBT inclusion rules. Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.
5. Remove any existing specs/feature-<feature-slug>-task-<task_id>-test-*.md to avoid stale tests.
6. Write each test to specs/feature-<feature-slug>-task-<task_id>-test-<test_id>.md.

## Test template
Use the installed `loopr-common/test-templates.md` (see Inputs). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

## Output requirements
- Ensure specs/ exists.
- Use zero-padded numeric test IDs (01, 02, 03...).
- Keep each test atomic and readable.
- If specs/test-order.yaml exists, update it to reflect new/changed tests for this task.
- Include at least one **Unit** test unless the task explicitly marks unit tests as not suitable.
- If unit tests are not suitable, update the task file with the rationale.
- Do not invent a testing framework; if missing, stop and ask.

## Version
- 2026-01-24

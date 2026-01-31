---
name: loopr-tests
description: Generate test files for tasks listed in specs/task-order.yaml and write specs/test-order.yaml. Use when asked to expand ordered tasks into tests in the Loopr workflow.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

# Loopr Tests

## Overview
Read specs/task-order.yaml, then for each task in that order, generate test files using the Loopr test template and output a consolidated specs/test-order.yaml.
This assumes task-order.yaml follows the canonical format produced by loopr-tasks.

## Inputs
- `specs/task-order.yaml`
- `specs/feature-*-task-*.md`
- `specs/feature-*.md`
- `specs/spec.md`
- `$CODEX_HOME/skills/loopr-common/test-templates.md` (if CODEX_HOME is set)
- `~/.codex/skills/loopr-common/test-templates.md` (fallback; use the first path that exists, otherwise stop and ask to reinstall Loopr skills)
- `$CODEX_HOME/skills/loopr-common/pbt-guidance.md` (if CODEX_HOME is set)
- `~/.codex/skills/loopr-common/pbt-guidance.md` (fallback; use the first path that exists, otherwise stop and ask to reinstall Loopr skills)

## Outputs
- `specs/feature-<slug>-task-<id>-test-*.md`
- `specs/test-order.yaml`

## Workflow
1. Verify specs/task-order.yaml exists; if missing, stop and ask to run loopr-tasks.
2. Read specs/task-order.yaml and extract the ordered task IDs by feature.
3. Read specs/spec.md and the installed `loopr-common/pbt-guidance.md` (see Inputs) for testing stack and PBT guidance. Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.
4. For each task, open specs/feature-<slug>-task-<id>.md and specs/feature-<slug>.md; if missing, stop and ask to regenerate.
5. Derive tests covering each acceptance criterion and key edge cases.
   - Include at least one **Unit** test per task unless the task file explicitly marks unit tests as not suitable (with rationale).
   - If unit tests are not suitable, update the task file to mark `Unit tests required: No` with a brief rationale.
   - Follow the installed `loopr-common/pbt-guidance.md` (see Inputs) for PBT inclusion rules. Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.
6. Remove any existing specs/feature-<slug>-task-<id>-test-*.md to avoid stale tests.
7. Write new test files to specs/feature-<slug>-task-<id>-test-<test_id>.md.
8. Generate specs/test-order.yaml listing tests in the same task order, including brief notes.

## Test file format
Use the installed `loopr-common/test-templates.md` (see Inputs). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

## test-order.yaml format
Keep it machine-readable and ordered by feature, then task, then test:

```yaml
version: 1
features:
  - slug: <feature-slug>
    tasks:
      - id: "01"
        title: <task short title>
        tests:
          - id: "01"
            title: <short test title>
```

## Output requirements
- Ensure specs/ exists.
- Write tests to specs/feature-<slug>-task-<id>-test-<test_id>.md.
- Write specs/test-order.yaml.
- Preserve task order from specs/task-order.yaml.
- Include at least one **Unit** test per task unless the task explicitly marks unit tests as not suitable.
- If unit tests are not suitable, update the task file with the rationale.
- Do not invent a testing framework; if missing, stop and ask.

## Version
- 2026-01-24

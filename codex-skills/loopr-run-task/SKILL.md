---
name: loopr-run-task
description: Implement a single task end-to-end until all its tests pass. Use when asked to execute a Loopr task (specs/feature-*-task-*.md) and make the code changes required to pass its tests.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.

# Loopr Run Task

## Overview
Complete one task end-to-end, ensuring all associated tests pass before marking it done.

## Inputs
- `specs/feature-<slug>-task-<id>.md`
- `specs/feature-<slug>-task-<id>-test-*.md`
- `specs/spec.md`
- Codebase files relevant to the task

## Outputs
- Code changes required by the task
- Updated task file completion section

## Workflow
1. Read the task file and identify acceptance criteria and dependencies.
2. Locate associated tests; if missing, stop and ask to run $loopr-testify.
3. If property-based tests are present, consult specs/spec.md for seed/budget guidance; if missing, ask before running.
4. Implement the smallest change that satisfies each acceptance criterion.
5. Add or update tests per the test files.
6. Run tests; iterate until all task tests pass.
7. Update the task file with a clear completion note and test results.

## Execution rules
- Work in small, reversible steps.
- Avoid touching unrelated files.
- Prefer existing test commands; if unknown, ask or infer from project files.
- Log assumptions and open questions in the task file.
- Never mark done if tests are failing.
- If property-based tests are used, ensure deterministic execution (seeded runs) and record the seed and minimal failing case when applicable.

## Completion update
Append to the task file:
```
## Completion
- Status: Done
- Tests: <command(s) run and results>
- Notes: <any follow-ups; include PBT seed/repro info if applicable>
```

## Version
- 2026-01-24

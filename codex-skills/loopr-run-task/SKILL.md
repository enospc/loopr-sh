---
name: loopr-run-task
description: Implement a single task end-to-end until all its tests pass. Use when asked to execute a Loopr task (specs/feature-*-task-*.md) and make the code changes required to pass its tests.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

# Loopr Run Task

## Overview
Complete one task end-to-end with a strict tests-first flow: implement task-scoped unit tests first, make them pass, then mark the task done only after the unit tests are green.

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
4. Implement or update the task-scoped **unit tests** from the test files first.
5. Run the task-scoped unit tests and confirm they fail for the expected reason.
6. Implement the smallest code change that satisfies each acceptance criterion.
7. Re-run the task-scoped unit tests; iterate until they pass.
8. Run any remaining task-scoped tests (integration/e2e/manual) as required by the test specs.
9. Update the task file with a clear completion note and test results.

## Execution rules
- Work in small, reversible steps.
- Avoid touching unrelated files.
- Prefer existing test commands; if unknown, ask or infer from project files.
- Log assumptions and open questions in the task file.
- Never mark done if task-scoped unit tests are failing.
- If the task file explicitly marks unit tests as not suitable, include the rationale and still run the best available task-scoped tests.
- If property-based tests are used, ensure deterministic execution (seeded runs) and record the seed and minimal failing case when applicable.

## Completion update
Append to the task file:
```
## Completion
- Status: Done
- Tests (pre-implementation, expected fail): <task-scoped unit test command(s) + observed failure>
- Tests (post-implementation, must pass): <task-scoped unit test command(s) + pass result>
- Notes: <any follow-ups; include PBT seed/repro info if applicable>
```

Append a LOOPR_STATUS block at the end of your response:
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
- Use STATUS=COMPLETE with EXIT_SIGNAL=true only when the task is finished and tests are green.
- Use STATUS=BLOCKED when required inputs/tests are missing or a failure prevents progress.
- Use STATUS=ERROR when you encounter unexpected errors or tool failures.

## Version
- 2026-01-24

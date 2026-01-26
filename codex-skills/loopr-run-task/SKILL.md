---
name: loopr-run-task
description: Implement a single task end-to-end until all its tests pass. Use when asked to execute a Loopr task (specs/feature-*-task-*.md) and make the code changes required to pass its tests.
---

## Prerequisite
- Run `loopr init` (CLI) to ensure repo-id and transcript logging are initialized.
- Ensure the repo is greenfield (empty) or already Loopr-managed; otherwise stop and clarify scope.

# Loopr Run Task

## Overview
Complete one task end-to-end, ensuring all associated tests pass before marking it done.

## Workflow
1. Read the task file and identify acceptance criteria and dependencies.
2. Locate associated tests; if missing, invoke $loopr-testify.
3. Implement the smallest change that satisfies each acceptance criterion.
4. Add or update tests per the test files.
5. Run tests; iterate until all task tests pass.
6. Update the task file with a clear completion note and test results.

## Execution rules
- Work in small, reversible steps.
- Avoid touching unrelated files.
- Prefer existing test commands; if unknown, ask or infer from project files.
- Log assumptions and open questions in the task file.
- Never mark done if tests are failing.

## Completion update
Append to the task file:
```
## Completion
- Status: Done
- Tests: <command(s) run and results>
- Notes: <any follow-ups>
```

## Version
- 2026-01-24

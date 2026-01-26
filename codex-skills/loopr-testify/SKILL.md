---
name: loopr-testify
description: Break a single task into tests and write specs/feature-<slug>-task-<id>-test-<id>.md. Use when asked to derive tests for one task in the Loopr workflow.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.

# Loopr Testify

## Overview
Create focused tests that fully cover a task's acceptance criteria and edge cases.

## Inputs
- `specs/feature-<slug>-task-<id>.md`
- `specs/test-order.yaml` (optional; update if present)

## Outputs
- `specs/feature-<slug>-task-<id>-test-*.md`
- `specs/test-order.yaml` (updated if present)

## Workflow
1. Read the task file and acceptance criteria.
2. Identify test types (unit, integration, e2e, manual) as needed.
3. Create 1+ tests per acceptance criterion plus key edge cases.
4. Remove any existing specs/feature-<feature-slug>-task-<task_id>-test-*.md to avoid stale tests.
5. Write each test to specs/feature-<feature-slug>-task-<task_id>-test-<test_id>.md.

## Test template
```
# Test: <short title>

## Test ID
<test_id>

## Type
<Unit | Integration | E2E | Manual>

## Purpose

## Preconditions
- 

## Test Data
- 

## Steps
1. 

## Expected Results
- 

## Automation Notes
- 
```

## Output requirements
- Ensure specs/ exists.
- Use zero-padded numeric test IDs (01, 02, 03...).
- Keep each test atomic and readable.
- If specs/test-order.yaml exists, update it to reflect new/changed tests for this task.

## Version
- 2026-01-24

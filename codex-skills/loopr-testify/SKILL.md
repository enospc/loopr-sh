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
- `specs/feature-<slug>.md`
- `specs/spec.md`
- `specs/test-order.yaml` (optional; update if present)

## Outputs
- `specs/feature-<slug>-task-<id>-test-*.md`
- `specs/test-order.yaml` (updated if present)

## Workflow
1. Read the task file and acceptance criteria.
2. Read the feature file and spec testing strategy for PBT guidance.
   - If the feature marks PBT recommended and the framework is missing, stop and ask whether to select a framework or opt out.
   - If PBT is optional and the framework is missing, proceed with example-based tests and note the gap.
3. Identify test types (unit, integration, e2e, manual, property-based) as needed.
4. Create 1+ tests per acceptance criterion plus key edge cases.
   - If PBT is recommended, include at least one property-based test referencing feature invariants and generator notes.
   - If PBT is optional, include a property-based test only if the framework is known; otherwise use example-based tests and note the optional PBT gap.
   - If PBT is not suitable, use example-based tests only.
5. Remove any existing specs/feature-<feature-slug>-task-<task_id>-test-*.md to avoid stale tests.
6. Write each test to specs/feature-<feature-slug>-task-<task_id>-test-<test_id>.md.

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

### Property-based test template
```
# Test: <short title>

## Test ID
<test_id>

## Type
Property-based

## Purpose

## Properties
- 

## Generators
- 

## Preconditions
- 

## Test Data
- 

## Steps
1. Run the property tests with the configured budget and seed.

## Expected Results
- All properties hold across generated cases.

## Automation Notes
- Framework: <library>
- Budget: <iterations/time>
- Seed / replay: <how to reproduce a failure>
- Shrinking: <notes if supported>
```

## Output requirements
- Ensure specs/ exists.
- Use zero-padded numeric test IDs (01, 02, 03...).
- Keep each test atomic and readable.
- If specs/test-order.yaml exists, update it to reflect new/changed tests for this task.
- Do not invent a testing framework; if missing, stop and ask.

## Version
- 2026-01-24

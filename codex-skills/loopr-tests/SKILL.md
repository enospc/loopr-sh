---
name: loopr-tests
description: Generate test files for tasks listed in specs/task-order.yaml and write specs/test-order.yaml. Use when asked to expand ordered tasks into tests in the Loopr workflow.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.

# Loopr Tests

## Overview
Read specs/task-order.yaml, then for each task in that order, generate test files using the Loopr test template and output a consolidated specs/test-order.yaml.
This assumes task-order.yaml follows the canonical format produced by loopr-tasks.

## Inputs
- `specs/task-order.yaml`
- `specs/feature-*-task-*.md`
- `specs/feature-*.md`
- `specs/spec.md`

## Outputs
- `specs/feature-<slug>-task-<id>-test-*.md`
- `specs/test-order.yaml`

## Workflow
1. Verify specs/task-order.yaml exists; if missing, stop and ask to run loopr-tasks.
2. Read specs/task-order.yaml and extract the ordered task IDs by feature.
3. Read specs/spec.md for the testing stack and property-based testing guidance.
   - If a feature is marked PBT recommended and the framework is missing, stop and ask whether to select a framework or opt out.
   - If PBT is optional and the framework is missing, proceed with example-based tests and note the gap.
4. For each task, open specs/feature-<slug>-task-<id>.md and specs/feature-<slug>.md; if missing, stop and ask to regenerate.
5. Derive tests covering each acceptance criterion and key edge cases.
   - If PBT is recommended, include at least one property-based test referencing the feature invariants and generator notes.
   - If PBT is optional, include a property-based test only if the framework is known; otherwise use example-based tests and note the optional PBT gap.
   - If PBT is not suitable, use example-based tests only.
6. Remove any existing specs/feature-<slug>-task-<id>-test-*.md to avoid stale tests.
7. Write new test files to specs/feature-<slug>-task-<id>-test-<test_id>.md.
8. Generate specs/test-order.yaml listing tests in the same task order, including brief notes.

## Test file format
Use one of the templates below (match the chosen testing strategy):

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
- Do not invent a testing framework; if missing, stop and ask.

## Version
- 2026-01-24

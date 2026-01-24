---
name: loopr-tests
description: Generate test files for tasks listed in specs/task-order.yaml and write specs/test-order.yaml. Use when asked to expand ordered tasks into tests in the Loopr workflow.
---

## Prerequisite
- Run loopr-init to ensure repo-id and transcript logging are initialized.
- Ensure the repo is greenfield (empty) or already Loopr-managed; otherwise stop and clarify scope.

# Loopr Tests

## Overview
Read specs/task-order.yaml, then for each task in that order, generate test files using the Loopr test template and output a consolidated specs/test-order.yaml.
This assumes task-order.yaml follows the canonical format produced by loopr-tasks.

## Workflow
1. Preflight: run **loopr-doctor**; if it fails, stop and fix inputs.
2. Read specs/task-order.yaml and extract the ordered task IDs by feature.
3. For each task, open specs/feature-<slug>-task-<id>.md.
4. Derive tests covering each acceptance criterion and key edge cases.
5. Remove any existing specs/feature-<slug>-task-<id>-test-*.md to avoid stale tests.
6. Write new test files to specs/feature-<slug>-task-<id>-test-<test_id>.md.
7. Generate specs/test-order.yaml listing tests in the same task order, including brief notes.

## Test file format
Use the same template as loopr-testify:

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

## Version
- 2026-01-24

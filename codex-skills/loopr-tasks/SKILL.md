---
name: loopr-tasks
description: Generate task files for features listed in specs/feature-order.yaml and write specs/task-order.yaml. Use when asked to expand ordered features into tasks in the Loopr workflow.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.
- This skill requires `mode` (read `specs/.loopr/init-state.json`; if missing, assume `existing`).

# Loopr Tasks

## Overview
Read specs/feature-order.yaml, then for each feature in that order, generate task files using the Loopr task template and output a consolidated specs/task-order.yaml.
This assumes feature-order.yaml follows the canonical format produced by loopr-features.

## Inputs
- `specs/feature-order.yaml`
- `specs/feature-*.md`
- `specs/.loopr/init-state.json` (for `mode`)

## Outputs
- `specs/feature-<slug>-task-*.md`
- `specs/task-order.yaml`

## Workflow
1. Verify specs/feature-order.yaml exists; if missing, stop and ask to run loopr-features.
2. Read specs/feature-order.yaml and extract the ordered feature slugs.
   - If `mode=greenfield`, verify the first feature is `foundation`; if missing, stop and ask to re-run loopr-features.
3. For each slug, open specs/feature-<slug>.md; if missing, stop and ask to regenerate features.
4. Derive tasks (0.5–2 days each) in dependency order with zero-padded IDs.
   - If the feature slug is `foundation`, include tasks for repo scaffold, test harness smoke tests, and cross-component contract stubs.
5. Remove any existing specs/feature-<slug>-task-*.md to avoid stale tasks.
6. Write new task files to specs/feature-<slug>-task-<id>.md.
7. Generate specs/task-order.yaml listing tasks in the same feature order, including brief dependency notes.

## Task file format
Use the same template as loopr-taskify:

```
# Task: <feature title> / <task short title>

## Task ID
<task_id>

## Summary

## Goal

## Scope
- In scope:
- Out of scope:

## Acceptance Criteria
- 

## Implementation Plan
- 

## Dependencies
- 

## Risks
- 

## Test Plan
- 

## Notes
- 
```

## task-order.yaml format
Keep it machine-readable and ordered by feature, then task:

```yaml
version: 1
features:
  - slug: <feature-slug>
    title: <short feature title>
    depends_on: []
    tasks:
      - id: "01"
        title: <short task title>
      - id: "02"
        title: <short task title>
```

## Output requirements
- Ensure specs/ exists.
- Write tasks to specs/feature-<slug>-task-<id>.md.
- Write specs/task-order.yaml.
- Preserve feature order from specs/feature-order.yaml.

## Version
- 2026-01-24

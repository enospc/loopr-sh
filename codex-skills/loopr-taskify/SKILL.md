---
name: loopr-taskify
description: Break a single feature into tasks and write specs/feature-<slug>-task-<id>.md. Use when asked to plan tasks for one feature in the Loopr workflow.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.

# Loopr Taskify

## Overview
Turn a feature document into a set of executable tasks sized for safe, incremental delivery.

## Inputs
- `specs/feature-<slug>.md`
- `specs/task-order.yaml` (optional; update if present)

## Outputs
- `specs/feature-<slug>-task-*.md`
- `specs/task-order.yaml` (updated if present)

## Workflow
1. Read the feature file and identify distinct implementation steps.
2. Split work into tasks sized for 0.5-2 days.
3. Assign task IDs starting at 01 in dependency order.
4. Remove any existing specs/feature-<feature-slug>-task-*.md to avoid stale tasks.
5. Write each task to specs/feature-<feature-slug>-task-<task_id>.md.

## Task template
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

## Output requirements
- Ensure specs/ exists.
- Use zero-padded numeric task IDs (01, 02, 03...).
- Keep tasks independent where possible; note blockers explicitly.
- If specs/task-order.yaml exists, update it to reflect new/changed tasks for this feature.

## Version
- 2026-01-24

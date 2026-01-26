---
name: loopr-taskify
description: Break a single feature into tasks and write specs/feature-<slug>-task-<id>.md. Use when asked to plan tasks for one feature in the Loopr workflow.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

# Loopr Taskify

## Overview
Turn a feature document into a set of executable tasks sized for safe, incremental delivery.

## Inputs
- `specs/feature-<slug>.md`
- `specs/task-order.yaml` (optional; update if present)
- `$CODEX_HOME/skills/loopr-common/task-template.md` (if CODEX_HOME is set)
- `~/.codex/skills/loopr-common/task-template.md` (fallback; use the first path that exists, otherwise stop and ask to reinstall Loopr skills)
- `$CODEX_HOME/skills/loopr-common/pbt-guidance.md` (if CODEX_HOME is set)
- `~/.codex/skills/loopr-common/pbt-guidance.md` (fallback; use the first path that exists, otherwise stop and ask to reinstall Loopr skills)

## Outputs
- `specs/feature-<slug>-task-*.md`
- `specs/task-order.yaml` (updated if present)

## Workflow
1. Read the feature file and identify distinct implementation steps.
2. Split work into tasks sized for 0.5-2 days.
3. Assign task IDs starting at 01 in dependency order.
4. Follow the installed `loopr-common/pbt-guidance.md` (see Inputs) for PBT-related task criteria. Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.
5. Remove any existing specs/feature-<feature-slug>-task-*.md to avoid stale tasks.
6. Write each task to specs/feature-<feature-slug>-task-<task_id>.md.

## Task template
Use the installed `loopr-common/task-template.md` (see Inputs). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

## Output requirements
- Ensure specs/ exists.
- Use zero-padded numeric task IDs (01, 02, 03...).
- Keep tasks independent where possible; note blockers explicitly.
- If specs/task-order.yaml exists, update it to reflect new/changed tasks for this feature.

## Version
- 2026-01-24

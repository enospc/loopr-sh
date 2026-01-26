---
name: loopr-runner
description: Orchestrate the full Loopr workflow (prd to spec to features to tasks to tests to execution) by invoking the appropriate Loopr skills. Use when asked to run the end-to-end workflow for a project or feature set.
---

## Prerequisite
- Run `loopr init` (CLI) to ensure repo-id and transcript logging are initialized.
- Ensure the repo is greenfield (empty) or already Loopr-managed; otherwise stop and clarify scope.

# Loopr Runner

## Overview
Run the full Loopr workflow, generating artifacts and executing tasks until tests pass.

## Workflow
1. Preflight: verify the repo is greenfield and safe to proceed.
2. If specs/prd.md is missing, invoke $loopr-prd.
3. If specs/spec.md is missing or stale, invoke $loopr-specify.
4. Generate features and feature order with $loopr-features.
5. Generate ordered tasks with $loopr-tasks.
6. Generate ordered tests with $loopr-tests.
7. Execute tasks in order with $loopr-execute.

## Discovery helpers
- Feature files: specs/feature-*.md
- Feature order: specs/feature-order.yaml
- Task files: specs/feature-*-task-*.md
- Task order: specs/task-order.yaml
- Test files: specs/feature-*-task-*-test-*.md
- Test order: specs/test-order.yaml

## Execution rules
- Skip steps that are already complete and up to date.
- Run tasks in dependency order when possible.
- Stop and ask if blockers or missing context appear.

## Model guidance
If running in Codex CLI, use model gpt-5.2 with reasoning xhigh (or an equivalent profile).

## Version
- 2026-01-24

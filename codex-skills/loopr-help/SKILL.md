---
name: loopr-help
description: Guide users through the Loopr workflow and the correct order of loopr-* skills. Use when a user asks how to use the Loopr process, wants onboarding, or needs a greenfield repo walkthrough from a seed prompt.
---

## Prerequisite
- None. This skill is informational only and must not trigger other skills.

# Loopr Help

## Overview
Provide a thorough, step-by-step guide for using the Loopr workflow, including the correct order of loopr-* skills, when to use each, and a non-trivial greenfield example from a seed prompt. This workflow is optimized for brand-new, empty repos; existing repos require explicit `loopr init --allow-existing` and may skip foundation.

Important: This skill must not invoke or trigger any other skills. It should only explain what to run and when, without executing anything.

## Greenfield Preflight
Before starting the workflow in a new repo, confirm it is truly greenfield. If `specs/.loopr/repo-id` exists, treat the repo as Loopr-managed and proceed:
- Allowed: `.git/`, `.github/`, `.vscode/`, `docs/`, `specs/`, `README.md`, `LICENSE`, `.gitignore` (or similarly empty scaffolding).
- Not allowed: app code or build tooling (examples: `package.json`, `go.mod`, `Cargo.toml`, `pyproject.toml`, `pom.xml`, `build.gradle`, `src/`, `app/`, `backend/`, `frontend/`).
If disallowed signals exist, stop and ask for confirmation.

## Workflow Decision Tree
- If the repo is not greenfield → stop and clarify scope.
- If you only have a seed idea or prompt → use **loopr-prd**.
- If you already have a PRD in `specs/prd.md` → use **loopr-specify**.
- If you already have a spec in `specs/spec.md` → use **loopr-features**.
- If you already have features in `specs/feature-*.md` **and** `specs/feature-order.yaml` → use **loopr-tasks** (or **loopr-taskify** for a single feature).
- If you already have tasks in `specs/task-order.yaml` → use **loopr-tests** (or **loopr-testify** for a single task).
- To execute tasks end-to-end → use **loopr-execute**.
- To execute a single task → use **loopr-run-task**.
- To run everything end-to-end (from PRD onward) → use **loopr-runner**.
- To validate order files and references at any point → use **loopr-doctor**.

## Canonical Order (Greenfield)
1. **loopr init (CLI)**
2. **loopr-prd**
3. **loopr-specify**
4. **loopr-features**
5. **loopr-tasks**
6. **loopr-tests**
7. **loopr-execute**

## Skill Map (What Each Does)
- **loopr init (CLI)**: Creates repo-id and transcript location under `specs/.loopr/` (idempotent).
- **loopr-prd**: MCQ interview → `specs/prd.md`.
- **loopr-specify**: Expands PRD → `specs/spec.md` with requirement IDs.
- **loopr-features**: Splits spec → `specs/feature-*.md` + `specs/feature-order.yaml` (includes `foundation` for greenfield).
- **loopr-tasks**: Generates ordered task files + `specs/task-order.yaml`.
- **loopr-tests**: Generates test files + `specs/test-order.yaml`.
- **loopr-run-task**: Implements a single task to completion and appends completion notes.
- **loopr-execute**: Executes all tasks in order and stops on first failure.
- **loopr-runner**: Orchestrates the full Loopr pipeline end-to-end.
- **loopr-doctor**: Validates order YAML files and referenced artifacts before implementation.

## Greenfield Example (Lightweight)
**Seed prompt:**
"Build a simple local CLI that tracks personal TODOs, stores them in a local SQLite database, and exports to CSV."

**Step-by-step guide:**
1) Run `loopr init`
- Creates `specs/.loopr/repo-id` and session log path under `specs/.loopr/transcripts/<repo-id>/`.

2) Run **loopr-prd**
- Answer MCQ interview to clarify surfaces, users, goals, timeline.
- Output: `specs/prd.md`.

3) Run **loopr-specify**
- Expands the PRD into implementable requirements.
- Output: `specs/spec.md`.

4) Run **loopr-features**
- Splits the spec into orthogonal features.
- Outputs: `specs/feature-*.md` and `specs/feature-order.yaml` (with `foundation` first).

5) Run **loopr-tasks**
- Converts features into tasks in dependency order.
- Outputs: `specs/feature-*-task-*.md` and `specs/task-order.yaml`.

6) Run **loopr-tests**
- Converts tasks into test specs.
- Outputs: `specs/feature-*-task-*-test-*.md` and `specs/test-order.yaml`.

7) Run **loopr-execute**
- Executes tasks sequentially and stops on first failure.
- Writes progress to `specs/implementation-progress.md`.

## Updating Requirements Mid-Stream
- If PRD changes → re-run **loopr-specify**, **loopr-features**, **loopr-tasks**, **loopr-tests**.
- If Spec changes → re-run **loopr-features** and downstream skills.
- If a Feature changes → re-run **loopr-tasks** (or **loopr-taskify** for that feature) and downstream tests.
- If only a Task changes → re-run **loopr-testify** for that task.

## Common Pitfalls & Guardrails
- **Skipping `loopr init`**: you lose session correlation and transcript consistency.
- **Divergent docs**: keep PRD/spec/features in sync; rerun downstream skills after changes.
- **Over-sized tasks**: split tasks into 0.5–2 day units; keep dependencies explicit.
- **Unbounded scope creep**: update PRD/spec and reflow the pipeline before coding.
- **Missing foundation**: ensure `foundation` is first in feature/task order for greenfield scaffolding.

## When to Prefer loopr-runner
- Use **loopr-runner** if you want a single command to walk from PRD to tasks/tests.
- Use manual steps if you need tighter control between phases.

## Version
- 2026-01-24

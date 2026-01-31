---
name: loopr-doctor
description: Validate Loopr order YAML files and referenced feature/task/test artifacts. Use as a preflight before generating tasks/tests or before implementation, and anytime order files change.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

# Loopr Doctor

## Overview
Validate the Loopr order artifacts (`feature-order.yaml`, `task-order.yaml`, `test-order.yaml`) and their referenced feature/task/test files. Enforce greenfield invariants like `foundation` being first only when `.loopr/init-state.json` indicates `mode=greenfield`, and catch broken references before implementation.

## Inputs
- `specs/feature-order.yaml`
- `specs/task-order.yaml`
- `specs/test-order.yaml`
- Referenced `specs/feature-*.md`, `specs/feature-*-task-*.md`, `specs/feature-*-task-*-test-*.md`
- `.loopr/init-state.json` (optional; for `mode`)

## Outputs
- Validation report on stdout/stderr

## Workflow
1. Ensure `specs/` exists.
2. Run the doctor command:
   - `loopr doctor --specs --specs-dir specs`
   - Optional: add `--enforce-unit-tests` to treat missing task-scoped unit tests as errors (default: warnings only).
3. If validation fails, fix the reported issues and re-run until it passes.
4. If `loopr` is missing, stop and ask to install Loopr before running validation.

## Checks performed
- Required files exist: `specs/feature-order.yaml`, `specs/task-order.yaml`, `specs/test-order.yaml`
- `feature-order.yaml` has a non-empty features list and `foundation` is first when `mode=greenfield`
- Feature/task/test IDs are numeric strings (preserve zero padding)
- Order files reference existing feature/task/test markdown files
- Duplicate IDs or unknown references are flagged
- Missing tests are reported as warnings
- Missing task-scoped unit tests are reported as warnings (errors if `--enforce-unit-tests` is set), unless the task file explicitly marks unit tests as not suitable.
- Advisory checks (warnings): feature files include `Invariants / Properties` and `PBT Suitability` sections; spec includes `Testing Strategy`

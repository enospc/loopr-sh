---
name: loopr-doctor
description: Validate Loopr order YAML files and referenced feature/task/test artifacts. Use as a preflight before generating tasks/tests or before implementation, and anytime order files change.
---

# Loopr Doctor

## Overview
Validate the Loopr order artifacts (`feature-order.yaml`, `task-order.yaml`, `test-order.yaml`) and their referenced feature/task/test files. Enforce greenfield invariants like `foundation` being first only when `specs/.loopr/init-state.json` indicates `mode=greenfield`, and catch broken references before implementation.

## Workflow
1. Ensure `specs/` exists and the repo is greenfield or already Loopr-managed.
2. Run the doctor script:
   - `python3 ~/.codex/skills/loopr-doctor/scripts/loopr-doctor --specs-dir specs`
3. If validation fails, fix the reported issues and re-run until it passes.
4. If `python3` or PyYAML is missing, stop and ask to install prerequisites or proceed without validation.

## Checks performed
- Required files exist: `specs/feature-order.yaml`, `specs/task-order.yaml`, `specs/test-order.yaml`
- `feature-order.yaml` has a non-empty features list and `foundation` is first when `mode=greenfield`
- Feature/task/test IDs are numeric strings (preserve zero padding)
- Order files reference existing feature/task/test markdown files
- Duplicate IDs or unknown references are flagged
- Missing tests are reported as warnings

## Resources
### scripts/loopr-doctor
Python validator for Loopr order files.

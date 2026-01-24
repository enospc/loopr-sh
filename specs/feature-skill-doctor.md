---
order: 3
depends_on:
  - cli-core
---

# Feature: Skill drift detection and listing

## Summary
Validate installed skills against the embedded source of truth and report missing, drifted, or extra skills. Provide a list view of skill status.

## Goals
- Make drift and missing skills visible before running the workflow.
- Provide a quick summary of skill status via `list`.

## Non-goals
- Repairing drift automatically (handled by install).
- Tracking telemetry or remote reporting.

## User Stories
- As a developer, I want to verify skills are installed correctly so that the workflow runs reliably.
- As a developer, I want a compact list of skills and status.

## Scope
- In scope:
  - Compare installed skills with embedded skills by hash.
  - Report status per skill: installed, missing, drifted.
  - Report extra skills that exist locally but not in embedded set.
  - Provide verbose output for missing/drifted files.
- Out of scope:
  - Automatic remediation.

## Requirements
- Use embedded skills as the source of truth for comparisons.
- `doctor` reports status per skill and returns non-zero when drift or extras exist.
- `list` prints skill names with status derived from doctor results.
- Support `--only`, `--agent`, `--all`, and `--verbose`.

## Acceptance Criteria
- `loopr doctor` reports missing/drifted skills and extra skills accurately.
- `loopr list` prints skill names with status and respects `--only`.
- Verbose mode prints missing or drifted file paths.

## UX / Flow
- `loopr doctor [--verbose]` prints status lines and extra count.
- `loopr list` prints a compact tabular status.

## Data / API Impact
- Reads installed skills from skills root; no writes.

## Dependencies
- CLI command parsing and agent resolution.

## Risks & Mitigations
- Risk: false positives if skills root is incorrect → Mitigation: consistent root resolution logic.

## Open Questions
- Should doctor optionally auto-fix by invoking install?

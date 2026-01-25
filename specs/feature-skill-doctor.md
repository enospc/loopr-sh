---
order: 5
depends_on:
  - cli-core
---

# Feature: Skill drift detection and listing

## Summary
Provide `doctor` and `list` commands to compare installed skills against embedded sources, reporting drift and status.

## Goals
- Detect missing or drifted skills compared to embedded source of truth.
- Expose a consistent, script-friendly listing of skill status.

## Non-goals
- Installing or removing skills (handled by skill-install).

## User Stories
- As a developer, I want to know if my installed Loopr skills have drifted.
- As a developer, I want a quick list of skills and their status.

## Scope
- In scope:
  - `loopr doctor` command for per-skill status.
  - `loopr list` command built on doctor results.
  - Reporting extra installed skills not present in embedded list.
- Out of scope:
  - Writing or modifying skill files.

## Requirements
- Compare installed skills against embedded skills using content hashes.
- Report status per skill: `installed`, `missing`, or `drifted`.
- Report extra installed skills not present in the embedded list.
- Support filtering by `--agent`, `--all`, `--only`, and `--verbose`.
- `list` should reuse doctor logic and print skill names with status.

## Acceptance Criteria
- `loopr doctor` reports correct status for all embedded skills.
- `loopr doctor` reports extra installed skills separately.
- `loopr list` prints skill names with their status for the selected agent(s).

## UX / Flow
- `loopr doctor` → prints per-skill status and drift details when `--verbose`.
- `loopr list` → prints skill names with status.

## Data / API Impact
- CLI flags: `--agent`, `--all`, `--only`, `--verbose`.

## Dependencies
- CLI core for command parsing and flag handling.

## Risks & Mitigations
- Risk: false drift reports due to newline normalization → Mitigation: hash raw bytes as stored.

## Open Questions
- Should `list` output be machine-readable (e.g., JSON) in addition to text?

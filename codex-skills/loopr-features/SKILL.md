---
name: loopr-features
description: Split specs/spec.md into feature documents at specs/feature-<slug>.md and generate specs/feature-order.yaml. Use when asked to derive features from the spec in the Loopr workflow.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.
- This skill requires `mode` (read `specs/.loopr/init-state.json`; if missing, assume `existing`).

# Loopr Features

## Overview
Split specs/spec.md into independent, implementable features, generate a pragmatic dependency-based order, and write each feature to specs/feature-<feature-slug>.md with ordering metadata. For greenfield repos (`mode=greenfield`), always include a Foundation feature for scaffolding and test harness setup.

## Inputs
- `specs/spec.md`
- `specs/.loopr/init-state.json` (for `mode`)

## Outputs
- `specs/feature-*.md`
- `specs/feature-order.yaml`

## Workflow
1. Read specs/spec.md and identify distinct features.
2. If `mode=greenfield`, include a **Foundation** feature for repo scaffolding, tooling, and a test harness smoke test. This must be first in the order and have no dependencies. If `mode=existing`, the foundation feature is optional.
3. Prefer orthogonal features with minimal coupling.
4. Generate a short slug for each feature (kebab-case, unique).
5. Determine dependencies between features and produce a pragmatic build order.
6. Create one file per feature using the template, including ordering metadata.
7. Write an ordered list file (specs/feature-order.yaml) with the recommended sequence and brief dependency rationale.

## Slug rules
- Lowercase kebab-case.
- Keep under 40 characters when possible.
- If a collision exists, append a short suffix (e.g., -v2 or -auth).
- Reserve `foundation` for the greenfield setup feature (when `mode=greenfield`).

## Feature template
```

## feature-order.yaml format
Keep it machine-readable and stable. Include `foundation` only when `mode=greenfield`:

```yaml
version: 1
features:
  - slug: foundation
    title: Repository scaffolding and test harness
    depends_on: []
  - slug: <feature-slug>
    title: <short title>
    depends_on:
      - <feature-slug>
```
---
order: <integer>
depends_on:
  - <feature-slug>
  - <feature-slug>
---

# Feature: <title>

## Summary

## Goals
- 

## Non-goals
- 

## User Stories
- As a <user>, I want <capability> so that <benefit>.

## Scope
- In scope:
- Out of scope:

## Requirements
- 

## Acceptance Criteria
- 

## UX / Flow
- 

## Data / API Impact
- 

## Dependencies
- 

## Risks & Mitigations
- 

## Open Questions
- 
```

## Output requirements
- Ensure specs/ exists.
- Write each feature to specs/feature-<feature-slug>.md.
- Add ordering metadata to each feature file (order + depends_on).
- Write specs/feature-order.yaml listing the ordered sequence with brief dependency notes.

## Version
- 2026-01-24

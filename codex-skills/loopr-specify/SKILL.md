---
name: loopr-specify
description: Expand specs/prd.md into a detailed, implementable specification at specs/spec.md. Use when asked to turn a PRD into a spec in the Loopr workflow.
---

## Prerequisite
- Follow the installed `loopr-common/COMMON.md` (use `$CODEX_HOME/skills/loopr-common/COMMON.md` if set, otherwise `~/.codex/skills/loopr-common/COMMON.md`). Use the first path that exists; if neither exists, stop and ask to reinstall Loopr skills.

# Loopr Specify

## Overview
Transform specs/prd.md into a detailed, implementable spec at specs/spec.md.

## Inputs
- `specs/prd.md`
- `.loopr/init-state.json` (optional; for mode)

## Outputs
- `specs/spec.md`

## Workflow
1. Read specs/prd.md and note goals, scope, and open questions.
2. Ask up clarifying questions if key details are missing.
3. Produce a detailed spec with clear requirements, acceptance criteria, and a testing strategy (including property-based testing guidance when suitable).
4. Write specs/spec.md.
5. For greenfield repos, include explicit foundation requirements (repo scaffolding, test harness, and build/test entry points).

## Spec template
Use concise Markdown and include requirement IDs for traceability.

```
# Spec: <title>

## Summary

## Goals
- 

## Non-goals
- 

## Users & Use Cases
- 

## Functional Requirements
- FR-01: 
- FR-02: 

## Foundation / Tooling
- FD-01: 
- FD-02: 

## Testing Strategy
- Stack: <language + test framework>
- Property-based testing: <recommended | optional | not suitable> + <library or TBD>
- Invariants / properties: <list key properties that must hold>
- Determinism: <seed policy, time/iteration budget, replay notes>

## Non-functional Requirements
- NFR-01: 
- NFR-02: 

## UX / Flow
- 

## Data Model
- 

## API / Interfaces
- 

## Architecture / Components
- 

## Error Handling
- 

## Security & Privacy
- 

## Observability
- Logs:
- Metrics:
- Alerts:

## Rollout / Migration
- 

## Risks & Mitigations
- 

## Open Questions
- 

## Acceptance Criteria
- 
```

## Output requirements
- Ensure specs/ exists.
- Write the spec to specs/spec.md.
- Keep wording precise and implementation-ready.

## Version
- 2026-01-24

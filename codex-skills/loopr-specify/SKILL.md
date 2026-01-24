---
name: loopr-specify
description: Expand specs/prd.md into a detailed, implementable specification at specs/spec.md. Use when asked to turn a PRD into a spec in the Loopr workflow.
---

## Prerequisite
- Run loopr-init to ensure repo-id and transcript logging are initialized.
- Ensure the repo is greenfield (empty) or already Loopr-managed; otherwise stop and clarify scope.

# Loopr Specify

## Overview
Transform specs/prd.md into a detailed, implementable spec at specs/spec.md.

## Workflow
1. Read specs/prd.md and note goals, scope, and open questions.
2. Ask up clarifying questions if key details are missing.
3. Produce a detailed spec with clear requirements and acceptance criteria.
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

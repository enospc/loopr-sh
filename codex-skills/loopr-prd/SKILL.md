---
name: loopr-prd
description: Create a PRD from a seed prompt using an MCQ-style interview with intelligent defaults, then write it to specs/prd.md for the Loopr workflow. Use when asked to draft or refine a PRD from an initial idea.
---

## Prerequisite
- Follow `codex-skills/loopr-common/COMMON.md`.

# Loopr PRD

## Overview
Create a clear PRD by interviewing the author with MCQs thorough, confirming assumptions, and writing specs/prd.md.

## Inputs
- Seed prompt and explicit constraints (user-provided).
- Existing `specs/prd.md` (optional, if refining).

## Outputs
- `specs/prd.md`

## Workflow
1. Capture the seed prompt and any explicit constraints.
2. Run a MCQ interview (one at a time).
3. Summarize assumptions and ask for confirmation.
4. Write specs/prd.md using the template.

## MCQ interview rules
- Ask one question at a time.
- Provide 3-6 options plus "Other / Not sure".
- Select an intelligent default from the seed prompt and label it "Default:".
- If the user selects "Other", ask for a brief free-text answer and move on.
- Stop early if the PRD already has enough detail.
- Formatting: write the question as a standalone sentence (not numbered), then list options as a numbered list starting at 1. The question itself must never be an item in the numbered list.

Example:
```
What is the primary user?
1. Default: Developers evaluating Loopr for new projects
2. Admin/operators managing internal tooling
3. Internal team only
4. Decision makers / engineering managers
5. Other / Not sure
```

## MCQ question bank (pick only relevant)
1. Product surface: Web app, Mobile app, API/SDK, CLI, Integration/automation, Backend service, Other.
2. Primary user: End user, Admin/operator, Internal team, Developer, Other.
3. Primary goal: Revenue, Retention, Cost reduction, Reliability, Compliance, Delivery speed, Other.
4. Success metric type: Adoption, Conversion, Performance, Cost, Quality, Reliability, Other.
5. Data sensitivity: Public, Internal, PII, PHI, PCI, Other/Unknown.
6. Timeline: No fixed date, 2 weeks, 1 month, Quarter, Fixed date.
7. Rollout: Internal only, Beta, GA.
8. Tech constraints: Use existing stack, Open to change, Must integrate with X.
9. Non-goals: Operational analytics, Payment processing, Cross-team migration, Other.
10. Primary risks: Tech risk, Adoption risk, Compliance risk, Dependency risk, Other.

## PRD template
Write concise Markdown and keep bullets short.

```
# PRD: <title>

## Summary

## Problem / Opportunity

## Goals
- 

## Non-goals
- 

## Users & Use Cases
- 

## Scope
- 

## Requirements (high level)
- 

## Success Metrics
- 

## Assumptions
- 

## Constraints
- 

## UX Notes / Flows
- 

## Risks & Mitigations
- 

## Dependencies
- 

## Open Questions
- 
```

## Output requirements
- Ensure specs/ exists; create it if needed.
- Write the PRD to specs/prd.md.
- Include the seed prompt and a short "Interview summary" before the PRD if helpful.

## Version
- 2026-01-24

# LOOPR

**Build. Verify. Iterate.**

*A field guide for AI software engineers*

---

## Foreword

This is a guide for AI software engineers (human's as of today ;) ) in the era of AI coding agents. The center of gravity has shifted from writing every line of source code to defining intent, verifying outcomes, and building systems that evolve safely. The Loopr AI engineer respects models but does not defer to them. They set clear goals, curate context, and build feedback loops that turn uncertainty into reliable software. A 500-line patch from an agent is not progress until you can explain it and test it.

This guide distinguishes automatic programming from vibe coding. Automatic programming means humans set the vision, steer continuously, and understand the system they are building while the model supplies velocity. Vibe coding means accepting unsteered output without deep understanding. The former is the goal, the latter is a useful but risky exploration mode.

If you use Codex or any frontier model, the principles below apply. Models will change. The responsibilities will not. When output is cheap, judgment is expensive.

## How To Read This

- Read for decisions, not rules. The work is choosing tradeoffs, not memorizing techniques.
- Start with the sections that match your current constraints.
- Treat this as a living playbook. Revise it when your tooling or team changes.

## Contents

- **Part 1. The Loopr Mindset**

    1. Own Outcomes, Not Output
    2. Design for Verification
    3. Short Loops, Fast Reality
    4. Software Entropy in the Age of Agents
- **Part 2. Intent, Specs, and Contracts**

    5. Specs That Compile
    6. Interfaces First, Always
    7. Reversibility and Decision Logs
    8. Data Is a Product
- **Part 3. Collaboration With Agents**

    9. Roles, Boundaries, and Delegation
    10. Context Hygiene and Retrieval
    11. Determinism, Reproducibility, and Seeds
    12. Prompt Assets as Code
- **Part 4. Quality and Reliability**

    13. Tests as the Primary Oracle
    14. Property-Based Testing and Fuzzing
    15. Observability by Default
    16. Failure Containment and Graceful Degradation
- **Part 5. Architecture and Systems**

    17. Boundaries, Not Monoliths or Microservices
    18. Event-Driven and Async by Design
    19. Performance, Cost, and Token Budgets
    20. Evolve the Build and Release Pipeline
- **Part 6. Security, Safety, and Compliance**

    21. Threat Models for Agents
    22. Secrets, Permissions, and Isolation
    23. Supply Chain and Provenance
    24. Human-in-the-Loop as a Safety Valve
- **Part 7. Teams and Delivery**

    25. AI Pair Programming Rituals
    26. Review as Collaboration, Not Judgment
    27. Documentation That Stays True
    28. Operate What You Build
- **Part 8. Craft and Future-Proofing**

    29. The Modern Knowledge Portfolio
    30. Build Internal Platforms, Not One-Offs
    31. Ethics and Product Impact
    32. Stay Adaptable
- **Appendices**
  - A. Plan Template
  - B. Spec Template
  - C. Agent Handoff Template
  - D. Review Checklist
  - E. Incident Checklist
  - F. Glossary

---

# Part 1. The Loopr Mindset

## 1. Own Outcomes, Not Output

AI agents can produce more code than any team. Your job is to ensure that code is correct, maintainable, and aligned with the product. Outcomes are what users experience; output is just text. Do not confuse the two. Your name is on the incident, not the prompt.
If you are steering the work, the output is yours, and so are the consequences.

- Set measurable goals before asking an agent to write code.
- Define success conditions that can be tested.
- Hold yourself accountable for what ships, not what was generated.

## 2. Design for Verification

Verification is not a phase; it is the core of the workflow. If you cannot verify it, you cannot trust it. If it cannot be tested, it is not real.

- Build tests as part of the design, not as a follow-up.
- Prefer checks that run automatically and deterministically.

Assume that AI can be confident and wrong at the same time.

## 3. Short Loops, Fast Reality

Agents accelerate iteration, but only if feedback is fast. Avoid long pipelines that hide errors. Optimize for quick, repeated loops. A 15-minute loop beats a heroic weekend.

- Prototype in hours, not weeks.
- Integrate continuously, not at the end.
- Keep experiments small enough to discard.

## 4. Software Entropy in the Age of Agents

Automation can create entropy faster than humans can fix it. Every generated change should reduce complexity or increase clarity. If a change adds entropy, reject it even if it works.

- Refactor as you go. Do not defer cleanup.
- Track tech debt with the same rigor as features.
- Favor clarity over cleverness, even for AI-generated code.

---

# Part 2. Intent, Specs, and Contracts

> **Vision Is Not Automatic**  
> Models can execute and explore, but vision, product direction, and tradeoffs remain human. If you cannot explain why a system exists and what it optimizes, you are not done.

## 5. Specs That Compile

A good spec is executable in the mind of both model and human. It includes constraints, behavior, and non-goals. If a requirement cannot be turned into a test, it is probably a wish.

- Write requirements in testable terms.
- Explicitly list what the system must not do.
- Include data shapes and edge cases.

## 6. Interfaces First, Always

Define boundaries before implementation. Interfaces are the contract between human intent and agent execution. Types and schemas are the most durable form of documentation.

- Specify inputs, outputs, errors, and latency budgets.
- Treat interfaces as stable even when implementations change.

## 7. Reversibility and Decision Logs

Architecture is a series of reversible bets. Make it easy to change course and explain why you chose what you did. Document the why, not the debate.

- Maintain a short decision log with dates and context.
- Keep migration paths open for key dependencies.
- Prefer simple choices you can revisit.

## 8. Data Is a Product

Models depend on data. Bad data makes good code useless. Privacy and governance are design inputs, not afterthoughts.

- Own data quality: validation, lineage, and freshness.
- Treat data pipelines as first-class services.

---

# Part 3. Collaboration With Agents

## 9. Roles, Boundaries, and Delegation

Use agents as specialists: researcher, coder, tester, reviewer. Do not let a single agent hold the whole system in its head. One agent per role keeps ownership crisp.

- Assign clear tasks to each role.
- Require handoff notes and assumptions.
- Rotate responsibility to reduce blind spots.

## 10. Context Hygiene and Retrieval

Agent quality is limited by the relevance and cleanliness of the context it sees. More context is not always better. Context bloat is a quiet failure mode.

- Use retrieval to fetch only what is needed.
- Remove outdated or conflicting files from the prompt context.
- Prefer precise, small snippets over giant dumps.

## 11. Determinism, Reproducibility, and Seeds

If you cannot reproduce a result, you cannot debug it. Default to determinism when it exists.

- Record model versions, prompts, and tool inputs.
- Log agent steps in a trace that a human can inspect.

## 12. Prompt Assets as Code

Prompts are part of the system. Treat them like source code. Prompts should have owners and diffs.

- Version prompts with the repository.
- Test prompts against known cases.
- Refactor prompts when they become brittle.

---

# Part 4. Quality and Reliability

## 13. Tests as the Primary Oracle

In AI-assisted development, tests are the ground truth. If you trust only one artifact, trust tests.

- Add tests before or alongside code.
- Prefer high-signal tests that fail for real issues.
- Keep tests fast enough to run on every change.

Vibe-coded prototypes are fine for exploration, but redesign and verify them before shipping.

## 14. Property-Based Testing and Fuzzing

Agents are great at generating examples. Use that strength to test for edge cases that humans miss.

- Define properties that should always hold.
- Run fuzzing in CI and on new modules.
- Minimize failing inputs to make triage easy.

## 15. Observability by Default

Systems need to tell you when they are wrong. If you cannot see it, you cannot run it.

- Add structured logs, metrics, and traces from the start.
- Monitor outputs and model drift.

## 16. Failure Containment and Graceful Degradation

When a system fails, it should fail softly.

- Isolate risky components behind gates.
- Use circuit breakers and retries with limits.
- Provide a safe fallback for core user flows.

A fast fallback beats a slow perfect.

---

# Part 5. Architecture and Systems

## 17. Boundaries, Not Monoliths or Microservices

The structure is less about size and more about isolation of change. Boundaries are your entropy brakes.

- Define boundaries around data ownership and behavior.
- Keep dependencies flowing in one direction.
- Make boundaries explicit in code and documentation.

## 18. Event-Driven and Async by Design

AI features often run long or batch heavy. Design for async workflows. Separate user response time from processing time.

- Use queues and events for long-running tasks.
- Provide user feedback for background work.

Async buys user time and system stability.

## 19. Performance, Cost, and Token Budgets

AI compute is expensive. Design with budgets. Budget tokens like dollars.

- Set latency targets and token limits per request.
- Cache results where correctness allows.
- Measure cost as a first-class metric.

## 20. Evolve the Build and Release Pipeline

Automation is the delivery engine for AI-assisted teams. Release is a muscle, not a ceremony. Keep rollback paths tested and ready.

- Ship agent-generated changes behind feature flags.
- Validate with staging and canary releases.

---

# Part 6. Security, Safety, and Compliance

## 21. Threat Models for Agents

Agents introduce new attack surfaces. Assume prompts are hostile and inputs are tainted.

- Model prompt injection and data exfiltration.
- Restrict tool access to the minimum needed.
- Treat model outputs as untrusted input.

## 22. Secrets, Permissions, and Isolation

Access is power. Manage it tightly. Do not paste keys into prompts. Ever.

- Do not place secrets in prompt context.
- Use short-lived credentials with clear scopes.
- Sandbox tools and limit filesystem access.

## 23. Supply Chain and Provenance

Know what you ship and where it came from. If you cannot trace it, you cannot trust it.

- Track dependencies and licenses.
- Record model versions and dataset sources.
- Require provenance for generated artifacts.

## 24. Human-in-the-Loop as a Safety Valve

The human is still the last responsible party. Human review is a gate, not a rubber stamp.

- Define checkpoints that require human review.
- Escalate ambiguous or high-risk decisions.
- Keep humans in the loop for security and ethics.

---

# Part 7. Teams and Delivery

## 25. AI Pair Programming Rituals

Use AI like a strong pair, not a replacement. Pairing is a rhythm, not a tool. Keep a visible task list and update it.

- Start with a joint plan.
- Alternate between agent and human review.

## 26. Review as Collaboration, Not Judgment

Review is where quality becomes reality. It is also where you teach the system.

- Focus on risks, not style.
- Require evidence: tests, logs, or traces.
- Encourage small, reviewable changes.

## 27. Documentation That Stays True

Docs rot when they are not tied to code. If the docs do not change in the diff, assume they are wrong.

- Generate docs from code or tests when possible.
- Keep runbooks simple and actionable.

## 28. Operate What You Build

The feedback loop continues in production. Ops is where opinions meet reality.

- On-call reveals architecture.
- Track incidents and fix root causes.
- Close the loop with post-incident changes.

---

# Part 8. Craft and Future-Proofing

## 29. The Modern Knowledge Portfolio

Your value is in judgment, not keystrokes. Depth beats breadth when the pager goes off.

- Learn system design, testing, and data.
- Practice clear written communication.
- Keep a record of experiments and outcomes.

## 30. Build Internal Platforms, Not One-Offs

Scale the team by scaling the platform. Templates scale judgment.

- Invest in shared tooling and templates.
- Standardize patterns for prompts and agents.
- Reduce cognitive load across projects.

## 31. Ethics and Product Impact

AI features change user behavior. Respect that. If the metric is easy, the impact might not be.

- Evaluate harms and bias before shipping.
- Provide transparency when AI is involved.
- Measure long-term user outcomes, not just clicks.

## 32. Stay Adaptable

Model capabilities will change quickly. Treat model upgrades as migrations.

- Separate business logic from model logic.
- Use feature flags for model upgrades.
- Plan rollbacks for model changes.

---

# Appendix A. Plan Template

Use this as a lightweight execution plan alongside the PRD or spec.

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

# Appendix B. Spec Template

Canonical template: use the Appendix B template below as the versioned source of truth.

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

# Appendix C. Agent Handoff Template

Canonical template: `loopr/state/handoff.md` (created by `loopr run --codex`).

```
# Loopr Handoff

Initialized: <timestamp>

## Entry (append per step)
- Step:
- Decisions:
- Open questions:
- Tests run:
- Artifacts produced:
- Seed / replay (if PBT):
- Notes:
```

# Appendix D. Review Checklist

- Does the change meet the spec?
- Are tests added or updated?
- Are property-based tests (if any) deterministic and reproducible (seed logged)?
- Are failure modes understood?
- Are costs and performance acceptable?
- Are security and privacy covered?
- Are docs updated?

# Appendix E. Incident Checklist

- Stabilize the system and stop the bleed.
- Capture timeline and evidence.
- Identify root cause and contributing factors.
- Fix the root cause and add guards.
- Document the incident and share learnings.

# Appendix F. Glossary

- **Agent:** A model-driven process that can take actions using tools.
- **Context:** The information supplied to a model for a task.
- **Prompt asset:** A versioned, testable instruction set for a model.
- **Provenance:** The origin and lineage of code, data, or artifacts.
- **Determinism:** The ability to reproduce the same result from the same inputs.
- **Property-based testing:** Testing that checks invariants across generated inputs.
- **Invariant:** A property that must hold for all valid inputs.
- **Seed / Replay:** A value used to reproduce randomized test failures.
- **Loopr workspace:** The repo root containing `loopr/`.
- **Transcript:** A local log/JSONL record of an agent run.

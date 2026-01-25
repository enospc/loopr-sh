---
order: 1
depends_on: []
---

# Feature: Build and verification tooling

## Summary
Provide deterministic Makefile entry points for building, formatting, and vetting the Loopr CLI.

## Goals
- Ensure a stable build command that produces `bin/loopr`.
- Provide explicit formatting and vetting entry points.

## Non-goals
- Adding CI pipelines or release automation.
- Introducing new build systems beyond Make + Go toolchain.

## User Stories
- As a developer, I want a single build command so I can produce `bin/loopr` deterministically.
- As a developer, I want standard fmt/vet commands so I can validate changes consistently.

## Scope
- In scope:
  - `make build` builds the CLI and outputs `bin/loopr`.
  - `make fmt` runs Go formatting across the repo.
  - `make vet` runs Go vet across the repo.
- Out of scope:
  - Packaging, installation, or deployment workflows.

## Requirements
- Define `make build` as the deterministic build entry point.
- Define `make fmt` and `make vet` as validation entry points.
- Keep Make targets stable and documented in the repo.

## Acceptance Criteria
- `make build` produces `bin/loopr` on a supported system.
- `make fmt` formats Go sources without errors.
- `make vet` completes without errors.

## UX / Flow
- `make build` → build binary at `bin/loopr`.
- `make fmt` → format code.
- `make vet` → vet code.

## Data / API Impact
- None.

## Dependencies
- None.

## Risks & Mitigations
- Risk: Go toolchain version drift → Mitigation: document required version in README/Makefile.

## Open Questions
- Should `make test` be formalized as part of the tooling surface?

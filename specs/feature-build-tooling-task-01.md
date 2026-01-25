# Task: Build and verification tooling / Makefile build, fmt, and vet targets

## Task ID
01

## Summary
Define deterministic Makefile entry points for build, formatting, and vetting.

## Goal
Provide a stable build command and standard validation commands for the Loopr CLI.

## Scope
- In scope:
  - `make build` builds the CLI and outputs `bin/loopr`.
  - `make fmt` runs Go formatting across the repo.
  - `make vet` runs Go vet across the repo.
  - Update documentation if required to keep build targets discoverable.
- Out of scope:
  - CI/CD or release automation.
  - Packaging or distribution tooling.

## Acceptance Criteria
- `make build` produces `bin/loopr` on a supported system.
- `make fmt` formats Go sources without errors.
- `make vet` completes without errors.
- Build tooling usage is documented in the repo.

## Implementation Plan
- Review the existing Makefile targets for build, fmt, and vet.
- Add or adjust targets to ensure deterministic behavior and consistent output paths.
- Add an automated check script and a `make ci` target to run build, fmt, and vet.
- Update README or docs if build target usage is not documented.

## Dependencies
- None.

## Risks
- Go toolchain version drift could affect reproducibility.

## Test Plan
- Integration: run `scripts/ci/build-tooling-check.sh` (runs `make build`, `make fmt`, and `make vet`).

## Notes
- Keep targets stable to avoid breaking developer scripts.

## Completion
- Status: Done
- Tests: `make fmt`, `make vet`, `make build`
- Notes: None.

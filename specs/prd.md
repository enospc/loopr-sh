Interview summary (2026-01-24)
- Seed prompt: "Study this repo and generate PRD with code understanding."
- Product surface: CLI tool
- Primary user: Developer using Codex + Loopr
- Primary goal: Reliability (repeatable PRD→Spec→Features→Tasks→Tests workflow)
- Success metric type: Reliability (clean doctor rate / low drift)
- Data sensitivity: Internal (local developer machine data)
- Timeline: No fixed date
- Rollout: Internal only
- Tech constraints: Use existing Go CLI stack + embedded skills + Codex integration
- Non-goal: Long-running services or remote orchestration beyond the local CLI
- Primary risk: Codex CLI dependency changes

# PRD: Loopr CLI (Skills + Doctor + Workflow Runner)

## Summary
Loopr is a small, safe CLI that installs embedded Loopr skills into the Codex skills directory, validates drift against the embedded source of truth, initializes repo metadata, and runs the Loopr workflow via Codex while capturing transcripts. It targets developers who want a reliable, repeatable workflow for generating PRDs/specs/features/tasks/tests and implementing them with Codex.

## Problem / Opportunity
Developers using the Loopr workflow need a consistent way to install and validate skills locally and to capture transcripts for traceability. Manual skill management is error-prone and leads to drift, broken workflows, or missing artifacts. A lightweight CLI that embeds the canonical skills and provides a "doctor" check reduces operational risk and keeps the workflow stable across runs.

## Goals
- Provide a deterministic, safe installer for embedded Loopr skills.
- Detect drift between installed skills and embedded source of truth.
- Offer a `run` command that orchestrates the workflow and records Codex transcripts and metadata to the Loopr workspace.
- Keep the CLI small, predictable, and easy to operate.

## Non-goals
- Building long-running services or remote orchestration.
- Adding telemetry or centralized analytics.
- Supporting non-Codex agents unless explicitly implemented.

## Users & Use Cases
- Developer installs Loopr skills into their Codex environment.
- Developer validates installed skills for drift before running the workflow.
- Developer runs the workflow via Loopr to capture transcripts and metadata.
- Developer lists or uninstalls skills for cleanup or troubleshooting.

## Scope
- CLI commands: init, run, install, doctor, list, uninstall, version.
- Embedded skills are the source of truth and shipped with the binary.
- Transcript logging to `specs/.loopr/transcripts/<repo-id>/`.

## Requirements (high level)
- Install embedded skills into `$CODEX_HOME/skills` or `~/.codex/skills`.
- Detect and report missing/drifted skills; list extra skills.
- Backup existing skills before overwrite/removal (unless forced).
- Support `--only` filters and `--agent` / `--all` targeting (codex supported today).
- Orchestrate the Loopr workflow with `loopr run --codex` (execute) or `loopr run --dry-run` (dryrun mode).
- Require `specs/.loopr/repo-id` for transcript logging (created by `loopr init`).
- Provide deterministic build metadata via ldflags (version/commit/date).

## Success Metrics
- High "doctor" clean rate (installed skills match embedded source).
- Low rate of install/uninstall failures in standard environments.
- Consistent transcript creation during codex runs.

## Assumptions
- Linux is the primary target environment.
- Codex CLI is installed and available on PATH.
- Repos using Loopr have `specs/.loopr/repo-id` initialized via `loopr init`.

## Constraints
- Must remain a small, safe Go CLI with embedded skill assets.
- Must avoid destructive behavior; backups by default.
- Must not depend on external services or network access.

## UX Notes / Flows
- `loopr init` initializes repo metadata and decision log scaffolding.
- `loopr install` plants skills with backup and optional filters.
- `loopr doctor` compares installed skills to embedded skills and highlights drift.
- `loopr list` summarizes skill status using doctor results.
- `loopr uninstall` removes skills with optional backup.
- `loopr run --dry-run` prints the workflow steps without running Codex.
- `loopr run --codex --seed "<prompt>"` runs the workflow and writes transcripts and metadata.

## Risks & Mitigations
- Codex CLI changes break Codex invocation → keep the wrapper minimal; document assumptions.
- Skill drift or local edits → doctor command surfaces drift; backups on install.
- Missing repo-id for transcripts → fail fast with clear error; require `loopr init`.

## Dependencies
- Go toolchain (1.25+) for building from source.
- Codex CLI available on PATH for `loopr run --codex`.
- Local filesystem permissions to write into Codex skills and specs/.loopr.

## Open Questions
- Should Loopr support additional agents beyond Codex?
- Should `loopr run --codex` fallback behavior be configurable (e.g., always use `script`)?
- How should versioning be managed for embedded skills vs CLI releases?

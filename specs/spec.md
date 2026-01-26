# Spec: Loopr CLI (Skills + Doctor + Workflow Runner)

## Summary
Define a small, safe Go CLI that embeds Loopr skills and provides commands to init repo metadata, orchestrate the Loopr workflow, install/validate/list/uninstall skills, and capture Codex transcripts and reproducibility metadata in the Loopr workspace while scaffolding decision logs.

## Goals
- Provide deterministic installation of embedded Loopr skills into the local Codex skills directory.
- Detect and report drift between installed skills and embedded source of truth.
- Orchestrate the Loopr workflow with clear step sequencing and context boundaries.
- Capture Codex transcripts and reproducibility metadata in the Loopr workspace.
- Scaffold decision logs under specs/decisions/ to make decisions explicit and reversible.
- Keep operations local, safe, and reversible.

## Non-goals
- Building long-running services or remote orchestration.
- Adding telemetry, analytics, or network services.
- Supporting non-Codex agents unless explicitly implemented.

## Users & Use Cases
- Developer installs Loopr skills into their Codex environment.
- Developer checks for drift before running Loopr workflow steps.
- Developer runs Codex via Loopr to capture a transcript for traceability.
- Developer uninstalls or lists skills for cleanup or troubleshooting.

## Functional Requirements
- FR-01: Provide CLI commands: `init`, `run`, `install`, `doctor`, `list`, `uninstall`, `version`.
- FR-02: Embed Loopr skills in the binary and treat them as source of truth for install/doctor.
- FR-03: Default skill filter targets skills with name prefix `loopr-` unless `--only` is provided.
- FR-04: `install` writes skills into the Codex skills root:
  - If `CODEX_HOME` is set, use `$CODEX_HOME/skills`; else use `~/.codex/skills`.
  - Backup existing skills that would change into `.backup/loopr-<timestamp>/`.
  - Skip unchanged files; write changed files atomically; preserve executable mode for scripts.
  - Support `--agent`, `--all`, `--only`, `--force`, `--verbose`.
- FR-05: `doctor` compares installed skills against embedded skills:
  - Report status per skill: `installed`, `missing`, or `drifted` (missing or hash mismatch).
  - Report extra installed skills not present in embedded list.
  - Support `--agent`, `--all`, `--only`, `--verbose`.
- FR-06: `list` prints skills and status based on `doctor` results and supports `--agent`, `--all`, `--only`.
- FR-07: `uninstall` removes installed skills:
  - By default, back up skills to `.backup/loopr-<timestamp>/` before removal.
  - Support `--force` to remove without backup and proceed if backup fails.
  - Support `--agent`, `--all`, `--only`, `--verbose`.
- FR-08: `run` orchestrates the Loopr workflow:
  - Requires `--codex` (execute) or `--dry-run` (dryrun mode).
  - Determine the step sequence (`prd` → `spec` → `features` → `tasks` → `tests` → `execute`) based on missing artifacts unless `--step` or `--from/--to` is provided.
  - Support `--force` to re-run steps even if outputs exist.
  - Support `--confirm` to request confirmation before each step.
  - Create or update `specs/.loopr/handoff.md` as the minimal context handoff file.
  - When `--codex` is set, run Codex for each step using a minimal prompt that lists allowed inputs/outputs (skip prompt append when Codex args include `--help`/`-h`/`--version` or a Codex subcommand).
  - When `--codex` is set, print per-step progress (start/skip/done) for long-running runs.
  - When `--dry-run` is set, print the workflow steps without running Codex.
- FR-09: `run --codex` wraps Codex execution and captures transcripts:
  - Resolve the repo root for transcripts:
    - If `--loopr-root <path>` is provided, use it and require `specs/.loopr/repo-id` under that root.
    - Otherwise, search upward for the nearest `specs/.loopr/repo-id` (created by `loopr init`).
  - Create `specs/.loopr/transcripts/<repo-id>/` if missing.
  - Write `session-<timestamp>.log` and `session-<timestamp>.jsonl`.
  - If `script` is available, use it to capture terminal session; otherwise tee stdout/stderr into the log file.
  - JSONL metadata must include `start` and `end` events with timestamp and exit code.
- FR-10: `version` prints the binary version plus optional commit/date when provided via ldflags.
- FR-11: `init` initializes Loopr metadata and scaffolding:
  - Support `--root`, `--specs-dir`, and `--allow-existing`.
  - Detect non-greenfield signals unless `--allow-existing` is set.
  - Ensure `specs/.loopr/` exists and write `specs/.loopr/init-state.json`.
  - Write `specs/.loopr/.gitignore` to ignore transcripts and session logs.
- Ensure a 6-character NanoID `specs/.loopr/repo-id` using alphabet `useandom26T198340PX75pxJACKVERYMINDBUSHWOLFGQZbfghjklqvwyzrict` (create if missing).
  - Ensure `specs/.loopr/transcripts/<repo-id>/` exists.
  - Ensure `specs/decisions/` and `specs/decisions/template.md` exist with the required headings.
  - Init state includes: `schema_version`, `specs_dir`, `allow_existing`, `loopr_version`, `loopr_commit`, `loopr_date`.
- FR-12: `run --codex` JSONL metadata must include reproducibility fields in the `start` event:
  - Required: `loopr_version`, `loopr_commit`, `loopr_date`, `repo_root`, `repo_id`, `cwd`, `cmd`, `skills_embedded_hash`.
  - Optional when available: `git_commit`, `git_dirty`, `skills_installed_hash`.

## Foundation / Tooling
- FD-01: Provide deterministic build entry point: `make build` produces `bin/loopr`.
- FD-02: Provide formatting and validation entry points: `make fmt` and `make vet`.

## Non-functional Requirements
- NFR-01: Operate without network access; all actions are local filesystem operations.
- NFR-02: Favor safety and reversibility: backups by default, atomic writes, fail-fast errors.
- NFR-03: Linux is the primary target environment; Go 1.25+ for builds.
- NFR-04: Keep CLI behavior deterministic and stable across runs.

## UX / Flow
- `loopr init` → initializes repo metadata and decision log scaffolding.
- `loopr run --dry-run` → prints the workflow steps without running Codex.
- `loopr run --codex --seed "<prompt>"` → runs the full pipeline via Codex with transcript logging.
- `loopr run --codex --loopr-root <path>` → targets a specific Loopr workspace.
- `loopr install` → installs/updates skills, prints summary counts and backup path.
- `loopr doctor` → prints per-skill status and optionally drift details.
- `loopr list` → prints skill names with status.
- `loopr uninstall` → removes skills and prints summary counts and backup path.
- `loopr version` → prints version, commit, and build date when available.
- `loopr init` (CLI) → ensures `specs/decisions/` exists and installs the decision log template.

## Data Model
- Embedded skills index: list of skills, each with file entries (path, content hash, mode).
- Transcript artifacts:
  - `session-<timestamp>.log` (raw transcript)
  - `session-<timestamp>.jsonl` (start/end metadata)
- Reproducibility metadata (JSONL `start` event):
  - loopr version metadata, repo identifiers, command info, and skills hash snapshot.
- Decision log scaffolding:
  - `specs/decisions/` directory
  - `specs/decisions/template.md`

## API / Interfaces
- CLI flags:
  - Global: `--agent <name>`, `--all` (where supported)
  - Filters: `--only skill1,skill2`
  - Run: `--from`, `--to`, `--step`, `--seed`, `--force`, `--confirm`, `--codex`, `--loopr-root`, `--` for agent args
  - Run: `--dry-run` (dryrun mode; no Codex execution)
  - Safety: `--force` on install/uninstall
  - Output: `--verbose`
  - Init: `--root <path>`, `--specs-dir <dir>`, `--allow-existing`
- Environment variables:
  - `CODEX_HOME` to override the default Codex skills root.

## Architecture / Components
- `cmd/loopr`: CLI entry, command routing and flag parsing.
- `internal/agents`: agent resolution and skills root discovery.
- `internal/skills`: embedded skills index + hashing.
- `internal/ops`: file ops, install/doctor/list/uninstall, run orchestration, codex wrapper.
- `internal/version`: version metadata injected at build time.

## Error Handling
- Fail fast with non-zero exit codes on invalid flags or filesystem errors.
- Clearly report missing repo-id for transcript logging with remediation hint (run `loopr init`).
- On `--force`, proceed when backup fails; otherwise stop with error.

## Security & Privacy
- Do not transmit data over the network.
- Store transcripts locally under repo `specs/.loopr/`.
- Treat transcript contents as sensitive local data (developer-controlled).
- Treat optional prompt metadata as sensitive; only capture it when explicitly provided via env vars.

## Observability
- Logs: `session-*.log` transcript files.
- Metrics: none (explicit non-goal).
- Alerts: none.

## Rollout / Migration
- Internal-only distribution; no migration requirements.

## Risks & Mitigations
- Codex CLI changes break Codex invocation → keep wrapper minimal, rely on `codex` binary presence and arguments passthrough.
- Skill drift or local edits → `doctor` surfaces drift; backups on install/uninstall.
- Missing repo-id blocks transcript capture → explicit error with remediation guidance.

## Open Questions
- Should Loopr support agents beyond Codex?
- Should `loopr run --codex` allow explicit log paths or alternate transcript capture modes?
- How should embedded skill versioning be tracked across releases?

## Acceptance Criteria
- `loopr install` installs loopr-* skills into the Codex skills root, backing up modified skills.
- `loopr doctor` reports missing/drifted skills and extra skills accurately.
- `loopr list` prints skill names with status for the selected agent(s).
- `loopr uninstall` removes installed skills and backs them up unless `--force` is set.
- `loopr run --codex` creates transcript + JSONL metadata files under `specs/.loopr/transcripts/<repo-id>/`.
- `loopr version` prints version and includes commit/date when built with ldflags.
- Running `loopr init` results in `specs/decisions/template.md` with the required headings.
- `loopr run --codex` JSONL `start` events include the required reproducibility fields.

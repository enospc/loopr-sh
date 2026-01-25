# Spec: Loopr CLI (Skill Installer + Doctor + Codex Wrapper)

## Summary
Define a small, safe Go CLI that embeds Loopr skills and provides commands to install, validate, list, and uninstall those skills, plus a Codex wrapper that captures transcripts and reproducibility metadata in the Loopr workspace and scaffolds decision logs.

## Goals
- Provide deterministic installation of embedded Loopr skills into the local Codex skills directory.
- Detect and report drift between installed skills and embedded source of truth.
- Capture Codex transcripts and reproducibility metadata in the Loopr workspace.
- Scaffold decision logs under specs/decisions/ to make decisions explicit and reversible.
- Keep operations local, safe, and reversible.

## Non-goals
- Building or running the Loopr workflow itself (beyond a Codex wrapper).
- Adding telemetry, analytics, or network services.
- Supporting non-Codex agents unless explicitly implemented.

## Users & Use Cases
- Developer installs Loopr skills into their Codex environment.
- Developer checks for drift before running Loopr workflow steps.
- Developer runs Codex via Loopr to capture a transcript for traceability.
- Developer uninstalls or lists skills for cleanup or troubleshooting.

## Functional Requirements
- FR-01: Provide CLI commands: `install`, `doctor`, `list`, `uninstall`, `codex`, `version`.
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
- FR-08: `codex` wraps a Codex run and captures transcripts:
  - Resolve the repo root for transcripts:
    - If `--loopr-root <path>` is provided, use it and require `specs/.loopr/repo-id` under that root.
    - Else if `LOOPR_ROOT` is set, use it and require `specs/.loopr/repo-id` under that root.
    - Otherwise, search upward for the nearest `specs/.loopr/repo-id` (created by loopr-init).
  - Create `specs/.loopr/transcripts/<repo-id>/` if missing.
  - Write `session-<timestamp>.log` and `session-<timestamp>.jsonl`.
  - If `script` is available, use it to capture terminal session; otherwise tee stdout/stderr into the log file.
  - JSONL metadata must include `start` and `end` events with timestamp and exit code.
- FR-09: `version` prints the binary version plus optional commit/date when provided via ldflags.
- FR-10: Decision log scaffolding: the embedded Loopr skills must ensure `specs/decisions/` exists and include a `specs/decisions/template.md` file with the headings `Title`, `Date`, `Status`, `Context`, `Decision`, `Alternatives`, and `Consequences`.
- FR-11: `codex` JSONL metadata must include reproducibility fields in the `start` event:
  - Required: `loopr_version`, `loopr_commit`, `loopr_date`, `repo_root`, `repo_id`, `cwd`, `cmd`, `skills_embedded_hash`.
  - Optional when available: `git_commit`, `git_dirty`, `skills_installed_hash`, `codex_model`, `codex_prompt`.
  - `codex_model` and `codex_prompt` are populated from environment variables `LOOPR_CODEX_MODEL` and `LOOPR_CODEX_PROMPT` when set.

## Foundation / Tooling
- FD-01: Provide deterministic build entry point: `make build` produces `bin/loopr`.
- FD-02: Provide formatting and validation entry points: `make fmt` and `make vet`.

## Non-functional Requirements
- NFR-01: Operate without network access; all actions are local filesystem operations.
- NFR-02: Favor safety and reversibility: backups by default, atomic writes, fail-fast errors.
- NFR-03: Linux is the primary target environment; Go 1.25+ for builds.
- NFR-04: Keep CLI behavior deterministic and stable across runs.

## UX / Flow
- `loopr install` → installs/updates skills, prints summary counts and backup path.
- `loopr doctor` → prints per-skill status and optionally drift details.
- `loopr list` → prints skill names with status.
- `loopr uninstall` → removes skills and prints summary counts and backup path.
- `loopr codex -- <args>` → runs Codex and prints transcript/metadata paths.
- `loopr codex --loopr-root <path> -- <args>` → targets a specific Loopr workspace.
- `loopr version` → prints version, commit, and build date when available.
- `loopr-init` (skill) → ensures `specs/decisions/` exists and installs the decision log template.

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
  - Safety: `--force` on install/uninstall
  - Output: `--verbose`
  - Codex: `--loopr-root <path>`
- Environment variables:
  - `CODEX_HOME` to override the default Codex skills root.
  - `LOOPR_ROOT` to select a Loopr workspace for `loopr codex` (overridden by `--loopr-root`).
  - `LOOPR_CODEX_MODEL` and `LOOPR_CODEX_PROMPT` to populate optional reproducibility metadata fields.

## Architecture / Components
- `cmd/loopr`: CLI entry, command routing and flag parsing.
- `internal/agents`: agent resolution and skills root discovery.
- `internal/skills`: embedded skills index + hashing.
- `internal/ops`: file ops, install/doctor/list/uninstall, codex wrapper.
- `internal/version`: version metadata injected at build time.

## Error Handling
- Fail fast with non-zero exit codes on invalid flags or filesystem errors.
- Clearly report missing repo-id for transcript logging with remediation hint (run loopr-init).
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
- Codex CLI changes break wrapper → keep wrapper minimal, rely on `codex` binary presence and arguments passthrough.
- Skill drift or local edits → `doctor` surfaces drift; backups on install/uninstall.
- Missing repo-id blocks transcript capture → explicit error with remediation guidance.

## Open Questions
- Should Loopr support agents beyond Codex?
- Should `loopr codex` allow explicit log paths or alternate transcript capture modes?
- How should embedded skill versioning be tracked across releases?

## Acceptance Criteria
- `loopr install` installs loopr-* skills into the Codex skills root, backing up modified skills.
- `loopr doctor` reports missing/drifted skills and extra skills accurately.
- `loopr list` prints skill names with status for the selected agent(s).
- `loopr uninstall` removes installed skills and backs them up unless `--force` is set.
- `loopr codex` creates transcript + JSONL metadata files under `specs/.loopr/transcripts/<repo-id>/`.
- `loopr version` prints version and includes commit/date when built with ldflags.
- Running `loopr-init` results in `specs/decisions/template.md` with the required headings.
- `loopr codex` JSONL `start` events include the required reproducibility fields.

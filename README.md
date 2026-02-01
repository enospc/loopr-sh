# Loopr CLI

Loopr is a workflow harness for repositories. It orchestrates Codex with structured prompts, captures transcripts, and
keeps the PRD → Spec → Features → Tasks → Tests → Execute pipeline grounded in your repo.

Loopr is intentionally small and safe: it initializes repo metadata, drives a deterministic prompt sequence, logs decisions and outputs, and
validates specs order artifacts. You own intent, verification, and outcomes; Loopr keeps the loop tight.

## Requirements

- Unix-like host (Linux or macOS; Windows via WSL or Git Bash if using `just`)
- Codex CLI available on your PATH
- `just` available on your PATH
- If building from source: Rust (edition 2024)

## Install tooling

Just:

```
# macOS (Homebrew)
brew install just

# Debian/Ubuntu
sudo apt install just

# Fedora
sudo dnf install just

# Arch
sudo pacman -S just

# Omarchy (Arch-based, yay)
yay -S just

# Alpine
sudo apk add just

# Windows (Scoop)
scoop install just

# Windows (Chocolatey)
choco install just

# Rust (cargo)
cargo install just
```

Rust (rustup recommended):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Codex CLI:

```
# Install per OpenAI docs, then verify:
codex --help
```

## Build

From this directory:

```
just build
```

Binary will be at `bin/loopr`.

## Quick start

1) Initialize the repo:

```
./bin/loopr init
```

2) Run the workflow:

```
./bin/loopr run --codex --seed-prompt "<seed prompt>"
```

3) (Optional) run your own checks (tests, lint) once specs are written.

## End-to-end workflow (example)

Use this seed prompt to drive a full PRD -> Spec -> Features -> Tasks -> Tests -> Execute flow that validates an end-to-end system:

```
Build a specific web app: "OpsRunbook" — a personal runbook + incident notes manager.

Stack
- Frontend: Vite + TypeScript (SPA)
- Backend: Rust (axum) JSON API
- DB: SQLite3 (sqlx) with migrations

Goal
- Validate end-to-end delivery: schema/migrations -> REST API -> frontend UI -> persistence.
- Keep scope small but production-shaped (project structure, configs, error handling, a few tests).
```

Run:

```
./bin/loopr run --codex --seed-prompt "<paste seed prompt>"
./bin/loopr run --codex --seed-prompt @seed-prompt.txt
```

## Working example seed prompt (SSE + realtime)

This prompt showcases Loopr's full harness with a realtime UI, SSE streaming, and periodic sampling:

```
Build a production-shaped internal tool: "WebTop Live" -- a web-based Linux top with real-time updates via SSE.

Goal
- Provide a live, web UI for system/process stats with filtering and history, updated in real time.

Scope
- Single host, single tenant.
- Capture periodic snapshots of CPU/mem/process stats and stream updates to the UI via SSE.
- Allow filtering by user, CPU%, memory%, and command.

Stack
- Frontend + Backend: Next.js (App Router) with Server Actions + SSE route
- DB: SQLite (Prisma migrations)
- Data collection: read from /proc (no external agents)
- Testing: API route tests + frontend component tests

Core features
- Snapshot stream: SSE endpoint that publishes new samples every N seconds.
- Process table: pid, user, cpu%, mem%, command (updates live).
- Filters: user, min CPU%, min mem%, command search.
- History view: compare last N snapshots.
- "Top offenders" view: most CPU/mem across snapshots.
- UI controls: pause/resume stream, and small CPU/mem sparklines.

Requirements
- Schema + migrations for snapshots and process entries.
- Input validation with zod; structured error envelope.
- Include API contract examples in the spec.
- Seed data generator for demo mode if /proc unavailable.
- Must run locally with no external services beyond SQLite.

Non-Goals
- Multi-host monitoring, auth, or WebSocket streaming (SSE only).

Success criteria
- Migrations apply cleanly.
- Snapshot capture works and is streamed via SSE.
- UI updates in real time and filters apply live.
- UI demo flow: start stream -> watch table update -> filter -> view top offenders.
```

## Guided walkthrough (seed prompt -> PRD -> Spec -> Features -> Tasks -> Tests -> Execute)

This walkthrough shows the full Loopr harness from a seed prompt to per-task execution.
It assumes you are in your repo root and want Loopr artifacts under `loopr/` and `specs/`.

### 1) Initialize Loopr metadata

```
./bin/loopr init
```

This creates:
- `loopr/repo-id`
- `loopr/state/` for handoffs, transcripts, and status
- `loopr/.gitignore` to keep runtime state local
 - `loopr/state/docs-index.txt` (compressed docs index)
 - `AGENTS.md` (created or injected by default)

### 2) Save your seed prompt

Save a prompt (like the “WebTop Live” example above) to a file:

```
cat > seed-prompt.txt <<'EOF'
<paste seed prompt here>
EOF
```

### 3) Run the PRD step (seed prompt required)

```
./bin/loopr run --codex --step prd --seed-prompt @seed-prompt.txt
```

Output:
- `specs/prd.md`
- A completion note in `loopr/state/handoff.md`

### 4) Generate the spec

```
./bin/loopr run --codex --step spec
```

Output:
- `specs/spec.md`

### 5) Generate features, tasks, and tests

```
./bin/loopr run --codex --step features
./bin/loopr run --codex --step tasks
./bin/loopr run --codex --step tests
```

Outputs:
- `specs/feature-order.yaml`
- `specs/task-order.yaml`
- `specs/test-order.yaml`
- `specs/feature-*.md`
- `specs/feature-*-task-*.md`
- `specs/feature-*-task-*-test-*.md`

Tip: you can also run a contiguous range:
```
./bin/loopr run --codex --from spec --to tests
```

### 6) Execute the work

Single execute prompt per iteration:
```
./bin/loopr loop
```

Per-task mode (tests-first, PBT fail-first):
```
./bin/loopr loop --per-task
```

Per-task mode details:
- Reads `specs/task-order.yaml` and `specs/test-order.yaml`.
- Writes progress to `loopr/state/work-status.json`.
- Runs tests via `TEST_COMMAND` in `loopr/config` (default: `just test`).

### 7) Monitor progress and transcripts

Status files:
- `loopr/state/status.json` (overall loop status)
- `loopr/state/work-status.json` (per-task mode)
 - `loopr/state/docs-index.txt` (compressed docs map; regenerated on run/loop or via `loopr index`)

Transcripts:
```
loopr/state/transcripts/<repo-id>/session-*.log
loopr/state/transcripts/<repo-id>/session-*.jsonl
```

### 8) Resume or re-run specific steps

- Re-run a single step: `./bin/loopr run --codex --step tests`
- Run from a specific step onward: `./bin/loopr run --codex --from tasks`
- Use a different workspace root: `./bin/loopr run --codex --loopr-root <path> --seed-prompt @seed-prompt.txt`

Other useful runs:

```
# Preview steps without running Codex.
./bin/loopr run --dry-run

# Run only the PRD step (requires seed prompt).
./bin/loopr run --codex --step prd --seed-prompt "<paste seed prompt>"

# Run a bounded range (for example, spec -> tests).
./bin/loopr run --codex --from spec --to tests

# Run the execute loop (continues until exit condition or max iterations).
./bin/loopr loop --max-iterations 5 -- --model <model name>
./bin/loopr loop --per-task -- --model <model name>

# Point Loopr at a different workspace root.
./bin/loopr run --codex --loopr-root ./website --seed-prompt "<paste seed prompt>"
```

## AGENTS.md (recommended)

Keep agent instructions in `AGENTS.md` at the repo root. Treat it as a contract: goals, guardrails, and expectations
for Codex sessions. Loopr prompts assume this file is authoritative for how the agent should behave.
By default, `loopr init` creates `AGENTS.md` if missing or appends a Loopr-marked section if it already exists.
Use `loopr init --no-agents` to skip this behavior. The docs index is regenerated at the start of `loopr run`
and `loopr loop`, or manually via `loopr index`.

## Command summary

```
loopr init            # initialize Loopr metadata in a repo
loopr run             # orchestrate workflow (requires --codex or --dry-run)
loopr loop            # run the execute loop with safety gates
loopr index           # refresh the Loopr docs index (loopr/state/docs-index.txt)
loopr version         # show version info
```

Tip: `loopr run --help` shows Loopr-specific flags. If you include `--codex`, help/version flags are forwarded to Codex
(for example, `loopr run --codex --help`). To pass other Codex flags, place them after `--`
(for example, `loopr run --codex -- --model <model name>`). To open Codex without the Loopr prompt, add `--no-prompt`.
Note: `--codex` and `--dry-run` are mutually exclusive.

## Workflow steps and prompts

`loopr run` executes these steps in order. Use `--step <name>` for a single step, or `--from <name> --to <name>` for a range.
Step names are the CLI selectors; prompt names appear in transcripts and handoffs.
Loopr does not infer or skip steps based on repo contents.

| Step name | Prompt name     | Outputs |
| --- | --- | --- |
| `prd` | `loopr-prd` | `specs/prd.md` |
| `spec` | `loopr-specify` | `specs/spec.md` |
| `features` | `loopr-features` | `specs/feature-order.yaml`, `specs/feature-*.md` |
| `tasks` | `loopr-tasks` | `specs/task-order.yaml`, `specs/feature-*-task-*.md` |
| `tests` | `loopr-tests` | `specs/test-order.yaml`, `specs/feature-*-task-*-test-*.md` |
| `execute` | `loopr-execute` | `specs/implementation-progress.md` |

Notes:
- `--seed-prompt` is required when the `prd` step runs and `specs/prd.md` does not exist.
- `--seed-prompt` accepts inline text or `@path` to read from a file.
- Each prompt appends a completion note to `loopr/state/handoff.md` (decisions, open questions, tests).

## Repo layout

Loopr keeps durable artifacts under `specs/` and operational state under `loopr/`:

```
loopr/
  repo-id
  config
  .gitignore
  state/
    handoff.md
    transcripts/<repo-id>/session-*.log
    transcripts/<repo-id>/session-*.jsonl
    status.json
    work-status.json
specs/
  prd.md
  spec.md
  feature-order.yaml
  task-order.yaml
  test-order.yaml
  feature-*.md
  feature-*-task-*.md
  feature-*-task-*-test-*.md
  implementation-progress.md
```

`loopr/.gitignore` ignores `loopr/state/` by default so runtime state stays local.

## Loop mode (MVP)

`loopr loop` runs repeated execute iterations with safety gates (exit signals and missing-status limits).
Add `--per-task` to run one Codex session per task/test item (tests-first), tracking progress in
`loopr/state/work-status.json`. In per-task mode, tests are written and executed before implementation, and
PBT tests must fail on the first run. Per-task mode reads `specs/task-order.yaml` and `specs/test-order.yaml`
and uses `TEST_COMMAND` from `loopr/config` (default: `just test`).

Examples:
```
loopr loop
loopr loop --max-iterations 10
loopr loop --per-task --max-iterations 10
loopr loop --loopr-root /repo/apps/service-a -- --model <model name>
```

The loop relies on the `---LOOPR_STATUS---` block emitted by the `loopr-execute` step.
If the status block is missing, Loopr cannot confirm completion and will stop after `MAX_MISSING_STATUS` misses.
Per-task mode extends the status block with `ITEM_KEY`, `ITEM_TYPE` (task|test), and `PHASE` (tests|implement).
PBT detection uses `kind: pbt` in `specs/test-order.yaml` with a keyword fallback in the test spec
(property-based, PBT, proptest, quickcheck, fast-check).

Config is read from `loopr/config`:
```
CODEX_TIMEOUT_MINUTES=15
MAX_ITERATIONS=50
MAX_MISSING_STATUS=2
TEST_COMMAND=just test
```

Loop status is written to `loopr/state/status.json`. Per-task progress is tracked in
`loopr/state/work-status.json`.

## Monorepo usage (run --codex)

`loopr run` requires `--codex` (run Codex) or `--dry-run` (dryrun mode).
`loopr run --codex` needs a Loopr workspace root (the directory that contains `loopr/repo-id`).
In a monorepo, you can pick the workspace explicitly or let Loopr find the nearest one.

Resolution order:
1. `--loopr-root <path>` (explicit flag)
2. Nearest ancestor with `loopr/repo-id`

Note: Only `loopr run --codex` resolves a Loopr workspace. `loopr run --dry-run` is repo-agnostic and does not require `loopr/`.

Example:

```
loopr run --codex --step execute --loopr-root /repo/apps/service-a -- --help
```

## Property-based testing guidance

Loopr’s test-generation steps support property-based testing (PBT) when it is suitable. The intent is to make PBT explicit and deterministic:

- **specs/spec.md** includes a **Testing Strategy** section (language/test stack, PBT library, invariants, determinism/seed policy).
- **feature docs** include **Invariants / Properties** and **PBT Suitability** (Recommended/Optional/Not Suitable).
- **task docs** include **Testing Notes** (properties, generator notes, seed/replay guidance).
- **test specs** emit PBT templates only when a framework is named; otherwise they fall back to example-based tests and note the gap.
- **execution steps** require deterministic PBT runs and record seeds/minimal failing cases when applicable.

## Notes

- The CLI initializes Loopr metadata and orchestrates Codex runs; planning and coding happen through Codex.

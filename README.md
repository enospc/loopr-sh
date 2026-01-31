# Loopr CLI

Loopr is a workflow harness for repositories. It orchestrates Codex with structured prompts, captures transcripts, and
keeps the PRD → Spec → Features → Tasks → Tests → Execute pipeline grounded in your repo.

Loopr is intentionally small and safe: it initializes repo metadata, drives a deterministic prompt sequence, logs decisions and outputs, and
validates specs order artifacts. You own intent, verification, and outcomes; Loopr keeps the loop tight.

## Requirements

- Linux host (desktop, VM, Docker, or bare metal)
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

## AGENTS.md (recommended)

Keep agent instructions in `AGENTS.md` at the repo root. Treat it as a contract: goals, guardrails, and expectations
for Codex sessions. Loopr prompts assume this file is authoritative for how the agent should behave.

## Command summary

```
loopr init            # initialize Loopr metadata in a repo
loopr run             # orchestrate workflow (requires --codex or --dry-run)
loopr loop            # run the execute loop with safety gates
loopr version         # show version info
```

Tip: `loopr run --help` shows Loopr-specific flags. If you include `--codex`, help/version flags are forwarded to Codex
(for example, `loopr run --codex --help`). To pass other Codex flags, place them after `--`
(for example, `loopr run --codex -- --model o3`). To open Codex without the Loopr prompt, add `--no-prompt`.
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

Examples:
```
loopr loop
loopr loop --max-iterations 10
loopr loop --loopr-root /repo/apps/service-a -- --model o3
```

The loop relies on the `---LOOPR_STATUS---` block emitted by the `loopr-execute` step.
If the status block is missing, Loopr cannot confirm completion and will stop after `MAX_MISSING_STATUS` misses.

Config is read from `loopr/config`:
```
CODEX_TIMEOUT_MINUTES=15
MAX_ITERATIONS=50
MAX_MISSING_STATUS=2
```

Loop status is written to `loopr/state/status.json`.

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

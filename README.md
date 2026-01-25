# Loopr CLI

Loopr is a workflow installer for greenfield-first projects (existing repos require explicit `--allow-existing`). The `loopr` binary plants the
Loopr skills into your coding agent (Codex) so the agent can run the full
PRD → Spec → Features → Tasks → Tests → Implementation pipeline.

This tool is intentionally small and safe: it only installs skills and validates
that they match the embedded source. All planning and coding happens through your
agent (Codex) after the skills are installed.

## Requirements

- Linux host (desktop, VM, Docker, or bare metal)
- Codex CLI available on your PATH
- Optional for skill preflight scripts: Python 3 (`loopr-init`, `loopr-doctor`) and `pyyaml` (`loopr-doctor`)
- If building from source: Go 1.25+

## Build

From this directory:

```
make build
```

Binary will be at `bin/loopr`.

Note: `make build` checks for Go and exits with a helpful message if it isn't installed.

## Install the skills

```
./bin/loopr install
```

This installs the Loopr skills into your Codex skills directory:
- `$CODEX_HOME/skills` if `CODEX_HOME` is set
- `~/.codex/skills` otherwise

By default, Loopr backs up any modified skills before overwriting them.

## Validate the install

```
./bin/loopr doctor
```

This compares installed skills against the embedded source and reports drift.

## Command summary

```
loopr install   # plant skills
loopr doctor    # validate installed skills
loopr list      # list skills and status
loopr uninstall # remove skills (backed up by default)
loopr codex     # run Codex with transcript logging
loopr version   # show version info
```

## Monorepo usage (Codex wrapper)

`loopr codex` needs a Loopr workspace root (the directory that contains `specs/.loopr/repo-id`).
In a monorepo, you can pick the workspace explicitly or let Loopr find the nearest one.

Resolution order:
1. `--loopr-root <path>` (explicit flag)
2. `LOOPR_ROOT=<path>` (environment variable)
3. Nearest ancestor with `specs/.loopr/repo-id`

Note: Only `loopr codex` resolves a Loopr workspace. Other commands are repo-agnostic and only use `CODEX_HOME` to locate the skills directory.

Examples:

```
loopr codex --loopr-root /repo/apps/service-a -- --help
LOOPR_ROOT=/repo/apps/service-b loopr codex -- --help
```

## Codex skills installed

Loopr installs the following skills into your Codex skills directory. You invoke these inside Codex (they are not CLI subcommands):

Primary workflow:
- `loopr-init`: create repo metadata + transcript locations under `specs/.loopr/`.
- `loopr-prd`: MCQ interview -> `specs/prd.md`.
- `loopr-specify`: PRD -> `specs/spec.md`.
- `loopr-features`: Spec -> `specs/feature-*.md` + `specs/feature-order.yaml`.
- `loopr-tasks`: Features -> `specs/feature-*-task-*.md` + `specs/task-order.yaml`.
- `loopr-tests`: Tasks -> `specs/feature-*-task-*-test-*.md` + `specs/test-order.yaml`.
- `loopr-execute`: implement tasks in order and record progress.

Supporting/targeted skills:
- `loopr-help`: explain the Loopr workflow and decision tree.
- `loopr-runner`: orchestrate the full workflow end-to-end (skips completed steps).
- `loopr-run-task`: implement a single task end-to-end.
- `loopr-taskify`: split one feature into tasks (updates `specs/task-order.yaml`).
- `loopr-testify`: split one task into tests (updates `specs/test-order.yaml`).
- `loopr-doctor`: validate order YAMLs and referenced files.

Note: `loopr doctor` (CLI) validates installed skill drift; `loopr-doctor` (skill) validates `specs/*-order.yaml` and referenced artifacts.

## End-to-end walkthrough (seed prompt → working code)

This is a complete greenfield example for developers.

### Seed prompt

"Build a monorepo with two apps: (1) a local TODO CLI that stores tasks in SQLite and exports to CSV, and (2) a small website that documents the CLI and provides usage examples."

### 0) Create a clean monorepo

Start in a new empty repo with no application code:

```
mkdir todo && cd todo
mkdir -p cli website
```

### 1) Install Loopr skills

```
/path/to/loopr install
/path/to/loopr doctor
```

For transcript logging in a monorepo, run Codex through the wrapper and point it at the target workspace:

```
/path/to/loopr codex --loopr-root ./cli -- <codex args>
```


### 2) Run the workflow in Codex

Open Codex in the subproject you are working on and run the skills in order. Each step
creates concrete artifacts under `specs/` and the later steps implement code.

Use `loopr codex --loopr-root ./cli` (or `./website`) to capture transcripts into that
workspace’s `specs/.loopr/transcripts/<repo-id>/`.

Tip: If you want a guided walkthrough, run `loopr-help`. If you want a single orchestrated run, run `loopr-runner`.

1. **Initialize Loopr metadata**
   - Prompt: "Run loopr-init"
   - If the repo already has code, prompt: "Run loopr-init with --allow-existing"
   - Interaction: Autonomous (no questions expected)
   - Output: `specs/.loopr/` with repo id, init-state, and transcript path

2. **Create a PRD**
   - Prompt: "Run loopr-prd with seed prompt: <seed prompt above>"
   - Interaction: **User input required** (MCQ interview; answer each question)
   - Output: `specs/prd.md`

3. **Expand PRD → Spec**
   - Prompt: "Run loopr-specify"
   - Interaction: **User input required if prompted** (clarifying questions when PRD lacks detail)
   - Output: `specs/spec.md` (includes foundation requirements)

4. **Split Spec → Features**
   - Prompt: "Run loopr-features"
   - Interaction: Autonomous
   - Output: `specs/feature-*.md` + `specs/feature-order.yaml` (foundation first in greenfield mode)

5. **Generate Tasks**
   - Prompt: "Run loopr-tasks"
   - Interaction: Autonomous
   - Output: `specs/feature-*-task-*.md` + `specs/task-order.yaml`

6. **Generate Tests**
   - Prompt: "Run loopr-tests"
   - Interaction: Autonomous
   - Output: `specs/feature-*-task-*-test-*.md` + `specs/test-order.yaml`

7. **Implement**
   - Prompt: "Run loopr-execute"
   - Interaction: Mostly autonomous; **user input required** if the agent needs missing context (e.g., test command choice or failure resolution)
   - Output: working code, tests, and `specs/implementation-progress.md`

### 3) Verify the build

The foundation tasks define the build/test entry points. In most cases this will be
something like:

```
make test
make build
```

If the foundation chose a different runner (e.g., `go test ./...`, `npm test`),
follow the commands defined in the foundation task files.

### 4) Run the CLI (example)

Once tasks are complete, you should have a working binary with commands like:

```
./bin/todo add "buy milk"
./bin/todo list
./bin/todo done 1
./bin/todo export --csv ./todos.csv
```

(Exact command names may vary depending on what the spec/tasks defined.)

### 4) Document the CLI in the website app

Repeat the workflow in `website` with a seed prompt focused on documentation and examples
for the CLI. Use `loopr codex --loopr-root ./website` so transcripts and `specs/` artifacts
live under the website workspace.

## Updating or re-installing skills

Re-run install anytime you want to refresh skills:

```
/path/to/loopr install
```

If you have local edits, Loopr will back them up automatically before overwriting.

## Notes

- Loopr defaults to **greenfield**: it assumes a blank repo unless you explicitly run `loopr-init` with `--allow-existing`.
- The CLI installs skills only. Planning and coding happen through Codex.

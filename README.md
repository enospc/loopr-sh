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

"Build a simple local CLI that tracks personal TODOs, stores them in a local SQLite database, and exports to CSV."

### 0) Create a clean repo

Start in a new empty repo with no application code:

```
mkdir todo-cli && cd todo-cli
```

### 1) Install Loopr skills

```
/path/to/loopr install
/path/to/loopr doctor
```

For transcript logging, run Codex through the wrapper:

```
/path/to/loopr codex -- <codex args>
```


### 2) Run the workflow in Codex

Open Codex in this repo and run the skills in order. Each step creates concrete artifacts
under `specs/` and the later steps implement code.

Use `loopr codex` to capture transcripts into `specs/.loopr/transcripts/<repo-id>/`.

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

## Updating or re-installing skills

Re-run install anytime you want to refresh skills:

```
/path/to/loopr install
```

If you have local edits, Loopr will back them up automatically before overwriting.

## Notes

- Loopr defaults to **greenfield**: it assumes a blank repo unless you explicitly run `loopr-init` with `--allow-existing`.
- The CLI installs skills only. Planning and coding happen through Codex.

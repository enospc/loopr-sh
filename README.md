# Loopr CLI

Loopr is a workflow installer for greenfield projects. The `loopr` binary plants the
Loopr skills into your coding agent (Codex) so the agent can run the full
PRD → Spec → Features → Tasks → Tests → Implementation pipeline.

This tool is intentionally small and safe: it only installs skills and validates
that they match the embedded source. All planning and coding happens through your
agent (Codex) after the skills are installed.

## Requirements

- Linux host (desktop, VM, Docker, or bare metal)
- Codex CLI available on your PATH
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
loopr version   # show version info
```

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

### 2) Run the workflow in Codex

Open Codex in this repo and run the skills in order. Each step creates concrete artifacts
under `specs/` and the later steps implement code.

1. **Initialize Loopr metadata**
   - Prompt: "Run loopr-init"
   - Output: `specs/.loopr/` with repo id and transcript path

2. **Create a PRD**
   - Prompt: "Run loopr-prd with seed prompt: <seed prompt above>"
   - Output: `specs/prd.md`

3. **Expand PRD → Spec**
   - Prompt: "Run loopr-specify"
   - Output: `specs/spec.md` (includes foundation requirements)

4. **Split Spec → Features**
   - Prompt: "Run loopr-features"
   - Output: `specs/feature-*.md` + `specs/feature-order.yaml` (foundation first)

5. **Generate Tasks**
   - Prompt: "Run loopr-tasks"
   - Output: `specs/feature-*-task-*.md` + `specs/task-order.yaml`

6. **Generate Tests**
   - Prompt: "Run loopr-tests"
   - Output: `specs/feature-*-task-*-test-*.md` + `specs/test-order.yaml`

7. **Implement**
   - Prompt: "Run loopr-execute"
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

- Loopr is **greenfield-only**: it assumes a blank repo.
- The CLI installs skills only. Planning and coding happen through Codex.

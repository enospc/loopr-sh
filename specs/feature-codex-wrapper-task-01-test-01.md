# Test: Codex wrapper writes transcript and metadata

## Test ID
01

## Type
Integration

## Purpose
Verify `loopr codex` creates transcript and JSONL metadata files when repo-id is present.

## Preconditions
- Temp repo directory with `specs/.loopr/repo-id` initialized.
- A dummy `codex` executable available on PATH.

## Test Data
- Dummy `codex` script that prints a line and exits 0.
- Command: `go run ./cmd/loopr codex -- --help`

## Steps
1. Create a temp repo and run `loopr-init --allow-existing` to generate `repo-id`.
2. Create a dummy `codex` script in a temp bin directory and prepend it to PATH.
3. Run `go run ./cmd/loopr codex -- --help` from the repo root.
4. Inspect `specs/.loopr/transcripts/<repo-id>/` for new `session-*.log` and `session-*.jsonl` files.
5. Verify JSONL contains `start` and `end` events with an exit code.

## Expected Results
- Transcript log and metadata files are created.
- JSONL includes `start` and `end` events with timestamps and exit code 0.

## Automation Notes
- Use a temp directory and stub `codex` to avoid depending on the real Codex binary.

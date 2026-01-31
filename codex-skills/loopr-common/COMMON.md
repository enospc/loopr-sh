# Common Skill Rules

## Prerequisites
- Run `loopr init` (CLI) to ensure repo metadata and transcript logging are initialized.
- Ensure the repo is greenfield (empty) or already Loopr-managed; otherwise stop and clarify scope.

## Coordination
- Do not invoke other skills directly.
- If a required input artifact is missing, stop and ask to run the appropriate skill.

## Context Boundaries
- Read only the files listed under Inputs for the current skill.
- Do not scan the repo or load unrelated files.
- If required context is missing, stop and ask for it.

## Mode Handling
- If a skill requires `mode`, read `.loopr/init-state.json`.
- If `init-state.json` is missing, assume `mode=existing`.

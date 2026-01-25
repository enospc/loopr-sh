# Test: Scaffold and npm scripts exist

## Test ID
01

## Type
Manual

## Purpose
Verify the project scaffold and npm scripts are present.

## Preconditions
- Repo is at the website root.

## Test Data
- None.

## Steps
1. Open `package.json`.
2. Verify scripts include `dev`, `build`, `preview`, and `test`.
3. Verify directories for content, templates, assets, scripts, and output exist.

## Expected Results
- `package.json` includes required scripts.
- Required directories exist in the repo.

## Automation Notes
- Can be automated by checking filesystem and package.json scripts.

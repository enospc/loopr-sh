#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT_DIR"

command -v just >/dev/null 2>&1 || { echo "just is required to run CI checks. Install it and re-run."; exit 1; }

just ci

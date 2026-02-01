---
title: Install
description: Build and install Loopr from source.
---

<section class="doc-hero">
  <p class="eyebrow">Install</p>
  <h1>Install Loopr</h1>
  <p class="lead">Set up the Loopr CLI so you can build, verify, and iterate with confidence.</p>
</section>

<section class="doc-body">
  <h2>Requirements</h2>
  <ul>
    <li>Unix-like host (Linux or macOS; Windows via WSL or Git Bash if using <code>just</code>)</li>
    <li>Codex CLI on your PATH</li>
    <li><code>just</code> on your PATH</li>
    <li>Rust (edition 2024) to build the Loopr CLI</li>
  </ul>

  <h2>Build the CLI</h2>
  <pre><code>git clone https://github.com/enospc/loopr-sh
cd loopr-sh
just build</code></pre>

  <h2>Initialize metadata</h2>
  <p>From your repo root, initialize Loopr metadata and decision logs:</p>
  <pre><code>./bin/loopr init</code></pre>
  <p>This also writes <code>loopr/.gitignore</code> to keep <code>loopr/state/</code> local.</p>
  <p>By default it creates or injects <code>AGENTS.md</code> and writes the pipe-formatted <code>loopr/state/docs-index.txt</code>. Use <code>--no-agents</code> to skip AGENTS changes. The docs index refreshes at the start of <code>loopr run</code>/<code>loopr loop</code> or via <code>loopr index</code>.</p>

  <h2>Run the workflow</h2>
  <p>Run the workflow through Codex and log transcripts to the workspace you want to manage.</p>
  <pre><code>./bin/loopr run --codex --seed-prompt "&lt;seed prompt&gt;" --loopr-root ./website</code></pre>
  <p><code>--seed-prompt</code> accepts inline text or <code>@path</code> to read from a file.</p>
  <p>To use dryrun mode (show workflow steps without running Codex), use <code>./bin/loopr run --dry-run</code>.</p>
  <p>To pass Codex flags, add them after <code>--</code> (or use <code>--help</code>/<code>--version</code> with <code>--codex</code> to see Codex output).</p>

  <h2>Execute the loop</h2>
  <p>Run the execute loop to begin implementation.</p>
  <pre><code>./bin/loopr loop --loopr-root ./website</code></pre>
  <p>Use <code>--per-task</code> for tests-first execution (one Codex session per task/test item).</p>
  <pre><code>./bin/loopr loop --per-task --loopr-root ./website</code></pre>
  <p>Per-task mode runs tests via <code>TEST_COMMAND</code> in <code>loopr/config</code> (default: <code>just test</code>).</p>
</section>

---
title: Quickstart
description: Run the Loopr workflow end-to-end.
---

<section class="doc-hero">
  <p class="eyebrow">Quickstart</p>
  <h1>Run the Loopr workflow</h1>
  <p class="lead">Short loops, fast reality. Start with intent, end with verified code.</p>
</section>

<section class="doc-body">
  <h2>1) Start in a repo</h2>
  <pre><code>mkdir my-project && cd my-project</code></pre>

  <h2>2) Initialize Loopr metadata</h2>
  <p>From your repo root, run:</p>
  <pre><code>loopr init</code></pre>
  <p>This creates <code>AGENTS.md</code> (or injects a Loopr section) and writes the pipe-formatted <code>loopr/state/docs-index.txt</code>. Use <code>loopr init --no-agents</code> to skip AGENTS changes. The docs index refreshes at the start of <code>loopr run</code>/<code>loopr loop</code> or via <code>loopr index</code>.</p>

  <h2>3) Run the workflow</h2>
  <p>Provide a seed prompt and let Loopr drive Codex through PRD → Spec → Features → Tasks → Tests → Execute:</p>
  <pre><code>loopr run --codex --seed-prompt "&lt;seed prompt&gt;"</code></pre>
  <p><code>--seed-prompt</code> accepts inline text or <code>@path</code> to read from a file.</p>

  <h2>4) Run a single step (optional)</h2>
  <p>Use <code>--step</code> to target one step at a time:</p>
  <pre><code>loopr run --codex --step prd --seed-prompt "&lt;seed prompt&gt;"
loopr run --codex --step spec
loopr run --codex --step features
loopr run --codex --step tasks
loopr run --codex --step tests
loopr run --codex --step execute</code></pre>

  <h2>5) Execute the loop</h2>
  <p>Run the execute loop to start implementation:</p>
  <pre><code>loopr loop</code></pre>
  <p>For tests-first execution (one Codex session per task/test item), use:</p>
  <pre><code>loopr loop --per-task</code></pre>
  <p>Per-task mode writes progress to <code>loopr/state/work-status.json</code> and runs tests via <code>TEST_COMMAND</code> in <code>loopr/config</code> (default: <code>just test</code>).</p>

  <h2>6) Verify</h2>
  <p>Use the test command defined by the project (commonly <code>just test</code>, <code>npm test</code>, or <code>cargo test</code>).</p>

  <div class="callout">
    <strong>Verification is the workflow:</strong> If you cannot test it, you cannot trust it.
  </div>
</section>

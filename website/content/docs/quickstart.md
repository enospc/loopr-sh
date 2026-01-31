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

  <h2>3) Run the workflow</h2>
  <p>Provide a seed prompt and let Loopr drive Codex through PRD → Spec → Features → Tasks → Tests → Execute:</p>
  <pre><code>loopr run --codex --seed-prompt "&lt;seed prompt&gt;"</code></pre>

  <h2>4) Run a single step (optional)</h2>
  <p>Use <code>--step</code> to target one step at a time:</p>
  <pre><code>loopr run --codex --step prd --seed-prompt "&lt;seed prompt&gt;"
loopr run --codex --step spec
loopr run --codex --step features
loopr run --codex --step tasks
loopr run --codex --step tests
loopr run --codex --step execute</code></pre>

  <h2>5) Verify</h2>
  <p>Use the test command defined by the foundation tasks (commonly <code>npm test</code> or <code>cargo test</code>).</p>

  <div class="callout">
    <strong>Verification is the workflow:</strong> If you cannot test it, you cannot trust it.
  </div>
</section>

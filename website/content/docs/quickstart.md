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
  <h2>1) Start in a clean repo</h2>
  <pre><code>mkdir my-project && cd my-project</code></pre>

  <h2>2) Initialize Loopr metadata</h2>
  <p>From your repo root, run:</p>
  <pre><code>loopr init</code></pre>

  <h2>3) Create a PRD</h2>
  <p>Provide a seed prompt and answer the MCQ interview:</p>
  <pre><code>loopr-prd</code></pre>

  <h2>4) Expand to a spec</h2>
  <pre><code>loopr-specify</code></pre>

  <h2>5) Derive features, tasks, and tests</h2>
  <pre><code>loopr-features
loopr-tasks
loopr-tests</code></pre>

  <h2>6) Execute tasks</h2>
  <pre><code>loopr-execute</code></pre>

  <h2>7) Verify</h2>
  <p>Use the test command defined by the foundation tasks (commonly <code>npm test</code> or <code>go test ./...</code>).</p>

  <div class="callout">
    <strong>Verification is the workflow:</strong> If you cannot test it, you cannot trust it.
  </div>
</section>

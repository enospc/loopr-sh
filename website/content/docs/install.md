---
title: Install
description: Build and install Loopr from source.
---

<section class="doc-hero">
  <p class="eyebrow">Install</p>
  <h1>Install Loopr</h1>
  <p class="lead">Set up the workflow installer so you can build, verify, and iterate with confidence.</p>
</section>

<section class="doc-body">
  <h2>Requirements</h2>
  <ul>
    <li>Linux host (desktop, VM, Docker, or bare metal)</li>
    <li>Codex CLI on your PATH</li>
    <li>Go 1.25+ (to build the Loopr CLI)</li>
    <li>Python 3 (optional, for <code>loopr-doctor</code> preflight)</li>
  </ul>

  <h2>Build the CLI</h2>
  <pre><code>git clone https://github.com/enospc/loopr-sh
cd loopr-sh
make build</code></pre>

  <h2>Install the skills</h2>
  <pre><code>./bin/loopr install
./bin/loopr doctor</code></pre>

  <p>This installs Loopr skills into your Codex skills directory (usually <code>~/.codex/skills</code>).</p>

  <h2>Initialize metadata</h2>
  <p>From your repo root, initialize Loopr metadata and decision logs:</p>
  <pre><code>./bin/loopr init</code></pre>
  <p>This also writes <code>specs/.loopr/.gitignore</code> to keep transcripts local.</p>

  <h2>Run the workflow</h2>
  <p>Run the workflow through Codex and log transcripts to the workspace you want to manage.</p>
  <pre><code>./bin/loopr run --codex --seed "&lt;seed prompt&gt;" --loopr-root ./website</code></pre>
  <p>To pass Codex flags, add them after <code>--</code> (or use <code>--help</code>/<code>--version</code> with <code>--codex</code> to see Codex output).</p>

  <div class="callout">
    <strong>Greenfield note:</strong> Loopr expects a clean repo. If you must run it in an existing repo, use <code>loopr init --allow-existing</code> and ensure strong tests.
  </div>
</section>

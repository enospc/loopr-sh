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
    <li>Python 3 (optional, for <code>loopr-init</code> and <code>loopr-doctor</code> preflights)</li>
  </ul>

  <h2>Build the CLI</h2>
  <pre><code>git clone https://github.com/enospc/loopr-sh
cd loopr-sh
make build</code></pre>

  <h2>Install the skills</h2>
  <pre><code>./bin/loopr install
./bin/loopr doctor</code></pre>

  <p>This installs Loopr skills into your Codex skills directory (usually <code>~/.codex/skills</code>).</p>

  <h2>Use the Codex wrapper</h2>
  <p>For transcript logging, run Codex through the Loopr wrapper and point it at the workspace you want to manage.</p>
  <pre><code>./bin/loopr codex --loopr-root ./website -- --help</code></pre>

  <div class="callout">
    <strong>Greenfield note:</strong> Loopr expects a clean repo. If you must run it in an existing repo, use <code>--allow-existing</code> and ensure strong tests.
  </div>
</section>

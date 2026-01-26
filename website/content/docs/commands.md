---
title: Commands
description: Core Loopr CLI commands and Codex skills.
---

<section class="doc-hero">
  <p class="eyebrow">Commands</p>
  <h1>CLI + Codex skills</h1>
  <p class="lead">Treat prompts like code and keep interfaces stable. The CLI installs the skills that enforce that discipline.</p>
</section>

<section class="doc-body">
  <h2>CLI commands</h2>
  <ul>
    <li><code>loopr init</code> - initialize repo metadata in <code>specs/.loopr/</code> and write init-state + transcript ignore.</li>
    <li><code>loopr install</code> - installs Loopr skills into your Codex skills directory.</li>
    <li><code>loopr doctor</code> - validates installed skills against the embedded source.</li>
    <li><code>loopr list</code> - lists skills and status.</li>
    <li><code>loopr uninstall</code> - removes skills (backs up by default).</li>
    <li><code>loopr run --codex</code> - runs the workflow via Codex with transcript logging for the chosen workspace.</li>
    <li><code>loopr version</code> - prints version info.</li>
  </ul>

  <h2>Codex skills (installed by Loopr)</h2>
  <p>Run these inside Codex after installing:</p>
  <ul>
    <li><code>loopr-prd</code> - interview and write <code>specs/prd.md</code>.</li>
    <li><code>loopr-specify</code> - expand PRD into <code>specs/spec.md</code>.</li>
    <li><code>loopr-features</code> - split the spec into feature files.</li>
    <li><code>loopr-tasks</code> - generate task files for each feature.</li>
    <li><code>loopr-tests</code> - generate test files for each task.</li>
    <li><code>loopr-execute</code> - implement tasks in order.</li>
  </ul>

  <h3>Supporting skills</h3>
  <ul>
    <li><code>loopr-help</code> - guided workflow overview.</li>
    <li><code>loopr-run-task</code> - implement a single task.</li>
    <li><code>loopr-taskify</code> / <code>loopr-testify</code> - split a single feature/task into tasks/tests.</li>
    <li><code>loopr-doctor</code> - validates <code>specs/*-order.yaml</code> and referenced artifacts.</li>
  </ul>

  <div class="callout">
    <strong>Interfaces first:</strong> define inputs, outputs, and error boundaries before asking an agent to implement.
  </div>
</section>

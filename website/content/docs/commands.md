---
title: Commands
description: Core Loopr CLI commands and workflow prompts.
---

<section class="doc-hero">
  <p class="eyebrow">Commands</p>
  <h1>CLI commands</h1>
  <p class="lead">Treat prompts like code and keep interfaces stable. Loopr orchestrates Codex with structured prompts and logs transcripts.</p>
</section>

<section class="doc-body">
  <h2>CLI commands</h2>
  <ul>
    <li><code>loopr init</code> - initialize repo metadata in <code>loopr/</code> and runtime state in <code>loopr/state/</code>.</li>
    <li><code>loopr run --codex</code> - runs the workflow via Codex with transcript logging for the chosen workspace.</li>
    <li><code>loopr run --dry-run</code> - prints the workflow steps without running Codex.</li>
    <li><code>loopr loop</code> - runs the execute loop with safety gates (exit signals and missing-status limits).</li>
    <li><code>loopr version</code> - prints version info.</li>
  </ul>
  <p>
    Tip: use <code>loopr run --help</code> to see Loopr run flags. <code>loopr run</code> requires <code>--codex</code> or <code>--dry-run</code>.
    These flags are mutually exclusive.
    If you include <code>--codex</code>, help/version flags
    are forwarded to Codex (for example, <code>loopr run --codex --help</code> shows Codex help). To pass other Codex
    flags, place them after <code>--</code>.
  </p>

  <h2>Property-based testing guidance</h2>
  <p>Loopr supports property-based testing (PBT) when it is suitable, but it never guesses the framework. The workflow makes PBT explicit and reproducible:</p>
  <ul>
    <li><strong>specs/spec.md</strong> includes a Testing Strategy (stack, PBT library, invariants, determinism/seed policy).</li>
    <li><strong>feature docs</strong> include Invariants/Properties and PBT Suitability (Recommended/Optional/Not Suitable).</li>
    <li><strong>task docs</strong> include Testing Notes (properties, generators, seed/replay guidance).</li>
    <li><strong>test specs</strong> emit PBT templates only when a framework is named; otherwise they fall back to example-based tests and note the gap.</li>
    <li><strong>execution</strong> logs seeds and minimal failing cases for deterministic reproduction.</li>
  </ul>

  <h2>Workflow prompts (used by <code>loopr run</code>)</h2>
  <p>Loopr drives Codex through these prompt names:</p>
  <ul>
    <li><code>loopr-prd</code> - interview and write <code>specs/prd.md</code>.</li>
    <li><code>loopr-specify</code> - expand PRD into <code>specs/spec.md</code>.</li>
    <li><code>loopr-features</code> - split the spec into feature files.</li>
    <li><code>loopr-tasks</code> - generate task files for each feature.</li>
    <li><code>loopr-tests</code> - generate test files for each task.</li>
    <li><code>loopr-execute</code> - implement tasks in order.</li>
  </ul>

  <div class="callout">
    <strong>Interfaces first:</strong> define inputs, outputs, and error boundaries before asking an agent to implement.
  </div>
</section>

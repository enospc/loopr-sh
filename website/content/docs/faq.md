---
title: FAQ
description: Frequently asked questions about Loopr.
---

<section class="doc-hero">
  <p class="eyebrow">FAQ</p>
  <h1>Common questions</h1>
  <p class="lead">Short answers rooted in verification and responsibility.</p>
</section>

<section class="doc-body">
  <details>
    <summary>Is Loopr only for greenfield repos?</summary>
    <p>Yes by default. Loopr is designed for clean repos. Use <code>--allow-existing</code> only when you understand the risks and have strong tests.</p>
  </details>

  <details>
    <summary>What does Loopr install?</summary>
    <p>The CLI installs Codex skills that drive the PRD -> Spec -> Features -> Tasks -> Tests -> Execute pipeline.</p>
  </details>

  <details>
    <summary>How do I know the output is correct?</summary>
    <p>You don’t unless you can verify it. Loopr treats tests as the primary oracle and stops on failures.</p>
  </details>

  <details>
    <summary>How does Loopr reduce entropy?</summary>
    <p>It favors small, reversible changes and requires explicit non-goals to prevent scope drift.</p>
  </details>

  <details>
    <summary>Do I need internet access or cloud services?</summary>
    <p>No. Loopr runs locally and writes artifacts in your repo.</p>
  </details>

  <details>
    <summary>Does Loopr collect user data?</summary>
    <p>No. The workflow artifacts are local. This website uses Cloudflare Web Analytics for aggregate traffic.</p>
  </details>

  <details>
    <summary>What if tests fail during execution?</summary>
    <p>Loopr stops on failures. Fix the issue or adjust the plan before proceeding.</p>
  </details>

  <details>
    <summary>Can I run only part of the workflow?</summary>
    <p>Yes. You can run a single task with <code>loopr-run-task</code> or generate tests for one task with <code>loopr-testify</code>.</p>
  </details>

  <details>
    <summary>How do I see Codex help or pass Codex flags?</summary>
    <p>Use <code>loopr run --help</code> for Loopr run flags. If you include <code>--codex</code>, help/version flags are forwarded to Codex (for example, <code>loopr run --codex --help</code>).</p>
    <p>For other Codex flags, place them after <code>--</code>, like <code>loopr run --codex -- --model o3</code>.</p>
  </details>

  <details>
    <summary>How does Loopr handle property-based testing?</summary>
    <p>Loopr treats PBT as an explicit, optional strategy. It requires the chosen library and invariants to be recorded in <code>specs/spec.md</code>, and feature/task docs carry the properties, generators, and seed/replay guidance.</p>
    <p>Tests only emit PBT templates when a framework is named; otherwise they fall back to example-based tests and note the gap. Execution logs seeds and minimal failing cases to keep runs reproducible.</p>
  </details>

  <details>
    <summary>Where do the artifacts live?</summary>
    <p>All artifacts live under <code>specs/</code> in your repo.</p>
  </details>

  <details>
    <summary>Does Loopr work in monorepos?</summary>
    <p>Yes. Use <code>loopr run --codex --loopr-root &lt;path&gt;</code> to point at the workspace you want to manage (add <code>--seed</code> when bootstrapping a new repo).</p>
  </details>
</section>

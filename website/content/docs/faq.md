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
    <summary>Can I use Loopr in an existing repo?</summary>
    <p>Yes. Initialize Loopr metadata with <code>loopr init</code>, then run the workflow against your existing codebase and verify outputs with tests.</p>
  </details>

  <details>
    <summary>What does Loopr set up?</summary>
    <p>Loopr initializes <code>loopr/</code> (repo id, config, transcripts) and drives the PRD -> Spec -> Features -> Tasks -> Tests -> Execute pipeline via Codex prompts.</p>
  </details>

  <details>
    <summary>How do I know the output is correct?</summary>
    <p>You don’t unless you can verify it. Loopr treats tests as the primary oracle and stops on failures. In per-task mode (<code>loopr loop --per-task</code>), tests are executed before implementation.</p>
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
    <p>Loopr stops on failures. Fix the issue or adjust the plan before proceeding. In per-task mode, Loopr records progress and failures in <code>loopr/state/work-status.json</code>.</p>
  </details>

  <details>
    <summary>Can I run only part of the workflow?</summary>
    <p>Yes. Use <code>loopr run --step &lt;name&gt;</code> to run a single step or <code>--from</code>/<code>--to</code> for a range.</p>
  </details>

  <details>
    <summary>How do I see Codex help or pass Codex flags?</summary>
    <p><code>loopr run</code> requires <code>--codex</code> (execute) or <code>--dry-run</code> (dryrun mode).</p>
    <p><code>--codex</code> and <code>--dry-run</code> are mutually exclusive.</p>
    <p>Use <code>loopr run --help</code> for Loopr run flags. If you include <code>--codex</code>, help/version flags are forwarded to Codex (for example, <code>loopr run --codex --help</code>).</p>
    <p>For other Codex flags, place them after <code>--</code>, like <code>loopr run --codex -- --model &lt;model name&gt;</code>.</p>
  </details>

  <details>
    <summary>How does Loopr handle property-based testing?</summary>
    <p>Loopr treats PBT as an explicit, optional strategy. In per-task mode, PBT tests must fail on the first run before implementation begins.</p>
    <p>PBT is detected via <code>kind: pbt</code> in <code>specs/test-order.yaml</code> with a keyword fallback in the test spec (property-based, PBT, proptest, quickcheck, fast-check).</p>
  </details>

  <details>
    <summary>Where do the artifacts live?</summary>
    <p>Specs live under <code>specs/</code>. Operational state (repo id, transcripts, status) lives under <code>loopr/</code>. Per-task progress is recorded in <code>loopr/state/work-status.json</code>.</p>
  </details>

  <details>
    <summary>Does Loopr work in monorepos?</summary>
    <p>Yes. Use <code>loopr run --codex --loopr-root &lt;path&gt;</code> to point at the workspace you want to manage (add <code>--seed-prompt</code> when bootstrapping a new repo; <code>@path</code> reads from a file).</p>
  </details>
</section>

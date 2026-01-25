---
title: Workflow
description: The Loopr PRD -> Spec -> Features -> Tasks -> Tests -> Implementation workflow.
---

<section class="doc-hero">
  <p class="eyebrow">Workflow</p>
  <h1>PRD -> Spec -> Features -> Tasks -> Tests -> Execute</h1>
  <p class="lead">A workflow built for verification, reversibility, and short loops.</p>
</section>

<section class="doc-body">
  <h2>The sequence</h2>
  <ol>
    <li><strong>PRD</strong> - define the problem, goals, and constraints.</li>
    <li><strong>Spec</strong> - translate the PRD into testable requirements and non-goals.</li>
    <li><strong>Features</strong> - split the spec into orthogonal slices.</li>
    <li><strong>Tasks</strong> - break features into 0.5-2 day tasks.</li>
    <li><strong>Tests</strong> - define acceptance tests before implementation.</li>
    <li><strong>Execute</strong> - implement tasks in order and stop on failures.</li>
  </ol>

  <h2>Design for verification</h2>
  <ul>
    <li>Tests are the primary oracle for AI-assisted changes.</li>
    <li>Determinism and reproducibility make results debuggable.</li>
    <li>Decision logs keep architecture reversible.</li>
  </ul>

  <h2>Short loops, fast reality</h2>
  <ul>
    <li>Prototype in hours, not weeks.</li>
    <li>Keep experiments small enough to discard.</li>
    <li>Integrate continuously, not at the end.</li>
  </ul>

  <div class="callout">
    <strong>Outcome focus:</strong> Your name is on the incident, not the prompt. Own the results.
  </div>
</section>

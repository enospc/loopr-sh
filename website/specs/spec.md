# Spec: Loopr Marketing + Docs Website (loopr.sh)

## Summary
Build a public marketing + documentation site for Loopr on loopr.sh. The site must be fast, accessible, and content-first, with clear CTAs to install and use Loopr. Content must be generated from `Loopr.md` as the baseline source of truth and stay faithful to its gist. The global visual theme should be inspired by the Fly.io marketing site while retaining Loopr-specific branding and content. Provide light/dark themes with a global switcher (default to light), deploy on Cloudflare, and integrate Cloudflare Web Analytics.

## Goals
- Drive adoption of Loopr (visits -> installs/usage)
- Provide clear, accurate docs for installation and workflow
- Reinforce reliability and trust in the tool

## Non-goals
- Payment processing or billing
- User accounts/authentication
- Complex web app workflows
- Localization / multi-language support
- Copying Fly.io or Loopr.md verbatim

## Users & Use Cases
- Developers evaluating Loopr for new projects -> quick value props + install steps
- Admins/operators -> operational clarity, safety, and reliability signals
- Decision makers / engineering managers -> risk/ROI framing and trust signals

## Functional Requirements
- FR-01: Provide a marketing landing page with value proposition, key benefits, and primary CTA to install Loopr.
- FR-02: Provide a docs section covering install, quickstart, commands, workflow, and FAQs.
- FR-03: Include a dedicated narrative section on becoming a Codex power user and adopting AI-first software engineering.
- FR-04: Provide clear navigation between marketing and docs; include persistent CTAs to Install and View Docs.
- FR-05: Include a GitHub link and a clear path to the CLI repository.
- FR-06: Implement Cloudflare Web Analytics on all pages.
- FR-07: Track conversion proxies for CTA clicks, GitHub clicks, and docs pageviews (use URL-based proxies such as `/go/install` and `/go/github`).
- FR-08: Provide basic SEO metadata (title, description, Open Graph/Twitter tags) per page.
- FR-09: Provide a custom 404 page with navigation back to key sections.
- FR-10: Apply a Fly.io-inspired visual system (typography scale, spacing, section rhythm, CTAs, layout) consistently across marketing, docs, and Codex pages without copying Fly.io assets or content.
- FR-11: Provide light and dark themes with a global theme switcher; default to the light theme on first load.
- FR-12: Persist the user’s theme choice locally (no cookies), and apply it across all pages.
- FR-13: Use `Loopr.md` as the baseline doc to generate and refine marketing, docs, and Codex Power User content.

## Foundation / Tooling
- FD-01: Use a home-grown static site build approach (no large framework). Site content and templates live in-repo with a deterministic build script.
- FD-02: Use npm for dependency management and scripts.
- FD-03: Provide standard scripts: `npm run dev`, `npm run build`, `npm run preview`, `npm test`.
- FD-04: Build output must be a static directory suitable for Cloudflare hosting (e.g., `dist/`).
- FD-05: `npm test` must perform lightweight validation (required pages exist, CTA links present, internal links not broken).

## Non-functional Requirements
- NFR-01: Performance-first (static pages; minimal JS).
- NFR-02: Mobile-first responsive layout.
- NFR-03: Accessibility target: WCAG 2.1 AA for core flows.
- NFR-04: Privacy-first: no cookies or tracking beyond Cloudflare Web Analytics.
- NFR-05: Content must be easy to update by editing Markdown/text files.
- NFR-06: Visual design is inspired by Fly.io (bold headline-driven hero, crisp section stacking, strong CTA emphasis, trust/enterprise blocks, multi-column footer) but uses original Loopr copy, assets, and branding.
- NFR-07: Theme switching must be accessible (keyboard and screen reader) and avoid visible flash between themes.
- NFR-08: Content must remain aligned with the Loopr.md gist; avoid contradictions or drift from the guide’s principles.

## UX / Flow
- Top-level nav: Home, Docs, Codex Power User, FAQ, GitHub.
- Primary CTA: Install Loopr (links to docs/install via `/go/install`).
- Secondary CTA: View Docs.
- Global theme switcher in the header; default to light theme with explicit toggle for dark.
- Marketing home: bold two-line hero with primary CTA, stacked feature sections, trust/credibility block, enterprise/readiness block, closing CTA band.
- Docs and Codex pages reuse the same visual system (type scale, spacing, buttons, section rhythm) for a cohesive theme.

## Data Model
- Content stored as Markdown with minimal front matter (title, description, optional metadata).
- Theme preference stored in localStorage (`loopr-theme`), values: `light` or `dark`.
- Static assets stored under a single assets directory.

## API / Interfaces
- Cloudflare Web Analytics snippet injected in layout.
- No public APIs.

## Architecture / Components
- Static site generator/build script (home-grown).
- Shared layout templates (header, footer, nav).
- Shared design tokens (CSS variables) for typography scale, palette, spacing, buttons, and theme switching.
- Theme switcher UI and inline script to apply theme before paint.
- Content directories for marketing pages and docs.
- Build output directory for Cloudflare hosting.

## Error Handling
- Build fails on missing required content (e.g., missing required pages).
- Build reports broken internal links from validation script.

## Security & Privacy
- No user data collection; no cookies.
- LocalStorage used only for theme selection.
- Use safe default headers where possible via Cloudflare configuration (CSP, X-Content-Type-Options, Referrer-Policy).

## Observability
- Logs: build logs in CI/local.
- Metrics: Cloudflare Web Analytics pageviews + conversion proxy events.
- Alerts: none in v1 (optional Cloudflare alerts later).

## Rollout / Migration
- Single GA launch on loopr.sh.
- No migration of existing content beyond seeding from `README.md` and `Loopr.md`.

## Risks & Mitigations
- Adoption risk: mitigate with clear messaging, examples, and frictionless install docs.
- Content drift: mitigate by tying docs to `Loopr.md` and CLI behavior, and updating with releases.
- Theme regressions: mitigate with shared tokens and cross-page validation.

## Open Questions
- Final information architecture (exact pages/sections) for v1 beyond the current set.
- Whether to respect system color preference when no explicit user choice exists (current default: light).

## Acceptance Criteria
- AC-01: Site builds into a static output directory via `npm run build`.
- AC-02: `npm run dev` serves the site locally and supports content updates.
- AC-03: Marketing landing page includes value prop, benefits, trust/enterprise, and Install CTA.
- AC-04: Docs include install, quickstart, commands, workflow, and FAQ.
- AC-05: Codex power user / AI-first engineering narrative is present and linked from nav.
- AC-06: Cloudflare Web Analytics is integrated on all pages.
- AC-07: Conversion proxies for Install CTA, GitHub clicks, and docs pageviews are trackable.
- AC-08: Pages include SEO metadata and a custom 404 page.
- AC-09: `npm test` validates required pages/CTAs/links.
- AC-10: All pages share a cohesive Fly.io-inspired visual theme while using Loopr-specific content and assets.
- AC-11: Theme switcher is visible in the global header, defaults to light, and persists user choice across pages.
- AC-12: Marketing, docs, and Codex pages reflect Loopr.md themes and terminology.

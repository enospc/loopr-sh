# PRD: Loopr Marketing + Docs Website (loopr.sh)

## Summary
A public marketing and documentation website for Loopr on loopr.sh, targeting developers, admins/operators, and decision makers. The site should drive adoption, support retention and reliability perception, and provide clear docs for installation and workflow usage. Analytics will use Cloudflare Web Analytics.

## Problem / Opportunity
Loopr needs a clear public presence that explains what it does, why it matters, and how to use it. The current lack of a focused marketing + docs site creates friction for evaluation, onboarding, and long‑term retention.

## Goals
- Drive adoption of Loopr (visits → installs/usage)
- Provide clear, accurate docs for installation and workflow
- Reinforce reliability and trust in the tool

## Non-goals
- Payment processing or billing
- User accounts/authentication
- Complex web app workflows
- Localization / multi-language support

## Users & Use Cases
- Developers evaluating Loopr for new projects → need quick value props + install steps
- Admins/operators → need operational clarity and safety guarantees
- Decision makers / engineering managers → need risk/ROI framing and reliability signals

## Scope
- Marketing/landing pages (value prop, benefits, how it works)
- Documentation (install, commands, workflow, FAQs)
- Cloudflare Web Analytics integration
- Cloudflare deployment for loopr.sh

## Requirements (high level)
- Responsive, fast, accessible public website
- Clear IA separating marketing vs docs
- Documentation content that matches current CLI behavior
- Analytics for traffic and conversion signal proxies

## Success Metrics
- Adoption: increased installs/usage driven by site
- Conversion: visit → install/usage funnel proxy
- Performance: fast load times and stable uptime
- Quality: reduced confusion/support load from docs
- Reliability: uptime and error-free deployment signals

## Assumptions
- Content is public; no sensitive data collected
- Cloudflare is the deployment/analytics platform
- Existing repo stack is acceptable for the website

## Constraints
- Use existing stack in this repo
- Deploy on Cloudflare with loopr.sh domain
- Public data only; avoid tracking that increases compliance risk

## UX Notes / Flows
- Primary CTA: “Install Loopr” leading to docs/install section
- Secondary CTA: “View Docs” and “GitHub”
- Clear navigation between marketing and docs sections

## Risks & Mitigations
- Adoption risk: mitigate with clear messaging, examples, and frictionless install docs

## Dependencies
- Cloudflare DNS and hosting setup for loopr.sh
- Existing Loopr CLI documentation content

## Open Questions
- Which exact pages and doc sections are in v1?
- What is the preferred site stack within this repo?
- What conversion proxy should we track in Cloudflare Analytics?

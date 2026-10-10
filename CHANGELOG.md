# Changelog

Summaries of the history of the whole repository, newest first. Each subproject
keeps its own detailed changelog, and those are the record of what changed in
the code:

- [`project-portfolio-management-matcher-with-rust/CHANGELOG.md`](project-portfolio-management-matcher-with-rust/CHANGELOG.md)
- [`project-portfolio-management-service-with-rust/CHANGELOG.md`](project-portfolio-management-service-with-rust/CHANGELOG.md)
- [`project-portfolio-management-ui-with-svelte/CHANGELOG.md`](project-portfolio-management-ui-with-svelte/CHANGELOG.md)

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
It is pre-1.0 and no `vX.Y.Z` release has been cut from the root release
workflow yet. Dates are 2026.

## Unreleased

### 10 October

- **Critical path on a large plan is 2 to 3 times faster** (RG-16, in progress): a quadratic search removed; `?only=critical` added. See [`BENCHMARKS.md`](BENCHMARKS.md).
- **The plan's task list pages** (RG-15): `limit` and `offset` with the standard headers, counts over every
  task, and a 2,000-task ceiling when no paging is asked for. Found by the new load and soak test
  (`scripts/load-test.mjs`, RG-11), whose results are in [`BENCHMARKS.md`](BENCHMARKS.md).

- **Readiness for major medical and governmental organizations:** an honest
  assessment ([`governance/production-readiness.md`](governance/production-readiness.md):
  not ready for unsupervised production, ready for a controlled pilot); an idle
  session timeout, a database-protected audit trail, a retention purge task,
  encrypted backups and a strict mode for operator-configured outbound targets;
  an assurance checklist of what only the deployer can do.

- **Issues register** (IR-1, IR-2): issues under a plan and across plans, with
  age in calendar days and an overdue flag; an `/issues` page. Change requests
  are specified (FR-14a) and planned, not built.
- **Every page in Welsh** (HD-13): all page text in a locale catalogue, a Welsh
  draft for every text, tests that fail on literal English in a template and on
  any untranslated line on any route. The Welsh has not been reviewed by a
  professional translator.
- **Sign-in with Microsoft Entra ID** (HD-12): OpenID Connect with PKCE in the
  front end's server side, directly or through a Keycloak realm; tested against a
  mock provider and a real Keycloak.
- **Operations pack** (HD-10): deployment, backup and restore scripts and a
  restore drill in CI, stated recovery objectives, upgrade guide, runbook.
- **Information governance pack** (HD-11): data protection impact assessment,
  data-flow diagram, record of processing, retention schedule, accessibility and
  language statements, a DTAC checklist, and a personal-data inventory kept true
  to the schema by a test.
- **Public-repository files:** this changelog, `NEWS.md`, `INSTALL.md`,
  `CONTRIBUTING.md`, `GOVERNANCE.md`, `MAINTAINERS.md`, `SECURITY.md`,
  `AI_STATEMENT.md`, `COMPARISONS.md`, `BENCHMARKS.md`, `RFC.md`, `CITATION.cff`,
  `CODEOWNERS`.
- **Secure by default, root CI and releases** (HD-1 to HD-5, HD-8, HD-9): login
  enforced by default, security headers and a strict Content Security Policy,
  rate limiting, a licence choice with the texts in `LICENSE/`, builds from a
  fresh clone, continuous integration, a tag-driven release workflow, and
  documents generated from the code so they cannot drift.

### 7 October

- Locales are `<language>-<region>` directories only; two-letter prefixes are no
  longer locales. A chapter of the specification on locales.

### 6 October

- Import from Jira and Asana (CSV) and Microsoft Project (MSPDI XML).
- Controlled documents, multi-step approval processes, WBS and RBS views,
  hammock activities, cross-plan task links, onboarding and offboarding,
  paging for every list.
- Per-locale `sitemap.xml` and `robots.txt`; accessibility audit of every route.

### 5 October

- Cost rates, accrual, work contours and scheduling rules; the what-if sandbox
  for schedule edits; an announcement feed and a one-screen CEO dashboard;
  display preferences (simple view, dyslexia-friendly font).

### Earlier (from 18 July)

- The plan registry, explainable duplicate matching, tasks, risks, budgets and
  governance; critical-path scheduling, earned value, resource levelling and
  Monte Carlo schedule and cost risk; RACI; key measures and objectives;
  milestones; the backlog, plan and specification that these items are tracked
  in. See the subproject changelogs for the detail and the 0.1.0 to 0.3.0
  release notes.

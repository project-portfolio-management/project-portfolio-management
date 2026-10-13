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

### 13 October

- **Demonstration** (`operations/demo.md`): a compose profile `demo` starts an API with login off, fills it with a fictitious portfolio (the Northgate Transport Authority: ten plans, eight people, four organizations; `scripts/seed-demo.mjs`, safe to run twice) and a UI whose sign-in accepts only `example@example.com`. Sign-in lands on a guided tour (`/demo`) of 39 features, each linked; a banner on every page says the data is fictitious. Verified on a stack started from nothing: sign-in succeeds, and all 39 tour links open without an error page. The signed-in dashboard headline is now "My Home Page" (the header offers the Sign out button, and not Sign in, once signed in; it is a button because signing out is a POST). Not covered: email, anything needing token attributes, and the screens for thresholds, triage and the start-control override (still API-only). The tour's Welsh is an unreviewed draft.
- **UI container fix**: the container's entry point is now `container-server.mjs`. adapter-node 6 assumes https unless told, so over plain http every form post was refused as cross-site (403); the earlier "ORIGIN" note was wrong. Sign-in over http now works.
- **Financials by portfolio** (found by the demonstration): the API grouped a plan by its direct parent, so in a portfolio, programme, project tree every project landed in an unnamed group per programme and the UI crashed on duplicate keys. A plan now rolls up to its nearest portfolio ancestor (a pure function with tests), and the UI keys its groups safely.
- **Lily upgrade and two new pickers**: `@lilydesignsystem` headless 0.4.0, picker bar 0.3.0, locale, share, text-size and theme pickers 0.2.0, themes 0.4.0, and new link picker 0.1.1 and search picker 0.1.0. The picker bar now has a link picker (home icon: Home, Plans, Dashboard, Milestones, Proposals, Director packs, Start queue, Organizations, Display; current page marked) and a search picker that goes to a new `/search?<terms>` page listing matching plans. The 0.4 themes place each popup by CSS anchor positioning, so the header's old `top:100%` overrides (which threw popups to the bottom of the window) are removed; a Playwright test now opens every picker at desktop and phone width and checks the popup is under its own button, inside the window and not shifting the page. The Display button hides below 64rem (it is in the link picker) so the header stays one row on a tablet. English and Welsh (unreviewed draft). Not done: the search page shows plan names only.
- **Cutover go/no-go screen** (EX-6): a plan page `/plans/{id}/cutover` (linked from the plan workspace) drafts a decision from stop criteria, publishes and freezes them, opens the decision, marks each criterion met with evidence, takes co-signatures by role, says why go would be refused, and records go, no-go or delay for safety; a new version starts a new draft. English and Welsh (unreviewed draft); tested over stubbed answers. Whether a signed-in person's token carries the co-signing role is decided by the API.
- **Deployment moves screen** (EX-5): a plan page `/plans/{id}/deployment-moves` (linked from the plan workspace) lists requests to move a task's start with the knock-on and its labour cost, requests one, and lets the approver approve or refuse with a reason. English and Welsh (unreviewed draft; the knock-on sentence does not yet handle a count of one); tested over stubbed answers.
- **Funding ledger screen** (EX-12): `/ledger` lists funding allocations with planned and untraced money and traces one to its budget lines, gates, milestones met and benefits validated. Read-only: creating allocations, mapping cost codes and reconciling a finance export stay API-only. English and Welsh (unreviewed draft); tested over stubbed answers.
- **Milestone sign-off screen** (EX-4): `/milestone-signoffs` shows milestones that need partner sign-off with delivered and met apart, the gap, and who is outstanding, and records a sign-off or a withdrawal for a chosen organization. Choosing which organizations a milestone needs is still API-only. English and Welsh (unreviewed draft); tested over stubbed answers.
- **Key people screen** (EX-11): `/key-persons` lists single points of failure with their successor and readiness, records and removes them, and marks contingent workers; resource managers only (others see the API's 404 as a message). English and Welsh (unreviewed draft); tested over stubbed answers.
- **Organizations screen** (EX-3): `/organizations` lists the partner organizations with their kind and pack audience, adds one, and retires or restores one. English and Welsh (unreviewed draft); tested over stubbed answers, not a live service.
- **Email deliveries screen**: `/email-deliveries` lists each recipient's delivery (sent, will retry, failed, with attempts and the last error) and filters by status; administrators only (anyone else gets the API's 404, shown as a message). English and Welsh (unreviewed draft); tested over stubbed answers, not a live service.
- **Start queue screen** (EX-8): `/start-queue` lists the programmes refused a start in the order they asked, with each reason (limit full, or the constraint pool short in named months), and a Withdraw button per programme. In the nav, in English and Welsh (the Welsh is an unreviewed draft). Tested over stubbed API answers (list, withdraw, Welsh, axe in three locales); not run against a live service.

### 12 October

- **UI container** (HD-2 remainder): the UI now builds with `@sveltejs/adapter-node` (adapter-auto produced no runnable server), has a Dockerfile (Node 26, non-root, health check) and a compose profile `ui` on port 3000 (this entry first said `ORIGIN` from `UI_ORIGIN`; that was wrong, see 13 October). Verified locally with podman: the stack boots with the UI healthy, `/` redirects to `/en-001`, the sign-in page carries the CSP, and an anonymous call through the UI's proxy is 401; `svelte-check`, 678 unit tests and 449 Playwright tests (with axe) pass. The container CI job gained the matching steps, which have not run on GitHub yet. Not done: sessions stay in memory (one replica) unless `OIDC_SESSION_STORE=redis`.

### 10 October

- **Change requests** (IR-4, IR-5): drafted, submitted, reviewed and decided by one named approver who is not
  the requester, with a forecast from the what-if sandbox and a baseline taken by a person; a list page.
  **Break-glass access** (RG-10), **handling marks** (RG-7) and a **shared session store** (RG-12) also landed.

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

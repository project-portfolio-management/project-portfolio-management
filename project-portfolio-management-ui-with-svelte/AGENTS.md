# AGENTS.md — Portfolio Front-End

Operator UI for the [Portfolio Service](../project-portfolio-management-service-with-rust):
plan identity CRUD + matching + merge over **one recursive `/api/plans`
collection**, plus the project-management workspace (Kanban board,
governance, schedule) and cookie-session / SSO auth (BFF; the browser
holds no token).

> Read [`spec/index.md`](./spec/index.md) first — the living spec
> (§1–§18). Entity umbrella: [`../spec/index.md`](../spec/index.md).
>
> **Status: implemented (v0.1.0), verified 2026-10-06.** `pnpm run check`
> is **0 errors / 0 warnings**, `npx vitest run` passes (**529 tests**, 40
> files in `tests/unit/`), and the Playwright suite (14 files in
> `tests/e2e/`) runs against a production build. There are **64 page
> routes** under `src/routes`. Shipped: the unified identity surface
> (`/plans`, `/plans/new`, `/plans/[pid]`, `/plans/[pid]/edit`,
> `/plans/merge`), the Kanban board, governance and schedule views, the
> oversight / executive dashboards (listed in [README.md](README.md)), the
> scheduling canon (critical path, baselines, earned value, leveling,
> schedule and cost risk, what-if sandbox), work and cost-rate pages, WBS and
> RBS views, KPIs / OKRs / milestones, onboarding and handover, the CEO
> dashboard and announcements, cookie-session / SSO auth (BFF), and
> locale-prefixed URLs in **10 locales** (see **Layout shell**).
> **Not built:** the per-plan audit timeline, the recent-activity feed, a
> match-score breakdown visual, the detail page's child-plan roll-up, and the
> issues / timeline / goals sub-routes (the service defers those too). The
> backlog of record is the repo-root [`tasks.md`](../tasks.md) (every item
> ticked as of 2026-10-06; each entry lists what is "Not built"); research
> history is [`../plan.md`](../plan.md).

## What this is

A SvelteKit 2 / Svelte 5 (runes) **SPA**. It calls the portfolio service
REST API under `/api/plans/...`, whose request/response body for the
identity record **is** the `project_portfolio_management_matcher::Plan`
shape itself (the API DTO = the matcher type, persisted as JSONB). There
is **one recursive collection** — every record is a **plan**, all under
`/api/plans`, with a single controller shape; matching runs across the
whole collection (it is **not** gated by kind). A plan carries an
**optional** `kind` descriptive label (Portfolio / Project / Product /
Program / Practice / Process / Purpose / Pathway / Proposal) and an
optional `parent_ref` to another plan (any plan may contain any other,
so the tree is recursive). The operational sub-resources (goals, tasks,
issues) hang off a plan under `/api/plans/{pid}/…` and are **not** part
of the matching surface (except goal titles).

## Single source of truth

- The service's [`spec.md`](../project-portfolio-management-service-with-rust/spec/index.md)
  and [`agents/`](../agents/) describe the API contract. If a field
  changes in `Plan` (the matcher type) in the service, fix
  `src/lib/api/types.ts` here in the **same** change cycle — do not let
  the front-end drift.
- This project has its own [`spec.md`](spec/index.md) (§1–§18) for
  front-end-specific decisions: routes, components, design system,
  build, layout shell, i18n.

## Ground rules

1. **Runes only** (`$state` / `$derived` / `$effect` / `$props` /
   `$bindable`). No `export let`, no `$:`, events are callback props.
2. **SPA.** `+layout.ts` sets `ssr = false` / `prerender = false`.
3. **TypeScript strict** (`noUncheckedIndexedAccess`). No `any` without
   a justifying comment.
4. **SVAR DataGrid** for tabular data (the plans list, sub-resource
   tables). Native HTML for simple lists.
5. **Lily Headless** for accessibility primitives (focus trap,
   listbox, combobox, dialog) and the theme / locale selectors. Native
   HTML elsewhere.
6. **No global stores** for HTTP state — construct a
   `PlanRepository` (all paths under `/api/plans`) per page/component.
7. **No envelope.** The service is loco.rs and returns **raw JSON**;
   `src/lib/api/client.ts` is the lean wrapper (get/post/put/patch/
   delete).
8. **Drift accepted** (repo decision 2026-06-02): own copy of API
   client / types / form primitives; do not factor a shared package
   without explicit user approval.

## Layout shell (cross-cutting family rule)

- Global navigation is a **top navigation bar** (header) spanning the
  full viewport width. There MUST NOT be a left-hand sidebar / rail.
- The navigation toggle is a **hamburger menu placed LEFTMOST** in the
  top bar; narrow viewports collapse the nav behind it.
- The main content area is **full-width**.
- The nav is the `navItems` list in `src/routes/+layout.svelte` (Plans
  and the dashboard destinations); there is no collection switcher. `simple view`
  moves the advanced destinations behind a "More" link (`src/lib/preferences.ts`).
- The chrome area carries the **theme selector**
  (`lily-design-system-svelte-theme-picker` — full shared catalogue;
  selecting a theme restyles the whole site; persists to
  `localStorage["lily-theme"]`), the **locale selector**
  (`lily-design-system-svelte-locale-picker` — 10 locales, `LOCALES` in
  `src/lib/locales.ts`; selecting one switches the language; only `ar-001` is
  RTL; persists to `localStorage["mxi.case.locale"]`), and the session affordance (Sign
  in / Sign out; the browser holds no token — see **Auth** below).

## Layout (actual `src/` tree, portfolio-specific parts)

The oversight / executive dashboard route directories (`auditor/`,
`automations/`, `board/`, `calendar/`, `capacity/`, `compliance/`,
`dashboard/`, `engineering/`, `executive/`, `financials/`, `gantt/`,
`ideas/`, `lifecycle/`, `objectives/`, `prioritization/`, `proposals/`,
`regulator/`, `reports/`, `reviews/`, `risk/`, `scenarios/`,
`security/`, `technology/`) are listed in [README.md](README.md)'s
routes table, not repeated here.

```
src/
├── hooks.server.ts                BFF session handling (reads the httpOnly session cookie)
├── lib/
│   ├── config.ts                  API_BASE_URL → same-origin BFF proxy (/api/proxy); no browser-held bearer
│   ├── i18n.svelte.ts             string catalogues for the 10 locales in `locales.ts` (en-001 default, en fallback), formatted by `@sveltia/i18n`; exports `t()` and `localeHref()`; one file, not a directory
│   ├── locales.ts                 pure locale set (no two-letter aliases), RTL flag, `splitPath`; shared by hooks, store and tests
│   ├── *-view.ts, wbs.ts, rbs.ts  pure view modules (no runes, no `$app/*`): ceo-view, ev-view, cpm-view, whatif-view, work-view, onboarding-view, … — tested in `tests/unit/`
│   ├── glossary.ts                plain-language glosses for acronyms; rendered by `components/Term.svelte`
│   ├── api/
│   │   ├── client.ts              lean fetch wrapper (get/post/put/patch/delete + ApiError); credentials: 'include' (cookie) + CSRF header
│   │   ├── types.ts               Plan + PlanKind + PlanStatus + Goal + Task + Issue (+ IssueKind/IssueSeverity/IssueStatus) + Relationship + RelationKind + PlanIdentifier + IdentifierScheme + PlanRef + ScoredRef + MergeRequest + MergeResponse + MergeRecordRow. No MatchBreakdown / AuditEntry / PlanEvent / TimelineRow / BurndownPoint type — those capabilities are not built (see the Status note above).
│   │   ├── capabilities.ts        CapabilityClient (reviews, assignees, notifications, automations, scheduled actions, Smart Score, lifecycle)
│   │   ├── ppm.ts                 `PpmClient` — typed endpoints for the dashboards and the scheduling, cost, work, what-if, onboarding and WBS pages (the largest module)
│   │   ├── tba.ts                 `TbaClient`
│   │   └── plans.ts               PlanRepository (list + listPage + search + get + create + update + remove + checkDuplicates + merge + recentMerges; all paths under /api/plans). No audit() or recentEvents() method.
│   ├── server/                    BFF-only (never bundled to the browser): auth.ts (magic-link + session→PASETO exchange + currentUser(), T-28f), session.ts (cookie), config.ts (PROJECT_PORTFOLIO_MANAGEMENT_API_URL / AUTH_API_URL)
│   ├── nav.ts                     T-28f (repo `tasks.md` EV-1), pure, DB-free: viewAttr/orderNavForView/landingRouteForView — role-tailored nav ordering + landing route from a deployment-declared `view` ABAC attribute
│   └── components/                PlanForm.svelte, merge-validation.ts (pure guard). No MatchBreakdown / KanbanBoard / IssuesList / Timeline / Burndown / GoalsPanel / picker components — the board route uses @svar-ui/svelte-kanban directly.
└── routes/
    ├── +layout.svelte / +layout.ts / +layout.server.ts   top-bar nav (leftmost hamburger) + Plans destination + theme/locale selectors + session affordance + SPA toggle; +layout.server.ts also resolves `view`/`landingRoute` (T-28f)
    ├── signin/ · verify/          per-app magic-link sign-in (BFF server routes)
    ├── api/proxy/[...path]/+server.ts   BFF proxy → portfolio service (injects the PASETO bearer)
    ├── +page.svelte               landing (links to /plans)
    └── plans/
        ├── +page.svelte          list (SVAR DataGrid + client-side FilterBar over the loaded rows)
        ├── new/+page.svelte      create
        ├── [pid]/+page.svelte    detail + delete + check-duplicates (plain score/confidence, no breakdown visual)
        ├── [pid]/edit/+page.svelte    edit
        ├── [pid]/{board,schedule,governance}/+page.svelte
        └── merge/+page.svelte    merge a duplicate into a survivor + recent merge history
```

## API consumption

Every plan lives under `/api/plans` (one recursive collection).

Rows below are endpoints an actual route calls today. `PlanRepository`
also defines `search()` (`GET /api/plans/search?q=`) and `listPage()`'s
`?parent=` roll-up scope, but no route calls either — the list page's
search box is a client-side filter over the already-loaded rows instead.
There is no repository method, type, or route at all for a recent-activity
feed, a per-plan audit timeline, or a match-score breakdown visual (see
the Status note above).

| UI action | Endpoint |
|---|---|
| List | `GET /api/plans` |
| Create | `POST /api/plans` |
| Detail | `GET /api/plans/{pid}` |
| Edit | `PUT /api/plans/{pid}` |
| Delete | `DELETE /api/plans/{pid}` |
| Check duplicates | `POST /api/plans/check-duplicates` → `ScoredRef[]` (score + confidence; no breakdown) |
| Merge duplicate (`/plans/merge`) | `POST /api/plans/merge` (body `{main_pid, duplicate_pid, reason?}`) |
| Merge history (`/plans/merge`) | `GET /api/plans/merges/recent` |
| Schedule | `GET /api/plans/{pid}/schedule` |
| Tasks (board) | `GET / POST /api/plans/{pid}/tasks` · `PUT / PATCH /api/plans/{pid}/tasks/{tid}` (PATCH = status move) |
| Sprints + burndown (board) | `GET / POST /api/plans/{pid}/sprints` · `GET /api/plans/{pid}/burndown?sprint=` |
| Time-based analysis (`/plans/[pid]/flow`) | `GET /api/plans/{pid}/{time-analysis,constraints,aging-wip,flow,cumulative-flow}` · `GET /api/plans/{pid}/tasks/{tid}/{time-analysis,transitions}` · `GET /api/flow-classes` — all read-only; the transition log is written by the task create/move calls |
| Collaborative review (`/reviews`) | `POST / GET /api/reviews` · `/{pid}/respond` · `/{pid}/submit` · `GET /api/reviews/consensus` |
| Automations (`/automations`) | `POST / GET /api/automations` · `/{pid}/enable`·`/disable` · `GET /api/automations/runs` |
| Scheduled actions (`/automations`) | `POST / GET /api/scheduled-actions` · `POST /api/scheduled-actions/sweep` |
| Smart Score (`/prioritization`) | `GET /api/plans/{pid}/smart-score` · `GET /api/prioritization` |
| Lifecycle (`/lifecycle`) | `GET /api/lifecycle` · `GET /api/plans/{pid}/lifecycle` |

`src/lib/api/capabilities.ts` additionally defines a per-task assign
endpoint, an assignee-workload query, and a notifications inbox
(`GET /api/notifications`); none is called by a route yet — the CHANGELOG's
2026-07-22 entry already flags the notifications gap specifically. There
is no `/plans/[pid]/{issues,timeline,goals}` route or endpoint at all
(service spec §9.4 sub-resources not yet built on either side).

## Three-part change rule

A behavioural change is one PR with three parts:

1. **Spec edit** — `spec.md` §13 (Tasks) or the relevant numbered
   section.
2. **Code edit** — `src/`.
3. **Test edit** — `tests/unit/` (vitest) and/or `tests/e2e/`
   (Playwright).

## Front-end conventions (verified against the code, 2026-10-06)

- **Pure view modules first.** Logic a page needs (ordering, wording,
  roll-ups, guards) lives in `src/lib/*-view.ts`, `wbs.ts`, `rbs.ts`,
  `modules.ts`, `nav.ts` etc. as plain functions with no runes and no
  `$app/*` imports, and is unit-tested with vitest (`tests/unit/`). Pages
  only render. Do not recompute what the server owns (e.g. RAG, float).
- **API access.** Plan identity goes through `PlanRepository`
  (`src/lib/api/plans.ts`); everything else through `PpmClient`
  (`src/lib/api/ppm.ts`) or `CapabilityClient` / `TbaClient`, all on the lean
  `ApiClient`. Add a method there, not a raw `fetch` in a page.
- **i18n.** `@sveltia/i18n` behind `t(key)` in `src/lib/i18n.svelte.ts`.
  Every in-app `href` and `goto` goes through `localeHref("/plans")`, which
  prefixes the current locale (`/cy-001/plans`); the universal `reroute` hook
  (`src/hooks.ts`) strips the prefix before route matching, and the server
  redirects bare paths (a two-letter prefix like `/en/…` is not a locale and is not forwarded). To add a locale extend `LOCALES`,
  `LOCALE_LABELS` and the catalogues.
- **One primary action per screen.** At most one visible `button.primary`;
  repeated per-row buttons are never primary; a multi-step page marks the
  current step with `class:primary={…}`. Enforced by
  `tests/unit/primary-action.test.ts` and `tests/e2e/primary-action.spec.ts`.
- **Plain language.** An acronym never stands alone in a heading, label,
  button, link, table header or nav entry: follow it with plain words or wrap
  it in `<Term t="SPI" />` (`src/lib/components/Term.svelte`, glosses in
  `src/lib/glossary.ts`). Enforced by `tests/unit/plain-language.test.ts`.
- **CEO dashboard** (`/ceo`, `src/lib/ceo-view.ts`) must show every tile with
  no page scroll at **1080x810 and 810x1080** CSS px, with long content and
  with one failing tile (`tests/e2e/ceo.spec.ts`).
- **Accessibility.** `tests/e2e/a11y.spec.ts` runs axe over every page route,
  enumerated from `src/routes`, so a new page is audited automatically; the
  `mobile` project repeats the route list at 390x844. Status is never colour
  alone (a word and a shape).
- **Routes added since the MVP** (full table in [README.md](README.md)):
  `/ceo`, `/announcements`, `/kpis`, `/okrs`, `/milestones`, `/me`,
  `/display`, `/cost-rates`, `/resources`, `/resources/rbs`, `/tour`, and
  under `/plans/[pid]/`: `work`, `costs`, `what-if`, `onboarding`, `handover`,
  `wbs`, `critical-path`, `earned-value`, `raci`, `leveling`,
  `schedule-health`, `schedule-risk`, `cost-risk`, `joint-risk`, `flow`,
  `calendar`.
- **Plan workspace links** are listed in `PLAN_LINKS` (`src/lib/modules.ts`);
  a new per-plan page needs an entry there and a matching assertion in
  `tests/unit/modules.test.ts`.

## Commands

```bash
pnpm install
pnpm dev                       # http://localhost:5173 (needs the service for real data)
pnpm run check                 # svelte-kit sync + svelte-check (strict; 0/0 expected)
pnpm run build
npx vitest run                 # unit suite (same as `pnpm test`), 529 tests
pkill -f 'vite preview'        # free port 4173 if a stale preview is running
npx playwright test            # e2e: builds, then `vite preview` on :4173
npx playwright test --project=mobile   # just the 390x844 audit
```

The Playwright config (`playwright.config.ts`) builds and serves
`vite preview --port 4173 --strictPort`, with `reuseExistingServer` outside CI,
so a stale preview from an earlier run can serve an old build: kill it first.
Specs stub the backend per test with `page.route` (see `tests/e2e/fixtures.ts`),
so no Rust service is needed. `package.json` pins `engines.node` to `=26`.

## Auth — BFF + cookie session (no token in the browser)

**BFF model (current).** The browser holds no token: sign-in establishes
a server-side **cookie session** (`__Host-mxi_session`, httpOnly), the
browser talks only to this front-end's own SvelteKit server (BFF), and
the BFF exchanges the session for a short-lived **PASETO v4.public**
token and calls the portfolio service server-side. Mutating requests are
CSRF-protected; there is no `localStorage` and no `mxi_access_token`.
Service-side enforcement (`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`) is
on by default, so the BFF must send the operator bearer on every call.

**Security headers and CSP (HD-4).** `src/hooks.server.ts` adds the headers in
`src/lib/security-headers.ts` to every response (HSTS only over HTTPS), and
`kit.csp` in `vite.config.ts` sets a strict policy: `script-src 'self'` with
SvelteKit's nonces/hashes, no `'unsafe-inline'` for scripts or `<style>`
elements, and for style *attributes* a single hash (`unsafe-hashes` plus the
hash of SvelteKit's own `#svelte-announcer` style), never `'unsafe-inline'`. So
a page sets a computed style with Svelte's `style:left="{x}%"` directive (set
through the CSSOM, allowed), never a `style="…"` attribute (blocked). `tests/e2e/security-headers.spec.ts` fails on any CSP violation across
representative routes; a test helper that needs CSS must use a constructed
stylesheet (see `settle()` in `tests/e2e/fixtures.ts`), not an injected
`<style>` tag.

- **Sign in** — the top bar's **Sign in** leads to this app's own
  `signin/` route (not a redirect to a central authentication front-end);
  the magic-link flow completes at `verify/`, which establishes the
  server-side session and sets the httpOnly `__Host-mxi_session` cookie.
- **BFF** — `src/hooks.server.ts` reads the session cookie;
  `src/lib/server/auth.ts` performs the magic-link + session→PASETO
  exchange; `src/routes/api/proxy/[...path]/+server.ts` is the reverse
  proxy that injects the PASETO bearer and forwards to the portfolio
  service. There is no client-held session store
  (`$lib/auth.svelte.ts` does not exist).
- **`ApiClient`** — browser→BFF calls (`API_BASE_URL` = same-origin
  `/api/proxy`) send the cookie (`credentials: 'include'`); state-changing
  requests carry a **CSRF token** (`X-CSRF-Token`). Safe `GET`/`HEAD` are
  CSRF-exempt.
- **`currentUser()`** (T-28f, repo `tasks.md` EV-1) — `src/lib/server/auth.ts`
  exchanges the session for a bearer, then calls the auth service's
  `GET /api/auth/me`, which now returns `attrs` (the caller's own live
  ABAC subject attributes). `+layout.server.ts` reads this to derive
  `view`/`landingRoute` for role-tailored nav ordering — presentation
  only, never an authorization decision.

Source of truth:
`agents/share/authentication-sessions.md` (shared family guidance, not in this repository)
(RS256 JWT + JWKS and any cross-origin `#access_token` fragment handoff
are decommissioned).

Configure the BFF's upstream URLs with the server-side env vars
`PROJECT_PORTFOLIO_MANAGEMENT_API_URL` (portfolio service) and
`AUTH_API_URL` (authentication service) — see `src/lib/server/config.ts`;
both default to `http://localhost:5150` (see `.env.example`).

## What does NOT live here

- FHIR Plan UI — the service has no FHIR surface.
- Consent-management UI — out of scope for MVP.
- A general ledger or budgeting system — not a finance system (entity spec §1.3); the budget-variance, cost-rate and earned-value pages are read and decision views over the service's figures.
- Posts feed / threaded comments — not part of the v1 portfolio
  sub-resource set (tasks / goals / issues only); roadmap-only.
- Members / role panel — not part of the v1 sub-resource set;
  roadmap-only.
- A password / credential form — sign-on is passwordless magic-link
  (own `/signin` + `/verify` BFF routes calling the central
  authentication-service); this app never handles or stores a password,
  and the browser never holds a token.

## Sitemap

`/sitemap.xml` and `/robots.txt` are generated (`src/lib/sitemap.ts`, routes `src/routes/sitemap.xml`, `src/routes/robots.txt`). The page list comes from `import.meta.glob` over `+page.svelte`, so a new static page appears automatically, once per locale with `hreflang` alternates. Parameterized pages (`[pid]`), `/signin`, `/verify`, `/display` and `/api` are never listed. Set `PUBLIC_SITE_URL` (process env) to the public origin behind a proxy; otherwise the request's origin is used. Both paths are unlocalized (`isUnlocalized`).

## Documents (PL-6)

`/documents` and `/documents/[pid]` (`src/lib/document-view.ts` holds the words and the one next action per state). The page has a single `primary` button whose label and handler follow `nextAction(doc)`, because the static one-primary test counts literal occurrences. Actions act on `inFlight(doc)`, not the version being viewed.

## Import (T-28n)

`/plans/[pid]/import` (`src/lib/import-view.ts`): preview (`dry_run=true`) first, then the same single primary button imports. CSV goes through `ApiClient`'s `rawBody` with `content-type: text/csv`. `extractMessage` prefers the server's `description` (the reason) over `error` (the code).

## Flaky accessibility audits

Call `settle(page)` (`tests/e2e/fixtures.ts`) before any axe audit. A button that has just flipped from primary to secondary is otherwise measured halfway through its colour transition, and axe reports a false `color-contrast` failure about 1 run in 20. Check a state *before* the click that changes it, never after (an `expect.poll` for the old state after a click races the response).

## Root route and the browser's language

`/` goes to the locale the browser reads best: server `handle` (cookie, then `Accept-Language`) and client `+layout.ts` (remembered choice, then `navigator.languages`, then `navigator.language`, then `en-001`). A region the app has no locale for resolves to the language's own (`cy_GB`/`cy-GB` → `/cy-001`; `de-DE` → `/de-de`, which exists; `ja-JP` → `/en-001`). A remembered choice wins over the browser. Covered by `tests/e2e/root-locale.spec.ts`.

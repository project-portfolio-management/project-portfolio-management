# AGENTS.md — Portfolio Front-End

Operator UI for the [Portfolio Service](../project-portfolio-management-service-with-rust):
plan identity CRUD + matching + merge over **one recursive `/api/plans`
collection**, plus the project-management workspace (Kanban board,
governance, schedule) and cookie-session / SSO auth (BFF; the browser
holds no token).

## Specification Driven Development

**The `spec/` directory is the source of truth.**
- Before doing anything read all files inside the `spec/` directory using your file-reading tools.
- Treat all files inside `spec/` as strict requirements. Where any file disagrees with the spec, the spec wins: fix the file, or change the spec on purpose and bring the other files into line in the same commit.
- Do not improvise structure, style, or citation practice.

## More

> **Status: implemented (v0.2.0), verified 2026-10-30.** `pnpm run check`
> is **0 errors / 0 warnings**, `npx vitest run` passes (`tests/unit/`), and
> the Playwright suite (`tests/e2e/`, with axe audits) runs against a production
> build. There are 101 page routes under `src/routes` (2026-11-04). Test counts
> are not written here, as they go stale: test, API-route, migration and locale
> counts are in the entity spec's
> [§14 generated block](../spec/14-implementation-status.md) (`scripts/generate-status.sh`;
> it counts source lines, so a run reports more, as some tests are generated in loops). Shipped: the unified identity surface
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

A SvelteKit 3 / Svelte 5 (runes) **SPA**. It calls the portfolio service
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
  the front-end drift. `tests/unit/client-routes.test.ts` checks that every
  path in `src/lib/api/*.ts` is a route the service mounts (its
  `api-routes.txt`); it checks paths and methods, not bodies or types.
- This project has its own [`spec.md`](spec/index.md) (§1–§18) for
  front-end-specific decisions: routes, components, design system,
  build, layout shell, i18n.

## Ground rules

1. **Runes only** (`$state` / `$derived` / `$effect` / `$props` /
   `$bindable`). No `export let`, no `$:`, events are callback props.
2. **SPA.** `+layout.ts` sets `ssr = false` / `prerender = false`.
3. **TypeScript strict** (`noUncheckedIndexedAccess`). No `any` without
   a justifying comment.
4. **`DataTable`** (`src/lib/components/DataTable.svelte`, Lily's DataGrid) for
   every table: sorting, filter, paging, CSV / TSV / Excel export, copy, a
   Columns control. The Gantt (`GanttView`), calendar and task board are
   Lily's GanttChart, CalendarView and KanbanBoard. SVAR is gone. Native HTML
   for simple lists.
5. **Lily Headless** for accessibility primitives (focus trap,
   listbox, combobox, dialog) and the theme / locale selectors. Native
   HTML elsewhere.
6. **No global stores** for HTTP state — construct a
   `PlanRepository` (all paths under `/api/plans`) per page/component.
7. **No envelope.** The service is Axum and returns **raw JSON**;
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

```text
src/
├── hooks.server.ts                BFF session handling (reads the httpOnly session cookie)
├── lib/
│   ├── config.ts                  API_BASE_URL → same-origin BFF proxy (/api/proxy); no browser-held bearer
│   ├── i18n.svelte.ts             string catalogues for the 10 locales in `locales.ts` (en-001 default, en fallback), formatted by `@sveltia/i18n`; exports `t()` and `localeHref()`; one file, not a directory
│   ├── locales.ts                 pure locale set (no two-letter aliases), RTL flag, `splitPath`; shared by hooks, store and tests
│   ├── *-view.ts, wbs.ts, rbs.ts  pure view modules (no runes, no `$app/*`): ceo-view, ev-view, cpm-view, whatif-view, work-view, onboarding-view, … — tested in `tests/unit/`
│   ├── glossary.ts                plain-language glosses for acronyms; rendered by `components/Term.svelte`
│   ├── api/
│   │   ├── client.ts              lean fetch wrapper (get/post/put/patch/delete + ApiError); same-origin, so the session cookie goes with it
│   │   ├── types.ts               Plan + PlanKind + PlanStatus + Goal + Task + Issue (+ IssueKind/IssueSeverity/IssueStatus) + Relationship + RelationKind + PlanIdentifier + IdentifierScheme + PlanRef + ScoredRef + MergeRequest + MergeResponse + MergeRecordRow. No MatchBreakdown / AuditEntry / PlanEvent / TimelineRow / BurndownPoint type — those capabilities are not built (see the Status note above).
│   │   ├── capabilities.ts        CapabilityClient (reviews, assignees, notifications, automations, scheduled actions, Smart Score, lifecycle)
│   │   ├── ppm.ts                 `PpmClient` — typed endpoints for the dashboards and the scheduling, cost, work, what-if, onboarding and WBS pages (the largest module)
│   │   ├── tba.ts                 `TbaClient`
│   │   └── plans.ts               PlanRepository (list + listPage + search + get + create + update + remove + checkDuplicates + merge + recentMerges; all paths under /api/plans). No audit() or recentEvents() method.
│   ├── server/                    BFF-only (never bundled to the browser): auth.ts (magic-link + session→PASETO exchange + currentUser(), T-28f), session.ts (cookie), config.ts (PROJECT_PORTFOLIO_MANAGEMENT_API_URL / AUTH_API_URL)
│   ├── nav.ts                     T-28f (repo `tasks.md` EV-1), pure, DB-free: viewAttr/orderNavForView/landingRouteForView — role-tailored nav ordering + landing route from a deployment-declared `view` ABAC attribute
│   └── components/                PlanForm.svelte, merge-validation.ts (pure guard). Also DataTable, GanttView, CalendarMonth and the chart components. No MatchBreakdown / IssuesList / Timeline / Burndown / GoalsPanel; the board route uses Lily's KanbanBoard directly.
└── routes/
    ├── +layout.svelte / +layout.ts / +layout.server.ts   top-bar nav (leftmost hamburger) + Plans destination + theme/locale selectors + session affordance + SPA toggle; +layout.server.ts also resolves `view`/`landingRoute` (T-28f)
    ├── signin/ · verify/          per-app magic-link sign-in (BFF server routes)
    ├── api/proxy/[...path]/+server.ts   BFF proxy → portfolio service (injects the PASETO bearer)
    ├── +page.svelte               landing (links to /plans)
    └── plans/
        ├── +page.svelte          list (DataTable with its filter box over the loaded rows)
        ├── new/+page.svelte      create
        ├── [pid]/+page.svelte    detail + delete + check-duplicates (plain score/confidence, no breakdown visual)
        ├── [pid]/edit/+page.svelte    edit
        ├── [pid]/{board,schedule,governance}/+page.svelte
        └── merge/+page.svelte    merge a duplicate into a survivor + recent merge history
```

## API consumption

Every plan lives under `/api/plans` (one recursive collection).

The workforce-service bridge has four screens (FB-6): `/skill-pools` (set or clear a pool's link), `/capacity-heatmap`
(a linked pool's reading from WPM), `/capacity-commitments` (the outcome of the demand sent) and the people-impact box
on `/plans/[pid]` for a Program (`src/lib/components/PeopleImpact.svelte`). The browser never calls WPM; the service
does, under the signed-in person's token. Rules: entity spec §8.5a; tests: `tests/e2e/wpm-bridge.spec.ts`,
`tests/e2e/capacity-commitments.spec.ts`.

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
| My notifications (`/me/notifications`) | `GET /api/me/inbox?tz=` · `POST /api/notifications/{pid}/read` · `GET / PUT /api/me/notification-prefs` · `POST …/reset` · `GET / POST /api/me/notification-sinks` · `POST …/{pid}/delete` · `GET …/{pid}/deliveries` |
| Access review and emergency access (`/security/access`) | `GET /api/access-review?plan=&older_than_days=` (and `&format=csv`, a link) · `GET / POST /api/break-glass` · `POST …/{pid}/revoke` · `GET /api/plans/{pid}` (plan names) |
| My timesheet (`/me/timesheet`) | `GET /api/timesheets?week=&person=` · `POST /api/timesheets/submit` · `POST /api/timesheets/decide` · `GET /api/plans/{pid}` (plan names) |
| Strategy canvases (`/canvases`, `/plans/[pid]/canvases`, `/canvases/[pid]`, `CanvasList.svelte`) | `GET /api/canvas-templates` · `GET / POST /api/canvases` (`?plan=`) · `GET / DELETE /api/canvases/{pid}` · `POST …/items` · `PUT / DELETE …/items/{item}` · `POST …/items/{item}/promote` |
| Plan templates (`/plan-templates`, `/plan-templates/[pid]`, `/plans/[pid]/save-as-template`) | `GET /api/plan-templates` · `GET / DELETE /api/plan-templates/{pid}` · `POST /api/plans/{pid}/save-as-template` · `POST /api/plans/from-template/{tpl}` · `GET /api/plans` (the parent picker) |
| Export to Microsoft Project (`/plans/[pid]/export`) | `GET /api/plans/{pid}/export/mspdi` (a link, never drawn in the page) |
| Director pack drafting and issue (`/director-packs`, `/director-packs/[pid]`) | `POST /api/director-packs` · `POST …/{pid}/review` · `POST …/{pid}/issue` · `POST …/{pid}/reissue` · `GET /api/organizations` (organizations as audiences) |
| Director pack schedules and subscriptions (`/director-packs/schedules`, `/director-packs`) | `GET / POST /api/director-pack-schedules` · `PUT …/{pid}` · `POST …/sweep` · `GET / PUT /api/director-pack-subscriptions` |
| Delivery confidence (`/plans/[pid]/confidence`) | `GET / PUT /api/plans/{pid}/delivery-confidence` |
| Delivery exceptions (`/exceptions`) | `GET /api/exceptions?status=&breached=` · `PUT /api/exceptions/{pid}` · `POST …/{pid}/decision` · `GET /api/plans` (plan names) |
| Portfolio map and strategic aims (`/portfolio-map`) | `GET /api/portfolio-map?period=` · `PUT /api/objectives/{pid}/strategic-aim` · `PUT /api/plans/{pid}/primary-aim` · `GET /api/objectives` |
| Gates, programme status and project buffer (`/plans/[pid]/programme`) | `GET / PUT /api/plans/{pid}/project-buffer` · `GET /api/plans/{pid}/planned-gates` · `PUT …/planned-gates/{gate}` · `GET / POST /api/plans/{pid}/programme-status` · `GET …/gate-reviews` and `GET /api/exceptions` (what a decision cites) |
| Funding sources (`/funding`) | `GET / POST /api/funding-sources` · `PUT …/{pid}` · `PUT /api/plans/{pid}/budget-lines/{line}/funding-source` · `GET /api/portfolio-money?fy=&combine_classes=` · `GET /api/plans/{pid}/budget-lines` · `GET /api/plans` |
| Ledger writes (`/ledger`, `LedgerWrites.svelte`) | `POST /api/funding-allocations` · `PUT /api/plans/{pid}/budget-lines/{line}/funding-allocation` · `GET / PUT /api/cost-codes` · `POST /api/ledger/reconcile` (CSV text) · `GET /api/funding-sources` · `GET /api/plans/{pid}/budget-lines` |
| Benefit classification and validation (`/plans/[pid]/benefits`) | `PUT /api/plans/{pid}/benefits/{benefit}/classification` · `GET / POST …/validations` · `GET /api/plans/{pid}/benefits` · `GET /api/kpis?plan=` |
| Skill pool writes (`/skill-pools`, `SkillPoolWrites.svelte`) | `POST /api/skill-pools` · `PUT /api/skill-pools/{pid}` |
| Delivery model and reporting method (`/plans/[pid]/programme`) | `GET`/`PUT /api/plans/{pid}/delivery-model` · `PUT /api/plans/{pid}/reporting-method` |
| Cutover stop criteria, draft (`/plans/[pid]/cutover`) | `PUT /api/plans/{pid}/cutover-decisions/{did}/criteria` |
| Triage and thresholds (`/plans/[pid]/programme`, `ProgrammeThresholds.svelte`) | `GET`/`PUT /api/plans/{pid}/triage` · `GET`/`PUT /api/plans/{pid}/thresholds` |
| Who must sign a milestone off (`/plans/[pid]/governance`, `MilestoneSignoffRequired.svelte`) | `GET /api/milestones?plan=` · `PUT /api/plans/{pid}/milestones/{m}/signoff-required` |
| Threshold sweep and decisions (`/exceptions`) | `POST /api/threshold-sweep` · `POST /api/exceptions/{pid}/threshold-decision` |
| Scorecards and de-escalation (`/deescalation`, `DeescalationTracker.svelte`, `Scorecards.svelte`) | `GET /api/deescalation` · `PUT …/level` · `POST …/criteria` · `PUT …/criteria/{pid}/evidence` · `GET / POST /api/scorecards` · `GET /api/scorecards/{pid}` · `PUT …/{pid}/kpis` · `POST …/{pid}/from-template` · `GET /api/scorecard-template` · `GET /api/kpis` |
| Status updates and their evidence (`/plans/[pid]/status`, `StatusUpdateForm.svelte`; the due list on `/dashboard`) | `GET /api/plans/{pid}/status` · `GET`/`PUT /api/plans/{pid}/status-cadence` · `GET`/`POST /api/plans/{pid}/status-updates` · `GET /api/status-metrics` · `GET /api/status-updates/due` · `GET /api/plans/{pid}/{tasks,risks,milestones,issues}` (what a claim may cite) |
| A plan's issues, the rest (`/plans/[pid]/issues`; `/plans/[pid]/tasks/[tpid]` raises one from a blocked task) | `GET /api/plans/{pid}/issues/{issue_pid}` (read before editing) · `DELETE …/issues/{issue_pid}` (after a question) · `POST /api/plans/{pid}/issues/from-task/{task_pid}` · `GET /api/plans/{pid}/issues` (whether one is open from the task) |
| A plan's change requests, the rest (`/plans/[pid]/change-requests`) | `GET /api/plans/{pid}/change-requests/{cr}` · `POST …/{cr}/implement` · `PUT …/{cr}/links` · `GET …/{cr}/preview` · `POST …/{cr}/baseline` (after a question) · `GET /api/plans/{pid}/{issues,risks}` (what it may answer) |
| Changing a risk (`/plans/[pid]/governance`, `RiskEditForm.svelte`) | `PUT /api/plans/{pid}/risks/{risk_pid}` (only the fields that change) |
| An escalation's acknowledgement and resolution as records (`/plans/[pid]/escalations`) | `GET`/`PUT /api/plans/{pid}/escalations/{epid}/acknowledgement` (`{note?}`) · `GET`/`PUT …/resolution` (`{outcome, response}`, after a question; never overwritten) · `POST …/raise-level` and `…/withdraw` (unchanged) |
| Register completeness (`/register-completeness`, linked from `/director-packs` and `/exceptions`) | `GET /api/register-completeness` (the `note` is not shown) |
| Bulk imports and exports (`/jobs`) | `GET /api/plans/bulk-jobs` (`kind`, `status`) · `POST …/bulk-jobs/{id}/cancel` (after a question) · `POST /api/plans/export` (only what differs from the defaults) · `POST /api/plans/import?format=&on_keyless=` (the file as the raw body) · `GET …/bulk-jobs/{id}/errors?format=csv` and `GET /api/plans/export/{id}/download` (links) · `GET /api/plans/review-queue` (`job`) · `POST …/review-queue/{pid}/confirm` and `…/reject` (after a question) |
| A plan's record (`/plans/[pid]/export`, below the MSPDI file) | `GET /api/plans/{pid}/export` (a link with `download`; masked as the reader's access masks; audited by the service) |
| A template's plans and revision (`/plan-templates/[pid]`) | `GET /api/plan-templates/{pid}/plans` · `POST …/revise` (`{from_plan}`, after a question; not for a starter) |
| A plan's template changes (`/plans/[pid]/template`) | `GET /api/plans/{pid}/template-diff` (404: not made from a template) · `POST …/template-sync` (`{accept, overwrite_local?}`, after a question) |
| Many MS Project files (`/mspdi-import`, linked from `/jobs`) | `POST /api/import/mspdi-batch?dry_run=` (`{projects: [{plan_pid? | name?, xml}], parent_pid?}`; preview first, then after a question) |
| A plan's components (`/plans/[pid]/components`) | `GET /api/plans/{pid}/components` · `GET …/components/summary` · `POST …/components` · `PUT …/components/{c_pid}` · `DELETE …/components/{c_pid}` (after a question; only when no task belongs to it) |
| A framework's versions (`FrameworkPanel.svelte`, CF-4) | `GET /api/{ways-of-working,governance-frameworks}/{pid}` (another's text) · `GET …/{pid}/versions` · `PUT /api/plans/{pid}/{ways-of-working,governance-framework}` (no `version`: move to the newest; with one: pin it) |
| Checklist templates (`/onboarding-templates`) | `GET /api/onboarding/templates` · `PUT /api/onboarding/templates/{kind}` · `DELETE /api/onboarding/templates/{kind}` (after a question) |
| Custom fields (`/custom-fields`) | `GET /api/custom-fields` · `POST /api/custom-fields` (409 said first) · `DELETE /api/custom-fields/{pid}` (after a question) · `GET /api/custom-fields/search` |
| A plan's custom field values (`/plans/[pid]/fields`) | `GET /api/custom-fields` · `GET /api/custom-fields/values?applies_to=plan` · `PUT /api/custom-fields/values` (only what changed) · `GET /api/plans/{pid}` (its kind) |
| A plan's goals (`/plans/[pid]/goals`) | `GET /api/plans/{pid}/goals` (with its `ETag`) · `POST …/goals` · `PUT …/goals/{index}` · `DELETE …/goals/{index}` (both with `If-Match`; 412 read again) |
| People's primary roles (`/resources`, `ResourceRoles.svelte`) | `GET /api/resource-roles` · `PUT /api/resource-roles` (`{person_ref, role}`; `null` clears) |
| Meetings and tasks from their actions (`/plans/[pid]/meetings`, `MeetingTasks.svelte`) | `GET`/`POST /api/plans/{pid}/meetings` · `GET …/meetings/{mid}` · `POST …/{mid}/held` and `…/{mid}/cancel` (after a question) · `GET …/{mid}/pdf` (a link) · `POST …/{mid}/task-proposals/preview` · `POST …/{mid}/task-proposals/commit` (after a question, one idempotency key) · `GET /api/plans/{pid}/{tasks,risks,milestones,issues}` (what an item may point at) |
| Lessons learned and what refers to a record (`/plans/[pid]/lessons`; similar plans' lessons on `/proposals`) | `GET`/`POST /api/plans/{pid}/lessons` · `POST …/lessons/{lpid}/withdraw` (after a question) · `GET /api/plans/{pid}/references?kind=&pid=` · `GET /api/proposals/{pid}/lessons` |
| The pack's programme readings, now (`/portfolio-map`, drawn by `buildPackView`) | `GET /api/buffer-fever` · `GET /api/portfolio-earned-value?ac=` (left out for budget lines) · `GET /api/portfolio-benefits` · `GET /api/director-pack/issues-and-changes?window_days=` (left out for 30) |
| Forecast at completion rolled up (`/financials`) | `GET /api/financials/forecast?plan=&depth=` · `GET /api/plans` (the plan picker) |
| A plan's forecast, baselines, tranches and business case (`/plans/[pid]/costs`) | `GET /api/plans/{pid}/financials/forecast` · `GET`/`POST /api/plans/{pid}/budget-baselines` (after a question; one the same as the latest not sent) · `GET /api/plans/{pid}/budget-lines` and `GET /api/plans/{pid}/governance` (the stage) · `POST …/budget-lines/{line}/release` (after a question) · `POST /api/plans/{pid}/business-case` (after a question) |
| The value realized and an observed value (`/plans/[pid]/benefits`) | `GET /api/plans/{pid}/value-realization` · `POST /api/plans/{pid}/value-points` (the first measurable one after a question) |
| Phase and start (`/plans/[pid]/programme`, `PlanPhase.svelte`) | `GET /api/plans/{pid}/phase-history` · `PUT …/phase` (`start_override_reason` for a refused start) · `GET …/start-check` |
| Trade-off on a new ask (`/proposals`) | `POST /api/proposals` (`external_ask`, `tradeoff`) · `PUT /api/proposals/{pid}/tradeoff` |
| Ranking by WSJF and RICE, and a plan's estimates (`/prioritization`, `ScoringEstimates.svelte`) | `GET /api/prioritization?model=wsjf\|rice` (left out for the Smart Score) · `GET`/`PUT /api/plans/{pid}/scoring-inputs` (all eight sent; nothing for no change) · `GET /api/plans/{pid}/score?model=` |
| The efficient frontier, a generated draft and a rollback (`/scenarios`, `FrontierChart.svelte`) | `GET /api/scenarios/frontier?currency=&steps=\|caps=&must_include=` (ten steps not sent) · `POST /api/scenarios/generate` · `POST /api/scenarios/{pid}/rollback` (after a question) · `GET /api/plans` (names, and the plans to include) |
| Editing a draft and the intake's forecast (`/proposals`) | `GET`/`PUT /api/proposals/{pid}` (a draft only; nothing for no change; tags an external ask) · `GET /api/proposals/forecast?periods=&period_days=&history_periods=` (defaults not sent) |
| The day's plan (`/me`, `MyDayPlanner.svelte`) | `GET /api/me/plan?day=&tz=` · `POST /api/me/planned-slots` (sent in UTC) · `POST …/planned-slots/{pid}/delete` (after a question) · `GET /api/me/tasks` (the tasks to plan for) |
| The calendar feed (`/me`, `FeedToken.svelte`) | `POST /api/me/feed-token` (after a question; the addresses shown once, through the proxy) · `POST /api/me/feed-token/revoke` (after a question) |
| What I follow (`/me`, `MyWatching.svelte`) | `GET /api/me/watching` · `POST /api/watchers/unwatch` (after a question) · `GET /api/plans/{pid}` (plan names) |
| The work over time (`/me`, `MyWorkProfile.svelte`) | `GET /api/work-profile?period=&from=&to=` (weeks not sent) |
| Recording time (`/me/timesheet`, `TimeEntryForm.svelte`) | `POST /api/plans/{pid}/time-entries` (after a question; defaults not sent) · `GET /api/plans/{pid}/time-entries` (the caller's own kept) · `GET /api/plans` · `GET /api/plans/{pid}/tasks` |
| The time report (`/reports`, `TimeReport.svelte`) | `GET /api/time-reports?group_by=&from=&to=&plan=&person=` (and `&format=csv`, a link) · `GET /api/plans` |
| Hammocks (`/plans/[pid]/critical-path`, `Hammocks.svelte`) | `GET /api/plans/{pid}/hammocks` · `POST …/tasks/{t_pid}/hammock` (`{enabled}`, after a question) · `POST …/hammocks/{h_pid}/links` · `DELETE …/hammocks/{h_pid}/links/{member}` (after a question) · `GET /api/plans/{pid}/tasks` |
| The tasks of linked plans (`/plans/[pid]/critical-path`, `ExternalTasks.svelte`) | `GET /api/plans/{pid}/cpm` (its `external` nodes and `cross_plan` edges, drawn) |
| A plan's readings on asking (`/plans/[pid]/critical-path`, `PlanReadings.svelte`) | `GET /api/plans/{pid}/rollup?depth=` (32 not sent) · `GET …/effort` · `GET …/forecast?items=&periods=&period_days=&history_periods=` (defaults not sent) · `GET …/performance` · `GET …/workflow` |
| A task's dependencies on other plans and its sub-tasks rolled up (`/plans/[pid]/tasks/[tpid]`, `CrossPlanLinks.svelte`, `TaskRollup.svelte`) | `GET /api/dependencies?item=` · `POST /api/dependencies` (kind and lag only when not the defaults) · `DELETE /api/dependencies/{pid}` (after a question) · `GET /api/plans/{pid}/cpm` (names) · `GET /api/plans` · `GET /api/plans/{other}/tasks` · `GET /api/plans/{pid}/tasks/{t_pid}/rollup` |
| Aggressive and safe estimates, the frozen buffer plan (`/plans/[pid]/critical-chain`, `CcpmEstimates.svelte`, `FrozenBufferPlan.svelte`) | `GET /api/plans/{pid}/ccpm-estimates?limit=1000` · `PUT …/tasks/{t_pid}/ccpm-estimate` (nothing for no change; clearing after a question) · `GET …/buffer-plan` |
| The working day and time away (`/plans/[pid]/calendar`, `WorkingTime.svelte`) | `POST /api/working-time` (after a question) · `POST /api/non-working` (after a question; a blank note not sent) |
| The mix of finished work (`/plans/[pid]/flow`, `FlowMix.svelte`) | `GET /api/plans/{pid}/flow-distribution?window_days=&rollup=&depth=` (defaults not sent) |
| Many tasks at once (`/plans/[pid]/board`, `BoardBulk.svelte`) | `POST /api/tasks/bulk` (only the fields set or cleared; nothing for no change) · `POST /api/tasks/copy` (`keep_assignee` only when kept) · `POST /api/tasks/move` (after a question) · `DELETE /api/plans/{pid}/tasks/{t_pid}` (after a question, one call each) · `GET /api/plans` (the plans, on opening) |
| Following the board's changes (`/plans/[pid]/board`) | `GET /api/plans/{pid}/live` (an `EventSource` through the proxy, on asking; any event reads the board again) |
| A sprint's ceremonies and commitment (`/plans/[pid]/board`, `SprintCeremonies.svelte`) | `GET`/`POST /api/sprints/{pid}/ceremonies` (blank facilitator and summary not sent) · `GET /api/sprints/{pid}/commitment` · `POST /api/sprints/{pid}/commit` (after a question; not for a sprint with no tasks) |
| Placeholders, hours, skill gaps and the rate in force (`/resources`, `PlaceholderActions.svelte`, `PlannedHours.svelte`, `SkillGapView.svelte`, `RateLookupForm.svelte`) | `POST /api/plans/{pid}/allocations/{a_pid}/resolve` (`confirm: true` only after a question on a 409 `over_capacity`) · `DELETE /api/plans/{pid}/allocations/{a_pid}` (after a question) · `GET /api/resource-plan/hours?from=&to=` (blanks not sent) · `GET /api/plans/{pid}/skill-gap` · `GET /api/cost-rates/lookup?person_ref=&date=&table=` (`standard` not sent) · `GET /api/plans` (names) |
| Effort against capacity (`/capacity`, `UtilizationView.svelte`) | `GET /api/capacity/utilization?by=&window_days=&plan_pid=` (the team and 28 days not sent) · `GET /api/plans` |
| A plan's goal score and a key result's check-ins (`/okrs`, `PlanOkrScore.svelte`, `ObjectiveCard.svelte`) | `GET /api/plans/{pid}/okr` · `GET /api/objectives/{pid}/key-results` (on opening) · `GET /api/key-results/{pid}/check-ins` (on opening) · `GET /api/plans` |
| Every reading of a KPI (`/kpis`, `KpiCard.svelte`) | `GET /api/kpis/{pid}/readings` (the replaced ones marked) |
| Adoption recorded (`/plans/[pid]/benefits`, `AdoptionForm.svelte`) | `POST /api/plans/{pid}/adoption` |
| Task discussion and files (`/plans/[pid]/tasks/[tpid]`, `Discussion.svelte`) | `GET / POST /api/comments` · `PUT /api/comments/{pid}` · `POST …/{pid}/delete` · `GET …/{pid}/history` · `GET / POST /api/reactions` · `POST …/remove` · `GET / POST /api/watchers` · `POST …/unwatch` · `GET / POST /api/attachments` · `POST …/{pid}/delete` · `GET …/{pid}/download` (a link, never drawn in the page) |

`src/lib/api/capabilities.ts` additionally defines a per-task assign
endpoint, an assignee-workload query, and a notifications inbox
(`GET /api/notifications`); none is called by a route yet. The caller's own
inbox is read through `GET /api/me/inbox` on `/me/notifications` instead, which
marks a line read with `CapabilityClient.markRead`. There
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
- **Hermetic e2e (RE-10).** A spec imports `test` and `expect` from `./test`, never from `@playwright/test`
  (ESLint refuses it). That fixture blocks every browser request and WebSocket to an origin other than the app and
  the mock identity provider, unless the test faked it with `page.route`, and fails the test naming the blocked
  addresses. The server side of SvelteKit is not covered: it reaches only the mock API the config names.
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
  `calendar`, `status`, `meetings`, `lessons`.
- **Plan workspace links** are listed in `PLAN_LINKS` (`src/lib/modules.ts`);
  a new per-plan page needs an entry there and a matching assertion in
  `tests/unit/modules.test.ts`.

## Commands

```bash
pnpm install
pnpm dev                       # http://localhost:5173 (needs the service for real data)
pnpm run check                 # svelte-kit sync + svelte-check (strict; 0/0 expected)
pnpm run build
npx vitest run                 # unit suite (same as `pnpm test`)
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
protected against cross-site forgery by an `Origin` / `Sec-Fetch-Site` check in
the proxy (`src/lib/server/proxy-guard.ts`) on top of the SameSite=Lax cookie;
there is no `localStorage` and no `mxi_access_token`.
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
  (`src/lib/auth.svelte.ts` does not exist).
- **`ApiClient`** — browser→BFF calls (`API_BASE_URL` = same-origin
  `/api/proxy`) send the cookie because they are same-origin. There is **no
  CSRF token** (an earlier version of this file said `X-CSRF-Token`; the
  client never sent one): the proxy refuses a `POST`/`PUT`/`PATCH`/`DELETE`
  whose `Origin` is not the app's, or whose `Sec-Fetch-Site` says another site
  (CQ-5), and never forwards a browser-supplied `Authorization`.
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
both default to `http://localhost:1241` (see `.env.example`).

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

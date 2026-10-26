## 14. Implementation Status

**All three subprojects are implemented** (matcher + service +
front-end). What this file asserts about size and test volume is
**generated from the source tree**, not written by hand (§14.3); whether
the suites pass is a property of a run, so it is recorded in CI, not
here.

> **Revision note (2026-10-09).** The hand-written check counts that used
> to sit here (last run 2026-08-23 / 2026-08-26, long out of date) are
> replaced by the generated block below. The capability rows were
> reconciled against the code and the repo-root [`tasks.md`](../tasks.md),
> which is authoritative for task status.

### 14.0 Generated facts

Regenerate with `scripts/generate-status.sh`; `scripts/generate-status.sh
--check` fails when this block is stale. Do not edit between the markers.

<!-- generated:start -->
<!-- This block is written by scripts/generate-status.sh; do not edit it by hand. -->

**Generated facts** (counted from the source tree by `scripts/generate-status.sh`;
no network, database or compilation; counts are source lines, not test runs):

| Fact | Value | Method |
|---|---|---|
| Service version | 0.2.0 | `version` in the service `Cargo.toml` |
| Matcher version | 0.2.1 | `version` in the matcher `Cargo.toml` |
| Front-end version | 0.2.0 | `version` in the front-end `package.json` |
| Migrations | 132 | files named `m2*.rs` in `migration/src` |
| Mounted route registrations | 673 | `.add(` calls in `src/controllers/*.rs` (one per registration; a path registered for several methods counts once per method, so this exceeds the documented-path count) |
| Documented API paths | 443 | quoted `"/api/..."` keys in `src/openapi.rs` and `src/openapi_more.rs` |
| Service tests | 1425, plus 260 `#[ignore]`d (database or container gated) | `#[test]` and `#[tokio::test]` lines; `#[ignore]` lines, in `src`, `tests`, `migration/src` |
| Matcher tests | 80, plus 0 `#[ignore]`d | the same patterns in the matcher `src` and `tests` |
| Front-end unit tests (vitest) | 546 | `it(` / `test(` lines in `tests/unit` |
| Front-end end-to-end tests (Playwright) | 268 | `test(` lines in `tests/e2e` |
| Locales | 10 | entries of `LOCALES` in `src/lib/locales.ts` |

**Task boxes in [`tasks.md`](../tasks.md)** (ticked / open, by prefix):

| Prefix | Ticked | Open |
|---|---|---|
| LT | 12 | 0 |
| RM | 14 | 0 |
| PL | 10 | 0 |
| SC | 11 | 0 |
| PJ | 13 | 0 |
| KO | 3 | 0 |
| MS | 4 | 0 |
| CR | 4 | 0 |
| DP | 12 | 0 |
| EX | 14 | 0 |
| HD | 13 | 0 |
| CC | 6 | 0 |
| **Total** | **116** | **0** |

<!-- generated:end -->

> **This section was materially wrong until 2026-08-23** and the
> correction is worth recording rather than quietly overwriting. Its
> header had been updated when the trio landed (2026-06-19), but §14.1
> still read *"Nothing else is delivered"* and §14.2 *"The entire build
> is open"* — a document that contradicted itself in consecutive
> paragraphs, listing "No matcher crate" and "No service crate" as gaps
> against a tree containing both. A status file that is not regenerated
> is worse than no status file: it is the one place a reader goes for
> exactly the question it answers wrongly. Every row below now names
> what backs it.

The four former work-item kinds (Portfolio / Project / Product /
Program) were unified into one recursive `Plan` on 2026-07-20 (§13
T-10): `kind` is an optional descriptive label that neither gates nor
scores, containment is general `parent_ref`, and one `/api/plans`
collection sits over one `plans` table.

The 2026-08-25/26 delivery (§13 T-15 … T-27, service 0.3.0) added the
**project-management suite**: custom workflows, the OKR engine, effort
/ time tracking with per-person utilization, sprint ceremonies, project
phases, Flow Distribution, Total Project Control, the controls
register, and value realization / strategic performance — nine
migrations (`m20260825_000001` … `m20260826_000003`) and a matcher
0.2.0 wire addition (`Plan.phase`, pinned never-scored).

### 14.1 Delivered

| Subproject | Capability | Backed by |
|---|---|---|
| (entity) | Canonical specification | This §1–§18 entity spec: the domain model (§5, the canonical home) — the recursive `Plan`, its optional `kind` label, kind-agnostic matching, the matchable/operational partition — the cross-subproject DTO contract, and the family-integration adoptions |
| (entity) | Time-based analysis contract | The cross-cutting [`time-based-analysis.md`](time-based-analysis.md) (§1–§18): the transition log, cycle versus lead time, flow efficiency, first-pass yield, the service level expectation, constraint ranking, and queueing-theory flow |
| matcher | Kind-agnostic `Plan` matching | Name (Jaro-Winkler + Soundex), goal-title & keyword Jaccard, owner-scoped code, owner org, `parent_ref`, timeframe proximity, relationships & tags; deterministic short-circuits on Jira / Asana / Trello / MS-Project / GitHub / Linear ids / URI / UUID, same-owner code, `sameAs` URL. **No kind gate** — `MatchBreakdown.kind_gate_blocked` is vestigial and always `false` |
| matcher | `Plan.phase` / `PlanPhase` (0.2.0) | An additive `#[serde(default)]` DTO field — the five process groups (`initiating` … `closing`) — informational-only and **pinned never-scored** (`phase_is_not_scored`), following the `PlanStatus` precedent |
| matcher | Input-size caps + never-panic | SEC-M1 per-field and array caps; `proptest` property harness; `fuzz/` targets |
| service | Plans CRUD + matching + merge | One `/api/plans` collection over one `plans` table (nullable `kind`, nullable `parent_pid`); the API DTO **is** the matcher's `Plan`, stored verbatim as JSONB; `match`, `check-duplicates`, `merge` + `merges/recent`; containment-cycle `422` |
| service | Tantivy full-text search | `src/search/` — fuzzy + phonetic name search; `kind` is a **search filter** (`?kind=`), deliberately never a duplicate-detection gate |
| service | Operational sub-resources | Goals / objectives, tasks (Kanban board with WIP-limit caps), sprints + notes, milestones, risks, budget lines, benefits, ideas, proposals, scenarios, gate reviews, allocations, dependencies, automations, notifications, report definitions |
| service | Derived views | Burndown (honest — real `done_at` stamps only, no ideal line), velocity, standup digest, executive health / decisions / benefits / alignment, financial variance + exposure, technology radar / debt / flow, board pack, auditor trail + findings + evidence pack, compliance register, risk heatmap, regulator extract |
| service | **Time-based analysis** | `task_transitions` (append-only, written in-transaction by the existing task create/move calls, with a labelled backfill); the pure `src/tba.rs`; `GET /api/plans/{pid}/{time-analysis,constraints,aging-wip,flow}`, `GET /api/plans/{pid}/tasks/{t_pid}/{transitions,time-analysis}`, `GET /api/flow-classes`. See [`time-based-analysis.md`](time-based-analysis.md) §16 |
| service | Audit log + durable outbox events | `audit_logs`; `event_outbox` with `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT` (default `memory`); `src/relay.rs` relay + `FluvioSink` behind the off-by-default `fluvio` feature |
| service | Auth + ABAC | Offline PASETO v4.public verification via `authentication-verifier`; the blanket `/api/*` guard behind `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` (**on by default** since HD-3); record-level `authorize_record` + `mask` obligations; proved end-to-end by `tests/enforcement.rs` |
| service | Privacy masking | `src/privacy.rs` + `tests/masking.rs` |
| service | Integrity | `src/compliance/` — record content hashing, audit integrity, keyed HMAC MACs via the shared `integrity-mac` crate |
| service | Custom workflows (T-15 / FR-26) | Pure `src/workflow.rs`; migration `m20260825_000005_workflows` (three tables, schema-enforced state categories, one-initial-state partial unique index); `POST`/`GET /api/workflows`, `DELETE /api/workflows/{pid}`, `GET /api/plans/{pid}/workflow`; the task create/move paths validate against the workflow in force; `done_at` stamped from the state **category**; flow classes derived per plan |
| service | OKR engine (T-16 / FR-27) | Pure `src/okr.rs`; migration `m20260825_000006_key_results`; key results anchored to `objectives` (never `goals[]`, which carry no id); `POST`/`GET /api/objectives/{pid}/key-results`, `POST`/`GET /api/key-results/{pid}/check-ins`, `GET /api/plans/{pid}/okr` weighted by the existing `objective_links` weight |
| service | Effort / time tracking (T-17 / FR-28) | Pure `src/effort.rs`; migration `m20260826_000001_effort`; `POST`/`GET /api/plans/{pid}/time-entries`, `GET /api/plans/{pid}/effort` — roll-ups per plan/task/assignee, every one labelled asserted; uncategorized effort reported separately; > 1440 min/day refused |
| service | Sprint ceremonies (T-18 / FR-29) | Migration `m20260826_000002_ceremonies`; `POST`/`GET /api/sprints/{pid}/ceremonies`, `POST /api/sprints/{pid}/commit` (once-only, refused twice by handler *and* partial unique index), `GET /api/sprints/{pid}/commitment`; every ceremony kind reported even at zero |
| service | Project phases (T-19 / FR-30) | Pure `src/phase.rs`; migration `m20260825_000003_phase_transitions` (denormalized `plans.phase` + CHECK, append-only log, no backfill); `PUT /api/plans/{pid}/phase` (a skip is `422` naming the skipped phase; regression needs a reason), `GET /api/plans/{pid}/phase-history`; `DELETE` on the history is `405` |
| service | Flow Distribution (T-20 / FR-31) | Pure `src/distribution.rs`; migration `m20260825_000004_flow_type` (nullable CHECK-constrained `tasks.flow_type`; `unclassified` not storable); `GET /api/plans/{pid}/flow-distribution` with the subtree rollup; the work type is **declared** on task create, never derived |
| service | `plan_phase_changed` automation trigger (T-21, partial) | Its own trigger, not folded into `plan_stage_changed`; phase filters validate against phases, not task statuses; the phase change commits before the rule fires |
| service | Value realization (T-22 / FR-33) | Pure `src/value.rs`; migration `m20260826_000003_value` (four tables); `POST /api/plans/{pid}/{business-case,value-points,adoption,satisfaction}`, `GET /api/plans/{pid}/value-realization` — no value points is `unrealized` never a total loss; Time to Value is a p50/p85 distribution; mixed currencies withhold the ROI; the measured-vs-asserted evidence mix is disclosed |
| service | Strategic performance (T-23 / FR-34 / FR-36, partial) | `satisfaction_responses` + `GET /api/plans/{pid}/performance`: NPS always with its response count (`null` + `no_responses`, never zero); responses store a role, never an identity; SPI/CPI report `null` with `no_baseline`, never `1.0` |
| service | Per-person utilization (T-24 / FR-35) | `working_time_configs` + `non_working_periods`; `GET /api/capacity/utilization?by=plan\|team\|person` under the [`time-based-analysis.md`](time-based-analysis.md) §7.1 obligations — leave leaves the denominator (`null` + `all_non_working`, never 0%), suppression is visible, team ratios sum rather than average, ≥ 100% flags and never clamps; integer arithmetic throughout |
| service | Total Project Control (T-25 / FR-37) | Pure `src/tpc.rs` (DIPP / progress index / banding / triage; minor units + basis points, no float); migration `m20260825_000001_total_project_control` with the DPI as a `GENERATED ALWAYS` column; `POST`/`GET /api/plans/{pid}/tpc`, `GET …/tpc/report`, `GET /api/tpc`. Mapping doc: [`total-project-control/index.md`](total-project-control/index.md) |
| service | Controls register (T-26 / FR-38–39, partial) | Pure `src/controls.rs` (three timings, four comparators, verdict derivation, coverage); migration `m20260825_000002_controls` (the `verdict` CHECK keeps `unmeasured` a real third value); nine routes under `/api/plans/{pid}/controls`, `/api/controls`, `/api/readings`, `/api/actions`; feedforward may block, feedback may only record; `POST /api/actions/{pid}/convert` turns an action into a task on its own plan (issue conversion deferred — no `issues` store) |
| service | OpenAPI + Swagger | Hand-written `src/openapi.rs`, served at `/api-docs/openapi.json` + `/swagger-ui`, with tests pinning that the documented paths match the mounted routes. All mounted route groups — including the new workflow / effort / ceremony / value groups — are documented; that coverage is landing as PRO-P16 concurrently with this snapshot (2026-08-26) |
| service | API versioning | Version-free URLs; `Accepts-version` header negotiation (`src/version.rs`) |
| service | **2026-10 delivery (root `tasks.md` series LT / RM / PL / SC / KO / MS / CR / PJ)** | Migrations `m20261002_000001` onward (the migration count is in §14.0). Per-series status lives in [`tasks.md`](../tasks.md) (the LT / RM / PL / SC / KO / MS / CR / PJ series are ticked; the DP, EX and CC series are planned and the HD series is in progress — counts in §14.0); rows below name the controller or module that backs each recent group |
| service | Cost rates, assignments, work contours (PJ-2 / PJ-5 / PJ-6) | `src/cost_rates.rs`, `controllers/cost_rates.rs`, `controllers/assignments.rs`, `src/work_profile.rs`, `src/scheduling_rules.rs`; migrations `…_000040_assignment_fields`, `…_000041_cost_rates` (effective-dated, append-only rates; no rate is `null`, never zero); finance access decided by the policy (`auth::service_rules`) |
| service | Hammock activities (PJ-7) | `src/hammock.rs`, `controllers/hammock.rs`; migration `…_000044_hammocks` (`tasks.is_hammock`, `hammock_links`); a derived span outside the dependency network; `hammocks` on the CPM response; link edits serialized |
| service | What-if sandbox (PJ-9) | `src/sandbox.rs`, `controllers/sandbox.rs`: `POST /api/plans/{pid}/sandbox/preview` (writes nothing) and `…/sandbox/commit` (`If-Match` plan ETag, `428` without, `409 stale_preview`); the leveling apply goes through the commit |
| service | Cross-plan task links (PJ-13) | `controllers/cpm.rs` (`load_network_cross`, `external` nodes, `restricted_edges`, `external_truncated`); a link whose both ends the caller cannot read is opaque; deleting a plan with task links is `409` |
| service | Cancellable jobs with progress (PJ-11) | Migration `…_000045_job_progress_cancel` (monotone progress trigger, `cancelled` status); `POST /api/plans/bulk-jobs/{id}/cancel` |
| service | Onboarding, offboarding, handover | `src/onboarding.rs`, `controllers/onboarding.rs`, `controllers/handover.rs`; migrations `…_000043_onboarding`, `…_000048_onboarding_one_open` (one open case per person, enforced in the database); front-end `/plans/[pid]/onboarding`, `/plans/[pid]/handover`. Not a numbered task in `tasks.md` (landed in commit `e1214d4`) |
| service | Announcements | `src/announcements.rs`, `controllers/announcements.rs`; migration `…_000042_announcements`; policy-decided access; front-end `/announcements` |
| service | Project escalations (FR-54) — **built** | `src/escalations.rs` (pure rules, 4 unit tests), `controllers/escalations.rs`, migration `…_000083_escalations` (`escalations`, append-only `escalation_events`); raise at a level of the governance escalation path (FR-52), acknowledge, take up a level, resolve, withdraw; `GET /api/escalations` register. Tolerances report `open_escalations` and `escalation_needed`. Tests: `tests/requests/escalations.rs` (2); the private-plan census covers the register. Screens: `/plans/[pid]/escalations` (raise, work, history) and `/escalations` (the register), linked from a red tolerance, in English and Welsh. **Not built:** notification of the body escalated to, a check on who may acknowledge or resolve, raising an escalation automatically when a tolerance turns red, an escalation that spans several plans | `src/controllers/escalations.rs` |
| service | Tolerances with red/amber/green (FR-53) — **built** | `src/tolerances.rs` (pure rules, 5 unit tests), `src/inherit.rs` (the nearest-ancestor rule shared with FR-52), `controllers/tolerances.rs`, migration `…_000082_plan_tolerances`; `GET /api/plans/{pid}/tolerances`, `PUT/DELETE …/{dimension}`, `GET /api/tolerances`. Four dimensions measured from the schedule, the budget baseline, the risk register and the bookings. Test: `tests/requests/tolerances.rs` (every colour on every dimension, inheritance, refusals, the board); the private-plan census covers the board. Screens: `/tolerances` (the board) and `/plans/[pid]/tolerances` (set, remove, read), in English and Welsh, each colour a word and a shape. **Not built:** scope, quality and benefits tolerances; a baseline-relative time measure (it uses `target_date`); raising an escalation automatically when something turns red | `src/controllers/tolerances.rs` |
| service | Ways of working and governance frameworks (FR-52) — **built** | `src/frameworks.rs` (pure rules and inheritance, 5 unit tests), `controllers/frameworks.rs` (both families from one implementation), migration `…_000080_frameworks`; `POST/GET /api/ways-of-working` and `/api/governance-frameworks`, `PUT/GET /api/plans/{pid}/ways-of-working` and `…/governance-framework`. A plan follows its own, else the nearest ancestor's, resolved on read. Tests: `tests/requests/frameworks.rs` (2), and the private-plan and paging censuses cover the lists. **Not built:** any screen, versions of a record, or checks that a plan's real workflow matches its declared WIP limit | `src/controllers/frameworks.rs` |
| service | Approval processes and controlled documents (PL-2 / PL-6) — **built** | `src/processes.rs`, `controllers/processes.rs` (migration `…_000046_processes`: versioned definitions, append-only execution log); `src/documents.rs`, `controllers/controlled_documents.rs` (migration `…_000047_controlled_documents`: an effective version is immutable, one effective per document, enforced by trigger and partial unique index); both build on the PL-1 approval request. Front-end: `/documents` for controlled documents; none yet for processes |
| service | Private-plan guard and paging (RM-8 / RM-8b / RM-12b) | `controllers/plan_access.rs` (`HiddenScope`, applied to every aggregate; path-encoding bypass closed in commit `29154a6`); `controllers/paging.rs` (`paging_mw`: `limit`, `offset`, `X-Total-Count` / `X-Limit` / `X-Offset` on every bare-array `GET`); `controllers/task_get.rs` (single-task read with `?include=`) |
| front-end | 2026-10 pages | `/cost-rates`, `/plans/[pid]/costs`, `/work`, `/what-if`, `/onboarding`, `/handover`, `/wbs`, `/resources/rbs`, `/critical-path`, `/leveling`, `/earned-value`, `/schedule-risk`, `/joint-risk`, `/schedule-health`, `/raci`, `/okrs`, `/kpis`, `/milestones`, `/ceo`, `/announcements`, plus the LT-12 plain-language and one-primary-action rules as tested checks. Route list read from `src/routes`; test counts unverified |
| front-end | Operator SPA |       90 page routes (counted as `+page.svelte` files), including plans list / new / detail / edit / merge, per-plan board / schedule / governance, plus dashboard, executive, financials, technology, board, auditor, compliance, regulator, risk, security, scenarios, proposals, ideas, objectives, prioritization, capacity, calendar, gantt, engineering, lifecycle, reports, reviews, automations, signin, verify |
| front-end | Stack + chrome | SvelteKit 2 + Svelte 5 runes, Lily Design System widgets (`@lilydesignsystem` data grid behind one `DataTable` component, Kanban, Gantt, calendar, headless), 10-locale i18n, BFF server routes (`src/lib/server/`) so no token reaches the browser |
| service | Cross-plan rollup | `GET /api/plans/{pid}/rollup` — flow across a plan and everything it contains: the union of the subtree's tasks (never an average of the children's ratios) plus the per-plan comparison, over a bounded, cycle-safe containment walk that reports its own caps |
| service | Delivery forecasting | `GET /api/plans/{pid}/forecast` — Monte-Carlo over the plan's **throughput** history (both "how long for N items" and "how many in N periods"), deterministic by seed, refusing below a minimum history rather than computing from noise. Correcting a claim this spec itself had wrong: a batch forecast needs throughput, not cycle time |
| service | Flow gauges | `src/flow_metrics.rs` — a default-off refresh loop publishing the `ppm_flow_*` family: flow efficiency, p85 cycle time, WIP, first-pass yield and over-cap column count per plan, capped and small-board-suppressed because `/metrics.prom` is on the public allow-list |
| front-end | Time-based analysis | `/plans/{pid}/flow`: the SLE badge, aging-WIP board, an inline-SVG cumulative flow diagram (no charting dependency, validated palette, table view), constraints, Little's-Law rates, and the cycle/lead distributions. `src/lib/api/tba.ts` + `src/lib/components/CumulativeFlow.svelte` |
| front-end | Tests | vitest units in `tests/unit` and Playwright specs in `tests/e2e`; counts in §14.0 |

### 14.2 Open gaps

Live gaps, each pointing at the task that closes it. Absent from this
list means delivered above or in [`tasks.md`](../tasks.md). Rows last
checked against the code on 2026-10-06 unless marked unverified.

| Gap | Task |
|---|---|
| No cross-service link write-side (`entity_links`, `linked`/`unlinked` events): no such entity or route exists. The `plan_links` table (LT-8, migration `…_000020`) is a different thing, typed links from a plan to a document, ticket or URL | T-7 |
| No FHIR surface — **deliberate**, not a gap: no FHIR resource models a plan (`fhir.md` (shared family guidance, not in this repository) §3 puts portfolio out of scope) | — |
| **Issues register: plan CRUD and the cross-plan register** — `/api/plans/{pid}/issues` and `GET /api/issues` with the `/issues` page exist (IR-1, IR-2, 2026-10-10); raising an issue from a risk or a blocked task exists in the API only (IR-3, no button yet); change requests are built in the API with a list page (IR-4, IR-5); drafting and deciding have no screens, and the director-pack report (IR-6) waits for the pack | FR-14, FR-14a |
| Automation breadth: field-change, date-arrival and SLE-breach triggers and multi-action rules — status unverified since 2026-08-26 (see §13 T-21, partly landed) | T-21 |
| Source-tool import: MS Project (`POST /api/plans/{pid}/import/mspdi`, `src/controllers/mspdi_import.rs`, PJ-10), Jira and Asana (`POST /api/plans/{pid}/import/jira` and `…/asana`, `src/controllers/tracker_import.rs`, T-28n) are **built**. MS Project (MSPDI) **export** is built too (`GET /api/plans/{pid}/export/mspdi`, `src/controllers/mspdi_export.rs`, 2026-10-20; it does not carry assignees, resources, baselines or partial progress). **Not built:** no JSON or live-API pull for Jira / Asana | T-28n, PJ-10 |
| Hammock follow-ups: no front-end, the MSPDI hammock id is not imported, hammocks link tasks of one plan only | PJ-7 |
| WBS sibling reordering (`PUT /api/plans/{pid}/task-order`, buttons only, no drag and drop) and the RBS drill-down from a role to its people's open tasks (`GET /api/resource-plan/role-tasks`, resource managers and admins) exist since 2026-10-20. Still missing: a drill-down for skills (they have no people), for the unclassified row, and from a person to their tasks outside a role | PJ-12 |
| A cancelled export produces no file; a running import may process up to 9 rows after a cancel; simulations are not jobs and cannot be cancelled | PJ-11 |
| Front-end lint / e2e-in-CI status — unverified since 2026-08-23 | §15 / HD-2 |
| Members sub-resource (comments landed as LT-4; posts and members not found in the code) | §15 |
| No cross-service link **aggregator** (`link-graph-service`) — out of this trio's scope | §15 |

**Planned, not built:** nothing in the director-pack (DP-1 … DP-12),
critical-chain (CC-1 … CC-6) or escalation (EX-1 … EX-14) series is open
as a capability; each is built at the API, with the honest gaps recorded in
[`tasks.md`](../tasks.md) under its item. What is **not** built is mostly
screens: the pack page, the critical-chain, buffer and drum pages exist, and
the rest of these capabilities (organizations, thresholds, start control,
cutovers, scorecards, the ledger and so on) are reached through the API only.
The production-readiness series
(HD-1 … HD-9) is in progress — see [Release and operations in
§15](15-roadmap.md#release-and-operations).

Closed since the 2026-08-26 snapshot (moved out of this table per
§14.3): bulk import / export and the duplicate review queue (T-8, PJ-11,
`controllers/bulk.rs`, `review_queue_items`), the phased budget baseline
and SPI / CPI / EAC (T-28b), scenario rollback (T-28a), skill-aware
allocation and capacity at scale (T-28c / d), report grouping and
schedules (T-28e), role-tailored navigation (T-28f), saved views
(T-28l), outbound webhooks and chat / email sinks (T-28m, LT-6, RM-11),
and the go-live runbook (T-28o). Three older rows (a workflow edit
route, the PM-suite front-end routes, the prettier failures) were not
re-checked and are dropped here rather than carried forward unverified.

### 14.3 Keeping this section honest

This file is a **snapshot with an expiry date**, and the 2026-08-23
correction above is what happens when nobody regenerates one. Two rules
for editing it:

1. **A row claims only what a command proves.** §14.0 is generated by
   `scripts/generate-status.sh` and checked with `--check`; a capability
   row names the module or endpoint that backs it. "Implemented" without either is a claim,
   not a status.
2. **Moving a row out of §14.2 and into §14.1 is part of the PR that
   closes it** — the same three-part discipline as a spec change. A gap
   that outlives its fix, or a delivered row that outlives its feature,
   is the failure mode this section already had once.

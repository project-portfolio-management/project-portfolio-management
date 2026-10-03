# Plan: ideas from Leantime

Research date: 2026-10-02. Subject: [leantime/leantime](https://github.com/leantime/leantime)
(v3.10.1; PHP / MySQL; AGPLv3; ~11.7k stars).

**Evidence base, stated plainly.** Read: the GitHub README, the latest release
notes, and the `app/Domain` module listing (one directory per feature). Not
read: docs.leantime.io (it would not render for the fetcher), the source of any
module, or a running instance. Everything below about *what Leantime does* is
from those three sources; anything about *what we carry* was checked against our
mounted routes and entities, not the roadmap. Treat module names as pointers to
verify, not as a spec.

## 1. What Leantime is

"An open source project management system for non-project managers", built
with ADHD, dyslexia and autism in mind: "as simple as Trello, as feature-rich
as Jira". Its audience is small teams and individuals, not an enterprise PMO.
That is the opposite end of the market from this trio (a portfolio / governance
service with a disclosed-input stance), so the useful transfer is **usability
and lightweight strategy tooling**, not governance depth, where we are ahead.

Its domain modules, grouped:

| Group | Modules (`app/Domain/*`) |
|---|---|
| Work | `Tickets`, `Sprints`, `Timesheets`, `Calendar`, `Projects`, `Clients` |
| Strategy | `Canvas`, `Goalcanvas`, `Logicmodelcanvas`, `Blueprints`, `Ideas`, `Reports` |
| Knowledge | `Wiki`, `ContentTemplates`, `Comments`, `Reactions`, `Files`, `Tags` |
| Platform | `Api`, `Connector`, `Plugins`, `Modulemanager`, `Notifications`, `Queue`, `Cron`, `CsvImport`, `Audit` |
| Identity | `Auth`, `TwoFA`, `Ldap`, `Oidc`, `Users`, `Setting` |
| Experience | `Dashboard`, `Widgets`, `Menu`, `Gamecenter`, `Help` |

Recent direction (3.10.1 notes): personal webhooks for notifications;
server always in UTC with the user's timezone applied at the edge; file-upload
and CSV-export hardening; idea-board and Gantt fixes.

## 2. Where we already match or lead

Checked against our routes; no action needed:

- **Work:** tasks, board, Gantt / schedule with critical path, dependencies,
  sprints + velocity + burndown, milestones, time entries (with `billable`),
  working-time calendar, capacity and workload.
- **Strategy:** OKR engine, objectives registry and weighted alignment, ideas
  with voting and conversion, scenarios, Smart Score, gate reviews, business
  cases, benefits and value realization.
- **Ceremonies / retros:** `sprints/{pid}/ceremonies`, notes, standup.
- **Platform:** saved views (T-28l), outbound signed webhooks (T-28m),
  in-app notifications, audit + verify, OpenAPI + Swagger, 13 locales, search,
  role-tailored navigation.
- **Depth Leantime does not have at all:** per-currency financial forecasting,
  flow metrics, Monte-Carlo forecast, Total Project Control, compliance and
  regulator surfaces, ABAC with masking.

## 3. Gaps worth considering

Ranked by value to *this* product, not by Leantime's emphasis. Each maps to a
task in [tasks.md](tasks.md) (`LT-n`).

| # | Idea | Why it matters here | Fit | Task |
|---|---|---|---|---|
| 1 | **Strategy canvases** (Lean, Business Model, SWOT, risk, empathy, logic model, goal canvas) as one generic canvas engine | We have objectives and business cases but no lightweight pre-proposal thinking tool. Intake (`proposals`) starts too late for "what are we even trying?" | High; ties into proposals and OKRs | LT-1 |
| 2 | **Plan templates / blueprints** (clone a plan tree with tasks, milestones, workflow) | Every new project is built from scratch today. Biggest time-to-value gap for the onboarding story (T-28p) | High | LT-2 |
| 3 | **"My work" view** — to-dos bucketed by due date (overdue / today / this week / later), quick-add, one focus list | Leantime's whole pitch. We have persona dashboards but no personal task inbox | High; front-end plus one read endpoint | LT-3 |
| 4 | **Comments + reactions on any entity** | `collaboration` posts/comments are deferred in the API spec; reactions absent. Decisions get made in comments | Medium-high | LT-4 |
| 5 | **File attachments** | No attachment store; evidence for gate reviews and risks has nowhere to live. Shares the artifact store designed for T-8 | Medium-high; reuse T-8 artifact store | LT-5 |
| 6 | **Personal webhooks / per-user delivery** + **chat sinks** (Slack, Mattermost, Discord, Telegram payload shapes) | Our webhooks are deployment-level. Per-user delivery is Leantime's newest feature | Medium; extends T-28m | LT-6 |
| 7 | **iCalendar feed** (subscribe milestones, sprint dates, due dates in any calendar app) | We have a calendar page and milestone-calendar, but no export a calendar app can subscribe to | Medium; small, read-only | LT-7 |
| 8 | **Wiki / documentation with templates** | Knowledge lives outside the registry. Could instead be refused in favour of links (see §4) | Low-medium; decide | LT-8 |
| 9 | **Timezone doctrine**: store UTC, apply the user's zone at the edge | Leantime shipped this as a bug fix after timestamps stored in viewer-local time. Audit ours before it bites | Medium; an audit, not a feature | LT-9 |
| 10 | **Task relations beyond dependency** ("related to", "duplicates") | Leantime's `Entityrelations` and a 3.10.1 fix show it is used. We model plan dependencies only | Low-medium | LT-10 |
| 11 | **Timesheet views** (weekly grid, per-person approval, billable roll-up) | Entries exist with `billable`; the *view and approval* do not | Medium | LT-11 |
| 12 | **Neurodivergent-friendly UX pass** — reduce clutter, plain-language labels, reading-order, motion/contrast settings, dyslexia-friendly font option | Leantime's stated design goal; cheap, and aligns with our existing mobile audit (T-28k) | Medium; audit plus settings | LT-12 |

## 4. Deliberately not taken

- **Plugin architecture and marketplace.** Same reasoning as T-28's refusal of a
  connector catalogue: the open API, bulk, and signed webhooks are the
  integration surface. A plugin runtime would let third-party code run beside
  data this service guards with ABAC and masking.
- **JSON-RPC API.** We already publish a pinned OpenAPI 3 REST surface; a second
  protocol is cost without a consumer.
- **Screen and webcam recording.** Out of scope for a registry service and a
  privacy liability.
- **Gamification (`Gamecenter`).** Unverified what it does; points and badges
  conflict with the measurement stance (absent evidence stays `null`). Revisit
  only if LT-12's findings ask for it.
- **LDAP / OIDC / 2FA.** Belong to the authentication service (EV-2 tracks
  enterprise IdP federation); not a change here.
- **PHP/MySQL-era choices** (module manager, in-app marketplace updates).

## 5. Sequencing

**Owner input (2026-10-02): roughly half the users are individual workers,
not project managers.** That promotes the personal-experience items (LT-3,
LT-12, LT-4) from nice-to-have to core, and it changes the product's shape:
two audiences share one service, so the individual's path must work without
the PMO machinery being in the way. Governance depth stays; it just must not
be the first thing a worker sees.

1. **Individual-worker path first:** LT-3 (My work) is now the top item, with
   LT-12 (simple mode, plain language, accessibility) designed alongside it
   rather than after. LT-9 (timezone audit) is a prerequisite for LT-3's
   "today" being correct.
2. **Cheap and shared:** LT-7 (iCal), so a worker sees due dates in the
   calendar app they already use.
3. **Collaboration a worker uses daily:** LT-4 (comments / reactions), then
   LT-11 (timesheet views: workers fill them in, managers approve).
4. **Needs T-8's artifact store:** LT-5 (attachments); LT-2 (templates) shares
   the clone path with T-8's import.
5. **Strategy layer for the PM half:** LT-1 (canvases).
6. **Extend what exists:** LT-6, LT-10.
7. **Decide before building:** LT-8 (wiki).

LT-2 and LT-5 are blocked on, or shaped by, the T-8 design awaiting approval
([spec/bulk-import-export.md](spec/bulk-import-export.md)).

## 6. Open questions for the owner

- ~~Is the target user ever a non-project-manager?~~ **Answered:** about half
  are individual workers. LT-3 and LT-12 are core (see §5).
- With two audiences, should the signed-in landing page differ by role
  (T-28f's `view` attribute already exists), with workers defaulting to
  `/me`?
- Wiki: build, or link out to the organisation's existing docs system?
- Canvases: generic engine (one table, typed templates) or a bespoke table per
  canvas? The task assumes generic.
- Should comments / reactions be on *every* entity or only plans, tasks, risks
  and proposals?

---

# Second source: Redmine

Research date: 2026-10-02. Subject: [Redmine](https://www.redmine.org/projects/redmine/wiki)
(Ruby on Rails; GPL v2; self-hosted; 49 languages).

**Evidence base.** Read: the wiki home page, the Features page, the
RedmineIssues page and the REST API page. The issues page names many features
(watchers, private issues, bulk edit, saved queries, categories) without
describing them, so those are treated as pointers. Not read: the source, the
plugin directory, release notes. What we carry was checked against our
entities and routes (no `custom_fields`, watchers, bulk-edit or per-role
workflow rule exists today).

## 7. What Redmine is, and how it differs from Leantime

Redmine is the configurable generalist: **its core idea is that the
organisation defines its own vocabulary**: issue types, statuses, custom
fields, roles, and a transition matrix per type and role. It is
administrator-centric and sits at the opposite pole from Leantime's
"simple for non-project-managers". For this product that is useful as the
*other half* of the owner's two audiences (§5): Leantime informs the
individual worker's path; Redmine informs the PMO's configurability.

## 8. Where we already match

- **Configurable workflows:** T-15 (custom workflow states and transitions per
  plan, category-mandatory, WIP caps).
- **Issue relations (partially):** plan-to-plan `plan_dependencies` with cycle
  refusal, lag and critical path; LT-10 plans task relations.
- **Gantt, calendar, time entries, roles/ABAC, REST + OpenAPI, i18n (13
  locales), activity events, audit.**
- Redmine has **no** equivalent of our scenario modelling, flow metrics,
  forecasting, TPC, per-currency finance, masking or signed audit chain.

## 9. Gaps worth considering

| # | Idea | Why it matters here | Fit | Task |
|---|---|---|---|---|
| 1 | **Typed custom fields** on plans and tasks (text, date, bool, integer, list, multi-list), per `kind`, filterable and reportable | The single most-requested enterprise PM feature, and Redmine's defining one. Today a team that needs "cost centre" or "regulatory class" has no place for it | High | RM-1 |
| 2 | **Per-role transition rules** on workflows (who may move X → Y) | T-15 constrains *which* moves are legal, not *who* may make them. Redmine's matrix is per tracker **and role** | High; small extension of T-15 | RM-2 |
| 3 | **Watchers** — follow a plan / task and be notified of changes | Our notification targets are owner + prior commenters (LT-4). Following is how a worker keeps up without owning | High for the worker audience | RM-3 |
| 4 | **Relation semantics** — `precedes` auto-constrains dates, `blocks` prevents closing, `duplicates` closes the twin, `copied_from` | Refines LT-10 with Redmine's concrete, proven behaviours | Medium-high | amend LT-10 |
| 5 | **Subtask roll-up** — parent's dates, % done, estimate and spent time computed from children | Verify what we derive today; a PMO reads parents, not leaves | Medium; audit first | RM-5 |
| 6 | **Atom / RSS activity feeds** (activity, issues, changes) | Pairs with LT-7's iCal: a worker follows their work in the reader they use. Cheap and read-only | Medium | RM-6 |
| 7 | **Per-plan module toggles** (tracking, time, canvases, comments… on or off) | Directly serves two audiences: a personal plan shows tasks and time only; a PMO plan shows governance. Redmine does this at project level | High; ties to LT-12 simple mode | RM-7 |
| 8 | **Private plans / private tasks** | Redmine's public/private projects and private issues; we have ABAC but no simple owner-set visibility | Medium | RM-8 |
| 9 | **Bulk edit, copy and move** of tasks across plans | In-app counterpart to T-8's file import | Medium | RM-9 |
| 10 | **Time reports grouped by user / type / category / activity** | Entries exist; T-28e just gave saved reports `group_by` but only over plans | Medium; extends T-28e and LT-11 | RM-10 |
| 11 | **Email as a delivery sink** (outbound) | Redmine's notifications are email-first; ours are in-app only by design. Reuse LT-6's sink framework rather than a new subsystem | Medium; decide | RM-11 |
| 12 | **`include=` expansion and consistent paging metadata** on the REST API | One request instead of five for the worker's screens; Redmine's `include`, `limit`/`offset`, `total_count` | Low-medium | RM-12 |

## 10. Deliberately not taken (Redmine)

- **Inbound email creating or commenting on issues.** A mail receiver is a new
  attack surface (spoofed senders, attachments) for a service whose guard is
  bearer tokens. Outbound only (RM-11).
- **Admin impersonation (`X-Redmine-Switch-User`).** Useful for support, but
  it lets one principal act as another; if ever wanted it needs dual
  attribution in the audit chain, not a header.
- **SCM repository browsing** (SVN/Git/Hg diff and annotate). Out of scope;
  `devops/events` and `engineering/delivery-links` already link work to code.
- **Forums and a separate news module.** LT-4's comments cover discussion;
  announcements can be a plan-level pinned comment.
- **Plugin and theme system** — refused for the same reason as in §4.
- **Account self-registration modes and LDAP** — the authentication service.

## 11. Sequencing with the Leantime items

1. Worker path stays first (LT-3, LT-12, LT-9), now with **RM-7** (module
   toggles) as the mechanism that keeps a worker's screens uncluttered, and
   **RM-3** (watchers) so LT-4's comments reach people who follow without
   owning.
2. **RM-2 and RM-1** are the PMO half: configurability with permission.
3. LT-10 absorbs RM's relation semantics.
4. RM-6 ships with LT-7 (one "subscribe" story: iCal and Atom).

## 12. Open questions for the owner (Redmine)

- Custom fields: per `kind`, per plan, or global? (Assumed per `kind`, with an
  admin-declared field set.)
- Are custom fields ever **matchable** (deduplication signal)? Assumed no, in
  keeping with the matchable / operational partition.
- Is outbound email wanted at all, given in-app-only is a stated design stance?

---

# Third source: Platform Collective (a Huly fork)

Research date: 2026-10-02. Subject:
[Platform-Collective/platform](https://github.com/Platform-Collective/platform)
(TypeScript + Svelte; EPL-2.0; 57 stars; pushed today). An independent fork of
**Huly Platform**, so this is evidence about the Huly model.

**Evidence base.** Read: the README, `ARCHITECTURE_OVERVIEW.md`, the directory
listings of `models/`, `plugins/` and `server-plugins/`, and the exported type
names of the `tracker`, `request` and `process` plugins. Not read: any
implementation, the UI, or enum values (e.g. what `TimeReportDayType`
contains is **unverified**). **Licence note:** EPL-2.0 is weak copyleft; this
plan takes *ideas only*, and no code should be copied from it.

## 13. What it is

Not a project-management tool but a **business-application framework** whose
project tracker is one plugin among CRM, HR, recruiting, chat, documents,
calendar, inventory and quality management. Architecture: 30+ microservices
(account, transactor, workspace, fulltext, collaborator, export, print, sign,
process …) over CockroachDB, Elasticsearch, MinIO, Redpanda (Kafka) and Redis,
with a WebSocket transactor pushing every mutation to clients and a Yjs CRDT
service for co-editing.

The contrast is the main finding: Huly needs five infrastructure systems and
30 services; this service is one binary over Postgres. We should take its
*product* ideas, not its *shape*.

## 14. Where we already match or lead

- Issue-tracker core (statuses, milestones, sprints, time reports, templates
  planned in LT-2), automation (T-21 / `automations`), notifications, full-text
  search, export, audit, OpenTelemetry tracing, event outbox.
- We have no equivalent of its CRM / HR / ATS / chat / video modules, by design
  (out of scope for a PPM service).

## 15. Gaps worth considering

| # | Idea | Why it matters here | Fit | Task |
|---|---|---|---|---|
| 1 | **Approval requests** with a status and per-decision comments, on any subject | Our `reviews` cover idea / proposal / plan; gate reviews cover stages. Budget changes, scenario commits and task moves have no ask-and-decide step | High; small extension of `reviews` | PL-1 |
| 2 | **Multi-step processes** — states, human steps (to-do, approve), an execution record with a per-step log | Our automations are one trigger → N actions; real governance is "finance approves, then the director". Huly models `Process`, `Execution`, `ExecutionLog`, `ApproveRequest` | Medium-high; decide scope | PL-2 |
| 3 | **Live updates** — pushed changes to open boards | A worker and a PM looking at the same board see stale data. Huly's WebSocket transactor is its core. SSE fits our stack | Medium-high | PL-3 |
| 4 | **Personal planner** — schedule a to-do into a calendar slot | Extends LT-3 "My work" from *what is due* to *when I'll do it*; Huly's `time` plugin | Medium; worker audience | PL-4 |
| 5 | **Notification preferences** per event type and channel, one inbox | Our notifications are unconditional. With LT-4, RM-3, LT-6 and RM-11 adding sources and channels, users need control or it becomes noise | High (enabler) | PL-5 |
| 6 | **Controlled documents** — versioned, reviewed, approved, effective-dated | A QMS capability; overlaps LT-8 (wiki) and our compliance / regulator surfaces | Medium; decide with LT-8 | PL-6 |
| 7 | **Guest access** — an external collaborator scoped to named subjects | We have `reviewer_scope: external` on reviews only. Vendors and clients need a scoped view of a plan | Medium | PL-7 |
| 8 | **Document text extraction for search** (their `rekoni`) | Makes LT-5's attachments findable; raises a masking question | Low-medium; after LT-5 | PL-8 |
| 9 | **Issue templates with child issues** | Direct evidence for LT-2's design: a template carries its children | High; amend LT-2 | amend LT-2 |
| 10 | **Printable output (PDF) and signed documents** | `board/pack` and regulator extracts are JSON/CSV today; boards want a document. Huly has `print` and `sign` services | Medium | PL-10 |
| 11 | **Components** (grouping tasks inside a plan) | Huly *and* Redmine (issue categories) both have it: two independent sources agreeing. Cheap, filterable, reportable | Medium-high | PL-11 |
| 12 | **Time-report day types** | Huly's `TimeReportDayType` suggests overtime / holiday handling; our `non_working` periods cover only the calendar side. Values unverified | Low; verify first | amend LT-11 |

## 16. Deliberately not taken (Huly)

- **The microservice shape** — 30 services and five datastores for a registry
  of plans is the wrong trade; we keep one service and Postgres.
- **CRDT co-editing (Yjs collaborator).** Only worth it if LT-8's wiki is built
  *and* concurrent editing is a real complaint. Optimistic concurrency with
  ETags (already used by the dashboard) is the cheap answer first.
- **CRM, HR, recruiting, inventory, billing, video meetings and recording.**
  Other products.
- **An AI assistant inside the service.** Same refusal as T-28: it would be the
  one output that cannot disclose its inputs.
- **Desktop app, presence indicators.** Not a stated need.

## 17. Sequencing across all three sources

1. **Enabler first: PL-5** (notification preferences). LT-4, RM-3, LT-6 and
   RM-11 all add notification volume; shipping them without control creates
   the noise the worker audience is trying to escape.
2. Worker path as before (LT-3, LT-12, LT-9, RM-7), with **PL-4** (planner)
   and **PL-3** (live board) as its second wave.
3. PMO governance: RM-1, RM-2, **PL-1** (approvals), then **PL-2** only after
   PL-1 shows whether one-step approvals suffice.
4. **PL-11** (components) with RM-1 (custom fields): both are "name your own
   grouping".
5. Decide together: LT-8, PL-6, PL-8 (documents family).

## 18. Open questions for the owner (Huly)

- Approvals: is "ask and decide" on arbitrary subjects wanted, or only on the
  existing review subjects?
- Do boards need a PDF, and does it need a signature (who signs: the system or
  a person)?
- External guests: vendors only, or also clients? It changes the disclosure
  model.
- Is live update wanted for every board, or only plans a user has open?


---

# Research: scheduling and cost-control canon (CPM, EVM, DCMA, leveling, schedule risk)

Source of this section: the eight topics the owner listed on 2026-10-03 —
critical path and float, resource leveling, baselines and constraints, earned
value, portfolio resource management, Monte Carlo risk analysis, DCMA project
audits, and EVM cost and performance analytics. These are *disciplines with
published standards*, not a product, so the comparison below is to the standards
(PMI Practice Standard for Scheduling; AACE RP 49R-06 on identifying the critical
path; the DCMA 14-Point Assessment; ANSI/EIA-748 and ISO 21508 for earned value;
Lipke's earned schedule), and to **this repository's code, which I read**.

**Evidence base, stated plainly.** Read: `src/visibility.rs` (`critical_path`,
`violations`, `ScheduleEdge`), `src/tpc.rs`, the T-28b entry and the T-23 SPI/CPI
note in `spec/13-tasks.md`, the `allocations` entity, `summed_percent`,
`src/tba.rs` Monte-Carlo (TBA-11) and the `non_working_periods` table. **Not
fetched live:** the standards themselves; the DCMA thresholds below are the
commonly published ones and **must be checked against the current DCMA guide
before they are pinned in a test** (SC-9 says so in its acceptance).

## 19. What we already have, and the gaps it leaves

| Topic | Already here | Gap (verified in code) |
|---|---|---|
| Critical path and float | `critical_path()`: the longest-duration chain over finish-start edges; `violations()`: a successor that starts before predecessor end + lag | **No float at all** (no early/late dates, no total or free float). **Relationships are finish-to-start only** (`ScheduleEdge` has no type). **`critical_path()` ignores `lag_days`** — the path length counts item durations only, so a lagged chain can be reported shorter than it is (finding F-6). It returns **one** chain: ties and near-critical paths are invisible. Everything is **calendar days**, so a weekend counts as work |
| Resource leveling | `allocations` (percent over a window); `summed_percent` flags over-allocation; `/capacity` and `/capacity/utilization`; skill-gap | Detects over-allocation, **never proposes a fix**; nothing knows how much a task can slip without moving the finish (needs float) |
| Baselines and constraints | **Budget** baselines, append-only and versioned (T-28b) | **No schedule baseline** (planned start/finish per task), so no start/finish variance, slip count or baseline execution index. **No constraint types**; a date is a date. No distinction between a soft deadline and a hard "must start on" |
| Earned value | Budget baseline, EAC/ETC (T-28b), DIPP/EMV/CEC (`tpc.rs`); SPI/CPI deliberately `null` + `no_baseline`, and after T-28b "a baseline exists, but nothing scores it" | **No earned value**: no percent-complete method, so no EV, hence no SV/CV, SPI/CPI, TCPI or S-curve. Tasks carry `points` and `status`, so a physical percent complete is *derivable*, but nothing derives it |
| Portfolio resource management | Capacity rollup, utilization, skill-gap, scenarios, planner, timesheets (actual hours) | No **demand vs capacity by role over time**; no generic (role-only) placeholder demand; no planned-vs-actual hours report; nothing for a resource manager to *act on* |
| Monte Carlo | TBA-11: forecast "how long for N items / how many in N periods" from the **throughput** distribution, seeded and disclosed | That is *delivery* forecasting. **Schedule-risk** simulation (three-point durations, risk events from the register, P50/P80/P90 finish, criticality index, sensitivity) does not exist, and **needs CPM first** |
| DCMA audit | Gate reviews, compliance and auditor surfaces | No schedule-quality assessment; most of the 14 checks need float, constraints and a baseline that do not exist yet |
| EVM analytics | `budget_baselines`, per-currency variance, EAC/ETC/portfolio overrun forecast (T-28b) | No TCPI, VAC, EAC variants chosen by CPI/SPI, no S-curve series, no EV roll-up across plans |

**The main finding.** Seven of the eight topics are one dependency chain: a
*real* CPM engine (SC-1) → calendars and constraints (SC-2) → schedule baselines
(SC-3) → earned value (SC-4). Leveling, schedule risk and the DCMA audit are
all *consumers* of float. Building any of them first would mean inventing float
inside that feature. The cheapest honest order is therefore the engine first.

## 20. Gaps worth considering

| # | Idea | Why it matters here | Fit | Task |
|---|---|---|---|---|
| 1 | **CPM engine**: forward and backward pass, total and free float, all four relationship types (FS, SS, FF, SF) with lag and lead, multiple and near-critical paths | Every other item here is a consumer. Also fixes the lag-ignored defect (F-6) | High; the enabler | SC-1 |
| 2 | **Working calendars and constraints** (ASAP, ALAP, SNET, SNLT, FNET, FNLT, MSO, MFO) plus a separate soft **deadline** | A weekend counted as work and a date that cannot be pinned make every float wrong. A hard constraint is the thing DCMA audits for | High | SC-2 |
| 3 | **Schedule baselines**, append-only and versioned, with start/finish/duration variance and slip | The schedule twin of T-28b; the denominator for BEI, SV and "how late are we" | High | SC-3 |
| 4 | **Earned value**: a percent-complete method per task, then PV, EV, AC, SV, CV, SPI, CPI and earned-schedule SPI(t) | Closes the T-23 gap: SPI/CPI stop being permanently `no_baseline` | High; **per-currency** | SC-4 |
| 5 | **EVM analytics**: EAC variants, TCPI, VAC, S-curve series, portfolio roll-up | What a sponsor reads; every formula must name itself | Medium-high | SC-5 |
| 6 | **Resource leveling** as an *advisory proposal*, applied only on confirm | Today we say "over-allocated" and stop. Proposal = delay within float, never past a hard constraint | Medium | SC-6 |
| 7 | **Portfolio resource management**: demand vs capacity by role and period, generic placeholders, planned vs actual hours | The resource manager's real question: "can we staff Q1?" | Medium-high | SC-7 |
| 8 | **Schedule-risk Monte Carlo**: three-point durations, register risks as events, P50/P80/P90 finish, criticality index | The honest answer to "when will it finish" is a distribution, and we already do this for throughput | Medium; after SC-1/2 | SC-8 |
| 9 | **DCMA-style 14-point schedule health** with offender lists | A checklist people already know; each check is a pure function. Named a *self-check*, not a compliance claim | Medium-high | SC-9 |
| 10 | **Worker view of float**: "this task has 3 days of float" / "on the critical path" in *My work* | Half our users are workers; the one schedule fact that changes their day | High; small | SC-10 |
| 11 | **Front-end**: schedule-health scorecard, float column in the Lily Gantt table, S-curve, leveling diff, resource heatmap, risk histogram | None of it is usable as JSON alone | Medium | SC-11 |

## 21. Deliberately not taken (scheduling and cost canon)

- **A claim of ANSI/EIA-748 compliance (the 32 EVMS guidelines).** That is an
  accredited-system certification about process and people, not a feature. We
  compute EVM figures with disclosed formulas; we do not say the system is
  "EIA-748 compliant", and neither do the DCMA-style checks say "DCMA compliant".
- **Automatic resource-constrained scheduling (an RCPSP optimiser that rewrites
  dates).** Leveling is advisory and deterministic; it never silently moves a
  critical task. An optimiser is a different product and an unexplainable one.
- **Silent auto-reschedule when a predecessor slips.** The engine *reports*
  float and violations; a person decides. (Same stance as the automation work.)
- **Critical-chain buffers (CCPM)** and **crashing / fast-tracking advice.**
  Related, but the Theory-of-Constraints analysis already in `tba.rs` is the
  flow-side answer; a schedule-side one needs a real user asking.
- **Correlated risk (copula) models and cost-risk correlation matrices.**
  SC-8 states "inputs are independent" as a limitation rather than fake the
  rigour.
- **MS Project `.mpp` / Primavera `.xer` round trips.** T-28n covers
  interchange formats; binary formats are out.
- **Percent complete as a free-text vibe.** If a manual method is allowed it
  carries a note and is flagged `manual` everywhere EV is shown.

## 22. Sequencing

1. **SC-1 first** (CPM engine), including the lag fix. Nothing below is honest
   without float.
2. **SC-2** (calendars and constraints), then **SC-3** (schedule baselines).
3. **SC-9** (14-point health) can start as soon as SC-1 and SC-2 land: ten of
   the fourteen checks need only them; CPLI and BEI wait for SC-3.
4. **SC-4** (earned value) needs SC-3 and the existing budget baseline; **SC-5**
   follows it.
5. **SC-6** (leveling) and **SC-8** (schedule risk) need SC-1 and SC-2.
6. **SC-7** (portfolio resource management) is independent of the engine and can
   run in parallel; it reuses allocations, timesheets and scenarios.
7. **SC-10** (float for workers) the moment SC-1 lands; **SC-11** after each.

## 23. Findings (scheduling code, 2026-10-03)

- **F-6: `critical_path()` ignores lag.** `src/visibility.rs` sums item
  durations along edges and never adds `ScheduleEdge.lag_days`, while
  `violations()` does honour it. A chain with a 10-day lag can be reported as
  *shorter* than an unlagged one, so the wrong chain can be called critical.
  Fixed by SC-1; until then the "critical path" label on the Gantt is
  duration-only and should be read that way.
- **F-7: every duration is calendar days.** `(end - start).num_days() + 1` and
  `Days::new(lag)`: a Friday-to-Monday task is four days long. The doctrine for
  *dates* is zone-free, which is right; the missing piece is a **working
  calendar** (SC-2), not a time zone.
- **F-8: SPI and CPI cannot exist yet, and the message is accurate.** After
  T-28b the service says "a baseline exists, but nothing scores it". That stays
  true until SC-4 gives a task a percent complete.

## 24. Open questions for the owner (scheduling and cost canon)

- **Working time:** one working calendar per plan, or per resource (a part-time
  contractor, a regional holiday)? Per plan is far simpler and covers most uses.
- **Duration unit:** keep calendar days and add a calendar, or move durations to
  working days throughout? The second is more correct and touches every date.
- **Percent complete:** which method is the default — 0/100 (done or not),
  50/50, or points-based physical percent (derivable today)? And may a manager
  type a percent by hand (flagged `manual`)?
- **Hard constraints:** who may set one? They are the commonest source of a
  schedule that "works" and lies. A role gate (and an audit) is proposed.
- **DCMA relevance:** are there defence or government customers, or should the
  checks be presented as generic "schedule quality checks" with the 14-point
  thresholds as one preset?
- **Three-point estimates:** are workers willing to give optimistic / likely /
  pessimistic durations? If not, SC-8 can start from a single estimate plus a
  per-plan uncertainty band, and says that is what it did.
- **Money for EVM:** actual cost from timesheets (hours × rate) or from
  budget-line actuals entered by hand? They disagree, and EV needs one.


---

# Research: ProjectLibre (open-source desktop alternative to Microsoft Project)

Source: <https://sourceforge.net/p/projectlibre/code/ci/master/tree/> (branch
`master`, latest commit "ProjectLibre 1.9.8", 2025-04-30; a `dev` branch and old
tags exist). Plus the project's public description: "Gantt charts, network
diagrams, WBS/RBS charts, earned-value costing, resource histograms, multi-project
resource pools, Microsoft Project compatibility", positioned for **an individual
project manager managing one project locally on a desktop**.

**Evidence base, stated plainly.** Read: the directory trees of `projectlibre_core`
(`pm/{assignment,availability,calendar,costing,criticalpath,dependency,key,
resource,scheduling,snapshot,task,time}`, plus `algorithm`, `job`, `undo`,
`command`, `transaction`, `timescale`, `scripting` …), `projectlibre_exchange`
and `projectlibre_reports`, and the **full text of nine small constants files**
(`ConstraintType`, `DependencyType`, `EarnedValueMethodType`, `SchedulingType`,
`BookingType`, `RequestDemandType`, `ProjectStatus`, `ProjectType`,
`DateChangeStatus`). **Not read:** the scheduling algorithm itself
(`SchedulingAlgorithm`, `CriticalPath`), the costing calculators, the importer, or
any UI. So the *vocabulary and the data model's shape* below are evidenced; *how*
anything behaves is not, and the contour shapes and the meaning of
`PertWeights` are **inferred from class names** and flagged where used.

**Licence note — a hard line.** The files carry the **Common Public Attribution
License 1.0** (Mozilla-style with an *attribution-logo condition on every user
interface screen* and a **network-use clause** that treats a hosted service as
distribution). This service is hosted. **Ideas only; no code, no text, no
resources are to be copied.** Anything borrowed is a concept re-derived from
public standards (the MS Project / MPX vocabulary is an industry convention, not
ProjectLibre's).

## 25. Where we already match, and where they are ahead

| Topic | ProjectLibre | Here |
|---|---|---|
| Constraint types | Exactly the eight MS Project ones (ASAP, ALAP, MSO, MFO, SNET, SNLT, FNET, FNLT) **plus a hammock type**; ids "same as mpx" | SC-2 names the same eight; the **hammock** is new (PJ-7) |
| Dependency types | FF, FS, SF, SS with **FS the default**; ids FF=0, FS=1, SF=2, SS=3 | SC-1 has the same four; the **id order differs from the order we list**, which matters for import (PJ-10) |
| Baselines | A `snapshot` package (`Snapshottable`, `BaselineScheduleFields`, `TaskSnapshot`) | SC-3 is the same idea, append-only |
| Earned value | Two methods only: **percent complete** and **physical percent complete** | SC-4 has more (0/100, 50/50, points, manual). Their split is useful evidence that *schedule-percent and physical-percent are different things* and both are needed |
| Calendars | `WorkCalendar`, `WorkWeek`, `WorkDay`, `WorkRange`, `CalendarCatalog`, `HasCalendar` on **tasks and resources** | SC-2 plans one calendar per plan; ProjectLibre shows the per-resource and per-task case is wanted in practice (see Q in §30) |
| Critical path | `criticalpath` package with `ScheduleWindow`, `PredecessorTaskList/Tree`, `DateChangeStatus` | SC-1; their **change status** (which dates a recalculation moved) is a good output we had not planned (PJ-8) |
| Resources | `Resource`, `EnterpriseResource`, `EnterpriseResourcePool`, `ResourceType`, **`BookingType`: proposed / committed** | We have allocations only; **proposed vs committed** is new and valuable (PJ-4) |
| Costing | `CostRateTable` / `CostRateIndex` (several **effective-dated rate tables** per resource), `Accrual`, `ExpenseType`, `EarnedValueCalculator` | **We have no rates at all**, so "actual cost = hours × rate" (SC-4) has nothing to multiply by (PJ-2) |
| Time-phased data | `assignment/contour` (standard and personal contours), `TimeDistributedDataConsolidator` | Nothing time-phased below the plan; resource histograms and S-curves need it (PJ-5) |
| Scheduling rules | `FixedUnits`, `FixedDuration`, `FixedWork`, `CanBeLeveled`, `Delayable` | One date model; effort-driven rules are a deliberate decision (PJ-6) |
| Jobs | `Job`, `JobQueue`, `ExtendedProgressMonitor`, **`JobCanceledException`** | `bulk_jobs` run to completion; no progress, no cancel (PJ-11) |
| Interchange | `projectlibre_exchange`: `MicrosoftImporter`, `ImportedCalendarService` | T-28n is open and unscoped; this fixes its target (PJ-10) |
| Sub-projects | `SubProj`, `SubprojectHandler`, `ExternalTaskManager`, `TaskLinkReference` | We link *plans* (LT-8) and have plan-level dependencies, but no task-to-task edge across plans (PJ-13) |
| Statuses | PLANNING, PENDING_APPROVAL, ACTIVE, COMPLETED, CANCELLED, ON_HOLD | Our lifecycle and approvals (PL-1) cover these; **verify `on_hold` exists** before assuming |
| Scripting, undo, command, transaction | A scripting engine; a command/undo/transaction stack | Not taken (scripting); the *what-if then commit* idea is taken (PJ-9) |

**The main finding is about us, not them.** Reading how ProjectLibre models a
task (`Task` with dates, duration, constraint, calendar, assignments, baseline)
exposed that **our schedule network is plan-level, not task-level**:
`ScheduleItem` is built from a *plan's* `start_date` / `target_date`
(`src/controllers/visibility.rs`, `schedule_item`), `plan_dependencies` joins
plans, and a **task has only `due_date`** — no start, no duration, no estimate.
Every SC task I wrote last turn says "per task". That is not buildable as written.
It is recorded as finding F-9 and as **PJ-1, which must be decided first**.

## 26. Gaps worth considering

| # | Idea | Why it matters here | Fit | Task |
|---|---|---|---|---|
| 1 | **Task-level schedule fields** (start, working duration, constraint) and a decision on the schedule's unit | Without it SC-1…SC-4 can only be plan-level. Tasks (LT-10 relations, RM-5 subtasks, sprints) are where workers live | High; **decide first** | PJ-1 |
| 2 | **Effective-dated cost rates**, fixed-cost expenses and accrual | EVM's AC and any cost report need rates; a rate change must never rewrite past actuals | High; **sensitive data** | PJ-2 |
| 3 | **Resource availability over time** (part-time, leave, ramp-up) | Capacity is a flat percent today; real capacity changes by date | Medium-high | PJ-3 |
| 4 | **Proposed vs committed bookings** (and request/demand) | The portfolio question "what is firm, what is hoped for" — and what happens when we commit | High | PJ-4 |
| 5 | **Time-phased assignments with work contours** and a consolidator | The data behind resource histograms, S-curves and leveling; spreads work across periods without rounding drift | Medium-high | PJ-5 |
| 6 | **Scheduling rules**: fixed units / fixed duration / fixed work | Changing one of duration, units or work should recompute the others by a stated rule | Low-medium; **decide first** | PJ-6 |
| 7 | **Hammock (span-by-link) activities** | A "from the first design task to the last test" bar that follows its links | Low | PJ-7 |
| 8 | **Recalculation change set**: which dates moved, by how much | Feeds watchers, audit and "why did my date move?" | Medium; small | PJ-8 |
| 9 | **What-if sandbox**: apply edits to a copy, see the impact, commit or discard | The safe half of leveling and of any bulk reschedule | Medium-high | PJ-9 |
| 10 | **MS Project interchange (MSPDI XML)** as the T-28n codec | The one format most migrating users have; the id tables above are the traps | High | PJ-10 |
| 11 | **Cancellable jobs with progress** | Long exports, imports and Monte-Carlo runs need a stop button and a bar | Medium; small | PJ-11 |
| 12 | **Network (PERT) diagram, WBS tree with generated WBS codes, RBS** | The views schedulers ask for by name; WBS codes are derived display, never identifiers | Medium | PJ-12 |
| 13 | **Cross-plan task links** (sub-project style) | A vendor plan's task gates ours; the edge must not leak a hidden plan | Medium | PJ-13 |

## 27. Deliberately not taken (ProjectLibre)

- **Any code, text or assets.** CPAL 1.0 with an on-screen logo condition and a
  network-use clause; this is a hosted service. Concepts only.
- **An embedded scripting engine** (`script`, `scripting` packages). Running
  user scripts in a multi-tenant service is a security liability with no stated
  need; saved reports, automations and webhooks are the extension surface.
- **The desktop, single-user, local-file model** (workspace, session, server
  sync). Our value *is* the multi-user, access-controlled service.
- **A Swing-style rich desktop UI and its undo stack.** We take the *what-if*
  idea (PJ-9), not a global undo; self-service undo stays a per-feature POST.
- **Hard-wiring the MS Project numeric ids into our model.** They are kept at the
  edge, in the import codec (PJ-10); our enums stay readable strings.

## 28. Sequencing with the SC items

1. **PJ-1 first** — before SC-1 — because it decides what SC-1's nodes *are*.
2. **PJ-8** is a free by-product of SC-1; build it in the same change.
3. **PJ-2, PJ-3, PJ-4, PJ-5** feed SC-4 (AC), SC-6 (leveling) and SC-7; PJ-4 and
   PJ-3 are small and can run before SC-1.
4. **PJ-9** underpins SC-6's "apply"; **PJ-11** underpins SC-8's long runs.
5. **PJ-10** is T-28n's real scope and can start once PJ-1 fixes the task shape.
6. **PJ-12, PJ-13** after the engine; **PJ-6, PJ-7** only if asked.

## 29. Findings (from reading ProjectLibre against our code)

- **F-9: our CPM/schedule network is plan-level.** See §25. Tasks have no start,
  duration or estimate; the SC items assumed they did. Decision needed (PJ-1).
- **F-10: no rates anywhere.** There is no rate, rate table or cost-per-hour in
  the service, so the SC-4 note "AC from timesheets × rate" cannot be done until
  PJ-2 exists. Budget-line actuals (entered by hand) are the only AC source today.
- **F-11: import ids are a trap.** MS Project's dependency ids order FF, FS, SF,
  SS (0–3) — not the order a reader would guess, and FS is 1, not 0. The
  constraint ids run ASAP 0 … FNLT 7 with a hammock at 100. A codec that guessed
  would swap relationship types silently; PJ-10 pins both tables with tests.

## 30. Open questions for the owner (ProjectLibre)

- **Schedule unit (PJ-1):** is the schedule network of *plans* (as now), of
  *tasks*, or both in one network? Recommendation: both, with a task's parent
  plan as its container, so a worker's task and a PMO's plan share float.
- **Per-resource calendars:** ProjectLibre attaches calendars to tasks and
  resources. Is a per-person calendar (part-time, regional holidays) needed, or
  is one calendar per plan enough?
- **Cost rates are salary-adjacent.** Who may see and set them? Proposal: a
  finance attribute only; masked everywhere else, and never shown to a worker.
- **MS Project:** do migrating users bring `.xml` (MSPDI) files? If most bring
  `.mpp`, we would need to ask them to export XML — and say so in the UI.
- **Fixed-work scheduling (PJ-6):** wanted, or is "fixed duration" (the default
  everyone assumes) enough?
- **Proposed bookings:** should a proposed booking be visible to the person it
  names, or only to resource managers until committed?

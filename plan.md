# Plan: ideas from Leantime

> **Status (2026-10-06).** This file is research and decision history, not a
> live queue: its ideas became the `LT-`, `RM-`, `PL-`, `SC-`, `PJ-` and related
> items in [tasks.md](tasks.md), all of which are ticked there (each entry says
> what is built and what is "Not built"), except the `DP-` (§32–§38, the monthly
> director pack), `EX-` (§39–§45, a delivery organization under escalation),
> `HD-` (§46–§51, production readiness) and `CC-` (§52–§56, critical chain
> buffers) items, added 2026-10-09, which are open. Sections are numbered §1–§56
> across ten research passes (Leantime, Redmine, Platform Collective, scheduling
> and cost canon, ProjectLibre, owner decisions, director pack, escalation,
> production readiness, critical chain); the "Where we already match" tables are
> as of each pass's date and are not re-verified. **2026-10-21:** §64–§70 add a
> pass on new capabilities and code quality (`NC-`, `CQ-`); those items are open.

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
- Wiki: build, or link out to the organization's existing docs system?
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
organization defines its own vocabulary**: issue types, statuses, custom
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
| 10 | **Worker view of float**: "this task has 3 business days of float" / "on the critical path" in *My work* | Half our users are workers; the one schedule fact that changes their day | High; small | SC-10 |
| 11 | **Front-end**: schedule-health scorecard, float column in the Lily Gantt table, S-curve, leveling diff, resource heatmap, risk histogram | None of it is usable as JSON alone | Medium | SC-11 |

## 21. Deliberately not taken (scheduling and cost canon)

- **A claim of ANSI/EIA-748 compliance (the 32 EVMS guidelines).** That is an
  accredited-system certification about process and people, not a feature. We
  compute EVM figures with disclosed formulas; we do not say the system is
  "EIA-748 compliant", and neither do the DCMA-style checks say "DCMA compliant".
- **Automatic resource-constrained scheduling (an RCPSP optimizer that rewrites
  dates).** Leveling is advisory and deterministic; it never silently moves a
  critical task. An optimizer is a different product and an unexplainable one.
- **Silent auto-reschedule when a predecessor slips.** The engine *reports*
  float and violations; a person decides. (Same stance as the automation work.)
- **Critical-chain buffers (CCPM)** and **crashing / fast-tracking advice.**
  Related, but the Theory-of-Constraints analysis already in `tba.rs` is the
  flow-side answer; a schedule-side one needs a real user asking.
  *(Superseded 2026-10-09 for CCPM: the owner asked, and critical-chain
  buffers are taken in §52–§56. Crashing and fast-tracking advice stays not
  taken.)*
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
  `violations()` does honour it. A chain with a 10-calendar-day lag can be reported as
  *shorter* than an unlagged one, so the wrong chain can be called critical.
  Fixed by SC-1; until then the "critical path" label on the Gantt is
  duration-only and should be read that way.
- **F-7: every duration is calendar days.** `(end - start).num_days() + 1` and
  `Days::new(lag)`: a Friday-to-Monday task is four calendar days long. The doctrine for
  *dates* is zone-free, which is right; the missing piece is a **working
  calendar** (SC-2), not a time zone.
- **F-8: SPI and CPI cannot exist yet, and the message is accurate.** After
  T-28b the service says "a baseline exists, but nothing scores it". That stays
  true until SC-4 gives a task a percent complete.

## 24. Open questions for the owner (scheduling and cost canon)

- **Working time:** one working calendar per plan, or per resource (a part-time
  contractor, a regional holiday)? Per plan is far simpler and covers most uses.
- **Duration unit:** keep calendar days and add a calendar, or move durations to
  business days throughout? The second is more correct and touches every date.
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

## 31. Owner decisions (2026-10-03)

Answers to §24 and §30, recorded so they are not re-asked:

- **Schedule unit:** **both** — one network of plans *and* tasks (PJ-1).
- **Calendars:** **per person and per plan.** A plan has a working calendar; a
  person has their own (part-time, regional holidays, leave). A task is scheduled
  on its plan's calendar, and an assignment's *work* is scheduled on the
  intersection with the assignee's calendar (SC-2, PJ-3).
- **Cost rates:** governed by **ABAC**, not a hard-coded role: a `cost_rate`
  resource with its own actions in the policy, deny by default (PJ-2).
- **MS Project:** **XML (MSPDI) only**; binary `.mpp` is not read and the UI says
  so (PJ-10).
- **Fixed-work scheduling:** **yes, in addition to fixed duration** — and fixed
  units too, so all three rules (PJ-6).
- **Proposed bookings:** **visible to the person they name**, as well as to
  resource managers (PJ-4).
- **Still open:** default percent-complete method, who may set a hard constraint
  (now an ABAC action too), DCMA wording, three-point estimates, and the money
  source for EVM (§24).

---

# Research: the monthly director pack

Source of this section: the owner's brief of 2026-10-09, "What directors should
see each month" — **one pack, the same for the organization's executive, its
board, an escalation board, and the chief executives of partner
organizations**, with eight views: portfolio map, exceptions (the 48-hour red
rule), gate decisions, capacity heatmap, money, buffer fever chart, earned
value, and benefits.

**Evidence base, stated plainly.** Read: `src/controllers/oversight.rs`
(`board_pack_body`, `/board/trends`, `/board/snapshots`), `insights::rag_by_pid`
and `visibility::rag`, `governance.rs` (`GATES`, `DECISIONS`, gate reviews),
`lifecycle.rs` (`PHASES`), `strategy.rs` (`BENEFIT_STATUSES`, objectives and
`objective_links.weight`), the `budget_lines`, `benefits`, `allocations`,
`earned_value_snapshots` and `insight_snapshots` entities, `portfolio_resources`
(SC-7), `earned_value.rs` (thresholds, EAC variants), `report_schedules.rs`
(monthly cadence) and `spec/locales.md`. **Not read:** any published
delivery-confidence guidance, language regulation or public funding rules; what
this section says about them is general knowledge and is marked as an owner
question where it matters.

## 32. Assessment of the brief

The brief is sound and close to good public-sector portfolio practice
(**exception-led**, and **one pack for every audience** so no board sees a
friendlier story), and it joins delivery, money, people and benefits — the four
things usually reported by four teams in four spreadsheets. Five things in it
need sharpening before it can be built honestly:

1. **"The same pack" must mean the same frozen data, not the same live screen.**
   Partner chief executives and an escalation board are outside the
   organization; today the board pack is an admin-only privileged read (RM-8b)
   and cost rates are finance-only (PJ-2). The answer is an **issued pack**: one
   snapshot per month, cut off at a stated time, with an identifier and version,
   distributed to named audiences. Everyone reads the identical object; nobody
   needs access to the live estate. A correction after issue is a **reissue
   (v2) with a change note**, never a silent edit. Trend arrows compare with
   **the previous issued pack**, never a re-derivation of last month (the
   `/board/trends` doctrine).
2. **"Delivery confidence" is a judgement, and the code only has a derived
   three-colour RAG.** The five-point scale the brief implies (green,
   amber-green, amber, amber-red, red) is an assessment made by the senior
   responsible owner or an independent assurance reviewer, with a rationale and
   a date. It should be **stored as such, beside the derived RAG, and never
   blended with it**; when the two disagree by two or more steps the pack says
   so. A programme with no current assessment shows **"not assessed"**, never
   green.
3. **The 48-hour red rule needs a clock and a definition of done.** Proposed:
   the clock starts when a programme's confidence is *set* to amber-red or red
   (or worsens within them); it stops when an exception record holds a **cause,
   the decision needed, a named owner and a decide-by date**. Missing that in 48
   elapsed hours is a **breach, shown in the pack** and notified. Exceptions
   then stay open until the decision is recorded and closed.
4. **The gate vocabulary does not match.** Stored decisions are `approved`,
   `approved_with_conditions`, `hold`, `rejected`. "Passed / failed / deferred"
   map to approved (either form) / rejected / *a gate that was due and not
   held or was held over*, which needs a **planned gate date** we do not store.
   "Paused" and "stopped" are not lifecycle states at all (`PHASES` runs idea →
   closed); they need a programme status with a reason and the decision that set
   it.
5. **"Over threshold" for earned value is ambiguous.** The service already has
   CPI/SPI *alert* thresholds (`PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS`).
   The brief most likely means a **size** threshold (programmes whose budget at
   completion is above a set amount are EVM-reported). Recommended: an inclusion
   threshold on BAC, set by configuration and printed in the pack, and a
   programme below it shows "below EVM threshold", not a blank.

## 33. View by view: what exists, what is missing

| View | Already here | Missing |
|---|---|---|
| Portfolio map | Plans with `kind`, `stage` (last approved gate) and `phase`; objectives with weighted `objective_links`; derived RAG | **Strategic aims** as the top tier of objectives (or a flag on an objective); a **primary aim** per programme so a programme linked to two aims appears once and is cross-referenced, never double-counted; **stored delivery confidence** (§32.2); **trend arrows** from the previous issued pack |
| Exceptions | Risks and the risk-raise action, notifications and sinks | An **exception record** (programme, confidence change, cause, decision needed, owner, decide-by, outcome); the **48-hour clock** and breach flag; routing to a named decision forum |
| Gate decisions | `gate_reviews` (gate, decision, conditions, approver, date); stage advance on approval; tranche release on gate | **Planned gate dates** (to see *deferred*); **paused / stopped** status with reason and decision reference; a window query "decisions this month" across programmes |
| Capacity heatmap | SC-7 `/resource-plan` (demand against supply by role or skill and period, placeholders, masking of people for non-managers); availability (PJ-3) | **Skill pools** as a named taxonomy (roles and skills map to a pool); **month** grain fixed for the pack; **partner commitments** — supply (and demand) *committed by another organization*, which needs an **organization** on an allocation or a commitment record (none exists); committed-against-delivered for those commitments |
| Money | Budget lines (planned, actual, low/high, gate, release), budget baselines, forecast (`/financials/forecast`), variance per currency | A **funding source** on each budget line (reference data: e.g. a government allocation, a named investment fund, core budget, a partner contribution) and **capital / revenue**; a configurable **financial year** (e.g. April to March); per programme × source: budget, spend to date, full-year forecast, variance |
| Buffer fever chart | CPM with critical chains (SC-1), leveling (SC-6), schedule baselines (SC-3), schedule risk (SC-8) | **A project buffer** per programme and the two fever-chart measures: **chain completed %** and **buffer consumed %**, with zone boundaries — critical-chain buffers were *not taken* in §21 for want of a real user; this brief is that user (§34) |
| Earned value | Per plan EV, CPI, SPI, EAC variants, series, rollup, snapshots, thresholds | A **portfolio** EVM view filtered by the BAC inclusion threshold; the **AC source** stated in the pack (budget-line actuals or recorded labour — still open, §24) |
| Benefits | Benefits with category, target, realized, expected date, status (`planned`, `on_track`, `realized`, `missed`); realized events in the board pack | **Validated** as distinct from *claimed*: amount, validator (not the programme's own team), evidence, date; **per year** (recurring, annualized) versus one-off; **cash-releasing / non-cash-releasing / societal** classification; **non-financial benefits** measured through a linked KPI (KO-1) |

## 34. Reversing a "not taken": the buffer fever chart

§21 declined critical-chain buffers "until a real user asks". This brief asks,
so the item is taken — **narrowly**. Full CCPM (cut estimates by half, resource
buffers, feeding buffers, pacing by buffer only) is a method change for every
team; the fever chart needs much less:

- a **project buffer in business days** declared at baseline for each programme
  (explicitly, or as a stated rule such as 50% of the critical chain, or the
  square root of the sum of squared task contingencies — the rule is printed);
- **chain completed %** = the share of the baselined critical chain's working
  duration whose tasks are done (physical, not elapsed);
- **buffer consumed %** = business days between the baselined chain finish and
  the current CPM forecast finish, over the buffer (negative is "recovered" and
  shown as 0 with the true figure beside it; over 100% is off the chart and
  plotted at the edge with the number);
- **zones** (green, amber, red) bounded by two sloped straight lines over
  (chain completed, buffer consumed); the lines are configuration, printed in
  the pack, and not pinned here until the owner picks them (§37);
- one point per programme per pack, so the pack's **trail is last month to this
  month**, from issued packs only.

A programme with no buffer declared, no baseline or no forecast is listed as
**"no fever-chart point"** with the reason. The feeding-buffer and resource-
buffer parts of CCPM, and multi-project pipelining, were left out here and
**are now taken in §52–§56 (CC-n)**, which supersede this section's buffer
source.

## 35. Further views worth adding (owner to decide)

The brief is close to complete; these fill gaps a board will ask about:

- **Cover page:** as-of time, data cut-off, pack id and version, what changed
  since the last pack (new exceptions, closed exceptions, confidence moves), and
  a **data-quality panel** — programmes with no confidence assessment, stale
  assessments (older than the pack period), no baseline, budget lines with no
  funding source, benefits with no validator. Absence is listed, never shown as
  green or zero.
- **Decisions requested last month and what happened** — closes the loop on the
  exceptions view, which otherwise only shows new asks.
- **Top risks and issues above tolerance**, against the declared risk appetite
  (`/risk/heatmap` already has it).
- **Cross-programme and cross-organization dependencies at risk** (the
  dependency network already exists; a late predecessor in another organization
  is the commonest cause of red in a shared-services body).

## 36. Delivery, language and access

- **Formats:** an accessible HTML page and a PDF from the **same frozen JSON**
  (the PL-10 rule: a PDF is never more permissive than its JSON). Every chart
  has a table twin; colour is never alone (word and shape, as elsewhere).
- **Language:** an organization under a language obligation may need board
  papers in more than one language. The pack's fixed text (headings, labels,
  zone names, data-quality messages) goes through the locale catalogue, so
  every supported locale renders it; free text (causes, decisions) is shown as
  entered and its language recorded. Whether the pack is published publicly is
  an owner question (§37).
- **Access:** issuing a pack is an ABAC action (`director_pack:issue`); reading
  an issued pack is granted per audience (`director_pack:read` with an
  `audience` attribute), so a partner chief executive reads issued packs and
  nothing live. The pack contains **no person-level data**: capacity is by skill
  pool, money by programme and source, cost rates never appear.
- **Schedule:** a monthly cadence exists (`report_schedules`); the pack is
  *drafted* on schedule, *reviewed* by the portfolio office, and *issued* by a
  person — never auto-issued.

## 37. Open questions for the owner (director pack)

- **Confidence scale:** a recognized five-point delivery confidence scale, or
  the organization's own? Who sets it — the senior responsible owner, the
  portfolio office, or independent assurance? Can the owner's view be
  overridden, and is the override shown?
- **48 hours:** elapsed hours, or working hours (a red on Friday evening)? Does
  the rule apply to amber-red as well as red (the brief says both)?
- **Strategic aims:** which list — the organization's own strategy, a
  government's priorities, or both? One primary aim per programme?
- **Partner commitments:** are they people a partner lends to a programme
  (supply), work a partner must do for it (demand, e.g. local implementation),
  or both? Per partner by name, and do partners see each other's?
- **Funding sources:** the reference list, the financial-year start, and
  whether capital and revenue are reported separately in the pack.
- **EVM threshold:** the BAC above which a programme is EVM-reported; and the
  AC source (still open from §24).
- **Benefits:** who may validate, what evidence counts, and whether the
  cash-releasing / non-cash-releasing / societal classification is wanted.
- **Buffers:** declared per programme, or a portfolio rule? Fever-chart zone
  boundaries?
- **Publication:** is the pack a public board paper (possibly in more than one
  language, sensitive content redacted or held in private session), or
  internal only?
- **The four optional views in §35:** which to add.

## 38. Sequencing

1. **DP-1 (issued pack)** first: every view rides on the frozen, versioned
   snapshot and its access rules.
2. **DP-2 (delivery confidence)** and **DP-3 (exceptions and the 48-hour
   rule)** next — they are the pack's headline and the only new *process*.
3. **DP-4 (strategic aims and portfolio map)** and **DP-5 (gate decisions,
   paused and stopped)** — small schema changes on existing entities.
4. **DP-6 (funding sources)** and **DP-9 (validated benefits)** — reference
   data plus fields; can run in parallel.
5. **DP-8 (portfolio EVM)** is mostly a query over what exists.
6. **DP-7 (buffer fever chart)** needs SC-1 / SC-3, which have landed.
7. **DP-10 (skill pools and partner commitments)** needs the organization
   decision and is the largest schema change.
8. **DP-11 (rendering and languages)** and **DP-12 (schedule, review and
   issue)** last, as each view lands.

---

# Research: a delivery organization under escalation

Source of this section: the owner's private research notes (2026-10-09, not in
this repository) on a public-sector delivery organization placed under escalation
(external oversight and intervention) by its sponsoring government, whose directors had no portfolio tool
and could not see across the organization which projects were running, their
status, earned value, total project control, critical chain or resource
levelling. The notes' §"what directors should see each month" is the director
pack in §32–§38 above.

**Evidence base, stated plainly.** The notes are secondary — written from
public sources with stated judgements and estimates — and are not reproduced
here. Requirements below are **generalized** from them so they fit any
organization that delivers programmes with and for partner organizations under
a board and a sponsor. Facts about *this repository* were checked in the code.
The notes' timelines, costs and tool market survey are advice to that
organization, not requirements on us, and are not carried into the backlog.

## 39. What an organization under escalation needs a portfolio tool to make true

Read as requirements, the notes ask for seven capabilities. Each answers a
failure pattern common to large public-sector delivery bodies:

| # | Capability | Why |
|---|---|---|
| 1 | **One register; unregistered work gets no money and no people** | Money cannot be traced to outcomes ("what did this allocation buy?" takes calendar days of manual search); work starts without a decision |
| 2 | **Independent delivery confidence beside self-reported RAG, and the gap measured** | "Green reports, red delivery": milestone counts look healthy while major programmes fail |
| 3 | **Early warning: the 48-hour red rule and "late discovery" counted** | Risks and missed milestones not reported in a timely and transparent way |
| 4 | **Pause, reset, stop as real, visible controls with pre-agreed thresholds** | Sponsors increasingly ask for evidence they are used; programmes otherwise drift for years |
| 5 | **Capacity before commitment: no start without named, funded people; a WIP limit** | Over-commitment against capacity; several large replacements in one window |
| 6 | **Partner organizations inside the system**: their people, sign-offs, readiness and dates | Accountability without authority; partners moving go-live dates unilaterally |
| 7 | **Validated benefits and a measurable route to de-escalation** | Sponsors ask for programmes with *validated* benefits and sustained progress over consecutive quarters |

## 40. Where we already match

- **Register, stages, gates, tranche release:** plans with kind, stage, phase;
  gate reviews; budget lines released only when their gate is reached.
- **Proposals and intake:** `/proposals` with submit, review, approve, reject,
  promote, duplicates, and an intake forecast.
- **Capacity and levelling:** SC-7 resource plan (role or skill per period,
  placeholders), PJ-3 availability, SC-6 levelling, PJ-4 proposed bookings.
- **CPM, baselines, schedule risk, EVM, cost risk:** SC-1 to SC-9, CR-1 to CR-4.
- **Exit path from retired desktop and online scheduling tools:** PJ-10's MSPDI
  (XML) import reads what Microsoft Project exports.
- **Approvals, risk raising, KPIs, OKRs, satisfaction surveys** (the last
  usable for a partner pulse).
- **Private plans kept out of aggregates** (RM-8b), **ABAC with deny by
  default**, **audit of every change** — the trust base an issued pack needs.

## 41. Gaps worth considering (EX-n)

Beyond the director pack (DP-n), the notes imply these:

1. **Register completeness and the "no register, no money" rule (EX-1).** A
   completeness score per programme (SRO, partners, stage, baseline and
   forecast dates, budget, funding source, confidence, top risks,
   dependencies). Optionally *enforced*: a budget-line release or a firm
   allocation on a programme whose record is incomplete is refused with the
   missing fields named (a deployment switch, off by default).
2. **Delivery-model typology (EX-2).** Each programme is one of *central
   product* (the organization builds and runs it), *central platform with local
   deployment* (one contract and master schedule, partners deploy), or *local
   programme with central standards* (a partner leads). The type decides who the
   **delivery owner** is, who may move a deployment date, and whether the pack
   counts it against the organization. The pack shows the type so "not ours to
   deliver" is visible.
3. **Partner organizations as first-class records (EX-3).** An organization
   (the delivery organization, a partner body, a government sponsor, a
   supplier) can be a joint SRO, a supplier of people, a party to a milestone
   and an audience for the pack. This is the foundation for DP-10 and EX-4 to
   EX-6.
4. **Milestones "met" only when the partner signs off (EX-4).** A milestone may
   require sign-off from named organizations; it is *delivered* when the
   delivery organization says so and *met* when every required partner has
   signed. The pack reports both and the gap between them.
5. **Deployment-date change control (EX-5).** For a *central platform, local
   deployment* programme, a partner's request to move its go-live is a change
   request: the knock-on effect on other deployments (from the schedule network)
   is computed and shown, and the move needs the named approver (e.g. an
   escalation board). The cost of the knock-on is recorded against the requester.
6. **Go / no-go for safety-critical cutovers (EX-6).** A cutover decision with
   **pre-agreed stop criteria published in advance**, each criterion marked met
   or not at decision time, and **co-signatures** from the partner's accountable
   professional lead and the delivery organization. A "no-go" or "delay for
   safety" is recorded as a decision, not a slip, and the pack says so.
7. **Finish / Fix / Pause / Stop triage and stop thresholds (EX-7).** A
   portfolio triage is a dated decision set; each programme carries **explicit
   pause, reset and stop thresholds** (e.g. confidence red for two packs; a
   forecast past a date; cost risk P80 past a cap). Crossing a threshold opens
   an exception that **requires** a pause, reset, stop or recorded override. The
   count of pause/reset/stop decisions is itself evidence of grip for a sponsor.
8. **Start control: capacity check and WIP limit (EX-8).** A configurable limit
   on programmes in active delivery; a promotion from proposal, or a move into
   delivery, shows the capacity it needs per skill pool against what is free
   (SC-7) and is refused (or needs an override with reason) when the limit is
   reached or a constraint pool is over. New starts queue in order.
9. **Watermelon measures (EX-9).** Computed each pack: the gap between
   self-reported RAG and independent confidence per programme; **late
   discovery** (a programme first reported at amber-red or red with no earlier
   amber); milestones met with partner sign-off as a share; red without an
   approved recovery or stop decision.
10. **Outcome scorecard and de-escalation tracker (EX-10).** A named set of
    10–12 KPIs (KO-1) with targets, read by quarter; the tracker records the
    escalation level as set by the sponsor, its de-escalation criteria, the evidence
    linked to each, and **how many consecutive quarters** each has been met.
    The rule (e.g. "two consecutive quarters") is configuration, not code.
11. **Key-person register (EX-11).** People (or roles) who are single points of
    failure, with named successors and readiness; the contingent-workforce
    share of each skill pool. Visible to resource managers only; the pack shows
    counts, never names.
12. **Portfolio ledger: "what did this allocation buy?" (EX-12).** A funding
    allocation (DP-6's funding source plus an amount and a date) traced through
    budget lines, gates passed, milestones met and benefits validated, in one
    query, with **untraced money shown as untraced**. Finance cost codes map to
    a register id so ledger exports can be reconciled.
13. **New asks need a named trade-off (EX-13).** A proposal tagged as an
    external ask from the sponsor must name what it displaces (a programme
    slowed, paused or descoped) before it can be approved; the trade-off is
    shown in the pack.
14. **Method per programme: EVM or agile measures (EX-14).** A programme is
    flagged *EVM* (defined scope, contract-heavy, over the BAC threshold) or
    *product* (agile); product programmes report burn-up, release outcomes and
    service measures (the TBA flow metrics already exist), and the pack never
    shows an EVM blank for them — it shows the measure that applies.

## 42. Deliberately not taken (escalation)

- **A P3M3 maturity score in the tool.** It is an external assessment, not a
  metric the software can compute.
- **An AI register.** Valid governance, but a separate register product; a
  plan with a tag covers the portfolio view of AI pilots.
- **A live connector to a finance ledger.** EX-12 maps cost codes and
  reconciles exports; a live connector waits for a named ledger and an owner.
- **Scoring an organization's performance for its sponsor.** The tool reports
  facts and decisions; escalation levels are recorded as given, never computed.
- **Auto-pausing a programme.** Thresholds open an exception and require a
  person's decision (the same stance as §21: report, a person decides).

## 43. Sequencing with the DP items

1. **EX-3 (organizations)** early — DP-10, EX-4, EX-5 and EX-6 all need it.
2. **EX-1 (register completeness)** with DP-1, so the first pack has a
   data-quality panel.
3. **EX-9 (watermelon measures)** with DP-2 and DP-3: they read the same data.
4. **EX-7 (triage and thresholds)** after DP-3 (thresholds open exceptions).
5. **EX-4, EX-5, EX-6** after EX-3; **EX-2 (typology)** before EX-5 (it decides
   who may move a date).
6. **EX-8 (start control)** after DP-10 (skill pools).
7. **EX-10, EX-11, EX-12, EX-13, EX-14** independently, as asked.

## 44. Findings (from reading the notes against our code)

- **F-10: no organization concept.** Allocations, people, approvals and
  milestones have no organization; a partner's commitment cannot be stored
  today. EX-3 is the prerequisite.
- **F-11: confidence is derived only.** `rag_by_pid` colours a plan from
  dates, risks, budget overrun and dependency violations; no person's judgement
  of delivery confidence is stored. A self-reported status and an independent
  assessment both need a home (DP-2).
- **F-12: no paused or stopped state.** `PHASES` has no pause or stop and
  `DECISIONS` has `hold` but no *stop*; a stopped programme can only be
  deactivated or deleted, which loses the decision. DP-5 / EX-7.
- **F-13: benefits have no validation.** `BENEFIT_STATUSES` jumps from
  `on_track` to `realized` with no validator or evidence; sponsors ask for
  *validated* benefits. DP-9.

## 45. Open questions for the owner (escalation)

- **Who is the user?** The delivery organization's portfolio office, the
  sponsor's team, the partner organizations, or all three in one deployment? It
  decides multi-organization tenancy versus one estate with audiences.
- **Enforcement:** should EX-1 ("no register, no money") and EX-8 (WIP limit)
  *refuse* actions, or warn and record an override?
- **Typology:** the three models above, or an organization-defined list?
- **Go / no-go:** which roles must co-sign, and is the decision record shared
  with the partner organization's own governance?
- **Scorecard:** ship a seeded template of measures, or leave the set empty for
  each organization to define?
- **Key-person register:** named people (sensitive) or roles only?

---

# Research: production readiness (HD-n)

Source of this section: the owner's list of 2026-10-09 — licence file; buildable
from its own repository with automated checks; login enforcement on by default;
security headers; rate limiting; backup plan; data protection impact
assessment (DPIA).

**Evidence base, stated plainly.** Read: the repository root listing, every
`Cargo.toml` and `package.json` licence and repository field, the three
`.github/workflows` files, `rust-toolchain.toml`, `git remote -v`, the tracked
file list, `src/auth.rs` (`require_auth`, `area_allows`, `is_public_path`),
`src/app.rs` (the enforcement middleware), `config/{development,production,test}.yaml`,
`src/hooks.server.ts` in the UI, and `spec/12-compliance.md`. Not read: the
hosting environment, any deployment manifests outside this repository, or
legal advice on the licence or the DPIA.

## 46. What exists, and the gaps (verified)

| Item | Already here | Gap |
|---|---|---|
| Licence | `license` fields: both Rust crates `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`; the UI `MIT OR Apache-2.0` | **No licence file anywhere** (no `LICENSE*`, `COPYING`); **the three manifests disagree** (finding F-14); no SPDX headers; `cargo deny` licence checks exist (`deny.toml`) but govern dependencies, not us |
| Own repository, automated checks | `rust-toolchain.toml` pins 1.96.1 at the root; `api/.github/workflows/ci.yaml` and `security.yml`, `matcher/.github/workflows/security.yml` | **No workflow runs**: GitHub only reads `.github/workflows` at the repository root, so the three files in sub-directories are inert (F-15). The CI file floats `stable` against the pinned toolchain, uses the archived `actions-rs/cargo`, and has no UI job. **`repository` URLs point at an old monorepo** (`sixarm/main-x-service`), and comments still say "Main X Index repository" (F-16). **`project-portfolio-management-matcher-with-rust/target/` is committed** (822 build files, F-17). Pushes go to three forges (GitHub, Codeberg, GitLab), so checks must not depend on one forge only |
| Login enforcement | Blanket `/api/*` enforcement (PASETO and/or Keycloak, then ABAC) exists and is tested | **Off by default**: `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` unset ⇒ every request is anonymous **and every write is allowed** (`area_allows` returns true), including finance areas (F-18). A deployment that forgets one variable is wide open |
| Security headers | `nosniff`, `no-store` and attachment disposition on file downloads only | No HSTS, CSP, frame-ancestors / `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy` or COOP/CORP on API or UI responses; the UI sets no headers in `hooks.server.ts`; **CORS is enabled with the framework default in production** (F-19) |
| Rate limiting | `limit_payload` body limit (2 MB); scenario and simulation iteration caps; list paging ceiling (RM-12b) | **No request rate limiting** anywhere — not per caller, not per IP, not on token-bearing feeds, not on expensive endpoints (Monte Carlo, CPM, levelling, bulk export) |
| Backup plan | Soft delete everywhere; audit log; append-only histories; `compose.test.yaml` | **No backup or restore plan**: no RPO/RTO, no schedule, no restore test, nothing for the artifact store (attachments, exports) which lives outside the database; the production config comment says "no bulk/upload feature in this crate", which is no longer true (F-20) |
| DPIA | `spec/12-compliance.md` names GDPR / UK GDPR as engaged via people references; masking obligation; audited right-of-access export | **No DPIA**: no description of processing, necessity and proportionality, risk register for data subjects, or sign-off; personal data has grown since §12 was written (timesheets, cost rates, availability and leave, key-person register proposed in EX-11, comments, feed tokens) |

## 47. Recommendations

1. **Licence.** One licence set for the whole repository, chosen by the owner
   (§51). **All licence files live in a `LICENSE/` directory at the root**
   (owner decision, 2026-10-09), one file per licence in the expression;
   none elsewhere. Make every manifest's `license` field the same SPDX
   expression, and add a test that keeps them equal and finds a file in
   `LICENSE/` for each. (Forge licence detection usually looks for a root
   `LICENSE` *file*; with a directory the README carries the statement.)
2. **Its own repository.** Move CI to the root `.github/workflows`, one job per
   crate and one for the UI, on the pinned toolchain; fix the `repository`
   URLs; untrack `target/` and ignore it. A fresh clone must build and test
   with only the documented prerequisites, and a job proves it on every push.
   Mirrors on Codeberg and GitLab get a matching pipeline or are documented as
   mirrors only.
3. **Secure by default.** Enforcement **on unless explicitly turned off**:
   `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false` becomes the only way to
   disable it, logged at startup as a warning, and **refused when the
   environment is `production`**. With enforcement on and no verifier
   configured, the service refuses to start rather than reject every request
   silently. Development and test configs opt out explicitly.
4. **Headers.** One middleware on the API and one hook in the UI: HSTS (only
   over TLS), a strict CSP for the UI (no inline script; nonces if needed),
   `frame-ancestors 'none'`, `nosniff`, `Referrer-Policy:
   strict-origin-when-cross-origin`, a minimal `Permissions-Policy`, and
   `Cross-Origin-Opener-Policy: same-origin`. **CORS becomes an explicit
   allow-list** from configuration; none in production means same-origin only.
5. **Rate limits.** A token-bucket limit per authenticated subject (else per
   client IP, honouring a configured trusted proxy), a tighter bucket for
   expensive routes (simulations, CPM, levelling, exports, imports) and for
   authentication failures; `429` with `Retry-After` and the standard
   `RateLimit` headers; limits from configuration; the health and metrics
   routes exempt. In-process first; a shared store only when there are
   several replicas.
6. **Backups.** A written plan with RPO and RTO stated; PostgreSQL continuous
   archiving (base backup plus WAL) or managed point-in-time recovery; the
   artifact store backed up on the same schedule and **restored together**
   (a database restore with mismatched files breaks the SHA-256 checks);
   encrypted, off-site, with retention; **a scheduled restore test** that
   restores into a scratch database, runs migrations and a smoke suite, and
   reports. The plan lives in the spec; the scripts live in the repository.
7. **DPIA.** A DPIA document in the spec, kept with the code: the processing
   (who, what, why, where, how long), lawful basis per category, necessity and
   proportionality, data-subject rights and how each is met (export, masking,
   rectification, erasure limits given append-only audit), risks with
   likelihood and severity and the control that answers each, retention per
   table, international transfers, processors, and sign-off. **A test lists
   every column that can hold personal data** and fails when a migration adds
   one that the DPIA's inventory does not name.

## 48. Findings (production readiness)

- **F-14: three licence declarations, no licence text.** The crates and the UI
  declare different SPDX expressions and no file carries the terms.
- **F-15: CI never runs.** The workflows are under sub-directories that GitHub
  does not read.
- **F-16: stale repository identity.** `repository` URLs and comments name an
  earlier monorepo.
- **F-17: build output is committed.** `matcher/target/` is tracked (822
  files); a test run dirties the tree.
- **F-18: open by default.** Without one environment variable, every request
  is anonymous and allowed, including writes and finance areas.
- **F-19: CORS default in production.** Enabled with no allow-list.
- **F-20: stale production config comment.** It says there is no upload
  feature; attachments and bulk export exist.

**Status, 2026-10-09 (evening).** F-14 resolved (HD-1: one expression in all
three manifests, texts in `LICENSE/`, a test keeps them equal). F-15 and F-17
resolved in the tree: one workflow at the root, `matcher/target/` untracked and
ignored — but the workflows have **not yet run on GitHub**, so HD-2 stays open
until the first green run. F-16: the `repository` URLs name this repository.
F-18 resolved (HD-3), F-19 resolved (HD-4: no framework CORS, an explicit
allow-list), F-20 resolved (the config comment is true), F-21: the Dockerfile
and compose files use this repository's layout and a `ci/postgres-init`
exists, **with the image build verified locally** (see tasks.md HD-2). F-22
resolved (metrics need a token). F-23 resolved (HD-9). HD-6 (backups) and HD-7
(DPIA) are untouched.

## 49. Deliberately not taken (production readiness)

- **A web application firewall or DDoS protection in the service.** These
  belong at the edge; the in-process limits are a backstop, not a shield.
- **Writing the DPIA's legal conclusions.** The repository holds the template,
  the inventory and the controls; a data protection officer signs it.
- **Choosing the licence.** The owner chooses (§51).

## 50. Sequencing

1. **HD-2 (own repository and CI)** first, so every later item is checked on
   every push.
2. **HD-3 (secure by default)** next — the largest risk (F-18). It changes
   every test harness that relies on enforcement being off.
3. **HD-4 (headers)** and **HD-5 (rate limits)** together (one middleware
   layer).
4. **HD-1 (licence)** as soon as the owner chooses.
5. **HD-6 (backups)** and **HD-7 (DPIA)** in parallel; HD-7's personal-data
   inventory test lands with the code.

## 51. Open questions for the owner (production readiness)

- ~~**Licence:**~~ **Decided 2026-10-09:** any of MIT, Apache-2.0,
  GPL-2.0-only, GPL-3.0-only or BSD-3-Clause, or a custom licence by
  arrangement; texts in `LICENSE/` (HD-1, landed).
- **Canonical forge:** GitHub, Codeberg or GitLab — which runs the required
  checks?
- **Enforcement off in production:** forbidden outright, or allowed with a
  loud warning?
- **RPO / RTO targets** and where backups are stored.
- **Rate limits:** starting numbers per subject and for expensive routes.
- **DPIA owner:** who signs it.

## 52a. Second list of 2026-10-09: release hygiene (HD-8, HD-9, HD-2 to HD-4 amended)

The owner pasted a six-point list (licence; builds from a fresh clone; CI at
the repository root; secure by default; a release people can pin; documents
that match the code). It is written for a pair of products (this one and a
workforce planner); only this repository was checked. Against the code:

| Point | Verdict here |
|---|---|
| 1. Licence | **Done differently on purpose.** Texts are in `LICENSE/` (owner decision, HD-1), not `LICENSE-MIT` / `LICENSE-APACHE` at the root, so "GitHub detects a licence" is **not** a goal here; the README states the licence. The UI manifest now matches the crates. `"private": true` is still in the UI `package.json` (open question below) |
| 2. Fresh clone | **Confirmed, with a different cause.** The portfolio crates do not use `../../link` path dependencies: `integrity-mac` and `authentication-verifier` are **git dependencies pinned to the old monorepo** (`SixArm/main-x-service`, a revision). The `Dockerfile` copies monorepo paths that do not exist here (`integrity/…`, `authentication/…`, `project-portfolio-management/…`), and `compose.test.yaml` mounts `../../ci/postgres-init`, which is absent (F-21) |
| 3. CI at the root | **Confirmed** (already F-15, HD-2); also no `dependabot.yml` anywhere, and the CI uses archived `actions-rs` actions |
| 4. Secure by default | **Confirmed** (F-18, F-19); also **`/metrics.prom` is a deliberately public path** in `is_public_path`, so anything that can reach the service reads its metrics (F-22) |
| 5. Pinnable release | **Confirmed gap.** No release tags for this product (the tags in the repository belong to other services), no container image publication, no SBOM (the spec asks for a CycloneDX SBOM, §12, but none is produced), no release notes beyond `CHANGELOG.md` |
| 6. Documents match code | **Partly confirmed.** `agents/share` does not exist in this repository, and **108** markdown links point into it (more than the list's "about 20", F-23); `spec/14-implementation-status.md` is hand-written; the roadmap and status files disagree on what Jira, Asana and MSPDI import cover and no longer need the corrections the list names for controlled documents (built, PL-6). The "issues register" claim was not found in the roadmap |

Recommendations are HD-8 (releases) and HD-9 (documents), and amendments to HD-2,
HD-3 and HD-4 in tasks.md. The ZAP baseline scan the list asks for is added to
HD-4's acceptance as a CI job against the running service.

### Findings (second list)

- **F-21: the container and test database cannot be built from this
  repository.** `Dockerfile` and `compose.test.yaml` reference monorepo paths;
  two dependencies are pinned git revisions of a repository this one no longer
  lives in.
- **F-22: metrics are public.** `/metrics.prom` is exempt from login
  enforcement.
- **F-23: 108 dead documentation links** into a non-existent `agents/share`.

### Open questions (second list)

- **Dependencies:** publish `integrity-mac` and `authentication-verifier` to
  crates.io, vendor them into this repository, or keep git dependencies on
  their own repositories (and which ones)?
- **`"private": true`:** remove it, or keep it to block accidental npm
  publication?
- **Registry and signing:** GHCR (as the list suggests) or the canonical
  forge's registry; are signed images and provenance (SLSA) wanted?
- **Metrics:** behind login, or a separate internal port?
- **Dead links:** restore the shared guidance files or rewrite the links to
  the specs that now carry that content?

---

# Research: critical chain buffers (CC-n)

Source of this section: the owner's request of 2026-10-09, "critical chain
buffers". §21 declined critical-chain buffers until a real user asked, and
§34 (DP-7) took only the fever chart, keeping feeding buffers, resource
buffers and multi-project pipelining "not taken". **This request takes the
method itself**, so those exclusions are withdrawn here.

**Where it lives.** Critical chain, critical path and similar
schedule-control capabilities belong to project-portfolio-management, not to
workforce-planning-management (owner decision, 2026-10-09; `spec/02-scope.md`
§2.1). A workforce service may supply people, skills and availability as
inputs; it does not compute chains or buffers.

**Evidence base, stated plainly.** Read: `src/cpm.rs` (critical chains along
driving edges), `src/leveling.rs` (resource levelling, SC-6), schedule
baselines (SC-3), schedule risk (SC-8), three-point task estimates (CR-4's
optimistic / pessimistic fields), the portfolio resource plan (SC-7) and
availability (PJ-3). Not read: Goldratt's *Critical Chain* or any CCPM product;
the method below is the commonly published one and is marked where a choice is
the owner's.

## 52. The method, and what it needs from us

Critical chain project management (CCPM) differs from CPM in three ways:

1. **The critical chain is resource-constrained.** It is the longest chain of
   dependent tasks *after* resource contention is resolved, so it can cross
   between paths. We have both halves — CPM (SC-1) and levelling (SC-6) — but
   not the chain that results from running them together.
2. **Safety moves out of tasks into buffers.** Each task is scheduled at an
   aggressive (about 50% confidence) estimate; the removed safety is pooled
   into a **project buffer** at the end of the critical chain and **feeding
   buffers** where non-critical chains join it. **Resource buffers** are
   warnings, not time: a flag that a critical-chain resource must be ready.
3. **Execution is managed by buffer consumption**, not by task dates:
   relay-race working (start when the predecessor finishes), and a fever chart
   per project and per feeding buffer.

Multi-project CCPM adds a **drum**: the constrained resource across the
portfolio. Projects are **staggered** against the drum's capacity, with a
**capacity buffer** between them.

## 53. What exists, and the gaps (verified)

| Need | Already here | Gap |
|---|---|---|
| Resource-constrained chain | CPM chains (driving edges); levelling delays tasks to remove overloads | A **critical chain**: the longest path through the *levelled* schedule, including resource links (a task waiting for a person is linked to the task that person was on) |
| Two estimates per task | Optimistic / pessimistic (CR-4) beside the duration | An explicit **aggressive** and **safe** estimate (or a rule deriving one from the other), so safety can be pooled |
| Buffers | — | **Project, feeding and resource buffers** as schedule objects, sized by a stated rule (cut-and-paste 50%, or root sum of squares), in business days |
| Buffer management | DP-7 (project buffer and fever chart, narrow) | **Feeding-buffer** consumption, zone alerts per buffer, **buffer recovery** actions recorded |
| Multi-project | SC-7 demand and supply by role or skill | A **drum** resource, **pipelining** (staggered starts), and **capacity buffers** |

## 54. Recommendations

- **CC-1** computes the critical chain from the levelled schedule and shows it
  next to the CPM critical path; the two are never blended.
- **CC-2** stores two estimates per task, or derives the aggressive one by a
  stated rule.
- **CC-3** sizes and inserts project and feeding buffers (rule configurable,
  printed in every response), and lists resource buffers as alerts.
- **CC-4** measures each buffer's consumption against its chain's completion
  and replaces DP-7's narrow buffer with the real one; DP-7 keeps its
  fever-chart presentation.
- **CC-5** adds a drum and pipelining for a portfolio, with capacity buffers;
  proposals and starts (EX-8) can use the drum schedule.
- **CC-6** front-end: chain, buffers, fever charts, with table twins.

Doctrine, as for the rest of scheduling: **report, a person decides** — the
service never re-plans or re-sequences on its own; buffer sizes and zone
boundaries are configuration printed with every result; a plan without two
estimates or a levelled schedule gets "no critical chain" with the reason,
never a guess.

## 55. Open questions for the owner (critical chain)

- **Buffer sizing:** cut-and-paste (50% of the removed safety) or root sum of
  squares? Per plan or portfolio-wide?
- **Estimates:** will teams give an aggressive and a safe estimate, or derive
  one from the other (e.g. aggressive = 50% of safe)?
- **Multitasking rule:** should the levelled schedule forbid a person being on
  two critical-chain tasks at once (strict CCPM), or only warn?
- **Drum:** named by the portfolio office, or chosen as the most overloaded
  skill pool (DP-10)?

## 56. Sequencing (critical chain)

1. **CC-2 (estimates)** and **CC-1 (chain)** first.
2. **CC-3 (buffers)**, then **CC-4 (buffer management)**, which supersedes
   DP-7's buffer source.
3. **CC-5 (drum and pipelining)** after DP-10 (skill pools).
4. **CC-6 (front-end)** as each lands.

## 57. Issues register and change requests (IR-1..IR-6)

Requested 2026-10-10. **What exists (verified):** FR-14 specified an issues
register, but no migration, entity, controller or route exists
(`spec/14-implementation-status.md`); blockers, risks and the controlled
documents' approval flow do exist, as does the what-if scheduler and
baselines. **Recommendation:** build issues first (IR-1..IR-3) because
escalating a risk and the standup blocker both need them; then change requests
(IR-4, IR-5) reusing the controlled-documents separation of requester and
approver and the what-if scheduler for the impact preview; report in the
director pack last (IR-6). **Open questions for the owner:** (1) should an
approved change request be able to re-baseline automatically (recommended: no,
a person chooses); (2) is a *priority* separate from *severity* needed;
(3) do change requests need a configurable approval chain (more than one
approver) or is one approver enough to start; (4) is an external-ticket link
(Jira) wanted for issues. Not taken: automatic re-planning from a change
request, and cost-benefit scoring of requests.


# Research: readiness for major medical and governmental organizations (RG-n)

Requested 2026-10-10: "assess readiness for production use at major medical
governmental organizations". **Evidence base, stated plainly.** Read: the
governance and operations packs and their recorded gaps, the DTAC checklist, the
code for the specific controls named below (verified by search, not by running a
penetration test), the CI and release workflows. **Not done, and not claimed:**
any penetration test, any load test of the service, any assistive-technology test
by a person, any clinical safety assessment, any audit by a third party, any
production deployment. "Medical" here means a health organization using this as
a **non-clinical** portfolio and project tool (no patient records, no clinical
decision support); a clinical use would need a clinical safety case this project
has not written.

## 58. Verdict

**Not ready for unsupervised production use at a major medical or governmental
organization today. Ready for a controlled pilot** on non-clinical, non-secret
portfolio data, by an organization that will complete its own assurance. The
gaps are mostly *assurance and lifecycle* (evidence, retention, erasure, access
review), plus a few *technical controls* that were found missing and are listed
below. The foundations are better than a typical young project: secure by
default, tested restore, a governance pack, accessibility audit on every route,
supply-chain gates and signed provenance.

| Area | Where it stands | Evidence |
|---|---|---|
| Authentication | Strong: enforced by default; OIDC/Entra; Keycloak; MFA via the identity provider | NFR-12, NFR-18; tests |
| Authorization | Strong: attribute-based policy, private plans kept out of aggregates, finance gate | `auth.rs`, tests |
| Transport and headers | Good: CSP, HSTS over TLS, CORS allow-list, rate limit | NFR-13, NFR-14 |
| Supply chain | Good: `cargo deny`, Dependabot, CycloneDX SBOMs, build provenance | `ci.yml`, `release.yml` |
| Backup and recovery | Good, with one gap: tested restore, stated RPO/RTO; **backups are not encrypted by the script** | operations pack |
| Accessibility | Partly: automated WCAG 2.2 AA on every route; **no manual or assistive-technology test** | accessibility statement |
| Audit trail | Partly: audited writes with digests and optional MAC; **rows can be altered or deleted by anyone with database access**, and deletion is undetected | DPIA risk |
| Retention and erasure | **Gap:** a retention schedule exists but **nothing purges**; no data-subject erasure or export tool | DTAC rows "Partly" |
| Outbound requests | Good for **user-supplied** destinations (`https` only, public addresses only, checked again when the name is resolved, no redirects). **Gap, smaller than first thought:** **operator-configured** webhook and worker targets accept any `https` host, including private and metadata addresses; that is deliberate for internal services, but there is no strict mode | `sinks.rs`, `webhooks.rs` |
| Sessions | **Gap:** OIDC sessions have an absolute lifetime but **no idle timeout**; the store is in memory, so several front-end replicas need sticky sessions | `oidc-session.ts` |
| Handling marks | **Gap:** no security classification or sensitivity label on records or exports (governments and health bodies use them) | none |
| Attachments | **Gap:** no content allow-list or malware-scan hook | none |
| Access review | **Gap:** no periodic recertification report; no break-glass procedure | none |
| Performance | **Unknown:** no load or soak test; the service benchmark has not been run | `BENCHMARKS.md` |
| Assurance | **None yet:** no penetration test, no third-party audit, no clinical safety review | by design out of the software |
| Support | One maintainer; no service-level agreement | `GOVERNANCE.md` |

## 59. Recommendations

Build now, because each is small, verifiable and closes a finding above:
**RG-1** outbound request guard; **RG-2** idle session timeout; **RG-3** audit
trail protected in the database; **RG-4** retention purge task; **RG-5** encrypted
backups; **RG-14** a readiness document and the corrected DTAC rows.

Backlog, in this order: **RG-6** data-subject export and pseudonymisation;
**RG-7** handling marks; **RG-8** attachment safety; **RG-9** access review;
**RG-10** break-glass; **RG-11** load and soak test; **RG-12** shared session
store. **RG-13** is not software: penetration test, assistive-technology audit,
clinical safety review, disaster-recovery exercise, DPO sign-off, supplier
assurance. A checklist for it goes in the governance pack.

## 60. Deliberately not taken

- Claiming a certification (ISO 27001, Cyber Essentials, SOC 2, DTAC "approved").
  The project can supply evidence for an assessor; it cannot be certified as
  software alone.
- A clinical safety case. If a deployer puts clinical content in it, they need
  their own.
- Application-level encryption of fields at rest: disk and database encryption
  are the deployer's and are far better done there. (Encrypted *backups* are in.)
- Multi-tenancy inside one database: separate deployments per organization is the
  supported model.

## 61. Findings (from reading the code against the list)

- The delivery-log and outbox retention periods are stated in the retention
  schedule but no code enforces them (confirmed by search for any purge path).
- `webhooks.rs` (operator-configured targets) refuses `http` to non-loopback
  hosts but accepts any `https` host, including `169.254.169.254` and private
  ranges. **Correction made while building RG-1:** the first reading of this
  gap covered all outbound requests; in fact user-supplied notification
  destinations already have a full guard (`sinks::destination_url_ok`,
  `ip_is_public`, a re-check at send time). Only operator targets lack a strict
  mode.
- The DTAC checklist row "no vulnerability-disclosure policy file yet" is stale:
  `SECURITY.md` now exists.

## 62. Open questions for the owner (readiness)

1. Which buyers are first: health, central government, local government? Each
   brings a different marking scheme (RG-7) and assurance route.
2. Is erasure to be **pseudonymisation** (keep the audit row, replace the person
   reference) or deletion? Recommended: pseudonymise, because deleting audit rows
   destroys the evidence the audit exists for.
3. Retention periods: adopt the schedule's starting periods, or wait for the first
   buyer's records manager?
4. Will the project pay for a penetration test and an assistive-technology audit
   before a first regulated deployment (recommended), or leave that to the buyer?
5. Is a hosted service ever offered, or only software? A host takes on duties
   (processor terms, hosting assurance) that a software supplier does not.

## 63. Sequencing

RG-1, RG-2, RG-3, RG-5 and RG-14 first (small, independent); RG-4 after RG-3
because the purge needs the audit protection's controlled-delete path. RG-6 needs
RG-4's list of what is personal. RG-7 and RG-8 are independent. RG-11 before any
claim about capacity.

# Research: new capabilities and code quality (NC-n, CQ-n)

Research date: 2026-10-21, after the 0.2.0 release. Two questions: what do
comparable tools and standards offer that this trio lacks, and where is the
code itself weakest. The items are `NC-` (new capability) and `CQ-` (code
quality) in [tasks.md](tasks.md).

**Evidence base, stated plainly.** There were four read-only passes: the
service and matcher code, the UI code, the "Not built" and "Not done" notes in
this repository, and an online search. Online, two pages were read in full:
OpenProject's features page and the admin guide for its MCP server. The SAFe
Lean Portfolio Management page is behind a login, so only its definition was
read. Everything else about other products comes from search-result snippets,
much of it vendor or training copy, and should be treated as pointers to
verify. ISO 21502/21503/21504, the PMI portfolio standard, and the SCIM and MCP
specifications were not read. Taiga, Odoo and GitLab were not researched. In
the code, counts come from grep and are heuristic. `cargo clippy -W
clippy::pedantic` was run on the service. No gate, database test or browser
test was run. Where a finding below says *confirmed*, the code was read again
after the pass reported it.

## 64. Verdict

The scheduling, cost and governance core is broad. The service code is in
better shape than its size suggests: no panics in request paths, pedantic lints
on, and an OpenAPI route census. The UI has no Svelte 4 leftovers, runs axe on
every route, and checks English/Welsh key parity. The weak points are these.

1. **Silent failure.** About 220 audit writes discard their error
   (`AuditModel::record(..).await.ok()`). The UI swallows best-effort loads,
   including the handling-mark banner. Visibility lookups fail closed but log
   nothing.
2. **Copied helpers that have drifted.** There are three CSV escapers and only
   one of them defuses spreadsheet formulas. There are eight `is_admin` copies:
   three treat "no claims" as admin, two as not admin. There are seven
   `find_plan` copies, and 40 local `unprocessable` helpers emit two different
   error codes.
3. **No contract between the UI and the service beyond paths.** The OpenAPI
   document is hand-written: 7 `requestBody` entries over about 5,100 lines,
   and 52 paths are known to be undocumented. The 4,094-line `ppm.ts` client
   is hand-written too, and the e2e tests stub the API, so the client, the
   stubs and the service can all drift apart.
4. **The BFF proxy drops response headers** (confirmed). `+server.ts` relays
   only `content-type`. In the browser, paging totals (`X-Total-Count`),
   `ETag`, `Retry-After`, `Location` and `Content-Disposition` never arrive.
5. **About 98 service features have no screen.** This is the largest
   capability gap, and it is mostly already built at the API.

The new capabilities with the most value are machine access for AI agents (an
MCP adapter), the UK Programme and Project Data Standard, and a regular status
update with a due date.

## 65. Code-quality findings (CQ-n)

| ID | Finding | Evidence | Risk |
|---|---|---|---|
| CQ-1 | CSV formula injection in plan export, auditor evidence and timesheets | `src/bulk.rs:101` and `src/visibility.rs:333` quote but do not defuse `= + - @`; `src/access_review.rs:91` does (confirmed) | Security |
| CQ-2 | `is_admin` copies disagree on "no claims" | `milestone_signoffs.rs:123`, `deescalation.rs:73` use `is_none_or` (no claims ⇒ admin); `bulk.rs:69`, `plan_access.rs:43`, `attachments.rs:33` use `is_some_and` (confirmed). Whether "no claims" is reachable with auth on was **not** checked | Security, to verify |
| CQ-3 | Audit writes discard errors | ~220 `AuditModel::record(..).await.ok()`; e.g. `controllers/financials.rs:174` audits after `commit()` | Correctness: AGENTS.md promises an audit row per write |
| CQ-4 | BFF proxy relays only `content-type` | `ui/src/routes/api/proxy/[...path]/+server.ts:67-72` vs `client.ts:220` (confirmed) | Correctness: paging totals wrong in the browser |
| CQ-5 | Proxy forwards a browser `Authorization`; no Origin check on mutations; upstream failure is an unhandled 500 | `+server.ts:28`; `server/session.ts:17` SameSite=Lax is the only CSRF defence | Security, low |
| CQ-6 | Visibility lookups fail closed silently (200 with empty data during an outage) | `controllers/plan_access.rs:134,137,174` | Operability |
| CQ-7 | Copied controller helpers | 40 `unprocessable`, 26 `db_err`, 7 `find_plan`, 4 `find_task`, 17 open-coded `Uuid::parse_str`; `"unprocessable_entity"` ×51 vs `"unprocessable"` ×7 | Drift |
| CQ-8 | OpenAPI checks paths, not bodies | 7 `requestBody`; `KNOWN_UNDOCUMENTED` 52; census at `openapi.rs:2290` compares path and method only | Contract drift |
| CQ-9 | UI client and e2e stubs are unchecked against the service | `ppm.ts` ~250 methods, 169 not unit-tested; `page.route` stubs | Contract drift |
| CQ-10 | Full-text index writes block the async runtime | `streaming.rs:359,372` call tantivy sync under `std::sync::Mutex` with `commit()` | Latency |
| CQ-11 | Queries in loops, one of them racy | `financials.rs:184`, `strategy.rs:283`, `earned_value.rs:557` (select-then-upsert per currency, no transaction) | Performance, correctness |
| CQ-12 | `format!` SQL, and raw `DbErr` text in a 409 body | `retention.rs:162,178`, `controllers/subjects.rs:165,181,187`, `sandbox.rs:389` | Fragility, information leak |
| CQ-13 | "Today" computed in UTC | `Utc::now().date_naive()` ×93 vs `tz::local_today` ×14 | Off-by-one near midnight |
| CQ-14 | Lints do not reach the tests; stale allows hide | 64 `too_many_lines`, 174 `missing_docs` allows; ~150 pedantic warnings in `tests/` | Hygiene |
| CQ-15 | Large modules | `tba.rs` 3,401 lines, `controllers/engineering.rs` 1,649, `controllers/visibility.rs` 1,612 with no tests, `openapi_more.rs` 2,611 | Maintainability |
| CQ-16 | Probably unused dependencies | `regex`, `include_dir`, `futures-util`, sea-orm `sqlx-sqlite`; `sha2` listed twice; UI Lily pickers pinned but resolved as older duplicates | Supply chain |
| CQ-17 | UI effect-driven loads fetch twice and race | `calendar/+page.svelte:40,112`, `flow/+page.svelte:65,223`, `board/+page.svelte:71` | Stale data |
| CQ-18 | Param pages go stale on same-route navigation | 27 pages load in `onMount`; `plans/[pid]/+page.svelte:19` holds a non-reactive `pid` and links to other plans | Wrong record shown |
| CQ-19 | UI silent failures | WBS link counts, plan modules, Gantt calendar, announcements downgrade, **handling-mark banner** (`+layout.svelte:49`), security headers in `hooks.server.ts:53` | Correctness, one security |
| CQ-20 | Load/error boilerplate copied | `err instanceof Error ? err.message : t(..)` ×173 in 89 files; 84 `let error = $state` | Maintainability; untranslated service English reaches users |
| CQ-21 | DataTable is untyped | 180 of 182 `as unknown as` are column accessors (`DataTable.svelte:51-53`) | Type safety |
| CQ-22 | No UI lint or format gate | no ESLint, no Prettier config; 51 files unformatted | Hygiene |
| CQ-23 | i18n leaks | `ObjectiveCard.svelte:187` English `confirm()`; catalogue test skips strings in expressions; ~480 page keys possibly unused; 25 `as never` key casts | Locale quality |
| CQ-24 | Thin e2e for nine routes | `/announcements`, `/automations`, `/lifecycle`, `/reports`, `/reviews`, `/verify`, critical-path, earned-value, RACI | Coverage |
| CQ-25 | Documentation drift | CHANGELOG says link routes skip the private check, but `privacy_mw` guards `/api/plans/{pid}/…`; CHANGELOG l.339 says `style-src-attr 'unsafe-inline'`, but `vite.config.ts:32` now uses a hash; `COMPARISONS.md` says change requests are not built (IR-4 built them); `spec/15` "Planned, not built" lists DP/EX/CC; `ppm.ts` header says 13 locales; package description says "Case Service (Main X Index)" | Honesty of the docs |

**Corrections made while writing this.** The UI pass reported that "the service
has no OpenAPI". That is wrong: the document is hand-written (`src/openapi.rs`),
not derived. The harvest pass read the CSP note in the CHANGELOG as a live gap.
The CSP has since been tightened, so it is drift (CQ-25), not a risk.

## 66. Correctness gaps already recorded in the backlog notes

These are gaps the backlog entries already mention in passing. They are worth
lifting into their own items because each can give a wrong answer, not just
leave a feature missing:

- The WBS and earned-value pages read only the first 2,000 tasks and do not say
  so. Earned value can be silently wrong on a large plan (RG-15).
- Goals are addressed by position with no `If-Match` (OQ-5). Availability
  overlap is checked in the service, not by a constraint, so it can race
  (PJ-3).
- Nothing checks who may acknowledge or resolve an escalation. Any writer may
  edit a shared framework, in place, for every plan that follows it.
- The EX-13 "new asks" section is not filtered for private plans. RBS
  drill-down privacy is unproven.
- No one is notified of an automatic escalation. No scheduler runs the three
  cargo tasks (`tolerance_escalations`, `exception_breaches`,
  `director_pack_schedule`), and `operations/deployment.md` names only
  `retention_purge`.
- The critical-path page shows the 413 message instead of offering
  `?only=critical`.

## 67. New capabilities (NC-n), ranked

| ID | Capability | Who has it (source) | Here today | Size |
|---|---|---|---|---|
| NC-1 | Screens for the service features that have none (canvases, templates, comments, attachments, watchers, notification sinks and preferences, task relations, bulk edit, timesheets, MSPDI export, access review, break-glass, change-request drafting, issue raising, module settings …) | — (repository notes) | API only, ~98 notes | L, in slices |
| NC-2 | Read-only MCP adapter over the existing API, under the caller's own ABAC and handling marks; runs no model | OpenProject MCP server (openproject.org/docs/system-admin-guide/integrations/mcp-server/), Atlassian Rovo MCP, Plane, Planview Anvi (snippets) | None | M |
| NC-3 | Export to and validation against the UK Programme and Project Data Standard (v1 on trial to 31 Dec 2026; v2 early 2027) | projectdelivery.gov.uk (snippets), APM blog | Data mostly exists; no mapping | M |
| NC-4 | Status updates on a cadence: an on-track / at-risk / off-track narrative per plan, due reminders, an overdue follow-up, and a feed into the director pack | Linear initiative and project updates (snippets) | Confidence assessments and packs; no per-plan update | S–M |
| NC-5 | Extend the existing in-process ticker (`SCHEDULER_MINUTES`, which already runs scheduled actions, report schedules, director-pack drafting and milestone sweeps) to the other cargo tasks and the threshold sweep, one runner via an advisory lock, plus notification on automatic escalation | — (repository notes) | Partly: four of the jobs | M |
| NC-6 | WSJF / cost of delay and RICE as selectable scoring models beside the Smart Score, absent input = null | SAFe (snippet), Jira Product Discovery (snippet) | Seven-component weighted score | S |
| NC-7 | Efficient frontier: re-run the explainable scenario generator over a sweep of budget caps | Planisware glossary, Planview (snippets) | One cap | S |
| NC-8 | Lessons-learned register fed at gates and closure, surfaced at intake by the matcher | PRINCE2 lessons log (not read); no tool cited | Vocabulary only (`spec/01:244`) | S |
| NC-9 | Portfolio balance view: value, risk and cost bubbles by category or strategic aim, with imbalance flags (MoP "balance") | MoP course outline (snippet) | `distribution.rs` is flow work type, not investment | S |
| NC-10 | Impact/effort matrix and Now / Next / Later views over ideas and proposals | Jira Product Discovery views (snippet) | Kanban, pipeline, Gantt | S |
| NC-11 | CloudEvents envelope option per webhook target, and an AsyncAPI document for the event stream | CloudEvents spec; Fatture in Cloud (snippet) | Own envelope, HMAC | S |
| NC-12 | Blueprint sync: offer template changes to the plans made from it, with a diff and per-plan consent | Smartsheet Control Center (snippet) | One-shot instantiate | M |
| NC-13 | Multi-project import from Project Online exports: many MSPDI files, the enterprise resource pool, enterprise custom fields | Project Online retired 30 Sep 2026 (snippet) | One plan at a time | M |
| NC-14 | Meetings: agenda, minutes and outcomes linked to plan items, with a PDF | OpenProject 16.0 / 16.1 (release notes, snippet) | Decision log, ceremonies | M |
| NC-15 | Service accounts or scoped API tokens for integrations (needed by NC-2) | OpenProject (doc read) | PASETO from the auth service only | S–M, mostly in the auth service |
| NC-16 | Lean budgets with guardrails and participatory budgeting rounds | SAFe LPM (Cprime, Lean Wisdom; snippets) | Funding sources, tranches | M, only on demand |
| NC-17 | A weekly "portfolio exceptions" digest from the existing deterministic rules | monday.com risk agents (vendor copy) | Thresholds, tolerances, escalations | S |

**Already here (checked, not proposed):** iCalendar and Atom feeds, baselines
and schedule variance, scenario evaluation and generation, the intake pipeline
with votes, gates and tranches, sprints, timesheets, custom fields, templates,
automations, signed webhooks, OpenAPI, NDJSON bulk export, the KPI surface, the
decision log, RACI, skill pools, delivery confidence, OKRs and benefits.

## 68. Deliberately not taken (this pass)

- **SCIM provisioning.** Identity belongs to the authentication service, as
  with LDAP, OIDC and 2FA. At most, a deprovision webhook could revoke plan
  access and guest grants here.
- **AI copilots, sentiment analysis and generated scenarios inside the
  service.** Already refused in §16; they break "it says why". NC-2 is a
  protocol adapter that runs no model.
- **Claiming conformance with ISO 21503/21504 or the PMI portfolio standard.**
  They are guidance. At most, a coverage map belongs in `governance/`, as with
  P3M3.
- **A separate SAFe portfolio kanban.** The proposals pipeline covers it. A WIP
  limit per stage would be a small amendment.
- **Real-time co-editing, a plugin marketplace, inbound email.** Already not
  taken.

## 69. Sequencing

1. **Security and correctness first, each small:** CQ-1, CQ-2 (check first,
   then fix), CQ-4, CQ-5, the handling-mark part of CQ-19, CQ-3.
2. **Then the shared helpers that stop the drift from recurring:** CQ-7 with a
   census that bans local copies, CQ-20 (`resource()` / `errorText()`), which
   also closes CQ-17 and most of CQ-19, and CQ-6.
3. **Then the contract:** CQ-8 (request bodies documented and tested), then
   CQ-9 (generate or census the TS client against the document).
4. **Capabilities:** NC-5 and NC-4, which are small and make existing rules
   work; NC-1 in slices, ordered by the backlog's own correctness notes (§66);
   NC-6, NC-7, NC-9 and NC-10 together, since they share the selection screens;
   NC-2 after NC-15; NC-3 once the owner names UK government as a first buyer
   (§62 question 1).
5. **Hygiene, any time:** CQ-14, CQ-15, CQ-16, CQ-21, CQ-22, CQ-23, CQ-24,
   CQ-25.

## 70. Open questions for the owner (this pass)

1. **"No claims" when auth is off.** Should it mean admin (the convenient
   development behaviour) or not admin? CQ-2 needs one rule, written down.
   Recommended: not admin, with an explicit development flag.
2. **Audit failure.** Is a write whose audit row fails to land refused (same
   transaction), or kept and logged loudly? Recommended: refused, with a
   best-effort path only for reads that are audited.
3. **Error code.** At the next API version, should it be `unprocessable_entity`
   or `unprocessable`?
4. **NC-2.** Is AI-agent access wanted at all, given §16? If yes, read-only
   only, or with writes later?
5. **NC-3.** Is UK government a first buyer, which would make the data standard
   worth tracking through its v2?
6. **NC-5.** Should the service schedule the rest of its jobs (the ticker already exists and is opt-in),
   which is simpler for small deployers, or should that stay the operator's
   job?

# Research: Brainitive (BR-n)

Research date: 2026-10-25. Subject: [armendzekjiri/brainitive](https://github.com/armendzekjiri/brainitive)
(PHP / Laravel 12 + Inertia + React; AGPL-3.0; 20 stars; created 2026-02-23). The full map is
[spec/brainitive-lessons/index.md](spec/brainitive-lessons/index.md); this section is the assessment and the
recommendations.

**Evidence base.** Read: the README, `AGENTS.md`, `docs/ai/README.md`, two skills, the headings of `KNOWLEDGE.md` and
`CONVENTIONS.md` and its webhook and insights sections, 18 per-branch plan files, three middleware files, the file
listing of `app/`. Not read: most source, tests, UI, deployment. What it does is **as its own documents say**, not
as run. **Licence note:** AGPL-3.0 is strong copyleft; this plan takes *ideas only*, and no code, text or schema is to
be copied from it.

## 71. What it is, and why it is worth reading

A small-team workspace (boards, docs, whiteboards, Gantt plans, helpdesk, leads, billing, MCP server) shipped as one
monolith. It is not a portfolio tool, and most of it is out of scope here. It is worth reading because it documents
each decision in a per-branch plan file with its definitions, its permission rules and its test plan, and because the
parts that overlap with ours (an MCP server for AI clients, outgoing webhooks, status reporting, workload, templates,
and a security review) were hardened by real incidents they wrote down.

## 72. Where we already match or lead

- Planning depth: critical path, earned value, buffers, stage gates, escalation, tolerances, a director pack. They have
  Gantt rows and milestones.
- Templates (LT-2) and now sync of a template's change to its plans (NC-12); they have versions and runs.
- Time zones: "today" is the deployment's date and a period is local days converted to bounds. Same doctrine.
- Person-chosen status, no computed health score (NC-4). Same doctrine, stated in both.
- Signed webhooks with a private-network guard (`webhooks.rs`, `sinks.rs`); payloads without personal data.
- ABAC with an explicit read-only scope (NC-15); they rely on workspace roles and policies per model.
- Audit rows in the write's own transaction; they log an activity feed.
- A read-only MCP adapter is planned (NC-2); theirs has about 50 tools including writes.

## 73. Gaps worth considering

| # | Idea | Why it matters here | Fit | Task |
|---|---|---|---|---|
| 1 | **Idempotency keys on writes** a client may retry; same key and body replays, same key and other body is refused | We have `Idempotency-Key` only for bulk import. Integrations, webhook consumers and any future MCP write retry | High | BR-1 |
| 2 | **Evidence-backed status** — claims with source references validated on save, resolved for the reader, "unavailable" when unreadable; the definitions of every number published | NC-4's narrative is free text. A board wants the status and the numbers behind it, and must not learn from a source it may not read | High | BR-2 |
| 3 | **Propose, verify, create once** for action items from minutes, with no model in the service | NC-14 records "action" outcomes that nobody turns into tasks. The verify-and-create half keeps the §16 refusal | Medium-high | BR-3 |
| 4 | **Reverse references** — which meetings and lessons mention this task, risk or issue | NC-14 and NC-8 link out; nothing answers "referenced by". The batching and no-existence-leak rules are the useful part | Medium | BR-4 |
| 5 | **Webhook replay protection and redelivery** — sign `timestamp.body`, stable event id, a redeliver action | NC-11 signs the body only; a captured request can be replayed, and a failed delivery cannot be retried by hand | High | BR-5 |
| 6 | **A security sweep with fail-on-old-code tests** for the classes their review found | Cheap, generic, and our attachments, sinks, search and the future MCP adapter are the same shapes | High | BR-6 |
| 7 | **Planning rules audit** — no double counting parent and child, flag don't overwrite, unknown is not zero, Monday-dated weeks | Our `allocations` and capacity heatmap have not been read against a stated rule list | Medium | BR-7 |
| 8 | **Reviewer subagents over one checklist** | One copy of the review rules for every agent; our `agents/` has no reviewer | Medium | BR-8 |
| 9 | **Expiring, revocable share links** with a whitelisted payload, noindex, 410 | Useful for a meeting PDF; risky for portfolio data. Owner decision | Low-medium | BR-9 |
| 10 | **Tests must not depend on what other tests create** | We hit it on 2026-10-25: a shared search index and a plan named "Apollo" failed the restore drill | High | BR-10 |
| 11 | MCP details: no null in a declared string field, read-only and idempotent annotations, a location on every result, one page of tools, a server instructions block | Prevents the failures they hit with strict clients | High (amends NC-2) | amend NC-2 |
| 12 | Fixed query counts asserted per aggregate view; each definition in one place | Direct support for CQ-11 | High (amends CQ-11) | amend CQ-11 |

## 74. Deliberately not taken (Brainitive)

- **Helpdesk, ticket portal, lead forms, billing and invoicing, whiteboards, the document editor, Google Calendar
  sync, a browser extension, web push.** Other products.
- **A personal capture inbox and focus timer.** A worker-productivity feature; our audience is the portfolio.
- **AI writing helpers, and AI extraction of tasks from notes, inside the service.** The same refusal as §16: it
  would be the one output that cannot disclose its inputs. BR-3 takes the verification half only.
- **SQLite by default, and the monolith shape.** We are one Rust service over PostgreSQL.
- **Their commercial model** (hosted cloud, AGPL self-host, a separate commercial licence).
- **A single fix version per issue and a public changelog token.** Their releases are software releases; ours are
  milestones and stage gates. A public changelog is not wanted for portfolio data.

## 75. Sequencing

1. **BR-6** first: it can find real defects, and each finding is a small fix.
2. **BR-1** and **BR-5**: independent, small, and both protect integrations.
3. **BR-10** at any time; it is a test change.
4. **BR-3** and **BR-2**: they extend NC-14 and NC-4 and share the idea of validating references on write.
5. **BR-4**, **BR-7**, **BR-8**: small; BR-4 depends on NC-14 and NC-8, BR-7 only reads.
6. **BR-9** only on request.
7. The two amendments go into NC-2 and CQ-11 when those are picked up.

## 76. Open questions for the owner (Brainitive)

1. **BR-3.** Is an external agent proposing tasks from minutes, with the service only verifying and creating, an
   acceptable reading of the §16 refusal? Recommended: yes, because the service runs no model and every item names
   its evidence.
2. **BR-9.** Is a public, expiring link to a meeting PDF wanted at all? Recommended: no, until handling marks can
   refuse it for marked content.
3. **BR-1.** Should the key be required on creates (breaking) or optional? Recommended: optional on the REST API,
   required on any MCP write tool.
4. **BR-5.** Is a breaking change to the signature acceptable? Recommended: send both the old and the new header for
   one release, and document the receivers' window.

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

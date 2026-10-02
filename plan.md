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

1. **Cheap, high value first:** LT-3 (My work), LT-9 (timezone audit), LT-7 (iCal).
2. **Needs T-8's artifact store:** LT-5 (attachments); LT-2 (templates) can use
   the same clone path as T-8's import pipeline.
3. **Strategy layer:** LT-1 (canvases), then LT-4 (comments/reactions) because
   canvas items want discussion.
4. **Extend what exists:** LT-6, LT-11, LT-10.
5. **Decide before building:** LT-8 (wiki), LT-12 (UX pass scope).

LT-2 and LT-5 are blocked on, or shaped by, the T-8 design awaiting approval
([spec/bulk-import-export.md](spec/bulk-import-export.md)).

## 6. Open questions for the owner

- Is the target user ever a *non-project-manager*, or only a PMO? If the PMO
  only, LT-3 and LT-12 drop in priority.
- Wiki: build, or link out to the organisation's existing docs system?
- Canvases: generic engine (one table, typed templates) or a bespoke table per
  canvas? The task assumes generic.
- Should comments / reactions be on *every* entity or only plans, tasks, risks
  and proposals?

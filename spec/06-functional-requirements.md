## 6. Functional Requirements

Each requirement names its owning subproject. The trio is implemented
(§14); §6.1–§6.8 describe what is built unless a requirement says
**not built**, and delivery state per task is in the repo-root
[`tasks.md`](../tasks.md) (authoritative).

There is one plans collection — `/api/plans` — where `{pid}` is a plan
record. All matching, dedup, and merge endpoints are **kind-agnostic**:
any two plans may be compared (§5.5); the optional `kind` label neither
gates nor scores.

### 6.1 Registry CRUD — service (thin record)

- **FR-1** Create a plan: `POST /api/plans` with a `Plan` body (optional
  `kind` label); reject a blank `name` with `422`; return `{pid, name}`.
- **FR-1a** Validate the body (service): reject with `422` a blank
  `name`, a malformed `EntityRef` (`owner_org_id`, `lead_ref` — must
  parse as `<entity_type>:<id>`), a malformed `parent_ref` (must be a
  valid plan `pid`) or a `parent_ref` that would form a **containment
  cycle** (points a plan at itself or at one of its descendants), a
  malformed deterministic `identifiers` entry (UUID / URI / external-PM-id
  shape — rejecting a malformed *deterministic* identifier matters
  because a shared value short-circuits the match to 1.0, R-0), a
  relationship that self-references or names an unknown plan, or a
  malformed `in_language` (BCP-47). All problems are reported in one
  response.
- **FR-2** List active plans: `GET /api/plans` returns
  `{pid, name}` refs, most-recent first, capped at 100. A
  `?parent=<pid>` filter rolls up one plan's children.
- **FR-3** Read: `GET /api/plans/{pid}` returns the stored thin
  `Plan`; `404` for unknown or soft-deleted `pid`.
- **FR-4** Update: `PUT /api/plans/{pid}` replaces the whole
  thin payload (and the denormalized `name` / `parent_pid`); same
  validation as FR-1a.
- **FR-5** Soft delete: `DELETE /api/plans/{pid}` sets
  `deleted_at`; the record and its sub-resources disappear from list /
  read / match.

### 6.2 Name search — service

- **FR-5a** `GET /api/plans/search?q=` — case-insensitive name
  search (Postgres `ILIKE` on the denormalized `name`, cap 50,
  wildcards escaped); blank `q` → `400`. Tantivy full-text over the
  payload is roadmap (§15).

### 6.3 Matching — matcher (algorithm) + service (endpoints)

Algorithm reference: [`agents/matching.md`](../agents/matching.md) and
the matcher [spec §5–§18](../project-portfolio-management-matcher-with-rust/spec/index.md).

- **FR-6 (kind-agnostic matching)** The matcher compares any two plans
  regardless of their optional `kind` label; there is **no kind gate**.
  Service endpoints feed candidate plans without kind filtering.
- **FR-7** Deterministic short-circuits (matcher): score pins to 1.0
  on —
  - **R-0**: any shared value on a **deterministic identifier scheme**
    (`Uri`, `Uuid`, `JiraProjectKey`, `AsanaGid`, `TrelloBoardId`,
    `MsProjectId`, `GitHubProjectId`, `LinearId`); the owner-scoped
    `Code` / `LocalId` and `Custom` are **excluded**;
  - **R-1**: same non-empty `owner_org_id` + equal normalized `code`;
  - **R-2**: any case-folded `same_as` URL overlap.
- **FR-8** Probabilistic components (matcher), renormalized weighted
  average over the components both records carry:

  | Component | Weight | Algorithm |
  |---|---:|---|
  | Name | 0.30 | Best Jaro-Winkler over `name` + `alternate_names`; Soundex +0.05 bonus capped at 0.95 |
  | Goals | 0.15 | Jaccard over folded goal *titles* |
  | Code | 0.15 | Same `owner_org_id`: 1.0/0.0; differing or missing owner: skipped |
  | Owner org | 0.10 | `owner_org_id` exact `EntityRef`; skipped when either unset |
  | Parent | 0.08 | `parent_ref` exact parent-plan match; skipped when either unset |
  | Timeframe | 0.07 | Date proximity over `start_date` / `target_date`; skipped when no dates |
  | Keywords | 0.05 | Jaccard over folded sets |
  | Relationships | 0.05 | Typed-set Jaccard over `(relation, plan_id)`; skipped when either empty |
  | Tags | 0.05 | Set Jaccard over case-folded sets; skipped when either empty |

  Weights sum to 1.00. (`status` and the optional `kind` label are
  informational-only and carry no weight; the parent component replaces
  the plan-family `plan_type` weight.)
- **FR-9** Explainability (matcher): every result carries `score`,
  `Confidence` (`High` ≥ 0.95, `Medium` ≥ 0.70, else `Low`),
  `is_match` (threshold 0.85 default; `strict` 0.95 / `lenient`
  0.70), and a per-component `MatchBreakdown`.
- **FR-10** Ad-hoc ranking (service): `POST /api/plans/match`
  scores a `{query, candidates}` set without persistence, returning
  ranked `(index, MatchResult)` pairs.
- **FR-11** Duplicate check (service):
  `POST /api/plans/check-duplicates` matches a query against
  stored plans and returns hits above threshold as
  `{pid, name, score, confidence, is_match}`, sorted by score
  descending.
- **FR-11a** Real-time duplicate detection on create (service):
  `POST /api/plans` returns `409 Conflict` with candidate
  matches when a likely duplicate is detected (family baseline,
  `agents/share/match-search-merge.md` (shared family guidance, not in this repository));
  a `force` flag bypasses for deliberate near-duplicates.
- **FR-11b** Record merge (service): `POST /api/plans/merge`
  folds a confirmed-duplicate plan into a surviving one (any two plans;
  no kind restriction) — union the list fields, keep the duplicate's
  name as an `alternate_names` entry, **re-home the duplicate's
  sub-resources** (tasks / issues re-keyed to the survivor's `pid`),
  soft-delete the duplicate, write a `merge_records` history row
  (snapshot of the transferred payload), and publish a `Merged` event.
  Equal `main_pid`/`duplicate_pid` → `422`; unknown pid → `404`.
  `GET /api/plans/merges/recent` lists the history.

### 6.4 Operational sub-resources — service (project-management tool)

Each is a child resource of a plan, keyed by `parent_pid`, in its own
Postgres table (§10.1). None enters the matcher payload (§5.6). All
sub-resource writes emit events and audit rows (§6.6). The sub-resources
hang off **any** plan.

- **FR-12 Goals.** CRUD goals under a plan
  (`/api/plans/{pid}/goals`); goal writes mutate `data.goals[]` on
  the parent so the matchable payload and the sub-resource stay
  consistent (§5.3, §10.2). `Goal { title, description?, target_date?,
  status }`. A goal has no identifier, so it is addressed by its
  zero-based position: `GET` and `POST /api/plans/{pid}/goals`,
  `PUT` and `DELETE /api/plans/{pid}/goals/{index}`, and the POST twin
  `…/{index}/delete`. Each answer is the whole list, every goal with its
  `index`; a delete moves the later goals down one, and an index past the
  end is `404`. Writes rewrite the plan like `PUT /api/plans/{pid}` (same
  validation, so a blank title is `422`, same audit row and event).
  Because a position moves, every answer carries a weak `ETag` of the goal
  list; a `PUT` or delete that sends `If-Match` with an older tag is `412`
  (`precondition_failed`) and changes nothing, so the caller re-reads instead
  of editing whatever goal now sits at that position (CF-2). Without
  `If-Match` the write goes ahead, as before. A list page that is the whole
  list keeps its `ETag`; a page that leaves rows out drops it.
- **FR-13 Tasks.** CRUD tasks under a plan
  (`/api/plans/{pid}/tasks`). `Task { pid, parent_pid, title,
  description?, assignee_ref? (EntityRef), status:
  Todo|InProgress|InReview|Done|Blocked, goal_id?, parent_task_id?,
  estimate?, remaining?, due_date? }`. A task may attach to a goal and
  nest under a parent task. Siblings (tasks with one parent, or the top
  level) have an order: `PUT /api/plans/{pid}/task-order` takes
  `{parent_task_pid?, order: [pid, ...]}`, which must name exactly the
  parent's live children once each (`409` otherwise, `422` for an unknown
  parent or a repeat), and task lists return positioned siblings first
  (`sibling_position`), the rest by age.
- **FR-14 Issues register.** An issue is something that has **already
  happened** and needs managing (a risk is something that might). CRUD issues
  under a plan (`/api/plans/{pid}/issues`) and a portfolio-wide register
  (`GET /api/issues`, filterable by plan, severity, status, owner and age).
  `Issue { pid, parent_pid, title, description?, kind:
  Bug|Risk|Blocker|Question|Improvement|Concern, severity:
  Low|Med|High|Critical, priority?, status:
  Open|InProgress|Resolved|Closed, owner_ref? (EntityRef), raised_by,
  raised_on, due_date?, resolution?, resolved_on?, source_risk_pid?,
  source_task_pid?, change_request_pid? }`. Rules: a `Closed` issue records a
  resolution; ageing is counted in **calendar days** since `raised_on`; an
  issue raised from a risk (escalation) or from a blocked task keeps a link
  back and the source is never altered by closing the issue; every change is
  audited (FR-19). Reads obey the same attribute-based policy as tasks.
  **Built (IR-1):** the plan-scoped CRUD at `/api/plans/{pid}/issues`
  (migration 50, entity, rules in `src/issues.rs`, controller, OpenAPI,
  audited, soft delete, in the `risks` module). Also built (IR-2): the
  portfolio-wide `GET /api/issues` (readable plans only; filters by plan,
  status, severity, owner, open and `min_age_days`; summary by severity and
  status, overdue count and the oldest open issue) and the UI page `/issues`.
  Also built (IR-3): `POST …/issues/from-risk/{risk_pid}` (severity from
  exposure; the risk is untouched) and `POST …/issues/from-task/{task_pid}`
  (a blocked task only; the task is untouched), one open issue per source
  (`409` otherwise). **Not built:** a button for either on the pages, and
  `priority` as anything but free text.
- **FR-14a Change requests.** A change request is a controlled proposal to
  change an approved baseline: scope, schedule, budget or a deliverable.
  `ChangeRequest { pid, plan_pid, title, description, reason, kind:
  Scope|Schedule|Cost|Quality|Other, impact_schedule_days? (**business
  days**), impact_cost_minor? + currency, impact_summary?, status:
  Draft|Submitted|UnderReview|Approved|Rejected|Implemented|Withdrawn,
  requested_by, requested_on, decided_by?, decided_on?, decision_reason?,
  issue_pids[], risk_pids[] }`. Rules: decision is by a person with the
  approval permission, never the requester (the same separation as controlled
  documents); an approved request that moves the finish or the budget can
  take a new **baseline** (never replacing one) only when a person chooses to;
  the decision log keeps who decided, when and why; impacts are stated, not
  guessed (the schedule impact may be shown as a *what-if* preview from the
  existing scheduler and is labelled a forecast). **Built (IR-4, IR-5):** `/api/plans/{pid}/change-requests` with the actions `submit`, `review`, `decision`, `implement`, `withdraw`, `links`, `preview` and `baseline`, and `GET /api/change-requests` across plans; the named approver is the only person who reviews or decides, and not even an administrator decides for them. **Not built:** screens to draft, submit and decide; an approval chain of more than one approver; a notification to the approver.
- **FR-15 Timeline (derived).** `GET /api/plans/{pid}/timeline`
  returns a Gantt-style projection: goals with a `target_date` as
  milestones + tasks with `due_date` / date ranges as bars. Read-only;
  computed, not stored.
- **FR-16 Burndown (derived).** `GET /api/plans/{pid}/burndown`
  returns remaining-vs-estimate over time, from periodic snapshots of
  task `estimate` / `remaining`. Read-only; computed from snapshots.

### 6.4a Collaboration, automation, and prioritization — service

Delivered 2026-07-22 (service spec §9.4a). None of this enters the
matcher payload (§5.6); every mutation audits (§6.6).

- **FR-16a Collaborative review.** Delegate an idea / proposal / plan
  to one internal or external expert (`/api/reviews`), track
  accept / decline, collect a verdict (`score` 0–100 optional,
  `recommendation` advance|hold|reject), and read the consensus.
  Reviewers are `EntityRef` URNs, **never raw email** — an external
  expert is a record in the person registry — and `reviewer_scope`
  (internal / external) is recorded explicitly, because disclosure to an
  outsider is a decision, not an inference. Only a reviewer who
  **accepted** may submit, so an unanswered invitation never becomes
  evidence; consensus reports a **strict** majority only (a tie or a
  plurality reports none), a `null` mean until somebody scores, and the
  count still outstanding.
- **FR-16b Assignee management.** Assign / unassign a task
  (`…/tasks/{t_pid}/assign`; `null` unassigns) and read the open
  workload per assignee, which surfaces the **unassigned** pile rather
  than dropping it.
- **FR-16c Workflow automation.** Rules configured once
  (`/api/automations`) fire as work crosses the Kanban board (or when a
  plan review is submitted) and apply one action: assign, add label,
  notify, schedule an action, or set a task status. Action shapes are
  validated at **write** time; a failing rule is recorded as a `failed`
  run and **never** undoes the operator's move; actions are applied
  without re-entering the engine, so automations cannot cascade. Every
  firing — applied, skipped, or failed — is logged.
- **FR-16d Set and forget.** A deadline configured once
  (`/api/scheduled-actions`) is held until due and fired **exactly
  once**: the sweep claims each row with a conditional update, so the
  optional in-process ticker and a manual sweep cannot double-fire.
  Notifications are **in-app only** — no email, no push.
- **FR-16e Smart Score (data-driven prioritization).** A derived,
  renormalized weighted average over ROI, strategic alignment, expert
  review, risk (inverted), demand, MoSCoW priority, and momentum, with a
  full per-component breakdown. **Absent evidence is dropped and
  disclosed** (`missing` + `coverage`), never scored zero; no evidence at
  all ⇒ `null` / `unscored`, sorted last rather than as a zero. ROI is
  computed only within a single currency (no FX conversion). Weights are
  deployment-tunable as a complete basis-point map; anything else falls
  back to the documented defaults with a warning. Nothing is stored, so
  the score cannot drift from its inputs.
- **FR-16e2 WSJF and RICE (NC-6).** Two more ways to rank, beside the Smart Score, from numbers a person
  estimates and the service stores per plan (`PUT /api/plans/{pid}/scoring-inputs`, all optional, audited with
  the before and after; `GET` reads them). **WSJF** is `(business value + time criticality + risk reduction or
  opportunity enablement) / job size`, each on a relative 1 to 20 scale. **RICE** is `reach × impact ×
  confidence / effort`: reach is a count per period, impact one of 0.25, 0.5, 1, 2 or 3, confidence 0 to 100
  percent, effort person-months above 0. `GET /api/plans/{pid}/score?model=wsjf|rice` returns the score, the
  formula in words, every component with what it means, and the inputs still missing, each with the reason it is
  needed. **Absent input is absent, not zero:** with any input missing the score is `null`; a stated zero (zero
  confidence) is a stated answer and scores 0. `GET /api/prioritization?model=smart|wsjf|rice` ranks by the chosen
  model (the Smart Score by default), highest first, the unscored last with their missing inputs; `band` applies
  to the Smart Score only (`422` otherwise). The ranking reads the plans the caller may read, as the Smart Score
  ranking does. Nothing but the inputs is stored. Not built: a screen for the inputs and the choice of model; weights.
- **FR-16e3 Efficient frontier (NC-7).** `GET /api/scenarios/frontier?currency=` re-runs the scenario generator
  (FR-16d) over a sweep of budget caps (`caps=` up to 50 minor-unit amounts, or `steps=` 2 to 50 caps spread evenly from
  0 to the cost of every scored, priceable plan; `must_include=` as in generate) and returns, per cap, the value
  (the sum of the Smart Scores of the included plans), the cost, the number included, and the same per-plan rationale
  rows a single `POST /api/scenarios/generate` at that cap returns. Nothing is saved. The selection is greedy, so it
  is not guaranteed optimal and the value need not rise with every cap: a point whose value is no more than that of a
  cheaper cap is marked `dominated`. Plans the caller may not read are not candidates.
- **FR-16e4 Lessons-learned register (NC-8).** A plan records a lesson at a phase gate or at closure
  (`POST /api/plans/{pid}/lessons`: `occasion` is a gate or `closure`; `category` is `went_well`, `went_badly` or
  `recommendation`; a one-sentence `summary`, optional `detail`). Lessons are append-only: one can be withdrawn
  (`POST …/{lpid}/withdraw`), after which it is no longer shown; both are audited. `GET /api/proposals/{pid}/lessons`
  matches the proposal's title with the engine the duplicate check uses against the plans that have lessons and returns
  each probable match with its score, confidence and per-component breakdown and that plan's lessons, best match first.
  Plans the caller may not read are not compared. Not built: matching on more than the title; limiting the comparison
  to closed plans; a screen; showing the lessons in the intake form.

- **FR-16e5 CloudEvents webhooks and an AsyncAPI document (NC-11).** A webhook target may set
  `"format": "cloudevents_structured"` (`application/cloudevents+json`, the envelope as `data`) or
  `"cloudevents_binary"` (`ce-specversion`, `ce-id`, `ce-source`, `ce-type`, `ce-subject` headers, the envelope as
  the body); the default `native` is unchanged. The CloudEvents `id` is the envelope's `event_id`, `source` is
  `urn:project-portfolio-management:<entity>`, `type` is `org.project-portfolio-management.<entity>.<kind>`, `subject`
  the record's pid; `time` is sent only when the envelope carries `occurred_at` (it does not today). The signature is
  unchanged: HMAC-SHA256 over the exact request body bytes, `X-Mxi-Signature`. `asyncapi.json` (AsyncAPI 3.0) describes
  the per-plan live stream (`task_created`, `task_updated`, `task_moved`, `raci_updated`, `comment_added`, `resync`)
  and the three webhook shapes. Not built: serving the document over HTTP; CloudEvents on the live stream.

- **FR-16e6 Weekly portfolio-exceptions digest (NC-17).** One short message of what is wrong across the portfolio:
  every red or amber tolerance, every open exception opened by a threshold and every live escalation (overdue ones
  first), worst first, with counts over all of them and at most 20 listed. `GET /api/exceptions/digest` shows it for
  the caller (only plans they can read). The `exceptions_digest` task, run weekly, builds it as the system identity
  (so a private plan or a plan under a handling mark is never in it) and sends it as an `exceptions_digest`
  notification, through each recipient's own sinks and preferences, to the `person:` references in
  `PROJECT_PORTFOLIO_MANAGEMENT_DIGEST_RECIPIENTS`; a recipient who already has this week's (Monday to Sunday of the
  deployment's "today") is skipped. Not built: recipients as an administrator setting or a per-person subscription;
  per-recipient content (every recipient gets the same digest); a screen.

- **FR-16e7 Status updates on a cadence (NC-4).** A plan sets how often it expects a status update
  (`PUT /api/plans/{pid}/status-cadence`, 1 to 92 days; setting it restarts the clock). A person posts an
  `on_track`, `at_risk` or `off_track` call with a narrative (`POST /api/plans/{pid}/status-updates`); updates are
  append-only and audited, newest first in the history. `GET /api/plans/{pid}/status` gives the latest update and where
  the plan stands: the date the next is due (a cadence after the last update, or after the cadence was set when there
  has been none), whether it is due or overdue and by how many days, in the deployment's time zone.
  `GET /api/status-updates/due` lists the plans the caller can read whose update is due, most overdue first.
  `POST /api/status-updates/remind`, and the scheduler ticker when it is on, tell the person who last posted (else who
  set the cadence) once per due date, as a `status_update_due` notification. The director pack carries a
  `status_updates` section: per included plan with a cadence, the latest update posted on or before the cut-off and
  where it stood on the cut-off's date. Not built: removing a cadence; a screen; the pack screen showing the section;
  a reminder to anyone but that one person.

- **FR-16e8 The service runs its own scheduled jobs (NC-5).** Opt in with `PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULED_JOBS`
  (comma-separated: `tolerance_escalations`, `exception_breaches`, `retention_purge`, `threshold_sweep`,
  `exceptions_digest`; unset runs none) and the scheduler ticker (`PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULER_MINUTES`).
  Each job has its own minimum interval (5 minutes for breaches, an hour for the threshold sweep, a day for escalations
  and retention, a week for the digest). A replica claims a job in a short transaction that takes a PostgreSQL advisory
  lock and writes a `scheduled_job_runs` row; one that finds a run started within the interval skips, so several
  replicas run a job once. Each run is audited at its start and its end as `system:scheduler` and counted in
  `scheduled_job_runs_total{job,outcome}`. Retention is a dry run unless
  `PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULED_RETENTION_APPLY` is true. Not built: a notice to the plan lead on an automatic
  escalation (who the plan lead is is owner question §70.6); an endpoint to read the runs; a job that failed being
  retried before its interval is up.
- **FR-16e9 Blueprint sync (NC-12).** A plan made from a template remembers it: the template, its place in the
  template's tree, the version it was last brought up to, a baseline of the template's tasks for that plan, and which
  of its tasks came from which template task. A template is revised from a plan tree
  (`POST /api/plan-templates/{pid}/revise`, a new version); the revision may change tasks and add plans at the end of
  the tree but not move, rename or drop one. Tasks keep their template keys by title, so a task renamed in a revision
  reads as a removal and an addition. `GET /api/plan-templates/{pid}/plans` lists the plans made from a template that the
  caller can read, with the number of changes waiting. `GET /api/plans/{pid}/template-diff` lists them as
  `add:`, `change:` or `remove:` plus the template task's key, each marked `locally_edited` when the plan's people have
  since changed or removed that task. `POST /api/plans/{pid}/template-sync` applies the changes named in `accept` in
  one transaction and audits it; a change marked `locally_edited` is refused (listed, not an error) unless it is also
  named in `overwrite_local`; a change not named stays waiting. Scope: the tasks' title, description and points. Not
  built: milestones, components, plan fields, workflows and task dates or hierarchy; plans made before this existed
  (no link, so no diff); events for the tasks a sync creates; a screen.
- **FR-16e10 Meetings (NC-14).** A plan holds meetings. `POST /api/plans/{pid}/meetings` plans one with a title, a
  date and an agenda; each agenda item, and later each outcome, may point at a task, risk, issue, milestone, change
  request or cutover decision **of the same plan** (a record of another plan, or of a kind not listed, is `422`).
  `POST …/{mid}/held` records the minutes (required, up to 20,000 characters) and the outcomes (`decision`, `action` or
  `note`), once; a held or cancelled meeting answers `409` to being held or cancelled again and is not edited.
  `GET …/{mid}` shows it with each linked record named as it is now, or marked as no longer there;
  `GET …/{mid}/pdf` prints the same, stating the meeting's own dates and not the moment of rendering, so two renders
  are byte-identical. Planning, holding and cancelling are audited in the write's own transaction. Meetings are
  retention class `COM`, kept with the plan and not purged by the service. Not built: attendees and apologies
  (people are not recorded, to keep the personal-data inventory small); a single decision log across kinds (outcomes
  of kind `decision` are listed per meeting and link to the change-request or cutover decision that records the
  formal decision); editing or correcting held minutes; a screen.
- **FR-16e11 Many MSPDI files in one job (NC-13).** `POST /api/import/mspdi-batch` (dry run by default; `dry_run=false`
  applies) takes up to ten MSPDI files. Each goes into an existing plan (`plan_pid`) or a new plan (`name`, else the
  file's project name, under `parent_pid`); every file is read and checked before anything is written, and a bad one is
  `422` naming its position. The resources of all the files are listed once (names equal after trimming, collapsing
  spaces and ignoring case), with the projects that use each and its assignments; they are still not matched to people.
  Each file is imported by the single-file importer (FR-PJ-10), so what that leaves out is still reported. A project's
  declared custom fields, enterprise ones included, are stored on the matching tasks according to the caller's
  `field_map` (the field's alias, name or id to the key of an existing task custom field, validated by that field's type);
  a field with no mapping, and a value the type refuses, are listed with the reason. Not one transaction: the job stops
  at the first project that fails and lists the rest as not attempted; a plan created by a failed run is kept.
  **Links between files:** a predecessor link written with `CrossProject` and `CrossProjectName` is placed when that
  name (last path segment, extension ignored) is another file of the job, matched to its label or project name, and the
  predecessor's `UID` is a task of that file; it becomes a dependency between the two plans' tasks (already-linked pairs
  skipped, a cycle refused). A link to a file not in the job, or to a task it lacks, is listed with the reason.
  **Pay:** each resource's standard and overtime rates (per hour, or per minute or day converted; week, month and year
  are not), calendar and per-use cost are merged across the files; a value the files give differently is a conflict and
  is not taken. `resource_map` says who a resource is (`person_ref` or `role`); only then, with finance access and a
  currency in the files, are rates written to that person's or role's `standard` / `overtime` rate table from
  `rates_effective_from` (append-only; the same rate again is "already there"). A calendar is written only for the
  caller's own `person_ref`, since a person's calendar is theirs to set; otherwise it is shown. A task's `FixedCost`
  becomes a fixed task cost once (with finance access and a currency). Costs that follow from rates and work are not
  carried; the service computes its own. **Custom fields:** with `create_fields`, a mapped field that has no task custom
  field is declared with the type that fits all its values (integer, date, yes/no, else text) and labelled by its alias.
  Not built: week, month and year rates, per-use costs, resource skills; writing another person's calendar; links the
  other direction (a successor in another file); a screen.
- **FR-16e12 Webhook replay protection and redelivery (BR-5).** Every webhook attempt carries `X-Mxi-Timestamp` (Unix
  seconds, taken afresh on each attempt), `X-Mxi-Signature-V2` (the integrity MAC over `"<timestamp>."` and the exact
  body bytes), `X-Mxi-Delivery-Id` (one delivery, all its attempts) and, unchanged for one more release,
  `X-Mxi-Signature` over the body alone. A receiver should refuse a timestamp more than 300 seconds from its clock.
  `GET /api/webhook-deliveries[?event_id=][&status=][&limit=]` lists the delivery log and
  `POST /api/webhook-deliveries/{id}/redeliver` sends a logged delivery again: one attempt, to the same target if it
  is still configured, from the event the outbox still holds (about a week); the event id is the same, the delivery id
  is new, a new log row records the outcome, and the audit row is written first. Both are administrators only (`404`
  for anyone else). Not built: redelivery of a notification-sink delivery (those have their own retry); redelivery
  after the outbox has dropped the event; an automatic replay-window check on our side (it is the receiver's rule).
- **FR-16e13 Action items from minutes to tasks (BR-3).** The service runs no model. For a **held** meeting (any other
  is `409`), `POST /api/plans/{pid}/meetings/{mid}/task-proposals/preview` judges up to 20 proposed tasks and writes
  nothing; `…/commit` creates them in one transaction, all or none, audited (`meeting_tasks_created`). A proposal gives
  a `title`, an `excerpt`, the `outcome` (0-based position among the meeting's outcomes, which must be an `action`)
  and optionally `assignee_ref`, `due_on` and `points`. It is **dropped** with a named reason when the excerpt (at least
  8 characters, case and spacing ignored) is not in the minutes (`excerpt_not_in_minutes`), the outcome is missing or
  not an action or already has a task, the due date is more than 7 days before or 365 days after the meeting, the title
  or points are invalid, or the assignee is not already a person of the plan (on a live task or a RACI row of it).
  Preview returns each verdict and the counts; commit refuses the whole request (`422`, every reason listed) if any
  item is not ready. Each task is linked to its outcome in `meeting_tasks`, whose unique `(meeting, outcome)` means an
  outcome has at most one task, and `GET …/{mid}` shows `index` and `task_pid` on each outcome. A retry under the same
  `Idempotency-Key` (NFR-7) returns the first answer. Not built: matching a *name* in the minutes to a person (the
  service holds no names, only `person:`/`worker:` URNs, so the proposer resolves them; the ambiguous-assignee choice
  in the task text is therefore not built); creating risks or issues from minutes; a screen.
- **FR-16e14 Evidence behind a status update (BR-2).** `POST /api/plans/{pid}/status-updates` may carry up to 10
  `claims`, each a sentence (up to 500 characters) with up to 5 `sources`: `task:<pid>`, `risk:<pid>`, `issue:<pid>`,
  `milestone:<pid>`, `change_request:<pid>`, `cutover_decision:<pid>` or `metric:<key>`. Every source is checked on
  save: a record that is not a live record of this plan, an unknown kind or metric, or a malformed id makes the whole
  update `422` and nothing is stored. Sources are stored as ids and never as titles. When an update is read (the
  history, the latest, the response to the post) each source is resolved as it is now: a record gives its title; one
  that has since been deleted is `unavailable` with no title and no id; a metric gives its current value and its
  definition. `GET /api/status-metrics` publishes the definitions: `overdue` (open tasks due before today; due today is
  not overdue), `due_soon` (due today to 7 days ahead), `blocked` and `no_due_date`, over the plan's open tasks (a
  workflow state that is not done). `overdue` and `due_soon` are `null` (unknown, `known: false`), never zero, when
  the plan has open tasks and none has a due date. The service computes no health score: the status stays a person's
  call. Not built: a per-record visibility check beyond the plan's (the service has none for tasks and risks, so a
  reader of the plan can open every source in it); claims on the director pack's section; editing a posted update; a
  screen.
- **FR-16e15 Reverse references (BR-4).** `GET /api/plans/{pid}/references?kind=&pid=` lists the meetings (agenda items
  and outcomes) and the lessons that point at one task, risk, issue, milestone, change request or cutover decision of
  the plan, and a `count` that is the length of the two lists together (each list is capped at 100). A lesson may now carry an optional `link`
  (`{kind, pid}`), checked on save like a meeting's (a record of another plan or an unknown kind is `422`); a lesson
  recorded before has none and appears under nothing. The plan guard runs first, so a private plan the caller cannot
  read is `404`; then the target must be a live record of this plan, else `404`, so a wrong plan, an unknown kind and a
  missing record are indistinguishable. A withdrawn lesson and a deleted meeting are not listed; a cancelled meeting is
  (its `meeting_status` says so). Not built: references from comments, tasks or risks; a screen; setting a link on an
  existing lesson.
- **FR-16e16 Starter templates (BR-13).** Five templates ship with the service, in the template snapshot format and as
  code rather than rows: a stage-gated project (four stages, gates 0 to 5), a programme with three workstreams, a
  benefits-realisation review, a change-request process and an incident response. `GET /api/plan-templates` lists them
  after the stored templates with `"starter": true`; each is read with `GET /api/plan-templates/{pid}` and instantiated
  with `POST /api/plans/from-template/{pid}` like any template (a `start_date` is required: the dates are offsets).
  A starter has no people or identifiers and tasks start at the workflow's initial state. It cannot be deleted or
  revised (`422`), and a saved template cannot take its name (`409`, case-insensitive); to change one, instantiate it
  and save the result under another name. Changing a starter in a release changes later instantiations only. Not
  built: a starter per locale (the names and task titles are English); a task workflow of its own for any starter; a
  screen that marks them; hiding them for a deployment that does not want them.
- **FR-16f Bird's-eye visibility.** The challenge funnel
  (`/api/lifecycle`) reports every phase even at zero, with live and
  stalled counts, and items in an unresolvable phase counted separately.
  Per plan (`/api/plans/{pid}/lifecycle`), readiness for the next gate
  is a five-check **checklist** — each check reported with the count
  behind it — so "ready" means every check ran and passed.

### 6.4b Full-suite capabilities — service (§1.4–§1.6)

Committed by §1.4–§1.6; delivered in service 0.3.0 except where a
requirement says otherwise (§14). None enters the
matcher payload (§5.6); every mutation audits and emits its event
(§6.6). Types in §5.9, endpoints in §9.2b, tables in §10.6.

- **FR-26 Custom workflows.** CRUD workflow configurations
  (`/api/workflows`), deployment-wide or scoped to one plan, for tasks
  and for issues. Each state declares one of `todo` / `active` /
  `waiting` / `done`; **a state without a category is refused `422`**,
  never defaulted, so the board, burndown, timeline and every
  time-based-analysis figure stay computable over a custom vocabulary.
  A workflow must declare exactly one initial state and at least one
  `done` state, every transition must name declared states, and a state
  holding live work cannot be deleted. The built-in vocabularies become
  the default workflows, so a plan with nothing configured is unchanged.
  A task or issue move to a state its workflow does not permit from the
  current one is `422`, and — as today — a refused move writes **no**
  transition row.
- **FR-27 Objectives and key results.** CRUD key results under a goal
  (`/api/plans/{pid}/goals/{gid}/key-results`) and dated check-ins under
  a key result. Progress, objective score and plan score are **derived
  on read**, never stored. An objective with no measurable key result
  scores `null` / `unmeasured` and sorts last — never `0`.
  `start_value` is captured once and never recomputed. Check-in
  confidence is recorded and **never blended into the score**. Alignment
  rolls up through `parent_ref`; there is no second OKR hierarchy.
  Currency-valued key results compare within one currency only.
- **FR-28 Time tracking.** CRUD time entries against a plan and
  optionally a task (`/api/plans/{pid}/time-entries`), with roll-ups per
  task, per plan, and per assignee. Recorded effort is **never**
  substituted for elapsed time in a flow ratio, and is **never** an
  input to Smart Score (FR-16e) or to any Flow Framework metric
  (FR-31). Per-assignee roll-ups serve capacity, cost, "who should be
  asked about this", and **per-person utilization** (FR-35). Every
  roll-up is labelled as **asserted**, distinguishing it from the
  observed transition log — which matters more now that a per-person
  figure is published from it.
- **FR-29 Sprint ceremonies.** CRUD sprints and the four ceremonies
  (planning / daily / review / retrospective) with categorized notes.
  Planning writes a **commitment snapshot** of the task set at sprint
  start, so later scope change reads as change rather than as a moved
  goalpost. Daily blockers convert into issues (FR-14); `action` and
  `feedback` retrospective notes convert into tasks, so an improvement
  gets an owner. Sprint burndown and velocity stay **sprint-scoped and
  count-based**, and are neither derived from nor merged into the
  item-scoped Flow Framework metrics.
- **FR-30 Project phases.** A plan carries `phase` (§5.9.4) and a
  transition log. Advancement is **one step at a time** — a skip is
  `422`. A backward move is permitted and **records a reason**; only a
  silent backward move is refused. `GET /api/plans/{pid}/phase-history`
  reports the transitions and the duration spent in each phase, with
  every phase present even at zero. **Phase never gates an operational
  write**: tasks may be created in Initiating and issues raised in
  Closing, because refusing writes on that basis would teach operators
  to misreport the phase. No constraint is enforced between `phase`, the
  lifecycle funnel, and the gate stage (§1.5.1); divergence surfaces as
  a readiness finding (FR-16f), not as a refused write.
- **FR-31 Flow Distribution.** `GET /api/plans/{pid}/flow-distribution`
  reports the feature / defect / risk / debt mix of completed work over
  a window, per plan and rolled up across a containment subtree. Items
  are classified from records already held (§5.9.5); an unclassifiable
  item is reported as **`unclassified` and counted separately**, never
  folded into `feature`. Where a deployment declares an intended mix the
  gap is shown against it; absent one, the mix is reported without
  judgement. The other four Flow Framework metrics are **already
  delivered** under time-based-analysis vocabulary and are not
  reimplemented — Flow Time is cycle/lead time, Flow Velocity is
  throughput, Flow Efficiency is flow efficiency, Flow Load is WIP
  (§1.6).
- **FR-32 Automation breadth.** Extends FR-16c within its existing
  invariants: additional triggers (a field change, a **phase
  transition**, a date arriving, an SLE breach) and **more than one
  action per rule**, applied in declared order with each action's
  outcome logged separately. Unchanged and non-negotiable: a failing
  rule never undoes the operator's action, actions are applied without
  re-entering the engine so automations cannot cascade, and every firing
  is logged whether applied, skipped or failed.

### 6.4c Value realization and strategic performance — service

The realized-gains and strategic-performance metric set. Types in
§5.9.6, endpoints in §9.2c, tables in §10.6. **Every figure is derived
on read, never stored**, so it cannot drift from the evidence it claims
to summarize (the Smart Score rule, §6.4a).

Honest starting position: `benefits`, `budget_lines`, `risks`,
`allocations` and a basis-point ROI helper already exist (PPM-10/11/12,
2026-07-18). These requirements add the measurement layer plus the three
missing inputs — adoption, business-case targets, and stakeholder
sentiment.

- **FR-33 Realized gains.** `GET /api/plans/{pid}/value-realization`
  reports, per plan and rolled up across a containment subtree:
  - **Transformation ROI** = `(realized value − total investment) /
    total investment`, where investment is the plan's recorded costs
    (budget-line actuals, plus recorded effort where time tracking is
    configured). Reported as a ratio *and* its numerator and denominator,
    never a bare percentage.
  - **Value Realization Rate** = completed initiatives that delivered
    their projected value ÷ completed initiatives. **This is a count of
    initiatives**, deliberately distinct from Benefit Realization Rate
    (FR-34), which is a ratio of *value*. A portfolio can score 90% on
    one and 40% on the other, and that difference is the finding.
  - **Time to Value** = `approved_at` → the first `ValuePoint` flagged
    `is_first_measurable`. Reported as a distribution with percentiles
    across a cohort, never as a mean (the percentiles rule,
    [time-based-analysis.md](time-based-analysis.md) §7.1).
  - **Adoption Rate** = `active_users / target_users` from the latest
    snapshot, returned **with** its stored `definition` and
    `window_days`, and refused rather than computed where
    `target_users` is zero or absent.
  - **Performance to Business Case** = actual against each
    `BusinessCaseTarget`, per metric, with the promised date and the
    approval that set it.

  **Absent evidence is disclosed, never scored zero** — a plan with no
  value points reports `unrealized` with a reason and sorts last, not
  `0%`, which would read as measured failure. ROI is computed **within a
  single currency only** (no FX), the same restriction Smart Score
  already carries.

- **FR-34 Strategic performance metrics.** `GET /api/plans/{pid}/performance`
  and the portfolio-wide `GET /api/performance`, organized in the six
  dimensions the source framework uses:

  | Dimension | Metric | Formula | State |
  |---|---|---|---|
  | Strategic | Strategic Alignment Index | aligned deliverables ÷ strategic objectives × 100 | new; objectives exist (§5.9.2) |
  | Strategic | Benefit Realization Rate | realized ÷ planned benefits × 100 | substrate delivered (PPM-11) |
  | Financial | ROI | net gain ÷ investment × 100 | helper delivered (`roi_basis_points`) |
  | Financial | Cost Performance Index | earned value ÷ actual cost | new; needs a phased budget baseline |
  | Financial | Budget Variance | budgeted − actual | delivered (financial variance view) |
  | Financial | Net Present Value | Σ net cash flow ÷ (1 + discount rate)^t | new; discount rate is deployment config |
  | Schedule | Schedule Performance Index | earned value ÷ planned value | new; same baseline dependency |
  | Schedule | Time-to-Value · Lead Time | FR-33 · already delivered (TBA §6.1) | partly delivered |
  | People | Capacity Utilization | effective hours ÷ available capacity × 100 | new — **see the constraint below** |
  | Quality & Risk | Defect Density · Technical Debt | defects ÷ unit of output · open `tech_debt` register effort | partly delivered (risk register) |
  | Quality & Risk | Risk Exposure · Mitigation Effectiveness | Σ probability × impact · residual reduction | substrate delivered (PPM-12) |
  | Stakeholder | NPS · CSAT | standard instrument scoring | new (§5.9.6) |

  **SPI and CPI report `null` with a reason on a plan with no phased
  budget baseline**, never `1.0` — a project with no baseline is
  unmeasured, not on track. NPV's discount rate is deployment
  configuration and is echoed in the response, because an NPV whose rate
  is not visible is not reviewable.

- **FR-35 Utilization, including per person.** Report effective effort
  against available capacity at **plan, team, and individual** level
  (`/api/capacity/utilization?by=plan|team|person`). Effort comes from
  `time_entries` (FR-28); available capacity from `allocations` plus the
  deployment's working-time configuration.

  **This reverses a standing refusal, deliberately.**
  [time-based-analysis.md](time-based-analysis.md) §12.4 refused
  per-person measurement and §2.4 declined utilization; both were
  amended on 2026-08-25 to record this decision (§12.4a). The reversal
  is **scoped to utilization**: per-assignee **cycle time, throughput,
  and flow efficiency remain refused**, and no endpoint returns them.

  Five obligations make the figure honest rather than merely available,
  and each is testable:

  1. **The denominator is declared and returned with the number** —
     never assumed at 100%. The response carries effective hours,
     available capacity, and the configuration that produced it.
  2. **Non-working time is excluded from the denominator**, not counted
     as idle: leave, holiday and non-project duty are absence of
     capacity, not failure to use it.
  3. **Small denominators are suppressed** — below a configured floor of
     capacity-days in the window the figure is `null` with a reason, the
     same posture as the flow gauges' small-board suppression.
  4. **It is never the sole ranking key of a person list**, and ships
     beside its numerator, its denominator, and the same period's queue
     and wait figures — because a reading near 100% is a warning about
     the queue (§2.3 there), not an achievement.
  5. **Effort stays labelled as asserted** (FR-28, §5.9.3): a person can
     move this number by logging more hours or declaring less capacity,
     which the transition log's incidental collection protected against
     and a timesheet does not.

- **FR-38 Controls — the Controlling process (§5.9.8).** CRUD controls
  under a plan (`/api/plans/{pid}/controls`), record readings, and
  record the action a failing reading provokes. This gives the
  **Controlling** phase (§1.5) a mechanism rather than only a name, and
  implements the four process steps as records: **set a standard**
  (`ControlStandard`), **measure** (`ControlReading`), **compare**
  (the derived verdict and gap), **act** (`ControlAction`).

  **What a failing control may do is fixed by its timing**, and this is
  the requirement, not a convention:
  - **Feedforward** controls may **block** a write (`422` naming the
    control) — acting before the fact is their entire purpose.
  - **Concurrent** controls may **warn and escalate** but must **never**
    silently undo the operator's action, the invariant FR-16c already
    holds for automations.
  - **Feedback** controls may only **record**. The work they judge is
    finished, and a control that rewrites history is not a control.

  Rules: a control whose `source` names a metric the service does not
  produce is refused at **write** (`422`), never left permanently
  `Unmeasured`; `Unmeasured` is a third verdict and is excluded from
  pass rates rather than counted as either; a failing reading with
  neither an action nor an explicit `Accept` is reported as
  **unanswered**; actions convert into existing tasks and issues rather
  than a fifth work store; readings are **append-only** and audited.

- **FR-39 Control coverage.** `GET /api/plans/{pid}/controls/coverage`
  and the portfolio-wide `GET /api/controls/coverage` report what is
  **not** being controlled — the question a register exists to answer:
  controls that have **never produced a reading**, controls whose last
  reading is older than their cadence, phases with **no controls at
  all**, and failing readings still unanswered. Every phase and timing
  appears even at zero, the same honesty rule as the lifecycle funnel
  (FR-16f): an empty cell is a finding, not a row to omit.

- **FR-37 Total Project Control (§5.9.7).** Record TPC observations per
  plan (`/api/plans/{pid}/tpc`) and report the derived view:
  - **DIPP** = EMV ÷ CEC, in basis points. Above `10_000` (1.0) the
    remaining value exceeds the remaining cost.
  - **DIPP Progress Index** = actual ÷ baseline DIPP, a Postgres
    `GENERATED ALWAYS` column so it cannot disagree with its own
    numerator and denominator.
  - **Divergence check**: the stored `dipp` against EMV ÷ CEC, reported
    as a finding rather than resolved by preferring either.
  - **Triage ordering**: `GET /api/tpc` ranks plans by DIPP descending,
    which is the use Devaux intends — scarce resources go to the highest
    DIPP, and **sunk cost appears nowhere in the figure**.

  Rules: money in minor units, ratios in basis points, **no float**;
  `CEC = 0` yields `null` with a reason, never infinity; a **negative
  EMV is legitimate and is not clamped**, because a project that is
  worth less than nothing to finish is exactly what the metric exists to
  expose; comparisons and rankings are **within one currency only**;
  every figure is labelled **asserted**, being an estimate rather than
  an observation.

- **FR-36 Stakeholder satisfaction.** Record NPS / CSAT responses
  against a plan (`/api/plans/{pid}/satisfaction`) and report the
  standard aggregate with its **response count and response rate**. A
  score without them is not reportable — an NPS of 100 from two
  respondents is not a finding. Responses store a role, never an
  identity (§5.9.6).

### 6.4d Scheduling, cost, people and governance extensions — service

Delivered 2026-10-05 / 2026-10-06 (migrations `…_000040` – `…_000048`;
tasks PJ-2, PJ-5 – PJ-7, PJ-9, PJ-11, PJ-13, PL-1, PL-2, PL-6, RM-8b,
RM-12b in [`tasks.md`](../tasks.md)). Endpoints in §9; tables in §10.
Each is audited (§6.6). Money is integer minor units and never summed
across currencies.

- **FR-40 Assignments, scheduling rules and work contours (PJ-5, PJ-6).**
  `PUT /api/plans/{pid}/tasks/{t_pid}/assignment` sets units / work /
  duration under a rule (`fixed_units`, `fixed_duration`, `fixed_work`)
  and recomputes the third; an assignment needs an assignee.
  `PUT …/tasks/{t_pid}/contour` spreads work (`flat`, `front_loaded`,
  `back_loaded`, `bell`, `turtle`, `custom`). `GET …/work-profile` and
  `GET /api/work-profile` return work as day / week / month buckets,
  **derived on read** (task → plan → portfolio). Bounds are CHECKed in
  the database (`work_minutes` 0..17 520 000, `units_percent` 1..1000).
- **FR-41 Cost rates and labour cost (PJ-2).** Named rate tables per
  person **or** role (exactly one), each rate effective from a date and
  append-only; a day before the first rate has no rate (`null`, never
  zero). Material and fixed task costs with an accrual (`start`,
  `prorated`, `end`); `GET …/cost-plan` (planned cost by period, per
  currency); `POST …/labour-costs/record` snapshots the rate used so a
  later rate change never rewrites an actual. **Finance gate:** every
  route is admitted only for `access=admin` or `finance=cost_rates`
  (or authentication off); anyone else gets `404`, and no rate or cost
  appears in any other response. Earned value can take actual cost from
  recorded labour (PJ-2 follow-up).
- **FR-42 Hammock activities (PJ-7).** `POST …/tasks/{t_pid}/hammock`
  turns a task into a hammock (or back); `POST …/hammocks/{h_pid}/links`
  and `DELETE …/links/{member}` link it to the tasks or hammocks whose
  span it follows; `GET …/hammocks` lists each with its derived span.
  A hammock carries no duration, work, constraint or dependency (CHECK
  `tasks_hammock_is_derived`; a trigger refuses a dependency naming
  one), is never in the dependency network and cannot drive float; a
  link cycle or self-link is refused; no links or all-unscheduled
  members read as unscheduled, never a made-up span.
- **FR-43 What-if sandbox (PJ-9).** `POST /api/plans/{pid}/sandbox/preview`
  applies edits (move, duration, constraint, add / remove link) to an
  in-memory copy and returns dates, float, critical path and labour-cost
  impact; it writes nothing. `POST …/sandbox/commit` applies the same
  edits in one transaction, one audit entry per changed item, and needs
  `If-Match` with the preview's plan ETag (`409` if the plan changed;
  never merged). The resource-leveling apply goes through the same
  commit. Constraint permission, assignee masking and the finance gate
  apply to a preview as to a commit.
- **FR-44 Cross-plan task links (PJ-13).** A plan's critical-path
  network includes the tasks of other plans its tasks are linked to via
  `plan_dependencies`, so float is computed across the link (each node
  on its own plan's calendar); cross-plan edges are flagged
  `cross_plan`. The walk is bounded (8 other plans, 3000 tasks). A link
  to a plan the caller cannot read is omitted from the calculation and
  shown only as a restricted edge, with nothing about the far side.
- **FR-45 Private plans and the aggregate guard (RM-8, RM-8b).** A
  private plan answers `404` (existence not disclosed) to any request
  under `/api/plans/{pid}` from a non-member; lists, search and
  duplicate checks omit it. Every cross-plan aggregate loads through
  one shared `HiddenScope`, so a hidden plan contributes to no total,
  list, ranking or count; the admin-only bulk reads stay full and
  answer `403` to others. A failed lookup never fails open: when the
  guard cannot tell whether a plan is private it answers `503`
  (`unavailable`), and an aggregate hides what it could not check;
  either way the failure is logged and counted in
  `visibility_lookup_failed_total` (CQ-6).
- **FR-46 Uniform paging and task `include` (RM-12b).** Every `GET`
  answering `200` with a bare JSON array pages: `?limit=` (default and
  ceiling 500), `?offset=` (above 10 000 is `400`), headers
  `X-Total-Count` / `X-Limit` / `X-Offset`, body still the bare array.
  `GET /api/plans/{pid}/tasks/{t_pid}?include=` embeds sub-resources
  (`children`, `relations`, `transitions`, `raci`, `time_entries`); a name whose module is off is
  omitted with reason `module_disabled`, one naming people with reason
  `masked`, and an unknown name is `422` listing the allowed ones.
- **FR-47 Cancellable bulk jobs (PJ-11).** `bulk_jobs` carry `progress`
  (0-100, never decreasing, enforced by trigger) and the status
  `cancelled`. `POST /api/plans/bulk-jobs/{id}/cancel` is cooperative:
  a queued job is cancelled at once; a running job stops after its
  current row, keeping the rows already processed; cancelling a
  finished job is a no-op. Only the requester or an admin may cancel
  (others get the `404` of an unknown job).
- **FR-48 Approval processes (PL-1, PL-2).** Approval requests
  (`POST /api/approvals`, `…/{pid}/decide`, `…/{pid}/withdraw`) apply to
  any subject. A **process** is a named list of up to 20 steps
  (`approve`, `todo`, `notify`); each edit adds an immutable version and
  an execution is pinned to the version it started on. Steps run one at
  a time; a rejection ends the execution `rejected` and skips and logs
  every later step; `cancel` does the same. State is in the database
  (execution status `running`, `completed`, `rejected`, `cancelled`,
  `failed`) with an append-only log; `resume` reconciles an execution
  with its approval after a restart.
- **FR-49 Controlled documents (PL-6).** Versions move `draft` ->
  `in_review` -> `approved` -> `effective` -> `superseded`. Review is an
  approval request on the version, so only the named approver approves;
  `make-effective` requires that approval to have been approved and, in
  the same step, supersedes the prior effective version naming its
  successor. Only a draft's text may change (an edit after that is the
  next draft). Exactly one effective and at most one in-flight version
  per document; immutability is enforced by a database trigger.
  `GET /api/documents/{pid}/in-force` returns the effective version.
- **FR-50 Onboarding, offboarding and handover.** Cases of kind
  `person_join`, `person_leave`, `plan_intake`, `plan_closeout` start
  from the kind's checklist (built-in or a custom template per kind);
  due dates are the effective date plus an item offset; an item is
  `done` or `skipped` (with a note) and a case completes only when none
  is open; one open case per (plan, kind, subject). Access grants and
  revocations are an append-only record (this service records access;
  it cannot change an identity provider). `GET …/closeout` lists what
  stands between a plan and closing (open tasks, risks, live bookings,
  unrevoked access, open cases). `POST …/handover/preview` lists a
  person's live holdings (unfinished tasks, bookings, RACI roles, open
  risks) and what each becomes; `…/handover/commit` applies it in one
  transaction and needs `If-Match` with the preview's handover ETag
  (`409` if holdings changed). Anything naming a person needs the
  permission to manage people.
- **FR-51 Announcements.** `GET /api/announcements` is the live feed for
  anyone; publish / edit / retract need `access=admin` or
  `comms=publish` (otherwise `403`); severity `info`, `important`,
  `urgent`; optional expiry after publication; `?include=all`
  (publishers only) also lists scheduled and expired ones.
- **FR-52 Ways of working and governance frameworks.** Every plan can follow a
  **ways of working** (how the team works: `methodology`, `cadence_days`, `wip_limit`,
  `definition_of_ready`, `definition_of_done`, `ceremonies`) and a **governance
  framework** (how the work is governed: `decision_model`, `review_cadence_days`,
  `reporting_cadence_days`, `escalation_path`, `decision_rights`, `forums`), each as a
  named, reusable record. A record is organisation-wide, or owned by one plan
  (`plan_pid` on create). `POST/GET /api/ways-of-working` and
  `/api/governance-frameworks`, `GET/PUT/DELETE …/{pid}` (and a POST delete twin).
  A plan links to one record per family with `PUT /api/plans/{pid}/ways-of-working`
  or `…/governance-framework` (`{framework_pid}`, null to unlink), and reads what is
  **in force** with `GET` on the same path: its own link, else the **nearest
  ancestor's** (`source: inherited`, with `from_plan_pid`), else `none` with a reason,
  never invented defaults. The inheritance is resolved on read and never stored, so
  changing a programme's link changes every project beneath it that has none of its
  own. An unknown field is `422`, not dropped; a name is unique per family and owner
  (`409`); a record plans still follow cannot be deleted (`409`). A record owned by a
  plan the caller cannot read is `404` and absent from lists, and an inherited record
  from such a plan is withheld with a reason. **Versions and who may change one (CF-4).** An edit
  (`PUT …/{pid}`) is a new **version**: it appends to the record's history (`GET …/{pid}/versions`, newest
  first, each with the plans following it) and never changes an earlier one. A plan's link **pins** the version it
  follows (the record's current one when it links, or the `version` it names) and keeps it, with the escalation
  path and every other field of that version, until it is moved by linking again; the answer says
  `version`, `latest_version` and `behind`. So one plan's edit, or a programme's, does not change the plans that
  follow it. Existing records became version 1 and existing links were pinned to 1. Creating, editing or deleting an
  **organisation-wide** record needs `access=admin` or `framework=edit` (policy area `shared_framework`, which a
  deployment's policy file cannot widen), otherwise `403`; a record a plan owns follows that plan's write access.
  Not built: a notice to the plans that are behind; moving all of them at once; a diff between versions.
- **FR-53 Tolerances.** Every plan can set how far it may drift on five dimensions before it must be
  referred upward, and sees how much of each limit is used as **red / amber / green**.
  `PUT /api/plans/{pid}/tolerances/{dimension}` (`time`, `schedule`, `cost`, `risk`, `resources`; body
  `{limit, amber_from_percent?, note?}`), `DELETE` or `POST …/delete` to remove it, `GET /api/plans/{pid}/tolerances`
  for the five in force, and `GET /api/tolerances?status=&dimension=` for every readable plan with a tolerance,
  worst first. A plan's limit is its own, else the nearest ancestor's (`source: inherited`; resolved on read,
  as for FR-52). **Measured, never entered:** `time` is the calendar days the critical-path forecast finish
  is after the plan's `target_date`; `schedule` is the calendar days that forecast finish is after the finish
  recorded in the plan's latest schedule baseline (`unknown`, never green, when there is none); `cost` is the basis points the estimate at completion is over the latest
  approved baseline; `risk` is the highest probability x impact among open and mitigating risks; `resources` is
  the percentage points above 100 that the plan's named, committed people are booked at worst over the next 90 days
  across the plans the caller can read. **Colour:** green below `amber_from_percent` (default 75) of the limit,
  amber from there up to it, red beyond it; zero or favourable is green; a limit that could not be measured is
  `unknown` with a reason and is never green (absent evidence is not good news); `overall` is the worst, with
  unknown ranking above green; `not_set` when there is no limit here or above. A zero limit means zero tolerance.
- **FR-54 Project escalations.** A project can refer something upward. `POST /api/plans/{pid}/escalations`
  raises one (`title`, `reason_kind` of tolerance, risk, issue, dependency, decision, resource or other,
  `tolerance_dimension` only for a tolerance, `level`, `escalated_to`, `owner_ref`, `respond_by`) at a **level of the
  escalation path of the governance framework in force for the plan** (FR-52: its own, else the nearest ancestor's):
  the level defaults to the path's first, a level not on the path is `422`, `escalated_to` defaults to the body the
  path names, and with no declared path any level 1 to 9 is allowed. It is `open`, then `acknowledged`; **raising
  it a level** (`…/raise-level`, with a reason of at least 10 characters) moves to the next level the path declares and
  back to `open`; it ends `resolved` (`…/resolve`, an outcome of decided, guidance_given or rejected and a written
  response of at least 10 characters) or `withdrawn`. A finished escalation accepts nothing more (`409`). Every step is
  kept in an append-only history that a database trigger protects. `GET /api/plans/{pid}/escalations` (`?status=` incl.
  `live`), `GET …/{epid}` with the history, and `GET /api/escalations` across the plans the caller can read, with
  `counts` and `overdue` (a live escalation past its `respond_by`). **Tolerances (FR-53) point here:** each dimension
  reports its `open_escalations`, and a red one with none is `escalation_needed`. `POST /api/plans/{pid}/tolerances/escalate`
  raises one `tolerance` escalation at the path's first level for every such dimension (asking again raises nothing new).
  A person asking is one way; the other is the task `cargo loco task tolerance_escalations` (schedule it daily), which
  does the same for every live plan as the identity `system:tolerance-escalations`, marks each description "Raised
  automatically", and skips a dimension that has a live escalation or one raised in the last 7 days, so an answered
  escalation on a plan that stays red is not raised again at once. Nothing runs the task unless the operator schedules
  it. The person it names as responsible (`owner_ref`) and the person who raised it are told in their in-app
  inbox (kind `escalation`, which a person can switch off) when it is raised, acknowledged, taken up a level, resolved
  or withdrawn, never the person who acted; no email or push is sent.
  **Who may acknowledge or resolve (CF-4).** Two separate rights, on top of write access to the plan: to
  acknowledge, `access=admin` or `escalation=acknowledge` (policy area `escalation_acknowledgement`); to resolve,
  `access=admin` or `escalation=resolve` (area `escalation_resolution`). Neither implies the other, a deployment's
  policy file cannot widen them (the service puts these rules first, like `finance` and `comms`), and anyone else
  gets `403`. **The acknowledgement and the resolution are resources**: `GET`/`PUT
  /api/plans/{pid}/escalations/{epid}/acknowledgement` and `…/resolution`. `PUT` is the same act as the older
  `POST …/acknowledge` and `…/resolve` (same rights, same history event, same audit entry carrying the level and
  note, or the outcome and response), and is idempotent: repeating it answers the record already there and writes
  nothing. A resolution is a record and is never overwritten: a different outcome or response is `409`. Taking an
  escalation up a level sends it back to `open`, so the acknowledgement of the level below is `404` until the new
  level acknowledges. Not built: restricting who may act to the members of the body the escalation is addressed to
  (`escalated_to` is free text with no link to people); this is a right held by an attribute, not by a seat.

### 6.5 Cross-service links & bulk — service

- **FR-17 Cross-service links.** A plan / goal / task / issue can
  link to **any** index entity. The service ships the **write-side**:
  `POST`/`GET`/`DELETE /api/plans/{pid}/links` over an `entity_links`
  table and `linked` / `unlinked` events, per
  `agents/share/cross-service-linking.md` §4.
  Links are **never** a match signal (§7 there). The read-model
  aggregator (`link-graph-service`) is out of scope here (§2.3).
- **FR-18 Bulk import / export.** Async, job-based bulk per
  [`agents/share/bulk-import-export.md`](bulk-import-export.md):
  the five endpoints on `bg_pg`, JSONL / CSV / Parquet codecs,
  upsert-by-stable-key + dedupe-to-review, per-row error report, and
  export masking + audit, on the one plans collection.
  Portfolio-specific stable keys and CSV columns are in §9.6.

### 6.6 Auditability — service

- **FR-19** Audit log: a best-effort `audit_logs` row per create /
  update / delete / merge on a plan **and on every sub-resource
  write** (action + JSON snapshot + actor + timestamp), durable in
  Postgres; read at `/api/plans/audit/recent` and
  `/api/plans/{pid}/audit`. Per
  `agents/share/auditability.md`.
- **FR-20** Event streaming: a `PlanEvent`
  (`created`/`updated`/`deleted`/`merged`, plus sub-resource and
  `linked`/`unlinked` events) per write to an in-memory stream (MVP),
  read at `/api/plans/events/recent`; the durable bus
  (`agents/share/event-bus.md`) is
  roadmap (§15).

### 6.7 Security — service

- **FR-21** Offline PASETO v4 public token verification against the
  central auth-service's published Ed25519 key (embeds
  `authentication-verifier`, now a PASETO verifier); `whoami`
  protected; the audit / merge `actor` is stamped from the token when
  present. Blanket `/api/*` enforcement and paseto-keys-over-HTTP fetch are
  follow-ups (§13, awaiting the coordinated family SSO rollout). Auth
  source of truth (supersedes the RS256-JWT model):
  `agents/share/authentication-sessions.md`.

### 6.8 Operator UI — front-end

- **FR-22** List active plans at `/plans`; create at `/plans/new`;
  detail at `/plans/[pid]` (render, edit, delete, check-duplicates);
  edit at `/plans/[pid]/edit`. A plan detail rolls up its child plans.
- **FR-23** Sub-resource workspaces under `/plans/[pid]`: goals, tasks
  (board + list), issues; plus the timeline and burndown views (§9.3).
- **FR-24** Check-duplicates posts the current record and lists matches
  (name, score, confidence), excluding the record itself; a merge
  action initiates `POST /api/plans/merge` (roadmap leg of the
  front-end).
- **FR-25** The plan form edits the full thin DTO: an optional `kind`
  label selector, comma-list inputs for names / keywords / tags /
  sameAs, row editors for goals, identifiers, and relationships, an
  `EntityRef` picker for `owner_org_id` / `lead_ref`, and a
  parent-plan picker for `parent_ref`.

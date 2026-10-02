# Tasks: ideas from Leantime

Derived from [plan.md](plan.md) (research 2026-10-02). IDs are `LT-n` so they
do not collide with the numbered build-out in
[spec/13-tasks.md](spec/13-tasks.md); promote an item there (as `T-29…`) when
it is scheduled. Sizes: S ≈ a day, M ≈ a few days, L ≈ a week+. A box is ticked
only when an automated test or a clearly described manual check confirms the
acceptance criterion.

Convention carried over from T-28: absent evidence is `null` with a reason,
never zero; new endpoints are pinned in `src/openapi.rs`; every write is
audited; people stay references.

- [x] **LT-1 (L) — Strategy canvases.** **Service landed 2026-10-02**
  (`src/canvas.rs`, `src/controllers/canvas.rs`, migration
  `m20261002_000001_canvases`, 7 unit + 4 DB-gated request tests, all passing
  against Postgres). **Not built:** the front-end pages, and OpenAPI entries
  (the six routes are on the `KNOWN_UNDOCUMENTED` register in
  `src/openapi.rs`, to be documented). The `risk` and `goal` templates are
  this service's own box sets, not a copy of Leantime's. A generic canvas engine: a
  `canvases` table (plan-scoped or free-standing, `template` key) and
  `canvas_items` (`box` key, text, optional owner ref, `position`), with
  built-in templates Lean, Business Model, SWOT, risk, empathy map, logic
  model and goal canvas. Template = ordered box list; adding a canvas type is
  data, not a migration. A canvas item may be **promoted** to an idea, a risk
  or an objective (reusing the existing convert paths).
  **Acceptance:** each built-in template round-trips; an unknown `box` for a
  template is refused 422; promoting an item writes the target record and an
  audit row naming the source item; deleting a canvas soft-deletes its items.
  *Front-end:* `/canvases` and `/plans/[pid]/canvases`, English-first.

- [ ] **LT-2 (M) — Plan templates (blueprints).** `POST
  /api/plans/{pid}/save-as-template` and `POST /api/plans/from-template/{tpl}`.
  A template captures the plan's subtree shape, tasks (without assignees or
  dates), milestones as **offsets** from a start date, and the workflow
  reference. Instantiating takes a start date and rewrites offsets to dates.
  **Acceptance:** an instantiated tree has the template's shape with fresh
  pids; no person reference is copied; milestone dates equal start + offset;
  the template itself is unaffected by later edits to the source plan; every
  created plan goes through `create_and_emit` (events + audit not bypassed).
  *Depends on:* nothing; shares clone logic with T-8's import pipeline.

- [ ] **LT-3 (M) — "My work" personal task view.** **Priority 1 — about half
  the users are individual workers (owner, 2026-10-02).** Default landing page
  for a caller whose `view` attribute is not a PMO persona. `GET /api/me/tasks`
  bucketed `overdue / today / this_week / later / undated` in the **caller's**
  timezone, plus a quick-add (`POST /api/plans/{pid}/tasks` already exists;
  the front-end adds a one-line form). Assigned-to-me by `actor_ref` from the
  token `sub`.
  **Acceptance:** a task due 23:30 local appears under `today` for that user
  and `today` or `this_week` correctly for another timezone (pinned by a test
  with two zones); a user with no assignments gets empty buckets, not an
  error; unreadable plans' tasks are omitted, not shown masked.
  *Front-end:* `/me`, one focus list, no chrome. Pairs with LT-9.

- [x] **LT-4 (M) — Comments and reactions.** **Service landed 2026-10-02**
  (`src/discussion.rs`, `src/controllers/discussion.rs`, migration
  `m20261002_000002_comments_reactions`, 6 unit + 3 DB-gated request tests).
  **Deviation:** one generic surface (`/api/comments`, `/api/reactions` with
  `subject_kind` + `subject_pid`) instead of per-entity paths, so a new subject
  kind is code, not routes. **Not built:** front-end; OpenAPI entries (on the
  `KNOWN_UNDOCUMENTED` register); the author-only rule is proven by a unit
  test, not an end-to-end token test (request tests run with auth off).
  Notifications address an earlier commenter as `person:<sub>` only when the
  `sub` is a UUID — an assumption to confirm with the auth service.
  *Priority 2 (workers discuss their own tasks daily).* `comments` and `reactions`
  tables keyed `(subject_kind, subject_pid)`; endpoints under
  `/api/{subject}/{pid}/comments` and `/reactions`. Subject kinds: plan, task,
  risk, proposal, idea, canvas item. Reactions are a small closed set,
  one per `(subject, user, kind)`. A comment emits a notification to the
  subject's owner and prior commenters.
  **Acceptance:** a second identical reaction is idempotent (no duplicate
  row); a comment on a plan the caller cannot read is 403/404 like the plan
  itself; edit and delete are the author's, delete is a soft delete, and the
  edit history is kept; a notification fires once per recipient.
  *Reconcile with* the deferred `posts / comments` item in the API spec §9.

- [ ] **LT-5 (M) — File attachments.** `attachments` table
  (`subject_kind`, `subject_pid`, `artifact_id`, `filename`, `content_type`,
  `bytes`, `sha256`, `uploaded_by`) over the **artifact store from T-8 §7**.
  Upload, list, download, soft-delete; size and content-type allow-list.
  **Acceptance:** an upload outside the allow-list or over the cap is refused
  before it is stored; the stored `sha256` matches the bytes; a download is
  re-authorised against the *subject's* ABAC, not just the attachment's;
  UTF-8 filenames survive upload and download (Leantime shipped a bug here).
  *Blocked on:* T-8 step 1–2 (artifact store). Gate-review and risk evidence
  are the first consumers.

- [ ] **LT-6 (M) — Per-user webhooks and chat sinks.** Extend T-28m
  (`src/webhooks.rs`): a user registers their own destination for **their**
  notifications; and a `format` option renders the payload for Slack,
  Mattermost, Discord or Telegram instead of the family envelope.
  **Acceptance:** a personal webhook receives only that user's notifications;
  the signature scheme is unchanged (HMAC over the exact body sent); a
  per-format golden test pins each renderer; a destination that fails is
  retried and logged like a deployment webhook and never blocks the
  notification write; no secret is returned by any read endpoint.

- [ ] **LT-7 (S) — iCalendar feed.** `GET /api/calendar.ics?token=…`: milestones,
  sprint start/end, and task due dates the caller may read, as VEVENTs with
  stable `UID`s, so a calendar app updates instead of duplicating.
  Per-user revocable feed token (not the session token).
  **Acceptance:** the feed validates as RFC 5545; `UID`s are stable across
  two fetches; a revoked token returns 401; an item the caller cannot read is
  absent; all-day versus timed values use the correct `VALUE=DATE`.

- [ ] **LT-8 (M) — Wiki / documents** — **decide first.** Options:
  (a) build `pages` + `page_versions` with templates; (b) a `links` field that
  points plans at an external doc system; (c) refuse. Recommendation: **(b)**
  first. A registry that stores prose competes with the organisation's real
  docs tool, and links are cheap. Revisit (a) only if LT-1/LT-4 usage shows
  people pasting long text into descriptions.
  **Acceptance (for (b)):** a plan carries typed links (`kind`, `url`, label);
  non-`http(s)` schemes are refused; links are shown, never fetched
  server-side.

- [x] **LT-9 (S) — Timezone audit.** **Done 2026-10-02.** Findings:
  every stored instant is `timestamptz` written from `Utc::now()` (clean);
  no source file reads the process zone (clean, now enforced by
  `clippy.toml` and a test); the 20-odd "today" computations all use the UTC
  date, which is correct for portfolio-wide views and **wrong only for
  personal ones** (none existed; LT-3 is the first, via `src/tz.rs`);
  `engineering.rs` already converts with `to_utc()` before taking a date
  (clean); date-only fields are zone-free by design. **Gap found:** tasks
  had **no due date**, which LT-3 needs (added there). Doctrine in spec §7.x. Verify every stored timestamp is UTC
  (`timestamptz`) and that every *date bucket* (due today, overdue, sprint
  day, burndown day) is computed in an explicit zone, not the server's.
  Write down the doctrine in the spec and add a test that runs the date-bucket
  code under a non-UTC `TZ`.
  **Acceptance:** a short findings list (clean or fixed, per site); a test
  that fails if a bucket changes with the process `TZ`; the doctrine stated in
  `spec/07-non-functional-requirements.md`.
  *Why:* Leantime's 3.10.1 fixed timestamps stored in viewer-local time,
  midnight due-date buckets and "Invalid DateTime" in the calendar.

- [ ] **LT-10 (S) — Task relations.** `task_relations` (`from`, `kind` ∈
  `related | duplicates | blocks`, `to`); `blocks` reuses dependency cycle
  refusal. Shown on the task detail.
  **Acceptance:** self-relations and exact duplicates are refused; a `blocks`
  cycle is refused; deleting a task soft-deletes its relations; a relation to
  a task in an unreadable plan is hidden from the reader.

- [ ] **LT-11 (M) — Timesheet views and approval.** On the existing
  `time_entries`: `GET /api/timesheets?person=&week=` (a week grid by day and
  plan), `POST …/submit` and `…/approve|reject` per person-week, billable
  roll-up per plan. Approved weeks are locked against edit.
  **Acceptance:** totals equal the sum of entries (no rounding drift,
  integer minutes); an edit to an approved week is refused 409; a week with no
  entries reports `null` hours with a reason, not zero; billable and
  non-billable never merge into one number without both being shown.

- [ ] **LT-12 (M) — Neurodivergent-friendly UX pass.** **Priority 1, built
  alongside LT-3, not after it.** Audit the front-end
  against Leantime's stated goal: plain-language labels, one primary action
  per screen, predictable reading order, `prefers-reduced-motion` honoured,
  contrast at WCAG AA, a dyslexia-friendly font option, and a per-user
  "simple mode" that hides advanced panels. Extends T-28k's Playwright audit.
  **Acceptance:** an axe-core run over the 35 audited routes with zero
  serious/critical violations; reduced-motion disables the tour and splash
  animation; simple mode persists per user (reusing `saved_views` storage)
  and does not remove any capability, only its prominence.
  *Scope settled:* workers are about half the users, so simple mode is the
  default for a non-PMO persona and an opt-in toggle for everyone else.
  Success is measured on the worker path: a first-time worker reaches "what
  do I do today?" in one click from sign-in (a Playwright check on `/me`).

## Not taken (recorded so they are not re-proposed)

Plugin runtime and marketplace; JSON-RPC API; screen/webcam recording;
gamification; LDAP / OIDC / 2FA (authentication service, EV-2). Reasons in
[plan.md §4](plan.md).

---

# Tasks from Redmine

Derived from [plan.md](plan.md) "Second source: Redmine" (research 2026-10-02).
IDs are `RM-n`; same sizes, tick rule and conventions as above. Nothing here is
started.

- [ ] **RM-1 (L) — Typed custom fields.** `custom_field_defs` (`key`,
  `label`, `type` ∈ `text | date | bool | integer | list | multi_list`,
  `applies_to` ∈ `plan | task`, optional `kinds[]`, `required`, `options[]`)
  and values stored on the record's `data` JSON under a `custom` key, validated
  against the definition. Filterable on list endpoints and projectable in saved
  reports. **Never fed to the matcher** (the matchable / operational partition).
  **Acceptance:** a value of the wrong type is refused 422; a `required` field
  blocks create; a `list` value outside `options` is refused; deleting a
  definition keeps stored values readable (soft-delete the definition, never
  silently drop data); a definition scoped to `kind=Project` is rejected on a
  `Program`; filtering by a custom field returns exactly the matching rows.
  *Scope question:* plan.md "Open questions (Redmine)".

- [ ] **RM-2 (S) — Per-role workflow transitions.** Add an optional `roles[]`
  to `workflow_transitions`: when present, only a caller whose ABAC role
  attribute is listed may make that move. Empty ⇒ unconstrained, preserving
  T-15's opt-in constraint.
  **Acceptance:** a transition restricted to `lead` is refused 403 for another
  role and **writes no transition row**; the same move by `lead` succeeds; a
  workflow with no `roles` behaves exactly as before; the refusal names the
  allowed roles.

- [ ] **RM-3 (M) — Watchers.** `watchers` table keyed `(subject_kind,
  subject_pid, sub)`; `POST/DELETE /api/watchers`, `GET` a subject's watchers
  and `GET /api/me/watching`. A watcher is added to LT-4's `notify_targets`
  and to status-change notifications. Auto-watch on creating or being assigned
  a subject (opt-out per user).
  **Acceptance:** watching twice is idempotent; a watcher gets a notification
  on comment and on status change exactly once; the actor of a change is never
  notified of their own change; a watcher of a plan the caller can no longer
  read stops receiving notifications and sees nothing (record-level read check
  re-applied at delivery).

- [ ] **RM-5 (M) — Subtask roll-up audit and fill.** Verify what a parent
  derives from its children today (start, due, % done, estimate, spent time,
  priority). Where Redmine's rule is sound and we lack it, add it **derived on
  read** (never stored, so it cannot drift): start = earliest child start, due
  = latest child due, spent/estimated = sums, done ratio = estimate-weighted.
  **Acceptance:** a findings list (carried / missing per field); for each added
  field, a test that it equals the recomputation from children; a parent with
  no children reports `null` with a reason, not zero; a child with no estimate
  is excluded from the weighting and **named**, not treated as zero.

- [ ] **RM-6 (S) — Atom activity feeds.** `GET /api/feeds/activity.atom`,
  `…/plans/{pid}.atom`, `…/me.atom` with a per-user revocable feed token (shared
  with LT-7's iCal token). Entries from the existing activity events.
  **Acceptance:** valid Atom 1.0; entry ids are stable across fetches; a revoked
  token returns 401; entries the caller cannot read are absent; `If-None-Match`
  returns 304 when nothing changed (reuse the ETag approach of `at-a-glance`).

- [ ] **RM-7 (M) — Per-plan module toggles.** A `modules` set on a plan
  (`tasks`, `time`, `canvases`, `comments`, `governance`, `financials`,
  `risks`, …) with deployment defaults and a personal-plan preset (`tasks` +
  `time`). A disabled module's routes for that plan return 404 and its
  front-end panels disappear. Disabling never deletes data.
  **Acceptance:** with `risks` off, `GET /plans/{pid}/risks` is 404 and the
  data is intact when re-enabled; defaults apply to a plan with no explicit
  set; the toggle is audited; a plan created from the personal preset exposes
  only its modules. *Pairs with LT-12 simple mode.*

- [ ] **RM-8 (S) — Private plans and tasks.** A `visibility` of `private`
  limits a plan (and its tasks) to its members and admins, via the existing
  record-level ABAC rather than a second mechanism.
  **Acceptance:** a non-member gets 404, not 403, so existence is not
  disclosed; search, duplicate-check and rollups omit it for non-members; a
  member sees it everywhere; flipping to public is audited.

- [ ] **RM-9 (M) — Bulk edit, copy and move tasks.** `POST
  /api/tasks/bulk` with `{ids, set: {...}}`, `…/copy`, `…/move` between plans.
  Moving re-validates against the **target** plan's workflow; per-item results
  like T-8's report (`ok | refused` + reason), one failure never aborts.
  **Acceptance:** a move to a plan whose workflow lacks the task's status maps
  it to that workflow's initial state **and says so**; copy gets a fresh pid and
  no transition history; counts reconcile; every change is audited.

- [ ] **RM-10 (S) — Time reports.** Extend saved reports (T-28e) with a
  `collection: "time_entries"`, `group_by` ∈ `person | plan | category |
  billable | month`, integer minutes, CSV. Depends on T-28e's `group_by`
  (landed) and shares LT-11's views.
  **Acceptance:** group totals equal the sum of entries (no rounding drift);
  billable and non-billable never merge without both shown; a person with no
  entries in range reports `null`, not zero.

- [ ] **RM-11 (S) — Outbound email sink** — **decide first.** In-app only is a
  stated stance. If email is wanted: a sink in LT-6's framework (templates, a
  per-user opt-in, a digest option, no tracking pixels). Inbound email is
  refused (plan.md §10).
  **Acceptance (if built):** a user without an address or opt-in receives
  nothing and the notification still lands in-app; a failed send is retried and
  never blocks the write; the message contains no more than the in-app text.

- [ ] **RM-12 (S) — REST ergonomics.** `?include=` on plan and task GET to
  embed named sub-resources (`milestones`, `risks`, `time`, …); `total_count`,
  `limit`, `offset` present on every list.
  **Acceptance:** `include` of an unknown name is 422 listing the allowed
  names; an included sub-resource the caller cannot read is omitted, not
  masked; the OpenAPI document describes the parameter on each route.

## Amendment to LT-10 (task relations)

Adopt Redmine's behaviours as the acceptance set: `precedes` forbids a
successor start on or before the predecessor's end; `blocks` prevents closing
the blocked task while the blocker is open; `duplicates` closing the primary
closes the duplicate; `copied_from` is informational. Each refusal names the
relation that caused it.

## Not taken (Redmine)

Inbound email; admin impersonation header; SCM browsing; forums/news modules;
plugin and theme system; LDAP / self-registration modes. Reasons in
[plan.md §10](plan.md).

---

# Tasks from Platform Collective (Huly)

Derived from [plan.md](plan.md) "Third source: Platform Collective" (research
2026-10-02). IDs are `PL-n`; same sizes, tick rule and conventions as above.
Nothing here is started. **Ideas only — no code from the EPL-2.0 project.**

- [ ] **PL-1 (S) — Approval requests on any subject.** Extend `reviews` (or
  add a thin `approval_requests`) so the subject may be a task, budget line,
  scenario commit, canvas item or custom subject, with a `decision_comment`
  required on reject. Status `pending → approved | rejected | withdrawn`.
  **Acceptance:** an approver who is not named is refused 403; a reject without
  a comment is 422; a decided request cannot be decided again (409); the
  decision is audited with the comment; withdrawing writes no decision;
  record-level read check applies to the subject's owning plan.
  *Reconcile with* the existing review consensus (`/reviews/consensus`).

- [ ] **PL-2 (L) — Multi-step approval processes** — **decide after PL-1.** A
  `process` is an ordered list of steps (`approve`, `todo`, `notify`), each
  naming an approver / assignee; starting one creates an `execution` with a
  per-step `execution_log` (started, completed, skipped, failed). Triggered
  manually or by an automation action.
  **Acceptance:** a step runs only after its predecessor completes; a rejected
  approval ends the execution `rejected` and skips the rest, **logging each
  skipped step**; an execution is resumable after a restart (state in the
  database, not memory); the log is append-only; a process definition edited
  mid-run does not change a running execution (it is pinned by version).

- [ ] **PL-3 (M) — Live board updates (SSE).** `GET
  /api/plans/{pid}/live` as `text/event-stream`: task created / moved / edited,
  comment added, with the same record-level read check as the plan.
  **Acceptance:** a task move by one client appears on a second open stream;
  a caller who cannot read the plan gets 403 before any event; the stream
  resumes from `Last-Event-ID` without duplicating or dropping within the
  retained window; a slow client is dropped, not buffered without bound;
  nothing is sent for a soft-deleted plan.

- [ ] **PL-4 (M) — Personal planner.** `planned_slots` (`task_pid`, `sub`,
  `starts_at`, `ends_at`); `GET /api/me/plan?day=` returns the day's slots
  beside LT-3's due buckets; the iCal feed (LT-7) includes them.
  **Acceptance:** overlapping slots for one user are allowed but **reported**
  as a clash, not refused; a slot on a task the user can no longer read is
  omitted; slots are in UTC and rendered in the caller's zone (LT-9); a slot
  never changes the task's due date.

- [ ] **PL-5 (M) — Notification preferences.** `notification_prefs` keyed
  `(sub, kind, channel)` with `enabled` and an optional `digest` (`instant |
  daily`); an in-app inbox that groups by subject. Defaults preserve today's
  behaviour (everything on, in-app).
  **Acceptance:** a disabled kind writes no notification for that user and
  every other recipient is unaffected; a user with no preferences gets exactly
  the current behaviour (pinned by an existing test unchanged); a digest
  collapses N events into one entry listing all N; preferences are per-user
  and never readable by another user.
  *Enabler for* LT-4, RM-3, LT-6, RM-11.

- [ ] **PL-6 (L) — Controlled documents** — **decide with LT-8.** Versioned
  documents with `draft → in_review → approved → effective → superseded`,
  an effective date, and a mandatory approval (PL-1) before `effective`.
  **Acceptance:** a version once `effective` is immutable (an edit creates the
  next draft); exactly one version is effective at a date; superseding records
  the successor; a document with no approval cannot become effective; every
  transition is audited. Recommendation: do not build unless a regulator or
  auditor asks (`compliance/*` surfaces first).

- [ ] **PL-7 (M) — Guest access.** A `guest` principal scoped to named
  `(subject_kind, subject_pid)` grants, read-only by default, optionally
  comment-only (LT-4). Time-boxed (`expires_at`), revocable, listed per plan.
  **Acceptance:** a guest sees only granted subjects and gets 404 elsewhere;
  an expired or revoked grant stops working immediately; a guest never sees
  people references (the masked view); every grant and revoke is audited;
  guests cannot enumerate plans or search.

- [ ] **PL-8 (M) — Attachment text extraction for search.** After LT-5: extract
  text from PDF / DOCX / plain text on upload into the search index, scoped to
  the subject's ABAC.
  **Acceptance:** a phrase inside an uploaded PDF finds its plan for a reader
  who may read it and **not** for one who may not; an unparseable file is
  stored, flagged `unindexed`, and never fails the upload; extraction runs
  out of the request path; deleting the attachment removes its text from the
  index. *Privacy:* extracted text is personal data if the file is; it
  inherits the attachment's masking.

- [ ] **PL-10 (S) — Printable output.** `GET …/pack.pdf` for the board pack
  and a plan summary, server-rendered, deterministic for fixed data, carrying
  its `as_of`. Signing is **out of scope** until a signer is named (plan.md
  open question).
  **Acceptance:** two renders of unchanged data are byte-identical; the PDF
  states its `as_of` and the masking profile used; a caller denied the JSON
  pack is denied the PDF; masked fields do not appear in the PDF text layer.

- [ ] **PL-11 (S) — Components (task grouping within a plan).** `components`
  per plan (`name`, optional `lead_ref`) and an optional `component_pid` on a
  task; filter and group-by in lists and saved reports. Corroborated by
  Redmine's issue categories.
  **Acceptance:** deleting a component with tasks is refused 409 (or reassigns
  explicitly — never silently orphans); a task's component must belong to the
  same plan; group-by reports an `unassigned` bucket rather than dropping
  those tasks; counts reconcile with the unfiltered total.

## Amendments

- **LT-2 (plan templates):** a template **carries its child tasks** (Huly's
  `IssueTemplate` has `IssueTemplateChild`); acceptance gains "children are
  instantiated with fresh pids, preserving order and nesting, and none carry
  assignees or dates".
- **LT-11 (timesheets):** check Huly's `TimeReportDayType`; if it models
  overtime / holiday, add a `day_type` to time entries, sourced from the
  existing `non_working` periods. **Unverified**: confirm the enum values
  before designing.

## Not taken (Huly)

Microservice architecture and its five datastores; Yjs co-editing (until a
wiki exists and concurrency is a real complaint); CRM, HR, recruiting,
inventory, billing, video meetings; an AI assistant inside the service;
desktop app and presence. Reasons in [plan.md §16](plan.md).

# Tasks: ideas from Leantime

Derived from [plan.md](plan.md) (research 2026-10-02). Later sections add the
prefixes `RM-` (Redmine), `PL-` (Platform Collective), `SC-` (scheduling and
cost canon), `KO-`, `MS-`, `CR-` and `PJ-` (ProjectLibre); as of 2026-10-06
every checkbox below is ticked, and the "Not built" notes inside an entry are
the remaining gaps. IDs are `LT-n` so they
do not collide with the numbered build-out in
[spec/13-tasks.md](spec/13-tasks.md); promote an item there (as `T-29…`) when
it is scheduled. Sizes: S ≈ one business day, M ≈ a few business days, L ≈ a business week
or more. A box is ticked
only when an automated test or a clearly described manual check confirms the
acceptance criterion.

Convention carried over from T-28: absent evidence is `null` with a reason,
never zero; new endpoints are pinned in `src/openapi.rs`; every write is
audited; people stay references.

- [x] **LT-1 (L) — Strategy canvases.** **Service landed 2026-10-02**
  (`src/canvas.rs`, `src/controllers/canvas.rs`, migration
  `m20261002_000001_canvases`, 7 unit + 4 DB-gated request tests, all passing
  against Postgres). **OpenAPI documented 2026-10-02.** **Not built:** the front-end pages. The `risk` and `goal` templates are
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

- [x] **LT-2 (M) — Plan templates (blueprints).** **Landed 2026-10-02.** Service: `plan_templates` (migration `m20261003_000013_plan_templates`), `POST /api/plans/{pid}/save-as-template`, `POST /api/plans/from-template/{tpl}`, template list/get/delete (+ POST delete twin); `src/plan_templates.rs` (4 unit tests) + 1 DB-gated test. A template is a **self-contained snapshot** (the plan + readable descendants, their tasks, components, milestones, subtask nesting and each plan's own workflow; ≤200 plans / 2000 tasks / 6 levels). **No person or identity is copied** (lead, assignee, identifiers, same_as, code, alternate names are dropped); **every date is an offset** from a base (the root start date, else the earliest known date) and is re-derived from the `start_date` given — required when the template carries any; tasks restart at the workflow's **initial state** with no transitions; every plan goes through `create_and_emit` (events + audit as a single create). A descendant the caller cannot read is left out and counted. **Failure handling:** instantiation is not one transaction (each plan's outbox write is its own) — a partial failure soft-deletes what was created and reports the error. The template stands alone: editing the source afterwards changes nothing (asserted). **Not built:** custom-field values and relations are not captured; front-end. `POST
  /api/plans/{pid}/save-as-template` and `POST /api/plans/from-template/{tpl}`.
  A template captures the plan's subtree shape, tasks (without assignees or
  dates), milestones as **offsets** from a start date, and the workflow
  reference. Instantiating takes a start date and rewrites offsets to dates.
  **Acceptance:** an instantiated tree has the template's shape with fresh
  pids; no person reference is copied; milestone dates equal start + offset;
  the template itself is unaffected by later edits to the source plan; every
  created plan goes through `create_and_emit` (events + audit not bypassed).
  *Depends on:* nothing; shares clone logic with T-8's import pipeline.

- [x] **LT-3 (M) — "My work" personal task view.** **Landed 2026-10-02**
  (`src/my_work.rs`, `src/controllers/my_work.rs`, `GET /api/me/tasks?tz=`,
  migration `m20261002_000003_task_due_date` adding `tasks.due_date`; 4 unit
  tests + `tests/my_work.rs` end to end with real tokens, two zones). **Added
  because the audit found none:** tasks had no due date. **Front-end:** `/me`
  page (English-first), `PpmClient.myTasks`, a nav entry, a unit test, and the
  mobile-audit route list (the Playwright audit was **not run** here — no
  browser). A deployment makes `/me` a persona's landing page with `view=me`
  (T-28f); the default landing is unchanged. **Not built:** the one-line
  quick-add (it needs the caller's own `person:` URN in the browser, which the
  BFF does not expose yet). Matching assumes a token `sub` that is a UUID is
  addressable as `person:<sub>` / `worker:<sub>`; otherwise empty buckets with
  `reason: sub_not_addressable`. **Priority 1 — about half
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
  kind is code, not routes. **OpenAPI documented 2026-10-02.** **Not built:** front-end; the author-only rule is proven by a unit
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

- [x] **LT-5 (M) — File attachments.** **Landed 2026-10-03.** Service: the **artifact store** (`src/artifacts.rs`, 5 unit tests; local directory `PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR`, sharded random keys, a key can never escape the root) with its `artifacts` index (SHA-256, size, declared type, optional `expires_at`) and `attachments` (migrations `…_000015_artifacts`, `…_000016_attachments`); `POST /api/attachments` (the **raw body** + `Content-Type`, `?subject_kind=&subject_pid=&filename=`), list, **download**, remove (DELETE + POST twin). **Refused before anything is stored:** a type outside the allow-list (pdf, png, jpeg, gif, webp, txt, csv, markdown, json, docx, xlsx, pptx — never html, svg, archives, executables), an empty file, a file over the cap (10 MiB default) — the test asserts **zero bytes on disk** after each refusal. The stored SHA-256 equals the bytes' hash and is **re-checked on every download** (a tampered file is 500, never served). A download is **re-authorized on the subject** (ABAC + private plan), served `nosniff`, `private, no-store`, `attachment`, with an ASCII fallback **and** the RFC 5987 UTF-8 name — a non-Latin filename survives and is never doubled. Subjects: plan, task, risk, proposal, idea, canvas item, budget line, scenario. 1 DB-gated test. **Not built:** an expiry sweep (this is T-8's); front-end; multipart upload; the gate-review subject kind. `attachments` table
  (`subject_kind`, `subject_pid`, `artifact_id`, `filename`, `content_type`,
  `bytes`, `sha256`, `uploaded_by`) over the **artifact store from T-8 §7**.
  Upload, list, download, soft-delete; size and content-type allow-list.
  **Acceptance:** an upload outside the allow-list or over the cap is refused
  before it is stored; the stored `sha256` matches the bytes; a download is
  re-authorized against the *subject's* ABAC, not just the attachment's;
  UTF-8 filenames survive upload and download (Leantime shipped a bug here).
  *Blocked on:* T-8 step 1–2 (artifact store). Gate-review and risk evidence
  are the first consumers.

- [x] **LT-6 (M) — Per-user webhooks and chat sinks.** **Landed 2026-10-03.** Service: `notification_sinks` + `sink_deliveries` (migrations `…_000021`, `…_000022`), `POST/GET /api/me/notification-sinks`, remove (DELETE + POST twin), `…/{pid}/deliveries`, admin `POST /api/notification-sinks/dispatch`; a ticker (`PROJECT_PORTFOLIO_MANAGEMENT_SINKS_TICK_SECS`, 0 = off); `src/sinks.rs` (8 unit tests) + `tests/sinks.rs` end to end with a **real local HTTP receiver**. A person registers destinations for **their own** notifications: signed `webhook` (the family envelope + `X-Mxi-Event-Id` / `X-Mxi-Signature`), and chat formats `slack`, `mattermost`, `discord`, `telegram` (golden-pinned bodies carrying **only the notification's own text**). **Never in the notification's path:** `notify()` only *enqueues* (asserted: nothing is sent until dispatch); failures retry at 1/5/25/125 min then give up, keeping the last error, so a dead destination cannot delay or fail a notification. Per-channel preferences (`webhook`/`chat`/`email`) are honoured per kind; one person's notifications never reach another's sink. **Safe by construction (SSRF):** https to a **public** host only — loopback, private, link-local/metadata (169.254.169.254), CGNAT, unique-local, `.internal`/`.local` names and URLs with credentials are refused, the name is **re-resolved at send time** and refused if any answer is private, redirects are not followed; the destination is a **secret returned only masked**. **Residual risk, stated:** DNS rebinding between the check and the connection is not closed (the client resolves again). **Not built:** a front-end; a per-sink test-send button. Extend T-28m
  (`src/webhooks.rs`): a user registers their own destination for **their**
  notifications; and a `format` option renders the payload for Slack,
  Mattermost, Discord or Telegram instead of the family envelope.
  **Acceptance:** a personal webhook receives only that user's notifications;
  the signature scheme is unchanged (HMAC over the exact body sent); a
  per-format golden test pins each renderer; a destination that fails is
  retried and logged like a deployment webhook and never blocks the
  notification write; no secret is returned by any read endpoint.

- [x] **LT-7 (S) — iCalendar feed.** **Landed 2026-10-02.** Service: shared `feed_tokens` (migration `m20261003_000010_feed_tokens`; only the SHA-256 is stored, 244-bit token shown once), `POST /api/me/feed-token` + revoke, and `GET /api/feeds/calendar.ics?token=`: the owner's open **assigned** tasks with a due date plus the open milestones and sprint dates of those plans, as RFC 5545 all-day events (CRLF, 75-octet folding on char boundaries, escaped text, exclusive next-day DTEND, **stable UIDs**). `/api/feeds/` is exempt from the bearer guard (a calendar app cannot send one) and authenticates by token alone; revocation is immediate. `src/feeds.rs` (5 unit tests) + `tests/feeds.rs` end to end **with enforcement ON**. **Deviation:** scope is *your own work*, not *everything you may read* — a feed fetch carries no claims, so record-level ABAC cannot be evaluated for it. **Not built:** a per-user feed UI. `GET /api/calendar.ics?token=…`: milestones,
  sprint start/end, and task due dates the caller may read, as VEVENTs with
  stable `UID`s, so a calendar app updates instead of duplicating.
  Per-user revocable feed token (not the session token).
  **Acceptance:** the feed validates as RFC 5545; `UID`s are stable across
  two fetches; a revoked token returns 401; an item the caller cannot read is
  absent; all-day versus timed values use the correct `VALUE=DATE`.

- [x] **LT-8 (M) — Wiki / documents** **Landed 2026-10-03.** **Decision taken (the recommended option (b)): typed links, not a wiki.** `plan_links` (migration `m20261003_000020_plan_links`), `POST/GET /api/plans/{pid}/links`, remove (DELETE + POST twin); `src/plan_links.rs` (2 unit tests) + 1 DB-gated test. A plan links to where its real documents live (docs, repo, ticket, design, chat, other); the service stores the link and **never fetches or renders it**. **Only absolute http(s) URLs with a host** — never `javascript:`, `data:`, `file:`, a protocol-relative URL, or one with embedded credentials (asserted). One link per URL per plan; ≤100 per plan; audited. **Not built:** a wiki or controlled documents (PL-6 stays a separate, undecided item); front-end. — **decide first.** Options:
  (a) build `pages` + `page_versions` with templates; (b) a `links` field that
  points plans at an external doc system; (c) refuse. Recommendation: **(b)**
  first. A registry that stores prose competes with the organization's real
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

- [x] **LT-10 (S) — Task relations.** **Landed 2026-10-02.** Service: `task_relations` (migration `m20261003_000008_task_relations`), three routes under `/api/plans/{pid}/tasks/{t}/relations`; `src/task_relations.rs` (4 unit tests). Redmine's behaviours as the acceptance set, every refusal naming the relation: **`blocks`** stops the blocked task finishing while the blocker is open; **`precedes`** needs the successor due strictly after (checked at create **and** when either due date is edited); **`duplicates`** closes the duplicate (with a transition row and audit) when the primary finishes; `related` (symmetric) and `copied_from` are informational. `blocks`/`precedes` must stay acyclic; self and exact-duplicate relations refused; cross-plan allowed; a relation to a task in an unreadable plan is hidden and counted. 1 DB-gated test. **Deviation:** tasks have a due date, not a start date, so `precedes` is on due dates. **Not built:** front-end. `task_relations` (`from`, `kind` ∈
  `related | duplicates | blocks`, `to`); `blocks` reuses dependency cycle
  refusal. Shown on the task detail.
  **Acceptance:** self-relations and exact duplicates are refused; a `blocks`
  cycle is refused; deleting a task soft-deletes its relations; a relation to
  a task in an unreadable plan is hidden from the reader.

- [x] **LT-11 (M) — Timesheet views and approval.** **Landed 2026-10-02.** Service: `timesheet_weeks` (migration `m20261003_000012_timesheet_weeks`), `GET /api/timesheets?person=&week=` (week grid by day and plan), `POST /api/timesheets/submit` (your own week only; an empty week is 422; once), `POST /api/timesheets/decide` (approver only — `access=admin` or the `timesheet_approver` role — never on their own week; a rejection needs a comment; **approved is final**); `src/timesheets.rs` (5 unit tests) + `tests/timesheets.rs` end to end with an owner, an approver and a bystander. **Integer minutes; billable and non-billable are always carried apart**; an empty week is `null` + `no_entries`, never zero. An **approved week is locked**: a new time entry against it is 409 (the lock is in the time-entry create path). **Deviation:** there is no edit/delete route for time entries, so the lock covers *creation*; if edit/delete is added it must call `timesheets::week_is_locked`. **Not built:** front-end. On the existing
  `time_entries`: `GET /api/timesheets?person=&week=` (a week grid by day and
  plan), `POST …/submit` and `…/approve|reject` per person-week, billable
  roll-up per plan. Approved weeks are locked against edit.
  **Acceptance:** totals equal the sum of entries (no rounding drift,
  integer minutes); an edit to an approved week is refused 409; a week with no
  entries reports `null` hours with a reason, not zero; billable and
  non-billable never merge into one number without both being shown.

- [x] **LT-12 (M) — Neurodivergent-friendly UX pass.** **Landed 2026-10-05 (acceptance met; the two audit items were done on 2026-10-06 — see the follow-up below).** **Axe over every page route:** `tests/e2e/a11y.spec.ts` audits every page route (**55** when this landed, **64** on 2026-10-06), **enumerated from `src/routes` when the suite runs** (so a new page cannot go unaudited; the phone audit now shares that list), with zero serious or critical WCAG 2 A/AA violations. It found and fixed: the **muted-text and danger colours** failed 4.5:1 on some surfaces, and the **SVAR grid emits an invalid `aria-rowindex="0"`** (a third-party bug, repaired by a small action, `src/lib/aria-repair.ts`, that removes the invalid value now and on re-render); the phone audit then caught that my own new header controls overflowed a 390px screen (fixed: the header wraps). **Reduced motion:** one global `prefers-reduced-motion` rule switches every animation, transition and smooth scroll off, the embedded widgets included (browser-tested with a probe element, with and without the setting; the splash and tour have no animation of their own today, so this keeps it that way). **Dyslexia-friendly font:** a toggle on `/display` that applies OpenDyslexic (bundled, `@fontsource/opendyslexic`, downloaded only when switched on) with looser letter, word and line spacing — offered as an option, not as a claim that it helps everyone. **Simple view:** puts the advanced destinations behind a "More" link in the nav and a plan's workspace links behind "More tools" — **prominence, not capability: nothing is removed (a pure `splitNav` / `splitPlanLinks`, tested to drop none), and every page still works at its address.** It is on by default for the worker persona (the deployment's `view=me`), an opt-in for everyone else, and **an explicit choice always wins over that default** (with a "go back to the default for my role" button). **Persistence:** kept on the device always (so it works signed out) and, when signed in, as **one saved view of the user's own** (`route=/__preferences`, `name=display`) — reusing `saved_views` with no API change; saving to the account is best effort and says when it failed; on sign-in the account's choices are adopted (or the device's are kept there if the account has none). Unit tests: `preferences.ts` (the rules), the store (device, account, failure, adoption), `aria-repair`; browser tests: simple view toggles, drops none and persists across a reload; the plan page's More tools; the dyslexia font applies and persists; reduced motion; the worker's first screen. **Deviations, stated plainly:** the worker's "one click from sign-in" is covered by the landing rule (`view=me` lands on `/me`, unit-tested) and a browser check that `/me` shows today's tasks with no click — **a full sign-in flow is not browser-tested** (the authentication service is called server-side and cannot be stubbed from the browser); the persona default (`view=me`) is likewise unit-tested, not browser-tested; preferences appear in a user's "all my saved views" listing (it is just another saved view); the audit is Chromium, the default theme and a stubbed API, and axe finds only part of what a person using assistive technology would hit. **Follow-up 2026-10-06 — the two audit items, done as rules the code is held to.** **One primary action per screen:** at most one visible `button.primary` on a screen. A browser test checks every route as it first renders (all 62 pass; the landing page's repeated *Sign in* closing link was the only offender, now a secondary link), the page sources are held to at most one unconditional primary and none inside an `{#each}` (63 unit tests), and the multi-step pages say which step is primary now with `class:primary` (what-if, handover, merge, onboarding: tested in the populated states). Repeated per-row buttons (*Commit* on scenarios, *Promote* on proposals, *Convert* on ideas, *Save assignment* per task, *Done* per checklist item) are no longer primary, and about twenty action pages that had no primary now mark their one main action. **Plain-language labels:** an acronym never stands alone in a heading, button, label, table header, link or nav entry — it follows plain words (*Key measures (KPIs)*, *Who does what (RACI matrix)*, *Progress against budget (earned value)*, *Balance workloads (resource leveling)*) or is a `<Term>` carrying its gloss (`src/lib/glossary.ts`, `<abbr title>`); the twelve offenders found were fixed, and a unit test scans every page, component and the nav for the rule (102 tests, with a check that the scanner itself catches a bare acronym). Twelve jargon-titled pages now open with one plain sentence under the heading (`.lede`; tested). **Limits, stated plainly:** the checks are mechanical — they keep jargon from standing bare, they cannot judge whether a label is clear to a person, and **no one has tested the wording with real users**; only English labels in the sources were changed (the other locales keep their existing words); acronym glosses are `title` tooltips, which touch and keyboard users may not see, so the plain sentence under the heading carries the meaning; headings such as *Float*, *Baseline* and *Percentile* in running text and table cells are not covered; the primary-action test sees the first render and the populated states of the pages that have their own browser tests, not every state of every page; the 2.7:1 / 3.8:1 contrast fixes cover the shared colours, not every theme. Original spec: **Priority 1, built
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

- [x] **RM-1 (L) — Typed custom fields.** **Landed 2026-10-02.** Service: `custom_field_defs` + `custom_field_values` (a separate table, so a value can **never** reach the matcher), six routes under `/api/custom-fields`, six types validated and normalized (`src/custom_fields.rs`, 6 unit tests), kind scoping, atomic writes, equality/contains search, withdrawn definitions keep readable values. **Deviation:** a **required** field blocks **task** create (the `custom` map on the task body); plan create takes the bare matcher `Plan`, so a required plan field is reported as `missing_required` on read rather than blocking create. **Not built:** list-endpoint filtering inside `GET /api/plans` (use `/custom-fields/search`), saved-report projection, front-end. `custom_field_defs` (`key`,
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

- [x] **RM-2 (S) — Per-role workflow transitions.** **Landed 2026-10-02.** `workflow_transitions.roles` (migration `m20261003_000005_workflow_transition_roles`, empty = anyone); `workflow::may_transition_as` (3 unit tests); the move path refuses with 403 naming the allowed roles and writes **no** transition row. Roles come from the token's `role` attribute (a deployment-declared vocabulary, like `view`). `tests/workflow_roles.rs` end to end with real tokens. **Not built:** front-end display of the role on a transition. Add an optional `roles[]`
  to `workflow_transitions`: when present, only a caller whose ABAC role
  attribute is listed may make that move. Empty ⇒ unconstrained, preserving
  T-15's opt-in constraint.
  **Acceptance:** a transition restricted to `lead` is refused 403 for another
  role and **writes no transition row**; the same move by `lead` succeeds; a
  workflow with no `roles` behaves exactly as before; the refusal names the
  allowed roles.

- [x] **RM-3 (M) — Watchers.** **Landed 2026-10-02.** Service: `watchers` table (migration `m20261003_000006_watchers`), `POST/GET/DELETE /api/watchers` and `GET /api/me/watching`. A watcher joins a comment's targets and a task move's: each told once, **never the actor**, and never twice when also a prior commenter (`discussion::notify_targets`, now taking watchers). Auto-watch on creating a task, commenting, or being assigned; `auto_watch` off (a notification preference) stops it. Other watchers are never named. `tests/watchers.rs` end to end with three real users. **Deviation:** the read check is applied when following; at **delivery** it cannot be re-run for an absent user (their claims are not held), so the notification text is deliberately generic and the subject stays readable only under the reader's own ABAC. Status-change notification covers tasks; plan status changes are not wired. **Not built:** front-end. `watchers` table keyed `(subject_kind,
  subject_pid, sub)`; `POST/DELETE /api/watchers`, `GET` a subject's watchers
  and `GET /api/me/watching`. A watcher is added to LT-4's `notify_targets`
  and to status-change notifications. Auto-watch on creating or being assigned
  a subject (opt-out per user).
  **Acceptance:** watching twice is idempotent; a watcher gets a notification
  on comment and on status change exactly once; the actor of a change is never
  notified of their own change; a watcher of a plan the caller can no longer
  read stops receiving notifications and sees nothing (record-level read check
  re-applied at delivery).

- [x] **RM-5 (M) — Subtask roll-up audit and fill.** **Landed 2026-10-02.** **Audit finding:** before this, tasks had **no parent concept** (carried: none of start, due, % done, estimate, spent, priority rolled up). Added `tasks.parent_task_pid` (migration `m20261003_000009_task_parent`; same plan, no cycle, at most ten deep) and `GET /api/plans/{pid}/tasks/{t}/rollup`, **derived on read, never stored**: due = latest descendant due, estimate = summed points, progress = estimate-weighted basis points over the estimated descendants only, spent minutes across the family; no-children / no-estimates / no-dated-children are `null` with a reason, and unestimated children are **named** in `excluded_unestimated`, never counted as zero. `src/task_rollup.rs` (5 unit tests) + 1 DB-gated test. **Not built:** priority roll-up (tasks carry no priority), a start date (tasks have none), front-end. Verify what a parent
  derives from its children today (start, due, % done, estimate, spent time,
  priority). Where Redmine's rule is sound and we lack it, add it **derived on
  read** (never stored, so it cannot drift): start = earliest child start, due
  = latest child due, spent/estimated = sums, done ratio = estimate-weighted.
  **Acceptance:** a findings list (carried / missing per field); for each added
  field, a test that it equals the recomputation from children; a parent with
  no children reports `null` with a reason, not zero; a child with no estimate
  is excluded from the weighting and **named**, not treated as zero.

- [x] **RM-6 (S) — Atom activity feeds.** **Landed 2026-10-02.** Delivered with LT-7 (one `feed_tokens` mechanism): `GET /api/feeds/me.atom?token=` — the owner's notifications (newest 100), valid Atom 1.0, stable `urn:uuid` entry ids, **`If-None-Match` ⇒ 304** (an empty feed is epoch-stamped so it is byte-stable too — the conditional-GET test caught a clock-stamped bug). **Deviation:** the per-plan and portfolio activity feeds are **not built**: a token fetch carries no claims, so plan-level ABAC cannot be applied to them. `GET /api/feeds/activity.atom`,
  `…/plans/{pid}.atom`, `…/me.atom` with a per-user revocable feed token (shared
  with LT-7's iCal token). Entries from the existing activity events.
  **Acceptance:** valid Atom 1.0; entry ids are stable across fetches; a revoked
  token returns 401; entries the caller cannot read are absent; `If-None-Match`
  returns 304 when nothing changed (reuse the ETag approach of `at-a-glance`).

- [x] **RM-7 (M) — Per-plan module toggles.** **Landed 2026-10-03**
  (`src/modules.rs`, `src/controllers/modules.rs`, migration
  `m20261003_000001_plan_modules`; 7 unit + 3 DB-gated request tests, plus
  OpenAPI). Six modules (`tasks`, `time`, `risks`, `financials`, `milestones`,
  `governance`) own the plan-scoped path segments listed in `src/modules.rs`; a
  guard layer answers `404 module_disabled` for a disabled module on **any**
  method, so no handler can forget it. Stored in its own `plan_modules` table
  (no row ⇒ the deployment default, `PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES`,
  all modules when unset), which leaves the plan row and its integrity hash
  untouched. `PUT /api/plans/{pid}/modules` takes `modules`, `preset:
  "personal"` or `reset`; `POST /api/plans?modules_preset=personal` creates a
  plan already on the preset. **Not gated, by design:** portfolio-wide views
  (`at-a-glance`, `capacity`, reports) read across plans. **Front-end:** the plan page hides the links of disabled modules
  (`src/lib/modules.ts`, fail-open), 4 unit tests. **Not built:** a settings UI
  to change a plan's modules (the API and `PpmClient.setPlanModules` exist), and
  hiding inside the sub-pages themselves. A `modules` set on a plan
  (`tasks`, `time`, `canvases`, `comments`, `governance`, `financials`,
  `risks`, …) with deployment defaults and a personal-plan preset (`tasks` +
  `time`). A disabled module's routes for that plan return 404 and its
  front-end panels disappear. Disabling never deletes data.
  **Acceptance:** with `risks` off, `GET /plans/{pid}/risks` is 404 and the
  data is intact when re-enabled; defaults apply to a plan with no explicit
  set; the toggle is audited; a plan created from the personal preset exposes
  only its modules. *Pairs with LT-12 simple mode.*

- [x] **RM-8 (S) — Private plans and tasks.** **Landed 2026-10-02.** Service: `plan_access_settings` (migration `m20261003_000011_plan_access_settings`), `GET/PUT /api/plans/{pid}/access`; `src/plan_access.rs` (4 unit tests) + `tests/plan_access.rs` end to end with real tokens. A guard layer answers **404, never 403** to a non-member for `/api/plans/{pid}` and everything beneath it; plan **lists, duplicate checks**, discussion/watchers/approvals/custom fields (via the shared resolver), task relations and **My work** all omit or 404 a private plan for a non-member (My work counts it in `omitted_unreadable`); members and `access=admin` callers see it; making a plan private always adds the caller as a member; a private plan is **hidden from an anonymous caller (fail closed)**; changes are audited. **NOT COVERED — important:** cross-plan **aggregate views** (at-a-glance, executive, board, financials, reports, capacity, scenarios) still include a private plan's figures, so "private" hides a plan's *records*, not its contribution to portfolio totals (see **RM-8b**). **Search results** are filtered by the same hidden set but were not separately tested. **Lesson:** loco's `create_table` pluralizes table names — a singular name creates a different table than the SQL refers to; the scaffolder now asserts a plural name. A `visibility` of `private`
  limits a plan (and its tasks) to its members and admins, via the existing
  record-level ABAC rather than a second mechanism.
  **Acceptance:** a non-member gets 404, not 403, so existence is not
  disclosed; search, duplicate-check and rollups omit it for non-members; a
  member sees it everywhere; flipping to public is audited.

- [x] **RM-9 (M) — Bulk edit, copy and move tasks.** **Landed 2026-10-02.** Service: `POST /api/tasks/bulk`, `/copy`, `/move`; `src/task_bulk.rs` (5 unit tests) + 1 DB-gated test. **One result per requested task** (an unknown id never aborts the rest; duplicates collapse) and `requested = ok + refused` always. **edit** sets `assignee_ref` / `due_date` / `component_pid` only — `status` is refused as a bulk field because a status change must pass the workflow, WIP, role and blocker rules that belong to the board move; a due date that breaks a `precedes` relation refuses *that task*; assigning notifies and auto-watches. **copy**: fresh pid, the target workflow's initial status, no transition history, assignee only with `keep_assignee`, a `copied_from` relation, custom-field values carried. **move**: re-validated against the **target workflow** — a status it lacks maps to its initial state **and the result says so**; component/sprint/parent links cleared; a task with subtasks is refused. **Deviation:** earlier transition history keeps its original plan. **Not built:** bulk status change, front-end. `POST
  /api/tasks/bulk` with `{ids, set: {...}}`, `…/copy`, `…/move` between plans.
  Moving re-validates against the **target** plan's workflow; per-item results
  like T-8's report (`ok | refused` + reason), one failure never aborts.
  **Acceptance:** a move to a plan whose workflow lacks the task's status maps
  it to that workflow's initial state **and says so**; copy gets a fresh pid and
  no transition history; counts reconcile; every change is audited.

- [x] **RM-10 (S) — Time reports.** **Landed 2026-10-02.** Delivered with LT-11: `GET /api/time-reports?group_by=person|plan|category|billable|month&from=&to=&plan=&person=&format=csv`. Group totals equal the sum of entries (asserted); **billable and non-billable are separate groups and every row carries both halves**; no entries ⇒ `total_minutes: null` + `no_entries`; plans the caller cannot read are omitted and counted. **Deviation:** a dedicated endpoint rather than a `collection: time_entries` on saved reports (a saved report is a plan filter + projection; time is a different shape). Extend saved reports (T-28e) with a
  `collection: "time_entries"`, `group_by` ∈ `person | plan | category |
  billable | month`, integer minutes, CSV. Depends on T-28e's `group_by`
  (landed) and shares LT-11's views.
  **Acceptance:** group totals equal the sum of entries (no rounding drift);
  billable and non-billable never merge without both shown; a person with no
  entries in range reports `null`, not zero.

- [x] **RM-11 (S) — Outbound email sink** **Landed 2026-10-03.** Delivered with LT-6 (one sink framework): an `email` sink — **outbound only, inbound email stays refused** — sent through loco's mailer (`ctx.mailer`; the test profile's stub, asserted), from `PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM` (unset ⇒ the delivery fails with that reason and is never retried). The subject and body carry **only the notification's text** (a header-injection attempt in the address or text is neutralized, unit-tested), the channel is `email` in preferences so a user can switch it off per kind, and a person with no sink or no address receives nothing by email while the notification still lands in-app. **Decision taken:** built (the in-app-only stance is unchanged for anyone who registers no email sink). **Not built:** digest emails; a verified-address handshake (an address is trusted as given by its owner). — **decide first.** In-app only is a
  stated stance. If email is wanted: a sink in LT-6's framework (templates, a
  per-user opt-in, a digest option, no tracking pixels). Inbound email is
  refused (plan.md §10).
  **Acceptance (if built):** a user without an address or opt-in receives
  nothing and the notification still lands in-app; a failed send is retried and
  never blocks the write; the message contains no more than the in-app text.

- [x] **RM-12 (S) — REST ergonomics.** **Landed 2026-10-02.** Delivered the `?include=` half: `GET /api/plans/{pid}?include=milestones,risks,tasks,components,time_entries,budget_lines` returns `{plan, included, omitted}` (the same live rows each sub-route returns, ≤200 each); **without `include` the response is the bare plan, unchanged**; an unknown name is 422 listing the allowed ones; a disabled module's sub-resource is omitted **and listed** (`module_disabled`); a masked read pulls nothing operational (`masked`). `src/controllers/includes.rs` (3 unit tests) + 1 DB-gated test, OpenAPI parameter documented. **Not built (split out as RM-12b):** `include` on task GET, and `total_count`/`limit`/`offset` on **every** list (only some lists carry `X-Total-Count` headers today). `?include=` on plan and task GET to
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

- [x] **PL-1 (S) — Approval requests on any subject.** **Landed 2026-10-02.** Service: `approval_requests` (migration `m20261003_000007_approval_requests`), `POST/GET /api/approvals`, `/{pid}/decide`, `/{pid}/withdraw`; `src/approvals.rs` (5 unit tests); only the **named approver** decides, a decision is **final** (409), a rejection needs a comment (422), only the requester withdraws, and withdrawing writes no decision. Subjects are the comment kinds **plus budget lines and scenarios** (now also commentable/watchable). Both sides are notified (`approval_requested`, `approval_decided`) through notification preferences. `tests/approvals.rs` end to end with three real users. **Not built:** reconciliation with `/reviews/consensus` (separate mechanism), multi-approver, front-end. Extend `reviews` (or
  add a thin `approval_requests`) so the subject may be a task, budget line,
  scenario commit, canvas item or custom subject, with a `decision_comment`
  required on reject. Status `pending → approved | rejected | withdrawn`.
  **Acceptance:** an approver who is not named is refused 403; a reject without
  a comment is 422; a decided request cannot be decided again (409); the
  decision is audited with the comment; withdrawing writes no decision;
  record-level read check applies to the subject's owning plan.
  *Reconcile with* the existing review consensus (`/reviews/consensus`).

- [x] **PL-2 (L) — Multi-step approval processes** **Landed 2026-10-06.** Migration `…_000046_processes` (`processes`, immutable `process_versions`, `process_executions`, append-only `process_execution_logs`; `approval_requests` gain `process_execution_pid` / `process_step`), `src/processes.rs` (6 unit tests), `POST/GET/PUT/DELETE /api/processes`, `POST …/start`, `GET /api/process-executions[/{pid}]`, `POST …/steps/{i}/complete`, `…/cancel`, `…/resume`, and a hook in the PL-1 decide / withdraw. Steps are `approve` (a PL-1 approval request to the named person), `todo` (the named person completes it) and `notify` (completes at once). **Acceptance, as tested** (`tests/processes.rs`, five real identities): **a step runs only after its predecessor completes** (a later step's completion is 409, another person's is 403, a todo is the only step completed by hand); **a rejected approval ends the execution `rejected` and logs each skipped step** (`step_skipped` for every step after it, then `execution_rejected`), and cancelling or withdrawing the approval does the same; **state is in the database, not memory** — a decision applied but not followed (simulated by deciding the approval directly in SQL) is picked up by `resume`, which is idempotent (a second call changes nothing); **the log is append-only in the database itself** (a trigger refuses any UPDATE or DELETE, as it does for versions); **an edit mid-run does not change a run under way** — each edit is a new immutable version and an execution is pinned to the one it started on (a run on v1 keeps its four steps while a new run on v2 uses the new one). Only the named approver decides, only the named person completes a todo, only the starter or an admin cancels or resumes, only the definer or an admin edits; a run is readable by its starter, an admin, anyone a step names, and anyone who can read the subject. **Deviations, stated plainly:** **no automation action starts a process** — an automation runs with no person behind it, so an execution would have no starter to hold accountable (to cancel, to be told the outcome); that needs a decision (the rule's owner?) before it is built; a step is always one person (no groups or any-of approvals); `failed` is reached only when a step cannot start (its reason is logged); the run list shows your own runs unless you name a subject you can read; no front-end. — **decide after PL-1.** A
  `process` is an ordered list of steps (`approve`, `todo`, `notify`), each
  naming an approver / assignee; starting one creates an `execution` with a
  per-step `execution_log` (started, completed, skipped, failed). Triggered
  manually or by an automation action.
  **Acceptance:** a step runs only after its predecessor completes; a rejected
  approval ends the execution `rejected` and skips the rest, **logging each
  skipped step**; an execution is resumable after a restart (state in the
  database, not memory); the log is append-only; a process definition edited
  mid-run does not change a running execution (it is pinned by version).

- [x] **PL-3 (M) — Live board updates (SSE).** **Landed 2026-10-03.** Service: `GET /api/plans/{pid}/live` (`text/event-stream`); `src/live.rs` (3 unit tests) + 1 DB-gated test using the `limit`/`idle_ms` parameters to make the infinite stream finite. Events `task_created`, `task_moved`, `task_updated`, `comment_added` (identifiers and the changed fact only). Read-checked **once at connect** (ABAC + private plans; unknown plan 404 before any event). Resume with `Last-Event-ID` from a **bounded ring**; a gap larger than the ring ⇒ a `resync` event, never silence; a slow subscriber is told to `resync` and **disconnected** (bounded channel, no unbounded buffer); another plan's events are not delivered. **Limits, stated plainly:** the hub is **per process** (with several replicas an event reaches only clients on the replica that handled the write — the outbox relay is the extension); authorization is checked at connect, not per event; a soft-deleted plan is refused at connect but an open stream is not actively closed. New deps: `async-stream`, `futures-util`, tokio `sync`+`time`. **Not built:** front-end subscription. `GET
  /api/plans/{pid}/live` as `text/event-stream`: task created / moved / edited,
  comment added, with the same record-level read check as the plan.
  **Acceptance:** a task move by one client appears on a second open stream;
  a caller who cannot read the plan gets 403 before any event; the stream
  resumes from `Last-Event-ID` without duplicating or dropping within the
  retained window; a slow client is dropped, not buffered without bound;
  nothing is sent for a soft-deleted plan.

- [x] **PL-4 (M) — Personal planner.** **Landed 2026-10-03.** Service: `planned_slots` (migration `m20261003_000014_planned_slots`), `POST /api/me/planned-slots`, remove (DELETE + POST twin), `GET /api/me/plan?day=&tz=`; `src/planner.rs` (4 unit tests) + `tests/planner.rs` end to end with real tokens. A slot is UTC, ≤12h, private to its owner, and **never changes the task's due date** (asserted). Overlaps are **allowed and reported as clashes** (transitively; back-to-back is **not** a clash). The day window is built from **local midnights** so it is 23/25 hours across daylight saving (unit-tested on the real 2026 US changes); each slot is returned both in UTC and rendered in the caller's zone; `due_on_day` lists what the day still owes. A slot on an unreadable task is omitted and counted. The slots also appear in the **iCalendar feed** as timed UTC events (`DTSTART:…Z`). **Side finding:** the LT-9 guard test matched `chrono::LocalResult` as `chrono::Local` — it now matches whole identifiers only. **Not built:** front-end. `planned_slots` (`task_pid`, `sub`,
  `starts_at`, `ends_at`); `GET /api/me/plan?day=` returns the day's slots
  beside LT-3's due buckets; the iCal feed (LT-7) includes them.
  **Acceptance:** overlapping slots for one user are allowed but **reported**
  as a clash, not refused; a slot on a task the user can no longer read is
  omitted; slots are in UTC and rendered in the caller's zone (LT-9); a slot
  never changes the task's due date.

- [x] **PL-5 (M) — Notification preferences.** **Service landed 2026-10-02**
  (`src/notification_prefs.rs`, `src/controllers/notification_prefs.rs`,
  migration `m20261002_000004_notification_prefs`; 6 unit tests +
  `tests/notification_prefs.rs` end to end with real tokens). Suppression
  lives in `models::capabilities::notify` (now returns `Ok(None)` on opt-out),
  so **no call site can forget it**; the automation engine reports an opt-out
  as `skipped` (`recipient_opted_out`), not `failed`. Digest is applied **at
  read** (`GET /api/me/inbox?tz=`), grouping per (kind, subject, local day) and
  listing every member — nothing is delayed or dropped. **OpenAPI documented 2026-10-02.** **Not built:**
  front-end settings page; channels beyond `in_app` (LT-6 / RM-11 add them). A new `notify(...)` kind must be
  added to `notification_prefs::KINDS` or it cannot be switched off. `notification_prefs` keyed
  `(sub, kind, channel)` with `enabled` and an optional `digest` (`instant |
  daily`); an in-app inbox that groups by subject. Defaults preserve today's
  behaviour (everything on, in-app).
  **Acceptance:** a disabled kind writes no notification for that user and
  every other recipient is unaffected; a user with no preferences gets exactly
  the current behaviour (pinned by an existing test unchanged); a digest
  collapses N events into one entry listing all N; preferences are per-user
  and never readable by another user.
  *Enabler for* LT-4, RM-3, LT-6, RM-11.

- [x] **PL-6 (L) — Controlled documents** **Landed 2026-10-06.** Built because it was asked for, **not** because a regulator or auditor has (the recommendation stands: nothing in `compliance/*` surfaced a need). Migration `…_000047_controlled_documents` (`controlled_documents`, `document_versions`), `src/documents.rs` (6 unit tests), `POST/GET /api/documents`, `GET …/{pid}`, `…/in-force?on=`, `POST …/versions` (the next draft), `GET/PUT …/versions/{n}`, `POST …/submit`, `…/make-effective`; review is a **PL-1 approval request** on a new `document_version` subject kind, and the PL-1 decide / withdraw moves the version. **Acceptance, as tested** (`tests/controlled_documents.rs`, four real identities, plus raw SQL against the database): **a version once `effective` is immutable** — the API answers 409 and a trigger refuses an `UPDATE` of its text and a `DELETE` of it, and an edit is the **next draft** (a copy of the text to change); **exactly one version is effective** — a partial unique index refuses a second even when forced in SQL, and only one version is in flight at a time; **a version with no approved approval cannot become effective** — `make-effective` checks the approval row (approved, and about this version), so forcing the status flag in SQL while the approval is still pending is refused (422), and a CHECK refuses an approved/effective/superseded version with no approval at all; **superseding records the successor** (`superseded_by`, `superseded_on`, set in the same transaction as the new version going effective, and a superseded version cannot come back); **every transition is audited** (created, edited, submitted, approved, returned to draft, made effective, superseded — counted); a rejected or withdrawn review returns the version to draft; `in-force?on=` answers the one version in force on a date or `null`. A document of a private plan is 404 to someone outside it and not listed (RM-8b; part of the private-plan test). **Deviations, stated plainly:** a version becomes effective **today only** — no back-dating and no future-dating (a scheduled effective date would need the status to change with the clock, which nothing here does); making a version effective needs the document's creator or an admin, separate from the approver; no groups or multi-step approval of a document (a PL-2 process could drive it, but is not wired to); no diff between versions. **Front-end landed 2026-10-06:** `/documents` (list with the version in force, create) and `/documents/[pid]` (versions with status word and shape, edit a draft, one primary button per state — send for review / approve / bring into force today / start the next draft — a final version is read-only, and "what was in force on a date"); review, approval and coming into force act on the version in flight, not the one on screen; 9 unit and 3 e2e tests with axe. — **decide with LT-8.** Versioned
  documents with `draft → in_review → approved → effective → superseded`,
  an effective date, and a mandatory approval (PL-1) before `effective`.
  **Acceptance:** a version once `effective` is immutable (an edit creates the
  next draft); exactly one version is effective at a date; superseding records
  the successor; a document with no approval cannot become effective; every
  transition is audited. Recommendation: do not build unless a regulator or
  auditor asks (`compliance/*` surfaces first).

- [x] **PL-7 (M) — Guest access.** **Landed 2026-10-03.** Service: `guest_grants` (migration `m20261003_000019_guest_grants`), `POST/GET /api/plans/{pid}/guests`, `POST …/guests/{grant}/revoke`; `src/guests.rs` (4 unit tests) + `tests/guests.rs` end to end with real tokens (a granter, two guests). A **guest** is a token with `role=guest`; a guard layer confines them (the outermost plan guard): they may read only a plan they hold a **live grant** on — its **masked** record (no people references), milestones and components — and, with `comment` access, read/add comments on it, plus their own `/api/me` endpoints; **everything else is 404**, so they cannot enumerate plans, search, or read a report, and cannot write. Grants are `read` or `comment` (comment implies read), time-boxed, **revocable, and both take effect on the very next request** (asserted for revoke and for expiry). Audited. **Deviations, stated plainly:** grants are on **plans** only (the design said "named subjects"); **tasks, risks and allocations are not offered to guests at all** because they carry assignee/owner references — a guest-safe task view is future work; a guest cannot upload. **Not built:** a front-end. A `guest` principal scoped to named
  `(subject_kind, subject_pid)` grants, read-only by default, optionally
  comment-only (LT-4). Time-boxed (`expires_at`), revocable, listed per plan.
  **Acceptance:** a guest sees only granted subjects and gets 404 elsewhere;
  an expired or revoked grant stops working immediately; a guest never sees
  people references (the masked view); every grant and revoke is audited;
  guests cannot enumerate plans or search.

- [x] **PL-8 (M) — Attachment text extraction for search.** **Landed 2026-10-03.** Service: `attachment_texts` (migration `…_000023`), `src/extract.rs` (5 unit tests), `GET /api/attachments/search?q=` with a snippet, DB-gated test. Text is extracted **at upload, best-effort, never failing it**: plain text, CSV, Markdown and JSON as UTF-8, and `.docx`/`.xlsx`/`.pptx` from their XML parts (the `zip` crate, deflate only). **PDFs and images are not indexed** (no extractor, no OCR) and are recorded `unindexed` with the reason; the search response carries `unindexed_attachments` so no result is not mistaken for not there. Untrusted-file bounds: ≤500 zip entries, 5 MiB per entry, 20 MiB per archive, 1M characters kept (a zip-bomb test). Postgres full-text (GIN, `simple` config, no stemming). **Access follows the subject:** each candidate hit goes through the same `resolve_subject` check as reading the attachment, and one the caller may not read is dropped (no title, no count); soft-deleted attachments never match; guests are denied the route. **Deviations:** extraction runs inline in the upload (off the async threads) rather than in a worker; the hit cap is applied after the access filter over the 200 best candidates, so a caller who can read few files may see fewer than exist; no re-index of files uploaded before this shipped. **Not built:** PDF text, OCR, per-type stemming, the front-end. After LT-5: extract
  text from PDF / DOCX / plain text on upload into the search index, scoped to
  the subject's ABAC.
  **Acceptance:** a phrase inside an uploaded PDF finds its plan for a reader
  who may read it and **not** for one who may not; an unparseable file is
  stored, flagged `unindexed`, and never fails the upload; extraction runs
  out of the request path; deleting the attachment removes its text from the
  index. *Privacy:* extracted text is personal data if the file is; it
  inherits the attachment's masking.

- [x] **PL-10 (S) — Printable output.** **Landed 2026-10-03.** Service: `GET /api/board/pack.pdf?from=&to=&as_of=` and `GET /api/plans/{pid}/summary.pdf?as_of=`; a dependency-free PDF writer (`src/pdf.rs`, 5 unit tests: valid xref offsets, determinism, escaping so a plan name cannot inject PDF syntax, WinAnsi fallback, wrapping/pagination) + 1 DB-gated test. The board PDF is built from the **same function** as the JSON pack (`oversight::board_pack_body`, extracted) **behind the same oversight authorization** — a caller denied one is denied the other. The plan summary is rendered from the **masked** record and counts only: the test asserts the lead, assignee and risk-owner ids and the owner organization name **never appear in the text layer**. **Deterministic when stamped**: with `?as_of=` two renders are byte-identical (asserted); without it `as_of` is the render time. **Not signed** — signing needs a named signer and key policy (still an open question in `plan.md`). Text outside WinAnsi is written as `?`, not guessed. **Not built:** signing; a front-end link. `GET …/pack.pdf` for the board pack
  and a plan summary, server-rendered, deterministic for fixed data, carrying
  its `as_of`. Signing is **out of scope** until a signer is named (plan.md
  open question).
  **Acceptance:** two renders of unchanged data are byte-identical; the PDF
  states its `as_of` and the masking profile used; a caller denied the JSON
  pack is denied the PDF; masked fields do not appear in the PDF text layer.

- [x] **PL-11 (S) — Components (task grouping within a plan).** **Landed 2026-10-02.** Service: `components` table + `tasks.component_pid`, five routes under `/api/plans/{pid}/components`, `?component=<pid|unassigned>` task filter, and a per-component summary that always reconciles (every component, empty ones at zero, plus `unassigned`). Module `tasks` owns the route. 3 unit + 1 DB-gated test, OpenAPI. **Not built:** saved-report group-by on component (the summary endpoint covers the need); front-end. `components`
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

## Scheduling and cost-control canon (SC-n)

Source: the eight topics the owner listed on 2026-10-03; analysis and the
evidence base in [plan.md §19–§24](plan.md). Seven of the eight are one
dependency chain — **SC-1 (CPM) → SC-2 → SC-3 → SC-4** — and leveling, schedule
risk and the DCMA-style audit all consume float, so **SC-1 comes first**.
Money stays integer minor units and **per currency, never merged**; absent
evidence is `null` with a reason, never zero; every derived figure names the
formula and the baseline version it used; no claim of ANSI/EIA-748 or DCMA
compliance is made anywhere.

- [x] **SC-1 (L) — CPM engine: float, all relationship types, multiple paths.** **Landed 2026-10-03.** Pure engine `src/cpm.rs` (15 unit tests incl. the textbook network worked by hand, lag changing which chain is critical, each of FS/SS/FF/SF, a lead, equal chains both critical, near-critical, cycles, unscheduled reasons, DST-free dates, and a 200-network randomized property test that **found and fixed a real bug**: free float exceeding total float under a lead). `GET /api/plans/{pid}/cpm?near=`: early/late start and finish, **total and free float**, critical and near-critical, **every** critical chain, `pushed_days` (logic forcing a start later than planned), `unscheduled` with a reason, ignored edges listed, cycle ⇒ 422. **Fixes F-6**: the plan-level `critical_path` now counts lag (regression test `critical_path_counts_lag`), and `/schedule` gains a float-based `cpm` block over a plan's children. **Deviations, stated plainly:** the old `critical_path` field and `on_critical_path` flag **remain the longest dependent chain** (so the existing Gantt highlight and test are unchanged) — the `cpm` block is the float-based answer and **the one to trust when they differ** (they do differ whenever a successor has slack); the calendar is **seven-day** until SC-2, so durations are not yet business-day-aware; the container plan is a summary and is not a node; completed tasks are included and carry their status; `violations()` judges FS edges only. **Not built:** constraints and negative float (SC-2), the front-end Gantt highlight switch, a unified plan+task run.
  A pure module (`src/cpm.rs`): forward pass (early start / finish), backward
  pass (late start / finish), **total float** (LS − ES) and **free float**,
  relationship types **FS, SS, FF, SF** each with **lag and lead** (negative
  lag), and a `near_critical_days` threshold. Returns **every** critical chain
  (ties are all reported) and the near-critical set. Replaces the
  duration-only `critical_path()` in `src/visibility.rs`; `ScheduleEdge` gains a
  `kind` (default `fs`, so existing edges mean what they meant) and the
  dependency endpoints accept it. `GET /api/plans/{pid}/cpm?as_of=` returns per
  item `{early_start, early_finish, late_start, late_finish, total_float,
  free_float, critical}`; items with no usable dates are listed under
  `unscheduled` with a reason, never given a float of zero. **Fixes F-6**
  (lag was ignored by the critical path).
  **Acceptance:** a textbook network (golden test, numbers worked by hand in
  the test) gives the published ES/EF/LS/LF and floats; a 10-calendar-day lag lengthens
  the chain and can change which chain is critical (a test that fails on the
  old code); SS, FF and SF each behave per definition; two equal chains are
  both critical; total float ≥ free float ≥ 0 unless a constraint makes it
  negative (SC-2); a cycle cannot be created (existing check) and a cyclic
  input from import yields an error, not a hang; items without dates are
  `unscheduled`, not `float: 0`; the Gantt's critical highlight reads from this.

- [x] **SC-2 (M) — Working calendars and constraints.** **Landed 2026-10-03.** Migration `…_000026_work_calendars` (`work_calendars` + `tasks.constraint_kind`, `constraint_date`, `deadline`). **Calendars, per plan and per person (owner decision):** `GET/PUT /api/plans/{pid}/calendar` and `GET/PUT /api/me/calendar` (weekday mask `1111100` + holidays, validated, audited); a plan with none works seven days, so every stored plan is unchanged. The engine counts **business days** (Fri + 2 = Mon; a Monday holiday is skipped). **Per-person:** a task is scheduled on its plan's calendar and the assignee's **narrows it** (their weekly pattern, holidays and recorded absences from `non_working_periods`, intersected) — a 4-business-day task for a Mon–Wed person runs Mon, Tue, Wed, next Mon; lag and float stay in the plan's business days; an assignee sharing no business day with the plan is listed under `calendar_conflicts`, not hidden. **Constraints:** `snet`, `snlt`, `fnet`, `fnlt`, `mso`, `mfo` plus a soft `deadline`; `snlt`/`fnlt`/deadline bound the **late** dates so an unmeetable one gives **negative float** and never moves an early date (tested); `mso`/`mfo` are **hard**, override logic and a node pinned earlier than its predecessors allow is flagged `hard_conflict`. **Setting a hard constraint needs a reason and, with auth on, `access=admin` or `scheduling=hard_constraints`** (403 otherwise, tested end to end with real tokens); every change is audited with old, new and the reason. Engine tests: 14 new, incl. a 200-network property test over per-node calendars and constraints. **Deviations:** **`alap` is not supported** (it needs a shift-late pass and is refused with a message); the permission is a token-attribute check in the handler, **not a policy-file entry** — moving it into the ABAC policy is a follow-up; constraints are on tasks only (a plan node has none); the new calendars do not yet feed the capacity rollup (PJ-3); `/schedule`'s plan-level float block uses the plan calendar. **Not built:** the front-end calendar editor, `alap`. A plan **working
  calendar** (working weekdays, plus holidays; reuse `non_working_periods`
  where they apply), durations measured in **business days** by the engine
  (fixes F-7), and a per-task **constraint**: `asap` (default), `alap`,
  `snet`, `snlt`, `fnet`, `fnlt`, `mso`, `mfo`, with a separate soft
  **deadline** that **never moves an early date** but bounds the late dates,
  so a threatened deadline produces negative float (corrected 2026-10-03: the
  first wording said it changed no date, which cannot give negative float). `mso` / `mfo` are flagged **hard**. Setting a hard
  constraint needs a role (see plan.md Q) and is audited with a reason.
  **Acceptance:** a Friday-to-Monday task is 2 business days; a holiday inside
  a span is skipped; `fnlt` earlier than the early finish yields **negative
  total float** rather than silently shifting the date; `mso` overrides logic
  and is reported as a hard constraint on that item; a deadline never changes
  an early date; no calendar means 7-day behaviour, so every existing
  plan is unchanged (an existing test passes untouched); constraint and
  calendar changes are audited.

- [x] **SC-3 (M) — Schedule baselines.** **Landed 2026-10-03.** Migrations `…_000030_schedule_baselines` and `…_000031_schedule_baseline_tasks` (no `deleted_at`, no update path); `POST/GET /api/plans/{pid}/schedule-baselines` and `GET …/schedule-variance?baseline=`; `src/baselines.rs` (8 unit tests) + 2 DB tests. **Append-only:** no update or delete route exists (tested: PUT/DELETE are 404/405); a re-baseline is the next version with a **required reason** (≤500 chars) and who took it, and the earlier version stays comparable (tested: against v1 it still says 2 slipped, against v2 none); **bounded per plan** (11, `PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES`) — at the limit it is **409 "never overwritten"**, not a silent overwrite; refused 422 with nothing scheduled or a cycle. The snapshot is the **critical-path engine's early dates** at that moment, so baseline and variance share a footing; variance is in **business days of the plan's calendar**. **Standing is explicit:** slipped / pulled_in / on_baseline, **added — not slip**, **removed — kept with its baselined dates**, **unmeasurable — null with the reason, never zero** (tested, and the counts reconcile: matched+removed = baseline tasks, matched+added = current tasks). The plan's own finish movement is stated and can be **negative because a long task was deleted** (tested), which the page flags. With no baseline: `baseline: null, variance: null, reason: no_baseline`. Front-end: take a baseline, pick a version, an outline under each Gantt bar where it was baselined, a variance table with a marker and a sentence per row. **Deviations:** the baseline covers **tasks** (the plan-level network of children is not baselined); two baselines cannot be compared with each other, only each with now; the snapshot has no per-task actuals (SC-4). `schedule_baselines` (+ per-task rows),
  **append-only and versioned**, mirroring T-28b: snapshot every task's planned
  start, finish and working duration; a re-baseline is a new version **with a
  reason**; a bounded number kept per plan (default 11, as scheduling tools do).
  `GET /api/plans/{pid}/schedule-variance?baseline=<n>` gives start / finish /
  duration variance per task, plan finish slip, and counts of tasks slipped,
  pulled in and **added or removed since** the baseline.
  **Acceptance:** a baseline is immutable; a re-baseline keeps its predecessor
  and records who and why; a task deleted after baselining stays in the
  baseline and shows as `removed`; a task added later shows as `added`, not as
  slip; no baseline ⇒ variance `null` + `no_baseline`; variance sums reconcile
  with per-task rows; every figure names the baseline version used.

- [x] **SC-4 (M) — Earned value.** **Landed 2026-10-03.** Migration `…_000033_earned_value` (`earned_value_settings`; `tasks.percent_method`/`percent_manual`/`percent_note`); `GET /api/plans/{pid}/earned-value`, `PUT …/earned-value-settings`, `PUT …/tasks/{pid}/percent-complete`; `src/earned_value.rs` (16 unit tests) + a DB test + the front-end `/plans/[pid]/earned-value`. **PV** from the phased budget baseline (T-28b; the latest version per currency; **linear within a period**, the standard EVM convention, stated), **EV = BAC × physical percent complete**, **AC** = recorded budget-line actuals in that currency; **SV, CV, SPI, CPI and earned-schedule SPI(t)**. Integer throughout: money minor units, ratios basis points, `i128` intermediates. **Percent complete is physical, not elapsed time:** per plan default `zero_hundred` / `fifty_fifty` / `points` (weights by story points; a task with no points carries no weight and is **listed**), per task override including `manual` (**a number and a note are mandatory, the figure is flagged everywhere it appears, audited old→new**). **Tested:** EV never exceeds BAC (even for a nonsense percent); completing a task moves EV by exactly its weight (a 2-point task of 4 is half the budget); PV is whole periods plus a linear share, flat across a gap, never decreasing; **SPI(t) says a project that finished 100% on day 30 of a 20-day plan was late (0.67) when SPI says 1.00**; on-plan is exactly 1.00. **Undefined is `null` with a reason, never zero or 1.0:** no planned value yet, no actuals recorded, no planned value, and the plan-level absents `no_baseline` and `no_weighted_tasks`. **One block per currency**; budget lines in a currency with no baseline are listed in `unbaselined_currencies`, never added in. `/performance` now reports real SPI/CPI (basis points) instead of a permanent `no_baseline` (T-23's gap closed; the `no_baseline` path is unchanged for plans without one). **Deviations, stated plainly:** **measured as of today only** — actuals carry no date and intermediate task progress is not stored by date, so the past is not reconstructed (`as_of` is not a parameter; the period-by-period series is SC-5's S-curve); **AC comes from budget-line actuals only** — timesheets × a rate is not offered because cost rates do not exist (PJ-2), rather than offering an option that always answers null; planned value is **not aligned to the schedule baseline** (SC-3) — it follows the budget baseline's own periods; `/performance` with several currencies reports `several_currencies` and points to `/earned-value`. **Not built:** the S-curve and EAC/TCPI (SC-5). A per-task **percent-complete method**
  (`zero_hundred`, `fifty_fifty`, `points` — derived from `points` and `status`,
  and `manual` with a mandatory note and a `manual` flag shown wherever EV
  appears), a per-plan default, and the figures **PV** (from the phased budget
  baseline, T-28b, aligned to the schedule baseline), **EV**, **AC** (from the
  source the owner picks: timesheets × rate or budget-line actuals), **SV, CV,
  SPI, CPI**, and **earned-schedule SPI(t)** (SPI stays meaningful after a
  project is late). `GET /api/plans/{pid}/earned-value?as_of=`.
  **Acceptance:** EV never exceeds BAC; completing a task moves EV by exactly
  that task's weight; no baseline ⇒ `null` + `no_baseline` exactly as today;
  **never merged across currencies** (one block per currency); ratios in basis
  points and money in minor units, no float; a plan whose tasks all use
  `manual` is labelled so; PV = 0 gives SPI `null` + reason, not infinity;
  every figure carries `as_of` and the baseline version.

- [x] **SC-5 (M) — EVM analytics.** **Landed 2026-10-03.** Migration `…_000034_earned_value_snapshots`; `GET …/earned-value/forecast`, `…/series[?format=csv]`, `…/rollup`, `POST …/snapshot`; `src/evm.rs` (12 unit tests) + a DB test + front-end sections on `/plans/[pid]/earned-value`. **EAC by named formula** — `budget_rate` AC+(BAC−EV), `cpi` AC+(BAC−EV)/CPI, `cpi_spi` AC+(BAC−EV)/(CPI×SPI), `bottom_up` — each with **ETC and VAC**, each pinned against hand-worked numbers (BAC 10 000, EV 4 000, AC 5 000, CPI=SPI=0.80 → 11 000 / 12 500 / 14 375). **No variant is presented as the answer:** `recommended` is a *stated rule* (cost efficiency continues when the schedule is on track; both pressures when it is behind the amber line; the one-off reading when there is no CPI) and the others are returned beside it. **TCPI** to the budget and to the recommended forecast; above 1.00 it says the remaining work needs improved efficiency; a target already spent is `budget_exhausted`, **never infinity**; CPI/SPI of 0 or missing is `undefined_denominator`, never zero. **Traffic lights from per-deployment thresholds** (`PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS`, basis points; a malformed value falls back to the defaults and the response says the source). **S-curve:** the planned curve is the whole baseline; **earned value and actual cost appear only on the days a snapshot was recorded** (daily by the scheduler, or on request) and are null elsewhere — never joined or filled in. **Roll-up** over a plan and its descendants (≤10 levels, 500 plans), **per currency, never across**, SPI/CPI from the totals, **no CPI when some plan's cost is unknown**, EAC/VAC only when every plan has one; plans the caller cannot read are left out without a trace; plans with no figures are counted not guessed. **Deviations, stated plainly:** history **starts when snapshots start** (the past is not reconstructible; no backfill); `bottom_up` is offered **only when a TPC cost-estimate-to-complete exists** — the baseline's not-yet-elapsed periods is a schedule fallback, not someone re-estimating, so it is not passed off as one; the roll-up does one computation per plan (bounded at 500, no batching yet); TCPI thresholds and ratio thresholds share one env value. **Not built:** the S-curve spreadsheet link on the page (the CSV is in the API); aligning planned value to the schedule baseline. On top of SC-4: **EAC** by named variant
  (`AC + BAC − EV`; `AC + (BAC − EV)/CPI`; `AC + (BAC − EV)/(CPI × SPI)`; and
  the bottom-up ETC from T-28b), **ETC, VAC, TCPI** to BAC and to EAC, an
  **S-curve series** (PV, EV, AC per period) for charting and CSV, a portfolio
  roll-up **per currency**, and a RAG from configurable thresholds. The
  response says which EAC variant it used and why.
  **Acceptance:** each formula is pinned against hand-worked textbook numbers;
  CPI = 0 or SPI = 0 gives `null` + `undefined_denominator`; TCPI > 1 flags
  "needs improved efficiency", and a TCPI that cannot be met is stated, not
  hidden; the series sums to the totals; the roll-up never adds two currencies;
  RAG thresholds are per deployment and shown in the response.

- [x] **SC-6 (M) — Resource leveling (advisory).** **Landed 2026-10-04 (API).** `src/leveling.rs` (11 unit tests incl. a 150-network property test: finish never moves, no dependency broken, no negative float) + `GET /api/plans/{pid}/leveling[?allow_critical=&load_percent=]` and `POST …/leveling/apply` + a DB test. Advice only: the first over-allocated day is resolved by keeping tasks in priority order (cannot-move, then least float, then earliest start, then input order) and delaying the lowest-priority one left; a delay is accepted only if the re-run network has no new negative float or hard-constraint conflict and the finish is unchanged (unless `allow_critical`, which marks the move and states the finish impact). Started tasks and hard-constrained tasks never move; what cannot be fixed is listed with a reason, never forced. Apply needs `confirm=true`, recomputes and refuses (409) any move not in the current proposal, sets `start_date` through the bulk edit (which gained `start_date`) and writes one `task_leveled` audit entry per task. Callers who may not manage a person see `person-N`, no ref, no percentages. **Deviations, stated plainly:** a task has no per-task effort, so each unfinished assigned task asks `load_percent` (default 100) of its assignee per business day; this plan's own allocations are not counted (they describe the same work), other plans' committed ones are fixed background; only this plan's tasks move; delays are by one business day per step (greedy, deterministic, capped at 3 000 iterations, `truncated` says so); a delayed task's stored `start_date` is a lower bound. **Not built:** the front-end leveling diff (SC-11). Original spec: Detect over-allocation by
  period (day or week) from allocations and dated tasks, then **propose** a
  deterministic set of delays that stay **within total float** (SC-1), respect
  hard constraints (SC-2) and priority (higher priority keeps its date).
  Output is a proposal: each change with the finish-date effect; **apply is a
  separate, confirmed call** that goes through the bulk edit (RM-9) so it is
  audited. Unresolvable conflicts are reported, not forced.
  **Acceptance:** the same input always gives the same proposal; a proposal
  never violates a dependency or a hard constraint; it never moves a critical
  task without saying so and showing the finish impact; with nothing
  over-allocated the proposal is empty; applying records one audit entry per
  moved task; a person's allocation detail is masked for a caller who may not
  see people (the same masking as `/capacity`).

- [x] **SC-7 (M) — Portfolio resource management.** **Landed 2026-10-04 (API).** Migration `…_000035_resource_roles`; `src/portfolio_resources.rs` (8 unit tests) + `GET /api/resource-plan[?from&to&period=week|month&group_by=role|skill&scenario=]`, `GET …/resource-plan/hours`, `GET/PUT /api/resource-roles`, `POST /api/plans/{pid}/allocations/{a_pid}/resolve` + a DB test. Demand and supply are summed as integer **percent-days** and only then turned into **FTE** (percent-days / (100 × calendar days)); the gap is supply minus demand in FTE with **headcount a separate field**. A **placeholder** is `POST …/allocations {placeholder: true, role, percent}` (stored as `person_ref = placeholder:<uuid>`; role mandatory, no person), counts toward its role and is reported apart (`placeholder_fte`, plus a `placeholders` list to resolve); it **never enters per-person capacity** (`/capacity`, commit, leveling exclude it) or the skill-gap lookup. **Resolve** is a manager's call, 409 `over_capacity` with the numbers unless `confirm=true`, audited. **Demand with no role and people with no role are the `unclassified` row, never dropped**; a role nobody supplies has supply 0 and a negative gap. **Proposed bookings are a separate series** (PJ-4) and never enter demand, gap or totals. The per-plan series sums to the per-period totals (tested). **What-if:** `?scenario=` limits demand to a scenario's plans. **Hours:** planned = a named committed allocation's percent of 8 h on each Mon–Fri; actual = recorded time entries; a plan with none is `null` + `no_actuals`, never zero; time by someone with no allocation on that plan is **`unplanned_actuals`**, per person for a manager and per plan with a count for anyone else. A caller without `access=admin`/`role=resource_manager` sees **roles and totals only** (`masked: true`, no person refs). **Deviations, stated plainly:** there was no person→role registry, so supply needs `resource_roles` (one primary role per person; a person with several roles counts once, under the primary); days are **calendar days** on both sides (working calendars, SC-2, are not applied to supply); skills are **demand-only** and overlap (skills are resolved live from the worker service, never stored); the 8-hour day is a constant (`working_time_configs` is not consulted); actuals count recorded entries **whether or not the timesheet week is approved**; the scenario filter is by plan membership only (scenario proposals are not demand). **Not built:** the heatmap front-end (SC-11). Original spec: Demand vs capacity **by role
  or skill and period** across plans (a heatmap's data), **generic
  placeholders** (role-only demand with no person yet) that can be resolved to a
  named person, and a **planned vs actual hours** report from timesheets
  (LT-11). What-if through the existing scenarios. Independent of SC-1.
  **Acceptance:** per-period totals reconcile with the sum of per-plan
  allocations; a role gap is stated in **FTE** (never as a percentage mixed with
  headcount); unassigned demand is shown as its own row, never dropped; actual
  hours with no matching allocation are listed, not ignored; a caller who may
  not see people sees roles and totals only; absent timesheets give `null` +
  `no_actuals`, not zero hours.

- [x] **SC-8 (L) — Schedule-risk Monte Carlo.** **Landed 2026-10-04 (API).** Migration `…_000036_schedule_risk_inputs` (`tasks.duration_optimistic_days`/`duration_pessimistic_days`; `risks.task_pid`/`schedule_impact_days`/`schedule_probability_percent`); `src/schedule_risk.rs` (10 unit tests) + `GET /api/plans/{pid}/schedule-risk[?iterations&seed&distribution=triangular|pert&fallback_band_percent]`, `PUT …/tasks/{pid}/estimate`, `PUT …/risks/{pid}/schedule-impact` + a DB test. Each iteration draws whole business-day durations (triangular, or Beta-PERT weight 4), lets each register event strike with its probability, and **re-runs the full CPM analysis** (calendars, constraints and relationship types included). Output: **P10/P50/P80/P90 finish** (nearest rank), `cpm_finish_confidence_bp` (share finishing by the deterministic date), a **criticality index** per task (share of iterations with no float), a **sensitivity** correlation per task (null when either did not vary), a finish histogram. **Seeded** (default 1, echoed; SplitMix-scrambled xorshift64*): same seed ⇒ identical result (tested at unit and request level); with zero uncertainty **every percentile equals the CPM finish exactly** (tested, both distributions); percentiles are ordered; **iterations capped** (20 000; iterations × tasks ≤ 5 000 000) and refused, not truncated. **Assumptions echoed**, including `correlation: none` and the independence statement. A task with a likely duration but **no estimate uses the stated fallback** (± `fallback_band_percent`, default 20) and is marked `source=fallback` and listed — never treated as certain; an estimate not around the likely value is widened and flagged. A **risk event** needs a task, an impact and a probability and be open/mitigating; one missing a probability (or task, or impact, or closed, or on an unscheduled task) is **skipped and named with a reason**; risks with no schedule fields are only counted. **Deviations, stated plainly:** the register's 1–5 ordinal probability is **not** converted to a percent (that would be invented precision), so a schedule event needs its own `schedule_probability_percent`; a task's likely duration is its `duration_days`; durations are whole business days, so a sub-day spread rounds; correlation between tasks is not modelled (the response says the spread is therefore narrower than a correlated one); the simulation covers one plan's tasks (not the portfolio); events add business days to a task, they do not move a start. **Not built:** the risk histogram front-end (SC-11). Original spec: Optimistic / likely / pessimistic
  duration per task (triangular or PERT; or a single estimate plus a per-plan
  uncertainty band, **labelled as such**), the existing **risk register** as
  discrete events (probability × schedule impact on a named task), a seeded
  simulation over the CPM network (SC-1, SC-2), and outputs **P50 / P80 / P90
  finish**, a **criticality index** per task (share of iterations it was
  critical), and a sensitivity measure. Seeded, bounded and disclosed like
  TBA-11 (seed, iterations, assumptions echoed in the response).
  **Acceptance:** same seed ⇒ identical result; with zero uncertainty P50 equals
  the CPM finish exactly; P50 ≤ P80 ≤ P90; iterations are capped; the response
  states that **inputs are treated as independent** (no correlation); a task
  with no estimate is listed and uses the stated fallback, never silently zero;
  a risk event with no probability is skipped and named.

- [x] **SC-9 (M) — DCMA-style schedule health (14 checks).** **Landed 2026-10-05 (API).** `src/schedule_health.rs` (15 unit tests: a golden test per check on a small crafted network, plus cycle, nothing-incomplete and threshold-config cases) + `GET /api/plans/{pid}/schedule-health[?as_of=]` + a DB test. Fourteen pure checks, each `{count, of, value, threshold, pass, reason, offenders[], of_what, note}` with offenders listed by pid; named a **self-check** with a disclaimer in the response; preset `dcma-default` echoed with its thresholds; `PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULE_HEALTH_THRESHOLDS` (JSON) overrides any, a malformed or out-of-range value falls back to the whole preset and the response says the source. **A check that cannot be computed is `pass: null` + a reason and never a pass** (no baseline, a dependency cycle, nothing incomplete or due to measure); `all_pass` is true only when all fourteen were computed and passed. **Denominators are stated per check** (`of_what`): incomplete tasks; relationships into incomplete tasks; incomplete scheduled tasks (float); tasks baselined due by the status date (missed tasks, BEI). **Thresholds checked 2026-10-05** against two secondary descriptions ([schedulelens](https://schedulelens.com/blog/dcma-14-point-assessment/), [projectsanalytics](https://projectsanalytics.com/dcma-14-point-assessment/)) — **not the DCMA Program Analysis Pamphlet itself, which was not fetched**: missing logic ≤5%, leads 0%, lags ≤5%, FS ≥90%, hard constraints ≤5%, high float (>44 business days) ≤5%, negative float 0%, high duration (>44) ≤5%, invalid dates 0%, missed tasks ≤5%, CPLI ≥0.95, BEI ≥0.95 all agree. **Where the sources are silent or differ, the choice is in the check's `note`:** *resources* has no prescribed threshold, so zero unassigned tasks is this preset's own rule (a resource is an assignee); the *critical path test* adds 600 business days (the sources say only "a large duration"; one blog says one business day) to the last-finishing critical task and expects the finish to move at least that; *hard constraints* are must-start-on, must-finish-on, start-no-later-than and finish-no-later-than; the BEI denominator is tasks baselined due, not "plus tasks missing baseline dates" as one source words it. **Deviations:** missing logic exempts only the project's own start and finish; CPLI uses the least total float of incomplete tasks and counts from the status date; baseline checks use the plan's latest schedule baseline (SC-3) and tasks only; the status date defaults to today, and `as_of` makes a run reproducible. **Not built:** the scorecard front-end (SC-11). Original spec: `GET
  /api/plans/{pid}/schedule-health` runs the 14 checks as **pure functions**,
  each returning `{value, threshold, pass, offenders[]}` so a scheduler can fix
  what failed: missing logic, leads, lags, relationship-type mix (FS share),
  hard constraints, high float, negative float, high duration, invalid dates,
  missing resources, missed tasks, a critical-path test, the critical-path
  length index (CPLI) and the baseline execution index (BEI). Thresholds are
  the commonly published ones as a **default preset**, configurable. Named a
  *self-check*, never "DCMA compliant".
  **Acceptance:** each check has a golden test on a small crafted network; the
  **denominator is incomplete tasks** where the check says so; a check that
  cannot be computed (BEI or variance with no baseline, float with no CPM
  inputs) reports `null` + a reason and is **never a pass**; offenders are
  listed by pid; **before the thresholds are pinned in tests they are checked
  against the current DCMA guide** (they were not fetched when this was
  written); the preset name appears in the response.

- [x] **SC-10 (S) — Float for the worker.** **Landed 2026-10-05.** `GET /api/me/tasks` adds, per task, `schedule: {total_float, critical, late}` (the CPM total float of **that task** in business days) or `null` + `schedule_reason` (`no_duration`, `no_anchor`, `cycle`, `no_schedule_data`) — **never a zero for a task with no schedule data**. Only the caller's own tasks' figures are read out of each plan's analysis (no chain, nothing about anyone else's task, tested), and it uses the plans the caller can already read, so **a worker with no plan-level view still sees their own float**. Front-end: `/me` shows "On the critical path", "N business days of float", or "This is already late for the plan (N business days behind)." — ordinary text a screen reader reads with the row, with a decorative ⚠ (aria-hidden) and colour as extras, and **nothing at all** when there is no schedule data (`src/lib/my-work-view.ts`, 5 unit tests). Extended `tests/my_work.rs` end to end with real tokens (critical, float, negative float, unscheduled, another person's task absent). **Deviations:** float is relative to the plan's own project finish, so in a plan with several independent chains every task finishing with the project reads as critical; the sentence is English (the page's other labels are English-first); one critical-path analysis per readable plan per request (no cache). Original spec: In *My work* (LT-3), a task shows
  "on the critical path" or "N business days of float", with a plain sentence for a
  negative float ("this is already late for the plan"). Dependent on SC-1.
  **Acceptance:** a task with no schedule data shows nothing rather than "0
  business days"; the figure is the CPM total float for the caller's task and nothing
  about other people's; a worker with no plan-level view still sees their own
  tasks' float; text is reachable by screen reader.

- [x] **SC-11 (M) — Front-end for the scheduling canon.** A schedule-health
  scorecard (offenders link to the item), a float / critical column in the Lily
  Gantt table and the SVAR chart, an S-curve with the baseline, a leveling diff
  preview with an explicit apply, a resource-demand heatmap, and a risk
  histogram with P50 / P80 / P90 marked. Each lands with its service task.
  **Acceptance:** each view states its `as_of`, baseline version and
  assumptions; colour is never the only signal; every chart has a data table
  twin; undefined figures read "not enough data" with the reason.
  **Partly landed 2026-10-03:** the float/critical **Gantt** with float tails and a negative-float limit marker, the **table twin**, the **network diagram**, the **working-calendar editors** (`/plans/{pid}/calendar`, `/me/calendar`) and a schedule editor that shows *what moved* — all with a symbol and a sentence besides colour. **Completed 2026-10-05.** The S-curve (with its table twin, `as_of` and baseline version) had landed with SC-5. Now also: `/plans/{pid}/schedule-health` (the fourteen checks; pass / fail / **"Not computable" with "Not enough data: <reason>", never a pass**; offenders linked to the task on the critical-path page, which now anchors its rows; the self-check disclaimer, `as_of`, baseline version and preset shown); `/plans/{pid}/leveling` (the proposal as a before-and-after table with the finish effect, what is over-allocated, what could not be fixed and why, then **an explicit apply: tick the moves, confirm, give a reason, apply** — the button stays disabled until confirmed and sends only the ticked moves); `/resources` (the demand-against-supply **heatmap whose cells say "Short 0.5 FTE" / "Spare 1 FTE" / "Balanced" with a symbol — shading only shows size — plus a table of every figure**, skill rows flagged as overlapping demand-only, masked callers told they see roles and totals only); `/plans/{pid}/schedule-risk` (the **histogram with P50, P80, P90 and the plan's own date marked by labelled lines of different dash styles**, a table twin, the percentile table, per-task criticality and sensitivity, fallback tasks marked, risks used and skipped with reasons, and every assumption including independence). Pure helpers in `src/lib/scheduling-view.ts` (13 unit tests). Plan page links to each; `/resources` is in the nav. **Browser audit:** `tests/e2e/scheduling.spec.ts` runs axe (WCAG 2 A/AA, serious and critical) over every new page plus milestones, KPIs and OKRs over a stubbed API, with behaviour checks (not-computable is not a pass; leveling needs the confirm and sends only ticked moves; the P labels and table twins exist). **It found two real defects, both fixed globally:** links inside running text relied on colour alone (now underlined), and the muted-text colour was 4.3:1 against a 4.5:1 minimum (darkened). **Deviations, stated plainly:** the audit covers the new pages in Chromium over a stubbed API and the default theme only (other themes and the older pages were not audited), and axe finds only part of what a person using assistive technology would; the pages are English-first; the heatmap shading uses two hues plus strength and is meaningful only with its words; the S-curve was not changed.

## KPIs and OKRs (KO-n)

Requested 2026-10-05: "add kpis and okrs". The OKR engine (objectives, key results, check-ins, `/plans/{pid}/okr`) already existed with no front-end and no portfolio view, edit or delete; KPIs did not exist.

- [x] **KO-1 (M) — KPIs: standing measures with dated readings.** **Landed 2026-10-05.** Migration `…_000037_kpis` (`kpis`, `kpi_readings`); `src/kpi.rs` (7 unit tests) + `POST/GET /api/kpis`, `GET/PUT/DELETE /api/kpis/{pid}`, `POST/GET /api/kpis/{pid}/readings` + a DB test. A KPI has a name, unit, direction (`higher_better`/`lower_better`), `target_value`, `red_value` (the line beyond which it is red; **refused 422 unless strictly on the worse side of the target**, also by a database check), optional cadence, owner, plan (none = portfolio) and objective. **Status is derived on read, never stored:** green at or better than the target, red at or worse than the red line, amber between, **`no_data` with no reading — not red and not green**; a real zero is a reading. Also derived: a direction-aware `trend` (last two readings), `stale` (a cadence KPI with no reading within twice its interval; null with no cadence), and a `summary` over every visible KPI whatever the status filter. Readings are **append-only**; a correction is a newer reading for the same day (the earlier is marked `superseded`); a future date is refused. The direction and the plan are **not editable** (it would re-colour history) and sending one is refused. KPIs on plans the caller cannot read are left out. Values are integers (scale them). **Deviations:** no per-reading edit or delete; stale uses fixed interval lengths (monthly 31 calendar days, quarterly 92); a KPI belongs to one plan or none.
- [x] **KO-2 (S) — OKR gaps in the API.** **Landed 2026-10-05.** `GET /api/okrs[?period=]` (every objective with each key result's derived progress or the reason it has none, its latest check-in with confidence, and the objective score; **least progressed first, unmeasured last and never zero**); `PUT/DELETE /api/objectives/{pid}`; `PUT/DELETE /api/key-results/{pid}` — the baseline (`start_value`), metric, direction and currency are **not editable** and sending one is refused; the target may not equal the baseline; every change audited with old and new. A deleted objective no longer counts toward a plan's `/okr` score. Confidence is still never blended into a score.
- [x] **KO-3 (M) — Front-end for KPIs and OKRs.** **Landed 2026-10-05.** `/okrs` (objectives with score and progress bars, key results with progress or the reason, check-ins with confidence marked "not part of the score", add / edit / delete) and `/kpis` (summary by status, status filter, define a KPI, add a reading, history, edit, delete); nav entries; `src/lib/okr-view.ts` and `kpi-view.ts` (unit-tested). Status is a **word and a shape, never colour alone**; "Not measured" and "No data yet" are their own states. English-first like the other recent pages. **Not built:** a browser (axe/Playwright) audit, KPI sparklines, linking a KPI from a plan page, and a KPI roll-up per plan.

## Milestones as a concept (MS-n)

Requested 2026-10-05: "add concept: milestones". Plans already had milestone records (name, due, done, kind) with create, list, reschedule and complete, but no portfolio view, no link to the schedule, no completion rules and no slip history. All four were asked for.

- [x] **MS-1 (M) — Portfolio view and link to the schedule.** **Landed 2026-10-05.** Migration `…_000038_milestone_links` (`milestones.completion_mode`/`done_reason`; `milestone_tasks`; `milestone_slips`); `src/milestones.rs` (6 unit tests); `GET /api/milestones[?plan&status&kind&from&to&due_soon_days&at_risk_days&as_of]`, `GET /api/milestones/{pid}`, `PUT /api/plans/{pid}/milestones/{m_pid}/tasks`. Across the plans the caller can read (others left out without a trace): **status** against the calendar (achieved / late / due soon / upcoming — due today is not late); **schedule state** from the critical-path forecast of the linked tasks still open (`forecast_late` when they finish after the due date, `at_risk` within `at_risk_days` business days before it — default 5 — else `on_track`); **`no_forecast` with a reason** (`no_linked_tasks`, `no_scheduled_tasks`, `cycle`) when there is nothing to forecast from — **never "on track" by default**; a forecast from only some of the open tasks is marked `partial`. One critical-path analysis per plan per request. Linking: the tasks must be live tasks of the milestone's plan (≤100, no repeats); a linked milestone must wait for at least one task; the set is replaced in one transaction and audited.
- [x] **MS-2 (S) — Completion rules.** **Landed 2026-10-05.** A `linked` milestone is **achieved when every task it waits for is done** (and it waits for at least one), derived on read with the tasks' latest completion date as `achieved_on`; a `manual` one when marked done. **Completing a linked milestone over open tasks needs a reason** (422 otherwise), stored as `done_reason` and audited with the number of open tasks; `POST …/milestones/{m_pid}/reopen` needs a reason, clears the manual completion and is audited; **a milestone achieved only by its tasks cannot be reopened (409): reopen a task.** `complete` still takes no body for a manual milestone, keeps the first completion stamp on a repeat, and the due-date automation sweep now skips linked milestones already achieved by their tasks.
- [x] **MS-3 (S) — Slip history.** **Landed 2026-10-05.** `PUT …/milestones/{m_pid}` still takes `{due}` and now an optional `reason` (≤500 characters); **every real move appends one `milestone_slips` row** (from, to, reason, who) — re-sending the same date records nothing. The view derives `original_due` (the first move's *from*), `reschedules`, `later_moves` and `net_days` (calendar days, negative if pulled in); `GET /api/milestones/{pid}` returns the full history.
- [x] **MS-4 (M) — Front-end.** **Landed 2026-10-05.** `/milestones` (summary by status, status filter, late first) with a card per milestone: status as a **word and a shape, never colour alone**, the forecast and slip as plain sentences, achieved-how, open tasks, and the forms to complete (reason required when it must be), reopen, reschedule with a reason, pick the tasks it waits for, and the slip history; `src/lib/milestone-view.ts` (7 unit tests); nav entry. **Deviations, stated plainly:** the reschedule reason is **optional** (a required one would break existing callers) and shown as "no reason given" when absent; the older `GET /api/plans/{pid}/milestones` list and the iCalendar and PDF outputs still show the stored `done` flag, not the derived achievement; a milestone is a record, not a zero-duration task (those, from PJ-1, stay in the schedule network); the forecast uses the plan's working calendar; milestones of one plan only link to that plan's tasks. **Not built:** a milestone on the Gantt, per-milestone notifications, a browser accessibility audit.

## Cost-risk and joint Monte Carlo (CR-n)

Requested 2026-10-05: "add Monte Carlo risk analysis". The schedule simulation already existed (SC-8). Asked for and built: a cost-risk simulation, a joint cost-and-schedule simulation, and a portfolio-level simulation.

- [x] **CR-1 (M) — Cost-risk Monte Carlo, per plan and across the portfolio.** **Landed 2026-10-05.** Migration `…_000039_cost_risk_inputs` (`budget_lines.low_minor`/`high_minor`, checked against the planned amount; `risks.cost_impact_minor`/`cost_currency`); `src/cost_risk.rs` (11 unit tests); `GET /api/plans/{pid}/cost-risk`, `GET /api/cost-risk`, `PUT …/budget-lines/{l_pid}/estimate`, `PUT …/risks/{r_pid}/cost-impact` + a DB test. Each iteration draws a final cost for every budget line from its low / likely (**the planned amount**) / high (triangular, or PERT), **never below the recorded actual — what is spent is spent** — then lets each open or mitigating register risk with a cost impact, a currency and a probability strike. **Per currency, never added across.** Reports the budget (sum of planned amounts), mean, cost at P10/P50/P80/P90, **the share at or under the budget**, a 20-bin histogram, a sensitivity correlation per line, how often each event struck against its stated probability, and each plan's mean beside its budget. **Seeded** (same seed ⇒ identical result; iterations capped at 50 000 and iterations × (lines + events) ≤ 5 000 000, refused not truncated); **with no uncertainty every percentile is the budget exactly**. Assumptions echoed, including `correlation: none` and the independence statement. **A line with no estimate uses the stated fallback band (±20%) and is marked `source=fallback`**; an estimate not around the planned amount is widened and flagged; a risk with no probability, closed, or in a currency with no budget line is **skipped and named**. **Portfolio scope** = every plan the caller can read (others left out without a trace).
- [x] **CR-2 (M) — Joint cost and schedule.** **Landed 2026-10-05.** `src/joint_risk.rs` (7 unit tests) + `GET /api/plans/{pid}/joint-risk[?currency&delay_cost_per_day_minor&…]`; `schedule_risk::simulate` is now `simulate_with` plus a per-iteration hook (same results, covered by its unchanged tests). One simulation drawn from **one random stream**: the schedule (as SC-8) and the cost of one currency together. **A risk with both a schedule impact and a cost impact is one event with one probability: it strikes once and drives both** (tested: every late run is also over budget). **A late finish adds a delay cost per business day (default 0; finishing early earns nothing back).** Reports the cost percentiles including delay cost, the share within budget, the share on time, **the joint share (both) — never above the smaller of the two** — the four quadrant counts, and a thinned sample (≤300) of (business days late, cost) points. Needs `currency=` when the plan budgets in several (422 otherwise); other currencies are listed as not simulated.
- [x] **CR-3 (M) — Front-end.** **Landed 2026-10-05.** `/plans/{pid}/cost-risk`, `/cost-risk` (portfolio, with each plan's mean beside its budget) and `/plans/{pid}/joint-risk`: histograms with **P50 / P80 / P90 and the budget marked by labelled, differently dashed lines** and a table twin; a quadrant table and a scatter whose numbers are in the table above it; the fallback lines flagged in words and with a symbol; risks used and skipped with reasons; every assumption stated. Plan page links and a nav entry; helpers in `src/lib/scheduling-view.ts` (+9 unit tests); the e2e spec runs axe over all three and checks the labels, table twins and the shared-event rule.

**Deviations, stated plainly:** **one probability per risk** — the cost fields reuse `risks.schedule_probability_percent` (named for the schedule simulation) as *the event's probability*, and each `PUT` of the schedule or cost fields replaces its own fields only (the cost endpoint changes the probability only when it is given); the ordinal 1–5 register probability is **still not converted** to a percent; **no per-line correlation or inflation** (inputs are independent, said in every response, and the spread is therefore narrower than a correlated one); a line's likely cost is its **planned amount**, not the phased baseline; the budget is the sum of lines, not the T-28b baseline; the joint run is **one plan and one currency**; the portfolio run does not model schedule (business days do not add across plans); delay cost is a rate you supply, not derived from resources; **the estimate inputs (line low/high, risk cost impact and probability, task optimistic/pessimistic) are set through the API — there is no form for them yet**; the front-end is English-first.

- [x] **CR-4 (M) — Forms for the simulation inputs.** **Landed 2026-10-05.** The estimates the simulations read can now be set in the app. On a plan's governance page: each **risk** has a "Simulation inputs" section (one probability; the task it delays and by how many business days; the cost it adds and its currency) and each **budget line** a range (low and high; the planned amount is the likely cost); on the critical-path page each task has a "Range for schedule risk" (optimistic and pessimistic; the duration is the likely value). Each says plainly when there is no range ("the simulation uses its fallback band"). The API lists now return the stored values (`GET …/risks`, `…/tasks`, `…/cpm` nodes; budget lines already did). What a risk form sends is decided by a pure, unit-tested planner (`src/lib/simulation-inputs.ts`, 13 tests): a cost-only risk never calls the schedule endpoint (so it is not named as a half-set schedule risk); a risk with both effects makes both calls in the order that keeps the **one shared probability** consistent; clearing the probability goes through the schedule endpoint, the only one that can; a task without its business days (or business days without a task), a cost without a currency, an effect without a probability, a high below the planned amount, and an optimistic above the likely duration are all refused in words **before any call**. **Two real bugs the browser tests found, fixed:** a number input binds a number, not a string (the helpers threw on `.trim()`), and a section's `toggle` event fired after the user had started typing and reset the form (the form now follows what is stored and is never reset by opening). The browser audit also caught the **Escalate button's 2.7:1 contrast** (the theme's error colour is a light tint), fixed in the shared danger colour, and the mobile audit's route list now includes every new page. **Not covered by a browser test:** the critical-path task-range form (its helper is unit-tested).

## Amendments (scheduling and cost canon)

- **T-28b / T-23:** once SC-4 lands, the "a baseline exists, but nothing scores
  it" message is replaced by real SPI/CPI; the `no_baseline` path stays
  byte-for-byte for plans without one.
- **TBA-11:** keep the throughput forecast as the *flow* answer. SC-8 is the
  *schedule* answer; the two are shown side by side, never blended.
- **Gantt (`/gantt`):** the "critical path" highlight is duration-only until
  SC-1 (finding F-6).

## Not taken (scheduling and cost canon)

A claim of ANSI/EIA-748 or DCMA compliance; an automatic resource-constrained
optimizer; silent auto-rescheduling; crashing advice (critical-chain buffers,
once listed here, are now taken as CC-n);
correlated risk models; binary `.mpp` / `.xer` round trips; free-text percent
complete. Reasons in [plan.md §21](plan.md).

## ProjectLibre ideas (PJ-n)

Source: research of ProjectLibre's code tree, 2026-10-03; evidence base, licence
constraint (**CPAL 1.0 — concepts only, copy nothing**) and findings in
[plan.md §25–§30](plan.md). These items support the SC items above: **PJ-1 must
be decided before SC-1**, because the schedule network is plan-level today
(finding F-9). Money stays integer minor units and per currency; absent evidence
is `null` with a reason, never zero.

- [x] **PJ-1 (M) — Task-level schedule fields and the schedule's unit** **Landed 2026-10-03.** Migration `…_000025_schedule_fields`: `tasks.start_date` + `tasks.duration_days` (0–3650 business days, 0 = milestone), and `plan_dependencies.kind` (`fs` default). `POST /api/plans/{pid}/tasks` takes both on create; **`PUT …/tasks/{pid}/schedule`** changes them (absent or null clears) and is audited with old and new. A dependency may now join **two tasks, two plans, or one of each**; kind `fs|ss|ff|sf`; lag `-3650..3650` (**negative = a lead**); cycle checks include task `precedes` relations. Every existing row and route is unchanged (the full suite passes untouched). **Decision taken (owner, 2026-10-03): one network of both.** **Deviations:** the plan-level and task-level networks are analysed separately (`/schedule` over a plan's children, `/cpm` over a plan's tasks) — a plan's own tasks do **not** roll up into the plan's dates, and an edge between a task and a plan is accepted but not yet analysed (PJ-13 / a unified run); only `precedes` relations (not `blocks`, which is about status) are schedule edges. **Not built:** a front-end. —
  **decided 2026-10-03: one network of both plans and tasks.** Add to a task an optional `start_date`, a working
  `duration_days` (or an end date), and a `constraint` (SC-2's set), so a task
  can be a node in the network. A **schedule node** is either a plan with a
  timeframe or a task with a start and duration; a task's parent plan is its
  container; LT-10 `precedes` / `blocks` relations become edges (type FS unless
  set). Recommendation: one network of both kinds. Undated tasks stay valid and
  are simply `unscheduled`.
  **Acceptance:** every existing task, plan and relation behaves exactly as
  today (existing tests pass unchanged); a task with no start is `unscheduled`
  in the engine, never float 0; an edge between two nodes needs read access to
  both; a task's dates are zone-free calendar dates; the OpenAPI shows the new
  fields; changing a task's schedule fields is audited.

- [x] **PJ-2 (M) — Cost rates (effective-dated), fixed-cost expenses, accrual.** **Landed 2026-10-05.** Migration `…_000041_cost_rates` (`cost_rate_tables`, append-only `cost_rates`, `task_costs`, `time_entry_costs`); `src/cost_rates.rs` (8 unit tests) + `src/controllers/cost_rates.rs` + a DB test. Rate for a day = latest effective on or before it; **no rate is `null`/`no_rate`, never zero**; daily labour costs sum exactly to the rounded total; accrual `start`/`prorated`/`end` sums exactly to the amount; **actuals snapshot the rate used and are never rewritten**; one block per currency, never summed. **Front-end:** `/cost-rates`, `/plans/{pid}/costs`. **Deviations:** **follow-up 2026-10-05:** finance access is now decided by the **policy** — the service puts its own `resource.area=finance` / `comms` rules in front of any deployment policy (`auth::service_rules`, 3 unit tests, and an end-to-end test with real tokens and a deliberately permissive policy), so a policy file cannot open rates; and `GET …/earned-value?ac=labour|both` takes AC from recorded labour cost (null, not zero, when none is recorded; `both` is only right if budget lines exclude labour; 403 without finance access). **Still budget-line actuals:** the forecast, S-curve, snapshots and portfolio figures. A finance holder also needs write access for the write routes (the default policy's coarse guard).
  `cost_rate_tables` per person or role (several named tables, as ProjectLibre
  has — standard, overtime, and so on), each rate **effective from a date**, in
  one currency; an expense type (labour, material, **fixed cost**) and an accrual
  rule (at start, prorated, at end). Feeds SC-4's AC and any cost report.
  **Rates are sensitive and governed by ABAC (decided 2026-10-03):** a
  `cost_rate` resource with `read` / `write` actions in the policy, deny by
  default; masked elsewhere; never in a guest view.
  **Acceptance:** a rate lookup by date returns the rate in force that day; a
  rate change **never rewrites an existing actual** (the rate used is recorded
  on the timesheet-derived cost); no rate ⇒ cost `null` + `no_rate`, **never
  zero**; two currencies are never summed; a caller without the finance
  attribute gets 404 on rate routes and no rate field anywhere; every rate
  change is audited with its effective date; accrual rules are pinned by tests
  (prorated across a 3-period task sums exactly to the cost).

- [x] **PJ-3 (M) — Resource availability over time.** **Landed 2026-10-03.** Migration `…_000028_person_availabilities`; `POST/GET /api/availability`, `DELETE` + `POST …/{pid}/delete`; `src/availability.rs` (8 unit tests) + a DB test + `tests/availability_acl.rs` with real tokens. A range is `from`..`to` inclusive and `max_percent` 0–100 (60 part-time, 0 leave, 50 ramp-up); **overlapping ranges for one person are refused 409**. **Capacity is read per day, never averaged** (tested: 80% demand over a fortnight whose second week is 40% is over for exactly that week, a window before the range is untouched), and `GET /api/capacity` gains `capacity {source, min_percent}`, `peak_percent` and `over_capacity` (runs of calendar days with demand and capacity). **With no range in the window the answer is exactly what it was** (`over_allocated` = summed demand > 100; `allocated_percent` is unchanged either way). **Access:** a person reads and sets their **own**; a resource manager (`role=resource_manager`) or an admin any; another person's is **404, not 403**. **Deviations:** overlap is checked in the service, not by a database exclusion constraint (two simultaneous requests could both pass; both then cover the same calendar days, and capacity reads the first range found); `/capacity/utilization` (hours-based, `working_time_configs`) is **not** yet folded in; availability is not combined with the person's weekly calendar (SC-2) — they are separate ideas (a calendar says which days work, availability how much of a business day); SC-6/SC-7 do not exist yet to consume it. **Not built:** a front-end editor. `availability` ranges per
  person (`from`, `to`, `max_percent`), so part-time, leave and ramp-up change
  capacity by date; folded into `/capacity`, `/capacity/utilization`, SC-6 and
  SC-7 as the denominator. Existing `non_working_periods` stay.
  **Acceptance:** with no ranges, capacity is exactly what it is today; ranges
  may not overlap for one person (409); utilization uses the availability in
  force each day, not an average; a person on 0% shows as unavailable, not as
  an error; a worker may see their own ranges only; changes audited.
  **Front-end landed 2026-10-03:** `/capacity` draws firm load vs the capacity in force with the proposed series hatched and apart and a strip of over-capacity days (`CapacityMeter`, pure `capacity-view.ts`); `/me/calendar` edits availability ranges; the plan's allocations list shows proposals with Commit and, on a 409, Commit anyway. The service now defaults `person_ref` to the caller on `POST /api/availability`.

- [x] **PJ-4 (M) — Proposed vs committed bookings.** **Landed 2026-10-03.** Migration `…_000029_allocation_booking` (`allocations.booking` default `committed`, `requested` default false). `POST /api/plans/{pid}/allocations` takes `booking` (`proposed`|`committed`) and `requested`; **`POST …/allocations/{a}/commit[?confirm=true]`**. **Existing allocations are `committed` and every figure is unchanged** (the full suite passes untouched). **A proposal never counts** toward utilization, over-allocation or at-a-glance (`all_allocations` is now committed-only; tested: 70% firm + 50% proposed reads 70, not over). `GET /api/capacity` reports it as its **own series** — `proposed {percent, allocations, would_exceed_capacity}` — never merged, and a person with only proposals still appears (allocated 0). **Committing** is allowed to the named person, a resource manager or an admin (others 404, tested with real tokens); if with it firm the person would be over their **per-day** capacity (PJ-3 availability) the answer is **409 over_capacity with the numbers** and only `?confirm=true` commits it (audited as an over-commitment); an already-committed booking is 409. **Decision applied (owner, 2026-10-03):** a proposal is visible to the person it names. **Deviations:** approval through PL-1 when a plan is governed is **not** wired (the commit endpoint is the gate for now); plan allocation lists are as visible as before, so a proposal is visible to anyone who could already list the plan's allocations — the decision added a right (the named person sees it), it did not hide it from others; `requested` is a flag with no inbox yet; the commit window for an open-ended booking is capped at one year. **Not built:** a front-end, a requests inbox for resource managers. An allocation gains
  `booking: proposed | committed` (default `committed`, so nothing changes) and
  an optional `request` flag for "asked of the resource manager". Capacity and
  utilization report the two **as separate series**; committing a proposal goes
  through approval (PL-1) when the plan is governed.
  **Acceptance:** existing allocations are `committed` and every figure is
  unchanged; **proposed never counts toward committed utilization**; both are
  shown, never merged; committing an allocation that would push a person over
  100% warns with the number and requires an explicit confirm; a person sees
  proposals naming them only per the owner's answer (plan.md Q); changes audited.
  **Front-end landed 2026-10-03:** `/capacity` draws firm load vs the capacity in force with the proposed series hatched and apart and a strip of over-capacity days (`CapacityMeter`, pure `capacity-view.ts`); `/me/calendar` edits availability ranges; the plan's allocations list shows proposals with Commit and, on a 409, Commit anyway. The service now defaults `person_ref` to the caller on `POST /api/availability`.

- [x] **PJ-5 (M) — Time-phased assignments and work contours.** **Landed 2026-10-05.** `src/work_profile.rs` (9 unit tests), `PUT …/tasks/{pid}/contour`, `GET …/work-profile`, `GET /api/work-profile`; largest-remainder spread so buckets sum to the work exactly, never on non-business days; task→plan→portfolio totals reconcile; unplaceable work is listed and counted. **Deviations:** contour shape definitions (`front_loaded`, `back_loaded`, `bell`, `turtle`) are this project's own, names unconfirmed against the reference tool. **Front-end:** `/plans/{pid}/work` (assignment form, contour, work table). Spread an
  assignment's work across periods by a **contour** — `flat` (default),
  `front_loaded`, `back_loaded`, `bell`, `turtle`, or `custom` (named per
  assignment) — into stored or derived **buckets**, with a consolidator that
  rolls buckets up plan → portfolio. The data behind resource histograms,
  S-curve PV and leveling. (Shape names are **inferred from class names** and
  are the usual scheduling-tool set; confirm before pinning.)
  **Acceptance:** buckets **sum to the assignment's work exactly** (largest
  remainder, no rounding drift); calendar-aware (no work on non-business days);
  `custom` must sum to 100%; the roll-up equals the sum of its children at every
  level; no assignment ⇒ `null` + `no_assignments`; a contour change keeps total
  work unchanged and is audited.

- [x] **PJ-6 (M) — Scheduling rules: fixed units / duration / work** — **Landed 2026-10-05** (all three rules). `src/scheduling_rules.rs` (9 unit tests), `PUT …/tasks/{pid}/assignment`, and the schedule editor recomputes on a duration change; over-capacity is flagged, never clamped; a task carrying work cannot become a milestone. Units are for the single assignee. **Front-end:** the assignment form on `/plans/{pid}/work`.
  **decided 2026-10-03: build all three (fixed work too).** A task with an assignment recomputes the third of
  (duration, units, work) when one of the other two changes, per its rule;
  integer minutes; deterministic. Whether ProjectLibre also implements an
  "effort-driven" flag is **not verified** (its constants file does not mention
  it).
  **Acceptance:** a worked example per rule pinned in a test; changing work under
  `fixed_duration` changes units, never duration; a result that implies units
  over the person's availability (PJ-3) is flagged, not clamped silently;
  default behaviour of every existing task is unchanged.

- [x] **PJ-7 (S) — Hammock activities.** **Landed 2026-10-06.** Migration `…_000044_hammocks` (`tasks.is_hammock`, `hammock_links`; a DB check that a hammock has no duration, work or constraint), `src/hammock.rs` (7 unit tests), `POST …/tasks/{pid}/hammock`, `POST/DELETE …/hammocks/{pid}/links`, `GET …/hammocks`, and `hammocks` on the CPM response. **Acceptance, as tested:** a hammock with no links is `unscheduled` (`no_links`; all-unscheduled or deleted links is `members_unscheduled`); it is **not in the dependency network** (excluded from `load_network`), so it never changes any task's float or the finish (asserted against the plan's CPM before and after), and it takes no dependency, schedule, assignment or sandbox edit (422); removing a link or deleting a member recomputes; a hammock can't link to itself, duplicate a link, link a task of another plan, or close a cycle of hammocks; hammocks nest. A task that carries schedule or dependencies can't become one (422 names what to clear). **Deviations:** links are to tasks **of the same plan** (a plan-level hammock and cross-plan spans wait for PJ-13); the MSPDI hammock id is still not imported; no front-end. A task or plan of type `hammock` whose
  start is the earliest and finish the latest of the items it links to, so a
  summary bar follows its links rather than its children (ProjectLibre's id
  100). Distinct from RM-5's roll-up (which follows children).
  **Acceptance:** a hammock with no links is `unscheduled`; it never drives a
  link target's float (it is a derived span, not work); removing a link
  recomputes; a hammock can't link to itself or create a cycle.

- [x] **PJ-8 (S) — Recalculation change set.** **Landed 2026-10-03.** Returned by `PUT …/tasks/{pid}/schedule` as `moved`: per task `start`, `finish` or `both`, with the business days each moved, across the **whole network** (not only the edited task); empty when nothing moved (tested). `cpm::diff` is pure. **Not built:** feeding watchers (RM-3) and the *My work* line; a cycle yields `moved: null` with a note. Every CPM recalculation (SC-1)
  returns **what moved**: per item `start_moved`, `finish_moved` or `both`,
  by how many business days, and what triggered it — ProjectLibre's
  `DateChangeStatus` idea. Feeds watchers (RM-3), the audit log and a "why did my
  date move?" line in *My work* (LT-3).
  **Acceptance:** a recalculation that changes nothing returns an empty set (not
  an error); the set names the edit that caused it; notifications honour the
  user's preferences (PL-5); dates that move *because of a hard constraint* say
  so.

- [x] **PJ-9 (M) — What-if sandbox for schedule edits.** **Landed 2026-10-05.** `src/sandbox.rs` (7 unit tests), `POST /api/plans/{pid}/sandbox/preview` and `…/sandbox/commit`, plus a DB test and a real-token test. Edits: move, duration, constraint, add link, remove link (≤100). **Acceptance, as tested:** a preview writes no row and no audit entry (even when it refuses) and the same edits give the same preview; commit applies exactly the previewed result (the test compares every changed task's dates and float, and the finish, with the CPM afterwards) with **one audit entry per changed item** (edits to one task coalesce; a no-op edit is no change); a commit needs `If-Match: <plan_etag>` (428 without) and a plan changed since the preview is **409 `stale_preview`, never merged**; masked fields stay masked (assignee only for who may manage people; cost impact only for the finance area, else `no_finance_access`); a hard constraint needs the same permission and a reason in a preview as in a commit. **The SC-6 leveling apply now goes through the sandbox commit** (still audited as `task_leveled`). Discard = do nothing (the preview is stateless). **Cost impact** is planned labour cost per currency (PJ-2 rates, PJ-5 spread), including the units/work a duration change recomputes (PJ-6); expenses are unchanged by edits and not shown. **Deviations, stated plainly:** commits to one plan are serialized with a Postgres advisory lock, but the freshness re-check inside it covers tasks and dependencies, **not calendars** (a calendar change in the few milliseconds between the check and the write is not caught), and contention is **untested** because the test pool has one connection; RM-9 (bulk reschedule) does not exist yet, so only leveling uses the sandbox. **Front-end:** `/plans/{pid}/what-if` (stage edits, preview, commit with the preview's etag, discard; a staged-list change drops the preview; a 409 says the plan changed and keeps the edits). Apply a list of edits
  (move, change duration, add or remove a link, change a constraint) to an
  in-memory copy, return the resulting dates, float, critical path and cost
  impact, then **commit or discard**. The apply step of SC-6 and of any bulk
  reschedule (RM-9) goes through it.
  **Acceptance:** a discarded preview writes nothing (asserted: no rows, no
  audit entry); committing applies exactly the previewed result and records one
  audit entry per changed item; the same input gives the same preview; a
  commit made against a plan that changed since the preview is refused 409
  (ETag), not merged; masked fields stay masked in the preview.

- [x] **PJ-10 (M) — MS Project interchange (MSPDI XML) — the T-28n codec.** **Landed 2026-10-03.** Migration `…_000027_task_external_ref` (`tasks.external_ref`, unique per plan). `src/mspdi.rs` (6 unit tests; parsed with `roxmltree`: **DTDs refused**, node cap, 5 MiB / 5000-task caps) and `POST /api/plans/{pid}/import/mspdi?dry_run=` (default **true**). **Id tables pinned (F-11):** dependency FF 0, FS 1, SF 2, SS 3; constraint ASAP 0 … FNLT 7 (an end-to-end test fails if FS and FF are swapped). Imports tasks, outline parents (subtasks), start, business-day duration (part business days **round up and say so**), milestones, constraints, deadlines, % complete → workflow state, dependencies with lag **or lead** (LinkLag is tenths of a minute), and the base calendar (only if the plan has none). **Idempotent:** UID → `external_ref=mspdi:<UID>`; a re-import updates in place, creates nothing, adds no duplicate dependencies, never changes status. **Nothing silently dropped:** the `unmapped` list names every inactive task, cross-project link, baseline, `alap`, resource, cost rate and unusable date with its reason. **A hard constraint (mso, mfo) obeys the same permission as setting one by hand** — without it the task imports *without* the constraint and the report says so (tested with real tokens). Summary tasks import without schedule fields so they cannot double-count in the critical path. **XML only; `.mpp` is refused with a message to export XML (owner, 2026-10-03).** **Deviations:** imports into an **existing plan** rather than creating one; **synchronous**, not a T-8 bulk job (no matching/review queue is needed for a new set of tasks — that stays a follow-up if matching to existing tasks is wanted); **resources are not matched to people** (names kept in the description, listed); baselines and cost rates not imported until SC-3 / PJ-2 exist; **no export** (the task text said import and export). **Not built:** MSPDI export, hammock tasks.
  Import (and export) tasks, WBS, outline levels, dependencies, constraints,
  calendars, resources, assignments, baselines and percent complete through the
  existing bulk machinery (T-8; review queue for ambiguous matches). **Id tables
  are pinned in tests (F-11):** dependency FF 0, FS 1, SF 2, SS 3; constraint
  ASAP 0, ALAP 1, MSO 2, MFO 3, SNET 4, SNLT 5, FNET 6, FNLT 7, hammock 100.
  Binary `.mpp` is **not** read: the UI says "export as XML".
  **Acceptance:** a fixture project round-trips with stable pids through the
  deterministic external ids; **every field not mapped is listed in the job's
  report**, never silently dropped; a dependency type is never swapped (a test
  that fails if FS and FF are exchanged); a row failing validation is reported
  with its line and the rest still import; a re-import is idempotent; no code or
  text from ProjectLibre is used (CPAL).

- [x] **PJ-11 (S) — Cancellable long jobs with progress.** **Landed 2026-10-06.** Migration `…_000045_job_progress_cancel` (`bulk_jobs.progress`, `cancel_requested_at`, `cancelled_by`, a `cancelled` status, and a **trigger that clamps any update that would lower progress**), `POST /api/plans/bulk-jobs/{id}/cancel`, progress/`cancelled` on every job view (`bulk::progress_percent`, 1 unit test + a DB test). **Acceptance, as tested:** progress never goes backwards (a direct `UPDATE` to a lower value leaves it unchanged; out of range is refused; 100 only when finished); cancelling a finished job is a no-op, not an error, and writes no audit; a queued job is cancelled before it runs and the worker does not resurrect it; a running job stops between rows, ends `cancelled`, **keeps and counts what it processed and counts what it did not reach** (`rows_omitted`), leaves no half-written row; only the requester or an admin may cancel (anyone else gets the unknown-id 404; real-token test); the cancel is audited once. **Deviations, stated plainly:** **SC-8 simulations are not jobs** — they run inside the request and are bounded (≤20 000 iterations), so there is nothing to cancel or report progress on; making them jobs is a separate change; a cancelled **export produces no file** (a partial one would pass for a whole one); the stop is noticed **every 10 rows**, so a running import may process up to 9 more rows after the request; a mid-run stop is tested by raising the flag before the run (the loop is deterministic that way) — a race between a live cancel and a live run is not tested; no front-end. `bulk_jobs` (and
  SC-8 simulations) gain `progress` (0–100, monotone) and a cooperative `POST
  /api/…/jobs/{pid}/cancel`; a cancelled job ends `cancelled`, keeps what it
  finished, and says so. Source: ProjectLibre's `JobQueue`, `ExtendedProgress
  Monitor`, `JobCanceledException`.
  **Acceptance:** progress never goes backwards; cancelling a finished job is a
  no-op, not an error; a cancelled import leaves no half-written row (each row is
  atomic) and reports counts; only the requester or an admin may cancel; the
  cancel is audited.

- [x] **PJ-12 (M) — Network diagram, WBS tree and RBS views.** **Landed 2026-10-06** (network diagram earlier; WBS tree and RBS now, front-end only). `/plans/{pid}/wbs`: tasks as a tree with generated codes (`1`, `1.2`, `1.2.3`; `src/lib/wbs.ts`) — codes are display only, recomputed from the tree each time, every link and key uses the task pid, and nothing sends a code to the API; a task under a parent the reader cannot see sits under a nameless "Restricted item"; cycles in data never loop or lose a task; the same rows are a table twin, and past 500 rows only the table shows. `/resources/rbs`: roles with the people in them (headcount only when masked) and skills as demand only, with the service's own caveat that skills overlap and supply is unknown; tables for both. Critical-path network: past 150 tasks it degrades to the table with a note, and dependencies to unreadable plans are counted, never named. Critical status was already never colour alone. Tests: 11 unit, 2 e2e with axe. **Not done:** no WBS reordering (siblings order by age, since the API has no sibling position), no RBS-to-task drill-down.
  (original spec follows) Front-end: a
  precedence **network diagram** (nodes = schedule nodes, edges = links,
  critical and near-critical emphasized), a **WBS tree** with **generated WBS
  codes** (`1`, `1.2`, `1.2.3`) derived from the plan / task hierarchy, and an
  **RBS** grouping of resources by role and skill. WBS codes are **display only —
  never an identifier**, because they change when siblings are reordered.
  **Acceptance:** every diagram has a table twin (the Lily tables, SC-11);
  critical status is never conveyed by colour alone; WBS codes recompute on
  reorder and no API accepts one as a key; a node the caller may not read shows
  as "restricted", not by name; a very large network degrades to a list rather
  than freezing.
  **Partly landed 2026-10-03 (front-end):** the **precedence network diagram** (`NetworkDiagram`: layered by longest predecessor chain, critical chains heavier with a ● marker, lag/kind labels, a list twin, keyboard selection) on `/plans/{pid}/critical-path`; layout is the pure, tested `layoutNetwork`. **Still open:** the WBS tree with generated codes and the RBS.

- [x] **PJ-13 (S) — Cross-plan task links.** **Landed 2026-10-06.** No migration: `plan_dependencies` already joined any two items; what was missing was the network and the privacy. `GET …/cpm` now includes the tasks of other plans a plan's tasks are linked to (`load_network_cross`: up to 8 plans / 3000 tasks, `external_truncated` says when more), as `external` nodes with `plan_name`, `cross_plan` edges, `network_finish` beside the plan's own `project_finish`, and float computed across the link; `GET /api/dependencies`, `POST` and `DELETE` and `DELETE /api/plans/{pid}` now respect readability. **Acceptance, as tested:** a link a caller cannot read **both** ends of is left out of the calculation and shown only as `restricted_edges` / an opaque `{restricted: true}` dependency — **no name, pid, owner or date of the hidden side, asserted by searching the response text** — in the critical-path view, the dependency list and a refused delete; the hidden side does not move the reader's float; **a private plan (RM-8) and a guest each have a regression test** (real tokens, `tests/cross_plan.rs`; a guest cannot reach the CPM view or the list at all, and their plan record names nothing); a link cannot be created or removed by someone who cannot read both ends (404, so ids cannot be probed); deleting a plan with inbound or outbound task links is **409**, naming the dependants the caller can read and counting the rest; **cycle checks span plans**; float across plans uses the SC-2 rule — each plan's tasks run on its own calendar intersected with the analysing plan's (tested with a Monday-to-Friday plan linked to a seven-day plan). **Deviations, stated plainly:** only the **critical-path view** is cross-plan — schedule health, the risk simulations, leveling and the what-if sandbox still use a plan's own tasks, so they ignore cross-plan links; because a hidden link is excluded, **float for a reader who cannot see a link is not the float a member sees** (privacy over agreement; the response says so); an external task's **assignee calendar is not applied** (only its plan's); task `precedes` relations stay within a plan; no front-end. A task may depend on a task in
  **another plan** (ProjectLibre's sub-project and external-task idea), included
  in the network and the float calculation. Visibility: an edge is shown only if
  the caller can read **both** ends; otherwise it appears as "external
  dependency (restricted)" with **no name, owner or date** of the hidden side.
  **Acceptance:** a private plan (RM-8) or guest view never reveals the other
  plan through an edge (a regression test per case); deleting a plan with
  inbound external edges is refused 409 or shows the dependants; cycle checks
  span plans; float across plans uses the same calendar rule (SC-2).

- [x] **RM-13 (M) — RACI matrix.** **Landed 2026-10-03** (requested by the owner). Migration `…_000032_raci_assignments` (soft-deleted history; a **partial unique index allows one Accountable per task at the database**, so two simultaneous requests cannot both win); `GET /api/plans/{pid}/raci[?format=csv]`, `PUT /api/plans/{pid}/tasks/{pid}/raci`; `src/raci.rs` (7 unit tests) + a DB test + the front-end `/plans/[pid]/raci`. R Responsible, A Accountable, C Consulted, I Informed; a person may hold several (the usual A/R). **A row is replaced whole and atomically.** **Refused 422:** more than one Accountable, a letter that is not R/A/C/I, a person twice, a person with no letters, a non-URN person, more than 50 people. **Allowed and reported, never hidden:** no Accountable, no Responsible (how a plan looks while it is set up) — each row says what is missing, the summary counts reconcile with the rows, and each person's load (R/A/C/I counts) is given. A task with no assignments is still a row. CSV export has people as columns and a gaps column in words. People who are R, A or I are **auto-subscribed** to the task (RM-3, honouring `auto_watch`); removing them from the row does not unsubscribe them. Front-end: tasks down, people across, four toggle buttons per cell (a visible letter plus a sentence for assistive technology, `aria-pressed` for state), giving someone A **moves it and says so**, a ⚠ gap per row in words, load table, export link. **Deviations:** columns are **people only** (no role columns), rows are **tasks only** (no deliverable level); there is no copy-from-another-task shortcut; the matrix is not yet in the PDF board pack.

## Amendments (ProjectLibre)

- **Hardening pass 2026-10-06 (after PJ-7…PL-6).** An adversarial read of the new backend code found, and this pass fixed (each with a regression test): the private-plan guard parsed the **raw** path, so `/api/plans/%7B<id>%7D/…` skipped it while the handler served the plan (now percent-decoded first; unit + real-token test); onboarding **case** routes had no private-plan check (now 404 like the plan); what-if **commit** was a permanent 409 in any plan containing a hammock (digest rows now match); approval **decide/withdraw** raced (the write now carries `WHERE status='pending'`, so one of two racing calls wins and the other is 409, which also stops a document version stuck `approved` under a withdrawn request); a document owner could **approve their own** document (refused); document submit re-reads under the per-document lock; `task?include=relations` and `GET /api/dependencies?item=` no longer reveal ids across a privacy boundary; process/document reads need sign-in; a process stopped between creation and its first step is recovered by `resume`, and a late `complete_step` no longer double-logs after a cancel; schedule edits, hammock changes, document edits and onboarding case creation are one transaction with their audit row (audit failure now fails the request); DB guards added (migration 48): one open onboarding case per plan/kind/subject, and no dependency on a hammock. **Since fixed:** process and execution lists page in SQL; the document list builds views only for the requested page; the network loaders query only the dependencies touching the plan(s) asked about; hammock link creation and plain schedule edits take a per-plan advisory lock (the latter the same one a what-if commit holds). **Known and not changed:** a named approver can read the whole process execution including its comments even without access to the subject (by design, stated). A plain edit that reads a task before a commit and writes after it still overwrites with the values it was sent (an explicit PUT is last-writer-wins). UI: 529 vitest, 260 Playwright; API: lib 795, `--test mod` 152, every token binary green, clippy clean.
- [x] **T-28n remainder — Jira and Asana import.** **Landed 2026-10-06** (deferred to the end by owner priority, then started on request). Reader `src/tracker_import.rs` (11 unit tests: RFC 4180 CSV with quotes, embedded newlines, BOM and CRLF; size/row/column/field caps; Jira `15/Oct/26 12:00 AM` and ISO dates, day-first numeric dates refused as ambiguous), controller `src/controllers/tracker_import.rs`: `POST /api/plans/{pid}/import/jira` and `…/import/asana`, body = the CSV, `dry_run` default true. **Idempotent** by `external_ref` (`jira:<key>`, `asana:<task id>`, unique per plan): a re-import updates titles, deadlines, start dates and parents in place and never changes status or description. **Nothing silently dropped:** every unread column, skipped row, link, date and parent is in `unmapped` with a reason. Jira: Issue key, Summary, Status (bucketed by words), Description, Assignee, Due date, Parent, Labels, Blocks links (other link types reported). Asana: Task ID, Name, Notes, Assignee, Due/Start Date, Completed At, Parent Task (by name — used only when exactly one task has it), Tags, Blocked By / Blocking. Parent loops and chains deeper than 10 are cut and said; dependencies that would close a cycle are skipped and said. One transaction, audited (`jira_import`/`asana_import`). UI `/plans/{pid}/import` now also takes **Microsoft Project XML** (the PJ-10 endpoint, which had no screen; a `.mpp` is refused with how to export XML; its report is shown in the same shape, the task UID as the key) (preview first, the same single primary button then imports, unmapped grouped by kind) with `rawBody` on the API client. Tests: DB test for dry run, apply, idempotent re-import (status kept), refusals and source separation; 9 unit + 2 e2e with axe. **Deviations, stated plainly:** CSV only (no JSON export, no live API pull); due dates become **deadlines** and no durations are guessed, so imported tasks have no schedule until given one; assignees are names kept in the description, never matched to people; comments, attachments, sprints, custom fields and worklogs are not imported (reported as columns). **Also fixed on the way:** a plan id that is not a UUID was a 500 everywhere (`find_by_pid`), now 404; the UI client showed the error *code* (`unprocessable_entity`) instead of the reason on every refusal, now the reason.
- **Owner priority 2026-10-03:** **Jira and Asana import (the remainder of T-28n) was deferred to the end of the backlog and has now landed (2026-10-06).** Focus now: **calendars and visualizations** (the front-end for float, critical chains, calendars and the network diagram — PJ-12, SC-11).

- **Decisions 2026-10-03 ([plan.md §31](plan.md)):** one network of plans and
  tasks; calendars **per person and per plan**; cost rates under ABAC; MSPDI XML
  only; all three scheduling rules; proposed bookings visible to the person they
  name.

- **SC-1 / SC-3 / SC-4:** their "per task" wording assumed task-level dates that
  do not exist (finding F-9); each depends on **PJ-1**. Until then they operate
  on plans, as `critical_path()` does today.
- **SC-2:** the constraint set is the same eight ProjectLibre and MS Project
  use; the hammock is PJ-7. **Per-resource calendars** are in question (plan.md
  §30) — start with one per plan, with PJ-3 availability as the per-person
  mechanism.
- **SC-4:** keep ProjectLibre's split of *percent complete* and *physical percent
  complete* visible: they answer different questions (time elapsed vs work
  done) and EV must say which it used. AC's source needs **PJ-2** (finding F-10).
- **SC-6 / SC-7:** consume PJ-3 (availability), PJ-4 (committed vs proposed) and
  PJ-5 (time-phased buckets); SC-6's apply goes through **PJ-9**.
- **SC-8:** PERT-style weighting `(O + 4M + P) / 6` is the standard three-point
  mean; ProjectLibre's `PertWeights` appears to store such weights (**inferred
  from the name, unverified**). Run long simulations as jobs with **PJ-11**.
- **T-28n:** its scope is **PJ-10**; the spec should cite MSPDI as the target.
- **LT-2 (plan templates):** ProjectLibre's project types (Professional
  Services, Product Development, Event Planning, Marketing Campaign, Sales
  Campaign, Technical Support, IT) suggest a `category` on templates for
  discovery. Optional; no behavioural change.
- **Statuses:** ProjectLibre's PENDING_APPROVAL and ON_HOLD map to our approvals
  (PL-1) and lifecycle; **verify an `on_hold` state exists** before assuming.

## Not taken (ProjectLibre)

Any code or text from it (CPAL 1.0, logo and network-use conditions); an embedded
scripting engine; the single-user local-file desktop model; a global undo stack;
the MS Project numeric ids as our own enums. Reasons in
[plan.md §27](plan.md).

## Monthly director pack (DP-n)

Requested 2026-10-09: "What directors should see each month" — one pack, the
same for the organization's executive, its board, an escalation board, and the
chief executives of partner organizations. Assessment, gaps and owner questions in [plan.md §32–§38](plan.md).
Doctrine for every item: the pack is **frozen and versioned**; trends compare
with **the previous issued pack**; absent data is **"not assessed" / null with
a reason, never green or zero**; money is per currency, never summed across;
**no person-level data** in the pack; colour is never alone.

- [x] **DP-1 (L) — Issued pack: frozen, versioned, per audience.**
  Draft a pack for a month (`POST /api/director-packs` with `period`, cut-off
  time); every view below is computed once into the pack's stored JSON; a
  person **issues** it (ABAC `director_pack:issue`), after which it is
  immutable; a correction is a **reissue (v2) with a change note** and the old
  version stays readable. Readers are granted by **audience** (`executive`,
  `board`, `escalation_board`, `partner`) via ABAC `director_pack:read`;
  a reader of an issued pack needs no access to the live estate. Cover page:
  pack id, version, period, cut-off, what changed since the last pack, and a
  data-quality panel (EX-1). **Acceptance:** an issued pack's body is
  byte-identical on every read and after any later data change; issuing twice
  is 409; a reissue keeps v1 readable and links v1↔v2 with the note; a
  `partner` reader gets 404 on every live aggregate and 200 on the issued
  pack; the pack body contains no person ref and no cost rate (a test scans
  it); every draft, issue and reissue is audited.

  **Built 2026-10-10, with a body that holds only the sections that exist.** `POST/GET /api/director-packs`, `GET /{pid}`, `POST /{pid}/issue`, `POST /{pid}/reissue`; migration 55. The body is computed once at drafting and stored as exact text; a database trigger refuses any change to, or deletion of, an issued pack (the single exception is a data-subject pseudonymisation blanking `drafted_by`/`issued_by`, with the body proven unchanged). One pack per period and audience (409 otherwise); issuing twice is 409; a reissue needs a note of at least 10 characters, keeps v1 readable and links v1 to v2, and only the latest version can be reissued. Drafting needs `access=admin` or `directorpack=editor|issuer`, issuing needs `access=admin` or `directorpack=issuer`; readers hold `pack_audience=<audience>`; a `role=pack_reader` caller (the partner case) is confined by the guest guard to `GET /api/director-packs…`, every live route answering 404 (or 403 where the policy refuses first). The pack counts only plans that are neither private nor carrying a handling mark and says how many were left out. Cover: pack id, version, period, cut-off, what changed against the previous issued pack for the audience (counts, in words), and a data-quality panel that lists the sections not yet built. Tests: rules, and a real-token end-to-end (who drafts and issues, audience reading, confinement, private plan's issue not counted, no person ref or cost rate, byte-identical after data changes and across versions, audit trail, eight database refusals). **Not done, stated:** the body has only issues and change requests (DP-2 to DP-10 add their sections); the cut-off limits records by date but statuses are those at drafting (there is no status history); the data-quality panel is the small one above, not EX-1; no notification on issue (DP-12); no screen; no discard of a draft; audience subscribers and a per-audience content difference do not exist (every audience gets the same body).
- [x] **DP-2 (M) — Delivery confidence: stored, dated, beside the derived RAG.**
  `PUT /api/plans/{pid}/delivery-confidence` with a value on a five-point scale
  (`green`, `amber_green`, `amber`, `amber_red`, `red`; scale configurable,
  §37), `kind` (`self` or `independent`), rationale (required), assessor and
  date; append-only history. The portfolio map shows the latest independent
  value (else self, marked so), the derived RAG beside it (never blended), and
  a **trend arrow against the previous issued pack** (`up`, `down`, `same`,
  `new`). **Acceptance:** no assessment ⇒ `not_assessed`, never green; an
  assessment older than the pack period is marked `stale`; a two-step
  disagreement between independent and self (or derived) is flagged; history is
  never rewritten; who may set `independent` is an ABAC action.

  **Built 2026-10-10.** `PUT /api/plans/{pid}/delivery-confidence` appends a stated assessment (five-point scale, `self` or `independent`, rationale of at least 10 characters, date not in the future); `GET` returns the whole history and what the map shows. Append-only: the database refuses edit and delete (the one exception is a data-subject pseudonymisation blanking `assessor`). Recording `independent` needs `access=admin` or `confidence=independent` (a token attribute, not the policy file). The map shows the latest independent value, else the latest self one marked as self, beside the derived RAG and never blended; no assessment is `not_assessed`; older than the pack's period is `stale`; a two-step gap between independent and self, or between the stated value and the derived RAG, is flagged; the trend (`up`, `down`, `same`, `new`, `not_assessed`, or `no_earlier_pack`) compares with the previous issued pack. Migration 56. **Not done, stated:** the five-point scale is fixed, not configurable (§37); the live map (`GET /api/portfolio-map`) shows no trend, only issued packs do; no screen.
- [x] **DP-3 (M) — Exceptions and the 48-hour red rule.**
  Setting confidence to `amber_red` or `red` (or worsening within them) opens an
  **exception** for the programme and starts a clock; the exception is
  *complete* when it holds a cause, the decision needed, a named owner and a
  decide-by date, and *closed* when the decision is recorded. Not complete
  within the configured window (default 48 elapsed hours, §37) is a **breach**,
  notified through the existing notification sinks and shown in the pack.
  `GET /api/exceptions[?status&breached]`. **Acceptance:** the clock is tested
  with a fixed clock at 47h59 / 48h00 / 48h01; a second worsening while open
  does not restart the clock but is recorded; improving to amber does not close
  the exception (a decision does); the pack lists open, breached and closed-in-
  period exceptions with owner and decide-by; every change audited.

  **Built 2026-10-10.** Recording `amber_red` or `red` opens an exception and starts the clock; a worsening while open is recorded (`worst_value`, `worsened_count`, audited) without restarting it; improving to amber does not close it; only `POST /api/exceptions/{pid}/decision` (at least 10 characters) closes it. `PUT /api/exceptions/{pid}` fills in the cause, the decision needed, a named owner (with the owner's role) and the decide-by date; complete when all four are there. Breach = not complete within the window (default 48 elapsed hours, `PROJECT_PORTFOLIO_MANAGEMENT_EXCEPTION_WINDOW_HOURS`, 1 to 720) from when it opened; the clock is pinned by unit tests at 47h59, 48h00 (still in time) and 48h01, and a late completion stays a breach. `GET /api/exceptions[?status&breached]` across the plans you can read; exceptions on a plan you cannot read are 404 to edit or decide. The task `cargo loco task exception_breaches` notifies each breach once through the notification inbox, to the named owner or else whoever opened it. The pack lists open, breached and closed-in-period exceptions, naming the owner by **role**, never by person. **Not done, stated:** the task must be scheduled by the operator (nothing runs it for you); the notice goes only to one person; the decision is not tied to a gate review; no screen.
- [x] **DP-4 (S) — Strategic aims and the portfolio map.**
  Mark top-tier objectives as **strategic aims**; each programme has one
  **primary aim** (others are cross-references). The map groups programmes by
  primary aim, then stage (gate) and phase, with delivery confidence and trend.
  **Acceptance:** a programme linked to two aims appears once, under its
  primary, with the other named; a programme with no aim is listed under
  "no strategic aim", not dropped; totals never double-count.

  **Built 2026-10-10.** `PUT /api/objectives/{pid}/strategic-aim` marks an objective as a strategic aim (unmarking clears programmes' primary flags); `PUT /api/plans/{pid}/primary-aim` sets a programme's one primary aim (the database allows only one; any earlier primary becomes an ordinary link). `GET /api/portfolio-map` and the pack's `portfolio_map` group programmes by primary aim with stage, phase, confidence, derived RAG and trend; a programme appears once with its other aims named; one with no aim, or with aims but none primary, is under "no strategic aim" with the reason; totals do not double-count. **Not done, stated:** "programme" means a plan whose kind is `Program` (there is no other definition in the data); the objectives registry has no hierarchy, so "top-tier" is whatever you mark; the map's rows are not grouped by gate beyond showing stage and phase.
- [ ] **DP-5 (M) — Gate decisions in the period; paused and stopped.**
  Add a **planned gate date** per programme and gate; a gate whose planned date
  passed without a review, or reviewed as `hold`, is **deferred**. Map
  `approved` / `approved_with_conditions` → passed, `rejected` → failed. Add a
  programme **status** `active | paused | stopped` set only through a recorded
  decision (reason, decider, date, link to gate review or exception);
  `stopped` is terminal and keeps the record (finding F-12). **Acceptance:**
  the pack's gate view lists passed, failed, deferred, paused and stopped in
  the period; pausing and resuming are audited with reasons; a stopped
  programme's money and benefits stay reportable; a paused programme's
  allocations are flagged, not deleted.

- [ ] **DP-6 (M) — Funding sources and financial-year money.**
  A `funding_sources` reference list (owner-supplied, §37) and on each budget
  line a funding source and `capital | revenue`; periods by a configurable **financial
  year** (e.g. April–March). Pack view per programme × source: budget, spend to date,
  full-year forecast (existing forecast), variance. **Acceptance:** a line with
  no funding source is counted under "no funding source" and listed in the
  data-quality panel; capital and revenue are never summed unless asked; a
  financial-year boundary test (31 March / 1 April); currencies never summed.

- [ ] **DP-7 (M) — Buffer fever chart.** Reverses the §21 "not taken" narrowly
  (plan.md §34). A **project buffer** in business days per programme, declared
  at baseline or by a stated rule; **chain completed %** from the baselined
  critical chain's done work; **buffer consumed %** from the CPM forecast finish
  against the baselined chain finish; zones from two configurable sloped lines,
  printed in the pack; the trail is last pack → this pack. **Acceptance:** no
  buffer, no baseline or no forecast ⇒ "no fever-chart point" with the reason;
  recovery below 0% and overrun above 100% are plotted at the edge with the
  true number; zone boundaries are pinned by unit tests at the line; a table
  twin carries every point. Feeding and resource buffers and multi-project
  pipelining are CC-3 and CC-5; once CC-4 lands it supplies this chart's
  buffer.

- [ ] **DP-8 (S) — Portfolio earned value over the threshold.**
  A BAC **inclusion threshold** (configuration, printed in the pack); for each
  programme above it: CPI, SPI, EAC (the variant used is named) and the AC
  source (`budget_lines` or `labour`). **Acceptance:** a programme below the
  threshold shows "below EVM threshold"; one with no baseline shows
  `no_baseline`; an EX-14 *product* programme shows its product measures, not
  an EVM blank; figures equal `/earned-value/rollup` for the same as-of.

- [ ] **DP-9 (M) — Validated benefits: per year and non-financial.**
  On a benefit: `recurring | one_off`, annualized amount, classification
  (`cash_releasing | non_cash_releasing | societal`, §37), and **validations**
  (amount, validator, evidence attachment, date) — the validator may not be a
  member of the programme (ABAC). Non-financial benefits link to a KPI (KO-1)
  and report its status. Pack: planned against validated per programme and in
  total, per currency. **Acceptance:** realized-but-unvalidated is shown as
  *claimed*, never as validated; a validator on the programme is refused 403;
  one-off and recurring are never summed into "per year"; a non-financial
  benefit without a KPI is "not measured".

- [ ] **DP-10 (L) — Capacity heatmap by skill pool and month, with partner
  commitments.** Needs EX-3. A `skill_pools` taxonomy mapping roles and skills
  to pools; SC-7's resource plan at **month** grain grouped by pool;
  **commitments** — an organization commits N FTE of a pool for a month to a
  programme (supply) or owes work (demand), with committed and delivered.
  **Acceptance:** demand, supply and the gap per pool and month, each cell with
  a word and a shape as well as colour and a table twin; partner supply
  shown separately from the organization's own supply and then combined; no person ref in the
  pack; a pool with no supply is "no supply recorded", not zero capacity;
  whether partners see each other's commitments follows the §37 answer.

- [ ] **DP-11 (M) — Rendering: HTML, PDF and every locale.**
  An accessible HTML page and a PDF, both from the **same issued JSON** (the
  PL-10 rule). All fixed text through the locale catalogue, so every
  supported locale renders it; free text shown as entered. **Acceptance:** the PDF and HTML show
  the same figures as the JSON (a test compares them); axe passes on the HTML
  in every locale; every chart has a table twin; no string in the pack view is
  missing from any locale's catalogue (catalogue test).

- [ ] **DP-12 (S) — Monthly rhythm: draft on schedule, review, issue by a person.**
  A monthly schedule (existing `report_schedules` cadence) drafts the pack; the
  portfolio office reviews; a person issues. **Acceptance:** the schedule never
  issues; a draft older than its period's cut-off is marked stale; issuing
  notifies each audience's subscribers with a link, never the body.

## Escalation-driven capabilities (EX-n)

Requested 2026-10-09: research what an organization under escalation (external
oversight by its sponsor) needs from a portfolio tool, generalized from the
owner's private notes. Assessment, gaps, findings F-10 to F-13 and owner
questions in [plan.md §39–§45](plan.md).

- [ ] **EX-1 (M) — Register completeness; optional "no register, no money".**
  A completeness score per programme over SRO, partners, stage, baseline and
  forecast dates, budget, funding source, confidence, top risks and
  dependencies, each missing field named. A deployment switch (off by default)
  refuses a budget-line release or a firm allocation on an incomplete
  programme, naming what is missing. **Acceptance:** the data-quality panel
  (DP-1) lists every incomplete programme; with the switch on, the refusal is
  422 listing fields; with it off, the action succeeds and the warning is
  audited.

- [ ] **EX-2 (S) — Delivery-model typology.** `central_product |
  central_platform_local_deployment | local_programme_central_standards` on a
  programme, with the **delivery owner** organization. **Acceptance:** the pack
  shows the type and owner; a local programme is shown as not delivered by the
  organization;
  EX-5's date-change rule reads the type.

- [ ] **EX-3 (L) — Partner organizations.** An `organizations` entity (the
  delivery organization, partner body, government sponsor, supplier) and optional organization on
  people, allocations, milestone sign-offs, approvals and SRO roles (a joint
  SRO is two). Finding F-10. **Acceptance:** existing records with no
  organization behave as today; an organization can be an audience (DP-1); a
  user's organization is an ABAC attribute, so "a partner sees its own
  commitments" is a policy, not code.

- [ ] **EX-4 (M) — Milestones met only with partner sign-off.** A milestone may
  require sign-off from named organizations; *delivered* (the organization) and *met* (all
  sign-offs) are separate dates. **Acceptance:** a milestone with an
  outstanding sign-off is delivered-not-met; withdrawing a sign-off is audited
  with a reason; the pack reports the met share and the delivered-not-met gap.

- [ ] **EX-5 (M) — Deployment-date change control.** For a
  `central_platform_local_deployment` programme, a partner's move of its
  go-live is a change request: the schedule network computes the knock-on to
  other deployments, the request names an approver (configurable, e.g. an
  escalation board) and the knock-on cost is recorded against the requester.
  **Acceptance:** an unapproved move does not change the schedule; the knock-on
  list equals a CPM run with the moved date; approval and refusal are audited.

- [ ] **EX-6 (M) — Go / no-go for safety-critical cutovers.** A cutover decision with
  **stop criteria published before the decision**, each marked met or not, and
  **co-signatures** (partner's accountable professional lead and the delivery
  organization, roles configurable).
  **Acceptance:** criteria cannot be edited after the decision opens (a new
  version is); "go" needs every co-signature; "no-go" or "delay for safety" is
  recorded as a decision and shown as such in the pack, not as slippage.

- [ ] **EX-7 (M) — Triage and pause / reset / stop thresholds.** A dated
  **triage** (Finish, Fix, Pause, Stop per programme, with reasons); per
  programme **thresholds** (confidence red for N packs; forecast past a date;
  cost-risk P80 past a cap). Crossing one opens an exception (DP-3) that needs
  a pause, reset, stop or recorded override. **Never auto-pauses.**
  **Acceptance:** each threshold kind tested at the line; the pack counts
  pause, reset and stop decisions in the period; an override names its
  decider and expiry.

- [ ] **EX-8 (M) — Start control: capacity check and WIP limit.** Needs DP-10.
  A configurable limit on programmes in active delivery; promoting a proposal
  or moving a programme into delivery shows its demand per skill pool against
  free supply and is refused (or needs an override with reason) at the limit or
  when a constraint pool is over. Waiting starts queue in order. **Acceptance:**
  the limit counts only `active` (not paused) programmes; an override is
  audited; the queue order is stable.

- [ ] **EX-9 (S) — Watermelon measures.** Per pack: self-vs-independent
  confidence gap per programme, **late discovery** (first report at amber-red or
  red with no amber in the previous pack), milestones met with sign-off as a
  share (EX-4), red without an approved recovery or stop decision. **Acceptance:**
  computed only from issued packs (never re-derived); each measure null with a
  reason when its inputs are missing.

- [ ] **EX-10 (M) — Outcome scorecard and de-escalation tracker.** A named set
  of KPIs (KO-1) with targets, read by quarter; a tracker of the escalation
  level (recorded as set by the sponsor, never computed), the criteria, evidence links per
  criterion, and **consecutive quarters met**; the rule (default two quarters)
  is configuration. **Acceptance:** a quarter with a missing reading breaks the
  run and says why; a starter set of measures ships as an optional
  template, not as defaults (§45).

- [ ] **EX-11 (S) — Key-person register.** Single points of failure (person or
  role), successor and readiness; contingent share per skill pool. Resource
  managers only; the pack shows counts. **Acceptance:** a non-manager gets 404
  on the register; the pack contains no names (scan test).

- [ ] **EX-12 (M) — Portfolio ledger: "what did this allocation buy?"** A
  funding allocation (source, amount, date) traced through budget lines, gates,
  milestones met and benefits validated in one query; finance cost codes map to
  a register id for reconciling ledger exports. **Acceptance:** untraced money
  is reported as untraced with its amount; the trace for a seeded allocation is
  complete and per currency; a CSV ledger import reconciles or lists unmatched
  codes.

- [ ] **EX-13 (S) — New asks need a named trade-off.** A proposal tagged as an
  external ask from the sponsor must name what it displaces before approval.
  **Acceptance:** approval without a trade-off is 422; the pack lists asks and
  their trade-offs in the period.

- [ ] **EX-14 (S) — Method per programme: EVM or product.** A programme is
  `evm` or `product`; product programmes report burn-up and flow (TBA) in the
  pack's EVM view. **Acceptance:** no EVM blank for a product programme; the
  method change is audited.

## Production readiness (HD-n)

Requested 2026-10-09 (two lists; HD-8 and HD-9 come from the second). Gaps,
findings F-14 to F-23 and owner questions in [plan.md §46–§51 and §52a](plan.md).

- [x] **HD-1 (S) — Licence files.** **Landed 2026-10-09.** `LICENSE/` holds `MIT.txt`, `Apache-2.0.txt`, `BSD-3-Clause.txt`, `GPL-2.0-only.txt` and `GPL-3.0-only.txt` (the SPDX licence-list texts; MIT and BSD carry `Copyright (c) 2026 Joel Parker Henderson`); the expression `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` is now in all three manifests (the UI said `MIT OR Apache-2.0`, F-14); the root and subproject READMEs state it, link each file, and offer custom licences on request. **Acceptance, as tested** (`tests/licence_files.rs` in the API crate, DB-free): every licence in the expression has its file in `LICENSE/` and nothing else is there; MIT and BSD have no template placeholder left; every manifest declares the expression; no `LICENSE*` / `LICENCE*` / `COPYING*` file exists outside `LICENSE/` except the root `LICENSE.md` (a stray `LICENSE-MIT` in the UI was caught); and `LICENSE.md` states the expression and links every text (added 2026-10-09 at the owner's request: it explains the choice and the custom-terms contact; spec NFR-15). **Deviations, stated plainly:** a crate published on its own cannot `include` files outside its directory, so a published crate would need copies (not done; nothing is published); forges that detect a licence from a root `LICENSE` file will not detect this directory, so the README carries the statement. Owner's choice (§51):
  one SPDX expression for the whole repository. **All licence files live in a `LICENSE/`
  directory at the root** (owner decision, 2026-10-09), one file per licence
  in the expression (e.g. `LICENSE/MIT.txt`, `LICENSE/Apache-2.0.txt`); no
  licence file anywhere else, including the crate and UI sub-directories.
  Every `Cargo.toml` and `package.json` `license` field is set to the
  expression; the README states it and links the directory. **Acceptance:** a
  test (or CI step) reads every manifest and fails if any `license` differs
  from the root expression, if a licence named in it has no file in
  `LICENSE/`, or if a licence file exists outside `LICENSE/`; `cargo deny
  check licenses` still passes; a crate published on its own includes the
  licence files (`include` in its manifest).

- [ ] **HD-2 (M) — Buildable from its own repository, with automated checks.**
  Root `.github/workflows/ci.yml` (the sub-directory workflows are inert,
  F-15): Rust jobs for both crates on the pinned toolchain (`fmt --check`,
  `clippy -D warnings`, unit tests, DB request suites against a Postgres
  service container, real-token binaries), `cargo deny`, and a UI job (`pnpm
  install --frozen-lockfile`, `check`, unit tests, production build, e2e with
  axe). Remove `actions-rs`; fix `repository` URLs and "Main X Index"
  mentions (F-16); untrack and ignore `matcher/target/` (F-17). Decide the
  canonical forge and give mirrors a matching pipeline or document them as
  mirrors. **Amended 2026-10-09 (second list):** add a root `dependabot.yml`
  (cargo, npm, github-actions); replace `actions-rs` with plain `cargo`
  steps; run the DB suites with `--include-ignored` against a Postgres
  service; remove the monorepo-path `Dockerfile` copies and the
  `../../ci/postgres-init` mount from `compose.test.yaml` (F-21), and
  settle how `integrity-mac` and `authentication-verifier` are obtained
  (crates.io, vendored, or git dependencies on their own repositories;
  plan.md §52a). **Acceptance:** `git clone && docker compose up` gives
  a working app on a clean machine and CI proves it; a job clones fresh
  and runs the documented build with only the documented prerequisites; a deliberately broken commit fails
  each job; `git status` is clean after running every test.
  **Status 2026-10-09 (built; stays open until the first green GitHub run):** root `.github/workflows/ci.yml` (jobs `api`, `api-db` with Postgres 18, `matcher`, `deny`, `ui` including Playwright and axe, `docs`, `container`, `zap`, `fresh-clone`, and the single aggregate check `ci-ok`) and `.github/dependabot.yml`; the three inert sub-directory workflows were removed; `matcher/target/` is untracked and ignored; every `Cargo.toml` `repository` names this repository. The `Dockerfile` now builds from this repository's layout, and `ci/postgres-init` plus a root `compose.yaml` exist. **Verified locally (podman):** the image builds from a clean context; `podman compose up --wait` boots the secure stack — `/_health` 200, anonymous `GET /api/plans` and `/metrics.prom` 401, security headers present; production with login off, and production with no key set, both refuse to start with a message naming the fix; the `dev-insecure` profile (config `container-dev`) serves anonymous calls with the loud warning. **Not verified:** any workflow on GitHub (no `actionlint` here; the YAML parses), the ZAP baseline, the `lychee` action, Codeberg/GitLab mirrors (no pipeline for them yet; owner's canonical-forge answer, plan.md §51), and the UI container (the UI builds with `adapter-auto`, which yields no runnable server, so compose runs the API and database only).

- [x] **HD-3 (M) — Login enforcement on by default.** **Landed 2026-10-09.** `auth::resolve_require_auth`: on unless `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` is `false`/`0`/`no`/`off` or the loco config's `settings.require_auth` is `false` (the variable wins; a typo stays ON). `auth::check_startup` (called from `App::after_routes`) logs a loud warning when it is off, **refuses to start** in the `production` environment with it off, and refuses to start with it on, no key set and no `PASETO_KEYS_URL` (with a URL the service boots and answers 401 until the keys arrive). `config/development.yaml` and `config/test.yaml` opt out explicitly via `settings:`; `production.yaml` never does; `config/container-dev.yaml` (a non-production environment name) serves the compose `dev-insecure` profile. `/metrics.prom` left the public path list. **Acceptance, as tested:** unit tests pin the resolver, the settings reader and the startup verdict; `tests/enforcement.rs` proves anonymous `GET /api/plans` and `POST` are 401, metrics are 401 without and 200 with a token, and health/ping stay open; the full suite (153 request tests, every real-token binary) passes with the defaults flipped. **Deviations, stated plainly:** the flag is still a process-wide `OnceLock` (first reader wins), so a test binary decides once; the production refusal keys on the loco environment name, so a deployment that runs the production shape under another name (as `container-dev` does on purpose) is not refused; the compose smoke boots the secure stack with a public-only test key set (`scripts/smoke-keys.sh`), which a real deployment replaces with its issuer's keys. Original brief: `require_auth()` is true
  unless `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` is explicitly false;
  turning it off logs a startup warning and is **refused in production**
  (§51); enforcement on with no token verifier configured **refuses to
  start**. Development and test configs opt out explicitly, and the test
  harnesses that rely on anonymous writes set the opt-out themselves (F-18).
  **Acceptance:** with no variable set, an anonymous `POST` is 401 and an
  anonymous `GET /api/plans` is 401; `/_health`, `/_ping` and the documented
  public paths still answer; production with the opt-out does not start; the
  full test suite passes with the defaults flipped.

- [x] **HD-4 (S) — Security headers.** **Landed 2026-10-09.** API: `src/http_security.rs` (`security_headers_mw`, `cors_mw`): `X-Content-Type-Options: nosniff`, `Content-Security-Policy: default-src 'none'; frame-ancestors 'none'` (a relaxed policy for `/swagger-ui` only), `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`, a `Permissions-Policy` denying the powerful features, `Cross-Origin-Opener-Policy` and `Cross-Origin-Resource-Policy: same-origin`, and `Strict-Transport-Security` only when a trusted proxy says the request was HTTPS (`…_HSTS=proxy|always|off`, `…_TRUSTED_PROXIES`); a header a handler already set (file downloads) is never overwritten. CORS is an explicit allow-list (`…_CORS_ORIGINS`, exact origins, no wildcard; unset = none) and loco's framework CORS is disabled in production. UI: `src/lib/security-headers.ts` applied in `hooks.server.ts`, and `kit.csp` in `vite.config.ts` (`script-src 'self'` with SvelteKit nonces/hashes; no `'unsafe-inline'` for scripts or style elements); SVAR's icon font is self-hosted (`static/svar/`) and its themes mounted with `fonts={false}`. **Acceptance, as tested:** unit tests per module; `tests/hardening.rs` drives the real router (headers on a 200 and on a 429, allowed and denied origin, HSTS absent over HTTP); `tests/e2e/security-headers.spec.ts` asserts the header set and **no CSP violation** on representative routes in `en-001`, `cy-001` and `ar-001` (30 tests); the whole UI suite (316 Playwright tests including axe) passes under the CSP. **Deviations, stated plainly:** `style-src-attr 'unsafe-inline'` is allowed because the SVAR grid, Gantt, calendar and Kanban set computed `style="…"` attributes that cannot be hashed (scripts and `<style>` elements stay strict); `/swagger-ui` loads its bundle from a CDN under its own relaxed CSP; `/metrics.prom` is behind login (HD-3) rather than on a separate port; the ZAP baseline job is written (`ci.yml`, `.zap/rules.tsv`) but has not run, because it needs GitHub. Original brief: One API middleware and one UI hook:
  HSTS (TLS only), `X-Content-Type-Options: nosniff`, `frame-ancestors 'none'`
  (and `X-Frame-Options: DENY`), `Referrer-Policy:
  strict-origin-when-cross-origin`, a minimal `Permissions-Policy`,
  `Cross-Origin-Opener-Policy: same-origin`; a strict CSP for the UI (no
  inline script; nonces where SvelteKit needs them). **CORS becomes an
  explicit allow-list** from configuration (F-19). **Acceptance:** a request
  test asserts every header on an API route, an error response and a file
  download; an e2e test asserts the UI headers and that the app still runs
  under the CSP (no console violations); a disallowed origin gets no CORS
  grant. **Amended 2026-10-09:** `/metrics.prom` leaves the public path
  list (behind login, or on a separate internal port, plan.md §52a, F-22);
  a CI job runs an OWASP ZAP baseline scan against the running service and
  UI with the default configuration and fails on new findings.

- [x] **HD-5 (M) — Rate limiting.** **Landed 2026-10-09.** `src/rate_limit.rs` (+ `src/trusted_proxy.rs`): in-process token buckets in three families — general (keyed by a hash of the bearer credential, else the client address), expensive (CPM, levelling, schedule/cost/joint risk, forecasts, imports, exports, bulk jobs, evidence pack, PDF, scenario compare/generate/evaluate) and authentication failures (per client address, charged on each `401` and checked before the guard runs). `/_health` and `/_ping` are exempt. `429` carries `Retry-After` and `RateLimit-Limit` / `-Remaining` / `-Reset`; allowed responses carry the headers of the tightest bucket charged. The client address is the socket peer, or the rightmost untrusted `X-Forwarded-For` hop only when the peer is in `…_TRUSTED_PROXIES`. Defaults: general 600/min with burst 200, expensive 30/min with burst 10, failures 6/min with burst 20 (`…_RATE_LIMIT_GENERAL_PER_MINUTE`, `…_BURST`, `…_EXPENSIVE_*`, `…_FAILURES_*`; `…_RATE_LIMIT=off` or `settings.rate_limit: false` disables, as the development and test configs do). Memory is bounded (100 000 keys per family, idle buckets shed, then one shared overflow bucket). **Acceptance, as tested:** unit tests with a hand-driven clock (burst, refill, no over-fill, independent keys, bounded memory); middleware tests (one subject limited while another is not, the expensive bucket trips first, a spoofed `X-Forwarded-For` from an untrusted peer is ignored while a trusted proxy tells clients apart, repeated failures cut a client off, health never limited, disabled passes everything); `tests/hardening.rs` proves it on the real router with the security headers on the `429`. **Deviations, stated plainly:** state is per process, so with several replicas each enforces its own share (a shared store is a later step); the credential key is a non-cryptographic hash of the bearer string, so a rotated token starts a fresh bucket; failures are counted per client address, so many users behind one untrusted NAT share a failure budget; the numbers are starting points awaiting the owner's answer (plan.md §51). Original brief: Token buckets per authenticated subject
  (else per client IP, honouring a configured trusted proxy only); a tighter
  bucket for expensive routes (simulations, CPM, levelling, imports, exports,
  PDF) and for failed authentication; health and metrics exempt; `429` with
  `Retry-After` and `RateLimit` headers; numbers from configuration (§51).
  **Acceptance:** a test drives one subject past its limit and gets 429 with
  the headers while another subject is unaffected; a spoofed
  `X-Forwarded-For` from an untrusted peer is ignored; the expensive bucket
  trips before the general one; limits reset after the window.

- [x] **HD-6 (M) — Backup plan.** *(Folded into HD-10, 2026-10-09; implement and tick there.)* `spec/` chapter: RPO and RTO, PostgreSQL
  continuous archiving (base backup plus WAL) or managed point-in-time
  recovery, the **artifact store backed up and restored with the database**,
  encryption, off-site copy, retention, and who restores. Scripts in the
  repository; a **scheduled restore test** restores into a scratch database,
  runs migrations, re-verifies a sample of artifact SHA-256 checksums and runs
  a smoke suite. Fix the stale production config comment (F-20).
  **Acceptance:** the restore test runs in CI (or on a schedule) and fails
  when a backup is missing, older than the RPO, or restores with mismatched
  artifacts; the runbook names every step.

- [x] **HD-7 (M) — Data protection impact assessment.** *(Folded into HD-11, 2026-10-09; implement and tick there.)* `spec/` chapter: the
  processing (who, what, why, where, how long), lawful basis per category,
  necessity and proportionality, data-subject rights and how each is met
  (export, masking, rectification, erasure limits given append-only audit),
  risks with likelihood, severity and the control that answers each,
  retention per table, processors and transfers, review date and sign-off
  (§51). **Acceptance:** a test lists every column that can hold personal
  data (people refs, free text, timesheets, cost rates, availability, tokens)
  and fails when a migration adds one the DPIA inventory does not name; every
  risk in the DPIA links to a control that exists in the code or the
  deployment guide.

- [ ] **HD-8 (M) — A release people can pin.** Version tags for this product
  (`v<major>.<minor>.<patch>`), a container image published to the chosen
  registry on each tag (plan.md §52a), a CycloneDX software bill of materials
  for the service (`cargo cyclonedx`) and the UI (npm), attached to the release
  with release notes drawn from `CHANGELOG.md`; optionally signed images and
  build provenance. **Acceptance:** a named tag deploys to the same image digest
  every time; the SBOM lists every dependency in `Cargo.lock` and
  `pnpm-lock.yaml` (a test compares counts); the release job fails when the
  tag, the crate version and the UI version disagree; `cargo deny` and the ZAP
  baseline (HD-4) run before publication.
  **Status 2026-10-09 (built; stays open until a tag has been released):** `.github/workflows/release.yml` on tags `v*.*.*` verifies the tag equals the API crate version, runs `cargo deny`, builds and pushes a multi-arch image to GHCR (`ghcr.io/<owner>/<repo>`; tags version, major.minor, sha) with provenance and an SBOM attestation, produces CycloneDX SBOMs for both crates and the UI, and creates the release with notes from `CHANGELOG.md` and the SBOMs attached. `scripts/release-notes.sh` is tested locally (prints the `[0.3.0]` section; exits non-zero for a missing version). **Not verified:** the workflow itself (it needs GitHub), the SBOM tools, and signing (provenance only). Registry choice and signing are owner questions (plan.md §52a).

- [x] **HD-9 (M) — Documents that match the code.** **Landed 2026-10-09.** `scripts/generate-status.sh` rewrites the `<!-- generated:start -->` block in `spec/14-implementation-status.md` (migrations, mounted and documented routes, test counts, versions, ticked and open tasks per prefix, locales) and `--check` fails on drift; `scripts/check-links.py` (stdlib) resolves every relative Markdown link and `#anchor`, with `lychee.toml` for the equivalent `lychee --offline` run. The roughly 108 links into the non-existent `agents/share` were re-pointed at files in this repository or replaced by plain-text citations marked "shared family guidance, not in this repository"; the roadmap and status files were reconciled against the code (Jira, Asana and MSPDI **import** exist, MSPDI **export** and the **issues register** do not, controlled documents and approval processes are built), each correction citing its file or route. **Acceptance, as tested:** the link checker reports 0 broken links; `generate-status.sh --check` passes; the `docs` CI job runs both. **Deviations, stated plainly:** the checker covers Markdown links only (not links in code comments or JSON); the `lychee` CI step has not run on GitHub (the stdlib checker is the one verified here); the route counts are derived by counting `.add(` and quoted path keys, not by booting the service, so a route registered another way would be missed; the test-function counts are line counts of `#[test]`-style attributes. Original brief: Generate
  `spec/14-implementation-status.md` from commands (route count from the
  OpenAPI document, migration count, test counts, ticked tasks) instead of
  writing it by hand; fix or remove the **108** links into the non-existent
  `agents/share` (F-23, plan.md §52a); add a link checker (for example
  `lychee`) to CI; reconcile `spec/15-roadmap.md` with the status file (Jira
  and Asana import, MSPDI import and export, controlled documents are built).
  **Acceptance:** the link checker passes in CI; a regeneration step produces
  no diff against the committed status file; a test fails when a documented
  route is missing from the OpenAPI document or the reverse.

- [x] **HD-10 (L) — Operations pack (folds in HD-6).** Requested 2026-10-09
  (second list, item 10). `operations/`: a deployment guide (compose today;
  Kubernetes/Helm is out of scope until a cluster target is named), backup and
  restore scripts for the database **and** the artifact store, **stated RPO and
  RTO per backup mode**, an upgrade and migration guide with rollback, and a
  runbook. **Acceptance:** `scripts/backup.sh` and `scripts/restore.sh` produce
  and restore a verified archive (database dump, artifact files, checksums);
  `scripts/restore-test.sh` restores into a fresh database, proves migrations are
  current, compares per-table row counts and a content digest with the source,
  re-verifies sampled artifact checksums, and then the CI job `restore` runs the
  test suite against the restored database; the targets are written in
  `operations/recovery-objectives.md` and spec NFR-16; a backup older than the
  stated RPO fails a check script.
  **Built 2026-10-10.** `operations/` (deployment, backup-and-restore, recovery-objectives, upgrade-and-migration, runbook), `scripts/{backup,restore,restore-test,check-backup-age}.sh`, CI job `restore`. **Not run here:** the GitHub Actions workflows (the restore job included); the scripts and the drill were run locally against throw-away Podman Postgres.

- [x] **HD-11 (L) — Information governance pack (folds in HD-7).** Requested
  2026-10-09 (item 11). `governance/`: a DPIA template pre-filled for this
  service, a data-flow diagram, a record of processing, a retention schedule per
  kind of record (what the code enforces today, stated separately from what a
  buyer must decide), an accessibility statement (WCAG 2.2 AA, grounded in the
  axe tests), a language statement, and a DTAC checklist answered section by
  section with the evidence for each answer. **Acceptance:**
  `tests/personal_data_inventory.rs` (DB-free) fails when an entity gains a
  column that can hold personal data which `governance/personal-data-inventory.csv`
  does not name, or when the inventory names a column that does not exist; every
  DPIA risk links to a control that exists; every DTAC answer is `Met`,
  `Partly` or `Buyer to complete` with a pointer, never blank.
  **Built 2026-10-10.** `governance/` (DPIA, data-flow, record of processing, retention schedule, accessibility statement, language statement, DTAC checklist, personal-data inventory) and `tests/personal_data_inventory.rs`. **Deviations:** the DPIA, record of processing and DTAC checklist leave buyer decisions marked `‹buyer›` or `Buyer to complete`; the audit trail is documented as digests plus an optional MAC, not a hash chain.

- [x] **HD-12 (L) — Sign-in with Microsoft Entra ID.** Requested 2026-10-09
  (item 12). The UI's server side runs OpenID Connect (authorization code with
  PKCE) against a configured provider, directly or via a Keycloak realm that
  brokers Entra; tokens stay server-side; the API verifies the provider's access
  token (the `keycloak` feature) and maps roles to ABAC through the explicit
  role map. Documented in `operations/sign-in-with-entra.md`. **Acceptance:** a
  Playwright test signs in through an Entra-compatible mock OIDC provider
  (discovery, JWKS, v2.0 issuer, `tid`/`oid`/`roles` claims, PKCE enforced) and
  lands signed in; a container test (`keycloak_entra_broker`, Podman) signs in
  through a real Keycloak that brokers that provider and the API guard accepts
  the resulting token with the mapped role; a tampered or wrong-audience token
  is refused. **Not taken:** the API verifying raw Entra tokens without Keycloak
  (the verifier is realm-shaped); the auth-service federation route is
  documented, not tested here.
  **Built 2026-10-10.** `src/lib/server/oidc*.ts`, `/signin/oidc`, `tests/e2e/oidc-signin.spec.ts` against `tests/e2e/support/entra-mock.mjs`, and the Podman test `keycloak_entra_broker`. **Not tested:** a real Microsoft Entra tenant.

- [x] **HD-13 (L) — Every page in every supported locale, Welsh first.**
  Requested 2026-10-09 (item 13). All visible text moves into the catalogue;
  a CI test fails on visible English outside it and on any `cy-001` key missing;
  Welsh is drafted for every key and **marked draft until a professional
  translator reviews it**, with the review recorded in
  `governance/welsh-translation-review.md` and an export for the translator.
  **Acceptance:** `tests/unit/catalogue-coverage.test.ts` scans every template
  (Svelte AST) and helper for literal visible text and fails on any not
  allow-listed with a reason; `cy-001` has every `en-001` key; the Welsh e2e
  tests render the pages in `cy-001` with no English left visible (apart from
  the allow-list and API-generated sentences); the review status is recorded.
  **Deviations to state:** other locales fall back to English for new keys;
  sentences generated by the API are English.
  **Built 2026-10-10; professional review still open.** 2,327 page and component texts moved into `i18n-pages.ts` with Welsh in `i18n-pages.cy.ts`; `tests/unit/catalogue-coverage.test.ts` and `tests/e2e/welsh.spec.ts` (every route) guard it; code values (statuses, kinds, bands) and common API error codes are shown through the catalogue; a localized `+error.svelte`. The Welsh is **a draft by the assistant, not reviewed by a professional translator**: the review is recorded as *not done* in `governance/welsh-translation-review.md`, with `scripts/export-welsh-review.py` producing the file for the translator. **Deviations:** other locales fall back to English for the new page text; API-generated sentences other than the common error codes are English; compound sentences may need rewording for Welsh mutation.

## Critical chain buffers (CC-n)

Requested 2026-10-09: "critical chain buffers". Takes the method §21 declined
and DP-7 narrowed; assessment and owner questions in
[plan.md §52–§56](plan.md). Doctrine: **report, a person decides** (no
automatic re-planning); buffer rules and zone boundaries are configuration
printed with every result; durations in **business days**; missing inputs give
"no critical chain" with a reason, never a guess.

- [ ] **CC-1 (L) — The critical chain.** From the levelled schedule (SC-6),
  add **resource links** (a task waiting for a person is linked to the task
  that person was on) and compute the longest chain through dependencies and
  resource links. `GET /api/plans/{pid}/critical-chain`, shown beside the CPM
  critical path, never blended. **Acceptance:** a two-path network whose paths
  share one person yields a chain that crosses paths (pinned by a unit test);
  with no resource contention the chain equals the CPM critical path; a plan
  with no assignments returns `no_critical_chain` with the reason.

- [ ] **CC-2 (S) — Aggressive and safe estimates.** Per task, an aggressive
  (about 50% confidence) and a safe estimate in business days, or a stated rule
  deriving one from the other (§55); reuse CR-4's optimistic / pessimistic
  where set, and say so. **Acceptance:** the rule in force is echoed; a task
  with neither estimate is listed as missing, not defaulted silently; changes
  are audited.

- [ ] **CC-3 (M) — Project, feeding and resource buffers.** Schedule tasks at
  their aggressive estimates; size a **project buffer** at the end of the
  chain and a **feeding buffer** where each non-critical chain joins it, by
  the configured rule (cut-and-paste 50% of removed safety, or root sum of
  squares); list **resource buffers** as readiness alerts on critical-chain
  tasks. **Acceptance:** both sizing rules pinned by unit tests on a known
  network; a feeding buffer that would push the chain is reported, not
  silently absorbed; buffers are business days on the plan's calendar.

- [ ] **CC-4 (M) — Buffer management.** For each project and feeding buffer:
  chain completed %, buffer consumed %, zone (configurable sloped
  boundaries), history per snapshot, and recorded **recovery actions** when a
  buffer enters red. Supersedes DP-7's buffer source; DP-7 keeps the fever
  chart. **Acceptance:** consumption equals the forecast slip of the chain end
  over the buffer; zone boundaries pinned at the line; a red feeding buffer
  raises an alert naming its chain; recovery actions are audited.

- [ ] **CC-5 (L) — Drum, pipelining and capacity buffers.** A portfolio
  **drum** (named, or the most overloaded skill pool from DP-10, §55); a
  proposed **staggered start** for each project against the drum's capacity,
  with a **capacity buffer** between drum tasks of successive projects; EX-8
  can use the drum schedule for start control. **Acceptance:** two projects
  needing the drum at once are staggered and the stagger is explained; the
  service proposes, never moves, start dates; a portfolio with no drum named
  and no skill pools says so.

- [ ] **CC-6 (M) — Front-end.** Chain and buffers on the Gantt (chain marked
  in words and shape, buffers as labelled bars), a fever chart per project and
  feeding buffer with a table twin, the drum pipeline view. **Acceptance:**
  axe passes; every chart has a table twin; colour is never alone.

## Issues register and change requests (IR-n)

Requested 2026-10-10: "an issues register (FR-14), change requests". Specified
in [spec §6 FR-14 and FR-14a](spec/06-functional-requirements.md); assessment in
[plan.md §57](plan.md). Doctrine: an issue **has happened** (a risk **might**);
a change request is **decided by a person other than the requester**; durations
in **calendar days** (ageing) and **business days** (schedule impact); impacts
are stated, never guessed.

- [x] **IR-1 (M) — Issues store and API.** Migration, entity, controller and
  OpenAPI for `issues` under a plan (`/api/plans/{pid}/issues`) with the FR-14
  fields; ABAC like tasks; audited. **Acceptance:** CRUD round trip; a `Closed`
  issue without a resolution is refused (422); an issue of a plan the caller
  cannot read is not listed; routes appear in `src/openapi.rs` and the generated
  status block.
  **Built 2026-10-10:** migration `m20261010_000050_issues`, `src/issues.rs` (unit-tested rules), `controllers/issues.rs`, OpenAPI paths, `tests/requests/issues.rs` (DB-backed), inventory rows. A plan's issues are in the `risks` module, so switching that module off hides them. **Deviation:** `priority` is free text, and the register list is per plan until IR-2.
- [x] **IR-2 (M) — The register.** `GET /api/issues` across plans (filter by
  plan, severity, status, owner, age in calendar days; paged) and a
  **Register** page in the UI (every text in the catalogue, `cy-001` included).
  **Acceptance:** an issue list sorted by severity then age; ageing equals
  calendar days since `raised_on` (pinned by a unit test); the page passes the
  axe audit and the Welsh render test.
  **Built 2026-10-10:** `GET /api/issues` (readable plans only, filters, summary, 500-row cap), `/issues` page with every text in the catalogue and Welsh, enum labels, `tests/e2e/issues.spec.ts` (order, Welsh, filter, axe), `tests/requests/issues.rs`. **Deviations:** the Welsh is the unreviewed draft; the page lists and filters but does not raise or edit issues (that needs a form, not yet built).
- [x] **IR-3 (S) — Raise an issue from a risk or a blocked task.** Escalation
  keeps `source_risk_pid` / `source_task_pid`; closing the issue leaves the
  source untouched. Daily-standup blockers (spec §1.4) can become issues.
  **Acceptance:** raising from a risk links both ways; the risk's own status is
  unchanged.
  **Built 2026-10-10:** `POST /api/plans/{pid}/issues/from-risk/{risk_pid}` and `…/from-task/{task_pid}`; `severity_from_exposure` (unit-tested at each boundary); a request test covering conflict, 404, 422 and raising again after closing. **Deviations:** API only, no button on the risk or board pages yet; the daily-standup blocker conversion in the spec is the blocked-task route, not a separate flow; the risk is deliberately not moved to `materialized` by raising an issue (that stays the risk's own *escalate* action).
- [x] **IR-4 (L) — Change requests.** Store, API and UI for FR-14a with the
  status machine Draft → Submitted → UnderReview → Approved|Rejected →
  Implemented (Withdrawn from any open state). **Acceptance:** the requester
  cannot decide their own request (403); a decision records who, when and why;
  linking issues and risks works both ways; an illegal transition is 409.
  **Built 2026-10-10.** `/api/plans/{pid}/change-requests` (draft, edit, submit, review, decision, implement, withdraw, links) and `GET /api/change-requests` (across the plans you can read); migration 54; the whole state machine pinned by a unit test; **one named approver who cannot be the requester, not even an administrator can decide for them**, a decision needs a reason of at least 10 characters, and the database refuses an approval without decider, date and reason, a cost without its currency, a submission without an approver. Tests: the full flow with login off, and the separation of duties with real tokens (who may edit, submit, withdraw, review and decide; every form of own-approver; audit actors; five database refusals). A page `/change-requests` lists them with a status summary (English and Welsh draft, axe). Under the `governance` module. **Not done, stated:** drafting, submitting and deciding have **no screens** (API only); one approver, no approval chain; no notification to the approver when a request is submitted; relinking is limited to open requests.
- [x] **IR-5 (M) — Impact and baseline.** For a request with a schedule or cost
  impact, show a **what-if preview** from the existing scheduler (labelled a
  forecast) and, on a person's choice after approval, take a new baseline
  (never replacing one) with the request as the reason. **Acceptance:** the
  preview matches the what-if endpoint for the same edit; the baseline carries
  the change request's id; nothing re-baselines automatically.
  **Built 2026-10-10.** A request may carry `edits` (the what-if sandbox's edit list); `GET …/preview` returns the sandbox's own answer for them, labelled a forecast, beside the impact the requester stated, and a test proves it is identical to `POST …/sandbox/preview` for the same edits. `POST …/baseline` (approved or implemented, once) takes a schedule baseline through the same function the baseline endpoint uses, whose reason names the request, and the request keeps `baseline_pid`; nothing is re-baselined automatically. **Not done, stated:** a cost forecast is not computed (only the stated cost is kept); the baseline row itself has no change-request column (the link lives on the request and in the reason text).
- [x] **IR-6 (S) — Reports.** Issues and open change requests in the monthly
  director pack (DP) as counts by severity and age, with the oldest named.
  **Acceptance:** the pack lists them, or says "none recorded"; never a zero for
  an unavailable figure.

## Public-repository files (PF-n)

Requested 2026-10-10: "read spec/public* then implement"
([`spec/special-files-for-public-repos`](spec/special-files-for-public-repos/index.md),
requirement NFR-20).

  **Built 2026-10-10 as an endpoint, ticked for the data; the pack page that shows it does not exist.** `GET /api/director-pack/issues-and-changes?window_days=` (1 to 366, default 30) gives open issues by severity, overdue, raised and resolved in the window, the oldest open issue, change requests by status, those awaiting a decision, approved but not implemented, and the oldest open one, over only the plans the caller may read (same filter as every other cross-plan summary). Nothing recorded is said in words (`No issues recorded.`); when the caller can read no plan, `available: false` with a reason rather than zeros. Unit tests of the rules (window edge, all-closed register, oldest named) and a request test. **Not done, stated:** the director pack itself (DP-1 to DP-12) and its screen, so nothing displays this yet; no cost or schedule impact totals across change requests.
- [x] **PF-1 (M) — The special files.** Root `README.md`, `LICENSE.md` (existing),
  and new `CITATION.cff`, `NEWS.md`, `COMPARISONS.md`, `BENCHMARKS.md`,
  `INSTALL.md`, `CONTRIBUTING.md`, `RFC.md`, `CODEOWNERS`, `MAINTAINERS.md`,
  `CHANGELOG.md`, `AI_STATEMENT.md`, `GOVERNANCE.md`, `SECURITY.md`.
  **Acceptance:** `tests/public_files.rs` (database-free) fails when a file is
  missing or empty, when the maintainer address is absent from `MAINTAINERS.md`,
  `CODEOWNERS` and `SECURITY.md`, or when `CITATION.cff` lacks its required
  fields; the link checker reports 0 broken links. **Built 2026-10-10 with these
  deviations, stated plainly:** `BENCHMARKS.md` holds only the matcher's
  Criterion run of 2026-10-10 (the service benchmark was not run); `COMPARISONS.md`
  rests on the plan's research, not on running the other products;
  `AI_STATEMENT.md` follows the template's structure but is rewritten to this
  project's facts (the template's claim that tool attribution stays out of
  commit trailers is false here, so it says the opposite; its openEHR and
  patient-data text is dropped; the contradictory "AI is enabled to merge" line
  becomes "AI does not merge on its own initiative"); `SECURITY.md` response
  times are targets; no donation channel exists, and `CONTRIBUTING.md` says so;
  the ORCID is the one in the maintainer's other `CITATION.cff`. The
  maintainer should read `AI_STATEMENT.md`, `SECURITY.md` and `GOVERNANCE.md`
  before relying on them: they make commitments in the maintainer's name.

## Readiness for major medical and governmental organizations (RG-n)

Requested 2026-10-10. Verdict and evidence in [plan.md §58–§63](plan.md): **not
ready for unsupervised production use; ready for a controlled pilot on
non-clinical data**. These items close the technical findings. Doctrine: state
what was **not** tested; claim no certification.

- [x] **RG-1 (S) — Strict mode for operator-configured outbound targets.**
  User-supplied destinations are already guarded (`sinks::destination_url_ok`).
  Add `PROJECT_PORTFOLIO_MANAGEMENT_OUTBOUND_STRICT=on`, which applies the same
  public-address rule to operator-configured webhook and worker targets (default
  off, because internal targets are a legitimate operator choice; stated in the
  deployment guide). **Acceptance:** unit tests with strict on and off for a
  public host, a private literal, the metadata address and a `.internal` name;
  the existing sink tests still pass.
  **Built 2026-10-10.** `PROJECT_PORTFOLIO_MANAGEMENT_OUTBOUND_STRICT`, `webhooks::target_allowed`, used for webhook and worker targets; unit-tested. **Correction to the first assessment:** user-supplied destinations were already fully guarded; only operator targets lacked a strict mode. **Not tested:** a live request to a metadata address.
- [x] **RG-2 (S) — Idle session timeout.** OIDC sessions end after a period of
  inactivity (default 30 minutes, configurable) as well as at the absolute limit.
  **Acceptance:** unit test with an injected clock; the sign-in page says the
  session ended because of inactivity; operations guide updated.
  **Built 2026-10-10.** `OIDC_IDLE_MINUTES` (default 30, `0` off); sliding, capped by the hard limit; a page request after idle lands on the sign-in page with a sentence (`?error=oidc_idle`, catalogue key with Welsh draft). Four unit tests with an injected clock. **Not tested:** the redirect end to end in a browser (it needs the clock to move).
- [x] **RG-3 (M) — Audit trail protected in the database.** A trigger refuses
  `UPDATE` on `audit_logs` and refuses `DELETE` unless the retention task has set
  a transaction-local flag. **Acceptance:** request test that an update and a
  delete are refused; the guarantee and its limit (a database superuser can
  disable a trigger) are stated in the DPIA and the data-protection documents.
  **Built 2026-10-10.** Migration 51: trigger refusing `UPDATE` (unless `ppm.audit_pseudonymise` is set, reserved for RG-6) and `DELETE` (unless `ppm.retention_purge` is set, used by the purge). Request test shows update and delete refused, flagged delete allowed in one transaction, flag not leaking. **Limit, stated:** a database administrator can drop the trigger; `TRUNCATE` is not blocked; there is still no hash chain.
- [x] **RG-4 (L) — Retention purge task.** `cargo loco task retention_purge`:
  per class from the retention schedule, **dry run by default**, prints counts per
  table, deletes only with `apply:true`, writes an audit row per table purged, never
  touches a class marked "No" for deletion. **Acceptance:** DB test that an old
  read notification, a delivered outbox row and an expired export are purged and
  a recent one is kept; dry run changes nothing; the DTAC and DPIA "no purge"
  statements are corrected. **Not in scope:** purging plans, tasks or documents
  (the buyer decides).
  **Built 2026-10-10, partly.** `cargo loco task retention_purge` (`src/retention.rs`): dry run unless `apply:true`; read notifications 90 calendar days, webhook log 30, notification delivery log 30, trend and adoption snapshots 3 years; periods overridable; one audit row per table. Unit and request tests. **Not covered, deliberately:** plans, tasks, time, documents, the audit trail, bulk-job rows and files (the buyer decides; each needs its own rule). The DPIA, DTAC and schedule are corrected.
- [x] **RG-5 (S) — Encrypted backups.** `scripts/backup.sh` encrypts the archive
  when `BACKUP_ENCRYPTION_RECIPIENT` (an `age` recipient) or
  `BACKUP_ENCRYPTION_PASSPHRASE_FILE` is set; `restore.sh` decrypts; the checksum
  manifest covers the encrypted file. **Acceptance:** a round trip in the restore
  test with encryption on; a wrong key fails with a clear message.
  **Built 2026-10-10.** `BACKUP_PASSPHRASE_FILE` or `BACKUP_GPG_RECIPIENT` in `scripts/backup.sh` (gpg); `restore.sh` decrypts into a private temporary directory. Verified by hand with a PostgreSQL 18 server: plain and passphrase drills pass, a wrong passphrase stops before the database is touched, a changed file fails the checksum, both options set is refused; CI runs the encrypted drill too (**not run on GitHub**). **Not tested:** the public-key mode.
- [x] **RG-6 (L) — Data-subject requests.** Export everything held about a person
  reference (`GET /api/admin/subjects/{ref}`), and pseudonymise it across the
  schema (replace the reference with a stable pseudonym, keep audit rows).
  **Acceptance:** a person who appears in tasks, risks, time entries and audit
  rows is exported completely and pseudonymised completely, pinned by a test that
  walks `governance/personal-data-inventory.csv`.
  **Built 2026-10-10, with the plan's recommended policy (pseudonymise, not delete) because the owner has not ruled otherwise (plan §62 q2).** `GET /api/admin/subjects/{ref}` and `POST …/pseudonymise` (preview unless `?apply=true`), `src/subjects.rs` (75 columns), `controllers/subjects.rs`; administrators only (a `subjects` area closed to the policy file); audited without the original reference; the audit trigger is relaxed only inside that transaction. A test walks the inventory so a new identifier column cannot be missed. **Not covered, stated:** free text, uploaded files, audit snapshot JSON, notification destinations (listed, not changed), backups. **Not tested:** a concurrent request during the apply.
- [x] **RG-7 (M) — Handling marks.** A configurable marking scheme (for example
  OFFICIAL, OFFICIAL-SENSITIVE; or Public, Internal, Confidential) on plans and
  controlled documents, shown on every page and in every export, with an ABAC
  rule by clearance attribute. **Acceptance:** an unmarked export is impossible
  when a scheme is configured; a caller without clearance gets 404.
  **Built 2026-10-10, with the scheme left to the deployment (plan §62 q1 is therefore still open as to which preset to ship).** `PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS` is an ordered JSON list, lowest first (for example `["OFFICIAL","OFFICIAL-SENSITIVE"]` or `["Public","Internal","Confidential","Restricted"]`); off when unset, and a malformed value stops the process rather than starting with no marks. A plan has a mark or none (none is the lowest level); a caller's level is their `clearance` attribute; **a plan above the caller's level is `404` to them, administrators included** (decided in the same two places that hide private plans, so lists, search and every cross-plan view follow); a mark the scheme no longer knows counts as the highest; marks can be set only up to your own clearance and on plans you can see. Every response about a plan carries `X-Handling-Mark` (the lowest level when unmarked), which covers exports and downloads; the web client shows the mark as words in a bordered bar, in English and Welsh. Migration 53, pure rules (6 unit tests), an end-to-end test with real tokens (every visibility case above, the set rules, reset, an unknown mark, the database shape checks, audit), three browser tests. **Not done, stated:** it hides and labels, it does not encrypt or stop a cleared person copying; the mark is on the response header and in the web client, not stamped into the bytes of a generated PDF or CSV; mark changes are audited but not routed for approval; aggregate counts that mention a hidden plan by name are hidden (same scope as private plans) but the proof of that is the `at-a-glance` check, not the full census with a scheme on; `clearance` must be issued by the identity provider (the service cannot check it against a personnel system).
- [x] **RG-8 (M) — Attachment safety.** Content-type allow-list by sniffing,
  size limit, and an optional scan hook (ClamAV over a socket) that quarantines.
  **Acceptance:** a renamed executable is refused; a scan failure quarantines.
  **Built 2026-10-10.** `src/content_check.rs`: the bytes must match the declared type (PNG, JPEG, GIF, WebP, PDF, Office files by signature; text and JSON by encoding and parse), and a program or script (Windows, Linux, macOS, Java, `#!`) is refused whatever it claims. An optional `ATTACHMENT_SCAN_ADDR` (`clamd` over TCP, `INSTREAM`) has the last word: infected refuses with an audit row, and a configured scanner that cannot answer **fails closed** (503). Unit tests including a fake `clamd`; request tests for both. The existing fixtures `b"%PDF"` became `b"%PDF-1.7"` (a real PDF header). **Not done, stated:** no parsing of PDFs or Office files for macros or exploits (a scanner's job); a file valid as two formats passes as the declared one; a Unix-socket `clamd` and a `quarantine` area for later review are not built (a refused file is never stored); the scanner itself was tested only against a stand-in, not a real `clamd`.
- [x] **RG-9 (M) — Access review.** A report of who holds what access, when it
  was last used and who granted it, exportable as CSV for recertification.
  **Built 2026-10-10.** `GET /api/access-review` (JSON or `?format=csv` with a decision column and spreadsheet formula defusing); `src/access_review.rs` (standing grants from the access ledger: latest entry per person, plan and access; age in calendar days; `review_due` at `older_than_days`, default 365); administrators only (an `access_review` area closed to the policy file); readable plans only; audited. Unit and request tests. **Deviation, stated:** it reports what people **recorded** in the access ledger, not what the identity provider enforces (the service cannot read that); it does not record the reviewer's decision back (the CSV is the artefact; recording decisions is a follow-up).
- [x] **RG-10 (M) — Break-glass access.** Time-limited elevated access with a
  required reason, a second-person notification and a loud audit entry.
  **Built 2026-10-10.** `POST /api/break-glass` (plan, reason of at least 20 characters, 1 to 240 minutes, a second person to tell), `GET /api/break-glass`, `POST /api/break-glass/{pid}/revoke`; migration 52 (the database itself refuses a window over four hours and a thin reason); policy area `break_glass` for `breakglass=eligible` or `access=admin`, closed to the policy file. A live grant makes the grantee a temporary member **for visibility only** in the two places that decide whether a private plan is hidden (`privacy_mw` and `hidden_plans`, so lists, aggregates and every plan route follow); each request let through only by the grant is audited as `break_glass_access`. The second person is told in the same transaction and **if they cannot be told no grant is made**; the kind is deliberately not in the preferences list, so it cannot be switched off. At most 5 a person a day, one live grant per plan and person. End-to-end test with real tokens (who may ask, every refusal, visibility before/during/after, expiry, revocation by holder or admin only, the limit, the database checks); unit tests for the rules and the policy. **Not done, stated:** aggregate views opened by a grant are not audited per request (only plan routes are); a grant is visibility only, so a grantee without write attributes still cannot write; no alert beyond the in-app notice to one person (a chat or e-mail sink applies only if that person has registered one).
- [x] **RG-11 (M) — Load and soak test.** A repeatable script and published
  results (latency percentiles, error rate, database size sweep) in
  `BENCHMARKS.md`. **Acceptance:** results recorded with date, machine, command.
  **Built 2026-10-10.** `scripts/load-test.mjs` (`seed`, `run`, `soak`; Node and `oha`); results, machine and caveats in `BENCHMARKS.md`. **Findings:** ordinary requests on 100-task plans answer in tens of milliseconds at 32 connections; a plan of 12,667 tasks is a cliff (task list ~1.9 s median and ~400 KB, critical path ~2.5 s, memory to ~1.2 GB); bulk writing through the API ran at about 100 rows a second (cause not investigated); a six-minute soak showed 4 errors in about 800,000 requests and flat memory. **Deviations, stated:** client, service and database shared one machine; login and rate limiting were off; run-to-run variation was 2 to 4 times, so the numbers are an order of magnitude, not a capacity plan; no long soak, no larger database, no front-end profile.
- [x] **RG-15 (M) — Page a plan's task list.** `GET /api/plans/{pid}/tasks` returns every task (about
  400 KB for 12,667) and took ~1.9 s at the median under load. Page it the standard way
  (`limit`, `offset`, `X-Total-Count`) with the counts kept in the body, keep the old shape working for
  callers that send no paging parameters up to a stated cap, and move the UI to pages. **Acceptance:** the
  list-paging census names it; the large-plan scenario of `scripts/load-test.mjs` improves and the figures
  are recorded in `BENCHMARKS.md`.
  **Built 2026-10-10.** `GET /api/plans/{pid}/tasks` accepts `limit` (default 100, at most 500) and `offset` (at most 10,000) with `X-Total-Count`, `X-Limit` and `X-Offset`; the body adds `total`, `truncated`, `limit` and `offset`, and `counts` now come from one grouped query over every matching task. With no paging parameters the answer is the old shape capped at 2,000 tasks (`truncated` says if more exist). The board page shows a notice when truncated (catalogue text with Welsh). Unit test of the window rule and a request test that walks the pages. **Not done, stated:** the improvement was **not re-measured** on the 12,667-task plan (the load-test database was gone); the work-breakdown and earned-value pages still read the first 2,000 tasks without saying so; the board does not page, it only warns.
- [x] **RG-16 (M) — Critical path on very large plans.** The calculation took ~2.5 s at the median for
  12,667 tasks with 99 dependencies, and the process grew to ~1.2 GB. Profile it, remove the avoidable work,
  and decide a stated cap above which the answer is a clear refusal or a background job, never a timeout.
  **Acceptance:** a recorded before and after on the large plan; the cap is in the OpenAPI text.
  **Progress 2026-10-10 (not ticked: the cap in the acceptance is not built).** A quadratic search in the per-task loop of `GET /plans/{pid}/cpm` is now a map lookup; one request on a 12,667-task chain went from 879 ms to 331 ms (1 connection) and 1,184 ms to 533 ms (4 connections); `?only=critical` keeps only critical and near-critical tasks and the dependencies between them (request test on a network with float). The pure calculation is 52 ms for 12,667 tasks, so the rest is loading (~110 ms), building the answer (~135 ms) and its 10.8 MB size. **Still to do:** decide and state a size above which the full answer is refused or becomes a background job; try loading the network with one query per table instead of an `IN` list of every task id; consider compressing or streaming the answer. The earlier 2.5 s figure was measured on an unplaced network and is withdrawn.
  **Completed 2026-10-10.** A stated size: above 5,000 tasks (`PROJECT_PORTFOLIO_MANAGEMENT_CPM_FULL_MAX_TASKS`) the full answer is refused with `413` and a message naming the two ways out, `?only=critical` (always allowed) and `?full=true` (asked for knowingly); unit test of the rule and a request test with the ceiling set low. Together with the earlier changes (the quadratic lookup removed, 879 to 331 ms on a 12,667-task plan) this meets the acceptance. **Still open, stated:** loading the network still uses an `IN` list of every task id (about 110 ms of the 330) and was not changed; the web client's critical-path page shows the 413 message for a plan over the size instead of offering the critical part; the 413 was not measured under load.
- [x] **RG-12 (S) — Shared session store.** An optional Redis- or
  PostgreSQL-backed store for OIDC sessions, so several front-end replicas need
  no sticky sessions.
  **Built 2026-10-10.** `OIDC_SESSION_STORE=redis` + `REDIS_URL`. A small dependency-free RESP client (`src/lib/server/resp.ts`: `redis://` and `rediss://`, `AUTH`, `SELECT`, one connection, per-command timeout), `RedisSessionStore` (atomic `GETDEL` so a login state is spent once, `SET … XX KEEPTTL` so a late write cannot resurrect a deleted session, idle timeout shared across replicas, refreshed tokens written back), identifiers validated as random tokens before they reach a key, corrupt or foreign values treated as no session and removed, sign-in starts capped at 600 a minute, Redis down means signed out. **Tested:** 23 unit tests against a fake server and the same checks against a real Redis 7 with a password; the whole browser sign-in flow (7 tests) with sessions in a real Redis, locally; CI runs both against a Redis service container (**green on GitHub**, run 2026-10-10). **Not tested:** two live front-end replicas at once (the store-level test of a second client seeing refreshed tokens stands in); a TLS Redis; Redis Cluster or Sentinel (not supported).
- [x] **RG-13 (outside the software) — Assurance checklist.** Penetration test,
  manual assistive-technology audit, clinical safety review (if clinical content
  is ever in scope), disaster-recovery exercise, DPO sign-off, supplier
  assurance. **Acceptance:** `governance/assurance-checklist.md` lists each, who
  owns it, what evidence an assessor will ask for and what the project can supply.
  **Built 2026-10-10 (the checklist only).** `governance/assurance-checklist.md` lists 14 assurance items with owner, what the project supplies, and state; every item is **open**. The software cannot do these.
- [x] **RG-14 (S) — Readiness document and corrected claims.**
  `governance/production-readiness.md` (the verdict, honestly) and DTAC/DPIA rows
  brought into line with the code. **Acceptance:** link checker passes; no row
  claims a control that the code lacks.

## Not taken (director pack, escalation, production readiness)

A P3M3 maturity score; an AI register; a live finance-ledger connector;
computing escalation levels; auto-pausing a programme; a web application
firewall in the service; writing the DPIA's legal conclusions. Reasons in
[plan.md §42 and §49](plan.md). (CCPM feeding and resource buffers and
multi-project pipelining, once listed here, are now taken as CC-n.)

## Findings (surfaced while building the backlog)

- **DELETE is admin-only under the default ABAC policy** (`access=admin` ⇒
  destructive/delete; `write` does not include it). Every *self-service*
  removal built as a DELETE — revoke my feed token, unfollow, un-react, reset a
  notification preference, delete my own comment — would therefore be refused
  to an ordinary user the moment enforcement is on. Found by the feeds test
  (enforcement ON). **Fixed** with POST twins (`/api/me/feed-token/revoke`,
  `/api/watchers/unwatch`, `/api/reactions/remove`,
  `/api/me/notification-prefs/reset`, `/api/comments/{pid}/delete`) that need
  only write access; the DELETE routes remain. **Rule for new endpoints:** a
  person's undo of their *own* action is a POST. The older `DELETE /api/saved-views/{pid}` had the same flaw and
  now has the twin `/api/saved-views/{pid}/delete`.

  **Built 2026-10-10.** `governance/production-readiness.md`; DTAC, DPIA and retention rows corrected.
- [x] **RM-8b (L) — Keep private plans out of the aggregate views.** **Landed 2026-10-06.** One shared query — `plan_access::HiddenScope::of(ctx, caller)` (the private plans the caller is not in, and the tasks inside them) — and every aggregate loads through `scope.keep(rows)` / `keep_edges` (a lookup failure is an error, never an empty scope). Routed through it: at-a-glance, capacity, capacity utilization, executive health / alignment / benefits / decisions, financials variance / exposure / forecast, lifecycle funnel, prioritization and Smart Score, proposals forecast, risk heatmap, security and compliance registers and findings, auditor findings, regulator extract, controls coverage, technology debt / flow / radar / dependency risk, engineering blocked / delivery links / milestone calendar / MoSCoW, DevOps metrics and releases, assignee workload, time reports, saved reports (and scheduled ones, which run as an anonymous reader), the plan roadmap and subtree rollups, scenarios (list, evaluate, compare, create: a member list never names a plan the caller cannot see), the proposal that became a hidden plan, and the audit / event / merge feeds. **Acceptance, as tested** (`tests/private_aggregates.rs`, real tokens, a private plan carrying data of every kind at twice the public plan's size): for a non-member every one of ~55 aggregate routes is **byte-identical before and after the private plan appears** (clocks scrubbed), a member's equal an admin's (except where a person's detail is masked), the private plan really does change the admin's totals, a route that names its plan in the query (`?plan=`) is 404 for a non-member, a scenario naming the private plan evaluates for a non-member as if it did not, and **a test enumerates the mounted GET routes: each parameterless one must be in the compared list, the privileged list or an allowlist with a reason, so a new aggregate cannot skip the decision**. The test found **sixteen** leaking routes before the fix. **Deviations, stated plainly:** the admin-only bulk reads (board pack, investments, auditor trail, evidence pack) stay full by design (403 to everyone else); **board trends is now an admin-only read** because its stored snapshots are totals recorded over every plan and cannot be filtered after the fact; the audit and event feeds filter by what each row is about and by plan ids named in it, so an entry that names a hidden plan only in free text is not caught; the test proves the routes it seeds data for — a field no seed exercises is not proven; contention and ordering are untested; `time-reports`' `omitted` count no longer counts private plans' rows (it counted them before). RM-8
  hides a private plan's *records* but cross-plan aggregates (at-a-glance,
  executive, board, financials, reports, capacity, scenarios, TBA rollups, the
  Smart Score ranking) are computed over every plan and still count it.
  Introduce one shared "plans this caller may see" query (live, not hidden by
  `plan_access_settings`) and route every aggregate through it.
  **Acceptance:** for a non-member, every aggregate endpoint's totals equal the
  totals computed with the private plan deleted; a member's equal the full
  totals; a test enumerates the mounted aggregate routes so a new one cannot
  skip the filter.

- [x] **RM-12b (S) — Consistent list paging and `include` on tasks.** **Landed 2026-10-06.** **One layer, not forty edits:** `controllers::paging::paging_mw` gives every `GET` that answers `200` with a **bare JSON array** the same paging — `?limit=` (default and ceiling 500), `?offset=` (above 10 000 is 400, a non-number is 400) and `X-Total-Count` / `X-Limit` / `X-Offset`, the body staying the bare array; handlers that already page in the database (and set the header) are left alone, so **a list added tomorrow pages today**. **Acceptance, as tested** (`tests/list_paging.rs`, real admin token): the test reads the OpenAPI document, seeds one of each kind of record, and calls every documented `GET` it can reach — **39 lists were found and every one** carries the three headers, honours `limit=1`, answers an offset past the end with an empty page that still says the total, refuses an absurd offset and a bad limit, and **documents `limit` and `offset`** (added to the spec from one registry, `openapi::PAGED_LISTS`, which the test keeps equal to what it finds); a documented `GET` it cannot reach must be named with a reason (`UNREACHED`: streams, files, and three lists that need data this census does not set up), so a new list cannot slip past by needing a parameter nobody supplied. **`include` on a task:** `GET /api/plans/{pid}/tasks/{t_pid}[?include=children,relations,transitions,raci,time_entries]` (there was no single-task read): bare task without `include`, `{task, included, omitted}` with it; a module that is off is omitted (`module_disabled`); what names people is omitted for a caller who may not manage them (`masked`, real-token test); an unknown name is **422 listing the allowed ones**; another plan's task and a deleted one are 404. **Deviations, stated plainly:** the layer slices the array a handler already built, so it bounds **what is sent, not what is loaded** (a database-level page would cost less); a list that used to return more than 500 rows now returns 500 and says how many there are; only **bare arrays** are lists — a report or aggregate that wraps arrays in an object is not paged; three lists are unproven by the census (custom-field search, key-result check-ins, a scheduled report's latest run) but are served by the same layer. Split from
  RM-12: give every list endpoint `limit` / `offset` and an `X-Total-Count`
  (only some lists carry them today), and add `include` to task GET.
  **Acceptance:** an OpenAPI-driven test enumerates every documented list
  `GET` and asserts the three paging parameters and the count header; an
  `include` on a task that names an unknown sub-resource is 422 listing the
  allowed names.

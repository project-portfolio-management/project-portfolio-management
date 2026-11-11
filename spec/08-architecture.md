## 8. Architecture

### 8.1 Trio composition

```
+--------------------------------------------------------------+
|              project-portfolio-management-ui-with-svelte                  |
|  SvelteKit 3 SPA · Svelte 5 runes · TypeScript strict         |
|  routes: /plans                                               |
|          /plans/new  /plans/[pid]  /plans/[pid]/edit          |
|          /plans/[pid]/{goals,tasks,issues}                    |
|          /plans/[pid]/{timeline,burndown}                     |
|  lib/api: client.ts → plans.ts + sub-resource repos           |
+------------------------------+-------------------------------+
                               | REST (raw JSON, no envelope)
                               | PROJECT_PORTFOLIO_MANAGEMENT_API_URL (default :5150)
+------------------------------v-------------------------------+
|              project-portfolio-management-service-with-rust                     |
|  Axum 0.8 with axa · port 5150                               |
|  controllers/plans.rs                                         |
|     CRUD + /match + /check-duplicates                         |
|                       + merge + search  (kind-agnostic)       |
|  controllers/{goals,tasks,issues}.rs   (sub-resources)        |
|  controllers/{timeline,burndown}.rs    (derived, read-only)   |
|  controllers/links.rs        entity_links write-side          |
|  controllers/bulk.rs         import/export jobs (bg_pg)        |
|  models/plans.rs + sub-resources                              |
+--------------+-------------------------------+----------------+
               |  path dependency (Cargo)      |
+--------------v---------------+  +------------v---------------+
|  project-portfolio-management-matcher           |  |  PostgreSQL (SeaORM 1.1)   |
|  pure library, no IO         |  |  plans ({…, data JSONB}) +  |
|  MatchingEngine ·            |  |  goals(via JSONB) · tasks · |
|  MatchConfig · Plan ·        |  |  issues · audit_logs ·      |
|  kind-agnostic               |  |  merge_records ·            |
|                              |  |  entity_links · bulk_jobs   |
+------------------------------+  +-----------------------------+
```

Dependency direction is strictly downward: front-end → service →
matcher. The matcher depends on nothing in the workspace (serde,
strsim, unicode-normalization, chrono/time for date proximity,
thiserror only). The service declares
`project-portfolio-management-matcher = { path = "../project-portfolio-management-matcher-with-rust" }` and
uses the matcher's `Plan` directly as its API DTO for the thin
record — there is no adapter layer (the care-pathway posture).

### 8.2 The single collection and the matchable/operational split

The service is **two co-located concerns** behind one app:

- the **identity registry** — CRUD + matching over the thin `Plan`
  on the one `plans` collection (a JSONB `data` column + a nullable
  denormalized `parent_pid`). Matching is **kind-agnostic**: the
  controller feeds candidate plans without kind filtering and the
  matcher never gates on `kind` (§5.5).
- the **project-management tool** — the operational sub-resources
  (tasks, issues, in their own tables keyed by `parent_pid`) and the
  two derived views, hanging off any plan.

Only the registry half touches the matcher. The PM half is ordinary
relational CRUD with events + audit. The one bridge is `goals[]`
(§5.3): goal sub-resource writes mutate `data.goals[]` so the matchable
payload stays current.

### 8.3a Boot and configuration

The service is ordinary [Axum](https://docs.rs/axum) with [axa](https://crates.io/crates/axa), not a framework.
`src/bin/main.rs` calls `App::run`, and every step from there is a function call in `src/app.rs`. There are no configuration
files. Configuration is the environment, with defaults in code, and `APP_ENV` picks the profile.

| `APP_ENV` | Login and rate limits | Migrates on start | Cache | Mail |
| --- | --- | --- | --- | --- |
| `production` | on, and it refuses to start with them off | no | Redis, **required** | `SMTP_*`, STARTTLS |
| `container-dev` | off unless turned on | no | memory, or Redis if named | `SMTP_*`, STARTTLS |
| `development` | off unless turned on | yes | memory, or Redis if named | localhost:1025, no TLS |
| `test` | off unless turned on | fresh each boot | memory | a stub |
| any other name | **on, and it refuses to start with them off: an unknown name fails closed** | no | Redis, **required** | `SMTP_*`, STARTTLS |

`PORT`, `BINDING`, `HOST`, `DB_URL`, `DB_MIN_CONNECTIONS`, `DB_MAX_CONNECTIONS`, `DB_CONNECT_TIMEOUT`, `DB_IDLE_TIMEOUT`,
`HTTP_REQUEST_BODY_LIMIT_BYTE_COUNT`, `HTTP_REQUEST_HEADER_READ_TIMEOUT_SECONDS`, `HTTP_COMPRESSION`, `WORKER_MODE`,
`SHUTDOWN_TIMEOUT_SECONDS`, `SHUTDOWN_DRAIN_SECONDS`, `LOG_LEVEL`, `LOG_FORMAT` and `LOG_FILTER` are axa's (since axa 0.11). CORS,
HSTS, the content security policy and the permissions policy stay with this service's own `http_security` layer, so axa's are turned
off, and neither `HTTP_CONTENT_SECURITY_POLICY` nor `HTTP_PERMISSIONS_POLICY` changes what is sent. axa 0.10 read four of these under
other names, which axa 0.11 ignores: `DATABASE_URL` (now `DB_URL`), `LOCO_ENV` (now `APP_ENV`), `BODY_LIMIT_BYTES` and
`HEADER_READ_TIMEOUT_SECONDS`. **The service refuses to start, in every environment, when an old name is set and its new name is
not**, naming both, rather than silently falling back to a default database, environment or limit. The body limit
(2 MiB unless set) binds every request body, read by an extractor or not; the attachment upload and the plan, MS Project
and tracker imports raise it to their own caps with axa's `middleware::body_limit`, and a census test keeps Axum's
`DefaultBodyLimit`, which since axa 0.11 raises only the extractors' limit, out of the controllers. There is no request timeout
unless `REQUEST_TIMEOUT_SECONDS` is set, because a timeout would cut the live event streams. A `GET` whose `ETag` matches
`If-None-Match` answers `304`, whichever handler set it. `/_health/ready` answers `503` when the database or the cache does not. One list of trusted proxies, `TRUSTED_PROXIES`, which axa
reads, decides both whose `X-Forwarded-For` gives the client address and whose `X-Forwarded-Proto` is believed for HSTS.
`PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES`, the old name, still works alone with a warning; set to a different list, the
service refuses to start. An idle connection is kept for `KEEP_ALIVE_TIMEOUT_SECONDS`, 75
by default, so a load balancer that keeps idle connections longer needs a higher value. A client that stops reading a
response is dropped after `SEND_STALL_TIMEOUT_SECONDS`, 60 by default; any write that goes through restarts it.

Every error is an RFC 9457 problem, `application/problem+json`: `type`, `title` and `status`, `detail` where there is more to say,
and the machine-readable `code` (such as `not_found`, `forbidden` or `idempotency_key_reused`), with `errors` for a validation
failure. A `500` carries no `detail`. Successes stay bare JSON. A body, path or query value the handler cannot read is a `400`
(a body's media type `415`, its shape `422`) whose `detail` says which part was wrong and quotes nothing the caller sent: handlers
take `axa::extract`'s `Json`, `Path` and `Query` through `crate::prelude`, and a census test keeps Axum's out. The service's own refusals (login, policy, idempotency, paging)
are problems too, and a guest's `404` is byte for byte the one a missing record answers. A violated unique or foreign key that
no handler maps answers `409`, and a violated check or not-null constraint `422` (`unprocessable_entity`), naming no column or value.

`db down`, `db reset` and `db truncate` destroy data, so they refuse to run unless `APP_ENV` is `development` or `test` **and**
`AXA_DESTROY_DATA=true` is set. No other environment can run them, whatever is set, and `--environment` cannot lift this: both
the process's own `APP_ENV` and the one the command names must allow it. axa's parser (`cli::parse`, and `Cli::parse` since 0.6) refuses
them, exiting `1` before the service connects to anything, so `bin/main.rs` must read its arguments through axa. Rollback in production
is a restore (`operations/upgrade-and-migration.md`).

The three health routes are axa's and follow Kubernetes' probes. `/_health/live` asks nothing. `/_health/ready` asks the database,
the queue and the cache, names what failed in a `503`, gives each check a time limit, reuses one answer for 5 seconds, because the
route is public, and answers `503` `shutting_down` from the moment shutdown begins. `/_health/startup` asks only the database and,
once it has passed, passes for good. The routes axa 0.10 served, `/_ping`, `/_health` and `/_readiness`, answer `404` (or `401`
with login on, as they are no longer public). A database that cannot serve for now (no free connection, a lost connection, or a
read-only, shutting-down or full server) answers `503` `unavailable` with `Retry-After: 5`, not `500`. `check` reads the configuration, connects to nothing, and
fails where a boot would for the settings it knows. Stopping is one limit for requests and jobs together:
`SHUTDOWN_TIMEOUT_SECONDS`, 25 by default, counted from the end of `SHUTDOWN_DRAIN_SECONDS` (0 by default), for which the service
still accepts requests while readiness refuses. In production, a `DB_URL` that leaves this machine must set `sslmode` to `require`,
`verify-ca` or `verify-full`, or the service refuses to start. `compose.yaml` runs in production, so its PostgreSQL
offers TLS (`ci/postgres-tls`: a self-signed certificate made on first start, TLS 1.3 only) and the API connects with
`sslmode=require`, encrypted but with the certificate unchecked. A database outside the stack should use `verify-full`.

### 8.3b The move from SeaORM to sqlx

The service is moving from SeaORM to plain [sqlx](https://docs.rs/sqlx), one controller at a time. The two share one connection
pool, so both work side by side. A transaction is not shared, so all of one handler's writes stay on one side, including the audit
row: `AuditModel::record` is for SeaORM and `AuditModel::record_on` for sqlx. `record_on` fails with sqlx's own error
(`axa::Error::Sqlx`), never wrapped, so where a caller passes it on (`?`) a database that cannot serve still answers `503`
`unavailable` with `Retry-After: 5` (§8.3a) and a broken constraint its `409` or `422`, as `record`'s SeaORM error did; a
caller that refuses the write with `audit_failed` answers `500` `audit_unavailable` on either side.

`src/db.rs` is the only way to sqlx: `db::pool` and `db::begin`. Both count each statement in `query_count`, where SeaORM's
statements are counted too, so a guard on how many statements a route runs covers both sides. A census test refuses the raw pool
anywhere else. An update names only the columns it changes, as SeaORM's did, and no row hook exists to carry across. A helper that
runs several statements takes `&mut impl db::Session`, which is a transaction or the pool, so the caller decides whether they share
one.

How far it has got (task SQ-1 in `tasks.md`): of the 147 files under `src/` (outside `src/models/_entities`) that named the SeaORM
API on 10 November, 134 have moved (batch 1: `approvals`, `processes`, `controlled_documents`, `onboarding`, `handover`,
`meetings`, `meeting_actions`, `lessons`; batch 2: `okr`, `kpis`, `controls`, `value`, `benefit_validation`, `frameworks`,
`key_persons`, `ledger`; batch 3: `financials`, `funding`, `tolerances`, `thresholds`, `effort`, `timesheets`,
`milestones`, `milestone_signoffs`, `announcements`, `components`; batch 4: `escalations`, `deescalation`, `confidence`,
`exceptions_digest`, `schedule_baselines`, `cost_risk`, `schedule_risk`, `tpc`, `director_packs`, `pack_rhythm`; batch 5:
`fever`, `programme_gates`, `programme_profile`, `capacity_heatmap`, `cutover`, `status_updates`, `register`,
`earned_value`, `insights`, `organizations`; batch 6: `start_control`, `drum`, `buffer_management`, `availability`,
`portfolio_resources`, `ccpm_estimates`, `hammock`, `cpm`, `plan_roles`, `raci`; batch 7: `calendars`, `task_relations`,
`assignments`, `sandbox`, `deployment_moves`, `phase`, `guests`, `watchers`, `visibility/dependencies`,
`visibility/allocations`; batch 8: `workflow`, `engineering/board`, `engineering/agile`, `engineering/devops`,
`engineering/estate`, `engineering/mod`, `task_bulk`, `task_tree`, `tba`; batch 9: `governance`, `discussion`; batch 10:
`issues`, `issue_configuration`, `custom_fields`; batch 11: `collaboration`, `feeds`, `notification_prefs`; batch 12:
`automation`, `break_glass`, `sinks`, `tasks/exception_breaches`, `plan_access`; batch 13: `streaming`,
`controllers/plans`, `plan_links`, `planner`, `models/event_outbox`; batch 14: `attachments`, `email_deliveries`,
`report_schedules`, `plan_templates`; batch 15: `blueprint_sync`, `mspdi_import`, `mspdi_batch`, `mspdi_export`,
`tracker_import`, `bulk`, `tba/persistence`; batch 16: `cost_rates`, `change_requests`, `canvas`, `ceremony`,
`oversight`, `snapshots`; batch 17: `strategy`, `scoring`, `prioritization`, `modules`, `pdf`, `handling_marks`,
`settings`, `compliance`, `subjects`, `visibility/milestones`, `visibility/reports`, `visibility/dashboard`,
`visibility/mod`; batch 18, the infrastructure: `app.rs`, `scheduled_jobs`, `retention`, `webhooks`, `relay`,
`idempotency`, `flow_metrics`, `tasks/search`, `tasks/outbox_retry_failed`, `controllers/mod.rs`, `db.rs`) and 13 are
left (the grep that measures it counts 15, because `engineering/board` and `task_bulk` name enum variants `Set` of their
own; so does `src/task_bulk.rs`, so 12 name the API). Those 12 are step (c)'s: `query_count`'s metric callback, which
counts what still runs through `ctx.db` as a SeaORM connection (the tests' own statements; the migrator's moved to
sqlx in step (b)), and the
`ActiveModelBehavior` impls and entity re-exports of `models/behaviors`, `canvases`, `strategy`, `capabilities`,
`visibility`, `governance`, `plans`, `merge_records`, `webhook_deliveries`, `saved_views` and `audit_logs`. Beyond the
grep, `db::pool`, `db::begin` and `db::for_axa_crate` reach the
pool through SeaORM's connection until `ctx.db` is the `PgPool` (step (c)). No statement of the service's own runs
through SeaORM after batch 18. Batch 9 also moved every statement of `plan_access`, `controllers/mod.rs` (`find_live_plan`,
`find_live_task`) and `models/governance`; `models/governance` keeps its `ActiveModelBehavior` impls. Batch 11 gave `plan_access::is_none_of` a sqlx form,
`plan_access::push_none_of`, and moved `HiddenScope::notes_visible` to sqlx (it now pushes its condition onto a
`QueryBuilder`). Batch 12 removed the SeaORM `is_none_of`, `capabilities::notify` and `sinks::enqueue` and the SeaORM
finders and writers only `automation` used: `notify_on` and `sinks::enqueue_on` serve every caller (break-glass calls
`notify_on` in its own transaction, so a grant and its notice still stand or fall together); `find_automation`,
`find_scheduled_action`, `enabled_automations` and `due_actions` take the context, and `record_run_on` and
`schedule_on` take a `db::Session`. `models/capabilities` keeps only its `ActiveModelBehavior` impls, for step (c).
Batch 13 moved the plan record's core: `streaming` writes a plan, its `axa_outbox` event and its audit row in one
`db::begin` transaction under the `outbox` transport (`db::Tx::write_outbox` hands it to `axa::outbox::write`, the
sqlx form of `write_seaorm`, and counts the statement); the SeaORM `update_data` and `soft_delete` are gone
(`update_data_on`, `soft_delete_on` and `create` take a `db::Session`), and the plan finders, `merge_records` and
`AuditModel::recent` and `for_entity` take the context. `models/plans` and `models/merge_records` keep only their
`ActiveModelBehavior` impls, for step (c).
`tba::insert_transition_on` writes every task transition; the SeaORM `tba::transition_row` is gone (batch 15), and
so is `AuditModel::record_or_log`: `record_or_log_on` serves every write that keeps a failed audit, and the census
of those writes counts it. `AuditModel::record` and `controllers::db_err` went with their last callers, `retention`
and `scheduled_jobs` (batch 18): every audit row is written by `record_on`. A scheduled job's claim and its closing
record are each one transaction with their audit row, which still refuses with `audit_failed`; a retention sweep's
delete of each table, its flag and its audit row are one transaction. `outbox_retry_failed` releases every failed
event in one `db::begin` transaction (`db::Tx::release_outbox`, counted as `write_outbox` is). `models::visibility`'s finders take the context and read the pool, and
`modules_in_force` and `settings::refresh` take the context (batch 17). `db::Tx::rollback` rolls back and says whether
it worked, for a caller that answered an error when `SeaORM`'s rollback failed (the subjects preview). The batch import's rate tables, rates and fixed costs are written by `cost_rates`' sqlx helpers, each with
its audit row in one transaction (batch 16). `snapshots::capture_on` takes a `db::Session`: `POST /api/board/snapshots`
writes the snapshot and its audit row in one transaction, and the optional ticker captures on the pool. The auditor's
trail and evidence pack still write their audit row on the pool before reading, and are not served when it cannot be
written (NFR-6). The MS Project import
writes its tasks and links in one transaction and its audit row after it, on the pool, and a failed audit row fails
the request (NFR-6): it is not one transaction, and a re-import converges. A scheduled report run writes its artifact row, the schedule's outcome
and its audit row in one transaction (batch 14). `custom_fields::store`, the SeaORM writer of custom values, is gone: `custom_fields::store_on` serves
the board, the bulk routes, `PUT /api/custom-fields/values` (each in its transaction) and the MSPDI batch import (on the
pool). `ctx.db` is still SeaORM's connection (step (c)).

**Migrations (step (b), 11 November 2026).** The migration crate runs on sqlx, not `sea-orm-migration`. Each of the
143 migrations is a SQL file in `migration/migrations/`, frozen from the statements its Rust migration ran on
PostgreSQL 18, in order (captured from the server's statement log, one migration per transaction), so the data
migrations keep their logic as SQL (`INSERT … SELECT`, `UPDATE … FROM`, `DO` blocks, `IF NOT EXISTS`) rather than
one run's effect. A migration is versioned by its SeaORM name (`m20261109_000109_axa_outbox_record` is
`20261109000109`, so the order is the same). One that rolls back has an `.up.sql` and a `.down.sql` (142); one that
refused to roll back is a plain `.sql`, refused by `db down` with its old reason (`REFUSED`, one:
`m20261109_000109_axa_outbox_record`). sqlx's migrator applies them, each in its own transaction, under its
advisory lock, and refuses a history naming a version it does not have or a file whose checksum changed. Its own
`undo` would skip a migration with no down file and go on to the older ones, so `db down` is the crate's: newest
first, each with sqlx's `revert`, stopping with the refusal at a migration that has none, as SeaORM's did.
`db reset` drops what SeaORM's `fresh` dropped (the base tables, then the non-array types of no extension, in
`current_schema()`) and migrates. `db status` reads and never writes.
On a database the SeaORM migrator built, `db migrate` (and the boot, where it migrates) first copies, in one
transaction, every migration recorded in `seaql_migrations` into `_sqlx_migrations`, marked applied with its frozen
file's checksum and SeaORM's time, and only when `_sqlx_migrations` does not exist yet, so a later `db down` is not
undone by a second copy. It refuses, changing nothing, when `seaql_migrations` names a migration the binary does
not know, and checks that on every run. `seaql_migrations` is left in place. The runner takes the pool through
`db::for_migrations`, uncounted by `query_count`. The move is one-way for an operator: an older image reads only
`seaql_migrations` (`operations/upgrade-and-migration.md`). Step (b) is done: a database the old migrator built and one the new runner
built give the same `pg_dump` schema and data but for the history tables, and an old-built database bridged in place
is unchanged but for `_sqlx_migrations` (task SQ-1; `tests/migrations.rs` holds the bridge and rollback cases).

### 8.3 Service layout

> **Corrected 2026-08-25.** This section was headed "— planned" and drew
> a layout that never landed: a controller per sub-resource
> (`goals.rs`, `tasks.rs`, `issues.rs`, `timeline.rs`, `burndown.rs`),
> plus `links.rs` and `bulk.rs` for features §14 lists as **open gaps**
> (T-7, T-8). The real crate groups controllers by *capability* rather
> than by resource. Drawn from the tree, not from the plan:

```
project-portfolio-management-service-with-rust/
├── src/
│   ├── app.rs                       plain functions: routes, workers, boot
│   ├── bin/                         command-line entrypoint
│   ├── controllers/
│   │   ├── plans.rs                 CRUD + match + check-duplicates + merge + search
│   │   ├── engineering.rs           tasks, board, sprints, milestones, burndown
│   │   ├── governance.rs            proposals, gates, risks, budget, benefits
│   │   ├── strategy.rs              ideas, scenarios, objectives
│   │   ├── collaboration.rs         reviews, assignment, notifications
│   │   ├── automation.rs            rules, runs, scheduled actions
│   │   ├── prioritization.rs        Smart Score + ranked queue
│   │   ├── visibility.rs            lifecycle funnel + readiness
│   │   ├── insights.rs              executive / financial / technology views
│   │   ├── oversight.rs             auditor, compliance, regulator extracts
│   │   ├── tba/                     time-based analysis (§12): mod.rs + one file per section
│   │   ├── compliance.rs            integrity + evidence
│   │   ├── metrics.rs docs.rs       Prometheus · OpenAPI + Swagger UI
│   ├── models/                      plans + sub-resource CRUD helpers, _entities/
│   ├── pure rule modules            governance.rs · engineering.rs · strategy.rs ·
│   │                                collaboration.rs · automation.rs ·
│   │                                prioritization.rs · lifecycle.rs · insights.rs ·
│   │                                visibility.rs · tba.rs · merge.rs · validation.rs
│   ├── auth/ (verifier · enforcement · policy · areas · identity) streaming.rs relay.rs search/ privacy.rs compliance/
│   ├── flow_metrics.rs scheduler.rs snapshots.rs version.rs
│   └── workers/                     background jobs
├── migration/migrations/            the frozen SQL migrations, run by sqlx: plans,
│                                    audit_logs, merge_records, event_outbox, governance,
│                                    visibility, strategy, engineering, capabilities, …
├── migration/src/lib.rs             the runner and the bridge from seaql_migrations
├── config/{development,production,test}.yaml
└── tests/                           matcher-embedding + request-level + enforcement + masking
```

**The pattern worth noting:** each capability is a **pure, DB-free rule
module** (`governance.rs`, `tba.rs`, `lifecycle.rs`, …) with a thin
controller over it. That is what makes the rules exhaustively
unit-testable without a database, and it is the shape the §6.4b / §6.4c
work should follow — a `workflow.rs`, an `okr.rs`, a `value.rs`, each
pure.

**Not present, and tracked as gaps rather than drawn as if built:**
`links.rs` / `entity_links` (T-7) and `bulk.rs` / `bulk_jobs` (T-8).

Run with `cargo run -- start` (needs PostgreSQL; `auto_migrate` on in
development). The front-end runs with `pnpm dev` against
`PROJECT_PORTFOLIO_MANAGEMENT_API_URL` (server side only; the browser calls the UI's `/api/proxy/…`).

### 8.4 Matching data flow

- **`/plans/match`** — request carries `{query, candidates}`
  (thin records); the controller calls `MatchingEngine::rank` and
  returns the scored pairs. No database access. `kind` labels do not
  affect scoring (§5.5).
- **`/plans/check-duplicates`** — request carries a `Plan`;
  the controller loads up to a capped set of active rows, deserializes
  each payload, calls `match_plans` per candidate, and returns hits
  with `is_match == true`, sorted by score.
  *(roadmap: replace the full scan with search-based candidate blocking
  — OQ-2.)*
- **create-time** — the same path runs on `POST /api/plans` to
  back the `409` real-time duplicate detection (FR-11a).

### 8.5 Cross-service & federation topology

A plan references other entities by `EntityRef` (sponsor org,
lead, assignees) and a parent plan by `parent_ref`, and may
carry `entity_links` to **any** entity, per
`agents/share/cross-service-linking.md` (shared family guidance, not in this repository).
The portfolio service is a **write-side** participant: it stores
outbound edges locally and emits `linked` / `unlinked` events on the
bus; it does **not** call the target service on the write path
(optimistic integrity, §5 there). The read-model aggregator
(`link-graph-service`) consuming those events to answer graph queries
is a separate service, out of scope for this trio (§2.3). `EntityRef`s
are never a match signal (§7 there); `parent_ref` is an in-entity
reference and **is** a (supporting) match signal (§5.5).

### 8.5a Family bridges (FB-n)

Decided 2026-10-30 at the owner's direction, for the family of three apps that meet here: contact-relationship
management (CRM), this portfolio service (PPM) and workforce planning (WPM, which **is** the worker service:
one base URL, `PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL`). Each app owns its model and is complete without
the others: with no link configured nothing is called, and a peer that is down, absent or slow never fails a PPM
request and never changes a number silently. WPM's side is in its `spec/cross-app-bridges.md` (WPM-R213,
WPM-D135, WPM-D138 to D141).

| Bridge | Direction | Contract | State in PPM |
| --- | --- | --- | --- |
| Delivery ideas | CRM → PPM | `POST /api/ideas` `{title, pitch, tags}` → `{pid}` (CRM's `POST /api/bridges/ppm/deals/{pid}`) | exists (PPM's route; CRM calls it); **checked live** 2026-10-30 (FB-8): a won deal in CRM opened one PPM idea, idempotent |
| Pool link | PPM ↔ WPM | `skill_pools.peer_pool_ref` = WPM's pool pid, set on both sides by a person | **built** (FB-1); set and removed on the `/skill-pools` screen (FB-6); **checked live** on both sides (FB-8) |
| Supply in | WPM → PPM | `GET {WPM}/api/capacity/peer-supply?pool=&from=YYYY-MM-01&months=N` | **built** (FB-2); shown on the `/capacity-heatmap` screen with its source and reading (FB-6); **checked live** through the API (FB-8): `live` with `as_of` and age, the entered-here figure not added, `hidden` under WPM's floor, and with WPM stopped `last_received` with its age and a refused caller `unknown` |
| Demand out | PPM → WPM | `POST {WPM}/api/capacity/peer-demand` | **built** (FB-3); zero sent as the withdrawal, and every sent figure withdrawn on unlink (FB-5); set on the `/capacity-commitments` screen, which shows the send outcome and the recent sends (FB-6); **checked live** through the API, sent and idempotent (FB-8: WPM held one `ppm` claim, `fte_centi` 250); the withdrawals (FB-5) **not** checked live |
| People impact | WPM → PPM | `GET {WPM}/api/capacity/people-impact?programme=thing:<plan pid>[&ends_on=YYYY-MM-DD][&months=N]` | **built** (FB-4): `GET /api/plans/{pid}/people-impact`, shown on a programme's plan page; **checked live** (FB-8): WPM's answer passed through unchanged, `ends_on` from `target_date`, a month-only date sending none |
| Hire and leave | WPM → CRM | `worker_hired`, `worker_terminated`/`worker_retired` webhooks | CRM's and WPM's; nothing in PPM |

**The live check (FB-8, in part).** On 2026-10-30 WPM, PPM and CRM were run together on one machine with
authentication on, one PASETO test key trusted by all three and the caller's token forwarded: PPM `e011cc7`, WPM
`36426f5`, CRM `fad29bd`, that is before FB-5 and FB-6. What passed is marked "checked live" above. It found the
cross-caller kept-figure leak, fixed since (rule 4, `bbad76e`). **Not exercised live:** the FB-5 withdrawals (a
zero, `sent_at`, and the withdrawals on unlink) against a real WPM (WPM supports them since `ab1ce88` and
`ca1462f`); the FB-6 screens against a live service; Keycloak; a mounted ABAC policy.

**The rules for the PPM ↔ WPM bridge.**

1. **Programme identity.** A PPM plan is named in WPM as `thing:<plan pid>`. PPM sends the plan of a commitment
   (which must be of kind Program) under that name; no other mapping exists.
2. **The pool link is explicit.** `PUT /api/skill-pools/{pid}/peer-link` `{peer_pool_ref: <uuid> | null}` sets or
   clears it: a lower-case hyphenated UUID (as WPM writes its pids), never a name, at most one PPM pool per WPM pool
   (a partial unique index; `409`), audited (`skill_pool_peer_linked`, `skill_pool_peer_unlinked`) with no figure.
   The pool list shows the link. An unlinked pool behaves exactly as before.
3. **One owner per fact for a linked pool: supply is WPM's, demand is PPM's.** `GET /api/capacity-heatmap` counts,
   for a linked pool, WPM's supply alone (hundredths of an FTE, shown in FTE) with `supply_source: "wpm"` and a
   `peer_supply` reading: `live`, `last_received` (with `as_of`, the time WPM took it, and `age_seconds`),
   `hidden` (WPM hides a month made up of fewer workers than its floor) or `unknown` (nothing received). The last
   figure is shown only when WPM could not be reached; when WPM **refuses** the caller (4xx), the month is `unknown`,
   so a caller WPM turns away never sees a figure another caller's token fetched (WPM-D135). A hidden
   or unknown month is the state `supply_unknown`, never zero and never a fall back to the local figure. Everything
   PPM holds on the supply side for that pool (its people's availability, its own and partners' supply commitments)
   is kept and shown as `entered_here_supply_fte`, and is **never added** to WPM's figure. Demand stays PPM's
   (allocations plus demand commitments) on both sides.
4. **WPM is asked under the caller's own token** (WPM-D135: no shared service account), with a 2-second timeout and
   no redirects, all linked pools in parallel. The base URL must be `https://` (or `http://` to `localhost`,
   `127.0.0.1` or `::1`); with `PROJECT_PORTFOLIO_MANAGEMENT_OUTBOUND_STRICT` on, it must also resolve to a public
   address; any other URL reads as `unreachable` with that reason. With no base URL (stub mode), no token, or a
   failure, the reading says why. Only the live heatmap asks. **Figures received are kept in this process only, by caller**: the
   authenticated subject (`sub`), or, when the request has none, the SHA-256 of its bearer token (the token itself
   is never kept). When WPM cannot be reached, the heatmap shows the last figure **this caller's** token fetched,
   with its age, or `unknown`; it never shows a figure another caller's token fetched (WPM-D135). The director
   pack, the drum, the pipeline and the start check make no call and show **no kept figure**: a linked pool's
   supply is `unknown` there, with the reason. (The pack is stored and read by others, and the start check looks
   at every plan for no one caller, so a figure kept for one caller would reach another.) The store is bounded:
   at most 1 000 callers (the one not seen for longest is dropped) and 2 400 figures per caller (the
   oldest-received is dropped). What remains, accepted and recorded here: figures are lost at restart (then
   `unknown` until WPM answers again); one subject's figure is shown to the same subject under another of its
   tokens, even one WPM would answer differently; and those four views cannot use WPM's supply for a linked pool,
   so the drum and pipeline treat that pool's supply as unknown.
5. **Demand out.** When a demand commitment on a linked pool is set (`PUT /api/capacity-commitments`, kind
   `demand`), PPM sends `POST {WPM}/api/capacity/peer-demand` with `{pool_pid: <WPM pool pid>, programme_ref:
   "thing:<plan pid>", month: "YYYY-MM-01", fte_centi, peer_event_id, sent_at}`. `fte_centi` is the programme's **total**
   demand commitments on the pool for the month (every organization's), rounded half up to hundredths, because WPM
   keeps one figure per programme, pool and month and would otherwise overwrite one partner's figure with
   another's. `peer_event_id` is `ppm:<pool>:<plan>:<YYYY-MM>:<microseconds of the write>`: the same version, the
   same id (WPM then changes nothing); a new write, a new id. `sent_at` (RFC 3339, UTC, to the microsecond) is the
   time of the PPM write that produced the figure, the same moment the event id names, so that WPM can ignore an
   older event that arrives after a newer one (two sends are not ordered on the wire). The send runs after the write has been stored and
   never holds it up or fails it; the caller's token is held in memory for that one send and **never stored**, so
   there is no outbox and no retry: setting the commitment again sends again. The outcome (`started`, `sent`,
   `duplicate`, `refused`, `unreachable`, `not_configured`, `not_sent_no_token`, `not_sent_too_large`) is in the
   `PUT` answer (`peer_demand_send`) and in `peer_demand_sends` (the latest per pool, programme and month; an older
   version never overwrites a newer one), shown on `GET /api/capacity-commitments` as `peer_demand_send`
   `{outcome, detail, fte_centi, peer_event_id, at}` with each demand commitment that has a send recorded (or, on a linked pool, `null` when none is). `not_sent_zero` is no longer produced (FB-5);
   the database still accepts it, for rows written before.
6. **Zero withdraws (FB-5, family contract decided 2026-10-30 at the owner's direction).** A total that rounds to
   zero is **sent** as `fte_centi: 0` with a new `peer_event_id` and its `sent_at`; WPM then withdraws its `ppm`
   claim for that pool, programme and month. It is recorded like any other send (`sent` with `fte_centi: 0` and
   the detail "WPM took the withdrawal"). PPM still has no route to delete a commitment; setting it to zero is the
   way to withdraw. **Unlinking a pool** (or changing its link to another WPM pool) withdraws, best effort, every
   figure PPM sent to the WPM pool it was linked to: for each programme and month of that pool whose latest
   recorded send carried a figure above zero (whatever its outcome, since an `unreachable` send may still have
   arrived), PPM sends `fte_centi: 0` with a new event id and the time of the unlink as `sent_at`, one after
   another, after the unlink is stored and without holding it up, under the unlinking caller's own token, each
   recorded in `peer_demand_sends` against the old WPM pool. The `PUT …/peer-link` answer lists them
   (`peer_demand_withdrawals`: `plan_pid`, `month`, `outcome`, `detail`, `fte_centi`, `peer_event_id`). With no
   base URL or no token, each is recorded as not sent (`not_configured`, `not_sent_no_token`) and the figure stays
   in WPM; there is no retry, and a later unlink does not try again (the latest record carries no figure). At most
   2 400 are sent for one unlink. A WPM that has already removed its side of the link refuses, recorded
   `refused`.
7. **Worker skills do not cross.** `src/workers_client.rs` calls `GET {base}/api/workers/{id}/skills`, which WPM
   does not have (WPM has `/api/workers/{pid}/skills-as-of`, catalogue skills with proficiency, a different shape).
   It is left as it is: the skill-gap view reports `unknown`. No mapping from WPM skills to PPM's tags is invented
   (FB-7).
8. **People impact in (FB-4).** `GET /api/plans/{pid}/people-impact[?months=N]` (N 1 to 36, WPM's horizon; `422`
   for 0 or above 36; WPM's default when absent) asks `GET {WPM}/api/capacity/people-impact?programme=thing:<plan pid>`
   under the caller's own token, with the same 2-second timeout and no redirects. WPM holds no programme end, so PPM
   sends `ends_on` from the plan's own `target_date` when it is a full date (`YYYY-MM-DD`, or a timestamp starting
   with one); a month or a year alone is not turned into a day, and then no `ends_on` is sent, `plan_ends_on_reason`
   says why, and WPM's own `unknown` for the engagements part passes through. The answer is WPM's, passed through
   unchanged and labelled `source: "wpm"`: `as_of`, `months`, `floor`, `workers_committed`,
   `contingent_ending_before_end`, `over_committed_pool_months` (each `known` or not with WPM's `reason`; a count
   under WPM's floor `null` with `hidden: true`), `derivation`, and WPM's echoes `programme_ends_on` and
   `programme_ends_on_source`; other fields WPM may add are left out until named here. PPM's own fields beside
   them: `plan_pid`, `programme_ref`, `source`, `plan_ends_on`, `plan_ends_on_source` (`target_date`, or `null`
   when none was sent), `plan_ends_on_reason`, `months_asked`, `status` and `reason`. PPM adds no figure and no total and never turns unknown or hidden into
   zero. `status` is `live`, or `unknown` with a `reason` (every part unknown with that reason, every figure `null`)
   when the URL is not set, the request has no token (nothing is called in either case), WPM cannot be reached, is
   slow, answers 5xx or an unreadable body, or **refuses** the caller (4xx). **Nothing is kept** between calls, so a
   refused caller is never shown an answer fetched under another caller's token. The private-plan guard applies as
   to any `/api/plans/{pid}/…` route (a non-member gets `404` and WPM is not asked). The plan may be of any kind;
   the interface offers it on a plan of kind Program, asked only when the reader presses the button.

### 8.6 Deployment topology (government-portfolio scale)

Target shape *(roadmap, §15)*, consistent with
`agents/share/architecture.md`
and `agents/share/availability.md`:

- N stateless service replicas behind a load balancer; PostgreSQL
  primary + replicas; connection pooling.
- PASETO v4 public token verification at the service edge against the
  central auth-service's published Ed25519 key (offline, no per-request
  auth-service call;
  `agents/share/authentication-sessions.md`,
  supersedes the RS256-JWT model).
- Durable event bus for CRUD + sub-resource + link events; OTLP
  observability pipeline.
- Per-department deployment with cross-registry linkage through
  deterministic identifiers and the `EntityRef` graph rather than
  shared databases.

### 8.7 Positioning vs full PM suites

> **Reversed 2026-08-25**, in step with §1.3. This section read: *"not a
> replacement for Jira / Asana / MS Project / Linear / GitHub Projects.
> Those tools own deep workflow, automation, and sprint mechanics."*
> They no longer do — §1.4 makes custom workflows, automation rules,
> time tracking, and sprint ceremonies capabilities this entity owns.

This entity is a **registry of plan identities (organized into recursive
containment trees) that is also a full project-management suite**. Both
faces are first-class (§1.1), and the second is no longer scoped to
charter level: the operational record covers goals, tasks, issues,
sprints and their ceremonies, recorded effort, configurable workflows,
automation rules, and the sequential project phase a plan is managed
through (§1.5), with derived views over all of it — timeline / Gantt,
burndown, and the Flow Framework metrics (§1.6).

**Architecturally the two faces stay separate**, and that is what makes
one record able to carry both: the thin matchable `Plan` is the API DTO,
the JSONB payload, and the matcher input, while the operational
sub-resources live in their own tables and never enter the matcher
payload (§5.6, §8.2). Adding suite depth therefore adds tables and
endpoints; it does not widen what matching sees.

**Interop is unchanged in mechanism and changed in purpose.** A Jira
project key still maps to a `JiraProjectKey` identifier, an Asana GID to
`AsanaGid`, and so on (the deterministic R-0 schemes), so a plan synced
from a source tool still deduplicates against its registry twin in the
`plans` collection. What those identifiers are *for* is now **migration
and coexistence** — running alongside a tool a department has not left
yet, and identifying the same initiative on both sides — rather than
delegating the parts this entity declined to build. See §17.

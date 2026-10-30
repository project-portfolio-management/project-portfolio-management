## 10. Persistence

PostgreSQL via SeaORM 1.1 + `sea-orm-migration`, owned entirely by the
service crate. Family conventions:
`agents/share/postgresql.md` (shared family guidance, not in this repository).

> **Corrected 2026-08-25.** This paragraph read *"All tables below are
> **planned** — the entity is spec-only (§14)"*, which §14 contradicts:
> the service is implemented and green. The tables in §10.1–§10.5 exist;
> §10.6 is the one section that is genuinely still planned, and says so.

### 10.1 Tables

The thin matchable record lives in **one `plans` table** (a nullable
`kind` label, a nullable `parent_pid` containment link); the operational
sub-resources each get their own table keyed by `parent_pid` (the
partition, §5.6).

**`plans`** — the thin matchable record:

| Column | Type | Notes |
|---|---|---|
| `id` | `PkAuto` | Internal row id |
| `pid` | `UuidUniq` | Public id (route param) |
| `name` | `String` | Denormalized `data.name` for listing |
| `data` | `JsonBinary` (JSONB) | Full thin `Plan` payload (incl. optional `kind`, `goals[]`) |
| `kind` | `StringNull` | Denormalized optional `data.kind` label (`Portfolio`\|`Project`\|`Product`\|`Program`\|`Practice`\|`Process`\|`Purpose`\|`Pathway`\|`Proposal`); nullable — descriptive only, not a discriminator |
| `parent_pid` | `UuidNull` | Denormalized `data.parent_ref` for cheap roll-up of a plan's children (indexed); NULL for a root plan |
| `active` | `BooleanWithDefault(true)` | Registry flag |
| `deleted_at` | `TimestampWithTimeZoneNull` | Soft delete |

The optional `kind` label rides in the payload verbatim; when present
the denormalized `kind` column mirrors it. `kind` does **not** map to a
table or gate matching — every plan lives in the one `plans` table and
matching is **kind-agnostic** (§5.5): any two plan rows may be compared.

**Operational sub-resources** — one table each, every row carrying
`parent_pid UUID` (indexed) + `pid UUID unique` + `deleted_at` for soft
delete. None is serialized into any `data` column; none reaches the
matcher.

| Table | Key columns (beyond `id`, `pid`, `parent_pid`, `deleted_at`) |
|---|---|
| `tasks` | `title`, `description?`, `assignee_ref?`, `status`, `goal_id?`, `parent_task_id?`, `estimate?`, `remaining?`, `due_date?` |
| `issues` | `title`, `kind`, `severity`, `status`, `assignee_ref?` |
| `task_snapshots` | `task_pid`, `estimate`, `remaining`, `observed_at` — feeds the burndown derived view (§6.4) |

Goals are **not** a separate table: they live in `data.goals[]` on the
parent plan's row (§5.3), exposed as a sub-resource via JSONB array
mutation (§10.2).

**Family-baseline tables:**

- **`audit_logs`** — one row per CRUD action on a plan **or any
  sub-resource**: `entity_pid` (the plan), `sub_kind?` (which
  sub-resource, NULL for the plan itself), `action`
  (`created`/`updated`/`deleted`/`merged`), `actor` (token `sub`, NULL
  until auth enforced), `snapshot` (JSONB at the time).
- **`merge_records`** — one row per record-merge: `main_pid`,
  `duplicate_pid`, `reason?`, `actor?`, `transferred` (snapshot of the
  duplicate's payload, plus a tally of re-homed sub-resources).
- **`plan_links`** (built, LT-8) — typed external links on a plan:
  `plan_pid`, `kind` (CHECK in `docs`/`repo`/`ticket`/`design`/`chat`/
  `tracker`/`other`), `url`, `label`, `system?` (CHECK in `jira`/`trello`/`asana`/
  `linear`/`monday`/`clickup`/`azure_devops`/`github`/`other`; a `tracker` link must
  have one), `external_id?` (that system's own key or board id, migration
  `m20261020_000081`), `task_pid?` (the task of the plan the link belongs to; null for a plan's own link,
  migration `m20261021_000084`), `created_by?`, `deleted_at`; partial unique
  index `plan_links_one (plan_pid, coalesce(task_pid, nil), url)` among live rows (created by
  `m20261003_000020`, replaced by `m20261021_000084`).
- **`frameworks`** and **`plan_frameworks`** (built, FR-52) — named, reusable
  ways of working and governance frameworks. `frameworks`: `family` (CHECK
  `ways_of_working`/`governance`), `name` (1 to 120 characters), `description?`, `body`
  (JSONB, the family's own fields, validated by the service), `owner_plan_pid?` (null
  is organisation-wide), `created_by?`, `deleted_at`; partial unique index
  `frameworks_name_once (family, owner or none, lower(name))` among live rows.
  `plan_frameworks`: `plan_pid`, `family`, `framework_pid`, `set_by?`, unique
  `(plan_pid, family)`. A plan with no row for a family inherits the nearest
  ancestor's, resolved on read (migration `m20261020_000080`). **Versions (CF-4,
  migration `m20261024_000088`):** `frameworks.version` is the current version and
  `plan_frameworks.version` the one a plan follows (both default 1);
  `framework_versions` (`framework_pid`, `version`, `name`, `description?`, `body`,
  `created_by?`, unique `(framework_pid, version)`) holds every version and is
  append-only (a trigger refuses UPDATE and DELETE). Existing rows became version 1.
- **`scoring_inputs`** (built, FR-16e2) — one row per plan (`plan_pid` unique) of the estimated inputs of WSJF
  (`business_value`, `time_criticality`, `risk_reduction`, `job_size`, each 1 to 20) and RICE (`reach` >= 0,
  `impact` in 0.25/0.5/1/2/3, `confidence_percent` 0 to 100, `effort` > 0), all nullable by CHECKs;
  `set_by?` (migration `m20261025_000089`). No score is stored.
- **`plan_status_cadences`** and **`status_updates`** (built, FR-16e7) — a plan's expected update cadence (`cadence_days` 1 to 92, `set_on`, `set_by?`, `reminded_for?`) and its append-only updates (`status` on_track / at_risk / off_track by CHECK, `narrative`, `posted_on`, `posted_by?`, and from BR-2 `claims` JSONB, a list of sentences with source ids, FR-16e14, migration `m20261025_000097`); migration `m20261025_000091`.
- **`plan_template_links`** and `plan_templates.version` (built, FR-16e9) — per plan made from a template: `plan_pid`
  (unique), `template_pid`, `node_key`, `template_version` (>= 1), `baseline` (the template's tasks for the plan when it
  was last brought up to date) and `task_map` (template task key to the plan's task); the template's `version` starts at
  1 and rises on each revision; migration `m20261025_000093`.
- **`meetings`** and **`meeting_items`** (built, FR-16e10) — a plan's meeting (`title`, `scheduled_on`, `status`
  planned / held / cancelled by CHECK, `minutes?`, `held_on?`, `created_by?`, `held_by?`, `deleted_at?`) and its agenda
  items and outcomes (`part`, `outcome_kind` decision / action / note by CHECK, `text`, optional `link_kind` and
  `link_pid` together); migration `m20261025_000094`.
- **`meeting_tasks`** (built, FR-16e13) — which task came from which outcome (`meeting_pid`, `outcome_no`, `task_pid`);
  `UNIQUE (meeting_pid, outcome_no)` so an outcome has at most one task; no personal data; migration `m20261025_000096`.
- **`idempotency_keys`** (built, NFR-7) — `caller`, `key` (unique together), `method`, `path`, `request_hash`,
  `state` (`in_progress`, `done` by CHECK), and for a kept answer `status`, `content_type`, `body`; purged after one day
  (class `GEN`); migration `m20261025_000095`. The stored body is the answer as sent, so it may hold personal data
  for as long as the key lives.
- **`scheduled_job_runs`** (built, FR-16e8) — `pid`, `job`, `started_at`, `finished_at?`, `outcome` (`running`, `ok`, `failed` by CHECK), `detail?`; one row per run of a job the service schedules for itself; migration `m20261025_000092`.
- **`lessons`** (built, FR-16e4) — `pid`, `plan_pid`, `occasion` (a gate or `closure`), `category` (`went_well`,
  `went_badly`, `recommendation`, by CHECK), `summary`, `detail?`, `recorded_by?`, `deleted_at?` (a withdrawal);
  append-only (migration `m20261025_000090`); `link_kind?` and `link_pid?` (BR-4, migration 98) name one record of the
  plan, indexed with `meeting_items.link_pid`.
- **`plan_tolerances`** (built, FR-53) — a plan's limit per dimension: `plan_pid`, `dimension` (CHECK
  `time`/`schedule`/`cost`/`risk`/`resources`, widened by `m20261021_000085`), `limit_value` (CHECK 0 to 100000, and 1 to 25 for risk),
  `amber_from_percent` (CHECK 1 to 99, default 75), `note?`, `set_by?`; unique `(plan_pid, dimension)`. A plan
  with no row for a dimension inherits the nearest ancestor's, resolved on read (migration `m20261020_000082`).
- **`escalations`** and **`escalation_events`** (built, FR-54) — `escalations`: `plan_pid`, `title`, `description?`,
  `reason_kind` (CHECK), `tolerance_dimension?` (CHECK: only with a tolerance reason), `level` (CHECK 1 to 9),
  `escalated_to?`, `owner_ref?`, `status` (CHECK open/acknowledged/resolved/withdrawn), `raised_by?`, `raised_on`,
  `respond_by?`, `outcome?`, `response?`, `resolved_on?` (CHECK: resolved needs all three), `deleted_at`.
  `escalation_events`: `escalation_pid`, `event`, `level`, `actor?`, `note?`, **append-only** by trigger
  (migration `m20261020_000083`).
- **`entity_links`** — **not built (T-7).** No migration, entity or
  route exists. The cross-service write-side table in
  `agents/share/cross-service-linking.md` §4.1
  (`from_pid`, `kind`, `to_ref`, `role?`, `confidence`, `provenance`,
  `valid_from?`, `valid_to?`, `UNIQUE (from_pid, kind, to_ref,
  valid_from)`) remains a target.
- **`bulk_jobs`** — the bulk import/export job table per
  [`agents/share/bulk-import-export.md` §3](bulk-import-export.md),
  with `UNIQUE (entity, kind, idempotency_key)`. Canonical column list
  lives there; not restated (§10.4).

**Reality check (verified against `migration/src`).** There is **no
`issues` table** (and no `issues` entity or controller); the only
migration mentioning issues is the `tasks.flow_type` one. The
`task_snapshots` table above is likewise not created by any migration
(unverified whether burndown derives from `task_transitions` instead).
The migrations are the authoritative table list; §10.8 indexes all 133 migrations.

### 10.2 JSONB rationale and the goals bridge

The thin payload is stored verbatim so that the matcher type remains
the single schema for the matchable record on the one `plans` table.
Adding a field to `Plan` is a matcher-crate change plus a CHANGELOG
entry — **no service migration** — because serde fills missing fields
with defaults on read. The trade-off: querying inside the payload needs
JSONB operators (or GIN indexes, roadmap), and the only relational
projections of the thin record are `pid`, `name`, the optional `kind`
label, and `parent_pid`.

`goals[]` is the **only** sub-resource inside the payload. Goal CRUD
endpoints (FR-12) read / mutate the parent row's `data.goals[]` array
transactionally and rewrite `data`, so the matchable payload and the
goal sub-resource view never diverge. The other sub-resources are
ordinary relational tables (querying tasks by status, listing issues by
severity, etc. is relational), which is why they are *not* in the JSONB
— high-volume, frequently-queried, non-identity-bearing.

### 10.3 Operations

- `auto_migrate` is on in development (`cargo run -- start` migrates).
- Soft delete is application-level (`deleted_at`); all queries filter
  `deleted_at IS NULL`. Soft-deleting a plan hides its
  sub-resources from read paths (§5.8); merge re-homes them to the
  survivor (§6.3).
- The `parent_pid` column on `plans` is indexed for the child roll-up
  (FR-2 `?parent=` filter); it is kept consistent with
  `data.parent_ref` by the model layer (§5.6 / §5.8).
- Open question OQ-3: index the high-traffic sub-resource columns
  (`tasks.status`, `tasks.assignee_ref`, `issues.severity`,
  `parent_pid`) and add JSONB GIN on the `plans` `data` column
  once search / candidate blocking lands.

### 10.4 Bulk import / export — `bulk_jobs`

Async bulk operations (§9.6) add one table, `bulk_jobs`, per the
family-wide schema in
[bulk import/export §3](bulk-import-export.md) — the
canonical column list lives there and is **not** restated. It tracks
each import/export job (`kind`, `format`, `status`, `params`, the
`rows_total`/`rows_created`/`rows_upserted`/`rows_to_review`/`rows_errored`
counts, `actor`, artifact URLs, and `expires_at` TTL), with
`UNIQUE (entity, kind, idempotency_key)` so a retried submit maps to
the same job. The `entity` discriminator is the one `plans` collection.
Jobs run on the loco `bg_pg` worker; artifacts (uploaded source, export
output, error report) live in the config-driven artifact store
(S3-compatible in deployment, local fs in dev), referenced by
short-lived access-controlled URLs.

Portfolio-specific notes:

- **Idempotency** is anchored on the §9.6 stable keys (a deterministic
  scheme-scoped external-PM identifier, the owner-scoped
  `(owner_org_id, code)`, or `pid`): re-submitting a file re-upserts
  the same rows to the same state.
- The matcher's **single schema** holds for the thin record: the JSONB
  `data` column is the full thin `Plan` payload, so JSONL round-trips
  losslessly and CSV flattens per §9.6 (every repeated / nested field a
  JSON-encoded cell); no bulk path bypasses the JSONB round-trip
  invariant (§5.8). Bulk import targets the thin plan record;
  sub-resource bulk is a roadmap extension.

### 10.5 Collaboration / automation tables (2026-07-22)

Five tables behind §6.4a, none of them part of the matcher payload:

| Table | Holds |
|---|---|
| `reviews` | One subject (idea / proposal / plan) delegated to one expert: `reviewer_ref` (EntityRef), `reviewer_scope`, `expertise`, status, due date, verdict (`score`, `recommendation`, `comment`). A partial unique index keeps one **live** invitation per subject + reviewer, so a fresh round is possible after a verdict |
| `automations` | One rule: optional `plan_pid` scope, trigger (`trigger_kind` + optional `from_status` / `to_status`), action (`action_kind` + JSONB `action_value`), `enabled` |
| `automation_runs` | Append-only log of every firing: outcome (`applied` / `skipped` / `failed`) + detail |
| `scheduled_actions` | The set-and-forget queue: `due_at`, `status`, payload, the automation that scheduled it, and the fired stamp + outcome |
| `notifications` | The in-app inbox written by automations and by the sweep; `read_at` for the read stamp |

**No Smart Score table.** The score (§6.4e) is derived on read from rows
the service already stores, so it can never drift from the evidence it
claims to summarize.


### 10.6 Full-suite tables (§1.4–§1.6, §6.4c)

**Built** (migrations `m20260825_000001` … `m20260826_000003`; §14.1).
The column lists below are the original design; the migrations are
authoritative and the column-level match was **unverified** in this
pass (table names were verified). Every table carries `pid UUID unique` and
`deleted_at` for soft delete; every plan-scoped table carries an indexed
`plan_pid UUID`. **None is serialized into any `data` column and none
reaches the matcher** (§5.6).

| Table | Holds | FR |
|---|---|---|
| `workflows` | One configuration: optional `plan_pid` scope, `applies_to` (task \| issue), `is_default` | FR-26 |
| `workflow_states` | `workflow_pid`, `key`, `label`, **`category`** (`todo`\|`active`\|`waiting`\|`done`, NOT NULL), `wip_limit?`, `is_initial`, `is_terminal` | FR-26 |
| `workflow_transitions` | `workflow_pid`, `from_key`, `to_key` | FR-26 |
| `key_results` | `goal_id`, `title`, `metric`, `start_value`, `target_value`, `current_value`, `direction`, `unit?`, `currency?`, `owner_ref?`, `due_date?`, `status` | FR-27 |
| `key_result_check_ins` | `key_result_pid`, `observed_at`, `value`, `confidence?`, `note?`, `actor` | FR-27 |
| `time_entries` | `plan_pid`, `task_pid?`, `actor_ref`, `spent_on`, `minutes`, `category`, `billable`, `note?` | FR-28 |
| `sprints` | `plan_pid`, `name`, `starts_on`, `ends_on`, `goal?`, `status` | FR-29 |
| `sprint_commitments` | `sprint_pid`, `task_pid`, `committed_at` — the planning snapshot | FR-29 |
| `ceremonies` | `sprint_pid`, `kind`, `held_at`, `facilitator_ref?` | FR-29 |
| `ceremony_notes` | `ceremony_pid`, `category`, `text`, `converted_task_pid?` | FR-29 |
| `phase_transitions` | `plan_pid`, `from?`, `to`, `occurred_at`, `actor?`, `reason?` — append-only | FR-30 |
| `business_case_targets` | `plan_pid`, `metric`, `baseline_value`, `target_value`, `unit?`, `currency?`, `promised_by`, `source`, `approved_at`, `approved_by_ref?` | FR-33 |
| `value_points` | `plan_pid`, `benefit_pid?`, `observed_at`, `value`, `is_first_measurable`, `method`, `evidence_ref?`, `actor` | FR-33 |
| `adoption_snapshots` | `plan_pid`, `observed_at`, `active_users`, `target_users`, `window_days`, `definition` | FR-33 |
| `satisfaction_responses` | `plan_pid`, `surveyed_at`, `instrument`, `score`, `respondent_role`, `comment?` — **no respondent identity** | FR-36 |
| `working_time_configs` | `scope_ref?`, `hours_per_day`, `working_days`, `holidays` — the declared capacity basis | FR-35 |
| `non_working_periods` | `person_ref`, `starts_on`, `ends_on`, `kind`, `note?` — **subtracts from the denominator**, never counted as idle | FR-35 |
| `total_project_control` | `plan_pid`, `currency`, `observed_at`, `dipp`, `dipp_progress_index_numerator`, `dipp_progress_index_denominator`, `dipp_progress_index_ratio` (**`GENERATED ALWAYS`**), `expected_monetary_value`, `cost_estimate_to_complete` | FR-37 |
| `controls` | `plan_pid`, `name`, `timing`, `metric`, `target_value`, `unit?`, `currency?`, `comparator`, `tolerance?`, `source_kind`, `source_ref?`, `cadence?`, `owner_ref?`, `enabled` | FR-38 |
| `control_readings` | `control_pid`, `observed_at`, `value?`, `verdict`, `gap?`, `method` — **append-only** | FR-38 |
| `control_actions` | `reading_pid`, `kind`, `description`, `owner_ref?`, `due_date?`, `converted_task_pid?`, `converted_issue_pid?`, `closed_at?`, `outcome?` | FR-38 |

Schema-level constraints that enforce spec rules rather than trusting
the handlers:

- `workflow_states.category` is `NOT NULL` with a CHECK on the four
  values, so an uncategorized state cannot exist even by direct SQL
  (§5.9.1). A partial unique index enforces one `is_initial` state per
  workflow.
- `phase_transitions` is **append-only** — no update or delete path,
  matching `task_transitions`
  ([time-based-analysis.md](time-based-analysis.md) §5.1) — because a
  phase history that can
  be rewritten cannot support a duration claim.
- `business_case_targets.approved_at` has no update path once set: the
  Time-to-Value clock start must not move (§5.9.6).
- `adoption_snapshots` requires `target_users > 0`; a rate with a zero
  denominator is refused at write, not divided at read.

**No table for any derived figure.** Transformation ROI, Value
Realization Rate, Time to Value, Adoption Rate, SPI, CPI, NPV, Flow
Distribution and the OKR scores are all computed on read from the rows
above, so none can drift from its evidence — the rule §10.5 already
applies to Smart Score.

### 10.7 Migrations 40-48 (2026-10-06)

Migration files are `migration/src/m20261006_0000NN_*.rs`. Tables carry
`id`, `pid` (unique uuid) and, where noted, `deleted_at`, as elsewhere.

| # | Migration | Adds |
|---|---|---|
| 40 | `assignment_fields` | `tasks.work_minutes` (bigint), `units_percent` (int), `scheduling_rule`, `contour` (NOT NULL, default `flat`), `contour_custom` (jsonb). CHECKs: `tasks_work_range` (0..17 520 000), `tasks_units_range` (1..1000), `tasks_scheduling_rule` (`fixed_units`/`fixed_duration`/`fixed_work` or NULL), `tasks_contour` (`flat`, `front_loaded`, `back_loaded`, `bell`, `turtle`, `custom`) |
| 41 | `cost_rates` | `cost_rate_tables` (`name`, `person_ref?`, `role?`, `currency`, `deleted_at`), `cost_rates` (`table_pid`, `effective_from`, `per_hour_minor`, `created_by?`), `task_costs` (`task_pid`, `kind`, `description`, `amount_minor`, `currency`, `accrual`, `deleted_at`), `time_entry_costs` (`time_entry_pid`, `plan_pid`, `person_ref`, `table_pid`, `spent_on`, `minutes`, plus the snapshotted rate; remaining columns **unverified**). CHECKs: table scope is person XOR role; currency `^[A-Z]{3}$`; `per_hour_minor >= 0`; cost `kind` in (`material`,`fixed`); `accrual` in (`start`,`prorated`,`end`); `amount_minor > 0`. Indexes: partial unique `(name, person_ref)` and `(name, role)` among live tables; unique `(table_pid, effective_from)` (a rate per day); `task_costs_task`; unique `time_entry_costs_entry` (one snapshot per entry); `time_entry_costs_plan` |
| 42 | `announcements` | `announcements` (`title`, `body`, `severity`, `pinned`, `publish_at`, `expires_at?`, `author_ref?`, `deleted_at`). CHECKs: severity in (`info`,`important`,`urgent`); `expires_at IS NULL OR expires_at > publish_at`. Index `announcements_live (publish_at DESC)` where not deleted |
| 43 | `onboarding` | `onboarding_templates` (`kind`, `items` json; unique on `kind`), `onboarding_cases` (`kind`, `plan_pid`, `subject_ref?`, `title`, `effective_date`, `status`, `created_by?`, `closed_at?`, `closed_reason?`), `onboarding_items` (`case_pid`, `position`, `title`, `owner_ref?`, `due_date?`, `resolution?`, `note?`, `resolved_at?`, `resolved_by?`), `access_records` (`plan_pid`, `case_pid?`, `person_ref`, `access`, `action`, `recorded_on`, `recorded_by?`, `note?`). CHECKs: kind in (`person_join`,`person_leave`,`plan_intake`,`plan_closeout`); case status in (`open`,`completed`,`cancelled`); `(kind LIKE 'person_%') = (subject_ref IS NOT NULL)`; item resolution in (`done`,`skipped`); access action in (`granted`,`revoked`). Indexes `onboarding_cases_plan (plan_pid, status)`, `onboarding_items_case (case_pid, position)`, `access_records_plan (plan_pid, person_ref)` |
| 44 | `hammocks` | `tasks.is_hammock` (bool NOT NULL, default false); `hammock_links` (`hammock_pid`, `member_pid`). CHECKs: `hammock_links_distinct` (hammock <> member); `tasks_hammock_is_derived` (a hammock has NULL `duration_days`, `work_minutes`, `constraint_kind`). Unique `hammock_links_pair`; index `hammock_links_member` |
| 45 | `job_progress_cancel` | `bulk_jobs.progress` (int 0..100, CHECK `bulk_jobs_progress`), `cancel_requested_at?`, `cancelled_by?`; `bulk_jobs_status` widened to add `cancelled` (`queued`, `running`, `completed`, `completed_with_errors`, `failed`, `cancelled`). Trigger `bulk_jobs_progress_monotone` (BEFORE UPDATE) keeps progress from decreasing |
| 46 | `processes` | `processes` (`name`, `current_version`, `created_by?`, `deleted_at`; CHECK `current_version >= 1`), `process_versions` (`process_pid`, `version`, `steps` json, `created_by?`; unique `(process_pid, version)`), `process_executions` (`process_pid`, `version`, `subject_kind`, `subject_pid`, `status`, `current_step`, `started_by`, `finished_at?`, `note?`; status in `running`/`completed`/`rejected`/`cancelled`/`failed`; index on `(subject_kind, subject_pid)`), `process_execution_logs` (`execution_pid`, `step_index?`, `event`, `actor?`, `note?`; event CHECK over ten event names; index `(execution_pid, id)`). Triggers `process_versions_immutable` and `process_execution_log_append_only` (BEFORE UPDATE OR DELETE) raise via `processes_refuse_change()`. Adds `approval_requests.process_execution_pid` and `process_step` |
| 47 | `controlled_documents` | `controlled_documents` (`title`, `plan_pid?`, `created_by?`, `deleted_at`), `document_versions` (`document_pid`, `version`, `body`, `change_summary?`, `status`, `approval_pid?`, `effective_from?`, `superseded_on?`, `superseded_by?`, `created_by?`). Status in (`draft`,`in_review`,`approved`,`effective`,`superseded`). Unique `(document_pid, version)`; partial unique **one `effective`** per document and **one in-flight** (`draft`/`in_review`/`approved`) per document. CHECKs: past review needs `approval_pid`; effective/superseded needs `effective_from`; `(status = 'superseded') = (superseded_by AND superseded_on set)`. Trigger `document_versions_guard`: a version past `draft` is never deleted and its `body`, `change_summary`, `version`, `document_pid` never change (the database refuses, not only the handler) |
| 48 | `onboarding_one_open` | Unique index `onboarding_cases_one_open` on `(plan_pid, kind, COALESCE(subject_ref, ''))` where `status = 'open'`; trigger `plan_dependencies_not_hammock` (BEFORE INSERT on `plan_dependencies`) refuses a dependency naming a hammock |

### 10.8 Migration index (all 133 migrations, by created tables)

Verified by listing `migration/src`; `+table` marks columns added to an
existing table. Migrations 1-7 are dated `20220101`.

```
20220101 01 plans; 02 audit_logs; 03 merge_records; 04 event_outbox; 05 proposals gate_reviews risks budget_lines (+plans); 06 plan_dependencies milestones allocations report_definitions; 07 ideas scenarios objectives objective_links benefits
20260719 02 (+budget_lines milestones risks); 03 insight_snapshots
20260720 01 sprints tasks (+milestones); 02 sprint_notes devops_events (+tasks)
20260722 01 reviews automations automation_runs scheduled_actions notifications
20260728 01 integrity_digests (no create_table seen: unverified)
20260823 01 task_transitions
20260825 01 total_project_control; 02 controls control_readings control_actions; 03 phase_transitions (+plans); 04 flow_type (+tasks); 05 workflows workflow_states workflow_transitions; 06 key_results key_result_check_ins
20260826 01 time_entries working_time_configs non_working_periods; 02 ceremonies sprint_commitments; 03 business_case_targets value_points adoption_snapshots satisfaction_responses
20260902 01 automation_multi_action; 02 automation_milestone_fires
20260908 01 webhook_deliveries
20260918 01 budget_baselines budget_baseline_periods; 02 allocation_skills; 03 saved_views
20260920 01 report_group_by (+report_definitions)
20261002 01 canvases canvas_items; 02 comments comment_edits reactions; 03 task_due_date (+tasks); 04 notification_prefs
20261003 01 plan_modules; 02 components (+tasks); 03 custom_field_defs; 04 custom_field_values; 05 workflow_transition_roles; 06 watchers; 07 approval_requests; 08 task_relations; 09 task_parent (+tasks); 10 feed_tokens; 11 plan_access_settings; 12 timesheet_weeks; 13 plan_templates; 14 planned_slots; 15 artifacts; 16 attachments; 17 bulk_jobs; 18 review_queue_items; 19 guest_grants; 20 plan_links; 21 notification_sinks; 22 sink_deliveries; 23 attachment_texts; 24 report_schedules; 25 schedule_fields (+plan_dependencies tasks); 26 work_calendars (+tasks); 27 task_external_ref (+tasks); 28 person_availabilities; 29 allocation_booking (+allocations); 30 schedule_baselines; 31 schedule_baseline_tasks; 32 raci_assignments; 33 earned_value_settings (+tasks); 34 earned_value_snapshots
20261004 35 resource_roles; 36 schedule_risk_inputs (+risks tasks)
20261005 37 kpis kpi_readings; 38 milestone_links (milestone_tasks milestone_slips, +milestones); 39 cost_risk_inputs (+budget_lines risks)
20261006 40-48 see §10.7
20261009 49 oxford_spelling
20261010 50 issues; 51 audit_logs_protected; 52 break_glass; 53 handling_marks; 54 change_requests; 55 director_packs; 56 delivery_confidence; 57 programme_gates; 58 funding_sources; 59 programme_buffers; 60 benefit_validation; 61 capacity_heatmap; 62 pack_rhythm; 63 ccpm_estimates; 64 buffer_management; 65 portfolio_drum; 66 organizations; 67 milestone_signoffs; 68 programme_profiles; 69 external_asks; 70 key_persons; 71 triage_thresholds; 72 email_deliveries; 73 start_control; 74 deployment_moves; 75 cutover_decisions; 76 scorecard_deescalation; 77 portfolio_ledger
20261019 78 blocked_reason
20261020 79 task_sibling_position; 80 frameworks; 81 link_systems; 82 plan_tolerances; 83 escalations
20261021 84 task_links; 85 schedule_tolerance
20261024 86 deployment_settings; 87 availability_no_overlap; 88 framework_versions
20261025 89 scoring_inputs; 90 lessons; 91 status_updates; 92 scheduled_job_runs; 93 blueprint_sync; 94 meetings; 95 idempotency_keys; 96 meeting_tasks; 97 status_claims; 98 lesson_links
20261030 99 peer_pool_link
```

From migration 49 on the index gives each migration's name, not its tables (read the file for those). Migration
99 (`m20261030_000099_peer_pool_link`, FB-1, FB-3) adds `skill_pools.peer_pool_ref` (nullable text, WPM's pool pid,
CHECKed as a lower-case hyphenated UUID) with the partial unique index `skill_pools_peer_pool_ref` (one PPM pool per
WPM pool), and `peer_demand_sends` (the latest send per pool, programme and month: unique
`peer_demand_sends_once (pool_pid, plan_pid, month)`, an outcome CHECK that still admits `not_sent_zero` for rows
written before FB-5).

Row-level soft delete, append-only logs and immutability are enforced
in the database (CHECKs and triggers) for the 2026-10 groups above;
for the earlier groups see their own migration files (not re-audited
here: **unverified**).

**Availability ranges (CF-3, migration `20261024 87`).** One person's non-removed `person_availabilities`
ranges cannot overlap, inclusive of both end dates: an exclusion constraint
`person_availabilities_no_overlap` (`EXCLUDE USING gist (person_ref WITH =, daterange(from_date, to_date, '[]') WITH &&) WHERE deleted_at IS NULL`).
It needs the `btree_gist` extension, which the migration creates (trusted from PostgreSQL 13). The service
still checks first, under a per-person advisory lock, and answers `409`; if a writer reaches the database
past that check, the constraint refuses the second range and the service also answers `409`. The migration
fails, naming the constraint, if overlapping ranges already exist.

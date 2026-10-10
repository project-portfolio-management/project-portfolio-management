# Bulk import / export — design (T-8)

**Status: implemented 2026-10-03** (migrations `…_000017_bulk_jobs`, `…_000018_review_queue_items`; `src/bulk.rs`, `src/controllers/bulk.rs`, `src/workers/bulk.rs`; tests in `tests/requests/bulk.rs`). The owner's open decisions in §12 were taken at their stated defaults — local-directory artifacts, 50 MiB / 100 000 rows, review queue used by import only, `full` export gated by the destructive right. Departures from the design as written are listed in *Implementation notes* at the end. It stands
in for the family document `agents/share/bulk-import-export.md`, which
`spec/13-tasks.md` T-8 and the API spec §9.8 / §10 cite but which is not
in this repository. If that document is later supplied, it wins on
anything it covers and this file is reconciled to it.

Scope: the one `plans` collection of
`project-portfolio-management-api-with-rust`. It unblocks T-28e's
scheduled report runs and T-28n's Jira / Asana import codec.

## 1. Goals and non-goals

Goals:

- Load and unload plans in bulk, asynchronously, without one bad row
  aborting the load, with counts that reconcile.
- Re-importing the same file is **idempotent**.
- Imports go through the same validators, matcher, events and audit as a
  single `POST /api/plans`. Bulk is never a back door.
- Every export is audited, including a zero-row one, and carries people
  references only under a gated profile.

Non-goals (this task): streaming ingestion, two-way sync, a connector
catalogue (refused in T-28), Parquet export (feature-gated, later),
importing sub-resources (tasks, budget lines — that is T-28n's codec).

## 2. The `bulk_jobs` table

One migration. Append-mostly; a job row is never deleted, only expired
artifacts.

| Column | Type | Notes |
|---|---|---|
| `id`, `pid`, `created_at`, `updated_at` | family standard | `pid` is the public `job_id` |
| `entity` | text | always `plans` here; kept so the table can be shared |
| `kind` | text | `import` \| `export` |
| `status` | text | `queued` → `running` → `completed` \| `completed_with_errors` \| `failed` |
| `format` | text | `jsonl` \| `csv` (`parquet` export-only, feature-gated) |
| `idempotency_key` | text, null | caller-supplied; **`UNIQUE (entity, kind, idempotency_key)`** so a retried `POST` returns the existing job |
| `requested_by` | text, null | token `sub` (a pid, never an email) |
| `params` | jsonb | import: `{on_keyless, owner_org_id?}`; export: `{masking_profile, include_soft_deleted, filter}` |
| `rows_total`, `rows_ok`, `rows_created`, `rows_updated`, `rows_unchanged`, `rows_queued_for_review`, `rows_failed` | bigint | **invariant:** `rows_total = rows_created + rows_updated + rows_unchanged + rows_queued_for_review + rows_failed` |
| `source_artifact`, `result_artifact`, `error_artifact` | text, null | artifact ids (§7) |
| `expires_at` | timestamptz, null | artifact TTL |
| `started_at`, `finished_at` | timestamptz, null | |
| `failure_reason` | text, null | set only for `failed` |

`completed_with_errors` means at least one row failed but the job ran to
the end; `failed` means the job itself could not run (unreadable file,
format mismatch, database unavailable) and `rows_*` may be partial.

## 3. Endpoints

All on the plans collection, under the blanket guard. Import is an
`Action::Destructive` (the verifier already classes "import" as a
destructive named POST); export is `Action::Read` plus the gates in §8.

```
POST /api/plans/import         202 {job_id}    body: file + params; header Idempotency-Key
GET  /api/plans/import/{id}    status, counts, errors_url, review_url
POST /api/plans/export         202 {job_id}    body: params
GET  /api/plans/export/{id}    status, download_url, expires_at
GET  /api/plans/bulk-jobs      list; filter by kind, status; paged
GET  /api/plans/bulk-jobs/{id}/errors   the per-row error report (§6)
```

`POST` returns `202` after inserting the job row (and, for import,
storing the uploaded source), never after the work. A repeated
`Idempotency-Key` returns the original `job_id` with `200`.

## 4. Execution

Work runs in a loco `BackgroundWorker` (Postgres-backed queue, `bg_pg`),
registered in `connect_workers` beside the placeholder `DownloadWorker`.
The worker claims a job by `UPDATE ... SET status='running' WHERE
status='queued'` (one winner), processes rows, writes counters in
batches, then sets the terminal status. A worker that dies leaves the job
`running`; a sweep re-queues a `running` job whose `updated_at` is older
than a lease (default 10 min). Re-running is safe because the row
pipeline is idempotent (§5).

## 5. The per-row import pipeline

For each row, in file order, independently (one failure never aborts):

1. **Decode** to a `Plan` (§9). A decode failure → error row (§6).
2. **Validate** with the same `validate(&plan)` the single create uses,
   plus the containment-cycle check. Failure → error row.
3. **Resolve the stable key**, in this order:
   1. `pid`, if supplied;
   2. a **deterministic identifier** in `identifiers[]` (`JiraProjectKey`,
      `AsanaGid`, `TrelloBoardId`, `MsProjectId`, `GitHubProjectId`,
      `LinearId`, `Uri`, `Uuid`) — globally unique;
   3. `(owner_org_id, code)` — only when `owner_org_id` is present, and
      same-owner only. A `code` without an owner is **not** a key.
4. **Keyed row:** look up the existing plan.
   - found → compare; identical → `unchanged`; else `update_and_emit`
     (`updated`, honouring record-level ABAC on the stored `stage`, so a
     gate-locked plan is refused as an error row, not silently skipped);
   - not found → `create_and_emit` (`created`).
5. **Keyless row:** run the normal duplicate detection
   (`SearchIndex::candidates` → matcher). No match → `create_and_emit`.
   A match above threshold → **`review_queue`** (§5.1), counted
   `queued_for_review`, never auto-merged. If the search index is
   unavailable the row is an error row (`search_unavailable`) — the same
   "do not guess" stance `check-duplicates` takes.
6. **Events and audit are not bypassed:** create/update go through
   `streaming::create_and_emit` / `update_and_emit`, so the outbox event
   and the audit row are written exactly as for a single request, with
   the job's `requested_by` as actor and the job pid in the audit note.

### 5.1 `review_queue`

Does not exist yet; introduced by T-8 (second migration).

| Column | Notes |
|---|---|
| `pid`, timestamps | |
| `status` | `pending` \| `confirmed` \| `rejected` \| `auto_merged` (reserved) |
| `provenance` | `import` here; the existing intake/idea flows keep their own `provenance` strings and may adopt this table later |
| `candidate` | jsonb — the incoming `Plan`, as decoded |
| `matches` | jsonb — `[{pid, score, confidence}]` from the matcher |
| `job_pid`, `source_line` | where it came from |
| `resolved_by`, `resolved_at`, `resolution_pid` | set on confirm/reject |

Resolution endpoints (small, follow-on): `GET /api/plans/review-queue`,
`POST .../{pid}/confirm` (create as new, or merge into a named match via
the existing `merge_plans`), `POST .../{pid}/reject`. Reviewing is
`Action::Write`; confirming-as-merge is `Destructive`.

## 6. The per-row error report

Downloadable as CSV or JSONL, one record per failed row:

`row_number, source_line, field, code, message`

- `row_number` is the 1-based ordinal among data rows; `source_line` is
  the physical line (they differ for multi-line CSV cells and for the
  CSV header).
- `code` is a stable machine string (`decode`, `validation`, `cycle`,
  `forbidden`, `search_unavailable`, `unmapped_field`, `conflict`);
  `message` is for people. `field` may be empty for a whole-row failure.
- A row can produce several records (one per bad field).
- Counts reconcile (§2 invariant); a test asserts it.
- The report contains **no** source values beyond the offending field
  name, so it can be shared without leaking people references.

## 7. Artifacts and retention

Uploaded sources, exports and error reports are stored as **artifacts**
under a configurable directory (`PROJECT_PORTFOLIO_MANAGEMENT_BULK_DIR`),
addressed by id, with a TTL (`..._BULK_TTL_HOURS`, default 24) after which
a sweep deletes the file and nulls the column; the job row remains.
Downloads are authenticated and re-checked against the caller's right to
read the job. This is the "artifact store" T-28e's scheduled runs reuse.

## 8. Export

Params: `masking_profile` (`masked` default | `full`), `include_soft_deleted`
(default `false`), and an optional filter (the same fields the saved
reports filter on: `kind`, `stage`, `status`, `name_like`).

- **Masked** (default) applies `privacy::mask_plan`: `lead_ref` dropped,
  owner organization fields partially hidden. Operational content is never
  masked.
- **`full`** requires `Action::Destructive` on the caller, because it
  carries people references (personal data).
- **`include_soft_deleted=true`** requires the same gate.
- Record-level ABAC applies per row: a plan the caller may not read is
  **omitted**, as `list`/`search` do (SEC-PPM-1), and counted
  `rows_omitted` in the job so a shortfall is explained.
- **Every export is audited** — a row in `audit_logs` per job with the
  profile, flags, filter and row count, **even when zero rows match**.
  This is the acceptance case the task names.
- Row order is `id` ascending, so two exports of unchanged data are
  byte-identical.

## 9. Codecs

- **JSONL** — one `Plan` JSON object per line; the lossless reference
  format. Round-trips exactly (a property test pins it).
- **CSV** — RFC-4180 (reusing `rules::csv_field`); header row required.
  Scalar fields are columns; **every repeated or nested field**
  (`alternate_names`, `goals`, `keywords`, `tags`, `identifiers`,
  `same_as`, `relationships`) is one cell of **compact JSON**, so CSV is
  also lossless. Unknown columns are an `unmapped_field` error record per
  row (reported, never dropped silently), not a job failure.
- **Parquet** — export only, behind a cargo feature, later.

Format is chosen by the `format` param, falling back to the upload's
extension; a mismatch between declared and detected format fails the job.

## 10. Verifiable acceptance

Integration tests (Postgres, in the existing DB-gated suite):

1. **Idempotent re-import** — import a keyed file twice: the second job
   is all `unchanged`; plan count and audit-row count do not grow.
2. **Per-row error report** — a file with a bad row among good ones
   completes `completed_with_errors`; good rows land; the report names
   `row_number`, `source_line`, `field`, `code`; counts reconcile.
3. **Keyless dedupe → review** — a keyless row matching an existing plan
   lands in `review_queue` with `provenance = import`, no plan created.
4. **Masked vs full export** of a plan with a lead and owner org: masked
   hides them; `full` shows them and is refused without the gate.
5. **Zero-row export audited** — an export matching nothing still writes
   its audit record.
6. Plus: `Idempotency-Key` replay returns the same job; an unreadable
   plan is omitted and counted; a gate-locked plan update is an error
   row; the lease sweep re-queues a stale `running` job.

## 11. Delivery order

1. `bulk_jobs` migration, entity, pure `src/bulk.rs` (codecs, row state
   machine, counters) with unit tests — no I/O.
2. Export (simplest end-to-end: job → worker → artifact → download).
3. Import without review (keyed upsert + error report).
4. `review_queue` migration + keyless path + resolution endpoints.
5. Artifact TTL sweep; OpenAPI entries; spec §9.8 / §10 updated from
   "deferred" to landed.

## 12. Open decisions for the owner

- **Artifact storage:** local directory (assumed above) vs object store.
  A directory is enough for a single-node deployment; a multi-node one
  needs shared storage or an object store.
- **Upload size cap** and row cap per job (suggest 50 MB / 100 000 rows).
- **Review-queue adoption** by the existing intake/idea `provenance`
  flows, or keep them separate.
- **Export `full` gate:** `Destructive` (assumed) or a dedicated
  attribute.

## Implementation notes (what differs from the design above)

- **`review_queue` is `review_queue_items`.** loco's `create_table` pluralizes
  table names; a singular name creates a different table than the SQL refers to.
- **Authorization is evaluated when the job is requested**, not in the worker
  (a background worker holds no claims). An export stores the plans the caller
  may read; `full` masking and soft-deleted rows need the destructive right.
- **Record-level stage policy is not applied per imported row** — an import is a
  destructive action at the guard instead. A gate-locked plan updated by an
  import is therefore *not* an error row, unlike the design's test 6.
- **Review resolution** offers `confirm` (create it) and `reject` (discard it);
  *merge into a match* is not implemented — reject, then use the merge endpoint.
- **Export filters** are `kind` and `name_like` (the design said the report
  filters; those are plans-collection reports).
- **The sweep** (`POST /api/plans/bulk-jobs/sweep`, admin) also re-queues
  `running` jobs older than a ten-minute lease; there is no ticker yet.
- **Not built:** Parquet export; multipart upload (the file is the raw body);
  a front-end.

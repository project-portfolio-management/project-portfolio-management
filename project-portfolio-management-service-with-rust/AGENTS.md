# AGENTS.md — Portfolio Service

Entry point for AI coding agents working in the
`project-portfolio-management` crate (v0.2.0, edition 2024, Rust 1.99.1 with a
minimum of 1.96, loco-rs 1.2, sea-orm 2.0, axum 0.8): a registry of **plan** records
— one recursive collection with an optional Portfolio / Project / Product
/ Program / Practice / Process / Purpose / Pathway / Proposal `kind`
label — **and** a project-management tool.

> Read [`spec/index.md`](./spec/index.md) first — the living spec for this
> crate. The entity-wide contract and canonical `Plan` model live in the
> [portfolio entity spec](../spec/index.md). Work queues: the numbered
> build-out is [`spec/index.md`](./spec/index.md) §13; root
> [`../tasks.md`](../tasks.md) (`LT-n`, derived from
> [`../plan.md`](../plan.md)) tracks the later PJ-/PL-/RM-/SC- items.
> The shared `agents/share/*` docs older notes cite are not in this
> repository; the in-repo guides are [`../agents/`](../agents/index.md).

## What this is

A **loco.rs** service. The API DTO **is**
`project_portfolio_management_matcher::Plan` — stored verbatim (JSONB) and
matched with the same type, so there is no adapter to drift. All plans
live in **one `plans` table** (`pid`, nullable `kind`, `name`, `data`
JSONB `Plan`, `parent_pid`, `active`, soft delete) behind one REST
collection (`/api/plans`). Any plan may contain any other via `parent_ref`
(a self- or descendant-cycle is `422`). `kind` is a label, never a
discriminator; **matching is kind-agnostic** (no kind gate).
Operational data (tasks, risks, bookings, …) lives in its own tables and
is **never** fed to the matcher; only goal **titles** bridge in via
`data.goals[]`.

| Question | Answer |
|---|---|
| Run | `cargo run -- start` (needs Postgres; port 5150; migrations auto-run in development) |
| Lint (must be clean) | `cargo clippy --all-targets --all-features -- -D warnings` · `cargo fmt --check` |
| DB-free tests | `cargo test --lib` (unit) and the DB-free binaries (`--test matching`, `--test otlp_export`, `--test otlp_middleware`, `--test licence_files`) |
| DB tests | `#[ignore]`d; need `DATABASE_URL` (`compose.test.yaml` provides Postgres: `loco`/`loco` on 5432) — see Testing |
| Features | `paseto` (default), `keycloak` (also accept Keycloak access tokens), `fluvio` (real-broker sink); all off except `paseto` |
| Migrations | `migration/src/` (crate `project-portfolio-management-migration`) — files `m2026…`; the count is in the entity spec's [§14 generated block](../spec/14-implementation-status.md) |

## API surface

Routes are under `/api/`; plans at `/api/plans`, plan-scoped resources at
`/api/plans/{pid}/…`. URLs are version-free; select a version with the
`Accepts-version` header (default `1.0`, `src/version.rs`). The OpenAPI
document (`src/openapi.rs` + `src/openapi_more.rs`) is hand-written and
pinned by tests: **every new route must be added there**, a new write must
document its `requestBody` (`WRITES_WITHOUT_REQUEST_BODY` only shrinks), and
`api-routes.txt` (the mounted routes, which the UI's tests check its client
against) is regenerated with `UPDATE_API_ROUTES=1 cargo test --lib api_routes_file`. Spec §9 is the
full contract; this is the map. One module per area in
`src/controllers/`, each with a pure-logic sibling in `src/`.

| Group | Paths |
|---|---|
| Plan CRUD | `POST`/`GET /plans`, `GET`/`PUT`/`DELETE /plans/{pid}` (record-level ABAC; `mask` obligation returns the redacted view), `/plans/{pid}/masked`, `/plans/{pid}/export` (GDPR), `GET /plans/search?q=` (Tantivy: `fuzzy`, `phonetic`, `kind`) |
| Match / merge | `POST /plans/match` · `/plans/check-duplicates` · `POST /plans/merge` (`422` equal pids, `404` unknown) · `GET /plans/merges/recent` |
| Bulk | `POST /plans/import` (202 + job id; body ceiling) · `/plans/import/{id}` · `POST /plans/export` · `/plans/export/{id}[/download]` · `/plans/bulk-jobs` (+ `/sweep`, `/{id}/errors`, `POST /{id}/cancel` — cooperative cancel, monotone `progress`) · `/plans/review-queue` (+ `/{pid}/confirm`/`reject`). Auth happens at request time (workers have no claims) |
| Task bulk | `POST /tasks/bulk` (edit) · `/tasks/copy` · `/tasks/move` — one result per task, `ok + refused == requested`; `status` is never bulk-editable |
| Task read | `GET /plans/{pid}/tasks/{t_pid}?include=` — sub-resources in one response; module-off omitted (`module_disabled`), people-bearing omitted (`masked`), unknown name `422` listing allowed |
| Strategy | `/ideas` · `/scenarios` (+ `evaluate`/`commit`) · `/objectives` (+ `alignment`) · `/plans/{pid}/objectives` · `/plans/{pid}/benefits` |
| Visibility | `/dependencies` · `/plans/{pid}/schedule` · `/milestones` · `/allocations` · `GET /capacity` · `/reports` · `GET /at-a-glance` (ETag) |
| Governance | `/proposals` (+ `submit`/`review`/`approve`/`reject`/`promote`/`duplicates`) · `/plans/{pid}/gate-reviews` · `/risks` (+ `escalate`) · `/budget-lines` (+ `actual`/`release`) · `GET /plans/{pid}/governance` |
| Executive / oversight | `/executive/*` · `/financials/*` · `/technology/*` · `/scenarios/compare` · `/board/*` · `/auditor/*` · `/compliance/{register,findings}` · `/risk/heatmap` · `/security/register` · `/regulator/extract` (persona gating = ABAC policy config) |
| Integrity | `GET /compliance/records/verify` · `/compliance/audit/verify` (`?limit=`, cap 10 000; `mac_absent` when `PORTFOLIO_INTEGRITY_MAC_KEY[_FILE]` unset) |
| Engineering | `/plans/{pid}/tasks` (+ move with WIP caps, story points) · `/sprints` · `burndown` · `velocity` · `standup` · `/devops/{events,metrics,releases}` · `/engineering/*` |
| Time-based analysis | `/plans/{pid}/{time-analysis,constraints,aging-wip,flow,cumulative-flow,forecast,rollup}` · per-task `transitions`/`time-analysis` · `/flow-classes`. Read-only; Monte-Carlo `forecast` over throughput, deterministic by seed |
| Workflows / phases / OKR / flow / effort / ceremonies / TPC / controls / value | `/workflows` · `PUT /plans/{pid}/phase` (one-step; skip `422`; regression needs a reason) · `/key-results` + check-ins · `/flow-distribution` · `/time-entries`, `/effort`, `/capacity/utilization` · `/sprints/{pid}/{ceremonies,commit,commitment}` · `/plans/{pid}/tpc`, `/tpc` · `/controls` (+ `register-standard`, coverage, readings, actions) · `/business-case`, `/value-realization`, `/performance` |
| Critical path / schedule | `GET /plans/{pid}/cpm` (float across **cross-plan** links: at most `MAX_LINKED_PLANS` = 8 plans / `MAX_LINKED_TASKS` = 3000 tasks; unreadable plans appear only as restricted edges) · `PUT …/tasks/{t_pid}/schedule` · baselines · calendars · leveling · schedule health/risk |
| Hammocks (PJ-7) | `POST …/tasks/{t_pid}/hammock` · `GET /plans/{pid}/hammocks` · `POST …/hammocks/{h_pid}/links` (+ delete member). A hammock has no duration/work/constraint/dependency, is **not in the dependency network** (never drives float); span derived on read |
| What-if sandbox (PJ-9) | `POST /plans/{pid}/sandbox/preview` (in-memory, writes nothing) · `…/sandbox/commit` (one transaction, audit per item, needs `If-Match: <plan_etag>`; stale = `409`, never a merge; leveling apply goes through it) |
| Assignments / cost | `PUT …/tasks/{t_pid}/assignment` (fixed units/work/duration rule) · `…/contour` · `/plans/{pid}/work-profile` · `/work-profile` · `/cost-rate-tables` (+ `rates`, append-only effective-dated) · `/cost-rates/lookup` · `…/tasks/{t_pid}/costs` · `…/cost-plan` · `…/labour-costs[/record]` (snapshots the rate used; per currency) |
| Onboarding / handover | `/onboarding/templates[/{kind}]` · `/plans/{pid}/onboarding/cases` · `/onboarding/cases/{pid}[/items/{item}\|complete\|cancel]` · `/plans/{pid}/access-records` (append-only) · `GET …/closeout` · `POST …/handover/preview` · `…/handover/commit` (`If-Match: <handover_etag>`, `409` if holdings changed). People-bearing parts need the manage-people permission; migration 48 allows one open case |
| Processes (PL-2) | `/processes` CRUD (each edit = new immutable version) · `POST /processes/{pid}/start` · `/process-executions` (+ `cancel`, `resume`). Steps `approve`/`todo`/`notify`; run pinned to its version; state and append-only log in the DB; a rejection skips and logs later steps |
| Approvals (PL-1) | `/approvals` (+ `decide`, `withdraw`) — only the named approver decides |
| Controlled documents (PL-6) | `/documents` · `…/in-force` · `…/versions[/{v}]` (+ `submit`, …) — `draft → in_review → approved → effective → superseded`; effectiveness requires a real PL-1 approval; text immutable past draft **in the database** (trigger `document_versions_guard`, migration 47) |
| Announcements | `GET`/`POST /announcements`, `PUT`/`DELETE /announcements/{pid}` — publish/edit/retract need the `comms` area (`access=admin` or `comms=publish`) else `403`; `?include=all` for publishers |
| Access / privacy | `GET`/`PUT /plans/{pid}/access` (private plan + members) · `/plans/{pid}/guests` (+ `revoke`) · `/plans/{pid}/modules` · `/plans/{pid}/links` (typed links) |
| Collaboration / automation | `/reviews` · `/assignees/workload` · `/notifications` · `/automations` · `/scheduled-actions` (+ `sweep`) · `/automations/milestones/sweep` (a rule/milestone pair fires once, ever) · saved views, canvases, comments, attachments, custom fields, components, templates, timesheets, `/my-work`, report schedules, notification sinks |
| Audit / auth / docs | `/plans/audit/recent` · `/plans/{pid}/audit` · `/plans/events/recent` · `GET /plans/whoami` · `/api-docs/openapi.json` · `/swagger-ui` · `/metrics.prom` · loco `/_health`, `/_ping` |

Every plan/record write writes an `audit_logs` row (or, if that insert fails, logs it and counts it in
`audit_write_failed_total` via `AuditModel::record_or_log`; never `record(..).await.ok()`, CQ-3) and publishes a
`created`/`updated`/`deleted` (and `merged`) event. **Not wired** (spec
§13): `POST /plans/deduplicate`, `/plans/{pid}/timeline`, gRPC, `posts`/`members` plan
sub-resources (a `/comments` route exists, as the discussion feature, not
as that sub-resource). Goals (`/plans/{pid}/goals`) and the issues register
(`/plans/{pid}/issues`, `/api/issues`) are built. The full route list is
`api-routes.txt`, written by a unit test; `openapi.json` documents every route in it.

**The workforce-service bridge (FB-1 to FB-5).** `src/wpm_bridge.rs` (the rules and the three outbound
calls), `controllers/capacity_heatmap.rs` (`PUT /api/skill-pools/{pid}/peer-link`, the heatmap's supply reading,
the demand send on `PUT /api/capacity-commitments`), `controllers/people_impact.rs`
(`GET /api/plans/{pid}/people-impact`), migration `m20261030_000099_peer_pool_link`. Off until
`PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL` is set; the URL must be `https://` or loopback `http://`. Every call
forwards the caller's own bearer token (no service account), has a 2-second timeout, follows no redirects and never
fails the PPM request. Supply figures are kept per caller in memory only; nothing else is kept. Test:
`tests/wpm_bridge.rs` (a stand-in WPM on a local port; `--ignored`, needs the database). Rules: entity spec §8.5a.
Walk-through: [`../operations/wpm-bridge.md`](../operations/wpm-bridge.md).

## Cross-cutting machinery

- **Middleware order** (`src/app.rs::after_routes`, innermost first):
  `paging_mw` → `modules_mw` → `privacy_mw` → `guest_mw` →
  `require_auth_mw` (**on by default**; opt out with
  `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false` or `settings.require_auth:
  false` in a non-production config, which `development.yaml`/`test.yaml` do;
  production refuses to start with it off) → `require_version_mw` →
  `rate_limit_mw` → `cors_mw` → `security_headers_mw` →
  `observability::trace_mw` (outermost; its span wraps auth failures and
  `429`s). See spec NFR-12 to NFR-14.
- **Paging** (`controllers/paging.rs`): any `GET` answering `200` with a
  bare JSON array is a list: `?limit=` (default and ceiling 500),
  `?offset=` (> 10 000 is `400`), headers `X-Total-Count`/`X-Limit`/
  `X-Offset`; body stays the bare array. Handlers that page in the
  database and set `X-Total-Count` themselves are left alone; the rest are
  sliced by the layer. `openapi::PAGED_LISTS` is the census of such lists;
  `tests/list_paging.rs` walks every documented `GET` and fails if a list is
  not in the census (or an unreachable `GET` is not named in `UNREACHED`
  with a reason). **Add a new list to `PAGED_LISTS`.**
- **Private-plan guard** (`controllers/plan_access.rs`): `privacy_mw`
  returns `404` (existence not disclosed) for any `/api/plans/{pid}/…` of a
  private plan to a non-member/non-admin, failing closed on lookup error
  for private plans only. Aggregate views (`insights`, `oversight`,
  `strategy`, `distribution`, `financials`, `prioritization`, `tba`,
  `visibility`, …) must route their rows through `HiddenScope`
  (`keep`/`plan`/`node`) so a private plan contributes to **no** total,
  list or count. `tests/private_aggregates.rs` is differential (non-member
  sees the world as if the private plan did not exist; member and admin see
  full, identical totals) and **enumerates the mounted GET routes**, so a
  new aggregate route fails it until someone decides whether it filters.
- **Auth** (`src/auth/`: `verifier.rs`, `enforcement.rs`, `policy.rs`, `areas.rs`, `identity.rs`): offline PASETO v4.public verification via
  `authentication-verifier` (published Ed25519 key; boot fetch from
  `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL`, env
  `…_PASETO_KEYS` fallback, service always boots). `AuthUser` requires a
  token; `MaybeAuthUser` feeds the audit `actor`. Policy (ABAC,
  `service_rules`) decides record access, areas (`comms`, finance,
  manage-people) and personas.
- **Privacy** (`src/privacy.rs`): thin field masking (`lead_ref` dropped,
  owner org masked to its tail) + audited GDPR export.
- **Events**: `streaming.rs` in-memory stream (default) or durable outbox
  (`PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT=outbox`, relay in
  `relay.rs`); `FluvioSink` behind the `fluvio` feature
  (`…_FLUVIO_ENDPOINT` set without the feature compiled in makes the relay
  refuse to start); best-effort signed webhooks (`webhooks.rs`,
  `…_WEBHOOKS[_FILE]`, `X-Mxi-Signature` HMAC, `5xx` retried, `4xx` not,
  every outcome in `webhook_deliveries`) composed via `CompositeSink`.
- **Observability** (`src/observability.rs`): OTLP/gRPC export **on by
  default** (`OTLP_ENDPOINT`, default `http://localhost:4317`; set `""` to
  disable; `OTLP_SERVICE_NAME`; both deliberately unprefixed).
  `trace_mw` is layered once because `App::routes`/`after_routes` is the
  only router-construction surface.
- **Search**: Tantivy (`src/search/`); `kind` is a search filter, but
  `SearchEngine::candidates` (the `check-duplicates` blocking query) must
  **never** filter on it.
- **Background tickers** (all env-gated, default off unless noted):
  `scheduler.rs` (scheduled-action and milestone-due sweeps),
  `snapshots.rs`, `flow_metrics.rs` (`ppm_flow_*` gauges), `sinks`
  deliveries (0 disables). The bulk-job sweep is an endpoint, not a ticker.

## Golden rules

1. **Spec-first.** Behavioural change updates `spec/`; the entity spec owns
   the canonical model (§5).
2. **Loco-idiomatic.** Controllers register in `app.rs`; tables are
   `sea-orm-migration` migrations; pure rules live in `src/<area>.rs` with
   no I/O and the controller is wiring.
3. **Reuse the matcher type**; do not fork a `Plan` DTO.
4. **One recursive collection**; no per-kind collections.
5. **Kind-agnostic matching**; dedup, check-duplicates and merge are not
   scoped by kind.
6. **Partition rule.** Operational sub-resources never reach the matcher.
7. **Absent evidence is `null` with a reason, never zero**; every write is
   audited; people are references; new endpoints are pinned in
   `src/openapi.rs`.
8. **Zero `#[allow]`**; clippy `-D warnings` clean on `--all-targets
   --all-features`.

## Gotchas (hard-won)

- **Pool size 1 deadlocks.** `config/{development,test}.yaml` default
  `DB_MAX_CONNECTIONS` to `1` (production 20). Holding a transaction while
  also querying `ctx.db` waits forever for a second connection. Inside a
  transaction use that transaction handle for every query; do reads first
  or after.
- **The verifier is a process-wide `OnceLock`** (`auth::VERIFIER`, seeded
  once by `auth::init`). A test binary can configure only one key set, so
  every real-token test lives in its **own binary** (`private_aggregates`,
  `list_paging`, `approvals`, `processes`, `controlled_documents`,
  `cross_plan`, `guests`, `plan_access`, `enforcement`, …). Do not add
  real-token tests to `tests/mod.rs`.
- **Percent-decode paths before parsing in guards.** Handlers see decoded
  segments; `plan_pid_of` decodes first because `%7B<uuid>%7D` (braced
  UUID) bypassed a guard that parsed raw text. Any new path-parsing
  middleware must do the same (and fail closed).
- **Migrations**: use `CREATE OR REPLACE FUNCTION` for trigger functions
  (re-runnable; see migrations 45–47); `create_table` pluralizes names, so
  reference the plural table name in later migrations and entities;
  register each file in `migration/src/lib.rs`. Append-only and immutability
  rules (processes, document versions) are database triggers, not just
  handler checks.
- **Optimistic commits**: sandbox and handover commits require `If-Match`
  with the preview's etag; stale is `409`. Hammock links and schedule
  edits are serialized (commit `cf55b22`).
- **Network loaders are bounded** (cross-plan caps above); keep new
  loaders bounded the same way.
- loco 0.16+ does not map `ModelError::EntityNotFound` to `404`; route
  lookups through `controllers::model_not_found`.
- `config/production.yaml` has no JWT `auth:` block (loco's `auth` feature
  is off; the family issues PASETO) and the Dockerfile must `COPY benches/`
  (manifest declares `[[bench]] service_bench`).

## Testing

| Layer | Command |
|---|---|
| Unit | `cargo test --lib` (DB-free) |
| DB-free binaries | `cargo test --test matching --test otlp_export --test otlp_middleware --test licence_files` |
| Request suites (`tests/mod.rs` → `tests/requests/*`) | `cargo test --test mod -- --ignored --test-threads=1` (shared database; serial) |
| Real-token binaries | `cargo test --test <name> -- --ignored` for e.g. `private_aggregates`, `list_paging`, `approvals`, `processes`, `controlled_documents`, `cross_plan` |
| Keycloak | `cargo test --features keycloak --test keycloak_guard` (in-process test IdP); `--features keycloak --test keycloak_container -- --ignored` needs Podman |
| Fluvio | `fluvio_relay.rs`, needs the `fluvio` feature and a broker (`compose.fluvio.yaml`) |

DB tests are `#[ignore]`d so a plain `cargo test` is DB-free. Mint tokens
in-process (ed25519 + `rusty_paseto`, as `private_aggregates.rs` does); no
auth-service is needed. A new list belongs in `PAGED_LISTS`; a new
aggregate GET needs a decision in `private_aggregates`.

## Layout

```
src/
├── app.rs                  loco Hooks: routes, middleware stack, tickers
├── controllers/            one file per area (HTTP wiring only); paging.rs, plan_access.rs (privacy_mw, HiddenScope), …
├── <area>.rs               pure rules beside each controller (hammock, sandbox, processes, documents, cost_rates, scheduling_rules, work_profile, onboarding, tba, tpc, …)
├── models/                 helpers + _entities/ (SeaORM)
├── auth.rs · privacy.rs · version.rs · validation.rs · merge.rs
├── search/                 Tantivy index + engine
├── streaming.rs · relay.rs · webhooks.rs · sinks.rs      events, outbox, sinks
├── observability.rs · metrics.rs · flow_metrics.rs
├── openapi.rs · openapi_more.rs     hand-written OpenAPI + PAGED_LISTS
├── compliance/             keyed integrity (mac, record_integrity, audit_integrity)
└── tasks/search.rs         search_reindex CLI task
migration/src/              ordered m2026… migrations (lib.rs registers them)
tests/                      mod.rs + requests/ (shared DB), one binary per real-token suite, otlp_*
config/                     development / production / test yaml
```

## Container image

`Dockerfile` (multi-stage, Debian 13 slim) must be built with the
**repository root** as context (it reads the root `rust-toolchain.toml`; the
matcher, `authentication-verifier` and `integrity-mac` all come from crates.io, and the `migration/` crate is a path
dependency inside this directory):
`podman build -f project-portfolio-management-service-with-rust/Dockerfile -t project-portfolio-management-service .`
Exclude every `target/` via `.containerignore`. `GET /_health` returns
`200` once Postgres is reachable.

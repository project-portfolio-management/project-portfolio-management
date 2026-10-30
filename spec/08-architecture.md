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
                               | REST (raw loco JSON, no envelope)
                               | PROJECT_PORTFOLIO_MANAGEMENT_API_URL (default :5150)
+------------------------------v-------------------------------+
|              project-portfolio-management-service-with-rust                     |
|  loco.rs 0.16 (Axum 0.8) · port 5150                          |
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

### 8.3 Service layout (loco.rs)

> **Corrected 2026-08-25.** This section was headed "— planned" and drew
> a layout that never landed: a controller per sub-resource
> (`goals.rs`, `tasks.rs`, `issues.rs`, `timeline.rs`, `burndown.rs`),
> plus `links.rs` and `bulk.rs` for features §14 lists as **open gaps**
> (T-7, T-8). The real crate groups controllers by *capability* rather
> than by resource. Drawn from the tree, not from the plan:

```
project-portfolio-management-service-with-rust/
├── src/
│   ├── app.rs                       loco Hooks (routes, workers, boot init)
│   ├── bin/                         loco CLI entrypoint
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
│   └── workers/                     loco background jobs
├── migration/src/                   plans, audit_logs, merge_records, event_outbox,
│                                    governance, visibility, strategy, engineering,
│                                    capabilities, insight_snapshots,
│                                    integrity_digests, time_based_analysis
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
| Delivery ideas | CRM → PPM | `POST /api/ideas` `{title, pitch, tags}` → `{pid}` (CRM's `POST /api/bridges/ppm/deals/{pid}`) | exists (PPM's route; CRM calls it) |
| Pool link | PPM ↔ WPM | `skill_pools.peer_pool_ref` = WPM's pool pid, set on both sides by a person | **built** (FB-1); set and removed on the `/skill-pools` screen (FB-6) |
| Supply in | WPM → PPM | `GET {WPM}/api/capacity/peer-supply?pool=&from=YYYY-MM-01&months=N` | **built** (FB-2); shown on the `/capacity-heatmap` screen with its source and reading (FB-6) |
| Demand out | PPM → WPM | `POST {WPM}/api/capacity/peer-demand` | **built** (FB-3); zero sent as the withdrawal, and every sent figure withdrawn on unlink (FB-5); set on the `/capacity-commitments` screen, which shows the send outcome and the recent sends (FB-6) |
| People impact | WPM → PPM | `GET {WPM}/api/capacity/people-impact?programme=thing:<plan pid>[&ends_on=YYYY-MM-DD][&months=N]` | **built** (FB-4): `GET /api/plans/{pid}/people-impact`, shown on a programme's plan page |
| Hire and leave | WPM → CRM | `worker_hired`, `worker_terminated`/`worker_retired` webhooks | CRM's and WPM's; nothing in PPM |

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

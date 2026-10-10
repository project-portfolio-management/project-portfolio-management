## 9. API Surface

Endpoint detail: [`agents/restful.md`](../agents/restful.md); source:
`project-portfolio-management-service-with-rust/src/controllers/`. The
service is implemented (§14); the route inventory in §9.7 is generated
from the controllers' `routes()` and is the check against drift. The
tables in §9.2–§9.2c keep their original FR-oriented shape and carry
verified corrections where the mounted route differs. The documented
OpenAPI surface is `src/openapi.rs` + `src/openapi_more.rs`.

### 9.1 Service REST API — plans (thin record)

The single collection `/api/plans` exposes the surface below; all
matching / dedup / merge is **kind-agnostic** (§5.5). List additionally
accepts a `?parent=<pid>` roll-up filter.

| Method | Path | Purpose | Returns |
|---|---|---|---|
| POST | `/api/plans` | Create (body: `Plan`, optional `kind` label; blank `name` rejected; `409` with candidates on likely duplicate, `force` to bypass) | `{pid, name}` |
| GET | `/api/plans` | List active, most-recent first, capped 100 (`?parent=<pid>` filter) | `[{pid, name}]` |
| GET | `/api/plans/search?q=` | Case-insensitive name search (Postgres `ILIKE`, cap 50); blank `q` → `400` | `[{pid, name}]` |
| GET | `/api/plans/{pid}` | Fetch the stored thin `Plan` | `Plan` |
| PUT | `/api/plans/{pid}` | Replace the thin payload | `{pid, name}` |
| DELETE | `/api/plans/{pid}` | Soft-delete (cascades to sub-resources on read paths) | empty JSON |
| POST | `/api/plans/match` | Rank `{query, candidates}` (no persistence) | `[(index, MatchResult)]` |
| POST | `/api/plans/check-duplicates` | Match a query against stored plans | `[{pid, name, score, confidence, is_match}]` sorted by score |
| POST | `/api/plans/merge` | Merge a duplicate into a survivor (any two plans; `{main_pid, duplicate_pid, reason?}`; re-homes sub-resources); equal pids → `422`, unknown → `404` | `{main_pid, duplicate_pid, main: Plan}` |
| GET | `/api/plans/merges/recent` | Recent merge-history records, newest first | `[MergeRecord]` |
| GET | `/api/plans/whoami` | Echo verified bearer-token claims; `401` without a valid token | `Claims` |
| GET | `/api/plans/audit/recent` | Recent audit-log entries (plans + sub-resources), newest first, cap 100 | `[AuditLog]` |
| GET | `/api/plans/{pid}/audit` | Audit trail for one plan (incl. its sub-resources), newest first | `[AuditLog]` |
| GET | `/api/plans/events/recent` | Recent events from the in-memory stream | `[PlanEvent]` |
| GET | `/api-docs/openapi.json` | OpenAPI 3 document | `OpenAPI` JSON |
| GET | `/swagger-ui` | Swagger UI (CDN assets) | HTML |

Plus loco's built-in `/_health` and `/_ping`.

Every create / update / delete / merge (on a plan **and** every
sub-resource) writes a best-effort `audit_logs` row and publishes a
`PlanEvent` to the in-memory stream; merge additionally writes a
`merge_records` row with a snapshot of the transferred payload. A
durable broker is roadmap (§15). Bearer-token verification (a PASETO v4
public token, verified offline against the auth-service's published
Ed25519 key;
`agents/share/authentication-sessions.md` (shared family guidance, not in this repository),
supersedes the RS256-JWT model) is via the `AuthUser` / `MaybeAuthUser`
extractors; `whoami` is protected; writes stamp the audit `actor` from
the token when present. Blanket `/api/*` enforcement + paseto-keys-over-HTTP
fetch are follow-ups (§13).

Conventions: **raw loco JSON** (no `{success, data, error}` envelope).
`404` for unknown / soft-deleted `pid`; `422` for a validation failure
(blank `name`, malformed `EntityRef` / `parent_ref` / containment cycle
/ deterministic identifier / relationship / `in_language` — FR-1a);
`400` for a malformed body; `409` for a real-time duplicate on create
(FR-11a).

### 9.2 Service REST API — operational sub-resources

Each sub-resource is nested under its plan. None enters the matcher
payload (§5.6); all writes emit events + audit rows.

| Method | Path | Purpose |
|---|---|---|
| GET/POST | `/api/plans/{pid}/goals` · PUT/DELETE `…/goals/{id}` | Goals — writes also mutate `data.goals[]` (§5.3) |
| GET/POST | `/api/plans/{pid}/tasks` · PUT/DELETE `…/tasks/{id}` | Tasks (board / list; status, assignee, estimate, due, nesting) |
| GET/POST | `/api/plans/{pid}/issues` · PUT/DELETE `…/issues/{id}` | Issues (kind / severity / status / assignee) |
| GET | `/api/plans/{pid}/timeline` | Derived Gantt projection (read-only) |
| GET | `/api/plans/{pid}/burndown` | Derived remaining-vs-estimate (read-only) |

### 9.2a Service REST API — collaboration / automation / prioritization

Delivered 2026-07-22 (§6.4a; service spec §9.4a). Derived views carry
`as_of` and are ETag-conditional.

| Method | Path | Purpose |
|---|---|---|
| POST/GET | `/api/reviews` · POST `…/{pid}/respond` · `…/{pid}/submit` · DELETE `…/{pid}` | Collaborative review invitations + verdicts |
| GET | `/api/reviews/consensus` | Strict-majority consensus for one subject |
| POST | `/api/plans/{pid}/tasks/{t_pid}/assign` | Assign / unassign a task |
| GET | `/api/assignees/workload` | Open work per assignee (incl. `unassigned`) |
| GET | `/api/notifications` · POST `…/{pid}/read` | In-app inbox (no email / push) |
| POST/GET | `/api/automations` · POST `…/{pid}/enable`·`/disable` · DELETE `…/{pid}` | Workflow-automation rules |
| GET | `/api/automations/runs` | What fired: applied / skipped / failed |
| POST/GET | `/api/scheduled-actions` · POST `…/sweep` · DELETE `…/{pid}` | Set-and-forget deadline queue |
| GET | `/api/plans/{pid}/smart-score` · `/api/prioritization` | Smart Score + ranked queue (with breakdown) |
| GET | `/api/lifecycle` · `/api/plans/{pid}/lifecycle` | Funnel + next-phase readiness checklist |

### 9.2b Service REST API — full-suite capabilities (§1.4–§1.6)

**Built** (§14.1; T-15 … T-27). Derived views carry `as_of` and are
ETag-conditional, matching §9.2a. The table below is the original FR
shape; where it differs from the mounted routes, the **corrections
list after it** is authoritative (verified against the controllers;
§9.7 is the full inventory).

| Method | Path | Purpose | FR |
|---|---|---|---|
| GET/POST | `/api/workflows` · PUT/DELETE `…/{pid}` | Workflow configuration (task / issue states, categories, transitions, WIP limits) | FR-26 |
| GET | `/api/plans/{pid}/workflow?applies_to=task\|issue` | The workflow in force for this plan (its own, else the default) | FR-26 |
| GET/POST | `/api/plans/{pid}/goals/{gid}/key-results` · PUT/DELETE `…/{kr_pid}` | Key results under an objective | FR-27 |
| GET/POST | `/api/key-results/{kr_pid}/check-ins` | Dated check-ins (value + optional confidence) | FR-27 |
| GET | `/api/plans/{pid}/okr` | Derived objective and plan scores with per-key-result breakdown; `unmeasured` where no metric exists | FR-27 |
| GET/POST | `/api/plans/{pid}/time-entries` · PUT/DELETE `…/{te_pid}` | Recorded effort | FR-28 |
| GET | `/api/plans/{pid}/effort` | Roll-ups per task / plan / assignee, labelled **asserted** | FR-28 |
| GET/POST | `/api/plans/{pid}/sprints` · PUT/DELETE `…/{s_pid}` | Sprints | FR-29 |
| GET/POST | `/api/sprints/{s_pid}/ceremonies` · PUT/DELETE `…/{c_pid}` | Planning / daily / review / retrospective + notes | FR-29 |
| POST | `/api/sprints/{s_pid}/commit` | Write the planning commitment snapshot | FR-29 |
| POST | `/api/ceremony-notes/{n_pid}/convert` | Note → task (`action`/`feedback`) or → issue (`blocker`) | FR-29 |
| PUT | `/api/plans/{pid}/phase` | Advance or (with a reason) move back; a skip is `422` | FR-30 |
| GET | `/api/plans/{pid}/phase-history` | Transitions + duration per phase, every phase present at zero | FR-30 |
| GET | `/api/plans/{pid}/flow-distribution` | Feature / defect / risk / debt mix, `unclassified` counted separately; `?rollup=true` walks the subtree | FR-31 |

**Corrections (verified against `src/controllers/`).**

- Workflows: `POST`/`GET /api/workflows`, `DELETE /api/workflows/{pid}`,
  `GET /api/plans/{pid}/workflow`; there is **no** `PUT` (withdraw and
  re-register).
- Key results hang off **objectives**, not goals:
  `POST`/`GET /api/objectives/{pid}/key-results`,
  `PUT`/`DELETE /api/key-results/{pid}`, `POST`/`GET
  /api/key-results/{pid}/check-ins`; scores at `GET /api/plans/{pid}/okr`
  and `GET /api/okrs`. `PUT`/`DELETE /api/objectives/{pid}` exist.
- Effort: `POST`/`GET /api/plans/{pid}/time-entries` and `GET
  …/effort` only (no `PUT`/`DELETE`); `POST /api/working-time`, `POST
  /api/non-working`.
- Sprints: `POST`/`GET /api/plans/{pid}/sprints`; ceremonies `POST`/`GET
  /api/sprints/{pid}/ceremonies`; `POST /api/sprints/{pid}/commit`, `GET
  …/commitment`. There is no `/api/ceremony-notes/…/convert`; the
  converter is `POST /api/plans/{pid}/sprints/{s_pid}/notes/{n_pid}/convert`.
- Phase: `PUT /api/plans/{pid}/phase`, `GET …/phase-history`.
- Flow distribution: `GET /api/plans/{pid}/flow-distribution` as listed.

**Not added, deliberately.** There is no endpoint for Flow Time,
Velocity, Efficiency or Load: those are already served by
`GET /api/plans/{pid}/{time-analysis,flow,constraints,aging-wip}`
([time-based-analysis.md](time-based-analysis.md) §10). Adding
Flow-Framework-named aliases would put the same number behind two names,
which §1.6 exists to prevent.

### 9.2c Service REST API — value realization and performance (§6.4c)

**Built.** Corrections to the table: the value routes are `POST`-only
writes (`POST /api/plans/{pid}/{business-case,value-points,adoption,satisfaction}`)
read back through `GET …/value-realization` and `GET …/performance`;
there is **no** `GET /api/performance` portfolio route and no `PUT` or
`DELETE` on business-case targets. TPC: `POST`/`GET /api/plans/{pid}/tpc`,
`GET …/tpc/report`, `GET /api/tpc` (no `DELETE`). Controls:
`POST`/`GET /api/plans/{pid}/controls`, `POST …/controls/register-standard`,
`DELETE /api/controls/{pid}`, `POST`/`GET /api/controls/{pid}/readings`,
`POST /api/readings/{pid}/actions`, `POST /api/actions/{pid}/convert`
(task conversion only), coverage at `GET /api/plans/{pid}/controls/coverage`
and `GET /api/controls/coverage` (no `PUT`, no `close`, no `GET` on
actions). `GET /api/capacity/utilization` is mounted from `effort.rs`.

| Method | Path | Purpose | FR |
|---|---|---|---|
| GET/POST | `/api/plans/{pid}/business-case` · PUT/DELETE `…/{t_pid}` | Charter / gate-approved targets, with the approval that set them | FR-33 |
| GET/POST | `/api/plans/{pid}/value-points` | Observed value, each carrying its `method` and the first-measurable flag | FR-33 |
| GET/POST | `/api/plans/{pid}/adoption` | Adoption snapshots (active ÷ target, with definition + window) | FR-33 |
| GET | `/api/plans/{pid}/value-realization` | Transformation ROI · Value Realization Rate · Time to Value · Adoption Rate · Performance to Business Case; `?rollup=true` walks the subtree | FR-33 |
| GET | `/api/plans/{pid}/performance` · `/api/performance` | The six-dimension metric set, per plan and portfolio-wide | FR-34 |
| GET | `/api/capacity/utilization?by=plan\|team\|person` | Effort against declared capacity at all three levels, each with its numerator, denominator and the period's queue figures; suppressed below the capacity floor. Per-assignee cycle time / throughput / flow efficiency remain **absent** | FR-35 |
| GET/POST | `/api/plans/{pid}/satisfaction` | NPS / CSAT responses; the aggregate always carries response count and rate | FR-36 |
| GET/POST | `/api/plans/{pid}/tpc` · DELETE `…/{t_pid}` | TPC observations: DIPP, EMV, CEC, PI numerator/denominator | FR-37 |
| GET | `/api/plans/{pid}/tpc/report` | Derived DIPP + progress index + the stored-vs-computed divergence finding | FR-37 |
| GET | `/api/tpc` | Portfolio triage ranked by DIPP descending, within one currency | FR-37 |
| GET/POST | `/api/plans/{pid}/controls` · PUT/DELETE `…/{c_pid}` | Control register: standard, timing, source, cadence | FR-38 |
| GET/POST | `/api/controls/{c_pid}/readings` | Append-only readings with verdict + gap | FR-38 |
| GET/POST | `/api/readings/{r_pid}/actions` · POST `…/{a_pid}/close` | The action a failing reading provoked; converts to a task or issue | FR-38 |
| GET | `/api/plans/{pid}/controls/coverage` · `/api/controls/coverage` | What is **not** controlled: never-read controls, overdue cadences, phases with no controls, unanswered failures | FR-39 |

**Response conventions**, uniform across both: every ratio ships its
numerator and denominator; a null figure carries a `reason` rather than
a sentinel zero; every derived view carries `as_of` and is
ETag-conditional (§9.2a); and any monetary figure names its currency,
since none of these metrics converts between them.

### 9.2d Service REST API — scheduling and cost canon (SC / PJ, landed 2026-10)

All plan-scoped paths are under `/api`. Full inventory in §9.7; the
behaviours worth pinning:

| Method | Path | Purpose / status codes |
|---|---|---|
| POST | `/plans/{pid}/tasks/{t_pid}/hammock` | Turn a task into a hammock (or back); a derived span, so it must carry no duration, work, constraint or dependency, else `422` |
| GET | `/plans/{pid}/hammocks` | Each hammock with the span derived on read from its links |
| POST / DELETE | `/plans/{pid}/hammocks/{h_pid}/links` · `…/links/{member}` | Link a hammock to tasks (or hammocks) of the **same plan**; a duplicate link or a cross-plan member is `422` |
| POST | `/plans/{pid}/sandbox/preview` | What-if: apply edits (move, duration, add/remove link, constraint) to an **in-memory copy**; writes nothing (no row, no audit); same edits, same answer; answers dates, float, critical path, labour-cost impact; applies the same masking, hard-constraint permission and finance gate as a commit |
| POST | `/plans/{pid}/sandbox/commit` | Apply the same edits in one transaction, one audit entry per changed item; needs `If-Match: <plan_etag from the preview>`; a changed plan is `409`, never a merge. The leveling apply (`POST …/leveling/apply`) goes through the same commit |
| PUT | `/plans/{pid}/tasks/{t_pid}/assignment` · `/contour` | Units / work / duration under a scheduling rule (`fixed_units`, `fixed_duration`, `fixed_work`; the third recomputed) · work contour (`flat`, `front_loaded`, `back_loaded`, `bell`, `turtle`, `custom`); an assignment needs an assignee |
| GET | `/plans/{pid}/work-profile` · `/work-profile` | Work as day/week/month buckets, derived on read |
| POST / GET | `/cost-rate-tables` · POST `…/{pid}/rates` · GET `/cost-rates/lookup` | Effective-dated, append-only rates per person **or** role |
| POST / GET / DELETE | `/plans/{pid}/tasks/{t_pid}/costs` · `…/{c_pid}` | Material / fixed expenses with accrual `start`/`prorated`/`end` |
| GET | `/plans/{pid}/cost-plan` | Planned cost by period, **per currency** |
| POST / GET | `/plans/{pid}/labour-costs/record` · `/labour-costs` | Cost of recorded time, snapshotted with the rate used |
| GET | `/plans/{pid}/cpm` | Network **including tasks of other plans it is linked to** (cross-plan float; each node on its own plan's calendar). A link to a plan the caller cannot read is left out of the calculation and shown only as a restricted edge; cross-plan edges carry `cross_plan: true`. Bounded: a cap on the other plans and tasks followed |

**Finance gate.** Every cost-rate route is available only to a caller
the policy admits to the `finance` area (`access=admin` or
`finance=cost_rates`; with authentication off, the deployment's write
posture); anyone else gets `404`, and no rate or cost appears in any
other response.

### 9.2e Service REST API — people lifecycle, processes, documents, announcements

| Method | Path | Purpose / status codes |
|---|---|---|
| GET | `/onboarding/templates` | Built-in and custom checklists by kind |
| PUT / DELETE | `/onboarding/templates/{kind}` | Custom template; `kind` is `person_join`, `person_leave`, `plan_intake` or `plan_closeout`, else `422` |
| POST / GET | `/plans/{pid}/onboarding/cases` | Open / list cases; a person kind needs a `person:`/`worker:` `subject_ref`, a plan kind must not have one (`422`); one **open** case per plan + kind + subject (partial unique index, migration 48) |
| GET | `/onboarding/cases/{pid}` · PUT `…/items/{item}` · POST `…/complete` · `…/cancel` | Read; resolve an item (`done` / `skipped`); close the case |
| POST / GET | `/plans/{pid}/access-records` | Append-only `granted` / `revoked` record; records access, does not change an identity provider |
| GET | `/plans/{pid}/closeout` | What stands between the plan and being closed |
| POST | `/plans/{pid}/handover/preview` · `…/commit` | List, then apply, the unfinished tasks, live bookings, RACI roles and open risks a person holds (handed to another, or released); preview writes nothing; commit needs `If-Match: <handover_etag>`, a changed holding is `409`; needs the manage-people permission |
| POST / GET | `/processes` · GET/PUT/DELETE `/processes/{pid}` | Named step lists (`approve`, `todo`, `notify`); each edit appends an immutable version; invalid steps `422` |
| POST | `/processes/{pid}/start` | Execution pinned to the current version on a subject (`subject_kind` + `subject_pid` together, else `422`) |
| GET | `/process-executions` · `/process-executions/{pid}` | List (paged in the database) / one with its append-only log |
| POST | `/process-executions/{pid}/steps/{index}/complete` · `…/cancel` · `…/resume` | Complete a `todo` step; cancel; reconcile with the approval being waited on. A rejected approval ends the run `rejected` and skips, and logs, every later step |
| POST / GET | `/documents` · GET `/documents/{pid}` · GET `…/in-force` | Controlled documents; the version currently `effective` |
| POST | `/documents/{pid}/versions` | Next draft (one in-flight version per document, else `409`) |
| GET / PUT | `/documents/{pid}/versions/{version}` | Read; edit a **draft** only (a later edit is the next draft) |
| POST | `…/versions/{version}/submit` · `…/make-effective` | Submit for review (a PL-1 approval on the version; `approver_ref` must be a person/worker URN, `422`); make effective only if that approval was approved, superseding the version it replaces; otherwise `409`/`403`/`422` |
| GET / POST / PUT / DELETE | `/announcements` · `/announcements/{pid}` | Anyone reads the live feed; publish / edit / retract need the `comms` area (`access=admin` or `comms=publish`), else `403`; `?include=all` (publishers) also lists scheduled and expired; severity `info`/`important`/`urgent` |

### 9.2f Bulk jobs — progress and cancel

On top of §9.6 the job routes include `POST /api/plans/bulk-jobs/{id}/cancel`,
`GET /api/plans/bulk-jobs/{id}/errors`, `POST /api/plans/bulk-jobs/sweep`,
`GET /api/plans/export/{id}/download` and the duplicate review queue
(`GET /api/plans/review-queue`, `POST …/{pid}/confirm`, `…/{pid}/reject`).
A job reports `progress` (0–100, never decreasing; trigger in
migration 45), `cancel_requested` and `cancelled`. Cancel is
cooperative: a **queued** job is cancelled at once; a **running** job
ends `cancelled` after the row it is on, keeping the rows it finished;
cancelling a finished or already cancelled job is a no-op; only the
requester or an admin may cancel (anyone else gets the `404` an unknown
id gets). Import answers `202 {job_id}`; formats are `jsonl` or `csv`
(others `422`; `parquet` is **not** accepted, unlike the shared
contract); export `masking_profile` is `masked` or `full` (else `422`).

### 9.2g Cross-cutting API behaviour

- **Paging** (`controllers::paging::paging_mw`). Every `GET` that
  answers `200` with a **bare JSON array** pages the same way:
  `?limit=` (default and ceiling **500**; a larger value is clamped),
  `?offset=` (above **10 000** is `400`), a non-numeric value is `400`
  (`invalid_paging`), and the response carries `X-Total-Count`,
  `X-Limit`, `X-Offset` with the body still a bare array. Handlers that
  page in the database (processes, executions, documents) set
  `X-Total-Count` themselves and are left alone. Objects, streams and
  files pass through untouched.
- **`GET /api/plans/{pid}/tasks/{t_pid}?include=`** (`task_get.rs`).
  Without `include` the answer is the bare task; with it, `{task,
  included, omitted}`. Allowed names: `children`, `relations`,
  `transitions`, `raci`, `time_entries`; an unknown name is `422`
  listing the allowed ones. Included sets are capped and reported in
  `omitted`.
- **Private-plan guard** (`plan_access::privacy_mw`). A request
  addressed to `/api/plans/{pid}` or anything beneath it for a private
  plan the caller is not a member of (and not `access=admin`) is `404`.
  The `pid` segment is **percent-decoded before it is parsed**, so
  `%7B<uuid>%7D` cannot bypass the guard while the handler still serves
  the plan. Plan lists, search and duplicate checks omit private plans
  for non-members; every cross-plan aggregate loads through one shared
  `HiddenScope`, so a private plan contributes to no total, list,
  ranking or count outside its members. Admin-only bulk reads (board
  pack, board investments and trends, auditor trail, evidence pack) are
  `403` to everyone else.
- **Status vocabulary** used by the newer groups: `422` validation (with
  the problems joined), `409` stale `If-Match` or a state conflict,
  `403` policy denial, `404` unknown **or not visible** (private plan,
  finance data, another user's job), `202` accepted async job.
- **Self-service undo is a POST.** `DELETE` is admin-only under the
  default policy, so a person's undo of their own action has a POST
  twin (`…/delete`, `/api/watchers/unwatch`, `/api/reactions/remove`,
  `/api/me/feed-token/revoke`, `/api/me/notification-prefs/reset`).

**Not built (verified: no route, no table).** The `goals` sub-resource
(§9.2) and the `issues` sub-resource (§9.2) have **no** controller and
no `issues` migration; `goals[]` is reachable only through the plan
payload. Tasks have `POST`/`GET /api/plans/{pid}/tasks`,
`PUT`/`PATCH`/`DELETE …/tasks/{t_pid}`.

### 9.3 Front-end routes

The front-end serves the plans collection at `/plans`. Rows marked
*(planned)* below predate the UI work; the authoritative route list is
the UI subproject's `src/routes/` tree and its spec (this table was not
re-verified against it: **unverified**).

| Route | Purpose | Endpoints consumed |
|---|---|---|
| `/plans` | List plans (may filter by parent) | `GET /api/plans` |
| `/plans/new` | Create form (optional `kind` label) | `POST /api/plans` |
| `/plans/[pid]` | Detail + delete + check-duplicates + sub-resource tabs (rolls up its children) | `GET`, `DELETE /api/plans/{pid}`; `POST …/check-duplicates` |
| `/plans/[pid]/edit` | Edit form | `GET`, `PUT /api/plans/{pid}` |
| `/plans/[pid]/goals` · `/tasks` · `/issues` | Sub-resource workspaces | the §9.2 sub-resource endpoints |
| `/plans/[pid]/timeline` · `/[pid]/burndown` | Derived views | `GET …/timeline`, `GET …/burndown` |
| `/prioritization` | Smart Score queue + per-component explanation | `GET /api/prioritization`, `…/smart-score` |
| `/lifecycle` | Bird's-eye phase funnel | `GET /api/lifecycle` |
| `/reviews` | Delegate, respond, submit a verdict, consensus | the §9.2a review endpoints |
| `/automations` | Rules, deadline queue + sweep, run log | the §9.2a automation endpoints |
| `/plans/[pid]/okr` | Objectives, key results, check-ins, derived scores *(planned, FR-27)* | the §9.2b OKR endpoints |
| `/plans/[pid]/sprints` | Sprints + the four ceremonies *(planned, FR-29)* | the §9.2b sprint endpoints |
| `/plans/[pid]/effort` | Time entry + roll-ups *(planned, FR-28)* | the §9.2b effort endpoints |
| `/plans/[pid]/distribution` | Flow Distribution mix *(planned, FR-31)* | `GET …/flow-distribution` |
| `/workflows` | Workflow configuration *(planned, FR-26)* | the §9.2b workflow endpoints |
| `/plans/[pid]/value` | Realized gains + business-case performance *(planned, FR-33)* | `GET …/value-realization` |
| `/performance` | The six-dimension metric board *(planned, FR-34)* | `GET /api/performance` |

### 9.4 Library API (matcher)

The matcher's SemVer surface is its `lib.rs` re-exports: `Plan`,
`PlanKind`, `Goal`, `PlanIdentifier`, `IdentifierScheme`,
`PlanStatus`, `GoalStatus`, `PlanRelationship`, `RelationKind`,
`MatchingEngine`, `MatchConfig`, `MatchResult`, `MatchBreakdown`,
`Confidence`, `Error`, `Result`. Entry point:

```rust
let engine = MatchingEngine::new(MatchConfig::default());
let result = engine.match_plans(&a, &b);
// matching is kind-agnostic — the optional `kind` label does not gate (§5.5)
// also: engine.rank(&query, &candidates), engine.find_matches(…)
```

The sub-resource types (`Task`, `Issue`) are **not** in the matcher
crate — they have no match role and are owned by the service crate
(§5.6, §5.7).

### 9.5 Plan links (typed) and cross-service links

**Built: typed plan links (LT-8), not cross-service edges.** The
`/api/plans/{pid}/links` routes are the typed external-link store
(`plan_links`: `kind` in `docs`/`repo`/`ticket`/`design`/`chat`/`other`,
`url`, `label`; one live row per plan + url). The service stores a link
and returns it; it never fetches or renders one.

| Method | Path | Purpose |
|---|---|---|
| POST | `/api/plans/{pid}/links` | Add a typed link |
| GET | `/api/plans/{pid}/links` | List the plan's links |
| DELETE | `/api/plans/{pid}/links/{lpid}` | Soft-delete (POST twin `…/{lpid}/delete`) |

**Not built: the cross-service write-side (T-7).** There is no
`entity_links` entity, table or route and no `linked` / `unlinked`
events; the design in
`agents/share/cross-service-linking.md` §4
(`{kind, to_ref, role?, valid_from?, valid_to?}` edges to any index
entity, optimistic writes, never a match signal) remains a target. The
aggregator and graph-query API are out of scope (§2.3).

### 9.6 Bulk import / export

The async, job-based bulk contract is fixed family-wide in
[bulk import/export](bulk-import-export.md)
(execution model on `bg_pg`, the five endpoints, JSONL/CSV/Parquet
codecs, upsert-by-stable-key + dedupe-to-review, the per-row error
report, and export masking + audit). This section declares only the
**portfolio-specific** bits; the shared doc is the source of truth for
everything else.

The five endpoints (shared doc §4) mount under the plans collection:

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/api/plans/import` | `202 {job_id}` — body: `format`, `dedupe_mode`, `dry_run`; file upload |
| `GET` | `/api/plans/import/{id}` | Job status + counts + `errors_url` + `review_url` |
| `POST` | `/api/plans/export` | `202 {job_id}` — body: `format`, `filter`, `fields`, `include_soft_deleted`, `masking_profile` |
| `GET` | `/api/plans/export/{id}` | Job status + `download_url` |
| `GET` | `/api/plans/bulk-jobs` | List (filter by `kind`/`status`); `GET .../{id}` for one |

Each job runs on the one plans collection. **Stable key(s) for upsert**
(shared doc §6, §10): a row upserts in place when it carries either:

- a **deterministic scheme-scoped identifier** — the same
  `identifiers` the matcher short-circuits on (R-0, §6.3), keyed by
  `(scheme, value)`: `Uri`, `Uuid`, `JiraProjectKey`, `AsanaGid`,
  `TrelloBoardId`, `MsProjectId`, `GitHubProjectId`, `LinearId`
  (these are exactly the external-PM-tool ids a migration carries); or
- the **owner-scoped code** keyed by `(owner_org_id, code)` — never
  globally unique, so it upserts **only within the same
  `owner_org_id`** (the cross-owner-uniqueness invariant, §5.8); the
  matcher likewise never short-circuits across owners; or
- the record **`pid`** (the plan UUID) when present in the row.

A row with neither runs the normal duplicate detection
(`check-duplicates` path) routing likely duplicates to the
review queue with `provenance = import` (matching the
cross-service-linking provenance vocabulary). Bulk import covers the
**thin plan record**; sub-resource bulk import is a roadmap
extension of the same job (§15).

**CSV column set + flattening** (shared doc §5). CSV is the operator /
spreadsheet format and is lossy for deep nesting — steer
fidelity-sensitive loads to **JSONL** (the lossless reference). Flat
columns:

- **scalar** (one column each): `pid`, `kind`, `name`, `code`,
  `owner_org_id`, `owner_org_name`, `lead_ref`, `parent_ref`,
  `status`, `start_date`, `target_date`, `in_language`, `active`;
- **arrays / arrays-of-objects** → a single **JSON-encoded cell** each:
  `alternate_names`, `goals` (`{title, description?, target_date?,
  status?}`), `keywords`, `tags`, `identifiers` (`{scheme, value}`),
  `same_as`, and `relationships` (`{relation, plan_id}`).

There is no single-nested-object scalar field, so no dotted columns;
every repeated / nested field is a JSON-encoded cell. JSONL
round-trips the whole thin `Plan` payload (including the optional
`kind` and `goals[]`) losslessly.

**Export sensitivity** (shared doc §8). Plans are **operational /
business data**; the **personal-data** angle is the `EntityRef`s to
people — `lead_ref`, task assignees (§12). Export defaults to
**masked** per the shared contract; full / unmasked output (which
reveals the people references) requires a `masking_profile` selecting
elevated authorization and must never reveal more than the caller could
read one record at a time. `include_soft_deleted` defaults `false` and
is gated. **Every export is audited** (actor, filter, format, row
count, masking profile, timestamp — written even for a zero-row
export).

### 9.7 Route inventory by controller

Mounted routes, generated from each controller's `routes()` (every
path is under `/api`; `plans.rs` paths are relative to `/api/plans`,
and its list `GET /` / `POST /` pair is shown as `/`). Routes whose
verb helper is not a plain `get/post/put/delete/patch` call, if any,
are **unverified** here. Treat `src/openapi.rs` as the documented
subset.

```
announcements.rs: GET /announcements; POST /announcements; PUT /announcements/{pid}; DELETE /announcements/{pid}
approvals.rs: POST /approvals; GET /approvals; POST /approvals/{pid}/decide; POST /approvals/{pid}/withdraw
assignments.rs: PUT /plans/{pid}/tasks/{t_pid}/assignment; PUT /plans/{pid}/tasks/{t_pid}/contour; GET /plans/{pid}/work-profile; GET /work-profile
attachments.rs: POST /attachments; GET /attachments; GET /attachments/search; GET /attachments/{pid}/download; DELETE /attachments/{pid}; POST /attachments/{pid}/delete
automation.rs: POST /automations; GET /automations; GET /automations/runs; POST /automations/{pid}/enable; POST /automations/{pid}/disable; DELETE /automations/{pid}; POST /scheduled-actions; GET /scheduled-actions; POST /scheduled-actions/sweep; POST /automations/milestones/sweep; DELETE /scheduled-actions/{pid}
availability.rs: POST /availability; GET /availability; DELETE /availability/{pid}; POST /availability/{pid}/delete
bulk.rs: POST /plans/import; GET /plans/import/{id}; POST /plans/export; GET /plans/export/{id}; GET /plans/export/{id}/download; GET /plans/bulk-jobs; POST /plans/bulk-jobs/sweep; GET /plans/bulk-jobs/{id}/errors; POST /plans/bulk-jobs/{id}/cancel; GET /plans/review-queue; POST /plans/review-queue/{pid}/confirm; POST /plans/review-queue/{pid}/reject
calendars.rs: GET /plans/{pid}/calendar; PUT /plans/{pid}/calendar; GET /me/calendar; PUT /me/calendar
canvas.rs: GET /canvas-templates; POST /canvases; GET /canvases; GET /canvases/{pid}; DELETE /canvases/{pid}; POST /canvases/{pid}/items; PUT /canvases/{pid}/items/{item_pid}; DELETE /canvases/{pid}/items/{item_pid}; POST /canvases/{pid}/items/{item_pid}/promote
ceremony.rs: POST /sprints/{pid}/ceremonies; GET /sprints/{pid}/ceremonies; POST /sprints/{pid}/commit; GET /sprints/{pid}/commitment
collaboration.rs: POST /reviews; GET /reviews; GET /reviews/consensus; POST /reviews/{pid}/respond; POST /reviews/{pid}/submit; DELETE /reviews/{pid}; POST /plans/{pid}/tasks/{t_pid}/assign; GET /assignees/workload; GET /notifications; POST /notifications/{pid}/read
compliance.rs: GET /records/verify; GET /audit/verify
components.rs: POST /plans/{pid}/components; GET /plans/{pid}/components; GET /plans/{pid}/components/summary; PUT /plans/{pid}/components/{c_pid}; DELETE /plans/{pid}/components/{c_pid}
controlled_documents.rs: POST /documents; GET /documents; GET /documents/{pid}; GET /documents/{pid}/in-force; POST /documents/{pid}/versions; GET /documents/{pid}/versions/{version}; PUT /documents/{pid}/versions/{version}; POST /documents/{pid}/versions/{version}/submit; POST /documents/{pid}/versions/{version}/make-effective
controls.rs: POST /plans/{pid}/controls; POST /plans/{pid}/controls/register-standard; GET /plans/{pid}/controls; GET /plans/{pid}/controls/coverage; GET /controls/coverage; DELETE /controls/{pid}; POST /controls/{pid}/readings; GET /controls/{pid}/readings; POST /readings/{pid}/actions; POST /actions/{pid}/convert
cost_rates.rs: POST /cost-rate-tables; GET /cost-rate-tables; POST /cost-rate-tables/{pid}/rates; GET /cost-rates/lookup; POST /plans/{pid}/tasks/{t_pid}/costs; GET /plans/{pid}/tasks/{t_pid}/costs; DELETE /plans/{pid}/tasks/{t_pid}/costs/{c_pid}; GET /plans/{pid}/cost-plan; POST /plans/{pid}/labour-costs/record; GET /plans/{pid}/labour-costs
cost_risk.rs: GET /plans/{pid}/cost-risk; GET /cost-risk; GET /plans/{pid}/joint-risk; PUT /plans/{pid}/budget-lines/{l_pid}/estimate; PUT /plans/{pid}/risks/{r_pid}/cost-impact
cpm.rs: GET /plans/{pid}/cpm; PUT /plans/{pid}/tasks/{t_pid}/schedule
custom_fields.rs: POST /custom-fields; GET /custom-fields; PUT /custom-fields/values; GET /custom-fields/values; GET /custom-fields/search; DELETE /custom-fields/{pid}
discussion.rs: POST /comments; GET /comments; PUT /comments/{pid}; DELETE /comments/{pid}; POST /comments/{pid}/delete; POST /reactions/remove; GET /comments/{pid}/history; POST /reactions; GET /reactions; DELETE /reactions
distribution.rs: GET /plans/{pid}/flow-distribution
docs.rs: GET /api-docs/openapi.json; GET /swagger-ui
earned_value.rs: GET /plans/{pid}/earned-value; GET /plans/{pid}/earned-value/forecast; GET /plans/{pid}/earned-value/series; GET /plans/{pid}/earned-value/rollup; POST /plans/{pid}/earned-value/snapshot; PUT /plans/{pid}/earned-value-settings; PUT /plans/{pid}/tasks/{t_pid}/percent-complete
effort.rs: POST /plans/{pid}/time-entries; GET /plans/{pid}/time-entries; GET /plans/{pid}/effort; POST /working-time; POST /non-working; GET /capacity/utilization
engineering.rs: POST /plans/{pid}/tasks; GET /plans/{pid}/tasks; PUT /plans/{pid}/tasks/{t_pid}; PATCH /plans/{pid}/tasks/{t_pid}; DELETE /plans/{pid}/tasks/{t_pid}; POST /plans/{pid}/sprints; GET /plans/{pid}/sprints; GET /plans/{pid}/burndown; GET /plans/{pid}/velocity; POST /plans/{pid}/sprints/{s_pid}/notes; GET /plans/{pid}/sprints/{s_pid}/notes; POST /plans/{pid}/sprints/{s_pid}/notes/{n_pid}/convert; GET /plans/{pid}/standup; POST /devops/events; GET /devops/metrics; GET /devops/releases; GET /engineering/blocked; GET /engineering/moscow; GET /engineering/delivery-links; GET /engineering/milestone-calendar
feeds.rs: POST /me/feed-token; DELETE /me/feed-token; POST /me/feed-token/revoke; GET /feeds/calendar.ics; GET /feeds/me.atom
financials.rs: POST /plans/{pid}/budget-baselines; GET /plans/{pid}/budget-baselines; GET /plans/{pid}/financials/forecast; GET /financials/forecast
governance.rs: POST /proposals; GET /proposals; GET /proposals/forecast; GET /proposals/{pid}; PUT /proposals/{pid}; POST /proposals/{pid}/submit; POST /proposals/{pid}/review; POST /proposals/{pid}/approve; POST /proposals/{pid}/reject; POST /proposals/{pid}/promote; GET /proposals/{pid}/duplicates; POST /plans/{pid}/gate-reviews; GET /plans/{pid}/gate-reviews; POST /plans/{pid}/risks; GET /plans/{pid}/risks; PUT /plans/{pid}/risks/{risk_pid}; POST /plans/{pid}/risks/{risk_pid}/escalate; POST /plans/{pid}/budget-lines; GET /plans/{pid}/budget-lines; POST /plans/{pid}/budget-lines/{line_pid}/actual; POST /plans/{pid}/budget-lines/{line_pid}/release; GET /plans/{pid}/governance
guests.rs: POST /plans/{pid}/guests; GET /plans/{pid}/guests; POST /plans/{pid}/guests/{grant}/revoke
hammock.rs: POST /plans/{pid}/tasks/{t_pid}/hammock; GET /plans/{pid}/hammocks; POST /plans/{pid}/hammocks/{h_pid}/links; DELETE /plans/{pid}/hammocks/{h_pid}/links/{member}
handover.rs: POST /plans/{pid}/handover/preview; POST /plans/{pid}/handover/commit
insights.rs: GET /executive/health; GET /executive/decisions; GET /executive/benefits; GET /executive/alignment; GET /financials/variance; GET /financials/exposure; GET /technology/dependency-risk; GET /technology/radar; GET /technology/debt; GET /technology/flow
kpis.rs: POST /kpis; GET /kpis; GET /kpis/{pid}; PUT /kpis/{pid}; DELETE /kpis/{pid}; POST /kpis/{pid}/readings; GET /kpis/{pid}/readings
leveling.rs: GET /plans/{pid}/leveling; POST /plans/{pid}/leveling/apply
live.rs: GET /plans/{pid}/live
metrics.rs: GET /metrics.prom
milestones.rs: GET /milestones; GET /milestones/{pid}; PUT /plans/{pid}/milestones/{m_pid}/tasks; POST /plans/{pid}/milestones/{m_pid}/reopen
modules.rs: GET /plans/{pid}/modules; PUT /plans/{pid}/modules
mspdi_import.rs: POST /plans/{pid}/import/mspdi
tracker_import.rs: POST /plans/{pid}/import/jira, /plans/{pid}/import/asana (CSV body, dry_run default true, idempotent by external_ref)
my_work.rs: GET /me/tasks
notification_prefs.rs: PUT /me/notification-prefs; GET /me/notification-prefs; DELETE /me/notification-prefs; POST /me/notification-prefs/reset; GET /me/inbox
okr.rs: POST /objectives/{pid}/key-results; GET /objectives/{pid}/key-results; POST /key-results/{pid}/check-ins; GET /key-results/{pid}/check-ins; GET /plans/{pid}/okr; GET /okrs; PUT /objectives/{pid}; DELETE /objectives/{pid}; PUT /key-results/{pid}; DELETE /key-results/{pid}
onboarding.rs: GET /onboarding/templates; PUT /onboarding/templates/{kind}; DELETE /onboarding/templates/{kind}; POST /plans/{pid}/onboarding/cases; GET /plans/{pid}/onboarding/cases; GET /onboarding/cases/{pid}; PUT /onboarding/cases/{pid}/items/{item}; POST /onboarding/cases/{pid}/complete; POST /onboarding/cases/{pid}/cancel; POST /plans/{pid}/access-records; GET /plans/{pid}/access-records; GET /plans/{pid}/closeout
oversight.rs: GET /board/pack; GET /board/investments; POST /board/snapshots; GET /board/trends; GET /auditor/trail; GET /auditor/findings; GET /auditor/evidence-pack; GET /compliance/register; GET /compliance/findings; GET /risk/heatmap; GET /security/register; GET /regulator/extract
pdf.rs: GET /board/pack.pdf; GET /plans/{pid}/summary.pdf
phase.rs: PUT /plans/{pid}/phase; GET /plans/{pid}/phase-history
plan_access.rs: GET /plans/{pid}/access; PUT /plans/{pid}/access
plan_links.rs: POST /plans/{pid}/links; GET /plans/{pid}/links; DELETE /plans/{pid}/links/{lpid}; POST /plans/{pid}/links/{lpid}/delete
plan_templates.rs: POST /plans/{pid}/save-as-template; POST /plans/from-template/{tpl}; GET /plan-templates; GET /plan-templates/{pid}; DELETE /plan-templates/{pid}; POST /plan-templates/{pid}/delete
planner.rs: POST /me/planned-slots; DELETE /me/planned-slots/{pid}; POST /me/planned-slots/{pid}/delete; GET /me/plan
plans.rs: POST /; GET /; GET /search; POST /match; POST /check-duplicates; POST /merge; GET /merges/recent; GET /whoami; GET /audit/recent; GET /events/recent; GET /{pid}; PUT /{pid}; DELETE /{pid}; GET /{pid}/masked; GET /{pid}/export; GET /{pid}/audit
portfolio_resources.rs: GET /resource-plan; GET /resource-plan/hours; GET /resource-roles; PUT /resource-roles; POST /plans/{pid}/allocations/{a_pid}/resolve
prioritization.rs: GET /prioritization; GET /lifecycle; GET /plans/{pid}/smart-score; GET /plans/{pid}/lifecycle
processes.rs: POST /processes; GET /processes; GET /processes/{pid}; PUT /processes/{pid}; DELETE /processes/{pid}; POST /processes/{pid}/start; GET /process-executions; GET /process-executions/{pid}; POST /process-executions/{pid}/steps/{index}/complete; POST /process-executions/{pid}/cancel; POST /process-executions/{pid}/resume
raci.rs: GET /plans/{pid}/raci; PUT /plans/{pid}/tasks/{t_pid}/raci
report_schedules.rs: POST /reports/{pid}/schedule; GET /report-schedules; POST /report-schedules/sweep; DELETE /report-schedules/{pid}; POST /report-schedules/{pid}/delete; GET /report-schedules/{pid}/latest
sandbox.rs: POST /plans/{pid}/sandbox/preview; POST /plans/{pid}/sandbox/commit
saved_views.rs: POST /saved-views; GET /saved-views; DELETE /saved-views/{pid}; POST /saved-views/{pid}/delete
schedule_baselines.rs: POST /plans/{pid}/schedule-baselines; GET /plans/{pid}/schedule-baselines; GET /plans/{pid}/schedule-variance
schedule_health.rs: GET /plans/{pid}/schedule-health
schedule_risk.rs: GET /plans/{pid}/schedule-risk; PUT /plans/{pid}/tasks/{t_pid}/estimate; PUT /plans/{pid}/risks/{r_pid}/schedule-impact
sinks.rs: POST /me/notification-sinks; GET /me/notification-sinks; DELETE /me/notification-sinks/{pid}; POST /me/notification-sinks/{pid}/delete; GET /me/notification-sinks/{pid}/deliveries; POST /notification-sinks/dispatch
strategy.rs: POST /ideas; GET /ideas; POST /ideas/{pid}/vote; POST /ideas/{pid}/dismiss; POST /ideas/{pid}/convert; POST /scenarios; GET /scenarios; POST /scenarios/generate; GET /scenarios/compare; GET /scenarios/{pid}/evaluate; POST /scenarios/{pid}/commit; POST /scenarios/{pid}/rollback; POST /objectives; GET /objectives; GET /objectives/{pid}/alignment; POST /plans/{pid}/objectives; GET /plans/{pid}/objectives; POST /plans/{pid}/benefits; GET /plans/{pid}/benefits; POST /plans/{pid}/benefits/{b_pid}/realize
task_bulk.rs: POST /tasks/bulk; POST /tasks/copy; POST /tasks/move
task_get.rs: GET /plans/{pid}/tasks/{t_pid}
task_relations.rs: POST /plans/{pid}/tasks/{t_pid}/relations; GET /plans/{pid}/tasks/{t_pid}/relations; DELETE /plans/{pid}/tasks/{t_pid}/relations/{r_pid}
task_tree.rs: GET /plans/{pid}/tasks/{t_pid}/rollup
tba.rs: GET /flow-classes; GET /plans/{pid}/tasks/{t_pid}/transitions; GET /plans/{pid}/tasks/{t_pid}/time-analysis; GET /plans/{pid}/time-analysis; GET /plans/{pid}/constraints; GET /plans/{pid}/aging-wip; GET /plans/{pid}/flow; GET /plans/{pid}/cumulative-flow; GET /plans/{pid}/forecast; GET /plans/{pid}/rollup
timesheets.rs: GET /timesheets; POST /timesheets/submit; POST /timesheets/decide; GET /time-reports
tpc.rs: POST /plans/{pid}/tpc; GET /plans/{pid}/tpc; GET /plans/{pid}/tpc/report; GET /tpc
value.rs: POST /plans/{pid}/business-case; POST /plans/{pid}/value-points; POST /plans/{pid}/adoption; POST /plans/{pid}/satisfaction; GET /plans/{pid}/value-realization; GET /plans/{pid}/performance
visibility.rs: POST /dependencies; GET /dependencies; DELETE /dependencies/{pid}; GET /plans/{pid}/schedule; POST /plans/{pid}/milestones; GET /plans/{pid}/milestones; PUT /plans/{pid}/milestones/{m_pid}; POST /plans/{pid}/milestones/{m_pid}/complete; POST /plans/{pid}/allocations; GET /plans/{pid}/allocations; POST /plans/{pid}/allocations/{a_pid}/commit; DELETE /plans/{pid}/allocations/{a_pid}; GET /plans/{pid}/skill-gap; GET /capacity; POST /reports; GET /reports; DELETE /reports/{pid}; GET /reports/{pid}/run; GET /at-a-glance
watchers.rs: POST /watchers; GET /watchers; DELETE /watchers; POST /watchers/unwatch; GET /me/watching
workflow.rs: POST /workflows; GET /workflows; DELETE /workflows/{pid}; GET /plans/{pid}/workflow
```

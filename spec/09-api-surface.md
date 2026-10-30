## 9. API Surface

Endpoint detail: [`agents/restful.md`](../agents/restful.md); source:
`project-portfolio-management-service-with-rust/src/controllers/`. The
service is implemented (§14); the route inventory in §9.7 is generated
from the controllers' `routes()` and is the check against drift. The
tables in §9.2–§9.2c keep their original FR-oriented shape and carry
verified corrections where the mounted route differs. The documented
OpenAPI surface is `src/openapi.rs` + `src/openapi_more.rs`.

### 9.0 API versioning

The API is versioned with a request header, never with the URL.

- Every path stays under `/api`. A path never carries a version segment: **do not** use `/api/v1`, `/api/v2` or
  any similar prefix, now or for a future version.
- A client selects the representation version with the `Accepts-version` request header (for example
  `Accepts-version: 1.0`; a bare major such as `1` stands for that major's current minor). The header name is
  case-insensitive on the wire.
- With no header the current version (`1.0`) is served. A version the service does not support is `406 Not
  Acceptable`. The version served is stamped on the response in the same header.
- A new version is added to `SUPPORTED_API_VERSIONS` in `src/version.rs`, not to the URL space, so existing links,
  bookmarks and the route inventory (§9.7) stay valid across versions.

Source: `src/version.rs`.

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
| GET/POST | `/api/plans/{pid}/tasks` · PUT/DELETE `…/tasks/{id}` | Tasks (board / list; status, assignee, estimate, due, `deadline` (set by an import or a paste), nesting) |
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
| GET | `/api/plans/{pid}/smart-score` · `/api/prioritization` | Smart Score + ranked queue (with breakdown); `?model=wsjf\|rice` ranks by those models (NC-6) |
| PUT / GET | `/api/plans/{pid}/scoring-inputs` | The estimated WSJF and RICE inputs (all optional; audited) |
| GET | `/api/plans/{pid}/score?model=wsjf\|rice` | One plan's WSJF or RICE score with formula, components and the inputs still missing |
| POST / GET | `/api/plans/{pid}/lessons` · POST `…/{lpid}/withdraw` | The lessons-learned register: record a lesson at a gate or closure; list; withdraw (audited) |
| GET | `/api/proposals/{pid}/lessons` | Lessons of past plans similar to a proposal, with the match score and breakdown |
| GET | `/api/exceptions/digest` | The weekly portfolio-exceptions digest as the caller sees it: tolerances, threshold exceptions, escalations |
| PUT / GET | `/api/plans/{pid}/status-cadence` | How often a plan expects a status update, and where it stands against it |
| POST / GET | `/api/plans/{pid}/status-updates` · GET `/api/plans/{pid}/status` | Post an on-track / at-risk / off-track update; the history; the latest with its due date |
| GET | `/api/plans/{pid}/references` | The meetings and lessons that point at one record of the plan (`kind`, `pid`); `404` when the plan or the record is not visible |
| GET | `/api/status-metrics` | How each number a status-update claim may cite is counted |
| GET / POST | `/api/status-updates/due` · `/api/status-updates/remind` | The plans whose update is due; send the once-per-due-date reminders |
| GET | `/api/plan-templates` | Stored templates, then the five starters (`starter: true`; BR-13); a starter cannot be deleted or revised |
| POST / GET | `/api/plan-templates/{pid}/revise` · `/api/plan-templates/{pid}/plans` | Revise a template from a plan tree; the plans made from it and the changes each has waiting |
| GET / POST | `/api/plans/{pid}/template-diff` · `/api/plans/{pid}/template-sync` | What a template changed since the plan was brought up to date; apply named changes, refusing locally edited tasks unless overwritten |
| POST / GET | `/api/plans/{pid}/meetings` · GET `…/{mid}` | Plan a meeting with an agenda; list; read one with its linked records named |
| POST / GET | `/api/plans/{pid}/meetings/{mid}/held` · `…/cancel` · GET `…/pdf` | Record the minutes and outcomes once; cancel; print as a deterministic PDF |
| POST | `/api/plans/{pid}/meetings/{mid}/task-proposals/preview` · `…/commit` | Judge proposed tasks against the minutes (nothing written); create them once, linked to their outcomes |
| POST | `/api/import/mspdi-batch` | Up to ten MSPDI files in one job: plans created or reused, the resource pool listed once, custom fields mapped; dry run by default |
| any `POST` | header `Idempotency-Key` | A retried write returns the first answer (`Idempotent-Replayed: true`); a different request under the key is `409` (NFR-7) |
| GET / POST | `/api/webhook-deliveries` · `/api/webhook-deliveries/{id}/redeliver` | The outbound webhook delivery log; send one logged delivery again (administrators only) (BR-5) |
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
| GET | `/ways-of-working/{pid}/versions` · `/governance-frameworks/{pid}/versions` | A framework's versions, newest first, with `plans_following` each (CF-4). A `PUT` of the record appends a version; a plan's link pins one (`PUT /plans/{pid}/…` with `{framework_pid, version?}`) and the plan keeps it until it is moved; the in-force answer carries `version`, `latest_version`, `behind`. Changing an organisation-wide record needs `access=admin` or `framework=edit` (`403`) |
| GET / PUT | `/plans/{pid}/escalations/{epid}/acknowledgement` · `…/resolution` | The acknowledgement and the resolution of an escalation as resources (CF-4). `PUT` needs `access=admin` or `escalation=acknowledge` / `escalation=resolve` (else `403`), is idempotent, and records the history event and the audit entry; a resolution is never overwritten (`409`). `GET` is `404` until there is one |
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

- **The "today" time zone** (`GET`/`PUT`/`DELETE /api/settings/today-time-zone`). The zone that
  defines "today" for portfolio-wide views and defaults; UTC until an administrator sets one (see §7.x).
  `GET` is open to anyone signed in and answers `time_zone`, `source` (`default` or `admin`),
  `updated_by`, `updated_at`, `now_utc` and the `today` it gives. `PUT { time_zone }` and `DELETE`
  are `access=admin` only (`403`); a name that is not an IANA zone is `422`; both are audited.
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
- **Link tokens.** A token carried in a URL (today only the per-user feed
  token) is a high-entropy secret, **shown once, stored as a SHA-256 hash**,
  revocable and **expiring**: a feed token is refused 365 days after it was
  issued, and issuing a new one **revokes the caller's earlier live tokens**
  (one live feed token per person, so a leaked old URL stops working on
  rotation). A feed token is deliberately **reusable**: a calendar app
  fetches the same URL on a schedule, so single use would end the
  subscription. Any future one-shot link (an invite, a share or export link)
  must be single-use: it is refused once redeemed or expired, decided by
  `link_token::check` and redeemed with one conditional update so two
  simultaneous requests cannot both win. The hash is looked up by equality
  in the database; a timing difference on a lookup of the hash of a 244-bit
  secret reveals nothing about the secret.

**Sub-resources.** Goals: `GET`/`POST /api/plans/{pid}/goals`, `PUT`/`DELETE …/goals/{index}` (the plan's
`goals[]`, by index). Issues: `GET`/`POST /api/plans/{pid}/issues`, `GET`/`PUT`/`DELETE …/issues/{issue_pid}`,
`POST …/issues/from-risk/{risk_pid}` and `…/from-task/{task_pid}`, and `GET /api/issues` across plans (IR-2, IR-3).
Tasks: `POST`/`GET /api/plans/{pid}/tasks`, `PUT`/`PATCH`/`DELETE …/tasks/{t_pid}`.

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
| POST / GET | `/api/plans/{pid}/tasks/{tpid}/links` | Add or list a **task's** links (same body and rules; a task's link is not in the plan's list) |
| GET | `/api/plans/{pid}/task-link-counts` | How many live links each task of the plan has (`counts`: task pid to number; tasks with none absent) |
| DELETE | `/api/plans/{pid}/tasks/{tpid}/links/{lpid}` | Soft-delete a task's link (POST twin `…/{lpid}/delete`) |

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

The mounted routes, one `METHOD /path` line each, are
[`project-portfolio-management-service-with-rust/api-routes.txt`](../project-portfolio-management-service-with-rust/api-routes.txt),
written from the router by a unit test in `src/openapi.rs` (CQ-9), so it cannot drift: it is the inventory. Every
route in it is also in `openapi.json` except `GET /api-docs/openapi.json` and `GET /swagger-ui`. The per-controller
list that stood here was written by hand and had fallen 52 controllers behind (among them the issues register, the
capacity heatmap and the workforce-service bridge, entity spec §8.5a); it was removed on 2026-10-30.

### 9.7a The workforce-service bridge (FB-1 to FB-6)

| Method | Path | Bridge |
| --- | --- | --- |
| `GET` | `/api/skill-pools` | each pool's `peer_pool_ref` (FB-1) |
| `PUT` | `/api/skill-pools/{pid}/peer-link` | link or clear (`{peer_pool_ref: <uuid> \| null}`; `409` already linked, `422` not a lower-case UUID); clearing withdraws what was sent (FB-1, FB-5) |
| `GET` | `/api/capacity-heatmap` | a linked pool's supply from WPM, `supply_source: "wpm"`, `peer_supply` reading (FB-2) |
| `PUT` | `/api/capacity-commitments` | a demand commitment on a linked pool sends the programme's total to WPM (`peer_demand_send`, FB-3, FB-5) |
| `GET` | `/api/capacity-commitments` | the latest send per demand commitment (`peer_demand_send`) |
| `GET` | `/api/plans/{pid}/people-impact` | WPM's people impact for the plan, `?months=1..36` (FB-4) |

The rules are §8.5a; the service detail is the service spec's §9.10a; the screens (FB-6) are the UI spec's.

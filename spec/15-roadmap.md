## 15. Roadmap

Roadmap items become §13 tasks when they are concrete enough to size
and accept.

### PPM feature roadmap (proposed 2026-07-18, with the rename)

The subproject rename (`portfolio` → `project-portfolio-management`)
repositions this trio as a **project portfolio management product**,
not just a matchable registry of plan identities. The catalogue
below maps the standard PPM capability set onto what the trio already
ships (registry CRUD + kind-agnostic matching/merge over the one
`plans` collection; goals / tasks / issues sub-resources (**the `goals` and `issues` stores are specified but not built**, §14.2); timeline + burndown
views; audit + events; bulk import/export; cross-service people/org
links; ABAC). Design doctrine for every item: new capabilities are
**operational sub-resources** (tables keyed `parent_pid`, like
goals/tasks/issues), every mutation audits + emits,
record-level ABAC attributes gate the governance actions, and nothing
here ever becomes a **matcher signal** (the §8 partition rule —
sameness evidence and operational state stay separate).

**Pillar 1 — Strategic alignment & pipeline management**

- **PPM-1 Work intake.** ✅ *Delivered 2026-07-18 (service T-PPM-A).* A `proposals` pipeline (draft → submitted →
  in-review → approved / rejected → promoted): demand records with
  sponsor, strategic rationale, rough sizing, and requested funding.
  Promotion mints the real plan with `provenance = intake` and a
  link back to the proposal. Bonus the registry heritage makes cheap:
  run the **matcher at intake** so a duplicate demand is flagged
  against both existing proposals and live plans before it is
  funded.
- **PPM-2 Idea management.** ✅ *Delivered 2026-07-18 (service T-PPM-C).* Lightweight `ideas` (title, pitch, tags,
  votes) convertible to proposals in one action; the roadmap's
  posts/comments sub-resources attach here first so brainstorming
  threads live with the idea. Ideas are deliberately schema-thin —
  the funnel is idea → proposal → plan, each step adding rigour.
- **PPM-3 Phase-gate approvals.** ✅ *Delivered 2026-07-18 (service T-PPM-A).* A per-plan `stage` plus
  first-class `gate_reviews` (gate name, decision, conditions,
  approver `worker:` ref, date). Writes on a gate-locked plan are
  policy-refusable via a `resource.stage` ABAC attribute ("deny write
  past gate-3 unless `access=admin`"), making governance a policy
  statement, not code. Approvals are audited governance events.
- **PPM-4 Scenario planning.** ✅ *Delivered 2026-07-18 (service T-PPM-C).* `scenarios`: named candidate portfolios
  (a set of plans / proposals + constraint knobs — budget cap,
  capacity cap, must-include). A pure-core evaluator scores each
  scenario against PPM-8 capacity and PPM-10 budget data (total cost,
  demand vs capacity, alignment score) so what-if comparison is
  arithmetic over live data, patient-flow-at-a-glance style.
  Committing a scenario stamps the chosen items' funding state.
- **PPM-5 OKR / objective alignment.** ✅ *Delivered 2026-07-18 (service T-PPM-C).* An org-level `objectives`
  registry (OKRs) and a plan → objective mapping with weights;
  alignment rolls up per parent plan (up the containment tree), so
  "how much of the portfolio serves objective X" is a query. Plan
  `goals` already exist; this adds the strategic layer above them.

**Pillar 2 — Execution & visibility**

- **PPM-6 Roadmap & dependency views.** ✅ *Delivered 2026-07-18 (service T-PPM-B).* Timeline/Gantt exists;
  add cross-plan `dependencies` (finish-start edges between
  tasks/plans, with lag) + milestone records, and derive the
  critical path + slipping-dependency warnings in the timeline view.
- **PPM-7 Portfolio dashboards.** ✅ *Delivered 2026-07-18 (service T-PPM-B).* A portfolio-level at-a-glance
  endpoint (the patient-flow pattern, ETag-conditional): per-parent-plan
  rollups of RAG health, schedule variance (timeframe vs today), open
  risks (and, once FR-14 lands, issues) by severity, budget variance (PPM-10), capacity
  hot-spots (PPM-8), gate-stage distribution — plus site-tile
  headlines for the executive view.
- **PPM-8 Resource capacity planning.** ✅ *Delivered 2026-07-18 (service T-PPM-B).* `allocations`: person/worker
  `EntityRef` + plan (or task) + percentage + timeframe. Rollup
  per person against a configurable weekly capacity ⇒ over-allocation
  detection, a capacity heatmap, and reassignment suggestions
  (largest-slack first). People stay references — no demographics
  copied (family doctrine), and allocations are never matcher
  signals.
- **PPM-9 Custom reporting.** ✅ *Delivered 2026-07-18 (service T-PPM-B; synchronous runs — scheduled/bulk-artifact runs await the family bulk machinery).* Saved report definitions (filter +
  field projection + grouping) executed through the existing bulk
  export machinery (JSONL/CSV; Parquet when the family lands it), a
  KPI/OKR snapshot endpoint for stakeholder decks, and scheduled
  report generation as a `bg_pg` job with the artifact-store TTL
  posture from `agents/share/bulk-import-export.md`.

**Pillar 3 — Value realization & governance**

- **PPM-10 Budget tracking.** ✅ *Delivered 2026-07-18 (service T-PPM-A).* `budget_lines` per plan
  (capex/opex, currency amount via `bigdecimal`, period) + recorded
  actuals (manual or bulk-imported from finance) ⇒ projected-vs-actual
  variance in the dashboards, rolled up the parent-plan containment
  hierarchy. Centralizes financial oversight without becoming a
  ledger — actuals are imported facts, not double-entry bookkeeping.
- **PPM-11 Benefits tracking.** ✅ *Delivered 2026-07-18 (service T-PPM-C).* `benefits` per plan: category,
  metric, baseline, target, expected realization date — then recorded
  actuals over time ⇒ realized-vs-projected value and simple ROI
  (with PPM-10 costs). Benefits are reviewed at phase gates (PPM-3),
  closing the loop between the funding case and delivery.
- **PPM-12 Risk management.** ✅ *Delivered 2026-07-18 (service T-PPM-A).* `risks` (probability × impact scoring,
  owner, mitigation, review date). *Correction (2026-10-09):* the "existing
  issues" this item assumed do not exist: there is no `issues` migration,
  entity, controller or `/issues` route, and `src/openapi.rs` documents none
  (FR-14, §14.2), so escalating a risk into an issue is **not built**; only the
  action-to-task conversion exists (`controllers/controls.rs`).
  Portfolio-level rollup (exposure by severity, overdue reviews) in
  PPM-7; cross-project dependency risks ride the PPM-6 edges.

**Suggested phasing** — A (governance core, ✅ service side delivered 2026-07-18): PPM-1, PPM-3, PPM-12,
PPM-10 — intake, gates, risks, budgets give the portfolio office its
control loop. B (visibility, ✅ service side delivered 2026-07-18): PPM-6, PPM-7, PPM-8, PPM-9 — dashboards
and capacity make the control loop observable. C (strategy, ✅ service side delivered 2026-07-18): PPM-2,
PPM-4, PPM-5, PPM-11 — scenarios, OKRs, and benefits need A + B's
data to be meaningful. **The whole catalogue is delivered service-side, and the
front-end views over it landed 2026-07-18** (dashboard, intake +
idea boards, per-item governance panel, portfolio schedule,
scenarios, objectives, capacity, reports — see the front-end
CHANGELOG). Locale catalogues for the new views landed 2026-07-18 (98 `ppm.*`
keys across all 13 locales, parity-tested) — the PPM catalogue is
now complete across service, UI, and i18n. Each item lands as spec §13 tasks (three-part
rule) when accepted.

### Longer arc (pre-rename roadmap)

The items below predate the PPM repositioning and remain valid.

- **Collaboration sub-resources.** The plan-family lineage carried
  posts, comments, and membership sub-resources; this entity ships
  goals / tasks / issues only (§2.3). Add posts / comments (Markdown
  update threads on a plan / task / issue) and members (a user's
  membership of a plan with a role) when collaboration becomes a
  requirement — each a new sub-resource table keyed by `parent_pid`, with membership-scoped write authorization.
- **Family parity — match / search / merge.** Beyond the MVP
  baseline, reach the mature-sibling shape
  (`agents/share/match-search-merge.md` (shared family guidance, not in this repository)):
  Tantivy full-text + fuzzy search over the JSONB payload of the
  `plans` collection, search-blocked duplicate candidates (replacing the
  in-memory scan, OQ-2), batch deduplicate scan, and a front-end merge
  action.
- **Auditability — durable event bus.** The MVP ships an in-memory
  `PlanEvent` stream (T-5); replace it with the durable event bus
  (`agents/share/event-bus.md`) so
  peer registries, analytics, and the cross-service link aggregator
  can subscribe across replicas. Plan events are high-volume
  (every task write), so batched outbox emission matters.
- **Cross-service link aggregator.** Stand up (or join) the
  `link-graph-service`
  (`agents/share/cross-service-linking.md` §4.3)
  so a plan's `EntityRef`s and `entity_links` become a traversable
  graph (a plan's people → their orgs, related plans across
  departments). The portfolio trio ships only the write-side (T-7); the
  aggregator is a separate service.
- **Sub-resource bulk + linking.** Extend bulk import/export and the
  cross-service link write-side to the sub-resources (bulk-load tasks
  from a source PM tool; link a task to a `case` or a `thing`).
- **Security.** Blanket auth enforcement on `/api/*` (PASETO v4 public
  token / cookie session per
  `agents/share/authentication-sessions.md`,
  superseding the RS256-JWT model) with **ABAC** authorization over
  the token's `attrs` claim (per
  `agents/share/authorization-attributes.md`;
  delivered, supersedes the earlier role-based sketch) — deployments
  express read/write/destructive tiers and any read-integrator vs
  plan-operator split as policy attributes, not fixed roles;
  rate limiting.
- **PM-tool sync.** Two-way sync with Jira / Asana / MS Project /
  Linear / GitHub Projects keyed on the deterministic external-id
  identifiers (R-0 schemes), so a registered project stays in step with
  its source-tool twin without becoming a full PM replacement (§8.7).
- **Localization.** Operator UI in the ten-locale set of
  [locales](locales.md) (landed); multilingual plan names via `alternate_names` +
  `in_language`; cross-language duplicate linkage through deterministic
  identifiers / `same_as`.
- **Scale-out and operations.** Multi-replica deployment, PostgreSQL
  replication, JSONB GIN + sub-resource indexing (OQ-3), OTLP
  observability pipeline, Prometheus metrics, backup / DR runbooks,
  container hardening.
- **gRPC.** Tonic stub once a high-throughput consumer exists. (The
  OpenAPI 3 doc + Swagger UI for the REST surface ship under T-9.)

### Status of the longer arc (reconciled 2026-10-06)

Read against the code and the repo-root [`tasks.md`](../tasks.md); the
bullets above are kept as written and annotated here rather than
rewritten.

- **Delivered since they were written:** the durable outbox and relay
  (`event_outbox`, `src/relay.rs`; §14.1); bulk import / export with a
  duplicate review queue (T-8, `controllers/bulk.rs`); Tantivy search
  (`src/search/`); blanket auth enforcement behind
  `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` (**on by default** since HD-3)
  with ABAC; comments and reactions (LT-4) and watchers (RM-3); an MS
  Project (MSPDI) **import** (PJ-10, `POST /api/plans/{pid}/import/mspdi` in
  `controllers/mspdi_import.rs`) and the Jira and Asana import codecs (T-28n,
  `POST /api/plans/{pid}/import/jira` and `…/asana` in
  `controllers/tracker_import.rs`); approval processes (PL-2,
  `controllers/processes.rs`) and controlled documents (PL-6,
  `controllers/controlled_documents.rs`, front-end `/documents`).
- **Still open:** the cross-service link write-side (T-7, no
  `entity_links`), a members sub-resource, gRPC, two-way PM-tool sync, and
  screens for much of the escalation and critical-chain work (§14.2 lists the
  gaps). MS Project (MSPDI) export, the `goals` sub-resource and the issues
  register are built since this line was written (2026-10-20; see
  [implementation status](14-implementation-status.md)). A link aggregator stays
  out of this trio's scope.
- **The 2026-10 catalogue is in `tasks.md`, not here:** Leantime
  (LT-n), Redmine (RM-n), Huly (PL-n), the scheduling and cost canon
  (SC-n, KO-n, MS-n, CR-n) and ProjectLibre (PJ-n) items. Every
  item in those series is ticked at this revision (counts per prefix are
  generated in [§14.0](14-implementation-status.md#140-generated-facts)),
  including hammocks (PJ-7), the
  what-if sandbox (PJ-9), cross-plan links (PJ-13), approval processes
  (PL-2) and controlled documents (PL-6). Items recorded there under
  "Not taken" are deliberate non-goals, not roadmap.

### Planned, not built

Open in [`tasks.md`](../tasks.md) and absent from the code (no route,
migration or page yet); each group is planned work, not delivered capability.

- **Monthly director pack (DP-1 … DP-12):** the issued, versioned pack per audience, delivery confidence, exceptions and the 48-hour red rule, gate decisions, funding sources, buffer fever chart, validated benefits, capacity heatmap by skill pool, rendering and the monthly rhythm.
- **Escalation-driven capabilities (EX-1 … EX-14):** register completeness, delivery-model typology, partner organizations and sign-off, deployment-date change control, go / no-go, triage thresholds, start control, the outcome scorecard, key-person register and portfolio ledger.
- **Critical chain buffers (CC-1 … CC-6):** the critical chain, aggressive and safe estimates, project / feeding / resource buffers, buffer management, drum and pipelining, and the front-end.

### Release and operations

**In progress** (the HD series in [`tasks.md`](../tasks.md), HD-1 … HD-9).
Landed: the licence files (HD-1, `LICENSE/`); login enforcement **on by
default**, refused off in production (HD-3); security headers, a strict CSP and
an explicit CORS allow-list on the API and the front-end (HD-4); per-caller
rate limiting (HD-5); and documents that match the code (HD-9: the generated
block in [§14.0](14-implementation-status.md#140-generated-facts), the link
checker `scripts/check-links.py` with `lychee.toml`, and this reconciliation).
Built but not yet proven on GitHub: CI at the repository root (HD-2) and the
release pipeline with pinnable version tags (HD-8). Open: a backup plan with
stated RPO and RTO (HD-6) and a data protection impact assessment (HD-7).
Until those close, treat a deployment as not yet fully production-hardened.

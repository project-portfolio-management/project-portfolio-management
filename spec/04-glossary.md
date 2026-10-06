## 4. Glossary

Entity-level terms. Per-subproject vocabularies: service
[spec §4](../project-portfolio-management-api-with-rust/spec/index.md), matcher
[spec §3](../project-portfolio-management-matcher-with-rust/spec/index.md), front-end
[spec §4](../project-portfolio-management-ui-with-svelte/spec/index.md).

| Term | Meaning |
|---|---|
| **Entity** | One domain concept (here: Portfolio) delivered as a trio of subprojects in one directory |
| **Trio** | The three subprojects: service crate, matcher crate, front-end project |
| **Entity-level spec** | This document set — source of truth for the cross-subproject contract and the canonical §5 domain model |
| **Crate spec** | A subproject's own `spec/` — source of truth for that subproject's internals |
| **Plan** | The canonical matchable type — the unit of work an organisation funds and reports on; it may carry an optional `kind` label (Portfolio, Project, Product, Program, Practice, Process, Purpose, Pathway, or Proposal) |
| **PlanKind** | The **optional** descriptive label on a `Plan`: `Portfolio`, `Project`, `Product`, `Program`, `Practice`, `Process`, `Purpose`, `Pathway`, `Proposal`. Used for description / display / grouping — **not** a discriminator, does **not** gate matching or fix a collection |
| **Collection** | The single REST collection `/api/plans` and its `plans` service table; every plan lives here regardless of its `kind` label |
| **Parent / child plan** | Any plan may **contain** any other plan via `parent_ref` (a recursive tree); the container is the **parent**, the contained plan a **child** |
| **Thin matchable record** | The `Plan` matcher type — the identity-bearing fields that feed deduplication; distinct from the operational sub-resources |
| **Operational sub-resource** | High-volume child data of a plan (task, issue; and goal) — lives in its own service table, keyed by `parent_pid` |
| **Goal** | A plan objective `{ title, description?, target_date?, status? }`; the one sub-resource **also** carried in the matcher payload (goal titles are a match signal) |
| **Task** | A unit of work under a plan `{ title, assignee_ref, status, estimate, remaining, due_date, … }`; operational only |
| **Issue** | A bug / risk / blocker / question / improvement raised under a plan; operational only |
| **Derived view** | A computed projection — **timeline** (Gantt from goal milestones + task date ranges) or **burndown** (remaining-vs-estimate over time) |
| **PlanStatus** | `Proposed`, `Active`, `OnHold`, `Completed`, `Cancelled`, `Custom(String)` — informational-only, not a match signal |
| **Owner-scoped code** | `code` (e.g. `PROJ-2026`) — unique only within the owning `owner_org_id`; never matched across owners |
| **Parent plan** | `parent_ref` — the parent plan's `pid`; an exact supporting match signal (§5.5); absent for a root plan |
| **EntityRef** | An opaque URN naming a record in another service: `<entity_type>:<id>` (e.g. `person:0c4f…`, `organization:9a2f…`) — the one shared cross-service contract ([cross-service-linking.md §3](../../agents/share/cross-service-linking.md)) |
| **Deterministic identifier** | Globally unique identifier (URI, UUID, Jira project key, Asana GID, Trello board id, MS Project id, GitHub project id, Linear id); a shared value pins the match score to 1.0 |
| **pid** | The public UUID of a stored plan record (route param; distinct from the row's internal `id`) |
| **`data`** | The `plans.data` JSONB column holding the full thin `Plan` payload verbatim |
| **DTO contract** | The API body for the thin record **is** `project_portfolio_management_matcher::Plan` — no separate service model, no adapter |
| **Match** | A comparison between two plans yielding a 0.00–1.00 score, `Confidence` band, `is_match`, and per-component breakdown |
| **Containment cycle** | A `parent_ref` that points a plan at itself or at one of its descendants; rejected `422` (§6.1) |
| **Check-duplicates** | `POST …/check-duplicates` — match a query against stored plans, return ranked hits above threshold |
| **Tag** | A short operator-applied label for grouping / workflow (e.g. `priority-1`, `q3-review`); a **supporting** match signal via set Jaccard; distinct from keywords |
| **Keyword** | A descriptive / discovery term about *what the plan is*; distinct from operator tags |
| **Relationship** | A typed plan-to-plan link `{ relation, plan_id }` (ParentOf / DependsOn / Supersedes / SimilarTo / RelatedTo / …) — within-entity; a supporting match signal |
| **Cross-service link** | An `entity_links` edge from a plan / goal / task / issue to **any** index entity ([cross-service-linking.md](../../agents/share/cross-service-linking.md)) — **never** a match signal |
| **Soft delete** | Retention with `deleted_at` set; never `DELETE FROM` |
| **Stable key** | The bulk-import upsert key — a deterministic external PM id, the owner-scoped `(owner_org_id, code)`, or `pid` ([bulk-import-export.md §6, §10](../../agents/share/bulk-import-export.md)) |
| **SSO** | Single sign-on via the [authentication entity](../../authentication/): magic-link, cookie session + PASETO v4 public token ([authentication-sessions.md](../../agents/share/authentication-sessions.md), supersedes RS256-JWT) |
| **Drift policy** | Front-ends keep per-project copies of types/client/forms; no shared package (repo decision 2026-06-02) |
| **Hammock** | A task whose span is derived from the tasks (or hammocks) it is linked to; carries no duration, work, constraint or dependency and never drives float (§6.4d FR-42) |
| **What-if sandbox** | `sandbox/preview` applies schedule edits to an in-memory copy and writes nothing; `sandbox/commit` applies them for real under `If-Match` (FR-43) |
| **Scheduling rule** | Which of units, work and duration stays fixed when an assignment is edited: `fixed_units`, `fixed_duration`, `fixed_work` (FR-40) |
| **Work contour** | How a task's work is spread over its days: flat, front-loaded, back-loaded, bell, turtle, custom (FR-40) |
| **Work profile** | A task, plan or portfolio's work as day / week / month buckets, derived on read (FR-40) |
| **Cost rate** | Minor units per hour for a person or role, effective from a date, append-only; a missing rate is `null`, never zero (FR-41) |
| **Finance gate** | The policy admission (`access=admin` or `finance=cost_rates`) required for rates and costs; others get `404` (FR-41) |
| **Cross-plan task link** | A dependency between tasks of two plans (`plan_dependencies`); float is computed across it, and a plan the caller cannot read appears only as a restricted edge (FR-44) |
| **Private plan** | A plan visible only to named members; `404` to everyone else, and excluded from every aggregate through `HiddenScope` (FR-45) |
| **HiddenScope** | The one shared query of the private plans (and their tasks) a caller may not see, applied to every cross-plan aggregate (FR-45) |
| **Paging headers** | `X-Total-Count`, `X-Limit`, `X-Offset` on every bare-array list; `limit` default and ceiling 500, `offset` ceiling 10 000 (FR-46) |
| **`include`** | Query parameter on `GET …/tasks/{t_pid}` embedding sub-resources (`children`, `relations`, `transitions`, `raci`, `time_entries`) (FR-46) |
| **Cooperative cancel** | A bulk job asked to stop ends `cancelled` after the row it is on, keeping rows already processed (FR-47) |
| **Process** | A versioned, ordered list of up to 20 steps (`approve`, `todo`, `notify`); each edit is a new immutable version (FR-48) |
| **Process execution** | One run of a process, pinned to a version, with its state and an append-only log in the database; a rejection skips every later step (FR-48) |
| **Controlled document** | A document whose versions move draft, in review, approved, effective, superseded; effectiveness requires an approved approval request (FR-49) |
| **In force** | The one `effective` version of a controlled document (`GET /api/documents/{pid}/in-force`) |
| **Onboarding case** | A checklist instance of kind `person_join`, `person_leave`, `plan_intake` or `plan_closeout`, with due dates from an effective date (FR-50) |
| **Access record** | An append-only grant or revocation of access for a person on a plan; the service records access and cannot change an identity provider (FR-50) |
| **Close-out readiness** | `GET …/closeout`: the findings (open tasks, risks, live bookings, unrevoked access, open cases) standing between a plan and closing (FR-50) |
| **Handover** | Preview then commit (under `If-Match`) of moving or releasing a person's live tasks, bookings, RACI roles and open risks on a plan (FR-50) |
| **Announcement** | A dated, severity-tagged feed item (`info`, `important`, `urgent`) published by admins or `comms=publish` holders (FR-51) |


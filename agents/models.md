# Domain Model Reference — Portfolio Entity

The portfolio entity is two things at once:

1. A **matchable identity** — the `Plan` header, with an optional
   descriptive `kind` label (Portfolio / Project / Product / Program /
   Practice / Process / Purpose / Pathway / Proposal). The matcher
   crate's `Plan` is the API DTO, the persisted JSONB payload, and the
   matching input. One shape, end to end — no adapter (mirrors
   care-pathway).
2. A **project-management tool** — a `Plan` owns operational records
   in their own tables (tasks, sprints, risks, milestones, allocations,
   approvals, documents, …) and exposes derived read views (burndown,
   critical path, flow, cost, …).

Normative definitions: entity spec [§5](../spec/05-domain-model.md)
and matcher [spec §6](../project-portfolio-management-matcher-with-rust/spec/index.md).

## The optional kind label + recursive containment

Every record is a `Plan`. **Portfolio**, **Project**, **Product**,
**Program**, **Practice**, **Process**, **Purpose**, **Pathway**, and
**Proposal** are optional values of the descriptive `kind` label — used
for display and grouping only. `kind` is **not required**, **not a
discriminator**, does **not** fix a collection, and does **not** gate
matching ([matching.md](matching.md) — matching is kind-agnostic).

Containment is **recursive**: any plan may contain any other plan via
`parent_ref`, forming a tree (`parent_ref` replaces the former
`portfolio_ref`). The service rejects a `parent_ref` that points a plan
at itself or at one of its descendants (a containment cycle) with
HTTP `422`.

## `Plan` (matchable identity)

**File:** [`project-portfolio-management-matcher-with-rust/src/plan.rs`](../project-portfolio-management-matcher-with-rust/src/plan.rs)

| Field | Type | Description |
|---|---|---|
| kind | Option\<PlanKind\> | **Optional.** Portfolio / Project / Product / Program / Practice / Process / Purpose / Pathway / Proposal — a descriptive display / grouping label; not a discriminator, not a gate |
| name | String | Plan title (required; service rejects blank) |
| alternate_names | Vec\<String\> | Aliases, former titles, codenames |
| code | Option\<String\> | Owner-scoped code (e.g. `PROJ-2026`) |
| owner_org_id | Option\<String\> | EntityRef `organization:<id>` — sponsoring / owning org; scopes `code` |
| owner_org_name | Option\<String\> | Owning-org display name (informational-only) |
| lead_ref | Option\<String\> | EntityRef `person:<id>` \| `worker:<id>` — the lead. **Not scored** |
| parent_ref | Option\<String\> | Parent plan `pid` (recursive containment); absent for a root plan |
| phase | Option\<PlanPhase\> | Initiating / Planning / Executing / Controlling / Closing — data only, **never scored** |
| status | Option\<PlanStatus\> | Lifecycle status — informational-only, NOT a match signal |
| goals | Vec\<Goal\> | In the payload; goal **titles** feed the `Goals` component |
| start_date | Option\<Date\> | Planned / actual start (feeds `Timeframe`) |
| target_date | Option\<Date\> | Planned completion / due date (feeds `Timeframe`) |
| keywords | Vec\<String\> | Descriptive / discovery terms (what it *is*) |
| tags | Vec\<String\> | Operator labels for grouping / workflow |
| identifiers | Vec\<PlanIdentifier\> | Typed external identifiers |
| same_as | Vec\<String\> | Canonical URLs (schema.org `sameAs`) |
| in_language | Option\<String\> | ISO 639-1 language code |
| relationships | Vec\<PlanRelationship\> | Typed links to other plans |

Construct with `Plan::new(name)` — `kind` defaults to `None`.

## Supporting types (matching surface)

| Type | Variants / shape |
|---|---|
| `PlanKind` | `Portfolio`, `Project`, `Product`, `Program`, `Practice`, `Process`, `Purpose`, `Pathway`, `Proposal` — **closed set** (no `Custom`, not `#[non_exhaustive]`); an optional descriptive label |
| `PlanStatus` | `Proposed`, `Active`, `OnHold`, `Completed`, `Cancelled`, `Custom(String)` — informational-only |
| `PlanRelationship` | `{ relation: RelationKind, plan_id: String }` (`plan_id` = `pid` or URI) |
| `RelationKind` | `ParentOf` / `ChildOf` (inverses), `DependsOn` / `BlockedBy` (inverses), `Supersedes` / `SupersededBy` (inverses), `SimilarTo` (symmetric), `RelatedTo` (symmetric), `Custom(String)` |
| `PlanIdentifier` | `{ scheme: IdentifierScheme, value: String }` |
| `IdentifierScheme` | Deterministic: `Uri`, `Uuid`, `JiraProjectKey`, `AsanaGid`, `TrelloBoardId`, `MsProjectId`, `GitHubProjectId`, `LinearId` · Owner-scoped: `Code`, `LocalId` · `Custom(String)` |

Deterministic schemes pin a match to 1.0 on a shared value;
owner-scoped schemes never do (see [matching.md](matching.md)).

## Operational records

These hang off a `Plan` by `plan_pid`, each in its own table, and are
**not** part of the matching surface (goal *titles* bridge in through
`data.goals[]`). `src/models/_entities/` holds the SeaORM entities (about
100); the route map is in the service
[AGENTS.md](../project-portfolio-management-api-with-rust/AGENTS.md).
Representative groups:

| Group | Tables (examples) |
|---|---|
| Delivery | `tasks`, `task_transitions`, `task_relations`, `sprints`, `sprint_notes`, `sprint_commitments`, `ceremonies`, `hammock_links` |
| Schedule | `plan_dependencies` (also cross-plan), `milestones`, `schedule_baselines`, `allocations`, `planned_slots` |
| Governance | `proposals`, `gate_reviews`, `risks`, `budget_lines`, `controls`, `approval_requests` |
| Cost / value | `cost_rate_tables`, `cost_rates`, `task_costs`, `time_entries`, `benefits`, `kpis` |
| Process / documents | `processes`, `process_versions`, `process_executions`, `process_execution_logs`, `controlled_documents`, `document_versions` (append-only / immutable past draft, enforced by triggers) |
| People | `onboarding_cases`, `onboarding_items`, `access_records`, `guest_grants`, `plan_access_settings` |
| Platform | `audit_logs`, `merge_records`, `event_outbox`, `bulk_jobs`, `webhook_deliveries`, `announcements`, `plan_links` |

`TaskStatus` values are `todo`, `in_progress`, `in_review`, `done`,
`blocked` (`src/engineering.rs`), unless a custom workflow is in force.
`*_ref` fields are opaque references (person `pid`, worker `pid`,
organization `pid`, auth user id) the service stores verbatim.

> Goals and issues have **no** sub-resource routes and there is no
> `/timeline`; they are deferred (spec §13).

## Derived views (read-only)

Computed on read from stored rows; never stored as canonical state:
`burndown` (done-at transitions only), `velocity`, critical path
(`/cpm`, floats across cross-plan links), hammock spans, work profile,
cost plan, time-based flow metrics, Smart Score, lifecycle.

## Service persistence model

**Files:**
[`src/models/`](../project-portfolio-management-api-with-rust/src/models/),
[`migration/src/`](../project-portfolio-management-api-with-rust/migration/src/)

One core table `plans` — `{id` (PK)`, pid` (public UUID)`, name`
(denormalized from `data.name`)`, kind` (nullable)`, parent_pid`
(nullable, the recursive containment link)`, data` (JSONB `Plan`)`,
active, deleted_at` (soft delete)`, phase}`. Model helpers: `create`,
`find_by_pid`, `list_paged`, `to_plan()`, `update_data`, `soft_delete`.
Migrations live in `migration/src/` (latest `…_000048_onboarding_one_open`);
`create_table` pluralizes names, and trigger functions use
`CREATE OR REPLACE FUNCTION`.

## Wire DTOs (service controller)

**File:** [`src/controllers/`](../project-portfolio-management-api-with-rust/src/controllers/)

| Type | Shape | Used by |
|---|---|---|
| `PlanRef` | `{ pid, name }` | create / update / list responses |
| `MatchRequest` | `{ query: Plan, candidates: [Plan] }` | `POST …/match` |
| `ScoredRef` | `{ pid, name, score, confidence, is_match }` | `POST …/check-duplicates` |

## Front-end TypeScript mirror

**File:** [`src/lib/api/types.ts`](../project-portfolio-management-ui-with-svelte/src/lib/api/types.ts)
— `Plan`, `PlanKind`, `PlanStatus`, `Goal`, `GoalStatus`,
`Task`, `TaskStatus`, `PlanRelationship`, `RelationKind`,
`IdentifierScheme`, `PlanRef`, `ScoredRef`. Hand-mirrored; MUST be
updated in the same change cycle as any matcher-type change (entity
spec §18).

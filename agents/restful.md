# RESTful API Reference — Portfolio Entity

The service is loco.rs and returns **raw JSON** — no
`{success, data, error}` envelope (unlike the pre-loco person service).
Source:
[`src/controllers/`](../project-portfolio-management-api-with-rust/src/controllers/).
Base URL in development: `http://localhost:5150`.

The service exposes **one matchable collection** — `/api/plans`. Every
record is a `Plan` with an optional descriptive `kind` label; there is no
per-kind collection and no `{collection}` path segment. Matching is
**kind-agnostic** — any two plans may match.

## Endpoints

### Health (loco built-ins)

| Method | Path | Description |
|---|---|---|
| GET | `/_health` | Health check |
| GET | `/_ping` | Liveness ping |

### Plan CRUD

| Method | Path | Body | Returns |
|---|---|---|---|
| POST | `/api/plans` | `Plan` | `{pid, name}`; `409` on a real-time duplicate |
| GET | `/api/plans` | — | `[{pid, name}]` — paged (default 100) |
| GET | `/api/plans/search?q=` | — | Tantivy full-text; `?fuzzy=`, `?phonetic=`, `?kind=` (an opt-in narrowing; never applied to duplicate detection); blank `q` is `400` |
| GET | `/api/plans/{pid}` | — | stored `Plan` (a `mask` obligation returns the redacted view) |
| PUT | `/api/plans/{pid}` | `Plan` | `{pid, name}` |
| DELETE | `/api/plans/{pid}` | — | `{}` (soft delete) |
| GET | `/api/plans/{pid}/masked` · `/export` | — | always-redacted view · audited GDPR export |

`kind` is an optional descriptive label taken verbatim from the body. A
`parent_ref` that targets the plan itself or a descendant is `422`. A
**private** plan is `404` to non-members on every `/api/plans/{pid}/…` path
and contributes to no aggregate view.

### Matching and merge

| Method | Path | Body | Returns |
|---|---|---|---|
| POST | `/api/plans/match` | `{query, candidates}` | ranked `[(index, MatchResult)]` |
| POST | `/api/plans/check-duplicates` | `Plan` | `[{pid, name, score, confidence, is_match}]`, score-descending |
| POST | `/api/plans/merge` | `{main_pid, duplicate_pid, reason?}` | `{main_pid, duplicate_pid, main}`; `422` equal pids, `404` unknown |
| GET | `/api/plans/merges/recent` | — | recent `merge_records` |

Matching is kind-agnostic. `POST /api/plans/deduplicate` (batch scan) is
**not wired** (spec §13).

### Paging (every list)

A `GET` that answers `200` with a bare JSON array pages the same way:
`?limit=` (default and ceiling 500), `?offset=` (above 10 000 is `400`),
and headers `X-Total-Count` / `X-Limit` / `X-Offset`; the body stays the
bare array. The service's `openapi::PAGED_LISTS` census is checked by
`tests/list_paging.rs`.

### Areas beyond the registry

The service is also a project-management tool. Rather than repeat the full
route map, see the crate's
[AGENTS.md](../project-portfolio-management-api-with-rust/AGENTS.md) (one
row per area: governance, visibility, strategy, executive and oversight
views, engineering and flow, critical path with cross-plan float,
hammocks, what-if sandbox, assignments and cost rates, onboarding and
handover, approval processes, controlled documents, announcements, bulk
import/export, task bulk edit/copy/move, `GET …/tasks/{t_pid}?include=`)
and [spec §9](../project-portfolio-management-api-with-rust/spec/index.md).
Two commit endpoints need `If-Match` with the etag from their preview, and
answer `409` if stale: `POST …/sandbox/commit` and `POST …/handover/commit`.

### Typed links

`POST`/`GET /api/plans/{pid}/links` and `DELETE …/links/{lpid}` store and
return typed links on a plan; the service never fetches or renders the
target.

### Authentication

| Method | Path | Returns |
|---|---|---|
| GET | `/api/plans/whoami` | verified PASETO-token `Claims`; `401` without a valid token |

Short-lived **PASETO v4.public** tokens are verified offline against the
auth-service's published **Ed25519 key** via the embedded
`authentication-verifier` (`src/auth.rs`). The `AuthUser` extractor
requires a token; `MaybeAuthUser` is optional and feeds the audit
`actor`. Blanket `/api/*` auth enforcement is off by default
(`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`); the published-key fetch at boot
is `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL`. (Front-ends use a BFF + cookie
session, so the browser holds no token; the BFF supplies this bearer
server-side.) Credentials come from
the central authentication-service (RS256/JWKS not used).

### Audit & events

| Method | Path | Returns |
|---|---|---|
| GET | `/api/plans/audit/recent` | recent `audit_logs` rows (all plans, cap 100) |
| GET | `/api/plans/{pid}/audit` | audit trail for one plan |
| GET | `/api/plans/events/recent` | recent `PlanEvent`s from the in-memory stream |

Each write writes an `audit_logs` row and publishes an event to the
in-memory stream by default; with
`PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT=outbox` a relay delivers from
a durable outbox (Fluvio sink behind the `fluvio` feature; optional signed
webhooks).

### API documentation & metrics

| Method | Path | Returns |
|---|---|---|
| GET | `/api-docs/openapi.json` | hand-written OpenAPI 3 document (every route is pinned there) |
| GET | `/swagger-ui` | Swagger UI page (CDN assets) rendering the spec |
| GET | `/metrics.prom` | Prometheus exposition |

## HTTP status codes

| Code | Meaning |
|---|---|
| 200 | Success |
| 400 | Malformed body (loco JSON rejection); blank `q` on search; `offset` above 10 000 |
| 409 | Duplicate on create; stale `If-Match` on a commit |
| 404 | Unknown or soft-deleted `pid`; a private plan to a non-member |
| 422 | Validation failure: blank `name` on create/update; containment cycle on `parent_ref`; blank sub-resource title (family convention) |
| 500 | Internal error |

## Example

```bash
curl -s localhost:5150/api/plans \
  -H 'content-type: application/json' \
  -d '{"kind":"Project",
       "name":"Q3 Platform Migration",
       "parent_ref":"0c4f1e2a-…",
       "goals":[{"title":"Ship v2 API","status":"InProgress"}]}'
```

## Front-end consumption

**Files:**
[`src/lib/api/client.ts`](../project-portfolio-management-ui-with-svelte/src/lib/api/client.ts)
(lean fetch wrapper + `ApiError`),
[`src/lib/api/plans.ts`](../project-portfolio-management-ui-with-svelte/src/lib/api/plans.ts)
(`PlanRepository`).

The UI project owns its route list; see its [AGENTS.md](../project-portfolio-management-ui-with-svelte/AGENTS.md) and spec.

A plan detail also rolls up its child plans (recursive containment). Base
URL: `PUBLIC_API_BASE_URL` (default `http://localhost:5150`).

## Matcher library API

```rust
use project_portfolio_management_matcher::{Plan, MatchConfig, MatchingEngine};

let engine = MatchingEngine::new(MatchConfig::default());
let result = engine.match_plans(&a, &b);
// result.score: f64 in [0.0, 1.0]  (kind-agnostic — kind never gates)
// result.confidence: High | Medium | Low
// result.is_match: bool
// result.breakdown: per-component Option<f64>
```

SemVer surface = `lib.rs` re-exports: `Plan`, `PlanKind`,
`PlanStatus`, `PlanIdentifier`, `IdentifierScheme`, `Goal`,
`GoalStatus`, `PlanRelationship`, `RelationKind`, `MatchingEngine`,
`MatchConfig`, `MatchResult`, `MatchBreakdown`, `Confidence`, `Error`,
`Result`.

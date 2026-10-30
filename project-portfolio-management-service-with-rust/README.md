# Project Portfolio Management Service

A registry of **plan** records — and a project-management tool —
built on **loco.rs** and embedding the canonical
[project-portfolio-management-matcher](../project-portfolio-management-matcher-with-rust).

A *plan* is a matchable identity in one recursive collection. Its `kind`
— **Portfolio**, **Project**, **Product**, **Program**, **Practice**,
**Process**, **Purpose**, **Pathway**, or **Proposal** — is an
**optional descriptive label** (a display / grouping hint), not a
required discriminator and not a separate collection. The service has
two faces that share one record: a deduplicated, matchable identity
registry (the thin `Plan` payload) and a project workspace — each plan
*owns* operational sub-resources (tasks, sprints, goals, the issues register and more) plus derived
timeline / burndown views. Any plan may contain any other plan via a
`parent_ref` to its parent (a recursive tree); a `parent_ref` that
points a plan at itself or at one of its descendants is rejected (`422`,
containment-cycle check).

- Spec: [spec/index.md](./spec/index.md)
- Entity-wide contract: [portfolio entity spec](../spec/index.md)
- Agent guide: [AGENTS.md](./AGENTS.md)
- Sibling UI: [project-portfolio-management-ui-with-svelte](../project-portfolio-management-ui-with-svelte)

> **Status: v0.2.0, a full project-management tool.** Deferred items
> (`/plans/deduplicate`, `/timeline`) live in
> [spec §13](./spec/index.md); see [CHANGELOG.md](./CHANGELOG.md).

## API

API URLs are version-free; select the version with the `Accepts-version` header (default `1.0`).

Routes are under `/api/`. Plans live in **one** collection, `/api/plans`;
sub-resources hang off `/api/plans/{pid}/...`.

| Method | Path | Purpose |
|---|---|---|
| POST | `/api/plans` | Create (`409` on real-time duplicate) |
| GET | `/api/plans` | List |
| GET | `/api/plans/{pid}` | Fetch |
| PUT | `/api/plans/{pid}` | Update |
| DELETE | `/api/plans/{pid}` | Soft-delete |
| GET | `/api/plans/search?q=` | Tantivy full-text/fuzzy/phonetic search (`?fuzzy=`, `?phonetic=`, `?kind=`) |
| POST | `/api/plans/match` | Rank `{query, candidates}` (kind-agnostic) |
| POST | `/api/plans/check-duplicates` | Match a query vs stored plans (blocked on the Tantivy index) |
| POST | `/api/plans/deduplicate` | Batch scan → review queue — **deferred, spec §13** |
| POST | `/api/plans/merge` | Merge a duplicate into any survivor plan |
| GET | `/api/plans/merges/recent` | Merge-history records |
| GET | `/api/plans/{pid}/masked` · `/export` | Masked view (always redacted) · audited GDPR export |
| * | `/api/plans/{pid}/tasks` | Task-board CRUD + Kanban `PATCH` move (story points, WIP limits); `/goals` and `/issues` are built too (see `api-routes.txt`) |
| GET | `/api/plans/{pid}/sprints` · `/burndown` · `/velocity` · `/standup` | Sprint tooling — `/timeline` (Gantt) **deferred, spec §13** |
| POST·GET·DELETE | `/api/plans/{pid}/links` | Typed links on a plan (stored and returned, never fetched) |
| POST·GET | `/api/plans/import` · `/export` · `/bulk-jobs` · `/review-queue` | Bulk import/export jobs (202 + job id, progress, cooperative cancel) and the review queue |
| POST·GET | `/api/reviews` (+ `/consensus`, `/{pid}/respond`, `/{pid}/submit`) | Collaborative review: delegate to internal or external experts |
| POST | `/api/plans/{pid}/tasks/{t_pid}/assign` | Assign / unassign a task (`null` unassigns) |
| GET | `/api/assignees/workload` | Open work per assignee, incl. the unassigned pile |
| GET | `/api/notifications` (+ `/{pid}/read`) | In-app inbox (no email / push transport) |
| POST·GET | `/api/automations` (+ `/{pid}/enable`·`/disable`, `/runs`) | Workflow automation: rules fired by board moves |
| POST·GET | `/api/scheduled-actions` (+ `/sweep`) | Set and forget: the deadline queue |
| GET | `/api/plans/{pid}/smart-score` · `/api/prioritization` | Smart Score + the ranked queue (with the full breakdown) |
| GET | `/api/lifecycle` · `/api/plans/{pid}/lifecycle` | Bird's-eye funnel + next-phase readiness checklist |
| GET | `/api/plans/audit/recent` · `/{pid}/audit` | Audit-log query |
| GET | `/api/plans/events/recent` | In-memory event stream |
| GET | `/api/plans/whoami` | Verified PASETO-token claims (`401` without one) |
| GET | `/api-docs/openapi.json` · `/swagger-ui` | OpenAPI 3 doc + Swagger UI |
| GET | `/metrics.prom` | Prometheus metrics (root path; needs a token like every route except health, ping and the docs) |

The table above is the identity-registry core. The service is also a
full **project-management tool**; one controller per area in
`src/controllers/`, mapped route by route in [AGENTS.md](./AGENTS.md) and
in [spec §9](./spec/index.md):

- Governance, visibility, strategy, executive insights, oversight, and
  row-level integrity verification (`/api/compliance/*/verify`).
- Engineering and flow: task board, sprints, ceremonies, burndown,
  time-based analysis, OKRs, effort and utilization, Total Project Control,
  controls, value realization.
- Scheduling: critical path with float across cross-plan links, hammock
  activities, a what-if sandbox (preview writes nothing; commit needs
  `If-Match`), assignments with scheduling rules, time-phased work, cost
  rates and labour cost.
- People and process: onboarding/offboarding cases, handover, multi-step
  approval processes (versioned, append-only log), controlled documents
  (versions gated by approval; immutable past draft), announcements.
- Access: **private plans** are `404` to non-members and appear in no
  aggregate; guests are confined to their grants; per-plan modules can be
  switched off.
- Every list pages the same way: `?limit=` (default and max 500),
  `?offset=` (max 10 000), headers `X-Total-Count` / `X-Limit` /
  `X-Offset`. `GET /api/plans/{pid}/tasks/{t_pid}?include=` returns a task
  with its sub-resources in one response.

See [AGENTS.md](./AGENTS.md) and [spec §9](./spec/index.md) for the full
route contract.

The body for a plan **is** the `project_portfolio_management_matcher::Plan`
shape (optional `kind` label, name, code + owner org, `parent_ref`, goals,
dates, keywords, tags, relationships, identifiers, sameAs). **Matching is
kind-agnostic** — there is no kind gate, so any two plans may match
regardless of their (optional) labels. The high-volume operational data
(tasks, issues) lives in separate tables and is **never** fed to the
matcher (goal **titles** bridge in via `data.goals[]`).

## Quick start

> Every mounted route is listed in [`api-routes.txt`](api-routes.txt) and documented in
> [`openapi.json`](openapi.json). The bridge to the workforce service is off until configured; see
> [the WPM bridge tutorial](../operations/wpm-bridge.md).

Requires PostgreSQL.

```bash
export DATABASE_URL=postgres://loco:loco@localhost:5432/project_portfolio_management_development
cargo run -- start      # migrations auto-run in development

# Create a plan labelled as a portfolio (the umbrella kind)
curl -s localhost:5150/api/plans -H 'content-type: application/json' \
  -d '{"name":"Digital Transformation Portfolio","kind":"portfolio",
       "goals":[{"title":"Modernize core systems","target_date":"2026-12-01"}]}'

# Create a plan under that one (carries parent_ref; kind label optional)
curl -s localhost:5150/api/plans -H 'content-type: application/json' \
  -d '{"name":"EHR Migration","kind":"project","parent_ref":"<parent-pid>",
       "code":"EHR-2026"}'

# Name search across all plans
curl -s 'localhost:5150/api/plans/search?q=migration'

# Match an explicit query against candidates (no persistence; kind-agnostic)
curl -s localhost:5150/api/plans/match -H 'content-type: application/json' \
  -d '{"query":{"name":"EHR Migration"},"candidates":[{"name":"EHR Migration Project"}]}'

# Merge a duplicate into any survivor plan (the survivor is `main_pid`)
curl -s localhost:5150/api/plans/merge -H 'content-type: application/json' \
  -d '{"main_pid":"<survivor-uuid>","duplicate_pid":"<duplicate-uuid>"}'

# Authenticated request: present a short-lived PASETO v4.public token
# minted by the auth-service (front-ends use a BFF + cookie session; the
# BFF holds the session and supplies this bearer server-side).
curl -s localhost:5150/api/plans/whoami \
  -H 'authorization: Bearer <paseto-v4.public-from-authentication-service>'
```

A validation failure returns `422` with **every** problem in one body
(blank `name` plus a malformed identifier plus a bad goal title come back
together). A real-time create duplicate returns `409 Conflict` with the
candidate matches.

## Testing

```bash
cargo test --lib                       # DB-free unit tests
cargo test --test matching             # DB-free: matcher embedding + JSON round-trip
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Need Postgres (DATABASE_URL; compose.test.yaml provides one on 5432):
cargo test --test mod -- --ignored --test-threads=1      # request suites
cargo test --test private_aggregates -- --ignored        # real-token binaries,
cargo test --test list_paging -- --ignored               #   one per binary
```

DB tests are `#[ignore]`d, so a plain `cargo test` needs no database.
Tests that sign in with real PASETO tokens each get their own test binary,
because the token verifier is a process-wide `OnceLock`. Details in
[AGENTS.md](./AGENTS.md).

## Status

v0.2.0. Beyond the registry core (CRUD, Tantivy search, kind-agnostic
matching, merge, validation, audit, events, OpenAPI, Prometheus, offline
PASETO auth, field masking and GDPR export), the service ships the areas
listed above; migrations run `m20220101…` through
`m20261030_000099_peer_pool_link` (133 files). Event delivery is in-memory by
default, a durable outbox relay when
`PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT=outbox`, a Fluvio sink behind
the `fluvio` feature, and optional signed webhooks. Blanket `/api/*` auth
enforcement is **on by default** (turned off only explicitly with
`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false` or `settings.require_auth:
false`; production refuses to start that way, and so does enforcement on with
no token verifier). Security headers, an explicit CORS allow-list
(`PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS`) and per-caller rate limiting
(`PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT*`) are built in. Deferred ([spec §13](./spec/index.md)): `/plans/deduplicate`,
`/timeline`, gRPC. Credentials are issued by the
central authentication-service (offline PASETO v4.public verification).

## Licence

Licence is any of these, or contact us for custom licence options.

* [MIT](https://opensource.org/license/mit) ([SPDX: MIT](https://spdx.org/licenses/MIT.html)), text in [`../LICENSE/MIT.txt`](../LICENSE/MIT.txt)
* [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([SPDX: Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)), text in [`../LICENSE/Apache-2.0.txt`](../LICENSE/Apache-2.0.txt)
* [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html) ([SPDX: GPL-2.0-only](https://spdx.org/licenses/GPL-2.0-only.html)), text in [`../LICENSE/GPL-2.0-only.txt`](../LICENSE/GPL-2.0-only.txt)
* [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0-standalone.html) ([SPDX: GPL-3.0-only](https://spdx.org/licenses/GPL-3.0-only.html)), text in [`../LICENSE/GPL-3.0-only.txt`](../LICENSE/GPL-3.0-only.txt)
* [BSD 3-Clause License](https://opensource.org/license/bsd-3-clause) ([SPDX: BSD-3-Clause](https://spdx.org/licenses/BSD-3-Clause.html)), text in [`../LICENSE/BSD-3-Clause.txt`](../LICENSE/BSD-3-Clause.txt)

SPDX expression: `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`.
</content>

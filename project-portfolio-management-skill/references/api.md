# API map

The service's route count is in the generated block of `spec/14-implementation-status.md`. Do not copy them into a document: the complete list is
`project-portfolio-management-service-with-rust/api-routes.txt` (one `METHOD /path` per line, written from the router
by a unit test), and `openapi.json` beside it documents every route except `GET /api-docs/openapi.json` and
`GET /swagger-ui`. Read those files for a route; this page only says where things are.

## Families

- **Plans** (`/api/plans`, about 300 routes): one recursive collection; `kind` is an optional label. Search, duplicate
  check, merge, import and export, bulk jobs, and the per-plan sub-resources under `/api/plans/{pid}/…` (tasks, goals,
  issues, risks, milestones, sprints, schedule and critical path, earned value, tolerances, costs, people impact, and
  more).
- **Portfolio views**: `/api/executive`, `/api/financials`, `/api/technology`, `/api/board`, `/api/risk`,
  `/api/security`, `/api/regulator`, `/api/auditor`, `/api/at-a-glance`, `/api/portfolio-*`, `/api/director-packs`.
- **Work intake and strategy**: `/api/ideas`, `/api/proposals`, `/api/objectives`, `/api/key-results`, `/api/kpis`,
  `/api/scenarios`, `/api/prioritization`.
- **Capacity**: `/api/capacity`, `/api/capacity-heatmap`, `/api/capacity-commitments`, `/api/skill-pools`,
  `/api/resource-plan`, `/api/availability`, `/api/timesheets`.
- **The caller**: `/api/me/…` (my work, my calendar, notification preferences, watching).
- **Governance**: `/api/approvals`, `/api/processes`, `/api/documents`, `/api/governance-frameworks`,
  `/api/ways-of-working`, `/api/escalations`, `/api/change-requests`, `/api/issues`.
- **Outside `/api`**: `/_health`, `/_readiness`, `/metrics.prom` (needs a token), `/api-docs/openapi.json`,
  `/swagger-ui`.

## The bridge to the workforce service (FB-1 to FB-6)

Off until `PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL` is set (`https://`, or `http://` on loopback). Every call
to WPM forwards the caller's own bearer token. Rules: entity spec §8.5a; service spec §9.10a; walk-through:
`operations/wpm-bridge.md`.

| Method | Path | What it does |
| --- | --- | --- |
| `GET` | `/api/skill-pools` | pools with `peer_pool_ref` |
| `PUT` | `/api/skill-pools/{pid}/peer-link` | `{peer_pool_ref: <lower-case uuid> \| null}`; `409` already linked, `422` bad id; clearing withdraws sent figures (`peer_demand_withdrawals`) |
| `GET` | `/api/capacity-heatmap?from=YYYY-MM&months=1..24` | a linked pool's supply from WPM: `supply_source: "wpm"`, `peer_supply.status` `live`, `last_received`, `hidden` or `unknown`; `entered_here_supply_fte` never added |
| `PUT` | `/api/capacity-commitments` | a `demand` commitment on a linked pool (plan of kind `Program`) sends the programme's total; `peer_demand_send.outcome` |
| `GET` | `/api/capacity-commitments` | commitments with the latest `peer_demand_send` |
| `GET` | `/api/plans/{pid}/people-impact?months=1..36` | WPM's people impact, `source: "wpm"`, `status` `live` or `unknown` with a reason |

Tests: `tests/wpm_bridge.rs` (service, a stand-in WPM, needs the database); `tests/e2e/wpm-bridge.spec.ts` and
`tests/e2e/capacity-commitments.spec.ts` (UI, stubbed). Checked against a running WPM in part (FB-8, 2026-10-30: link, supply, demand, people impact through the API); the withdrawals and the screens were not.

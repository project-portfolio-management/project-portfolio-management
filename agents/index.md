# AGENTS directory — Portfolio Entity

Entity-level reference documentation for the portfolio trio:
service crate + matcher crate + front-end.

## Documents in this directory

| Document | Description |
|---|---|
| [spec-driven-development.md](spec-driven-development.md) | SDD discipline at entity level — authority model, three-part PRs, section mapping |
| [subprojects.md](subprojects.md) | The trio — responsibilities, dependency direction, how to run each |
| [models.md](models.md) | Domain model reference — the `Plan` DTO, its optional `kind` label, recursive containment, sub-resources and derived views |
| [matching.md](matching.md) | Matching algorithm reference — kind-agnostic scoring, weights, components, deterministic rules |
| [restful.md](restful.md) | REST API (one `/api/plans` collection) + front-end consumption + matcher library API |
| [review-checklist.md](review-checklist.md) | The one list of what a reviewer checks: security, database, Rust, UI and every change |
| [testing.md](testing.md) | Testing strategy across the trio |

## Subproject docs

| Subproject | Spec | Agent guide | Detailed guides |
|---|---|---|---|
| [project-portfolio-management-service-with-rust](../project-portfolio-management-service-with-rust/) | [spec/index.md](../project-portfolio-management-service-with-rust/spec/index.md) | [AGENTS.md](../project-portfolio-management-service-with-rust/AGENTS.md) | — (see the crate's AGENTS.md) |
| [project-portfolio-management-matcher-with-rust](../project-portfolio-management-matcher-with-rust/) | [spec/index.md](../project-portfolio-management-matcher-with-rust/spec/index.md) | [AGENTS.md](../project-portfolio-management-matcher-with-rust/AGENTS.md) | [agents/](../project-portfolio-management-matcher-with-rust/agents/index.md) (algorithm, normalization, SDD, testing) |
| [project-portfolio-management-ui-with-svelte](../project-portfolio-management-ui-with-svelte/) | [spec/index.md](../project-portfolio-management-ui-with-svelte/spec/index.md) | [AGENTS.md](../project-portfolio-management-ui-with-svelte/AGENTS.md) | — (thin) |

## Related documents in this repository

| Document | Description |
|---|---|
| [../spec/index.md](../spec/index.md) | Entity-level living spec |
| [../project-portfolio-management-service-with-rust/AGENTS.md](../project-portfolio-management-service-with-rust/AGENTS.md) | Service guide: route map, middleware, gotchas, testing |
| [../tasks.md](../tasks.md) · [../plan.md](../plan.md) | The work queue (`LT-n` from Leantime, then RM, PL, SC, PJ, DP, EX, HD, CC, IR, RG, CQ, CF, NC, BR, FB, WW and others) and its research |
| [../spec/13-tasks.md](../spec/13-tasks.md) | Numbered build-out (`T-n`) |

Shared documents the older notes cite as `agents/share/…` (privacy,
auditability, observability, restful conventions, …) live outside this
repository.

## See also

- [`../spec/index.md`](../spec/index.md) — entity-level living spec
  (source of truth for the cross-subproject contract)

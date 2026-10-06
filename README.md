# Project Portfolio Management

One entity, three subprojects: a registry of **plans** (one recursive
`/api/plans` collection; an optional `kind` label such as Portfolio, Project
or Program; any plan may contain any other) with explainable duplicate
matching, plus a project-management workspace (tasks, schedule, cost,
resources, governance).

| Subproject | What it is | Run | Test |
|---|---|---|---|
| [`project-portfolio-management-matcher-with-rust`](project-portfolio-management-matcher-with-rust) | Rust library: the `Plan` type and the matching engine, embedded by the API | `cargo run` (demo) | `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check` |
| [`project-portfolio-management-api-with-rust`](project-portfolio-management-api-with-rust) | loco.rs REST service; the API DTO is the matcher's `Plan` | `cargo loco start` (needs Postgres) | `cargo test` (database-free), `cargo test -- --ignored` (request tests, need Postgres) |
| [`project-portfolio-management-ui-with-svelte`](project-portfolio-management-ui-with-svelte) | SvelteKit 2 / Svelte 5 SPA with a BFF; calls the API | `pnpm install && pnpm dev` (http://localhost:5173) | `pnpm run check`, `npx vitest run`, `npx playwright test` |

The Rust toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml)
(1.96.1). The UI needs Node 26 and pnpm; its e2e tests stub the backend, so
they do not need the service. Run `pkill -f 'vite preview'` before
`npx playwright test` if an earlier preview server may still be running.

## Where things live

- [`spec/`](spec) is the entity-wide specification (start at
  [`spec/index.md`](spec/index.md)). Each subproject also has its own `spec/`
  for its internals; the entity spec wins on the cross-subproject contract.
- [`agents/`](agents) holds shared guidance for AI agents (start at
  [`agents/index.md`](agents/index.md)); each subproject has an `AGENTS.md`.
- [`tasks.md`](tasks.md) is the backlog of record for the idea-driven items
  (`LT-`, `RM-`, `PL-`, `SC-`, `KO-`, `MS-`, `CR-`, `PJ-`); every item is
  ticked as of 2026-10-06, and each entry says what is "Not built".
- [`plan.md`](plan.md) is the research and decision history behind those
  items (six passes, §1 to §31).
- [`project-portfolio-management-skill/`](project-portfolio-management-skill)
  is an agent skill describing how to work in this repo.

See [`index.md`](index.md) for a link map.

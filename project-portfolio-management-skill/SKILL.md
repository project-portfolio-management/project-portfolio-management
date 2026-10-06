---
name: project-portfolio-management
description: How an AI agent works in the Project Portfolio Management repo (Rust matcher library, Rust loco.rs API, SvelteKit UI). Use when editing code, specs, tasks.md or tests here, or when you need the run and test commands, conventions and gotchas.
---

# Working in the Project Portfolio Management repo

## Layout

- `project-portfolio-management-matcher-with-rust/`: library; the `Plan` type and matching engine.
- `project-portfolio-management-api-with-rust/`: loco.rs service under `/api/...`; the DTO is the matcher's `Plan`.
- `project-portfolio-management-ui-with-svelte/`: SvelteKit 2 / Svelte 5 (runes) SPA with a BFF.
- `spec/` (entity contract), `agents/` (shared guidance), `tasks.md` (backlog), `plan.md` (research history).
- Each subproject has `AGENTS.md`, `README.md`, `index.md` and its own `spec/`. Read the subproject `AGENTS.md` before editing it.

## Commands

| Where | Command |
|---|---|
| matcher | `cargo test`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo fmt --check` |
| api | `cargo test` (database-free); `cargo test -- --ignored` (needs Postgres); `cargo loco start` |
| ui | `pnpm run check` (0 errors, 0 warnings expected); `npx vitest run`; `pkill -f 'vite preview'` then `npx playwright test` |

Toolchain is pinned in `rust-toolchain.toml`; the UI pins Node 26.

## Conventions

- Spec-driven: a behaviour change is a spec edit, a code edit and a test edit together. Entity-wide contract: `spec/index.md`; crate internals: that crate's `spec/`.
- One recursive `/api/plans` collection; `kind` is an optional label and never gates matching.
- Absent evidence is `null` with a reason, never zero; money is integer minor units, per currency, never summed across currencies.
- UI: runes only; pure view modules (`src/lib/*-view.ts`) with vitest; API calls through `PlanRepository` and `PpmClient` (`src/lib/api/ppm.ts`); in-app links via `localeHref()`; one primary action per screen; acronyms glossed with `Term.svelte`; the CEO dashboard fits 1080x810 and 810x1080 without scrolling. Details: the UI `AGENTS.md`.
- Playwright specs stub the backend with `page.route`; an axe audit covers every page route.
- Do not create a `*.github.io` site; none exists.

## Gotchas

- A stale `vite preview` on port 4173 can serve an old build to Playwright; kill it first.
- `tasks.md` is about 150 KB: edit entries surgically; keep history, do not delete.
- Counts in docs (tests, routes, locales) go stale; verify against code (the UI has 10 locales in `src/lib/locales.ts`).
- The API's OpenAPI document is hand-written (`src/openapi.rs`, `src/openapi_more.rs`); a new endpoint must be registered there. See the API `AGENTS.md`.

## Where the backlog lives

`tasks.md` at the repo root (ticked items state what landed and what is "Not built"); `plan.md` has the research behind it; each subproject's `spec/index.md` §13 is its own work queue. See `references/commands.md`.

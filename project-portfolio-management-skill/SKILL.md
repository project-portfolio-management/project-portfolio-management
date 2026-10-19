---
name: project-portfolio-management
description: How an AI agent works in the Project Portfolio Management repo (Rust loco.rs API with its plan-matcher module, SvelteKit UI). Use when editing code, specs, tasks.md or tests here, or when you need the run and test commands, conventions and gotchas.
---

# Working in the Project Portfolio Management repo

## Layout

- `project-portfolio-management-service-with-rust/`: loco.rs service under `/api/...`; the DTO is the `Plan` of its `src/matcher/` module. The crate and binary are `project-portfolio-management`; its migration crate (`migration/`) is `project-portfolio-management-migration`.
- `project-portfolio-management-ui-with-svelte/`: SvelteKit 2 / Svelte 5 (runes) SPA with a BFF.
- `spec/` (entity contract), `agents/` (shared guidance), `tasks.md` (backlog), `plan.md` (research history).
- Each subproject has `AGENTS.md`, `README.md`, `index.md` and its own `spec/`. Read the subproject `AGENTS.md` before editing it.

## Commands

| Where | Command |
|---|---|
| service (with the matcher) | `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test --lib` (database-free); with Postgres 18 (`podman compose -f compose.test.yaml up -d --wait`): `DATABASE_URL=postgres://loco:loco@localhost:5432/project_portfolio_management_test cargo test --test mod -- --ignored --test-threads=1`, then each other `tests/*.rs` with `-- --ignored`; run the app with `podman compose up -d --build --wait` |
| ui | `pnpm run check` (0 errors, 0 warnings expected); `npx vitest run`; `PW_PORT=4183 CI=true pnpm exec playwright test` (other projects on a shared machine may hold 4173) |

Toolchain is pinned in `rust-toolchain.toml` (1.99.1; minimum supported 1.96); the UI pins Node 26. Use Podman (`podman compose`), not Docker, in instructions.

## Conventions

- Spec-driven: a behaviour change is a spec edit, a code edit and a test edit together. Entity-wide contract: `spec/index.md`; crate internals: that crate's `spec/`.
- One recursive `/api/plans` collection; `kind` is an optional label and never gates matching.
- Absent evidence is `null` with a reason, never zero; money is integer minor units, per currency, never summed across currencies.
- UI: runes only; pure view modules (`src/lib/*-view.ts`) with vitest; API calls through `PlanRepository` and `PpmClient` (`src/lib/api/ppm.ts`); in-app links via `localeHref()`; one primary action per screen; acronyms glossed with `Term.svelte`; the CEO dashboard fits 1080x810 and 810x1080 without scrolling. Details: the UI `AGENTS.md`.
- Tables are `DataTable` (Lily's DataGrid; export, copy, columns control); widgets come from `@lilydesignsystem/*`. Computed styles use Svelte's `style:` directive, never a `style="…"` attribute (the CSP blocks those).
- Playwright specs stub the backend with `page.route`; an axe audit covers every page route.
- Do not create a `*.github.io` site; none exists.
- Stage named files (never `git add -A`); after adding tests run `bash scripts/generate-status.sh`; after editing `llms.json` run `python3 scripts/generate-llms.py`. Say plainly in `CHANGELOG.md` what was not done or not verified.

## Gotchas

- A stale `vite preview` on port 4173 can serve an old build to Playwright; kill it first.
- `tasks.md` is about 150 KB: edit entries surgically; keep history, do not delete.
- Counts in docs (tests, routes, locales) go stale; verify against code (the UI has 10 locales in `src/lib/locales.ts`).
- The API's OpenAPI document is hand-written (`src/openapi.rs`, `src/openapi_more.rs`); a new endpoint must be registered there. See the API `AGENTS.md`.

- `axum-test` and `jsonwebtoken` follow `loco-rs`; bump them only with it. `dtolnay/rust-toolchain` is bumped by hand with `rust-toolchain.toml`.
- A release is a `v*.*.*` tag equal to the service crate and UI versions; it pushes an image to GHCR (private until the owner makes it public).

## Where the backlog lives

`tasks.md` at the repo root (ticked items state what landed and what is "Not built"); `plan.md` has the research behind it; each subproject's `spec/index.md` §13 is its own work queue. See `references/commands.md`.

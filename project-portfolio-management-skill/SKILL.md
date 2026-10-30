---
name: project-portfolio-management
description: How an AI agent works in the Project Portfolio Management repo (Rust matcher library, Rust loco.rs API, SvelteKit UI). Use when editing code, specs, tasks.md or tests here, or when you need the run and test commands, conventions and gotchas.
---

# Working in the Project Portfolio Management repo

## Layout

- `project-portfolio-management-matcher-with-rust/`: library; the `Plan` type and matching engine.
- `project-portfolio-management-service-with-rust/`: loco.rs service under `/api/...`; the DTO is the matcher's `Plan`. The crate and binary are `project-portfolio-management`; its migration crate (`migration/`) is `project-portfolio-management-migration`.
- `project-portfolio-management-ui-with-svelte/`: SvelteKit 3 / Svelte 5 (runes) SPA with a BFF.
- `spec/` (entity contract), `agents/` (shared guidance), `tasks.md` (backlog), `plan.md` (research history).
- Each subproject has `AGENTS.md`, `README.md`, `index.md` and its own `spec/`. Read the subproject `AGENTS.md` before editing it.

## Commands

| Where | Command |
|---|---|
| matcher | `cargo test`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo fmt --check` |
| service | `cargo fmt --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --lib` (database-free); with Postgres 18 (`podman compose -f compose.test.yaml up -d --wait`): `DATABASE_URL=postgres://loco:loco@localhost:5432/project_portfolio_management_test cargo test --test mod -- --ignored --test-threads=1`, then each other `tests/*.rs` with `-- --ignored`; run the app with `podman compose up -d --build --wait`, or from source `cargo run -- start` (there is no `cargo loco`) |
| ui | `pnpm run lint`; `pnpm run check` (0 errors, 0 warnings expected); `npx vitest run`; `PW_PORT=4183 CI=true pnpm exec playwright test` (other projects on a shared machine may hold 4173) |

Toolchain is pinned in `rust-toolchain.toml` (1.99.1; minimum supported 1.96); the UI pins Node 26. Use Podman (`podman compose`), not Docker, in instructions.

## Conventions

- Before pushing: `scripts/generate-status.sh --check`, `python3 scripts/generate-openapi-bodies.py --check` and `python3 scripts/generate-llms.py --check`; the restore drill (`scripts/restore-test.sh`, see `AGENTS.md`) runs the whole suite on a restored copy, so a test must not depend on data other tests create.
- Spec-driven: a behaviour change is a spec edit, a code edit and a test edit together. Entity-wide contract: `spec/index.md`; crate internals: that crate's `spec/`.
- One recursive `/api/plans` collection; `kind` is an optional label and never gates matching. Every mounted route is in the service's `api-routes.txt` and `openapi.json`; `references/api.md` maps the families and the bridge to the workforce service (WPM).
- Absent evidence is `null` with a reason, never zero; money is integer minor units, per currency, never summed across currencies.
- UI: runes only; pure view modules (`src/lib/*-view.ts`) with vitest; API calls through `PlanRepository` and `PpmClient` (`src/lib/api/ppm.ts`); in-app links via `localeHref()`; one primary action per screen; acronyms glossed with `Term.svelte`; the CEO dashboard fits 1080x810 and 810x1080 without scrolling. Details: the UI `AGENTS.md`.
- Tables are `DataTable` (Lily's DataGrid; export, copy, columns control); widgets come from `@lilydesignsystem/*`. Computed styles use Svelte's `style:` directive, never a `style="…"` attribute (the CSP blocks those).
- Playwright specs stub the backend with `page.route`; an axe audit covers every page route.
- Do not create a `*.github.io` site; none exists.
- Stage named files (never `git add -A`); after adding tests run `bash scripts/generate-status.sh`; after editing `llms.json` run `python3 scripts/generate-llms.py`. Say plainly in `CHANGELOG.md` what was not done or not verified.

## Gotchas

- A stale `vite preview` on port 4173 can serve an old build to Playwright; kill it first.
- `tasks.md` is about 150 KB: edit entries surgically; keep history, do not delete.
- Counts in docs (tests, routes, locales) go stale; do not write them by hand. The current ones are in the generated block of `spec/14-implementation-status.md` (`bash scripts/generate-status.sh`, source-line counts; a test run reports more, as some tests are generated in loops), or verify against code.
- The API's OpenAPI paths are written by hand in `src/openapi.rs` and `src/openapi_more.rs`; request bodies are generated into `src/openapi_bodies.rs` by `scripts/generate-openapi-bodies.py`. Tests write `openapi.json` and `api-routes.txt` and fail when a mounted route is undocumented. See the API `AGENTS.md`.
- The WPM bridge is off (stub mode) until `PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL` is set; it forwards the caller's own token, never stores it, never fails a PPM request, and never adds WPM's supply to PPM's. Do not invent a mapping of WPM skills to PPM tags (FB-7).

- `axum-test` and `jsonwebtoken` follow `loco-rs`; bump them only with it. `dtolnay/rust-toolchain` is bumped by hand with `rust-toolchain.toml`.
- A release is a `v*.*.*` tag equal to the service crate and UI versions; it pushes an image to GHCR (private until the owner makes it public).

## Where the backlog lives

`tasks.md` at the repo root (ticked items state what landed and what is "Not built"); `plan.md` has the research behind it; each subproject's `spec/index.md` §13 is its own work queue. See `references/commands.md` and `references/api.md`.

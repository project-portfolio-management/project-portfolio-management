# Testing Strategy — Portfolio Entity

Entity-level view; normative inventory in entity spec
[§11](../spec/11-testing-strategy.md). Per-crate detail: matcher
[agents/testing.md](../project-portfolio-management-service-with-rust/agents/matcher/testing.md).

## What exists today

| Subproject | Suite | Command | Covers |
|---|---|---|---|
| matcher | Unit tests (`#[cfg(test)]` per module) | `cargo test` | Components, normalization, deterministic rules, kind-agnostic scoring, rank/find_matches |
| matcher | Public-API integration | `cargo test --test public_api` | The `lib.rs` re-export surface |
| matcher | Doctests | `cargo test` | rustdoc examples |
| service | DB-free embedding tests | `cargo test --test matching` | Matcher embedding + `Plan` JSON round-trip (the DTO/JSONB contract) |
| service | Unit tests | `cargo test --lib` | Pure rules beside each controller; validation pins (`422` on blank name, containment cycle) |
| service | Request suites (`tests/mod.rs` → `tests/requests/*`) | `cargo test --test mod -- --ignored --test-threads=1` + Postgres `DATABASE_URL` | Plan CRUD, match, merge, paging, bulk, cpm, task bulk, … `#[ignore]`d, so a plain `cargo test` is DB-free; serial because the database is shared |
| service | Real-token binaries | `cargo test --test <name> -- --ignored` | `private_aggregates` (private plans in every aggregate, enumerates mounted GET routes), `list_paging` (every list vs `openapi::PAGED_LISTS`), `approvals`, `processes`, `controlled_documents`, `cross_plan`, `guests`, … One binary each: the token verifier is a process-wide `OnceLock` |
| service | OTLP | `cargo test --test otlp_export --test otlp_middleware` | Real export to an in-process collector, no database |
| service | Keycloak | `cargo test --features keycloak --test keycloak_guard` | Keycloak access tokens through the real guard (in-process test IdP) |
| front-end | Type-level check | `pnpm run check` | svelte-check strict, 0 errors / 0 warnings |
| front-end | Build | `pnpm run build` | Production build |

## Quality gates (all PRs)

```bash
# both Rust crates
cargo test --lib
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# front-end
pnpm run check
pnpm run build
```

Three-part PR discipline: a behavioural change carries its spec edit
and its test edit in the same PR.

## What is missing

The request suites need a Postgres (`compose.test.yaml`); confirm what CI
runs against the repository's own CI configuration rather than assuming.
Front-end gaps are tracked by the UI project.

## Contract seams worth pinning

- **Service ↔ matcher.** No adapter exists, so the contract is just
  serde round-tripping plus engine behaviour — `tests/matching.rs`
  pins both. If an adapter ever appears, add a bridge suite.
- **Kind-agnostic matching.** Matching never consults `kind`; pin it
  directly in the matcher (two plans with differing `kind` labels can
  still match) and confirm `check-duplicates` scores across labels.
- **Containment cycles.** A `parent_ref` that points a plan at itself or a
  descendant must return `422`; pin this at the service.
- **Front-end ↔ service.** Types are hand-mirrored; only an e2e smoke
  (T-5) catches drift. Until then, treat any matcher-type change as a
  mandatory `types.ts` review.
- **Private plans and lists.** A new aggregate `GET` must be decided in
  `private_aggregates`; a new list must join `PAGED_LISTS`.
- **Goals bridge.** The goal-title → match-component coupling
  (`data.goals[]`) is worth explicit request-level coverage.

## Writing new tests

- Matcher: keep tests deterministic and IO-free, mirroring the library's
  own rules (no clocks, no RNG seeds that matter). Note the `Timeframe`
  component uses dates — pin fixed dates, never `now()`. Include at least
  one pair with differing `kind` labels to pin that matching is
  kind-agnostic.
- Service: prefer DB-free tests where the behaviour allows; request-level
  tests are `#[ignore]`-gated (need Postgres and a `DATABASE_URL`); tests
  that use real tokens go in their own binary. Never hold a transaction
  while querying the pool in a test setup either: the test pool has one
  connection.
- Construct minimal records with `Plan::new("…")` (kind defaults to
  `None`) and set only the fields under test — renormalization means
  absent components simply drop out of the score.

# AGENTS.md — Project Portfolio Management

Entry point for AI coding agents at the repository root. Three projects live here; each has its own, longer
`AGENTS.md`. [`llms.txt`](llms.txt) and [`llms.json`](llms.json) list every document.

| Project | Directory | Guide |
| --- | --- | --- |
| Service (Axum and axa REST API, PostgreSQL) | `project-portfolio-management-service-with-rust` | [AGENTS.md](project-portfolio-management-service-with-rust/AGENTS.md) |
| Matcher (pure Rust library) | `project-portfolio-management-matcher-with-rust` | [AGENTS.md](project-portfolio-management-matcher-with-rust/AGENTS.md) |
| Operator UI (SvelteKit 3, Svelte 5, Lily widgets) | `project-portfolio-management-ui-with-svelte` | [AGENTS.md](project-portfolio-management-ui-with-svelte/AGENTS.md) |

Versions: service 0.2.0, matcher 0.2.2, UI 0.2.0 (tag `v0.2.0`). Rust 1.100.0 (pinned in `rust-toolchain.toml`; the minimum supported version is 1.98 for every crate; axa 0.20 needs it for the service), Node 26, pnpm, Podman.

## How work is done here

1. **Spec first.** `spec/` and each project's `spec/index.md` are the single source of truth. A behavioural change is
   one commit with a spec edit, a code edit and a test edit (`agents/spec-driven-development.md`).
2. **One item, one commit, one push**, then look at CI (`gh run list`). Stage named files, never `git add -A`.
3. **Say plainly what is not built or not tested.** Every change to `CHANGELOG.md` states what was not done or not
   verified. Do not weaken a guard or census test to get green; fix the cause or record why.
4. **Run the gates before pushing** (below), and `bash scripts/generate-status.sh` after adding tests, and
   `python3 scripts/generate-openapi-bodies.py` after adding or changing a request body, or the docs job fails.
   `python3 scripts/generate-llms.py` after editing `llms.json`. Check all three with `--check`.
5. Welsh (and other) strings are machine-written until a speaker reviews them; `governance/welsh-translation-review.csv`
   lists them. Never claim a translation is reviewed.

## Review

[`agents/review-checklist.md`](agents/review-checklist.md) is the one list of what a reviewer checks. Four reviewer
agents read it: `security-reviewer`, `database-reviewer`, `rust-reviewer` and `ui-reviewer`
([`.claude/agents/`](.claude/agents/)). They add no rule of their own.

## Coverage

[`COVERAGE.md`](COVERAGE.md) records the service workspace's test coverage (cargo-tarpaulin, CI's `coverage` job),
its baseline by crate, and where tests are most needed. No threshold is set yet; do not add one without the
maintainers' decision.

## Gates

```bash
# service (a workspace: the service, `domain` and `migration`; run from its directory). The runner is cargo-nextest (once: `cargo install cargo-nextest --locked`;
# its settings are service/.config/nextest.toml); `cargo test` still runs every test, but not in CI's way
cargo fmt --all --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo nextest run --workspace --lib
# matcher
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --lib
# service: the one integration-test binary (tests/main.rs; each former tests/*.rs binary is a module of it), database-free part
cargo nextest run --test main
# database tests need Postgres 18 (the file also starts a Redis 8 on 127.0.0.1:6379, password `redis`, not needed by the tests): podman compose -f project-portfolio-management-service-with-rust/compose.test.yaml up -d --wait
#   then, as CI's `api-db` job runs them, the request suites and then everything else that is ignored:
#   DB_URL=postgres://postgres:postgres@localhost:5432/project_portfolio_management_test cargo nextest run --profile db --test main --run-ignored only -E 'test(/^requests::/)'
#   DB_URL=… cargo nextest run --profile db --test main --run-ignored only -E 'not test(/^requests::/) and not test(/^load_test::/)'
#   (the `db` profile runs the database suites one at a time, as they share one database; a second database lets two run at once)
#   (or a throwaway database on a local Postgres: createdb -U postgres -O "$USER" <name>, and drop it after)
#   one former binary is a module filter: cargo nextest run --profile db --test main --run-ignored only -E 'test(/^approvals::/)'
#   (load_test needs a running service: see its module docs; keycloak_container and fluvio_relay need their features)
# restore drill, as CI runs it (the request suites on a restored copy; run it alone, two at once collide):
#   scripts/restore-test.sh -t 'cd project-portfolio-management-service-with-rust && DB_URL="$RESTORED_DB_URL" cargo nextest run --profile db --test main --run-ignored only -E "test(/^requests::/)"'
# mutation test of the matcher (CI's `mutants` job mutates the changed lines; weekly, every mutant):
#   cd project-portfolio-management-matcher-with-rust && git diff --relative origin/main -- . > /tmp/d && cargo mutants --in-diff /tmp/d
# chaos test (random database and service faults under load; Podman, Node 22+; weekly in CI, not a merge gate): scripts/chaos-test.sh --seconds 60
# test coverage (cargo-tarpaulin 0.37.6, `cargo install cargo-tarpaulin --version 0.37.6 --locked`; no threshold yet, COVERAGE.md):
#   cd project-portfolio-management-service-with-rust && make coverage   (unit + DB-free integration; reports in target/coverage/)
#   make coverage-db with DB_URL set adds the database suites, as CI's `coverage` job runs them
# UI (other projects on this machine may hold port 4173: use PW_PORT)
cd project-portfolio-management-ui-with-svelte && pnpm run lint && pnpm run check && npx vitest run && PW_PORT=4183 CI=true pnpm exec playwright test
```

## Things that are easy to get wrong

- The crate is `project-portfolio-management`, the binary the same, the migration crate
  `project-portfolio-management-migration`, and the pure domain rules (`cpm`, `evm`, `tz`, `validation`, …) the crate
  `project-portfolio-management-domain` in `project-portfolio-management-service-with-rust/domain/`; the directories
  keep their `-service-with-rust` names. The service re-exports each domain module under its old name, so
  `crate::cpm` still works; the domain crate may not depend on the service, axum, axa, sqlx or tokio (spec §8.3c).
  Use `--workspace` (`--all` for fmt), or the domain crate's tests and lints are skipped.
- The UI wraps one table component (`DataTable`) and Lily widgets (`@lilydesignsystem/*`); there is no SVAR.
  Computed styles use Svelte's `style:` directive, never a `style="…"` attribute (the CSP blocks those).
- `axum-test` follows `axa`, and `jsonwebtoken` stays at the version the Keycloak and OIDC code was written against; do not bump them alone. `dtolnay/rust-toolchain` is pinned by commit SHA and
  given an explicit `toolchain:` input; bump both by hand together with `rust-toolchain.toml` (the `fresh-clone` job checks they agree).
- Releases: a `v*.*.*` tag must equal the service crate and UI versions; the workflow pushes an image to GHCR.
- Use Podman (`podman compose`), not Docker, in instructions and scripts. CI still builds with Docker.
- The service has no ORM: plain sqlx through `src/db.rs` (`db::pool(&ctx)`, `db::begin(&ctx)`), so every statement
  is counted; `ctx.db` is the `PgPool`, and a census refuses it outside `src/db.rs`, tests included (spec §8.3b).
  Rows are read into `src/models/_entities` (`Model` row structs, whose JSON some routes answer with).
- Migrations are frozen SQL files in `project-portfolio-management-service-with-rust/migration/migrations/`, run by
  sqlx (`_sqlx_migrations`); never edit a released one (its checksum is checked), add a new file. A database the old
  `sea-orm-migration` runner built is bridged once from `seaql_migrations`, which stays
  ([`operations/upgrade-and-migration.md`](operations/upgrade-and-migration.md)).
- Dates, instants and zones in the service are `crate::datetime` types (the `datetime/` crate, over `jiff` 1.0), which
  keep chrono's wire forms and readers on purpose; do not add `chrono` back, and use `Tz` (the bundled database),
  never jiff's system zone or global database (`clippy.toml` refuses them).
- The service binary is started with `cargo run -- start` (and `cargo run -- task <name>`); there is no
  framework subcommand.
- The bridge to the workforce service (WPM) is entity spec §8.5a and service spec §9.10a; it is off (stub mode)
  until `PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL` is set, and every call carries the caller's own token.
  Walk-through: [`operations/wpm-bridge.md`](operations/wpm-bridge.md).

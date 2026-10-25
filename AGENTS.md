# AGENTS.md — Project Portfolio Management

Entry point for AI coding agents at the repository root. Three projects live here; each has its own, longer
`AGENTS.md`. [`llms.txt`](llms.txt) and [`llms.json`](llms.json) list every document.

| Project | Directory | Guide |
| --- | --- | --- |
| Service (loco.rs REST API, PostgreSQL) | `project-portfolio-management-service-with-rust` | [AGENTS.md](project-portfolio-management-service-with-rust/AGENTS.md) |
| Matcher (pure Rust library) | `project-portfolio-management-matcher-with-rust` | [AGENTS.md](project-portfolio-management-matcher-with-rust/AGENTS.md) |
| Operator UI (SvelteKit 2, Svelte 5, Lily widgets) | `project-portfolio-management-ui-with-svelte` | [AGENTS.md](project-portfolio-management-ui-with-svelte/AGENTS.md) |

Version 0.1.0. Rust 1.99.1 (pinned in `rust-toolchain.toml`; the minimum supported version is 1.96), Node 26, pnpm, Podman.

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

## Gates

```bash
# service and matcher
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --lib
# database tests need Postgres 18 (the file also starts a Redis on 127.0.0.1:6379, password `loco`, not needed by the tests): podman compose -f project-portfolio-management-service-with-rust/compose.test.yaml up -d --wait
#   then DATABASE_URL=postgres://loco:loco@localhost:5432/project_portfolio_management_test cargo test --test mod -- --ignored --test-threads=1
# restore drill, as CI runs it (the whole suite on a restored copy; run it alone, two at once collide):
#   scripts/restore-test.sh -t 'cd project-portfolio-management-service-with-rust && DATABASE_URL="$RESTORED_DATABASE_URL" cargo test --test mod -- --ignored --test-threads=1'
# chaos test (random database and service faults under load; Podman, Node 22+; weekly in CI, not a merge gate): scripts/chaos-test.sh --seconds 60
# UI (other projects on this machine may hold port 4173: use PW_PORT)
cd project-portfolio-management-ui-with-svelte && pnpm run check && npx vitest run && PW_PORT=4183 CI=true pnpm exec playwright test
```

## Things that are easy to get wrong

- The crate is `project-portfolio-management`, the binary the same, the migration crate
  `project-portfolio-management-migration`; the directories keep their `-service-with-rust` names.
- The UI wraps one table component (`DataTable`) and Lily widgets (`@lilydesignsystem/*`); there is no SVAR.
  Computed styles use Svelte's `style:` directive, never a `style="…"` attribute (the CSP blocks those).
- `axum-test` and `jsonwebtoken` follow `loco-rs`; do not bump them alone. `dtolnay/rust-toolchain` is bumped by hand
  together with `rust-toolchain.toml`.
- Releases: a `v*.*.*` tag must equal the service crate and UI versions; the workflow pushes an image to GHCR.
- Use Podman (`podman compose`), not Docker, in instructions and scripts. CI still builds with Docker.

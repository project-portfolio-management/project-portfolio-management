# Command reference

Run from the subproject directory.

- Matcher: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, `cargo bench`, `cargo run` (demo).
- Service: `cargo build`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --lib` (database-free). Database tests: `podman compose -f compose.test.yaml up -d --wait` (Postgres on port 5432 by default, `TEST_DB_PORT=5433` to move it; it also starts Redis on 127.0.0.1:6379 with password `loco`, `TEST_REDIS_PORT` to move it; the tests use the in-memory cache and do not need it), then `DATABASE_URL=postgres://loco:loco@localhost:5432/project_portfolio_management_test cargo test --test mod -- --ignored --test-threads=1` and each other `tests/*.rs` binary with `-- --ignored --test-threads=1` (CI's job lists the exceptions). `cargo deny check` for advisories and licences. Run the stack: `podman compose up -d --build --wait` at the repository root.
- UI: `pnpm install`, `pnpm dev` (port 5173), `pnpm run check`, `pnpm run build`, `npx vitest run`, `PW_PORT=4183 CI=true pnpm exec playwright test` (builds and serves `vite preview`; `--project=mobile` for 390x844).
- UI environment: `PROJECT_PORTFOLIO_MANAGEMENT_API_URL` and `AUTH_API_URL`, both default `http://localhost:5150` (see `.env.example`).
- Repository: `bash scripts/generate-status.sh` (and `--check`), `python3 scripts/generate-llms.py` (and `--check`), `python3 scripts/check-links.py`, `python3 scripts/export-welsh-review.py`.

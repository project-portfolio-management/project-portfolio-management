# Command reference

Run from the subproject directory.

- Matcher: `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`, `cargo bench`, `cargo run` (demo).
- API: `cargo build`, `cargo test` (database-free), `cargo test -- --ignored` (request tests, need Postgres), `cargo loco start`.
- UI: `pnpm install`, `pnpm dev` (port 5173), `pnpm run check`, `pnpm run build`, `npx vitest run`, `npx playwright test` (builds and serves `vite preview` on 4173), `npx playwright test --project=mobile` (390x844).
- UI environment: `PROJECT_PORTFOLIO_MANAGEMENT_API_URL` and `AUTH_API_URL`, both default `http://localhost:5150` (see `.env.example`).

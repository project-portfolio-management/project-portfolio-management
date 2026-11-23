.PHONY: demo test coverage coverage-db

# The demonstration, as operations/demo.md runs it: the compose profile builds the API and UI, migrates, seeds the
# fictitious portfolio with scripts/seed-demo.mjs and starts the UI on http://localhost:3000. There is no `seed` task.
# Stop it, from the repository root, with: podman compose --profile demo down
POSTGRES_PASSWORD ?= demo
demo:
	cd .. && POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) podman compose --profile demo up -d --build --wait ui-demo

# The database-free gates from the root AGENTS.md (the database tests need Postgres; see AGENTS.md). Needs cargo-nextest
# (`cargo install cargo-nextest --locked`).
test:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cargo nextest run --workspace --lib
	cargo nextest run --test main

# Test coverage with cargo-tarpaulin 0.37.6 (install: cargo install cargo-tarpaulin --version 0.37.6 --locked): the
# workspace's unit tests and the database-free integration tests, as tarpaulin.toml sets them out (LLVM engine; its
# comments say what is left out of the denominator and why). Reports in target/coverage/ (HTML, lcov, JSON); the
# summary by crate is scripts/coverage-summary.py. No threshold is set (COVERAGE.md).
coverage:
	cargo tarpaulin --locked
	python3 ../scripts/coverage-summary.py

# The same with the database suites (tarpaulin-db.toml), as CI's coverage job runs it. Needs Postgres and DB_URL
# (see AGENTS.md, Testing); use a throwaway database.
coverage-db:
	cargo tarpaulin --locked --config tarpaulin-db.toml
	python3 ../scripts/coverage-summary.py

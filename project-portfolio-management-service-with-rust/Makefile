.PHONY: demo test

# The demonstration, as operations/demo.md runs it: the compose profile builds the API and UI, migrates, seeds the
# fictitious portfolio with scripts/seed-demo.mjs and starts the UI on http://localhost:3000. There is no `seed` task.
# Stop it, from the repository root, with: podman compose --profile demo down
POSTGRES_PASSWORD ?= demo
demo:
	cd .. && POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) podman compose --profile demo up -d --build --wait ui-demo

# The database-free gates from the root AGENTS.md (the database tests need Postgres; see AGENTS.md).
test:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --lib
	cargo test --test main

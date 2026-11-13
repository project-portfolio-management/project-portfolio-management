.PHONY: demo test

demo:
	cargo run -- db migrate
	cargo run -- task seed
	cargo run -- start

# The database-free gates from the root AGENTS.md (the database tests need Postgres; see AGENTS.md).
test:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --lib

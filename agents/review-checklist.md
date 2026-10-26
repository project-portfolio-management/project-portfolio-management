# Review checklist

The single copy of what a reviewer checks in this repository. The reviewer agents in
[`.claude/agents/`](../.claude/agents/) point here and add no rule of their own; to change what a reviewer checks,
change this file. Every item names where it comes from (a guide, a test or `AGENTS.md`), so it can be checked and
removed when it stops being true. A reviewer reports what it found, with the file and line, and says what it did not
look at.

## Security

1. A new `/api/plans/{pid}/…` route goes through the plan guard, so a private plan is `404` to a non-member. (service `AGENTS.md`, privacy)
2. A view that adds up, lists or counts across plans routes its rows through `HiddenScope`, so a private plan contributes to no total. A new parameterless `GET` is named in `tests/private_aggregates.rs` (aggregate or not, with a reason). (service `AGENTS.md`)
3. Code that parses a path in a guard percent-decodes first and fails closed. (service `AGENTS.md`)
4. A write is audited before it is committed: `AuditModel::record` inside the transaction, or `record_or_log`; never `record(..).await.ok()`. (service `AGENTS.md`, CQ-3)
5. Free text from a request is never put into SQL; a table, column or condition in a format string comes from a constant. (`describe` in `controllers/meetings.rs`)
6. Absent evidence is `null` with a reason, never zero. (service `AGENTS.md`, rule 7)
7. A token's scope can only narrow: `scope=readonly` and `scope=capture` are decided before the policy, and a policy file cannot widen them. (spec NFR-5)
8. The UI sends the operator bearer from the server only; the browser never holds a token and never sends `Authorization`; the proxy refuses a cross-site write. (UI `AGENTS.md`, CQ-5)

## Database

1. A new table is named in the plural in the migration, in the entity's `table_name` and in raw SQL (`create_table` pluralises). (memory: loco create_table pluralises)
2. Each migration file is registered in `migration/src/lib.rs`, in both the `mod` list and the `migrations()` list, and can run twice (`IF NOT EXISTS`, `CREATE OR REPLACE FUNCTION` for triggers). (service `AGENTS.md`)
3. Append-only and immutability rules are database triggers, not only code. (service `AGENTS.md`)
4. A list is bounded (a cap or `limit`/`offset`), and a list route is in `PAGED_LISTS` or, if the census cannot call it, in `UNREACHED` with a reason in `tests/list_paging.rs`. (`tests/list_paging.rs`)
5. A multi-row change is one transaction with its audit row; a change made in several steps says what happens when a step fails. (`controllers/plan_templates.rs`)

## Rust

1. `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` are clean; format only the files you touched. (root `AGENTS.md`; memory: CI gates before push)
2. Every new route is in `src/openapi.rs`/`openapi_bodies.rs` (regenerate with `python3 scripts/generate-openapi-bodies.py`), `api-routes.txt` (`UPDATE_API_ROUTES=1 cargo test --lib api_routes_file`) and `openapi.json` (`UPDATE_OPENAPI_JSON=1 cargo test --lib openapi_json_file`); a new write documents its request body. (service `AGENTS.md`)
3. Pure rules live in a DB-free module with unit tests; the controller only fetches, calls them and writes. (service `AGENTS.md`)
4. The matcher `Plan` type is reused, never forked; `kind`, `status` and `phase` are not scored and there is no kind gate. (matcher `AGENTS.md`)
5. A test that needs real tokens is in its own binary (the verifier is a process-wide `OnceLock`), never in `tests/mod.rs`. (service `AGENTS.md`)
6. A behaviour change is one commit with a spec edit, a code edit and a test edit. (root `AGENTS.md`, `agents/spec-driven-development.md`)
7. A guard or census test is not weakened to get green; the cause is fixed or the reason recorded. (root `AGENTS.md`)
8. A test does not compare the host clock with a database timestamp at the same instant; the two can differ by a fraction of a second. (`CHANGELOG.md`, 2026-10-26)

## UI

1. Logic a page needs is a plain function in a `src/lib/*-view.ts`-style module with a vitest test; the page only renders, and does not recompute what the server owns. (UI `AGENTS.md`)
2. Computed styles use Svelte's `style:` directive, never a `style="…"` attribute; the CSP blocks those. (root and UI `AGENTS.md`)
3. A new API shape is added to `src/lib/api/types.ts` in the same change as the service. (UI `AGENTS.md`)
4. No left-hand sidebar; an acronym never stands alone in a heading or label; status is never shown by colour alone. (UI `AGENTS.md`)
5. The UI wraps `DataTable` and the Lily widgets; there is no SVAR. (root `AGENTS.md`)
6. Welsh and other strings are machine-written until a speaker reviews them; never claim a translation is reviewed. (root `AGENTS.md`)
7. `pnpm run check`, `npx vitest run` and the Playwright run pass (`PW_PORT` set, since other projects hold 4173). (root `AGENTS.md`; memory: shared machine ports)

## Every change

1. `CHANGELOG.md` says what was not done and what was not verified. (root `AGENTS.md`)
2. Files are staged by name, never `git add -A`. (root `AGENTS.md`; memory: stage specific paths)
3. After adding tests: `bash scripts/generate-status.sh`; after editing `llms.json`: `python3 scripts/generate-llms.py`; each passes with `--check`. (root `AGENTS.md`)
4. Every test binary was run, not only the ones near the change. (`CHANGELOG.md`, BR-4: a census test in another binary failed CI)

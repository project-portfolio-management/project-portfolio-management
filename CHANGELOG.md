# Changelog

Summaries of the history of the whole repository, newest first. Each subproject
keeps its own detailed changelog, and those are the record of what changed in
the code:

- [`project-portfolio-management-matcher-with-rust/CHANGELOG.md`](project-portfolio-management-matcher-with-rust/CHANGELOG.md)
- [`project-portfolio-management-service-with-rust/CHANGELOG.md`](project-portfolio-management-service-with-rust/CHANGELOG.md)
- [`project-portfolio-management-ui-with-svelte/CHANGELOG.md`](project-portfolio-management-ui-with-svelte/CHANGELOG.md)

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
It is pre-1.0 and no `vX.Y.Z` release has been cut from the root release
workflow yet. Dates are 2026.

## Unreleased

### 20 October

- **Ways of working and governance frameworks (FR-52).** Every project can follow a ways of working and a governance
  framework: its own, a shared one (organisation-wide, or a programme's), or the nearest ancestor's by inheritance
  down the plan tree, resolved on read so a programme's choice reaches its projects without copying. Built as one
  implementation for both families. Verified: lint, unit tests (1040, incl. 5 new), the full database suite (178, incl. 2
  new), every real-token binary; the private-plan census now seeds a plan-owned record of each family and shows a
  non-member's lists unchanged. Decisions I made for you: the two families' fields (methodology, cadence, WIP limit,
  definitions of ready and done, ceremonies; decision model, review and reporting cadence, escalation path, decision
  rights, forums) and the value lists are my proposal, easy to change; a record plans follow can be edited in place with
  no versions; any caller who can write may create or link one (no special right). Not done: no screen; nothing checks
  that a plan's actual practice matches what it declares (for example its WIP limit against the board's); the
  family-specific values are checked by the service, not by database constraints; the existing `governance` module
  (gate reviews, change requests) is unrelated and not linked to this.

- **Code quality: three more shared error helpers.** `conflict`, `forbidden` and `refuse(status, code, message)` join
  `db_err` and `unprocessable` in `controllers/mod.rs`, replacing 37 identical copies in 32 files (156 fewer lines).
  No behaviour change. Verified: lint, unit tests, the full database suite (176) and every real-token test binary.
  Left alone: copies with a different body (a fixed message, a different code or status).

- **Code quality: shared error helpers.** The controllers carried 96 copies of `db_err` and 83 of `unprocessable`. The
  69 identical `db_err` copies and 44 identical `unprocessable` copies (code `unprocessable`) are now one definition each
  in `controllers/mod.rs`: 526 fewer lines over 73 files, no behaviour change. Verified: lint, unit tests, the full
  database suite (176) and every real-token test binary. Left on purpose: 30 `unprocessable` copies answer with the
  code `unprocessable_entity` (changing it would change API responses; the two spellings are a consistency problem
  worth a decision), 20 `db_err` copies return a different error, and the 66 `too_many_lines` suppressions each need
  their own split.

- **Control action to issue (T-26).** The issues register exists now, so `POST /api/actions/{pid}/convert?to=issue`
  turns a control action into an issue on the same plan. Verified: lint, unit tests, the full database suite (176) and
  every real-token test binary. Not done: no button in the interface (the control pages have no screens for actions
  yet, as far as I checked); the issue's kind and severity are fixed defaults (`concern`, `med`), not derived from the
  action; the action's own kind (for example `correct`) is only mentioned in the issue text.

- **Approval processes page.** `/approval-processes` defines processes, starts runs and follows them; the last open gap
  in the list (PL-2 had no front-end). Verified: type check, unit tests (714), the full Playwright run (611, incl. 2 new
  and an axe audit of the opened run). **Found on the way:** my first path, `/processes`, would have shadowed the
  existing plan-kind collection of that name, which a layout unit test caught; the page is at `/approval-processes`.
  Not done: editing or deleting a definition, picking a subject from a list, showing the approval a waiting step is
  blocked on; not tried against a running service, only with the API stubbed, so the page's request shapes are checked
  against the controller by reading it, not by a test. Welsh strings are machine-written and unreviewed.

- **RBS drill-down.** `GET /api/resource-plan/role-tasks?role=` and a button per role on the resource breakdown page
  open the open tasks of the people in a role. Admins and resource managers only. Verified: lint, unit tests, the full
  database suite (176), the full Playwright run (605), and the real-token census (`private_aggregates`: 403 for a
  non-admin, 200 for an admin). **Not tested: that a private plan's tasks are left out for a resource manager who is not
  a member**: the endpoint uses the same filter as `/api/me/tasks`, but nothing here proves it for this route.
  Skills and the unclassified row have no drill-down; the tasks shown are open tasks assigned to the role's people by
  `assignee_ref`, not the bookings that produce the demand figure, so the two need not agree.

- **MS Project (MSPDI) export.** `GET /api/plans/{pid}/export/mspdi` writes a plan as MSPDI XML that the importer reads
  back; imported tasks keep their UIDs; the file says what it leaves out (assignees and resources, partial progress,
  baselines, other relations). Also corrected three documents that still said there was no export (and the roadmap's
  claim that `goals` and issues do not exist). Verified: lint, unit tests incl. a writer-to-reader round trip, the
  full database suite (175) incl. an import, reorder, export, re-import test. **Not verified: the file was never
  opened in Microsoft Project or any other tool**, only read back by our own parser, so a real MS Project may
  complain about something our parser accepts (for example, no `Finish` dates, a project summary row of my own
  design). Not done: no UI button; no assignees or resources; progress is 0 or 100 only; an export over 5000 tasks is
  refused.

- **WBS sibling reordering (PJ-12).** `PUT /api/plans/{pid}/task-order` (the full child list of one parent, refused with
  409 unless it is exactly the current children) stores `tasks.sibling_position`; task lists come back positioned
  siblings first. The work breakdown page has move-up and move-down buttons. Verified: lint, unit tests (service 1032,
  UI 706 incl. 3 new), the full database suite (174, incl. a new request test), the full Playwright run (604, incl.
  one new test with axe). Not done: no drag and drop; moving to a different parent is still the task edit; the
  reorder button sends only the siblings the reader can see, so with a hidden sibling it gets a 409 and the task cannot
  be moved by that reader; tasks created later have no position until the next reorder (they follow the positioned
  ones); the RBS drill-down gap remains. Welsh strings are machine-written and unreviewed.

### 19 October

- **Chaos test.** `scripts/chaos-test.sh` (and `chaos-test.mjs`) runs the real service against the real test Postgres
  under a mixed workload (plans, goals, reads) while a seeded monkey pauses the database, drops every client
  connection, and kill -9s the service. It checks: the service is healthy again within 30 s of each fault and never
  exits on its own; every response is a success or a clean 5xx (no panics, no non-JSON errors, no wrong 4xx);
  every acknowledged plan and goal is still there afterwards, goals in order; and a listing is fast after the last
  fault. `--self-test` removes a plan behind the service's back and passes only if the check notices. A new
  `chaos.yml` workflow runs it weekly and on demand; it is not part of `ci-ok`. Ran locally: 40 s at seed 7 (8 faults,
  no violations) and the self-test. Not covered: a Postgres crash (the test database is in RAM, so a restart would
  empty it), full disk, network partitions, clock jumps, Redis (the `container-dev` profile uses the in-memory
  cache), several replicas, and the UI. Reads, goals and plan creation are the only operations in the workload. The
  workflow has not run on GitHub yet, and timing-dependent runs can flake there.

- **Blocked-reason vocabulary.** A move into `blocked` can say why (five fixed reasons, chosen with you), stored on the
  transition; blocked time is reported by reason, with `unrecorded` for moves that gave none. Verified: lint, unit
  tests (the split sums to the total; the validation rule), the full database suite (173). Not done: no screen or
  bulk-move path captures the reason (only the single-task move does), existing and imported transitions have none, the
  plan-level analysis and the aging view carry the split only through the finding text, and a request test cannot pin
  a non-empty split because test time is too short and varies by machine (CI first failed on that assumption; fixed in the next commit).

- **Goals sub-resource (FR-12).** The last "no goals sub-resource" gap: `/api/plans/{pid}/goals` (list, add) and
  `/goals/{index}` (replace, delete, plus a POST delete twin). Goals are addressed by position because `Goal` has no
  identifier and the published matcher crate was not changed. Verified: lint, unit tests, the paging census and the
  full database suite (172) pass locally. Not done: no screen; position addressing means a delete or reorder by
  another caller between a read and a write can hit a different goal (there is no `If-Match` check); the goal count is
  not capped beyond the plan's own validation. One earlier full run failed an unrelated search test that passed alone
  and on rerun; I changed my test's plan name to avoid a possible clash, and did not find the cause.

### 17 October

- **Documentation audit (remaining items).** Searched `tasks.md`, `plan.md`, `spec/`, `agents/`, `governance/`,
  `operations/` and `docs/` for old crate names, Docker commands, SVAR and old Rust versions. Fixed two current-state
  statements that were wrong: the accessibility statement and the implementation-status page still named the SVAR
  widgets (there are none; the interface uses Lily's), and the status page said 34 routes (there are 85 page routes).
  Left alone on purpose: the history in `tasks.md`, `plan.md`, `spec/13-tasks.md` and the changelogs, which describes
  what was true then. Not done: the accessibility statement's "table twin" claim for each widget was not re-verified,
  and I read only what the searches found, not every line of these documents.

- **Container file aligned with the crate.** The `Dockerfile` drops `git` from the builder (no git dependencies remain),
  takes `--build-arg VERSION` and fails the build unless it equals the crate's `version` in `Cargo.toml`, and labels the
  image (`org.opencontainers.image.version`, `title`, `description`, `source`, `licenses`, `revision`). The release
  workflow passes `VERSION` (the tag's version, already checked against the crate and the UI) and `REVISION`
  (the commit). Checked: a wrong `VERSION` (9.9.9) fails at that step with the message, and a correct `VERSION=0.1.0` passes that step
  and the release compile finishes (3 min 17 s). The image itself was NOT produced: committing the build layer
  failed with "no space left on device" in the shared Podman machine, so the labels and the runtime stage are untested
  (CI's container job builds the image and will test the Dockerfile as a whole, but passes no `VERSION`). Not done: the build source is still the repository checkout, so the image is not
  built from the published crate (that package has no `config/` files), the file is still called `Dockerfile`, and no
  `v0.1.0` tag has been pushed, so the release workflow's use of `VERSION` has not run.

- **Matcher 0.2.1 published; the service uses it.** `project-portfolio-management-matcher` 0.2.1 (this repository's
  code) is on crates.io and the service now depends on `0.2.1` instead of 0.2.0. Service clippy `-D warnings`, 1,030
  library tests, the matching test binary and `cargo deny check` pass. Not verified: the database test run against
  0.2.1 (read CI), and the container build. A published version cannot be deleted, only yanked.

- **Service and interface versions reset to 0.1.0.** `Cargo.toml` and the interface's `package.json` are 0.1.0 (they
  must agree for a release), with the version in `AGENTS.md`, `llms.json` and the status page updated. A number lower
  than the released `v0.4.0` is unusual: the `v0.4.0` tag and GitHub release (and its container image) still exist and
  are not touched, nothing was published to crates.io at 0.4.0, and the release workflow would accept a `v0.1.0` tag.
  Not done: no `v0.1.0` tag, no re-run of the build and tests (only the version strings changed), and the history
  below still says 0.4.0 where that was true at the time.

- **Redis in the test compose file.** `compose.test.yaml` now starts Redis 7 (alpine) beside Postgres: 127.0.0.1 only,
  port 6379 (`TEST_REDIS_PORT`), password `loco`, nothing saved to disk, 64 MB with LRU eviction. Checked with Podman:
  both containers report healthy, `PING` answers `PONG` with the password and `NOAUTH` without. Not done: the tests
  still use the in-memory cache (`config/test.yaml`), so nothing exercises the Redis cache provider automatically, and
  CI is unchanged (its `api-db` job does not start Redis). The root `compose.yaml` already had Redis.

- **Matcher bumped to 0.2.1, ready to publish (not published).** crates.io already holds a 0.2.0 built from the
  `main-x-service` copy of the source, which differs from this repository's code. Compared with it, the only public-API
  difference is the new `MatchConfig::validated`, so this repository's code is 0.2.1. Tests (63 unit, 7 property, 10
  public-API, 7 doctests), fmt and clippy pass. The service still depends on the published 0.2.0 until 0.2.1 is
  published. Not verified: behaviour differences between the two copies beyond their public items.

- **`integrity-mac` published and used from crates.io.** Version 0.2.0 (your `SixArm/main-x-service` repository, unchanged
  since the revision the service pinned) is on crates.io, and the service now depends on `integrity-mac = "0.2.0"` instead
  of the git revision. The crate's 21 tests and clippy pass; the service's clippy `-D warnings`, 1,030 library tests,
  `cargo deny check` and `cargo package` pass. The service is therefore packageable. Not done: `cargo publish` of the
  service, the database tests and the container build against this change, and the matcher is still 0.2.0 (the repository's
  copy has moved on and has not been republished). Nothing in the Dockerfile needed `git` any more; the install line is left.

- **Migration crate published.** `project-portfolio-management-migration` 0.1.0 is on crates.io (description, licence,
  repository and keywords added; `publish = false` removed). The service keeps `version = "0.1.0"` beside its `path`, so
  local builds use the directory and a packaged service would use crates.io. Not verified: the database tests after
  this change. `cargo package` on the service still fails on one thing: `integrity-mac` (git, no version, unpublished).
  A published crate cannot be deleted, only yanked.

- **Service uses the published matcher.** `project-portfolio-management-matcher = "0.2.0"` from crates.io replaces the path
  dependency; the Dockerfile no longer copies the matcher directory. Build, clippy `-D warnings` and the 1,030 library
  tests pass against 0.2.0. Not verified: the database tests, the container build (`podman build`), and whether the
  repository's matcher has changed since 0.2.0 in ways the service would use (the service compiles, so no API it needs
  is missing). Still not from crates.io: `integrity-mac` (git; not published) and the migration crate (path; not published).

- **Skill (`project-portfolio-management-skill`) updated** with the renamed crates, the current gates and database-test commands, Podman, `PW_PORT`, `DataTable`/Lily widgets and the `style:` directive rule, the staging and generator rules, the loco-tied dependencies and the release rule. It already said no `*.github.io` site exists, which is still true: there is no such folder to update. **Not done:** README and `index.md` were checked for stale names and versions and needed none; the other docs were not re-read in full.
- **Agent files brought up to date, and a root `AGENTS.md` and `CLAUDE.md` added.** All three project `AGENTS.md` files were already under 40 KB (19 KB, 23 KB, 5 KB); I did not shorten them. The UI guide still said SVAR DataGrid, 529 tests and 64 routes (now `DataTable`, 703 tests, 85 routes, version 0.4.0); the service guide said crate v0.3.0, Rust 1.96 and 83 migration files (now `project-portfolio-management` 0.4.0, 1.99.1 with a 1.96 minimum, 111 files, and names the renamed migration crate). The new root `AGENTS.md` (3 KB) says where to start, how work is done, the gates and the easy mistakes; `CLAUDE.md` just imports it. `llms.json` lists the root guide. **Not done:** I did not re-read the rest of those two guides line by line, so other stale details may remain (for example the route and component trees); the `agents/` folder's seven entity-level documents were not touched.
- **GitHub Actions updated (Dependabot #5), Dependabot told what to leave alone.** `actions/checkout` 7, `setup-node` 7, `upload-artifact` 7, `download-artifact` 8, `pnpm/action-setup` 6, the Docker actions (`setup-buildx` 4, `build-push` 7, `setup-qemu` 4, `login` 4, `metadata` 6), `attest-build-provenance` 4 and `zaproxy/action-baseline` 0.15, in both workflows. The ordinary CI exercises most of these on this push; **the release-only ones (`login`, `metadata`, `qemu`, `attest-build-provenance`, `download-artifact`) will first run at the next tag**, so a failure there would show up then, not now. `.github/dependabot.yml` now ignores major bumps of `axum-test` and `jsonwebtoken` (both follow `loco-rs`, which has not moved; #23 proposed them) and ignores `dtolnay/rust-toolchain` (its tag is the toolchain version and is bumped by hand with `rust-toolchain.toml`; Dependabot proposed a nonexistent 1.120.0). Closed #21 and #22; #17, #19 and #20 had closed themselves once superseded.
- **npm updates (Dependabot #22) and a new accessibility rule they brought.** Updated `@sveltia/i18n` 1.5, `@sveltejs/kit` 3.1, `svelte` 5.57.2, `vite` 8.3.4, `vitest` 5.0.5, `jsdom`, `prettier`, `svelte-check`, `@types/node`, `@playwright/test` 1.64.1 and `@axe-core/playwright` 4.14. **Not taken:** TypeScript 7.0.2, which `pnpm update --latest` selected but which breaks SvelteKit's `tsconfig` loader; it stays pinned at 6.0.3. axe-core 4.14 adds a serious rule, **label-content-name-mismatch** (WCAG 2.5.3, Label in Name: the visible text must be inside the accessible name), and it found real mismatches that the old rules missed: the calendar day cells (named by ISO date while showing "1"; now "1 October 2026, Thursday: …", and on the delivery calendar the name starts with exactly what the cell shows, the day number and its milestones), the plan links on the issues and change-requests pages (the name replaced the plan name; now "<plan name>: Open the plan's issues"), and two Welsh buttons whose names reordered the visible words ("Tynnu'n ôl" is now inside both). **Not checked:** other languages' names for the same controls (they fall back to English, which passes), and screen-reader behaviour of the new day names, which I have not tried. 703 unit tests and 602 Playwright tests pass; `deployment-moves` axe failed once in the full run and passed on repeat (the machine was loaded).
- **The migration crate is now `project-portfolio-management-migration`** (was `migration`, a name another project already holds on crates.io, which blocked publishing the service). The package, its library name (`project_portfolio_management_migration`), the service's dependency and the two `use` lines (`src/app.rs`, `src/bin/main.rs`) changed; the directory is still `migration/`, so the Dockerfile, the status script and the docs that name that path are untouched, and the `Migrator` and all 77 migrations are unchanged. Checked: `clippy -D warnings`, `fmt --check` on both crates, the 1030 library tests, the `approvals` binary and the 13 request-level `plans` tests against a throwaway Postgres, and `db status` applied all migrations to it. **Not re-run:** the whole database suite (only the parts above), and the container build, whose Dockerfile copies `migration/` by path and builds with `--locked` (the lock file is updated). **Still not publishable:** `integrity-mac` is not on crates.io, and the new migration crate has `publish = false` and no published version.
- **`authentication-verifier` now comes from crates.io (0.9.1)**, not a pinned git revision of `main-x-service` (it is the same project: 0.9.0 at that revision, 0.9.1 published 2026-10-17). Both the dependency and the dev-dependency (`test-idp`) changed; the code did not. **Full test run, against a throwaway Postgres 18 on this machine:** 1030 library tests; the 171 request-level tests (`tests/mod.rs`, `--ignored`); every other `tests/*.rs` binary as CI runs them; the DB-free binaries (`matching`, `otlp_export`, `otlp_middleware`, `licence_files`, `personal_data_inventory`, `public_files`); and, with `--features keycloak`, `keycloak_guard` and `keycloak_entra_broker` (the latter starts a Keycloak container with Podman). All passed. **Not run:** `keycloak_container` and `fluvio_relay` (they need a container runtime and a broker, which CI skips too) and the service's Playwright-free benches. **Still not publishable to crates.io:** `cargo publish --dry-run` now stops at `integrity-mac`, a git dependency that is not on crates.io, and the `migration` crate's name is taken by an unrelated crate.
- **`llms.json` and `llms.txt` rebuilt from one source.** `llms.json` is now the source (name, summary, a short facts block, and sections of links) and `scripts/generate-llms.py` writes `llms.txt` from it, so the two cannot drift; `--check` (now a step in the CI docs job) fails when `llms.txt` is stale or a listed file does not exist. New sections: install and operate, governance and security, project documents, and the AI agent skill; the facts block gives the version (0.4.0), stack, how to run and check, and the working rules. **Not done:** the facts (versions, commands) are written by hand in `llms.json`, so they can still go out of date; only the paths are checked. The older text of both files was kept where it was still right, and I did not re-read every description against its target.
- **Rust dependencies updated** (Dependabot #17 and #20, applied by hand): a compatible `cargo update` in the service (72 packages) and the matcher (`serde_json`, `proptest` 1.12), and the major bumps `sha2` 0.11, `roxmltree` 0.21, `criterion` 0.8 and `rstest` 0.27. Two source changes: `roxmltree` 0.21 added an `entity_resolver` option, so the MSPDI importer now spreads `ParsingOptions::default()` after its explicit `allow_dtd: false` (a DTD is still refused outright); and `sha2` 0.11 digests no longer print with `{:x}`, which affected one test helper. **Not taken:** `axum-test` 21, because `loco-rs` 1.2 depends on 17 and the tests mix the two (mismatched `TestServer` types), so it stays at 17.3.0 until loco moves. Checked: `clippy -D warnings`, `fmt --check`, `cargo deny check` on both crates, the matcher's tests and the service's 1030 library tests. **Not run:** the database tests, and `criterion` 0.8 was only compiled (benches not run). Closed #2, #3 and #6 (they target the long-gone `-api-with-rust` directory).
- **Renamed dependency and kanban-board 0.8.1.** `@lilydesignsystem/locale-translations` is now `@lilydesignsystem/svelte-locale-translations` 0.1.0 (published after I first found it missing; same contents, same files and exports), and `@lilydesignsystem/svelte-kanban-board` is 0.8.1. The imports in `src/lib/lily-labels.ts` and the Vitest workaround in `vite.config.ts` follow the new name; the new package still imports `./label-pack` without an extension, so that workaround is still needed. I did not read what 0.8.1 changed beyond the install. Checked: svelte-check 0 errors, 703 unit tests, 603 Playwright tests (one earlier run had two axe failures on `/organizations` that passed 45 of 45 on repeat; the machine's load average was 14 to 30).
- **Lily: calendar 0.7.0, data-grid 0.11.0, gantt-chart 0.8.0, kanban-board 0.8.0, headless 0.5.1; and new `@lilydesignsystem/locale-translations` 0.1.0.** The new package holds the grid, board, calendar, Gantt and filter labels as one pack per locale. `DataTable` now takes its words from the pack in the languages other than English and Welsh (German, Spanish, French, Hindi, Chinese, Arabic; `de-de` uses the German pack and `zh-cn` the `zh-001` pack), where before they fell back to English; English and Welsh keep the app's own wording. New `src/lib/lily-labels.ts`; a test in `tests/e2e/organizations.spec.ts` checks German shows the pack's "Exportieren" and Arabic does not show English. **Every pack but English is a machine draft that no native speaker has read** (`pack.reviewed` is false), so these labels are unreviewed; the Kanban board, calendar and Gantt still use the app's own labels in every language (only `DataTable` is wired to the pack), and `zh-cn` uses the `zh-001` pack without my having checked that it is Simplified. The package's compiled `dist/index.js` imports `./label-pack` with no extension, which Node cannot load, so `vite.config.ts` has Vitest process it (`server.deps.inline`). The kanban board gets the new `showSearchLabel` (the search field's label is now visible text too) and `fieldLayout="compact"`. **Not done:** `onTitleEdit` (editing a card title in place) because the board has no rename call wired; `canMove` with lanes (no swimlanes). **Not possible yet:** you asked for `@lilydesignsystem/svelte-locale-translations` and for kanban-board `0.8.1`; neither is on npm (the first returns 404; the latest kanban-board is 0.8.0), so the dependency is still the unprefixed name and the board is on 0.8.0. In the last two full Playwright runs different tests failed each time (ledger axe, then two scheduling tests) and passed in 102 of 102 repeats; the machine's load average was about 30, so I put those down to load, which is not proven.
- **Rust toolchain 1.99.1** (was 1.96.1): `rust-toolchain.toml`, the container build image, every `dtolnay/rust-toolchain` pin in CI and the release workflow, and the docs. The minimum supported version (`rust-version = "1.96"`) is **unchanged**; CI's MSRV job still checks it. Clippy 1.99 has a new lint, `assert_is_empty`, that flags about 115 `assert!(x.is_empty())` test assertions in the service and 2 in the matcher; its suggested rewrite (`assert_eq!(x, [] as [String; 0])`) reads worse, so it is allowed in test builds only (`#![cfg_attr(test, allow(clippy::assert_is_empty))]`, placed after `warn(clippy::pedantic)`, which would otherwise switch it back on). One `assert!(a.len() == 6)` became `assert_eq!`. Checked locally: `clippy -D warnings` and `fmt --check` on both crates and the migration crate, the matcher's tests and the service's 1030 library tests. **Not run:** the database tests and the container build; and Dependabot's suggestion to pin `dtolnay/rust-toolchain@1.120.0` was not taken (that is not a Rust release I can find; the pin must equal the toolchain).
- **Podman instead of Docker in the instructions and scripts.** Every `docker compose` in the README, `INSTALL.md`, the operations guides, the backup, restore and smoke-key scripts, the compose files' comments and the service's `AGENTS.md`/spec is now `podman compose`; "Docker or Podman" now says Podman, and the "or docker" alternatives were removed. `podman compose config` reads `compose.yaml` here. **Not changed, on purpose:** (1) **CI and the release workflow still build and run with Docker** (`docker/setup-buildx-action`, `docker/build-push-action`, `docker compose` in the smoke jobs, and the multi-architecture, provenance-attested image build in `release.yml`); switching them means replacing the build and attestation steps with Buildah or Podman equivalents, which I cannot test from here and would rather not do while the `v0.4.0` release run is in progress; (2) the files are still named `Dockerfile` (Podman reads them; renaming to `Containerfile` also touches CI, compose and dependabot); (3) image names such as `docker.io/library/postgres` are registry paths, not the tool. **Also:** `podman compose` hands off to an external provider, and on this machine that is `docker-compose`; a machine with neither `docker-compose` nor `podman-compose` installed cannot run these commands. The Rust tests that start containers (`testcontainers`) need `DOCKER_HOST` pointed at Podman's socket, as `spec/11-testing-strategy.md` already says.
- **Service renamed: the crate, the binary and the database names.** The crate is `project-portfolio-management` (was `…-service`), so Rust code imports `project_portfolio_management::…` and log targets/filters read `project_portfolio_management=info`; the `[[bin]]` is `project-portfolio-management`, so the container runs `/app/project-portfolio-management` (Dockerfile, compose, the operations guides and the fuzz crate's dependency follow); the default databases are `project_portfolio_management_test` and `…_development` (config, CI, `compose.test.yaml`, README). **Breaking for anyone already running it:** a deployment or script that calls `/app/project-portfolio-management-service`, sets a `RUST_LOG` filter by the old crate name, or points at the old database names must change; existing local databases keep their old names, so create the new ones (or set `DATABASE_URL`). **Left as they were:** the directory name `project-portfolio-management-service-with-rust`, the container image name `project-portfolio-management-service:local`, and the telemetry `service.name` default (changing it would split dashboards). Checked locally: `cargo check`, `clippy -D warnings` and `fmt --check` pass; **not run:** the database tests (no Postgres here), the container build or the fuzz crate. `v0.4.0` was tagged before this, so it still has the old names. I also set `rust-version` back to 1.96: it had been edited to 1.99 in the working tree again (see the toolchain note above), which CI's pinned 1.96.1 does not satisfy.
- **Release 0.4.0 prepared (HD-8).** The service crate and the operator interface are both 0.4.0 (the interface was 0.1.0), and the release workflow now **fails when the tag and the UI version disagree**, as the task required (it checked only the service before). The service `CHANGELOG.md`'s unreleased entries now sit under `[0.4.0]`, so the release notes are those. New `scripts/check-sbom.py` checks each SBOM against its lock file: it fails if an SBOM is empty or lists a package the lock file does not contain. **It does not prove the SBOM is complete**: the lock files hold more packages than a shipped SBOM (190 more for the service, 24 for the UI, locally), so the task's "lists every dependency" is only partly met. Found by running it: the workflow's UI SBOM step (`pnpm dlx @cyclonedx/cyclonedx-npm`) fails under pnpm ("Unexpected npmJsPath"), so it now uses `pnpm sbom --sbom-format cyclonedx`. **Not verified:** that `pnpm sbom` exists in the pnpm 11 the workflow installs (I ran pnpm 12.10 locally), and the release workflow as a whole, which only GitHub can run.
- **CI: the Postgres integration job waits up to 10 s for a database connection** (`DB_CONNECT_TIMEOUT=10000` in `.github/workflows/ci.yml`). `config/test.yaml` gives the test pool one connection and a 500 ms wait; `PoolTimedOut` failed that job twice, in two different tests, on commits that did not touch the service. **This is my reading of the cause, not a proven one**: the job passed on re-run both times, and I could not reproduce it locally (no Postgres here), so I cannot show the longer wait removes the failures; only a run of green builds will. A test that really deadlocks on the one connection will still fail, after 10 s instead of 0.5 s.
- **Gantt: choose which columns show.** The chart has no column control of its own (its columns are the time scale, set by zoom, plus any typed `fields`), so `/gantt` now gives it four optional columns between the task name and the timeline, **Kind, Stage, Start and End**, with a "Columns" group of checkboxes ("Show Kind" and so on). Start and End show by default; the choice is remembered in the browser (`localStorage`, guarded; a bad or missing value means the default). Logic is pure in `src/lib/gantt-view.ts` (`toggleGanttColumn`, `parseGanttColumns`), tested in `tests/unit/gantt-view.test.ts`, and `tests/e2e/gantt.spec.ts` hides and shows a column and reloads. The Gantt page itself is not a `DataTable`, so the table `hideable` control does not apply to it. **Not done:** moving or resizing these columns (the chart has no such option) and editing them (the chart is read-only). The two new Welsh words ("Math", "Cam") are machine-written and unreviewed. CI note: the Postgres integration job failed on 725b9c2 with `PoolTimedOut` in `mask_obligation_redacts_reads_and_exports`, a UI-free commit; an earlier run failed with the same error in another test, so this recurs. I re-ran it and have not diagnosed the cause (pool size or timeout in the test setup is my guess, unchecked).
- **Column moving is usable on the prioritization queue.** `/prioritization` now uses `DataTable`'s `grid` mode, so a column moves with Alt+Arrow on its header (or by drag) and the arrow keys move between cells; `storageKey` is set so the grid can remember the layout. New `tests/e2e/prioritization.spec.ts`: the queue lists, a column moves with Alt+Arrow, and the Explain button still opens the breakdown. **Not tested:** that the order survives a reload (the grid's `storageKey` behaviour, which I have not checked), and screen-reader announcement of a move (a message is supplied; no screen reader was used). In the full run one unrelated axe test (`start-queue`, Welsh) failed once and then passed 50 of 50 repeats; the same kind of one-off appeared earlier, so I treat it as load-related, which is not proven.
- **Gantt shades the plan's days off.** `/gantt` reads the selected plan's working calendar and shades its non-working weekdays and holidays (Lily's `weekends` and `unavailable`); a holiday carries the word "Holiday". The shading is a hatch as well as a tint, with a note under the chart saying what it means, so it is not colour alone. A plan with no calendar, or one that fails to load, is shaded not at all. Tests: `nonWorkingWeekdays`/`holidayRanges` in `tests/unit/gantt-view.test.ts`, and two in `tests/e2e/gantt.spec.ts`. **Not done:** a person's days off (the chart shows one plan, and absences are per person), and the Gantt's lag is still in calendar days, so the shading does not change any date. The new Welsh is machine-written and unreviewed.
- **Content-Security-Policy: no more blanket `'unsafe-inline'` for style attributes.** Removing it showed every route reported one violation: SvelteKit's own screen-reader announcer sets a style, and it is now allowed by its hash alone (`unsafe-hashes` + `sha256-…`). It also showed that **16 of our own components and pages set `style="…"` attributes** (bars and meters in the schedule, critical-chain, pipeline, capacity, earned-value, lifecycle, cumulative-flow and CPM Gantt views, chart widths, a few margins), which a strict CSP blocks; they now use Svelte's `style:` directive, which goes through the CSSOM and is allowed. `tests/e2e/security-headers.spec.ts` now also asserts the directive has a hash and no `'unsafe-inline'`. **Limits:** the CSP-violation tests load each route (the any-route test visits every one) but do not click through states, so a style attribute set only after an interaction, or by a Lily widget opened later, would not be caught; and I checked that no test fails, not by eye that every bar still sits in the right place. If SvelteKit changes the announcer's style string, that test fails on every route and the hash must be recomputed (the command is in `vite.config.ts`).
- **Tables: hide and show columns.** A `DataTable` of five or more columns now has a "Columns" control (one checkbox per column, "Show Kind" and so on); the row header and unnamed (action) columns cannot be hidden. `hideable` overrides the default and `storageKey` remembers the choice. Added to `tests/e2e/organizations.spec.ts`; the page's own test needed its field locators made exact, because "Show Kind" also matches a label search for "Kind". Page tests elsewhere that look up a field by a short label could hit the same clash; the full suite (596) passes, so none does today. The wording reuses existing keys, so no new Welsh.
- **Tables: sort by more than one column, and column moving where it can be done without a mouse.** Shift+click on a second header now sorts by it as a tie-break (checked in `tests/e2e/organizations.spec.ts`). `DataTable` has a `movable` prop for moving columns (Alt+Arrow on the header, or drag), **but the grid only handles Alt+Arrow in its grid keyboard model**, and no table in the app uses `grid`; so `movable` defaults to `grid` (a drag-only move would have no keyboard way) and **no table lets you move columns today**. Not tested: the move itself (my first test of it failed for exactly that reason, and I removed it rather than weaken it). To get column moving on a table, pass `grid` and accept arrow-key navigation between cells there. The new Welsh "moved to position" message is machine-written and unreviewed.
- **Lily: calendar 0.6.0, data-grid 0.10.0, gantt-chart 0.7.0, kanban-board 0.7.0**, and what they add, used: all four widgets (and every `DataTable`) now also export **TSV** and have a **Copy** button (tab-separated, for pasting into a spreadsheet); the Gantt draws a **dependency arrow** per link (hidden from screen readers; the "Depends on" text is still the description) and has a **search** box; the delivery calendar has a **search** box and opens as the **agenda below 640px**; `DataTable` sorts text by the page's language with natural number order, labels its pager as a landmark, and (when searchable) can limit the filter to one column. Labels are shared in `src/lib/table-labels.ts`. **Tried and removed:** a table-wide `emptyValue="—"`, because it replaces a column's own words for a missing value ("not recorded", "no successor named"), which three existing tests caught. **Not done:** weekend/holiday shading on the Gantt (`weekends`, `unavailable`; the chart spans plans with different calendars), column moving, multi-column sort, undo/redo (nothing is editable). The new Welsh is machine-written and unreviewed. In the full Playwright run, two ppm tests and one calendar axe test each failed once, a different one each time; none fails alone or in 120 repeats, so I judge it load-related, which is not proven.
- **Every table can be exported to CSV, TSV and Excel**, using the data grid's own export (`@lilydesignsystem/svelte-data-grid` 0.9.0), so we wrote no export code. `DataTable` gives the grid its labels, which switches on an "Export" group of three buttons on all of the app's tables. A file holds every row the filter leaves, in the current sort (not just the shown page), without hidden columns; the name comes from the table's label or `exportFilename`; `exportable={false}` opts a table out. A cell starting with `=`, `+`, `-` or `@` is written with a leading `'` so a spreadsheet does not run it, and CSV and TSV carry a byte-order mark so Excel reads Welsh correctly. Spec: new section "Exporting a table". **Not done:** exporting rows the page has not loaded, choosing columns at export time, and a selected-rows-only option. The new Welsh button group name and announcement are machine-written and unreviewed; "CSV", "TSV" and "Excel" are the same in Welsh. Added to `tests/e2e/organizations.spec.ts` (all three formats, and a formula-looking name).
- **Delivery calendar on Lily's CalendarView** (`@lilydesignsystem/svelte-calendar` 0.5.0): `/calendar` shows milestones by day, week, four weeks, month or as an agenda list, with each milestone's kind, work item and status as typed fields, a done milestone marked with a tick and the word "done", "+N more" past three a day, and CSV and Excel export. Axe found low-contrast outside-month days in the first version; fixed. With this, **no SVAR widget is left**: `@svar-ui/svelte-calendar` and `@svar-ui/svelte-filemanager` (unused) were removed, and so were the self-hosted SVAR icon font and its stylesheet link. **Not done:** the CSP still allows inline style *attributes* (`style-src-attr 'unsafe-inline'`), which SVAR needed; nothing yet shows that no Lily widget or page needs it, so it stays until that is checked on every route. Event dragging to reschedule (`onEventMove`) is not wired up, because milestones are changed on their work item. Added to `tests/e2e/calendar.spec.ts`.
- **Plan task board on Lily's KanbanBoard** (`@lilydesignsystem/svelte-kanban-board` 0.6.0): `/plans/[pid]/board` now uses it in place of the SVAR Kanban and the hand-built accessible table behind its toggle. A card moves by drag or its "Move to" menu; the board shows the card as pending, and puts it back and says so if the service refuses. Each card shows its assignee and story points as typed fields, a blocked task says "blocked N calendar days" on the card in words, and the board has a search box, empty-column text and CSV and Excel export. `@svar-ui/svelte-kanban`, `LilyKanbanTable` and the `kanbanGrid` and `gridKeyTarget` helpers were removed. **Not done:** no WIP limit on the columns, because the service has no per-column limit to show (its start-control limit is a different thing); no in-place field editing, no swimlanes. Added `tests/e2e/task-board.spec.ts`.
- **Phone-width fix on the plan page**: the row of tool links was inline links, and a label broken across two lines could push one box about 28px past the viewport, which failed `mobile.spec` in roughly 1 run in 60. It is now a wrapping flex row. 720 repeated runs of that test passed afterwards; that is good evidence, not proof, for a fault that rare. The test's failure message now names the element and its text.
- **Gantt on Lily's GanttChart** (`@lilydesignsystem/svelte-gantt-chart` 0.5.0): `/gantt` now shows one chart, `GanttView`, in place of the SVAR Gantt and the hand-built `LilyGanttTable` behind its Chart / Accessible table toggle. The chart is a keyboard- and screen-reader-first grid: a bar is a run of marked cells, each task lists its predecessors as text, the critical path is coloured and also said in the row's name ("Design (critical path)"), a task with no dates is listed with no bar, and there are zoom buttons (month, week, day) and CSV and Excel export. `@svar-ui/svelte-gantt` and the `ganttGrid` helper were removed. **Not done:** it is read-only (rescheduling by date picker needs a task-dates update in the service, not checked); the service's lag in business days is not shown, because the chart's lag is in calendar days; weekend and holiday shading (`isNonWorking`) and parent rows are not wired up; the page's phone-width list is unchanged. Added `tests/e2e/gantt.spec.ts` and `tests/unit/gantt-view.test.ts`.
- **Lily upgrades**: `svelte-calendar` 0.5.0, `svelte-data-grid` 0.9.0, `svelte-kanban-board` 0.6.0, the link, locale, search, share, text-size and theme pickers, `svelte-picker-bar` 0.4.0 and `themes` 0.5.0. No code changes were needed; all 578 browser tests and 708 unit tests pass.
- **Service advisories**: CI's `cargo deny` failed on four new rustls advisories (fixed by `cargo update -p rustls`, 0.23.46) and three zip advisories (fixed by moving `zip` to 9; `entry.name()` now returns a `Result`, so a part with an unreadable name is skipped). The `API (Postgres 18)` job on ad9cab8 failed once with `PoolTimedOut` in `saved_views_are_scoped_to_their_owner_and_their_route`, which looks like a runner flake and is not explained; it will be re-checked on this push.

### 14 October

- **Lily upgrades**: `svelte-calendar` 0.3.0, `svelte-data-grid` 0.5.0, `svelte-kanban-board` 0.3.0 and `svelte-headless` 0.5.0 (the other seven `@lilydesignsystem` packages were already current). Headless 0.5 no longer sets `aria-selected` on an active Gantt cell, because focus is not selection; our Gantt has no selection, so its unit test now checks for the tab stop and no `aria-selected`. All 578 browser tests and 708 unit tests pass.
- **Working calendar on Lily's CalendarView** (`@lilydesignsystem/svelte-calendar` 0.2.0): `CalendarMonth` (plan calendar and My calendar) now sits on it, so the calendar browses by week, four weeks or month, has a Today button, and moves with the arrow, Home, End and Page keys. Holidays are still marked by selecting a business day, and every state keeps its symbol (– off day, ✕ holiday, A absent) as well as its colour. All other `@lilydesignsystem` packages were already at their latest versions. The new button labels have Welsh text that is machine-written and unreviewed, like the rest of the Welsh. Added `tests/e2e/calendar.spec.ts`; all 578 browser tests and 708 unit tests pass.

### 13 October

- **Data tables on Lily's DataGrid** (`@lilydesignsystem/svelte-data-grid` 0.4.0, which added cell snippets, cell classes, a plain-table mode, an empty snippet and a footer): one wrapper, `DataTable` (`src/lib/components/DataTable.svelte`), now shows every data table in the app (about 95 tables on 60 screens, including the plans list, which had been on the SVAR grid). A table gets sorting on every column, an optional filter box, paging where a table is long, a named scrollable region, a caption for assistive technology, the empty text, and sort, filter and page announcements in English and Welsh. Buttons, links, forms and checkboxes sit in cells; status colours are cell tones with the word or symbol still in the text; glossary terms stay in headers. The SVAR grid and filter packages and their helper code were removed. **Left as plain tables on purpose:** the five-by-five risk matrix, the resource gap heatmap, the RACI matrix, the calendar month grid, and two-column key-and-value summaries with no header row (earned-value figures and the flow rates), because they are matrices or definitions, not tables of records. Tests were updated where a name became a row header or a table lost a hand-written class; all 574 browser tests (with axe) and 707 unit tests pass.
- **Data tables, step 1**: `@lilydesignsystem/svelte-data-grid` 0.4.0 (which adds cell snippets, cell classes, a plain-table mode, an empty snippet and a footer) and a `DataTable` wrapper (`src/lib/components/DataTable.svelte`): sorting on every column unless a column opts out, an optional filter box, paging, links and buttons in cells, tone classes for cells (a tone is never the only signal), the empty text, and sort, filter and page announcements in English and Welsh. Column headers look like text with a sort arrow. The email deliveries table is the first converted; the others follow table by table.
- **Demonstration identity**: the demonstration API now runs with login ON and trusts one PUBLISHED test key (the private half is in the repository on purpose; the stack is for fictitious data only). Signing in as example@example.com signs a PASETO v4.public token for "Example Person" (write and administrator access, a co-signing role) with that key; the seed job signs with the same key (a unit test keeps the two signers identical and the key set in compose.yaml the same key). Result, checked on a stack started from nothing: the Rust verifier accepts the Node-signed token, `/me` and `/kanban` show Example Person's 24 open tasks, a kanban move survives a reload, the seed runs with login on, and all 41 tour links open with no error and no 401. Not yet: thresholds and triage have no screens.
- **My kanban board** (`/<locale>/kanban`, on `@lilydesignsystem/svelte-kanban-board` 0.1.0): the signed-in person's open tasks across every plan (`GET /api/me/tasks`) in columns for to do, in progress, in review, blocked and done. A card moves by drag or by its Move menu (never drag-only); a move PATCHes the task's status on its own plan, and a failed move puts the card back and says so. Open tasks only, so a task moved to Done stays for the visit and leaves on the next load. In the nav and the link picker; English and Welsh (unreviewed draft); tested over stubbed answers. The browser tests now take `PW_PORT` so they can run when another project holds port 4173 (one did, and Playwright would have reused its server). Not yet: the demonstration has no signed-in identity with a person id, so `/kanban` and `/me` answer 401 there.
- **Demonstration** (`operations/demo.md`): a compose profile `demo` starts an API with login off, fills it with a fictitious portfolio (the Northgate Transport Authority: ten plans, eight people, four organizations; `scripts/seed-demo.mjs`, safe to run twice) and a UI whose sign-in accepts only `example@example.com`. Sign-in lands on a guided tour (`/demo`) of 39 features, each linked; a banner on every page says the data is fictitious. Verified on a stack started from nothing: sign-in succeeds, and all 39 tour links open without an error page. The signed-in dashboard headline is now "My Home Page" (the header offers the Sign out button, and not Sign in, once signed in; it is a button because signing out is a POST). Not covered: email, anything needing token attributes, and the screens for thresholds, triage and the start-control override (still API-only). The tour's Welsh is an unreviewed draft.
- **UI container fix**: the container's entry point is now `container-server.mjs`. adapter-node 6 assumes https unless told, so over plain http every form post was refused as cross-site (403); the earlier "ORIGIN" note was wrong. Sign-in over http now works.
- **Financials by portfolio** (found by the demonstration): the API grouped a plan by its direct parent, so in a portfolio, programme, project tree every project landed in an unnamed group per programme and the UI crashed on duplicate keys. A plan now rolls up to its nearest portfolio ancestor (a pure function with tests), and the UI keys its groups safely.
- **Lily upgrade and two new pickers**: `@lilydesignsystem` headless 0.4.0, picker bar 0.3.0, locale, share, text-size and theme pickers 0.2.0, themes 0.4.0, and new link picker 0.1.1 and search picker 0.1.0. The picker bar now has a link picker (home icon: Home, Plans, Dashboard, Milestones, Proposals, Director packs, Start queue, Organizations, Display; current page marked) and a search picker that goes to a new `/search?<terms>` page listing matching plans. The 0.4 themes place each popup by CSS anchor positioning, so the header's old `top:100%` overrides (which threw popups to the bottom of the window) are removed; a Playwright test now opens every picker at desktop and phone width and checks the popup is under its own button, inside the window and not shifting the page. The Display button hides below 64rem (it is in the link picker) so the header stays one row on a tablet. English and Welsh (unreviewed draft). Not done: the search page shows plan names only.
- **Cutover go/no-go screen** (EX-6): a plan page `/plans/{id}/cutover` (linked from the plan workspace) drafts a decision from stop criteria, publishes and freezes them, opens the decision, marks each criterion met with evidence, takes co-signatures by role, says why go would be refused, and records go, no-go or delay for safety; a new version starts a new draft. English and Welsh (unreviewed draft); tested over stubbed answers. Whether a signed-in person's token carries the co-signing role is decided by the API.
- **Deployment moves screen** (EX-5): a plan page `/plans/{id}/deployment-moves` (linked from the plan workspace) lists requests to move a task's start with the knock-on and its labour cost, requests one, and lets the approver approve or refuse with a reason. English and Welsh (unreviewed draft; the knock-on sentence does not yet handle a count of one); tested over stubbed answers.
- **Funding ledger screen** (EX-12): `/ledger` lists funding allocations with planned and untraced money and traces one to its budget lines, gates, milestones met and benefits validated. Read-only: creating allocations, mapping cost codes and reconciling a finance export stay API-only. English and Welsh (unreviewed draft); tested over stubbed answers.
- **Milestone sign-off screen** (EX-4): `/milestone-signoffs` shows milestones that need partner sign-off with delivered and met apart, the gap, and who is outstanding, and records a sign-off or a withdrawal for a chosen organization. Choosing which organizations a milestone needs is still API-only. English and Welsh (unreviewed draft); tested over stubbed answers.
- **Key people screen** (EX-11): `/key-persons` lists single points of failure with their successor and readiness, records and removes them, and marks contingent workers; resource managers only (others see the API's 404 as a message). English and Welsh (unreviewed draft); tested over stubbed answers.
- **Organizations screen** (EX-3): `/organizations` lists the partner organizations with their kind and pack audience, adds one, and retires or restores one. English and Welsh (unreviewed draft); tested over stubbed answers, not a live service.
- **Email deliveries screen**: `/email-deliveries` lists each recipient's delivery (sent, will retry, failed, with attempts and the last error) and filters by status; administrators only (anyone else gets the API's 404, shown as a message). English and Welsh (unreviewed draft); tested over stubbed answers, not a live service.
- **Start queue screen** (EX-8): `/start-queue` lists the programmes refused a start in the order they asked, with each reason (limit full, or the constraint pool short in named months), and a Withdraw button per programme. In the nav, in English and Welsh (the Welsh is an unreviewed draft). Tested over stubbed API answers (list, withdraw, Welsh, axe in three locales); not run against a live service.

### 12 October

- **UI container** (HD-2 remainder): the UI now builds with `@sveltejs/adapter-node` (adapter-auto produced no runnable server), has a Dockerfile (Node 26, non-root, health check) and a compose profile `ui` on port 3000 (this entry first said `ORIGIN` from `UI_ORIGIN`; that was wrong, see 13 October). Verified locally with podman: the stack boots with the UI healthy, `/` redirects to `/en-001`, the sign-in page carries the CSP, and an anonymous call through the UI's proxy is 401; `svelte-check`, 678 unit tests and 449 Playwright tests (with axe) pass. The container CI job gained the matching steps, which have not run on GitHub yet. Not done: sessions stay in memory (one replica) unless `OIDC_SESSION_STORE=redis`.

### 10 October

- **Change requests** (IR-4, IR-5): drafted, submitted, reviewed and decided by one named approver who is not
  the requester, with a forecast from the what-if sandbox and a baseline taken by a person; a list page.
  **Break-glass access** (RG-10), **handling marks** (RG-7) and a **shared session store** (RG-12) also landed.

- **Critical path on a large plan is 2 to 3 times faster** (RG-16, in progress): a quadratic search removed; `?only=critical` added. See [`BENCHMARKS.md`](BENCHMARKS.md).
- **The plan's task list pages** (RG-15): `limit` and `offset` with the standard headers, counts over every
  task, and a 2,000-task ceiling when no paging is asked for. Found by the new load and soak test
  (`scripts/load-test.mjs`, RG-11), whose results are in [`BENCHMARKS.md`](BENCHMARKS.md).

- **Readiness for major medical and governmental organizations:** an honest
  assessment ([`governance/production-readiness.md`](governance/production-readiness.md):
  not ready for unsupervised production, ready for a controlled pilot); an idle
  session timeout, a database-protected audit trail, a retention purge task,
  encrypted backups and a strict mode for operator-configured outbound targets;
  an assurance checklist of what only the deployer can do.

- **Issues register** (IR-1, IR-2): issues under a plan and across plans, with
  age in calendar days and an overdue flag; an `/issues` page. Change requests
  are specified (FR-14a) and planned, not built.
- **Every page in Welsh** (HD-13): all page text in a locale catalogue, a Welsh
  draft for every text, tests that fail on literal English in a template and on
  any untranslated line on any route. The Welsh has not been reviewed by a
  professional translator.
- **Sign-in with Microsoft Entra ID** (HD-12): OpenID Connect with PKCE in the
  front end's server side, directly or through a Keycloak realm; tested against a
  mock provider and a real Keycloak.
- **Operations pack** (HD-10): deployment, backup and restore scripts and a
  restore drill in CI, stated recovery objectives, upgrade guide, runbook.
- **Information governance pack** (HD-11): data protection impact assessment,
  data-flow diagram, record of processing, retention schedule, accessibility and
  language statements, a DTAC checklist, and a personal-data inventory kept true
  to the schema by a test.
- **Public-repository files:** this changelog, `NEWS.md`, `INSTALL.md`,
  `CONTRIBUTING.md`, `GOVERNANCE.md`, `MAINTAINERS.md`, `SECURITY.md`,
  `AI_STATEMENT.md`, `COMPARISONS.md`, `BENCHMARKS.md`, `RFC.md`, `CITATION.cff`,
  `CODEOWNERS`.
- **Secure by default, root CI and releases** (HD-1 to HD-5, HD-8, HD-9): login
  enforced by default, security headers and a strict Content Security Policy,
  rate limiting, a licence choice with the texts in `LICENSE/`, builds from a
  fresh clone, continuous integration, a tag-driven release workflow, and
  documents generated from the code so they cannot drift.

### 7 October

- Locales are `<language>-<region>` directories only; two-letter prefixes are no
  longer locales. A chapter of the specification on locales.

### 6 October

- Import from Jira and Asana (CSV) and Microsoft Project (MSPDI XML).
- Controlled documents, multi-step approval processes, WBS and RBS views,
  hammock activities, cross-plan task links, onboarding and offboarding,
  paging for every list.
- Per-locale `sitemap.xml` and `robots.txt`; accessibility audit of every route.

### 5 October

- Cost rates, accrual, work contours and scheduling rules; the what-if sandbox
  for schedule edits; an announcement feed and a one-screen CEO dashboard;
  display preferences (simple view, dyslexia-friendly font).

### Earlier (from 18 July)

- The plan registry, explainable duplicate matching, tasks, risks, budgets and
  governance; critical-path scheduling, earned value, resource levelling and
  Monte Carlo schedule and cost risk; RACI; key measures and objectives;
  milestones; the backlog, plan and specification that these items are tracked
  in. See the subproject changelogs for the detail and the 0.1.0 to 0.3.0
  release notes.

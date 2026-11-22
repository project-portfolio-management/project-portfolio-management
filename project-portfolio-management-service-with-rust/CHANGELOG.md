# Changelog

All notable changes to this crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> See also: [spec/index.md](./spec/index.md), [README.md](./README.md), [AGENTS.md](./AGENTS.md).

## [Unreleased]

### Changed — the simulations' generator comes from `rand_pcg`, not hand-written wrapping code (2026-11-22)

- `spec/rust-checked-arithmetic` (RCA-INT-4, RCA-EXC-3): `schedule_risk::Rng` (also used by the cost and joint risk
  simulations) was xorshift64* with a SplitMix64 seed, written with `wrapping_add` and `wrapping_mul`. It is now `rand_pcg`
  0.10 (`Pcg64`) with `rand_core` 0.10, two new direct dependencies of the domain crate that were already in
  `Cargo.lock` (via `rand`), so no new version enters. `cargo deny check` passes.
- **Behaviour change:** a seed gives a different (equally repeatable) stream, so a stored or documented result for a given
  seed will differ slightly. The unit tests that passed before still pass; none pinned a stream.
- Not done: `domain/src/leveling.rs` test code still has a `wrapping_mul` LCG (tests may, RCA-EXC-5); `tba::forecast`
  has its own small generator, not surveyed for this rule. The percentiles were not compared before and after.

### Changed — an invalid numeric setting stops start-up (2026-11-22)

- `spec/rust-checked-arithmetic` (RCA-IN-4, RCA-IN-2): `settings::validate` checks nineteen numeric environment settings
  at `App::context`; one supplied outside its range or not a whole number refuses to start and names the setting
  (service spec SET-1). Before, each reader silently used its default, the nearest bound or "off".
- **Behaviour change:** `BULK_TTL_HOURS=0`, `SCHEDULER_MINUTES=hourly`, `RATE_LIMIT` counts of `0` and the like no longer
  start the service. Blank still means absent.
- Not done: the ranges are mine and wide, and need the maintainers' review. The readers still contain their old
  `unwrap_or(default)`, now reachable only for an absent value; their `filter`, `max(1)` and "unparsable" doc comments
  were not rewritten. String and boolean settings, and settings read with other names (the `*_ENV` constants of other
  modules), were not surveyed. The `unwrap_or` in `ttl()` was not changed. No test starts the service with a bad setting.

### Changed — overflowing guards, periods and counters are errors, not bounds (2026-11-22)

- `spec/rust-checked-arithmetic` (RCA-DECL-6, RCA-IN-4, RCA-P-10): FIG-1 (a total past its type is reported at the bound)
  is withdrawn in the service spec, because a total past its type means its inputs were invalid. Thirteen sites now
  fail instead: the work-limit guards of cost, joint and schedule risk refuse a product past `usize`; the snapshot and
  scheduler tickers stay off, with an error log, when the period overflows; the reindex stops at an offset past `i64`;
  the search page end is an error; the bulk and attachment body ceilings are the cap when cap + 64 KiB overflows; the
  cumulative-flow step ends at the window's end; an idea's vote and a baseline's version answer `422` at their
  maximum; `wpm_bridge::milli_of_centi` returns `Option<i64>` and an overflow is an unknown supply figure.
- Not done: the money, minute and millisecond sums (about 90 `Figure::total_add`, `total_mul` and signed `total_sub`
  calls in the domain and service) still report the bound; each needs its own `Result` or unknown, so they come in
  later slices. The environment readers that fall back to a default for an invalid setting (RCA-IN-4) are untouched.
  Verified: lib tests and clippy; no test of the ticker's refusal, the reindex stop or the `422`s (database-gated).

### Changed — a supplied figure outside its range is refused, not clamped (2026-11-22)

- `spec/rust-checked-arithmetic` (RCA-IN-2, RCA-IN-3, RCA-IN-5, RCA-DECL-6, RCA-P-10): nine `clamp` calls on request
  figures now answer `422` naming the figure and range when the figure is supplied outside it, and take the stated
  default only when it is absent (`controllers::within`): `limit` on email deliveries, webhook deliveries, the issue
  register, announcements and the two verify endpoints; `review_days`; `months` on technology flow and DevOps metrics.
  `trials` of `0` is refused on both forecast endpoints, and the forecasts report a `reason` for out-of-range trials
  instead of clamping. Break-glass `expires_at` returns `None` outside 1 to 240 minutes (`problems` already refused them
  first). **Behaviour change:** `?limit=0`, `?limit=9999`, `?months=99` and the like were answered with a capped or raised
  figure and are now `422`. Two tests asserted the capping (`the_limit_is_clamped_at_both_ends`,
  `trials_are_clamped_rather_than_trusted`, and the `expires_at` cases for 10 000 and -3 minutes); they now assert the
  refusal, as the spec change requires.
- Not done: `clamp` on computed figures (`okr`, `fever`, `prioritization`, `schedule_risk` correlation, `deescalation::window`,
  `bulk::progress_percent`, `product_flow::flow`, `tba::outputs`), on stored or imported values (`earned_value::manual_bp`,
  `mspdi` percent complete, `insights` probability and impact, `value` rank), and the `min`/`max` bounds on external
  figures are still there. The `RCA-ENF-2` `clippy.toml` is not set. The database suites were not run, so the request-level
  `422` (including `tests/requests/oversight.rs`) is untested end to end.

### Changed — totals use declared bounds, not `saturating_*` (2026-11-22)

- `spec/rust-checked-arithmetic` (RCA-INT-4, RCA-NONE-2): about 110 `saturating_add`, `saturating_sub` and
  `saturating_mul` calls in the domain and service crates are now `Figure::total_add`, `total_sub` and `total_mul`
  (`domain/src/figures.rs`), each `checked_*` plus `unwrap_or(bound)` and declared in the service spec
  (FIG-1, FIG-2). `Page::fetch` does the same inline (a `const fn`).
- Not done: this renames and declares the existing behaviour; an overflowing total is still reported at its bound, not
  refused. No figure comes near the bound in practice. `clamp` (about 30 uses), the seeded generators' `wrapping_*`
  (`schedule_risk::Rng`, which RCA-EXC-3 wants from a vetted dependency) and `overflowing_*` are untouched; so is the
  `RCA-ENF-2` `clippy.toml`, which would forbid the old names. Verified: domain (870) and service lib tests, clippy
  `-D warnings`; the database suites were not run.

### Changed — no panicking date, time or duration operators (2026-11-22)

- `spec/rust-checked-arithmetic` (RCA-TIME-3, RCA-DOM-3, RCA-AGG-1): the `datetime` crate no longer has `date + duration`,
  `+=`, `-=`, `*`, `/`, unary `-` or `Sum` for `Date`, `NaiveDateTime`, `Timestamp`, `OffsetDateTime` and `Duration`, nor
  the constructors that panicked past the range. `Duration::weeks`, `days`, `hours`, `minutes` and `seconds` take an
  `i32` (never out of range); `Duration::try_*` take an `i64`; `Date::plus_days` and `minus_days`, `checked_neg` and the
  existing `checked_*` replace the rest. `Timestamp::from_timestamp_nanos` returns an `Option`. The crate now denies
  `unwrap_used`, `expect_used`, `panic`, `unreachable`, `todo` and `unimplemented` for non-test code, like the other three.
- About 75 call sites changed. At the edge of the date range a result is now: a stop in a day loop, a skipped item, an
  absent value (JSON `null`), or `422` naming the figure (`date_offset`, `date_before`, `timestamp_offset`,
  `timestamp_before` in `controllers/mod.rs`). Signatures that became fallible: `break_glass::expires_at`,
  `timesheets::monday_of`, `days_of` and `grid`, `status_updates::standing`, `plan_templates::shift`,
  `retention::cutoff`, `financial_year::year_of`. `Job::interval_minutes` is an `i32`. `email.rs` and `sinks.rs` back-off no
  longer use `saturating_pow`. About 55 test and bench lines were rewritten to the checked forms.
- Not done: the other RCA lints and `disallowed-methods` (RCA-ENF-1, RCA-ENF-2), about 2,100 sites, are not denied; about
  110 `saturating_*`, `wrapping_*` and `overflowing_*` and 30 `clamp` uses remain; release `overflow-checks` is not set;
  fallbacks that now stand in a few places (`Date::MAX` or `Date::MIN` for a window past the range, `unwrap_or(0)` in
  calendar code) do not yet cite a declaration (RCA-NONE-3); the new edge behaviour has no dedicated tests, apart from the
  `year_of` range edge and the `checked_neg` pair.
- Not verified: the database-backed suites (`-- --ignored`) were not run. Run: fmt, clippy `-D warnings` on every target,
  1,402 library tests, and the database-free integration tests (after the licence fix).

### Tests — the licence test allows a README symlink (2026-11-22)

- `tests/licence_files.rs`: each `LICENSES/<id>/` may also hold `README.md`, as long as it is a symlink to `index.md` (the
  repository now carries such symlinks). Any other extra file still fails. It was red since the symlinks were added.
- Not done: `spec/licenses/index.md` does not mention the symlink. Not verified beyond the six `licence_files` tests.

### Security — url 2.5.9 for RUSTSEC-2026-0592 (2026-11-22)

- `Cargo.lock`: `url` 2.5.8 to 2.5.9 and `form_urlencoded` 1.2.2 to 1.2.3 (which 2.5.9 needs). `Url::origin()` on nested
  `blob:` URLs overflowed the stack, and `cargo deny` failed CI on it.
- Not done to the usual rule: both crates were published about 40 hours before the lock, inside the 2-day minimum age
  in the maintainer's cargo config, so the lock used `CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow` for this one
  update, at the maintainer's choice. Not verified: the service tests were not rerun on the new versions (CI does).

### Changed — no panic in production code of the service, domain and migration crates (2026-11-22)

- `spec/arithmetic/index.md`: arithmetic must not panic or wrap. The three crates now deny `clippy::unwrap_used`,
  `expect_used`, `panic`, `unreachable`, `todo` and `unimplemented` for non-test code, and the 62 sites that broke it are
  gone: `Metrics::new` returns a `Result` and `Metrics::global()` an `Option` (callers use `Metrics::with`;
  `/metrics.prom` answers 500 if the set or the encoder fails), `auth::verifier()` is an `Option` (no verifier refuses the
  token), `Cursors::random` and `Cursors::of` return a `Result`, the three HTTP clients return a `Result`,
  `financial_year::year_of` returns an `Option` (the funding section answers 422 for a day at the end of the range),
  a mutex that was poisoned by a panic is recovered in `email.rs`, and `migration/build.rs` checks file names at build
  time. An invalid `PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS` now refuses to start (`App::context`) instead of panicking at
  first use. Two tests added (`epoch_is_the_first_monday_of_1970`, the `year_of` range edge).
- Not done: the `datetime` crate still has its 27 panicking sites (operators and constructors; removing them touches
  about 200 call sites) and is not denied; about 480 index and slice sites remain in the service, domain and
  datetime crates, with `indexing_slicing` and `arithmetic_side_effects` not yet denied; no panic-catching layer on the
  service; the `rag_counts`-style counters still use a bare `+= 1`.
- Not verified: the database-backed suites (`-- --ignored`) were not run, including the ones that read
  `Metrics::global().unwrap()` after this change; the handling-mark refusal at start-up has no test; the no-verifier,
  no-metrics and no-client branches are not exercised (they are not expected to be reachable).
  Run here: fmt, clippy `-D warnings` over every target, 1402 library tests, the 29 database-free integration tests.

### Tests — coverage expansion in progress (2026-11-20)

- Added focused unit and database coverage across the datetime and domain crates, migrations, authentication,
  caching, automation, relays, streaming, webhooks, truncation, canvas, collaboration, CPM, governance, oversight,
  strategy, scheduled jobs, processes, outbox/audit, live route handling, and the installed operator CLI.
- The latest complete database trace map is **94.11% (50,022/53,154)**; all runners passed. The CLI test was added and
  passed after that measurement. **Not done or verified:** 95% was not reached (475 lines remained before measuring
  the CLI test); the CLI coverage run was stopped during compilation; clippy was not rerun; the database-free
  integration gate's licence census found 18 pre-existing `.claude/worktrees/*/LICENSE.md` files outside `LICENSE/`
  (its other 25 tests passed); no threshold was added; feature-gated, load and mutation tests were not run.

### Security — CF-8: the OKR and skill-gap reads keep a private plan out (2026-11-19)

- The plan guard returns `404` to non-members on `GET /api/plans/{pid}/okr` and `…/skill-gap`. Members and
  administrators get `200`. Linking an objective to a private plan leaves the organisation-wide key-result
  and check-in responses identical. The database test is `private_routes::the_okr_and_skill_gap_reads_keep_a_private_plan_out`.
  Handling marks, canvas promotion and live worker-service calls were outside this verification.

### Security — lettre 0.11.24 for RUSTSEC-2026-0560 to 0565 (2026-11-18)

- `lettre` 0.11.23 had six advisories published on 18 November: a STARTTLS reply injection that lets a man in the middle
  forge SMTP replies, SMTP command injection through a bare CR or LF (missing dot-stuffing), mail for an address with a
  quoted local part sent to a different address, and two denial-of-service panics (an `EHLO` line of white space, and
  CR or LF in a display name). 0.11.24 fixes all six; `cargo deny --all-features check` is clean again.
- **Publish-age exception, approved by the user on 2026-11-18:** 0.11.24 was published 2026-11-17 16:58 UTC, inside the
  2-day minimum. Only `lettre`'s version and checksum changed in `Cargo.lock`; a plain re-resolve would also have
  re-picked `windows-sys`, `socket2` and `getrandom` for unrelated crates, so it was not used. A normal locked build
  accepts the lockfile.
- Tested: clippy (all targets, all features), the email unit tests, `cargo deny`. **Not verified:** sending through a
  real SMTP server, and whether any message was ever affected (the service sends to the addresses its operators
  configure).

### Fixed — the coverage job's database runs get 30 minutes (2026-11-18)

- `tarpaulin-db.toml`'s `timeout` is for a whole run, not one test, and the instrumented request suites took over 10
  minutes on a GitHub runner, so the coverage job stopped at 600 s ("Timed out waiting for test response") on its
  second run, part-way through `requests::`. The two database runs now have 1,800 s. The tests themselves passed in the
  `api-db` job. **Not verified:** the next run's time.

### Changed — the codes and fields a screen reads (NC-1 slice 12 part a, 18 November 2026)

Service spec, "Codes and fields a screen reads (NC-1 slice 12, part a)", is the contract for each item.

- **`threshold_kind` on every delivery exception** (`GET /api/exceptions` and the other answers built the same way):
  `confidence_red_packs`, `forecast_past` or `p80_cap` for one a threshold opened or joined, null for one opened by
  delivery confidence. The column was stored (EX-7) but not returned. Tests: `tests/organizations.rs`,
  `tests/delivery_confidence.rs`. **Not changed:** there is still no `GET /api/exceptions/{pid}`.
- **Each refusal its own code.** `POST /api/scorecards`: 409 `conflict` becomes `scorecard_name_taken` (a name taken)
  or `tracker_scorecard_taken` (another scorecard is the tracker's). The proposal pipeline: 422 `unprocessable` becomes
  `proposal_status_refused` (wrong status, on `submit`, `review`, `approve`, `reject` and `promote`) or
  `tradeoff_required` (approving an external ask with no named trade-off; the rule is now
  `governance::needs_named_tradeoff` in the domain crate). Statuses and `detail` text are unchanged.
  `POST /api/proposals/{pid}/approve` gains a hand-written OpenAPI operation naming both. Tests:
  `only_an_external_ask_needs_a_named_tradeoff`, `tests/organizations.rs`. **Breaking** for a client that matched
  the old codes; none in this repository did. **Not changed:** the other 422s of the proposal routes (a bad body,
  `PUT …/tradeoff`'s refusals) keep `unprocessable`.
- **`start_refused` carries its reasons as fields.** The 422 from `PUT /api/plans/{pid}/phase` adds the problem's
  `errors` member, `{ refusals, queue_position }`, in the shapes of `GET /api/plans/{pid}/start-check` (`limit_reached`
  with `in_delivery` and `limit`; `constraint_pool_over` with `pool` and `months`); the English `detail` is unchanged
  and now built by `start_control::Refusal::message` in the domain crate. Tests:
  `a_refusal_carries_its_reason_as_a_code_and_its_numbers`, `tests/organizations.rs` (the limit case). **Not
  verified:** the `constraint_pool_over` case through the route (the domain test covers its shape).
- **The de-escalation tracker names who acted.** `GET /api/deescalation` returns `set_by` on the level and the level
  history, and `recorded_by` and `recorded_on` on criterion evidence, as stored. Checked first: both people columns are
  in `governance/personal-data-inventory.csv` and covered by the data-subject tools, and other reads already return
  such references (an exception's `opened_by` and `decided_by`). The director pack's copy of the tracker
  (`pack_section`) removes them. Test: `tests/organizations.rs`. **Not done:** the references are token subjects, not
  names; the service holds no directory to turn one into a name, so a screen shows the reference (the same limit as a
  confidence assessment's assessor).
- **A KPI made from the scorecard template keeps the measure's key.** Migration `20261118000116_kpi_template_key` adds
  `kpis.template_key` (nullable, checked to a lower-case key; a down file drops it); `from-template` stores it, and every
  KPI answer returns it (`GET /api/kpis`, `/api/kpis/{pid}`, `/api/scorecards/{pid}`, the tracker). Chosen over
  translating the template in the service: the service answers in English, a client names a measure from its own
  catalogue by key, and the column is the small honest fix. Tests: the domain template test checks every key fits,
  `tests/organizations.rs` reads the key back. **Not done:** KPIs made from the template before this migration have
  no key (nothing is backfilled: a name match could be wrong); the template answer's `name` stays English; the plan
  templates' starter names and text are English too (BR-13), untouched.
- **The ledger trace's untraced reason as a code.** `money.untraced_reason_code` (`no_readable_line`, `not_on_a_line`,
  `lines_over_plan`, or null) beside the English `untraced_reason`, both from `ledger::untraced_reason` in the domain
  crate. Tests: `untraced_money_says_why_as_a_code`, `tests/organizations.rs` (`not_on_a_line`). **Not verified:** the
  other two codes through the route (the domain test covers them).

### Added — test coverage with cargo-tarpaulin, and its baseline (2026-11-18)

- **Tool and engine.** cargo-tarpaulin 0.37.6 (published 2026-11-05, past the two-day publish age; installed with
  `--locked --version`), LLVM engine on macOS and Linux: macOS has no other, the figures then compare, and the
  `own_process!()` suites run tests in child processes, each of which LLVM's runtime profiles (`%m-%p`) where ptrace
  would need `--follow-exec`.
- **Configuration.** `tarpaulin.toml` (read by `cargo tarpaulin` itself): two runs merged into one report, `--workspace
  --lib` and `--test main`. `tarpaulin-db.toml` (`--config`): the same and the two database runs split as `api-db`
  splits them (`requests:: --ignored --test-threads=1`; `--ignored --test-threads=1 --skip requests:: --skip
  load_test::`). Every run builds into `target/tarpaulin` with `skip-clean` (tarpaulin's default deletes
  `target/debug` before each run). Left out of the denominator, each with its reason in a comment: `tests/*`,
  `domain/tests/*`, the three `benches/` directories, `fuzz/*`, `migration/build.rs`, the generated
  `src/openapi_bodies.rs`, the unit-test module files `src/auth/tests.rs`, `src/auth/area_tests.rs`,
  `src/tba/tests.rs` and `src/datetime/golden.rs`, and `target/*`. Reports (HTML, lcov, JSON) go to
  `target/coverage/`.
- **Commands.** `make coverage` and `make coverage-db` (this directory's `Makefile`), each followed by
  `../scripts/coverage-summary.py` (totals by crate; `--least N` for the least-covered files, `--markdown` for CI).
- **CI.** A `coverage` job in `ci.yml`: Postgres 18 as in `api-db`, `cargo tarpaulin --locked --config
  tarpaulin-db.toml`, the summary printed and added to the job summary, `target/coverage` uploaded as `coverage-api`.
  No `fail-under`; not in `ci-ok` (COVERAGE.md says why). Actions pinned by SHA as the other jobs are.
- **Test.** `tests/coverage_config.rs` (DB-free, in `tests/main.rs`): every run section of both files sets the LLVM
  engine, `skip-clean` and `target/tarpaulin`, and excludes exactly the same files, and every excluded path but
  `target/*` exists.
- **Baseline** (`COVERAGE.md`, 2026-11-18, macOS, a throwaway database): with the database suites 91.9% (48,968 of
  53,272 lines; domain 96.3%, service 91.2%, controllers 92.0%, migration 88.3%, datetime 73.8%); without them 33.7%
  (controllers 3.8%). `make coverage` took 4 minutes with the instrumented build, `make coverage-db` 9 minutes 46
  seconds after it.
- **Not done:** a threshold; the tests COVERAGE.md says are most needed; coverage of the `keycloak` and `fluvio`
  features, the container and broker suites, `load_test` and doctests; branch coverage (tarpaulin has none). **Not
  verified:** the CI job itself (not yet run on GitHub), and so its run time on a runner.

### Changed — the benchmarks where their code lives, and the datetime crate's calendar arithmetic cheaper (2026-11-18)

- **Bench layout.** Before: `benches/service_bench.rs` (the request path) and `benches/calculation_bench.rs` (the
  engines, the row structs' JSON and the date work), both in the service crate, so measuring a pure engine compiled
  axum, axa, sqlx and the whole service. After:
  - `domain/benches/calculation_bench.rs` (`cargo bench -p project-portfolio-management-domain --bench
    calculation_bench`): `schedule_risk`, `cost_risk`, `leveling`, `schedule_health`, `mspdi`, `earned_value`, and the
    `timestamps` rows of the domain modules (Jira and flexible date parsers, ISO weeks, months, business-day index,
    report schedules, financial years, absence expansion). Dates are read with `Date::parse_from_str(s.trim(),
    "%Y-%m-%d")`, exactly what `plan_templates::full_date` (a service function) does, so the fixtures are unchanged.
  - `datetime/benches/datetime_bench.rs` (`cargo bench -p project-portfolio-management-datetime --bench
    datetime_bench`): `timestamps/decode_iso_dates_1000`, `encode_iso_dates_1000`, the two `rfc3339_instants_1000`
    rows and `full_date_1000` (moved), and the new `date_ops` and `duration_ops` groups: weekday, day steps inside
    and across a month, `Date - Date`, `monday_of`, the seven days of a week, `from_isoywd_opt`, `iso_week`, month
    steps, `from_ymd_opt`, a month's last day, ordinal, `succ_opt`/`pred_opt`, comparison and a holiday lookup, each on
    one date and on a thousand, with jiff's own weekday, day step and difference beside them for attribution.
  - `benches/service_bench.rs` gains the `timestamps` group of `plans`, `tasks` and `audit_logs` row JSON.
  Every benchmark ID is unchanged. `criterion` 0.8 (locked at 0.8.2, `html_reports`) is a dev-dependency of the
  domain and datetime crates too; `[[bench]]` entries moved with the files. The raw-pool census (`src/db.rs`) now also
  reads `domain/benches` and `datetime/benches`. The Dockerfile copies `domain/` and `datetime/` whole, so their
  `benches/` come with them (its comment says so).
- **Faster calendar arithmetic, nothing observable changed** (`datetime/src/date.rs`). `Date::weekday` reads jiff's
  weekday (computed from year, month and day) instead of going through a Hinnant day number; the private day number,
  `days_since` and the cross-month branch of `add_days` use jiff 1.0's day numbers (Neri-Schneider) through
  `duration_since` and `checked_add`. A step jiff refuses because it is longer than jiff's range of day numbers (from
  -9999-01-01 across the whole range) is retried through the absolute day number, so it still lands. Measured old
  and new alternately (BENCHMARKS.md): `weekday` 0.30 times its old time, a cross-month step 0.50, `from_isoywd_opt`
  0.62, a month's last day 0.30; in the domain functions, `report_schedules::next_after` monthly 0.38, the
  business-day index 0.56, `calendars::expand_absences` 0.64, an ISO week to its days 0.85.
- **Tests.** `jiff_day_numbers_agree_with_the_algorithms_they_replaced_on_every_date` keeps the old Hinnant
  algorithms as references and checks the day number, `from_epoch_day`, the weekday and `days_since` on every one of
  the 7.3 million dates from -9999-01-01 to 9999-12-31, day steps of ±1, ±7, ±30, 365, -366 and ±10,000 on every
  eleventh date and every date within 20,000 days of either end, and steps off both ends. It takes about 7 seconds in
  a debug build. The golden tests (`src/datetime/golden.rs`) and the crate's other tests pass unchanged.
- **Not done:** the same-month step is about 0.4 ns a date slower in the new build (inlining; `days_of_week` 1.12,
  `add_one_day` 1.22), and an ISO week to its seven days is still about twice chrono's. The service's
  `service_bench` still holds groups (`cpm`, `validation`, `merge`, `capacity_rollups`, `public_roadmap`,
  `dhcw_import`) that reach only domain code; they were left where they are, as the request path. **Not verified:** on
  a quiet machine (load 10 to 20 for every run); against chrono on the new code (chrono is no longer in the tree).
- **Fixed on the way:** `Date::checked_add_days(Days::new(i64::MAX as u64))` panicked with an overflow in a debug build
  (the same-month check added unchecked); it is `None`, as a release build already gave.

### Added — a milestone's `signoff` names every required organization (EX-4, 2026-11-18)

- The `signoff` object on `GET /api/milestones` and `GET /api/milestone-signoffs` gains `required`: every organization
  the milestone requires (`organization_pid`, `name`, `signed_on`, null while outstanding), in the stored order.
  `PUT …/signoff-required` replaces the whole set, and `outstanding` drops an organization once it signs, so until now
  no client could show the set or replace it without dropping one that had signed. Spec, "Milestone sign-off by partner
  organizations (EX-4)"; the OpenAPI description says it. Tests: `the_required_set_is_kept_whole_signed_or_not`
  (domain), and `tests/organizations.rs` checks the set once one has signed.
- **Not changed:** the order is the stored order, which is not the order the request gave (the set is de-duplicated
  through a hash set). **Not verified:** the field on `GET /api/milestones/{pid}`, which is built the same way.

### Changed — from `chrono` and `chrono-tz` to `jiff` 1.0, with nothing observable changed (2026-11-17)

- **Why:** RUSTSEC-2026-0543 (chrono) and RUSTSEC-2026-0555 (chrono-tz), unmaintained. The service's own code, sqlx's
  `chrono` feature, schemars' `chrono04` feature and `chrono-tz` are gone; so are `phf` and `phf_shared`, which only
  chrono-tz used. `cargo tree -i chrono -e normal` prints nothing; with `--all-features` it prints only `fluvio`.
- **A fourth workspace crate, `project-portfolio-management-datetime` (`datetime/`),** holds the types, over jiff:
  `Timestamp` (was `DateTime<Utc>`), `OffsetDateTime` (was `DateTime<FixedOffset>`, the `timestamptz` column type, so
  `models::_entities::DateTimeWithTimeZone`), `Date` (`NaiveDate`, and the `Date` alias), `Duration` (`TimeDelta`),
  `Days`, `Months`, `Weekday`, `IsoWeek`, `NaiveDateTime`, `NaiveTime`, `LocalResult` and `Tz` (`chrono_tz::Tz`). They
  keep chrono's method names where the service called them, so 215 files changed mostly in their imports. The service
  and the domain crate both name them `crate::datetime::…` (the service's `src/datetime/mod.rs` and the domain's
  `lib.rs` re-export the crate); the domain crate uses it without its `sqlx` feature, so it stays free of sqlx (root
  spec §8.3c). Service spec §8, "Dates and times", is the contract.
- **Golden tests first.** `src/datetime/golden.rs` (14 tests, written on chrono 0.4.45 and chrono-tz 0.10.4 and
  committed before any type changed) passes unchanged on jiff; only its helpers, the lines that name the types, were
  edited.
- **The serde helpers and readers** (`datetime/src/`): `instant.rs` and `date.rs` serialize as chrono did (`Z` for a
  zero offset for both instant types, the fraction in groups of three, the offset to the minute rounded to the
  nearest, years outside 0..=9999 with a sign) and give the same `Display`, `Debug`, `to_rfc3339`, `to_rfc3339_opts` and
  serde `expecting` messages; `parse.rs` is a port of chrono's relaxed RFC 3339 reader (request bodies, `str::parse`),
  its strict one (`parse_from_rfc3339`, query parameters), its `NaiveDate` reader and its `%Y %m %d %H %M %S %.f %#z`
  patterns (`%Y-%m-%d`, `%d/%m/%Y`, the database's text form), with chrono's order of checks and its error messages;
  `format.rs` writes the `%Y %m %d %H %M %S` patterns the service uses; `schema.rs` gives the inline
  `{"type":"string","format":"date-time"}` / `"date"` schemas of schemars' `chrono04` (its `jiff02` feature is for jiff
  0.2); `postgres.rs` (feature `sqlx`) encodes `timestamptz` (microseconds since 2000, sub-microsecond digits dropped
  toward zero) and `date` (days since 2000) as sqlx's `chrono` feature did, by OID, with the same text decoding;
  `zone.rs` reads zones only from jiff's bundled database (`TimeZoneDatabase::bundled()`) and matches names exactly.
- **Checked against chrono in a scratch crate outside the repository** (depending on chrono 0.4.45, chrono-tz 0.10.4
  and this crate, so chrono is not a dev-dependency here): 28.4 million comparisons, no unexplained difference.
  Random and mutated text through every reader (relaxed, strict, date, the three patterns, serde, non-string JSON);
  random instants with random offsets through JSON, `to_rfc3339`, `to_rfc3339_opts` (every form, with and without
  `Z`), `Display`, `Debug`, the seven patterns, unix counts, sub-second parts, fields and round trips; date arithmetic
  (months both ways, days both ways, a duration's whole days, differences, weekdays, ordinals, ISO weeks and
  `from_isoywd_opt`, days from CE both ways, `succ`/`pred`, `with_day`/`with_month`, leap years); durations (every
  count, `Debug`, `Display`, `*`, `/`, `to_std`, `abs`); and every zone's offset at 2,900 instants from 1800 to 2100
  and its civil-time lookups (gaps and folds). Two bugs this found were fixed before commit (an ISO week's `Debug`
  padding, and a day count past 2.9 million days that jiff takes in one step).
- **Where behaviour could have differed, and what was chosen:** month arithmetic clamps the day, as chrono did (ported,
  not jiff's spans); a `Duration` added to a date moves it by whole days toward zero; `Date - Date` is whole days;
  `num_days`/`num_hours`/… truncate toward zero; `timestamp`, `timestamp_millis` and `timestamp_micros` round down
  before 1970 (jiff's `as_millisecond` truncates; floor is computed instead); a `Duration` keeps chrono's range
  (`i64::MAX` milliseconds) and its `/` keeps chrono's rounding of a sub-nanosecond quotient; `OffsetDateTime`
  compares and hashes by instant, across both types, as chrono did; a zone gap is `LocalResult::None` and a fold is
  `Ambiguous(earlier, later)`, as chrono-tz gave (`planner::day_window` walks a gap in 15-minute steps as before);
  zone names are case-sensitive (jiff's lookup is not); a zone's `Debug` and `Display` are its name and its parse error
  is `failed to parse timezone`, as chrono-tz's; `Weekday`'s `Display` pads like chrono's. Each is pinned by a test in
  `datetime/src/` or `src/datetime/golden.rs`.
- **The unavoidable differences** (jiff cannot represent them; documented in service spec §8): a leap second (`:60`) is
  read as `:59` with its fraction (chrono kept `:60`); a year outside -9999..=9999, or an instant within about a day of
  those ends, is out of range (chrono went to ±262,143), and `NaiveDateTime::and_utc` saturates there rather than fail,
  so `?to=9999-12-31` still means "to the end"; a stored date or instant outside that range (`infinity`) is a decode
  error, where chrono panicked. **The zone data:** chrono-tz 0.10.4 carries tzdb 2025b, jiff-tzdb 1.0.0 tzdb 2026f
  (rearguard form). Compared zone by zone, 1970 to 2099 differ: from late 2026, `America/Vancouver`, `Edmonton`,
  `Winnipeg`, `Inuvik`, `Yellowknife`, `Rainy_River` and their `Canada/*` aliases (2026f keeps them on daylight time),
  `America/Havana` and `Cuba`, `Africa/Casablanca` and `El_Aaiun`; and `America/Tijuana` (with `Ensenada`,
  `Santa_Isabel`, `Mexico/BajaNorte`) in some 1970s summers. Before 1970, `EST5EDT`, `CST6CDT`, `MST7MDT`, `PST8PDT`
  (separate zones in rearguard data, links in 2025b) and `Europe/Dublin` around 1925; from 2100, chrono-tz stops
  applying daylight saving rules and jiff does not. The two accept the same 597 names; jiff-tzdb also lists `Factory`,
  which is refused, as before. A caller in those zones now gets the newer rules' "today" and slot times.
- **Dependencies:** removed `chrono` and `chrono-tz` (service and domain), sqlx's `chrono` and schemars' `chrono04`
  features, `phf`, `phf_shared`. Added `jiff-tzdb` 1.0.0 (the bundled zone database: the reason the image needs no
  `/usr/share/zoneinfo`); `jiff` 1.0.0 was already in the lockfile through axa, now with `default-features = false`
  and `std`, `tzdb-bundle-always`, `tz-fat`, `perf-inline` (no `tz-system`). No other crate moved.
- **`deny.toml`:** the RUSTSEC-2026-0555 (chrono-tz) exception is removed; RUSTSEC-2026-0543's reason is narrowed to
  axum-test's `expect-json` (tests only) and the optional `fluvio` feature; RUSTSEC-2026-0542 (humantime, fluvio)
  stays. `governance/security-policy.md` §16.4 says the same.
- **Guards:** `clippy.toml` bans jiff's ways to the process zone or its global database (`ZonedDateTime::now`,
  `TimeZone::system`, `TimeZone::try_system`, `TimeZone::get`, `tz::db`, and `in_tz` on `Timestamp`, `Date` and
  `DateTime`) in place of `chrono::Local`; `tz.rs`'s source guard keeps the chrono needles and adds the jiff ones and
  `Timestamp::now().date_naive()`, and both its guards also read `datetime/src`.
- **Tested:** `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` (and
  without `--all-features`); `cargo test --workspace --lib` (455 service, 849 domain, 16 datetime, 7 migration); `cargo test --test main` (25, 325 ignored);
  the database suites as CI runs them, each on a throwaway database (`requests::` 232; the rest, without `load_test::`, 92); the UI's `npx vitest run` (1236, 3 skipped), as its tests read two changed domain files; `cargo deny --all-features check` (advisories, bans, licences and
  sources ok). `openapi.json` and `asyncapi.json` are unchanged (their pinning tests pass; `git diff` shows no change),
  and `python3 scripts/generate-openapi-bodies.py --check` passes. `benches/calculation_bench.rs` compiles and runs
  unchanged, its fixtures too; its `timestamps` group, chrono and jiff back to back (`BENCHMARKS.md`): JSON encoding a quarter to a third faster, instants written in under half the time, the date parsers about 45% faster, and calendar arithmetic (business-day index, ISO weeks, report schedules, absence expansion) 2 to 3 times slower, 14 to 40 ns more per date. A first build was up to three times slower than chrono on the serde paths; the writers now fill a stack buffer, the two common patterns use fixed steps, day moves within a month and ISO weeks avoid jiff's checked arithmetic, dates compare as one packed integer, and the small methods are `#[inline]` across the crate boundary.
- **Not verified:** the container image build (not run; the `Dockerfile` copies `datetime/`); the real-token test
  binaries outside `tests/main.rs`, the Keycloak and Fluvio container tests and the load test (not run); a request
  against a running service, a log line from one, or a PDF/CSV/MSPDI export byte-compared with the chrono build
  (covered only by the unit, golden and database tests); the tzdb differences above against any source but the two
  databases themselves.

### Changed — axa 0.20 and axa-test 0.11 (2026-11-17)

- `axa` 0.19 to 0.20, `axa-test` 0.10 to 0.11; nothing else in the lockfile moved. axa's Zstd support is now behind a
  `zstd` feature, off by default; the service leaves it off (nothing in the service, its specs or tests uses Zstd), so a
  `zstd` request body is a `415` and a `zstd`-only `Accept-Encoding` gets an uncompressed answer. Gzip, Brotli and
  deflate are unchanged. Upgrade notes: `operations/upgrade-and-migration.md`.
- **Publish-age exception, approved by the user on 2026-11-17:** axa 0.20.0 and axa-test 0.11.0, published minutes
  before, resolved with `CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow`; nothing else was added.
- **Not verified:** a client that sends Zstd against a running service.

### Fixed — every HTTP client installs the TLS provider first (2026-11-17)

- `reqwest` is built with `rustls-no-provider` and `ring` was installed only in `App::context`, so 14 unit tests and 2
  integration tests (anything that builds a client without booting the app: `workers_client`, `wpm_bridge`, webhooks,
  the PASETO key fetch, the OTLP and trace tests) panicked with "No rustls crypto provider is configured". Each place
  that builds a client (`workers_client`, `wpm_bridge`, `webhooks`, `auth::oidc`, `auth::verifier::fetch_or`, and the
  integration tests that make their own) now calls `app::install_tls_provider()` first; it is idempotent, so the boot
  call is unchanged. Tested: `cargo test --workspace --lib` (1,297), `cargo test --test main`, `keycloak_guard`.
  **Not verified:** the Keycloak container and Entra broker tests (they need containers) and the load test.

### Changed — the pure rules move to a crate of their own, `project-portfolio-management-domain` (2026-11-17)

- A pure refactor: no route, answer, OpenAPI or AsyncAPI document, migration or test result changes. 121 top-level
  modules of `src/` (about 47,300 lines with their tests), the ones that, with everything they import through
  `crate::`, use none of `axum`, `axa`, `sqlx`, `AppContext`, `crate::db`, the controllers, the models or `app`
  (comments ignored), move to `domain/src/`, a new workspace member: package `project-portfolio-management-domain`, lib
  `project_portfolio_management_domain`, 0.2.0, edition 2024, `rust-version` 1.98, the service's licence, authors and
  lints, `publish = true` as the service. Among them `cpm`, `evm`, `earned_value`, `schedule_risk`, `cost_risk`,
  `leveling`, `schedule_health`, `tolerances`, `prioritization`, `fever`, `buffers`, `ledger`, `cutover`, `mspdi`,
  `validation`, `currency`, `tz`, `csv_cell`, `privacy`, `merge`, `governance` and `openapi_more`. The layering rule is
  root spec §8.3c: the domain crate may not depend on the service, `axum`, `axa`, `sqlx` or `tokio`.
- `src/lib.rs` re-exports every moved module under its old name (`pub use project_portfolio_management_domain::{…}`),
  so `crate::cpm` in the service, `project_portfolio_management::cpm` in `tests/` and `benches/`, and doc links are
  unchanged; no call site was edited. No `pub(crate)` item needed to become `pub`: nothing restricted in a moved module
  is used from the service.
- The domain crate's dependencies: `serde`, `serde_json` (with `float_roundtrip`, as the service), `schemars`, `chrono`,
  `chrono-tz`, `uuid`, `sha2`, `zip`, `roxmltree`, `url` and the matcher; `tempfile` for its tests. `sinks` parses
  destinations with `url::Url`, the type `reqwest::Url` re-exports, so the crate needs no HTTP client. The service
  drops `zip` and `roxmltree`, which only the moved `extract` and `mspdi` used (`cargo machete` clean).
- Guards that read the source tree read both crates: the raw-pool census in `src/db.rs` (also `domain/tests`), the two
  audit censuses in `models::audit_logs`, the extractor guard in `src/lib.rs`, the live-event census in `asyncapi`; the
  moved time-zone and CSV-escaping guards in `tz` and `csv_cell` read `domain/src` and the service's `../src`.
  `clippy.toml`'s `chrono::Local` bans reach the domain crate (clippy reads the file from the parent directory; a
  probe use of `chrono::Local::now` in a domain module was refused). The golden meeting PDF moves to
  `domain/tests/fixtures/` with its only test; the DHCW roadmap fixtures stay and the domain test reads them from
  `../tests/fixtures/`. 24 doc links from domain modules to service items (`crate::controllers::fever` and so on) are
  plain code spans now, as they cannot resolve from the domain crate.
- Tooling: `scripts/generate-openapi-bodies.py` looks for `JsonSchema` request types in `domain/src` too (without that
  its `--check` reports `openapi_bodies.rs` stale); `scripts/generate-status.sh` counts `domain/src` tests (the counts
  are unchanged, 1,652 and 329 ignored); CI's `api` job, the `Makefile` and the documented gates use `cargo fmt --all`,
  `cargo clippy --workspace` and `cargo test --workspace --lib`; the `Dockerfile` copies `domain/`.
- **Left in the service, and why.** Pure but tied to a service module: `blueprint_sync` and `starter_templates`
  (through `plan_templates`), `plan_templates` and `task_bulk` (through `workflow`), `workflow` and `metrics`
  (through `tba`), `openapi_schemas` (through `openapi`); `compliance` and `openapi_bodies` use the models and
  controllers directly. Pure by the rule but kept on purpose: `live` and `content_check` (a `tokio` broadcast channel
  and a `tokio` TCP client: I/O, not domain rules), `asyncapi` (its tests read the service's own sources and
  `asyncapi.json`), and the empty `data` and `initializers` extension points. The controllers, models, `db`, `app`,
  `auth`, `search`, `tba` and the rest stay.
- Build times, on one machine with a warm build, before and after, touching one file (a comment appended):
  the machine was never quiet (load average 100 to 285 on 16 cores, from other agents' builds; Cargo runs through
  the `kache` wrapper, whose daemon does much of the compiling, so `user` time undercounts). Wall time, with `user`:

  | Touched | Before: `cargo build` | Before: `cargo test --no-run` | After: `cargo build` | After: `cargo test --no-run` |
  | --- | --- | --- | --- | --- |
  | `src/controllers/fever.rs` (two runs) | 9 m 43 s (3 m 18 s); 1 m 15 s (30 s) | 7 m 54 s (10 m 53 s); 2 m 10 s (2 m 42 s) | 10 m 24 s (2 m 34 s); 3 m 39 s (31 s) | failed (kache daemon `EAGAIN`); 3 m 45 s (2 m 55 s) |
  | `cpm` (one run) | 2 m 05 s (32 s) | 3 m 29 s (2 m 54 s) | 3 m 13 s (53 s) | failed (kache daemon `EAGAIN`) |

  At this load the numbers do not show the gain, or a loss: the spread between two runs of the same build is larger than
  any difference. What did change, from Cargo's own output: after a controller edit Cargo recompiles only the service
  crate, whose `src/` is now about 105,500 lines instead of about 152,800, and the domain crate stays fresh; after an
  edit to a domain module it recompiles the domain crate (about 47,300 lines) and then the service. A measurement on a
  quiet machine is still to do.
- Tested: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
  `cargo test --workspace --lib`: 441 (service) + 849 (domain) passed, 1 ignored, which is the 1,290 passed and 1
  ignored the service library ran alone before, plus the migration crate's 7; `cargo bench --no-run --all-features`;
  the database suites on Postgres 18 (`tests/mod.rs` 232 passed, and every real-token binary as CI's `api-db` job runs
  them: 61 binaries in all, `mod` included, 324 passed, none failed); the DB-free binaries; `cargo deny --all-features check` (advisories, bans, licences, sources
  ok); `cargo machete`; the docs checks. The container image was **not** built with Podman: at that load a release build in the Podman machine was not practical; the `Dockerfile` change (one `COPY` of `domain/`) is checked by reading only.
- **Not verified:** the CI jobs on a GitHub runner; `cargo publish` of the domain crate (it is not published, and the
  service's `publish` would now need it on crates.io first); the release workflow's per-crate SBOM step with a third
  workspace member; `cargo doc` was not run as a gate (none exists).

### Changed — the integration tests are one binary (2026-11-17)

- **`tests/main.rs` is the one integration-test binary** (`autotests = false`, `[[test]] name = "main"`). The 69
  `tests/*.rs` files stay where they are and are its modules; the old `tests/mod.rs` root is gone and `tests/requests/`
  is the module `requests`. `tests/change_requests.rs` is the module `change_request_rights`, so that the filters
  `requests::` and `change_requests::` each select one module (a libtest filter is a substring). `otlp_collector` is
  one shared module. Run: `cargo test --test main` (database-free), `cargo test --test main requests:: -- --ignored
  --test-threads=1`, then `cargo test --test main -- --ignored --test-threads=1 --skip requests:: --skip load_test::`;
  one former binary is a module filter (`approvals::`).
- **Process isolation is kept where it was needed, without a second binary.** `own_process!()` (`tests/own_process.rs`),
  the first statement of 89 tests in 60 modules, re-runs this same executable for that one test (`--exact
  --include-ignored --test-threads=1`) and fails unless the child reports exactly one test run and passed. Why each
  needed it: the auth posture (`REQUIRE_AUTH`) and the ABAC policy are `OnceLock`s read once per process, and most
  real-token suites turn enforcement on while the request suites need it off; `handling_marks` and `public_reader` set
  different mark schemes; `webhook_redelivery` the MAC key; `outbox_audit` and `fluvio_relay` the event transport;
  `oidc_provider` and the `keycloak_*` suites their verifiers; `hardening` the rate limits and CORS list;
  `trace_span_concurrency` and `otlp_middleware` install a global `tracing` subscriber, which is what they test (a
  thread-local `set_default` would not reach the runtime's worker threads); and all of them set environment variables.
  Not isolated, as nothing of theirs outlives a test: `requests`, `matching`, `licence_files`,
  `personal_data_inventory`, `public_files`, `load_test`, `otlp_export` (already `with_default`), `migrations` (its own
  scratch databases) and `sqlx_layer`. No test kept a binary of its own. Nothing in `src/` changed.
- **Counts unchanged:** 354 tests before and after (26 not ignored, 328 ignored), the same names, the same count in
  every former binary (`mod` 232 ignored, `issue_configuration` 19, `private_routes` 11, `migrations` 7, …).
- **CI:** `api` runs `cargo test --test main` (it ran seven named DB-free binaries; `public_files`, 6 tests, ran nowhere
  before, since the `db` loop passed `--ignored`, and now runs here); `api-db` runs the two commands above instead of a
  loop over `tests/*.rs` (the same exclusions: `load_test`; `keycloak_container`, `keycloak_entra_broker` and
  `fluvio_relay` build only with their features); `restore` seeds with one filtered run and drills `requests::`.
- **Measured on a shared 16-core Mac** (other agents' builds kept the load average at about 40 for the baseline and
  130 to 170 for the second run, so the wall times do not compare): after a warm build and a comment appended to
  `src/controllers/tba.rs`, `cargo test --no-run --all-features` took 196 s wall, 466 s user, 66 s sys before and
  498 s wall, 251 s user, 11 s sys after. Test executables: 69 (12,861 MiB) before, 1 (267 MiB) after, plus the lib
  and bin unit-test executables in both.
- Tested: fmt, clippy (all targets, all features), `cargo test --lib`, `cargo test --test main`, `keycloak_guard` with
  `--features keycloak`, `cargo deny --all-features`, and both database commands on a throwaway database: 92 of 92
  and 231 of 232. **Not verified:** `requests::scale::capacity_views_do_not_fan_out_with_scale` failed once in that run
  (4 statements at 5 plans, 5 at 60) at a load average near 300 and passed when rerun alone; it counts statements, and
  a background ticker probably ran one during the window; the suite set in that process is the same as the old `mod`
  binary's, so the fold is unlikely to be the cause, but it was not shown. Not run: `keycloak_container`,
  `keycloak_entra_broker`, `fluvio_relay` (container or broker), `load_test` (a running service), the restore drill,
  and the old binaries' database suites as a baseline. The wall-time saving is not shown on a quiet machine.

### Changed — axa 0.19 and axa-test 0.10 (2026-11-17)

- `axa` 0.17 to 0.19 (through 0.18), `axa-test` 0.8 to 0.10; with them `axa-postgres-job-queue` 0.5.1 to 0.6.0. The job
  queue's times are `jiff` inside axa, stored and scheduled in UTC as before: no migration, no setting. axa no longer
  brings `chrono` (through `croner` or its `sqlx` features); the service's own use remains until its move to `jiff`.
  No service code named the changed types. Upgrade notes: `operations/upgrade-and-migration.md`.
- **Publish-age exception, approved by the user on 2026-11-17:** axa 0.19.0, axa-postgres-job-queue 0.6.0, axa-test
  0.10.0 (published that day) and jiff, jiff-core, jiff-sqlx and jiff-static 1.0.0 (published 2026-11-15) were
  resolved under the 2-day minimum with `CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow`; nothing else was added.
- Tested: fmt, clippy (all targets, all features), `cargo test --lib`, the database suite, `cargo deny --all-features`.
  **Not verified:** the real-token binaries other than `mod`, and the job queue under load.

### Changed — axa 0.17 and axa-test 0.8 (2026-11-17)

- axa 0.15 to 0.17 and axa-test 0.6 to 0.8, before the three-day minimum package age with the owner's approval;
  `rust-version` 1.98 here and in the migration crate, as axa 0.17 needs. `App::run` answers the new `health` command
  with `axa::cli::health` before building the configuration, and the final `match` covers it. The queue's listener
  takes one more pooled connection. `Cargo.lock` gains `ppp` (PROXY protocol, pulled in by axa).
- Tested: clippy (all targets and features) and `cargo fmt --check`; `cargo test --lib`, 1,290 passed; the database
  suite on Postgres 18, 232 passed. **Not verified:** `service health` against a running server; the listener under load.
- **Fixed after the push:** clippy had been run before `rust-version` was raised, and at 1.98 its `map_unwrap_or` lint
  asks for the new `Option::map_or_default`, so CI's clippy failed on 41 `.map(..).unwrap_or_default()` calls. They are
  rewritten by `cargo clippy --fix` (33 files, the same behaviour). `cargo fmt` then shortened `run_export` below
  clippy's length limit, so its `#[expect(clippy::too_many_lines)]` went unfulfilled and is removed. Clippy, run after
  the formatter, is clean and the library tests pass again.

### Changed

- **axa 0.15.0 and axa-test 0.6.0 (2026-11-16)**, with the owner's exception to the minimum package age. The job
  queue adds its new columns at boot; no migration. See the root `CHANGELOG.md` and
  `operations/upgrade-and-migration.md`.

### Fixed

- **`GET /api/portfolio-money` no longer reads each plan's forecast on its own (RG-18, 2026-11-15).**
  `controllers::financials::forecasts_of`; regression test `requests::scale::portfolio_money_does_not_fan_out_with_programmes`.
  See the root `CHANGELOG.md`.
- **A dropped connection under concurrent requests (2026-11-13).** `observability::trace_mw` held a `Span::enter` guard
  across an `.await`. The `tracing` registry then panicked in the connection task, about once in a thousand requests,
  and the client's connection dropped. It now uses `Instrument`. Regression test: `tests/trace_span_concurrency.rs`.
  See the root `CHANGELOG.md`, 13 November.

### Added

- **Organization short names and names in other languages (RE-7, 2026-11-16).** Migration
  `20261116000115_organization_names`; `controllers/organizations.rs`; the public reader serves them. Spec FR-16e18.
  See the root `CHANGELOG.md`.
- **Benchmarks for the critical path, the guard, cursors, the roadmap rules and the DHCW import (RG-17,
  2026-11-15).** `benches/service_bench.rs`; results in the root `BENCHMARKS.md`.
- **Product family colours (RE-5, 2026-11-15).** Migration `20261115000113_roadmap_families`;
  `controllers/roadmap_families.rs`. Spec FR-16e27. See the root `CHANGELOG.md`.
- **The DHCW import's apply step (RE-5, 2026-11-15).** `src/dhcw_apply.rs`, task `dhcw_roadmaps_apply`,
  `dhcw_import::parse`, `public_roadmaps::insert_item_keeping`. Spec FR-16e26. See the root `CHANGELOG.md`.
- **The editors' list of roadmaps (RE-5, 2026-11-15).** `GET /api/public-roadmaps`, paged. Spec FR-16e25. See the
  root `CHANGELOG.md`, 15 November.
- **A dry run of the DHCW roadmap spreadsheet (RE-5, 2026-11-14).** `src/dhcw_import.rs`, task
  `dhcw_roadmaps_dry_run`, fixtures in `tests/fixtures/dhcw-roadmaps/` (MIT, DHCW's notice kept).
- **Publishing a roadmap as one change set (RE-2, 2026-11-14).** `controllers/roadmap_publish.rs`. Spec FR-16e24.
- **Old slugs redirect (RE-8, 2026-11-14).** Migration `20261114000112_public_roadmap_slugs`. Spec FR-16e23.
- **The public roadmap reader, off by default (RE-4, 2026-11-14).** `controllers/public_reader.rs`,
  `tests/public_reader.rs`. Spec FR-16e22.
- **Freshness of a public product roadmap (RE-3, 2026-11-14).** `PUT …/roadmap/reviewed`,
  `GET /api/public-roadmaps/stale`, `PROJECT_PORTFOLIO_MANAGEMENT_ROADMAP_STALE_DAYS`. Spec FR-16e21.
- **The public product roadmap, with translations marked for review when their source changes (RE-1, RE-6,
  2026-11-14).** Migration `20261114000111_public_roadmaps`; `src/public_roadmap.rs`; `controllers/public_roadmaps.rs`;
  module `roadmap`. Spec FR-16e19, FR-16e20. See the root `CHANGELOG.md`.
- **Organization codes, a national scope and roadmap categories (RE-7, 2026-11-14).** Migration
  `20261114000110_roadmap_lists`, the first after the freeze; `/api/roadmap-categories`. Spec FR-16e18. See the root
  `CHANGELOG.md`, 14 November.
- **`tests/load_test.rs` (2026-11-13):** a read-only load test against a running service (`#[ignore]`d; not in CI).

### Changed

- **axa 0.14 and axa-test 0.5 (2026-11-15).** Migration `20261115000114_axa_outbox_tenant` (one way, in
  `migration::REFUSED`); `tenant: None` on every outbox event; `log_tenant: false` in the rate-limit test's
  `config::Http`. See the root `CHANGELOG.md`.
- **axa 0.13 and axa-test 0.4 (2026-11-14).** Feature `postgres-sqlx`; the cross-site write guard, with a listed
  CORS origin let through by `http_security::cors_mw`; `tests/requests/cross_site.rs`. See the root `CHANGELOG.md`.
- Replaced Loco with axa 0.1: see the root `CHANGELOG.md`, 5 November (Loco replaced with axa).

### Added

- **Role permissions, plan membership, field rules and plan types (OC-2 to OC-6, OC-9, 2026-11-05).** Migrations 101 to 104:
  `organization_roles.permissions`, `plan_role_members`, `workflow_states.field_rules`, `plan_issue_types`, and `labels` on types,
  roles and states. `/api/plans/{pid}/role-members`, `/api/plans/{pid}/issue-types`, `GET` and `PUT /api/workflows/{pid}`. A role can
  only narrow what a token allows. `?status=` confirms no hidden plan's word. A private plan's issues left the audit feed and
  `/api/plans/{pid}/audit`. `/api/workflows` answers `404` for a plan the caller cannot read. Not built: see
  `spec/issue-configuration.md`.
- **Organization-defined issue types, statuses and roles (OC-1, 2026-11-04).** `issue_types`, `organization_roles`,
  `issues.status_category` and `workflows.issue_type` (migration 100), `/api/issue-types`, `/api/organization-roles` and
  `GET /api/plans/{pid}/issues/options`. An issue follows the workflow in force for its type, and "open" and "overdue" read
  the status's category. A role in the catalogue grants nothing. Registering or withdrawing an issue workflow needs an
  administrator, and the migration withdraws issue workflows registered before. Changing an issue's type under a workflow that
  restricts moves needs an administrator. A `PUT` that omits `kind` keeps it. `GET /api/workflows` leaves out a plan the caller
  cannot read. Not built: see `spec/issue-configuration.md`.
- **Rustdoc on every private item, and on the request structs' fields (2026-11-03).** About 1,600 private functions, structs, fields and constants now say what they do, what an absent value means and which route, guard or audit entry they serve (`clippy::missing_docs_in_private_items` reports none). The request and query structs' field docs also flow into `openapi.json` as property descriptions (574 added), so the API reference now states units and what absent means. Broken intra-doc links were repaired, so `cargo doc --document-private-items` is clean under `-D warnings`. Comments only: no code changed.
- **The personal-data inventory is current again (2026-11-02).** 30 columns added since it was last updated now have a row with a class, purpose and retention class (escalations, frameworks and their versions, lessons, status updates, task transitions, peer-pool links and others). `framework_versions.created_by` is now in the subject-access and erasure tools. `tests/personal_data_inventory.rs` reads tables made by raw SQL (`idempotency_keys`), which have no entity file. CI does not run this test file; run it with `cargo test --test personal_data_inventory`.
- **Withdrawing a figure from the workforce service (FB-5, 2026-10-30).** A demand total that rounds to zero is now sent to WPM as `fte_centi: 0` (WPM withdraws its `ppm` claim) instead of being recorded `not_sent_zero`; every peer-demand send carries `sent_at` (RFC 3339, the time of the write); unlinking a pool, or changing its link, withdraws best effort every figure sent to the old WPM pool, each recorded, listed in `peer_demand_withdrawals` on the peer-link answer; `GET /api/capacity-commitments` shows a recorded send even after the pool is unlinked. `src/wpm_bridge.rs` (`plan_demand`, `demand_body`, `send_demand`), `src/controllers/capacity_heatmap.rs`, OpenAPI text and `openapi.json`, entity spec §8.5a rules 5 and 6. Tests: a unit test against a stand-in WPM (exact bodies of a figure and its withdrawal, `sent_at`, the duplicate), `tests/wpm_bridge.rs` (the zero sent and recorded, the unlink withdrawal to the old pool under the caller's token, a changed link with no URL recorded `not_configured`, linking withdraws nothing). **Not verified:** against a running WPM, which is building its side of this contract in parallel (FB-8); whether WPM refuses a zero for a pool it has already unlinked (then recorded `refused`). **Not done:** no retry of a withdrawal that was not sent or not answered; at most 2 400 withdrawals for one unlink; no interface shows `peer_demand_withdrawals` on `/skill-pools`.

- **People impact in from the workforce service (FB-4, 2026-10-30).** `GET /api/plans/{pid}/people-impact[?months=1..36]`: WPM's answer passed through unchanged with `source: "wpm"`, the plan's full `target_date` sent as `ends_on`, `unknown` with a reason when not configured, no token, unreachable, slow, unreadable or refused; nothing kept. `src/wpm_bridge.rs`, `src/controllers/people_impact.rs`, `tests/wpm_bridge.rs`. **Not verified:** against a running WPM (FB-8): every test uses a stand-in answering with the shape read from WPM's `controllers/capacity/people_impact.rs`; the stand-in's answer is not checked against WPM's real output, so a later change to WPM's shape would read as `unknown` (unreadable) or pass through unnoticed for fields PPM does not check; CI.

- **The bridge to the workforce service (FB-1 to FB-3, 2026-10-30).** `PUT /api/skill-pools/{pid}/peer-link`; WPM's supply for a linked pool on `GET /api/capacity-heatmap` with its source, time and status, never added to what was entered here; demand commitments on a linked pool sent to WPM under the caller's token, outcome recorded (`peer_demand_sends`, migration 99). `src/wpm_bridge.rs`, `tests/wpm_bridge.rs`. **Not built:** people impact in (FB-4), withdrawing a figure (zero is not sent, FB-5), retries, the UI (FB-6). **Not verified:** against a running WPM (a stand-in on a local port answers with the shapes read from WPM's code).

### Changed

- **Kept WPM supply figures are keyed by caller (FB-2 fix, 2026-10-30).** `src/wpm_bridge.rs`: the in-process store is keyed by caller (subject, else the SHA-256 of the bearer token, never the token), pool and month; when WPM cannot be reached the heatmap recalls only the figure this caller's token fetched; bounded at 1 000 callers × 2 400 figures. The director pack, drum, pipeline and start check make no call and now show `unknown` for a linked pool rather than any kept figure. Unit tests and `tests/wpm_bridge.rs` (a second caller with WPM down sees `unknown`). **Not verified:** against a running WPM (FB-8); a drum over a linked pool (no test); the same `sub` from two issuers would share figures; CI.

- **The mounted routes are written to `api-routes.txt` (CQ-9, 2026-10-22).** `openapi::tests::api_routes_file_is_current` fails when the file differs from the router (regenerate with `UPDATE_API_ROUTES=1 cargo test --lib api_routes_file`); the UI's tests read it to check the client's paths.

- **Request bodies can no longer go undocumented unnoticed (CQ-8, part, 2026-10-22).** Of 260 documented writes (`post`, `put`, `patch`), 227 have no `requestBody` in the OpenAPI document. They are now listed in `openapi::tests::WRITES_WITHOUT_REQUEST_BODY`, and `every_documented_write_has_a_request_body_or_is_listed` fails on a new write without one and on a listed write that gained one, so the list can only shrink. Not done: no body was documented; some listed writes take no body at all (approvals, POST twins of deletes) and are not told apart from real gaps; response bodies are not checked.

- **Availability ranges are checked and stored under a lock per person (CF-3, 2026-10-21).** `POST /api/availability` read the person's ranges, checked the overlap and inserted, with nothing stopping a second request from doing the same in between. It now does all three in one transaction holding `advisory_lock("availability", person_ref)`; `advisory_lock` takes any displayable key. A database exclusion constraint was not used because it needs the `btree_gist` extension. Verified: clippy, the request suite (189), `availability_acl`, and a request test that sends two overlapping ranges together and gets one 200 and one 409. **Not proven:** that test passes without the lock too, because the in-process test server runs the two requests one after the other; the race itself is not reproduced.

- **Goals can be edited safely by position (CF-2, 2026-10-21).** A goal has no id and is addressed by position, so a concurrent delete could make a later edit or delete hit the wrong goal. Every goals answer now carries a weak `ETag` of the list, and `PUT`/`DELETE …/goals/{index}` with an `If-Match` that no longer matches is `412 precondition_failed` and changes nothing; without `If-Match` nothing changes for existing callers. The paging layer used to drop every `ETag` from a list; it now drops it only when the page leaves rows out. Request test: two readers, the first deletes goal 0, the second's delete with the old tag is 412 and deletes nothing. Verified: clippy, `cargo test --lib`, the request suite (189) and the `list_paging` census. Not done: the check and the write are not one transaction (a write between them is not caught); the UI does not send `If-Match` yet; OQ-5 (a whole-plan `PUT` racing a goal write) is unchanged.

- **Fewer queries in loops, and an earned-value race closed (CQ-11, 2026-10-21).** `POST /api/plans/{pid}/earned-value/snapshot` looked up today's row and then updated or inserted it, so two at once could hit the `(plan_pid, currency, as_of)` unique index and answer 500; it is now one `INSERT … ON CONFLICT … DO UPDATE` per currency. The budget-baseline list loads every baseline's periods in one query instead of one per baseline, and scenario create checks its named plans and proposals with one query each instead of one per member. The request test fires two snapshots at once and still finds one row. Verified: clippy, `cargo test --lib`, the request suite (189). Not done: query counts are not asserted by a test; `thresholds.rs`, `includes.rs` and `task_get.rs`, named by the research pass, were not reviewed; the two concurrent requests are not guaranteed to have overlapped in the test.

- **Unused dependencies removed (CQ-16, 2026-10-21).** `regex`, `include_dir` and `futures-util` (no reference anywhere in `src`, `tests`, `benches` or `migration`), sea-orm's `sqlx-sqlite` feature (nothing uses SQLite; `libsqlite3-sys` is no longer compiled) and the second `sha2` under `[dev-dependencies]`. Verified: `cargo clippy --all-targets --all-features`, `cargo test --lib`. Not done: `serde_json`'s `arbitrary_precision` was kept (it changes how every number parses, and nothing yet shows it is not needed); no `cargo machete` job in CI.

- **Search-index writes run on the blocking pool (CQ-10, 2026-10-21).** `streaming::index_best_effort` and `deindex_best_effort` took tantivy's writer lock and committed (an fsync) on an async worker thread; they now run in `spawn_blocking`, and the request still awaits them, so a plan is searchable as soon as its create answers, as before. A panicked index task is logged like a failed one. Verified: clippy, `cargo test --lib`, the request suite (189, which searches for plans it has just created). Not done: the latency effect was not measured (`BENCHMARKS.md` not updated); searches (reads) and the reindex task still run inline.

- **SQL parameters bound, no database text in a 409 (CQ-12, 2026-10-21).** The seven advisory locks (`sandbox` ×2, `handover`, `hammock`, `document` ×3) go through `controllers::advisory_lock`, which binds `'<scope>:<pid>'` as `$1` and hashes the same string as before, so the locks still exclude each other. Retention binds its cut-off date as `$1` (`Rule::condition()` no longer takes it). The data-subject tool refuses a table or column that is not a plain identifier (`subjects::is_plain_identifier`) and no longer puts the database's error text in its 409 (it is logged). Unit tests: identifiers in `RULES` and `COLUMNS`, the condition text, the identifier check. Verified: clippy, `cargo test --lib`, the request suite (189), `controlled_documents` and `finance_gate`. Not done: none of these was injectable before (all inputs were fixed or UUIDs); this removes the pattern, not a known hole.

- **Controller helpers defined once (CQ-7, 2026-10-21).** `controllers/mod.rs` gains `unprocessable_entity`, `parse_pid`, `find_live_plan` and `find_live_task`; 38 local `unprocessable`, 20 `db_err`, 7 `find_plan`, 4 `find_task` (including `engineering::find_task`, now a re-export) and 89 hand-written `Uuid::parse_str(..).map_err(|_| Error::NotFound)` are gone. Checked while merging: every copy applied the same live-record filter and the same 404; the local `db_err` used `Error::string` and the shared one `Error::Model`, which loco renders as the same generic 500 and logs alike, so no response changed. A unit census in `controllers/mod.rs` fails on a new copy (five `db_err`s that map a named constraint to a 409, and governance's `validation` 422, are listed as different, with the reason). Verified: clippy, `cargo test --lib`, the full database suite (189). Not done: the two 422 codes (`unprocessable` ×7, `unprocessable_entity` the rest) remain, pending plan §70 question 3; the one-line `is_admin` wrappers from CQ-2 remain.

### Security

- **The pack's external asks leave out a private plan's ask (CF-5, 2026-10-21).** The `external_asks` section listed every external ask of the period; one already promoted to a plan that the pack leaves out (private or marked) named that plan's work. `governance::asks_section` now takes the pack's included plans and drops such an ask; an ask with no plan yet stays (it is portfolio-wide). `tests/organizations.rs` promotes the ask, makes its plan private and drafts another pack: none listed (and one listed with the filter removed). The role drill-down (`GET /api/resource-plan/role-tasks`) already filtered private plans but had no test: `tests/private_aggregates.rs` now shows a non-member resource manager the public task and not the private one, and an administrator both. Not done: the first run of `private_aggregates` failed once on a database pool timeout at boot and passed on the rerun (not investigated).

- **The privacy guard no longer fails open (CQ-6, 2026-10-21).** In `privacy_mw`, an error looking up whether a plan is private skipped the check, so a non-member's request reached the handler. It now answers `503 unavailable`. Every fail-closed visibility lookup (privacy settings, handling marks, break-glass grants, readable plans) logs at error level and increments the new `visibility_lookup_failed_total`. Database test: with the privacy table renamed for one request, `GET /api/plans/{pid}` is 503 and counted, and 200 again afterwards. Not done: the aggregates still answer 200 with what they could check (hidden-when-unsure), not 503, because 57 callers take a set, not a result; the break-glass audit and log paths were not tested under failure.

- **No audit error is discarded silently (CQ-3, 2026-10-21).** 226 calls in 75 files wrote the audit row as `AuditModel::record(..).await.ok()`, so a failed insert left a kept write with no audit row and no trace. They now call `AuditModel::record_or_log`, which logs at error level (entity, action, error) and increments the new `audit_write_failed_total` counter; the 70 calls that already passed the error on are unchanged. A unit test fails on any `Model::record(..).ok()`; a database test aborts a transaction and sees the failure counted once. Verified: clippy, `cargo test --lib`, the full database suite (188). Not done: the writes are still kept when their audit fails, and audits written after `commit()` were not moved into their transactions; both wait for plan §70 question 2 (refuse or keep).

- **Missing claims never grant with login on (CQ-2, 2026-10-21).** Nine checks (hard constraints, availability, resource detail, cutover co-signing, milestone sign-off, de-escalation, e-mail deliveries) read "no claims" as a pass so that everyone may act with login off; with login on they would also pass a token that expired or lost its key between the guard and the handler. They now go through `MaybeAuthUser::holds_or_open`, which passes no claims only when enforcement is off; the six `is_admin` copies use one `auth::has_attr`. Unit test of the rule and a source census against the old spelling; the 47 database tests on those routes pass. Not done: the race itself was not reproduced in a request test; the remaining raw attribute reads elsewhere are CQ-7.

- **CSV formula defusing everywhere (CQ-1, 2026-10-21).** Plan export (`/api/bulk` CSV), the auditor evidence pack, timesheet and report CSVs, and the RACI matrix wrote a cell like `=HYPERLINK(…)` as is, so a spreadsheet would run it; only the access review defused. One `src/csv_cell.rs` now writes every CSV cell (the four hand-written escapers are gone, and a unit test fails on a new one), and the plan import strips the prefix so a round trip is exact. Changed behaviour: the access review no longer prefixes a plain number such as `-1`. Not done: CSV the UI builds in the browser was not reviewed; no spreadsheet application was run against the output.

### Added

- **`GET /api/plans/{pid}/task-link-counts` (LT-8, 2026-10-21).** Live link count per task of a plan. Request test extended. (Corrected 2026-10-21, CQ-25: this entry said it does not check plan privacy; it does, through `privacy_mw`, like every `/api/plans/{pid}/…` route, and `every_link_route_is_behind_the_privacy_guard` proves it.)

- **`tolerance_escalations` task (FR-54, 2026-10-21).** Raises a tolerance escalation for each red dimension with none live and none raised in the last 7 days, as `system:tolerance-escalations`; `controllers::tolerances::raise_all_needed`. Request test covers once, not while open, quiet period, and again after 10 days. Not scheduled by the service; no notification; the quiet period is fixed.

- Schedule tolerance (FR-53): a fifth dimension `schedule`, the slip against the latest schedule baseline's finish (migration `m20261021_000085` widens the tolerance and escalation dimension checks).
- Task links (LT-8): `plan_links.task_pid` (migration `m20261021_000084`, replaces the uniqueness index); `POST/GET /api/plans/{pid}/tasks/{tpid}/links` and `DELETE`/POST-twin removal in `controllers/plan_links.rs`. API only; no screen.
- Escalation notifications (FR-54): `controllers/escalations.rs` `tell` writes an in-app `escalation` notification to the raiser and `owner_ref` on each step (not to the actor); `escalation` added to the notification kinds. No email; no opt-out test.
- Escalate red tolerances (FR-54): `POST /api/plans/{pid}/tolerances/escalate` raises one `tolerance` escalation per red dimension with none live, at the first governance-path level; idempotent; a person asks, nothing raises one on its own. `controllers/escalations.rs` gained a shared `insert`. Not done: automatic raising, notification.
- Project escalations (FR-54): `POST/GET /api/plans/{pid}/escalations`, `GET …/{epid}` (with history), `POST …/{epid}/acknowledge|raise-level|resolve|withdraw`, `GET /api/escalations`. Raised at a level of the governance framework's escalation path (own, else the nearest ancestor's); an append-only history protected by a trigger. Tolerances report `open_escalations` and `escalation_needed`. Migration `m20261020_000083_escalations`. `controllers::frameworks::in_force` and `escalation_path` expose the framework in force to other controllers.

- Tolerances (FR-53): `GET /api/plans/{pid}/tolerances`, `PUT` and `DELETE`/`POST …/delete` on `…/tolerances/{dimension}`, `GET /api/tolerances`. Per-plan limits on time, cost, risk and resources (own, else the nearest ancestor's), measured from the schedule, budget baseline, risk register and bookings and shown as red/amber/green; a limit that cannot be measured is `unknown`, never green. Migration `m20261020_000082_plan_tolerances`. The nearest-ancestor rule moved from `frameworks` to a shared `inherit` module.

- Links into work-tracking tools: a plan link may be a `tracker` (a project or board) or a `ticket` (one issue) in Jira, Trello, Asana, Linear, monday.com, ClickUp, Azure DevOps, GitHub or `other`, with the system's own `external_id` (migration `m20261020_000081_link_systems`). A hosted system's link must be on its own domain; Jira, GitHub and `other` accept any host (self-hosted). `GET /api/plans/{pid}/links` takes `?system=` and `?kind=`. Nothing is fetched or synced.

- Ways of working and governance frameworks (FR-52): named, reusable records, organisation-wide or owned by a plan, that a plan follows (`PUT /api/plans/{pid}/ways-of-working` and `…/governance-framework`) or inherits from its nearest ancestor (resolved on read). `POST/GET /api/ways-of-working`, `/api/governance-frameworks`, `GET/PUT/DELETE …/{pid}`. Migration `m20261020_000080_frameworks` (`frameworks`, `plan_frameworks`); one implementation for both families (`src/frameworks.rs`, `controllers/frameworks.rs`).

- Control action to issue (T-26): `POST /api/actions/{pid}/convert?to=issue` raises an issue on the control's plan (kind `concern`, severity `med`, owner and due date from the action, the description as title and in the text) and records `converted_issue_pid`; the default and `?to=task` are unchanged, and an action converts once, to one or the other. Test: the existing convert request test, extended.

- Role drill-down (PJ-12): `GET /api/resource-plan/role-tasks?role=` lists each person whose primary role it is with their open tasks, in plans the caller can read (others counted in `omitted_unreadable`). Admins and resource managers only (403 otherwise); the unclassified row is refused (422).

- MS Project export (PJ-10): `GET /api/plans/{pid}/export/mspdi` writes the plan as MSPDI XML (`mspdi::write`, `controllers/mspdi_export.rs`): outline in sibling order, dates, business-day durations, constraints, deadlines, notes, done as 100 percent, dependencies (plan links with kind and lag, `precedes` relations as finish-to-start) and the plan calendar. Imported tasks keep their UID. Refused above 5000 tasks. Audited as `mspdi_export`. The file lists what it does not carry in a comment.

- Sibling order of tasks (PJ-12): `PUT /api/plans/{pid}/task-order` with `{parent_task_pid?, order: [pids]}` sets the order of one parent's children (migration `m20261020_000079_task_sibling_position`, column `tasks.sibling_position`). The body must name exactly the live children once each (`409` if not, `422` for an unknown parent or a repeat), so concurrent reorders cannot drop a task. Task lists order positioned siblings first, then by id; `sibling_position` is in the task view. Audited as `task_order_set`.

- Blocked reasons (`spec/time-based-analysis.md` §17): `PATCH /api/plans/{pid}/tasks/{t_pid}` accepts an optional
  `blocked_reason` on a move into `blocked` (`waiting_on_person`, `external_dependency`, `awaiting_decision`,
  `awaiting_resource`, `other`), stored on the transition (migration `m20261019_000078_blocked_reason`, with a `CHECK`).
  The task time analysis reports `blocked_by_reason` and the `blocked_time` finding names the split.

- Goals under a plan (FR-12): `GET` and `POST /api/plans/{pid}/goals`, `PUT` and `DELETE /api/plans/{pid}/goals/{index}`
  and the POST twin `…/{index}/delete`. A goal has no identifier, so it is addressed by position (a delete shifts the
  later goals down). Writes go through the plan update path (validation, audit row, event). One request test, OpenAPI
  entries, listed in the paged lists.

## [0.1.0] - 2026-10-19

The version number was reset to 0.1.0 before the first crates.io release; the service and the operator interface
are both 0.1.0. This is a lower number than the 0.4.0 that was tagged and released on GitHub (`v0.4.0`, 2026-10-18),
which was never published to crates.io. No code changed.

## [0.4.0] - 2026-10-18

The service and the operator interface are versioned together from this release. The interface changes in this release (Lily widgets, table export, the strict style policy) are in the repository `CHANGELOG.md`.

### Added — the portfolio ledger (EX-12, 2026-10-11)

A funding allocation (source, amount, date) that budget lines draw on, traced in one query to the lines, gates, milestones met and benefits validated, with untraced money stated with its amount; a map from finance cost codes to budget lines; and a read-only reconciliation of a CSV ledger export that lists unmatched codes. Migration 77.

### Added — the outcome scorecard and the de-escalation tracker (EX-10, 2026-10-11)

A named set of KPIs read by finished quarter, an optional starter set of measures with no targets, and a de-escalation tracker: the escalation level as the sponsor set it, criteria with evidence per quarter, and the consecutive quarters met. A quarter with a missing reading breaks the run and says why; nothing ever changes the level. Migration 76.

### Added — go/no-go for safety-critical cutovers (EX-6, 2026-10-11)

A cutover decision with stop criteria that are published before it opens and cannot be edited afterwards (a new version is made instead), criteria marked met or not with evidence, and co-signatures from configured roles. A go needs every criterion met and every co-signature; a no-go or a delay for safety is a decision in its own right and the director pack shows it as such, not as slippage. Migration 75.

### Added — deployment-date change control (EX-5, 2026-10-11)

In a programme delivered as a central platform with local deployment, moving a deployment's go-live is a request: the knock-on to other deployments is computed from the schedule network when it is asked, its labour cost is recorded against the requester, and a named approver (a person or a role such as the escalation board, never the requester) approves or refuses with a reason. Nothing changes until it is approved. Migration 74.

### Added — start control (EX-8, 2026-10-11)

A limit on programmes in active delivery and a capacity check on the constraint pool: a programme moving into executing is refused with the reason and joins a queue, or starts with a recorded, audited override. Paused programmes do not count. Migration 73.

### Added — a sender trait and a delivery row per recipient for email (2026-10-11)

`src/email.rs` defines `Sender` (SMTP via the configured mailer, none, or a recording sender for tests) and the retry rules; `email_deliveries` (migration 72) holds one row per message per recipient with status, attempts, last error and sender; `controllers::email_deliveries::deliver` sends through the sender in force and never sends the same message to the same recipient twice; `GET /api/email-deliveries` lists them for administrators. The notification sinks of kind `email` now send through it. The service was not tested against the bundled Stalwart server.

### Added — triage and pause, reset and stop thresholds (EX-7, 2026-10-11)

Programmes can carry a dated triage and up to three thresholds (red confidence for N issued packs, forecast past a date, P80 cost over a cap). A sweep opens an exception for each crossing; a person records pause, reset, stop or an override with an expiry. Nothing is ever paused automatically; the director pack counts the decisions. Migration 71.

### Added — an optional Stalwart mail server in the compose file (2026-10-11)

`podman compose --profile mail up -d mail` runs Stalwart v0.16 (SMTP, IMAP, JMAP) on loopback ports, with its data in volumes and a pinned bootstrap administrator password; the API gains `SMTP_*` and `EMAIL_FROM` passthrough in the compose environment. First-time setup is by hand ([operations/email-with-stalwart.md](../operations/email-with-stalwart.md)); delivery from the service to a mailbox was not tested.

### Added — the key-person register (EX-11, 2026-10-11)

`/api/key-persons`: single points of failure (a person or a role), the successor and how ready they are, and who is contingent labour; resource managers only, 404 for anyone else. The director pack carries counts and the contingent share of each skill pool, and no name. Migration 70.

### Added — watermelon measures (EX-9, 2026-10-11)

Each director pack carries the watermelon measures, read from the pack and the previous issued pack as stored: self-against-independent gap, late discovery, milestones met with sign-off, and red without a decision, each null with a reason when it cannot be measured.

### Added — register completeness (EX-1, 2026-10-11)

`GET /api/register-completeness` scores each programme's register and names each missing field; the director pack's data-quality panel lists the incomplete ones. An optional deployment switch refuses a budget-line release or a firm allocation on an incomplete programme, naming what is missing; with it off the action goes ahead and the warning is audited.

### Added — named trade-offs for external asks (EX-13, 2026-10-11)

A proposal tagged as an external ask cannot be approved until it names what it displaces (`PUT /api/proposals/{pid}/tradeoff`); the director pack lists the period's asks and trade-offs. Migration 69.

### Added — the reporting method per programme (EX-14, 2026-10-11)

`PUT /api/plans/{pid}/reporting-method`: evm or product. A product programme shows burn-up and throughput in the director pack in place of earned value, or says it has no tasks yet; a change is audited.

### Added — the delivery-model typology (EX-2, 2026-10-11)

`PUT`/`GET /api/plans/{pid}/delivery-model`: central product, central platform with local deployment, or a local programme under central standards, and the organization that owns the delivery. The portfolio map and the director pack show both, and a local programme is shown as not delivered by the organization. Migration 68.

### Added — milestones met only with partner sign-off (EX-4, 2026-10-11)

A milestone can require sign-off from organizations; delivered and met are separate dates, a withdrawn sign-off (with a reason, audited) makes it outstanding again, and `GET /api/milestone-signoffs` and the director pack report the met share and the delivered-not-met gap. Migration 67.

### Added — partner organizations (EX-3, 2026-10-11)

`POST`/`GET /api/organizations` and `PUT /api/organizations/{pid}`: delivery, partner, sponsor and supplier as records. A capacity commitment can name one; an organization can be a director pack audience (`org:<pid>`); a caller whose token has the `organization` attribute reads the packs addressed to it and only its own commitments. Existing partner labels became partner organizations. Migration 66.

### Added — the drum and the pipeline (CC-5, 2026-10-11)

`PUT`/`GET /api/portfolio-drum` (a named skill pool, else the most overloaded) and `GET /api/portfolio-pipeline`: projects needing the drum at once are proposed a staggered start with a capacity buffer and an explanation, over the plans the caller may read. A proposal only; no date is changed. Migration 65.

### Added — buffer management (CC-4, 2026-10-11)

`POST`/`GET /api/plans/{pid}/buffer-plan` freeze the chain and buffers as planned; `GET …/buffer-management` reads each buffer (chain completed, consumed = forecast slip over the buffer, zone from the fever chart's lines), alerts naming a red chain, snapshots as history, and recovery actions that are recorded, audited and closed once. The fever chart now reads a frozen buffer plan in place of a declared buffer and the baselined critical path. Migration 64.

### Added — project, feeding and resource buffers (CC-3, 2026-10-11)

`GET /api/plans/{pid}/critical-chain-buffers`: the schedule at aggressive estimates, the project buffer and a feeding buffer per joining chain sized from the safety removed (cut-and-paste or root-sum-of-squares, printed), a feeding buffer that would push the chain flagged rather than absorbed, and resource buffers as readiness alerts. Advice only; no chain or an unestimated chain task gives `no_buffers` with the reason.

### Added — aggressive and safe estimates (CC-2, 2026-10-11)

`PUT /api/plans/{pid}/tasks/{t_pid}/ccpm-estimate` and `GET /api/plans/{pid}/ccpm-estimates`: each task's aggressive and safe figure, taken from what was recorded, else the schedule-risk range, else (only under the chosen rule) a derivation, with the source printed. A task with a figure missing is listed, never given one. Migration 63.

### Added — the critical chain (CC-1, 2026-10-11)

`GET /api/plans/{pid}/critical-chain`: the longest chain through dependencies and resource links on a schedule levelled for people's time (critical tasks may move, so the finish may), returned beside the critical path and never blended with it. `no_critical_chain` with a reason when nobody is assigned. Advice only; nothing is stored.

### Added — the monthly rhythm of the director pack (DP-12, 2026-10-11)

A schedule per audience drafts the pack for the month that just ended and never issues; the portfolio office records a review (needed before a schedule-made draft can be issued); a person issues; subscribers are notified with a link and never the body. Drafts that stop short of their period are marked stale. `director_pack_schedule` task, `POST /api/director-pack-schedules/sweep`, `/api/director-pack-subscriptions`. Migration 62.

### Added — the director pack as a PDF (DP-11, 2026-10-11)

`GET /api/director-packs/{pid}/pdf`: the stored pack as a deterministic PDF made from the JSON alone, readable by the same audiences; English words only (the writer is Latin-1). `DirectorPack` in the OpenAPI schemas now matches what `GET /api/director-packs/{pid}` returns (`pack` and `body`).

### Added — the capacity heatmap by skill pool and month (DP-10, 2026-10-11)

Skill pools (a role in at most one), partner capacity commitments, and `GET /api/capacity-heatmap` plus the pack's `capacity_heatmap`: demand, own and partner supply, and the gap per pool and month, each cell with a word, a shape and a colour, "no supply recorded" instead of zero, and a table twin. Partner organizations are labels until EX-3; their names are hidden unless the deployment shows them. Migration 61. The pack now lists no section as not yet built.

### Added — validated benefits, per year and non-financial (DP-9, 2026-10-11)

Benefits gain a recurrence, a yearly amount, a classification and a KPI; independent validations (append-only, with evidence) turn a claim into a validated figure, and a validator who is a member of the programme is refused. `GET /api/portfolio-benefits` and the pack's `benefits` section show planned, claimed and validated per programme and in total without adding recurring to one-off, and non-financial benefits by KPI status or "not measured". Migration 60.

### Added — portfolio earned value over the threshold (DP-8, 2026-10-11)

`GET /api/portfolio-earned-value` and the pack's `earned_value` section: each programme's roll-up per currency, from the same code as the plan roll-up, with a configurable BAC inclusion threshold (printed), "below EVM threshold" and `no_baseline` stated rather than blank, and the AC source and EAC variants named. The roll-up endpoint's rows gain `ac_source`, `state` and `eac.variants_used`.

### Added — the buffer fever chart (DP-7, 2026-10-11)

A declared (or rule-given) project buffer per programme, append-only; the pack's `buffer_fever` section and `GET /api/buffer-fever`: chain completed % against buffer consumed %, zones from two configurable sloped lines printed in the pack, edge-plotted recovery and overrun with the true figure, a table twin, and the trail from the last issued pack. The chain is the baselined critical path (the critical chain is not built). Migration 59.

### Added — OpenAPI request and response schemas for the director-pack work (2026-10-11)

`src/openapi_schemas.rs` defines sixty schemas (change requests, the pack summary and pack, delivery confidence, exceptions, aims, the portfolio map, gates, programme status, funding, money, the fever chart) and attaches request bodies and response content to thirty-nine operations already documented. Tests check that every reference resolves, every schema is used, every attachment names a real operation and status, and no name is defined twice. Sections of the pack body that are free-form (exceptions, issues, change-request counts) are documented as objects, not field by field.

### Added — funding sources and financial-year money (DP-6, 2026-10-11)

A funding-source list and an optional source on each budget line; `GET /api/portfolio-money` and the pack's `funding` section group programme money by source, class (capex is capital, opex is revenue) and currency for the financial year, with the existing forecast beside it; a line with no source is counted and listed; classes are never summed unless asked; currencies never. Migration 58.

### Added — gate decisions, planned gate dates and programme status (DP-5, 2026-10-11)

Planned gate dates per plan and gate; the pack's `gate_decisions` lists gates passed, failed and deferred (a hold, or a planned date that passed with no review) and programmes paused and stopped. Programme status changes only by a recorded decision (reason, decider, date, link to a gate review or exception); stopped is terminal, enforced by the service and by a database trigger; a pause counts the programme's allocations and deletes none. The map shows each programme's status. Migration 57.

### Added — delivery confidence, exceptions and the portfolio map (DP-2, DP-3, DP-4, 2026-10-10)

Stated delivery confidence (append-only, self or independent) beside the derived RAG; exceptions that open at amber-red or red, keep their clock and close only on a decision, with a breach task; strategic aims, primary aims and `GET /api/portfolio-map`. The director pack now carries the map (with trend against the previous issued pack) and the exceptions. Migration 56.

### Added — the monthly director pack, first version (DP-1, 2026-10-10)

`/api/director-packs`: drafted once into stored text, issued by a person, then frozen by the database; reissue with a change note; read by audience; a confined partner reader sees only the packs. Holds issues and change requests so far and lists the sections not yet built. Migration 55.

### Added — response cache for the critical path (2026-10-10)

`src/response_cache.rs`: entries keyed by caller and a write epoch that any successful non-read request replaces, 30-second lifetime as a backstop; the privacy gates run before it. Used for `GET /api/plans/{pid}/cpm` on plans of up to 5,000 tasks. `config/test.yaml` now has an in-memory cache so the tests exercise it.

### Added — issues and change requests for the director pack (IR-6, 2026-10-10)

`GET /api/director-pack/issues-and-changes`: counts over the plans the caller may read, with a sentence when nothing is recorded and `available: false` when no plan is readable.

### Changed — the production cache is Redis (2026-10-10)

Loco's `cache_redis` is enabled and `config/production.yaml` uses it (`PROJECT_PORTFOLIO_MANAGEMENT_REDIS_URL` required, pool size `..._REDIS_POOL`); `compose.yaml` gains a password-protected `redis` service with no volume. `/_readiness` is 503 when Redis cannot be reached. Development, tests and `container-dev` keep the in-memory cache. The critical-path answer uses it (see below).

### Changed — a ceiling on the full critical-path answer (RG-16, 2026-10-10)

`GET /api/plans/{pid}/cpm` for a plan of more than 5,000 tasks (`PROJECT_PORTFOLIO_MANAGEMENT_CPM_FULL_MAX_TASKS`) answers `413` unless `?only=critical` or `?full=true` is given. Smaller plans are unchanged.

### Added — change requests (IR-4, IR-5, 2026-10-10)

`/api/plans/{pid}/change-requests` with draft, edit, submit, review, decision, implement, withdraw, links, preview and baseline, and `GET /api/change-requests`. One named approver who is not the requester decides, with a reason; the database enforces the same. A request may carry what-if edits whose forecast equals the sandbox's own answer; a baseline because of one is taken by a person, never automatically. Migration 54.

### Added — handling marks (RG-7, 2026-10-10)

`PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS` configures an ordered scheme; `GET /api/handling-marks`, `GET`/`PUT /api/plans/{pid}/handling-mark`. A plan above a caller's `clearance` attribute is 404 to them (administrators included) in the guard layer and the shared hidden scope; every plan response carries `X-Handling-Mark`. Migration 53.

### Added — break-glass access to private plans (RG-10, 2026-10-10)

`/api/break-glass`: an eligible person opens a private plan for up to four hours with a stated reason and a named second person, who is told in the same transaction (no notice, no grant). Visibility only; every request it opens is audited; revocable by the holder or an administrator; five a day at most. Migration 52. Also hardened: the plan's issue list is capped at 5,000, the retention periods have a ceiling (an absurd value used to overflow the date arithmetic), strict outbound mode re-checks the resolved address at send time, spreadsheet exports defuse a leading tab or carriage return, and the content check is fuzzed.

### Changed — critical path on large plans (RG-16, 2026-10-10)

`GET /api/plans/{pid}/cpm`: the per-task lookup of a person's calendar is a map and not a scan (the answer was quadratic in the plan's size); new `?only=critical` returns the critical and near-critical tasks and the dependencies between them, with `nodes_total` and `only` in the answer.

### Changed — the plan's task list pages (RG-15, 2026-10-10)

`GET /api/plans/{pid}/tasks` takes `limit` (default 100, at most 500) and `offset`, sets `X-Total-Count`, `X-Limit` and `X-Offset`, and adds `total`, `truncated`, `limit`, `offset` to the body. `counts` are computed over every matching task. Without paging parameters the first 2,000 tasks are returned and `truncated` says whether more exist (before, all of them, however many).

### Added — attachment content checks and a virus-scan hook (RG-8, 2026-10-10)

An upload's bytes must match its declared type, and programs and scripts are refused whatever they claim to be. `ATTACHMENT_SCAN_ADDR` (a `clamd` over TCP) streams each file to the scanner before anything is stored; an infected file is refused and audited, and a scanner that cannot answer refuses the upload (503).

### Added — the access review (RG-9, 2026-10-10)

`GET /api/access-review`: the grants that stand today from the access ledger, oldest first, with age in calendar days and a `review_due` flag (default 365 calendar days); `?format=csv` for a spreadsheet with a decision column. Administrators only; plans the caller cannot read are left out.

### Added — data-subject export and pseudonymisation (RG-6, 2026-10-10)

`GET /api/admin/subjects/{ref}` (everything held about a person reference, all three forms: bare subject, `person:`, `worker:`) and `POST …/pseudonymise` (a preview unless `?apply=true`; one transaction; keeps each form's shape; reaches the protected audit trail only inside it). Administrators only; optional secret `PSEUDONYM_SALT`. The column list is kept in step with `governance/personal-data-inventory.csv` by a test.

### Added — controls for regulated buyers (RG-1 to RG-5, 2026-10-10)

Strict outbound mode for operator-configured webhook and worker targets (`OUTBOUND_STRICT`); a database trigger that refuses `UPDATE` and `DELETE` of audit rows outside the retention task (migration 51); `cargo loco task retention_purge` (dry run by default; read notifications, delivery logs, trend snapshots). Backup scripts can encrypt with gpg. See `governance/production-readiness.md`.

### Added — raise an issue from a risk or a blocked task (IR-3, 2026-10-10)

`POST /api/plans/{pid}/issues/from-risk/{risk_pid}` (kind risk, severity from exposure: 20 or more critical, 12 high, 6 med, else low; owner and text copied) and `POST …/issues/from-task/{task_pid}` (blocked tasks only; kind blocker, owner is the assignee). The source is never changed; one open issue per source, `409` otherwise.

### Added — the issues register across plans (IR-2, 2026-10-10)

`GET /api/issues`: every issue in the plans the caller can read, most serious first then oldest; filters `plan`, `status`, `severity`, `owner`, `open`, `min_age_days` (calendar days); a summary by severity and status, the overdue count and the oldest open issue; capped at 500 rows with `truncated`.

### Added — issues register, plan-scoped (IR-1, 2026-10-10)

`/api/plans/{pid}/issues` (list, raise, read, replace, soft-delete): an issue is something that has happened; kind, severity, status, owner, raised and due dates, a resolution (required to close, enforced by a database check as well), `resolved_on` kept by the service, age in calendar days and an overdue flag. Rules in `src/issues.rs`; migration 50; audited; part of the `risks` module. Request test `tests/requests/issues.rs`; five columns added to `governance/personal-data-inventory.csv`.

### Changed — secure by default (HD-3, HD-4, HD-5, 2026-10-09)

**Breaking.** Login enforcement is now **on** unless explicitly turned off with `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false` or `settings.require_auth: false` in the loco config (the shipped `development.yaml` and `test.yaml` do; the variable wins). The service **refuses to start** when the loco environment is `production` and enforcement is off, and when enforcement is on with no token verifier and no key URL. `GET /metrics.prom` now needs a token. Every response carries `X-Content-Type-Options`, `Content-Security-Policy` (`default-src 'none'; frame-ancestors 'none'`, relaxed for `/swagger-ui`), `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy`, `Cross-Origin-Opener-Policy` and `Cross-Origin-Resource-Policy`, plus `Strict-Transport-Security` only when the request arrived over TLS (`PROJECT_PORTFOLIO_MANAGEMENT_HSTS`, `…_TRUSTED_PROXIES`). CORS is an explicit allow-list (`PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS`, exact origins; unset means same-origin only) instead of the framework default. Requests are rate limited per caller (a token bucket per bearer credential, else per client address), tighter for expensive routes and for failed authentication; `429` with `Retry-After` and `RateLimit-*` headers (`PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT*`; off in the development and test configs). New modules `http_security`, `rate_limit`, `trusted_proxy`; tests `tests/hardening.rs` (real router) and unit tests in each module.

### Changed — Oxford spelling and explicit day units (NFR-10, NFR-11, 2026-10-09)

Spelling is Oxford (`-ize`) everywhere, including identifiers, routes and stored values: `GET /api/prioritization` (was `/api/prioritisation`), the risk status `materialized` (migration `m20261009_000049_oxford_spelling` rewrites stored rows), `materialized_risks`, `utilization`. Every duration in days now says "calendar days" or "business days" ("working days" became "business days").

### Added — licence files (HD-1)

The licence texts live in the root `LICENSE/` directory; `tests/licence_files.rs` keeps the manifests and the files in step.

### Fixed — a forecast could miss what had just happened

The throughput forecasts (`GET /api/proposals/forecast`, `GET /api/plans/{pid}/forecast`) bucket history over a half-open window ending at "now", so an event stamped in the same millisecond as the request was dropped — intermittently, and only when the request followed the event closely. The window now ends one millisecond after "now" (`tba::history_window`), so everything recorded up to and including it is counted; the number of periods is unchanged.

### Changed — simulation inputs are returned by the lists

`GET …/risks` now includes `task_pid`, `schedule_impact_days`, `probability_percent`, `cost_impact_minor` and `cost_currency`; task views and `GET …/cpm` nodes include `duration_optimistic_days` and `duration_pessimistic_days`.

### Added — cost-risk and joint Monte Carlo (CR-1..2, 2026-10-05)

`GET /api/plans/{pid}/cost-risk` and `GET /api/cost-risk` (portfolio) simulate final cost per currency from budget lines (low / planned / high, never below what is spent) and register risks with a cost impact. `GET /api/plans/{pid}/joint-risk` simulates cost and schedule together, with delay cost and a joint confidence; a risk with both effects strikes once. Inputs: `PUT …/budget-lines/{id}/estimate`, `PUT …/risks/{id}/cost-impact`.

### Added — milestones as a concept (MS-1..3, 2026-10-05)

`GET /api/milestones` (portfolio view with status, critical-path forecast and slip), `GET /api/milestones/{pid}`, `PUT …/milestones/{m_pid}/tasks` (the tasks a milestone waits for; a linked one is achieved when they are all done), `POST …/milestones/{m_pid}/reopen`. Completing a linked milestone over open tasks needs a reason; every reschedule is kept in an append-only history (optional `reason` on `PUT …/milestones/{m_pid}`).

### Added — KPIs, and the OKR portfolio view (KO-1, KO-2, 2026-10-05)

`/api/kpis` (+ readings): standing measures with a target and a red line; status, trend and staleness derived on read, `no_data` when unread. `GET /api/okrs` lists objectives with derived scores (unmeasured last); objectives and key results can now be edited and deleted (the baseline stays fixed).

### Added — float for the worker (SC-10, 2026-10-05)

`GET /api/me/tasks` gives each task a `schedule` block with its own critical-path total float (and `critical`, `late` flags), or `null` with a `schedule_reason` when there is no schedule data.

### Added — schedule health self-check (SC-9, 2026-10-05)

`GET /api/plans/{pid}/schedule-health` runs fourteen DCMA-style checks (missing logic, leads, lags, relationship types, hard constraints, high and negative float, high duration, invalid dates, resources, missed tasks, critical path test, CPLI, BEI), each with count, threshold, pass and offenders by pid. A check that cannot be computed is `pass: null` with a reason, never a pass. A self-check, not a DCMA compliance assessment.

### Added — schedule-risk Monte Carlo (SC-8, 2026-10-04)

`GET /api/plans/{pid}/schedule-risk` runs a seeded simulation over the task network: P10/P50/P80/P90 finish, the chance of meeting the CPM date, a criticality index and a sensitivity per task, and a finish histogram. Inputs: `PUT …/tasks/{pid}/estimate` (optimistic and pessimistic business days) and `PUT …/risks/{pid}/schedule-impact` (task, business days, probability). Inputs are treated as independent, and the response says so.

### Added — portfolio resource management (SC-7, 2026-10-04)

`GET /api/resource-plan` gives demand against supply by role (or skill) and week or month, in FTE, across plans; `/hours` planned against actual hours. Role-only placeholders (`POST …/allocations {placeholder: true, role}`) are resolved with `POST …/allocations/{a_pid}/resolve`; `PUT /api/resource-roles` sets a person's primary role.

### Added — resource leveling (SC-6, 2026-10-04)

`GET /api/plans/{pid}/leveling` proposes start delays that remove over-allocation within float (advice only); `POST …/leveling/apply` applies confirmed moves through the bulk edit with one audit entry per moved task. `POST /api/tasks/bulk` can now set `start_date`.

### Added — EVM analytics (SC-5, 2026-10-03)

`/api/plans/{pid}/earned-value/forecast` gives every EAC variant by its formula with ETC, VAC, TCPI and traffic lights; `/series` the S-curve data (earned value and cost only on recorded days, snapshotted daily); `/rollup` per-currency totals beneath a plan.

### Added — earned value (SC-4, 2026-10-03)

`GET /api/plans/{pid}/earned-value`: planned value, earned value and actual cost per currency, with SV, CV, SPI, CPI and earned-schedule SPI(t). Percent complete is physical and its method is chosen per plan and per task; a manual percent needs a note and is always flagged. `/performance` now reports the real ratios.

### Added — RACI matrix (RM-13, 2026-10-03)

`/api/plans/{pid}/raci` and `PUT …/tasks/{pid}/raci`: who is Responsible, Accountable, Consulted and Informed on each task. One Accountable per task (refused otherwise, and enforced in the database); incomplete rows are reported, not hidden; CSV export; R, A and I people are auto-subscribed.

### Added — schedule baselines and variance (SC-3, 2026-10-03)

`/api/plans/{pid}/schedule-baselines` takes numbered, append-only snapshots of a plan's task schedule (a reason is required); `/schedule-variance` shows, in business days, what slipped, was pulled in, was added or was removed since. Added tasks are not slip and removed tasks stay listed.

### Added — proposed vs committed bookings (PJ-4, 2026-10-03)

Allocations can be proposed (hoped for) or committed (firm, the default). A proposal never counts toward utilization; `GET /api/capacity` reports it as its own series. Committing one that would put the person over their per-day capacity needs `?confirm=true`.

### Added — availability over time (PJ-3, 2026-10-03)

`/api/availability`: a person's capacity by date range (part-time, leave, ramp-up). `GET /api/capacity` now reads capacity per day from it; with no ranges nothing changes. A person sets their own; resource managers and admins set anyone's.

### Added — MS Project XML import (PJ-10, 2026-10-03)

`POST /api/plans/{pid}/import/mspdi` reads an MS Project XML (MSPDI) file into a plan: tasks, outline, dates, durations, constraints, deadlines, dependencies (with their types, lags and leads) and the calendar. A dry run is the default; a re-import updates in place. Anything not carried over is listed with its reason.

### Added — calendars and constraints (SC-2, 2026-10-03)

Working calendars per plan and per person (a person's recorded absences count), task date constraints (snet, snlt, fnet, fnlt, mso, mfo) and soft deadlines with negative float. Hard constraints need a reason and a permission.

### Added — critical-path engine (SC-1, 2026-10-03)

`GET /api/plans/{pid}/cpm` gives total and free float, critical and near-critical tasks and every critical chain, for all four relationship types with lag and lead. The plan-level critical path now counts lag (previously ignored).

### Added — task schedule fields and dependency kinds (PJ-1, 2026-10-03)

Tasks gain `start_date` and `duration_days`; dependencies gain a relationship kind (fs, ss, ff, sf) and may carry a negative lag (a lead) and join tasks as well as plans. `PUT /api/plans/{pid}/tasks/{pid}/schedule` sets a task's schedule.

### Added — scheduled report runs (T-28e, 2026-10-03)

`report_schedules`, `POST /api/reports/{pid}/schedule`, `/api/report-schedules`: run a saved report daily, weekly or monthly. Each run is stored as an expiring artifact and audited like an export, even at zero rows. Claim-based, so a missed run runs once.

### Added — attachment text search (PL-8, 2026-10-03)

`GET /api/attachments/search?q=` finds attachments by their text (plain text, CSV, Markdown, JSON, .docx, .xlsx, .pptx). PDFs and images are reported as unindexed, never silently skipped. Results follow the access of the attachment's subject.

### Added — notification sinks (webhook and chat) (LT-6, 2026-10-03)

`notification_sinks`, `sink_deliveries`, `/api/me/notification-sinks`: a person registers a signed webhook or a Slack / Mattermost / Discord / Telegram destination for their own notifications. A notification only enqueues a delivery; a dispatcher sends it with bounded retries. Destinations must be public https hosts and are never returned in full.

### Added — PDF board pack and plan summary (PL-10, 2026-10-03)

`GET /api/board/pack.pdf` and `GET /api/plans/{pid}/summary.pdf`: server-rendered documents. The board pack uses the same data function and authorization as the JSON; the summary is masked and carries no people references; both are byte-identical for fixed data when stamped with `as_of`. Unsigned.

### Added — plan links (LT-8, 2026-10-03)

`plan_links` and `/api/plans/{pid}/links`: typed links (docs, repo, ticket, design, chat, other) from a plan to where its documents live. Only absolute http(s) URLs with a host; the service never fetches or renders them.

### Added — guest access (PL-7, 2026-10-03)

`guest_grants` and `/api/plans/{pid}/guests`: scoped, time-boxed, revocable access for outside collaborators (a token with `role=guest`). A guest sees only the plans they hold a live grant on, masked, plus comments if granted; everything else is 404 and they cannot write or enumerate.

### Added — bulk import / export of plans (T-8, 2026-10-03)

`bulk_jobs` and `review_queue_items` (migrations `…_000017`, `…_000018`);
`POST /api/plans/import` (the raw body; `Idempotency-Key`), `POST
/api/plans/export`, job status / download / error-report routes, the review
queue (`confirm` / `reject`) and an artifact sweep; a loco `BulkWorker`. JSONL
and CSV (every nested field as JSON in its cell) are both lossless. A row is
matched by `pid`, a globally-unique external identifier, or `owner_org_id` +
`code`; keyed rows are created, updated or reported unchanged (re-import is
idempotent); a keyless probable duplicate is held for review, never created or
merged. Counts always reconcile; the error report names the field, never the
value. Exports mask people references by default (`full` and soft-deleted rows
need the destructive right), are byte-stable, and are audited even at zero rows.

### Added — file attachments and the artifact store (LT-5, 2026-10-03)

`artifacts`, `attachments`, `/api/attachments`: attach a file (raw body) to a plan, task, risk, proposal, idea, canvas item, budget line or scenario. Type allow-list and size cap enforced before storage; SHA-256 re-verified on download; downloads re-authorized on the subject and served with `nosniff` and a UTF-8-safe `Content-Disposition`.

### Added — personal planner (PL-4, 2026-10-03)

`planned_slots`, `/api/me/planned-slots`, `/api/me/plan`: block time for a task and see a local day's slots in your own zone. Overlaps are reported as clashes (back-to-back is not one); a slot never changes the task's due date; slots also appear in the calendar feed as timed events.

### Added — live board updates (SSE) (PL-3, 2026-10-03)

`GET /api/plans/{pid}/live`: a server-sent-event stream of task created / moved / updated and comment added on a plan, with `Last-Event-ID` resume from a bounded ring and an explicit `resync` when a gap cannot be replayed. In-process hub: per replica.

### Added — plan templates (LT-2, 2026-10-02)

`plan_templates` and `/api/plans/{pid}/save-as-template`, `/api/plans/from-template/{tpl}`: save a plan tree as a template (no people, no identity, dates as offsets) and create a fresh tree from it with a start date. Tasks restart at the workflow's initial state; every plan goes through the normal create path so events and audit are written.

### Added — REST `include=` (RM-12, 2026-10-02)

`GET /api/plans/{pid}?include=…`: embed milestones, risks, tasks, components, time entries or budget lines in one response. Absent `include` the response is unchanged; a disabled module's sub-resource is omitted and listed rather than silently missing.

### Added — timesheets (LT-11, 2026-10-02)

`timesheet_weeks` and `/api/timesheets`, `/submit`, `/decide`: a person's week as a grid by day and plan (integer minutes, billable and non-billable never merged), submitted by its owner and approved or rejected by an approver; an approved week is locked against new entries.

### Added — bulk task edit, copy and move (RM-9, 2026-10-02)

`POST /api/tasks/bulk`, `/copy`, `/move`: change, copy or re-home many tasks in one call, with a result per task and counts that reconcile. A move re-validates against the target workflow and states any status it had to map; `status` is not bulk-editable.

### Added — private plans (RM-8, 2026-10-02)

`plan_access_settings` and `GET/PUT /api/plans/{pid}/access`: make a plan private for named members. A non-member gets `404` (never `403`) on the plan and everything beneath it; lists, duplicate checks, discussion, watchers, approvals, custom fields, task relations and My work omit it; members and admins see it. **Cross-plan aggregate views still include a private plan's figures.**

### Added — iCalendar and Atom feeds (LT-7, 2026-10-02)

`feed_tokens` and `/api/feeds/calendar.ics`, `/api/feeds/me.atom`: per-user revocable token (only its hash stored). The calendar carries the owner's due tasks plus the milestones and sprint dates of those plans (RFC 5545, stable UIDs); the Atom feed carries their notifications, with ETag / 304. Exempt from the bearer guard; scope is the owner's own work.

### Added — subtasks and the derived roll-up (RM-5, 2026-10-02)

`tasks.parent_task_pid` and `GET /api/plans/{pid}/tasks/{t}/rollup`: a parent's due date, estimate, estimate-weighted progress and spent time derived on read from its subtasks, with null + reason for anything that cannot be derived and unestimated children named rather than counted as zero.

### Added — task relations (LT-10, 2026-10-02)

`task_relations` and `/api/plans/{pid}/tasks/{t}/relations`: related, blocks, precedes, duplicates, copied_from. `blocks` stops the blocked task finishing, `precedes` constrains due dates (also at edit), finishing a primary closes its duplicates.

### Added — approval requests (PL-1, 2026-10-02)

`approval_requests` and `/api/approvals`: ask one named person to approve a plan, task, risk, proposal, idea, canvas item, budget line or scenario. Only that person decides, a decision is final, a rejection must carry a comment, only the requester withdraws. Budget lines and scenarios are now also valid comment / watcher subjects.

### Added — watchers (RM-3, 2026-10-02)

`watchers` and `/api/watchers`, `/api/me/watching`: follow a plan, task, risk, proposal, idea or canvas item. A watcher is told once of a comment or task move, never the actor. Auto-watch on create / assign / comment, switchable off with the `auto_watch` preference.

### Added — per-role workflow transitions (RM-2, 2026-10-02)

`workflow_transitions.roles`: a workflow transition may name the roles allowed to make it (empty means anyone, so every existing workflow is unchanged). A restricted move is refused `403` naming the roles and writes no transition row. Roles are the token's `role` attribute.

### Added — typed custom fields (RM-1, 2026-10-02)

`custom_field_defs` / `custom_field_values` and `/api/custom-fields` (definitions, values, search). Types text, date, bool, integer, list, multi_list; plan fields may be kind-scoped; a required task field blocks task creation; values are stored apart from the plan payload so they never reach the matcher; withdrawing a definition keeps stored values readable.

### Added — task components (PL-11, 2026-10-02)

`components` (migration `m20261003_000002_components`) and `tasks.component_pid`: five routes under `/api/plans/{pid}/components`, a `?component=` filter on the task list, and `/components/summary` whose buckets always sum to the plan's task total (every component, empty ones at zero, plus `unassigned`). A task's component must belong to its own plan; deleting one in use is `409`.

### Added — per-plan feature modules (RM-7, 2026-10-03)

`plan_modules` (migration `m20261003_000001_plan_modules`); `GET/PUT
/api/plans/{pid}/modules`; `POST /api/plans?modules_preset=personal`. A plan
exposes a set of modules (`tasks`, `time`, `risks`, `financials`, `milestones`,
`governance`); a plan-scoped route of a disabled module answers `404
module_disabled`, for every method, from a middleware layer. Disabling never
deletes data. No explicit set means the deployment default (all modules unless
`PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES` narrows it), so existing plans
are unchanged. Every change is audited with the before and after sets.

### Added — notification preferences (PL-5, 2026-10-02)

`notification_prefs` (migration `m20261002_000004_notification_prefs`);
`PUT/GET/DELETE /api/me/notification-prefs` and `GET /api/me/inbox?tz=`. A user
may switch a notification kind off (nothing is written for them; everyone else
is unaffected) or set it to a daily digest, which collapses the local day's
events for one subject into a single inbox line listing every member. No
preference means today's behaviour. `models::capabilities::notify` now returns
`Result<Option<_>>`: `None` is a recipient opt-out, reported by the automation
engine as `skipped`.

### Added — "My work" and the time-zone doctrine (LT-3 / LT-9, 2026-10-02)

`GET /api/me/tasks?tz=<IANA>[&as_of=]`: the caller's open tasks in five buckets
(`overdue` / `today` / `this_week` / `later` / `undated`, all always present),
computed in **the caller's** zone. New `tasks.due_date` (a zone-free `DATE`;
migration `m20261002_000003_task_due_date`) accepted on task create and update.
Plans the caller cannot read are omitted and counted (`omitted_unreadable`). A
fixed offset or abbreviation for `tz` is refused 422 (it cannot follow daylight
saving). Doctrine in `src/tz.rs` and spec §7.x: store UTC; dates carry no zone;
the server never reads its own zone (`chrono::Local` banned by `clippy.toml` and
a test); portfolio-wide views stay on the UTC date.

### Added — comments and reactions (LT-4, 2026-10-02)

`comments`, `comment_edits` (append-only) and `reactions` (migration
`m20261002_000002_comments_reactions`); routes `/api/comments[/{pid}[/history]]`
and `/api/reactions`. Subjects: plan, task, risk, proposal, idea, canvas item.
A plan-owned subject applies the owning plan's record-level read check. Only
the author edits or deletes; every edit keeps the previous body; a repeat
reaction is idempotent and every reaction kind is counted, zero included. A
comment notifies the subject's owner and earlier commenters, never the author,
once each.

### Added — strategy canvases (LT-1, 2026-10-02)

`canvases` / `canvas_items` (migration `m20261002_000001_canvases`) and nine
routes under `/api/canvases` + `/api/canvas-templates`. Seven built-in
templates (Lean, Business Model, SWOT, empathy map, logic model, risk, goal)
live in `src/canvas.rs` as box lists, so a new canvas type is data, not a
migration. A canvas may be plan-scoped or free-standing; a rendered canvas
reports **every** box, empty ones at count 0. An item can be **promoted** to
an idea, a risk (plan-scoped canvas only; probability and impact required,
never defaulted) or an objective, in one transaction with an audit row naming
the source item; a second promotion is `409`.

### Added — real-Keycloak container test (2026-10-02)

`tests/keycloak_container.rs` (feature `keycloak`, `#[ignore]`d): Testcontainers
starts Keycloak 26.0 under Podman, imports `tests/fixtures/keycloak/mxi-realm.json`,
and checks authentication, the role map and ABAC through the real guard. New
dev-dependency `testcontainers`. Run instructions: spec §11.2a.

### Added — report grouping (T-28e, first half, 2026-09-20)

`report_definitions.group_by` (migration `m20260920_000001_report_group_by`,
nullable; one of `kind` / `stage` / `status` / `start_date` / `target_date`).
A grouped `GET /api/reports/{pid}/run` returns one entry per group value with
the plan count and the group's budget lines summed **per currency** — never
across currencies (`insights::group_report`); `?format=csv` emits one row per
group × currency. Ungrouped definitions run unchanged. Scheduled runs remain
open, blocked on T-8.

### Added — optional Keycloak bearer acceptance; PASETO is now a feature (KC-1, 2026-09-29)

`keycloak` cargo feature (off by default): the blanket guard and the
`AuthUser` / `MaybeAuthUser` extractors also accept an
`Authorization: Bearer <Keycloak access token>`, verified by
`authentication-verifier`'s `keycloak` module and mapped onto the same
`Claims`, so ABAC, masking and audit are unchanged. Configured by
`PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_URL` / `_REALM` / `_AUDIENCES` /
`_ROLE_MAP[_FILE]` / `_REQUIRE_VERIFIED_EMAIL`; unset URL => off; a malformed
configuration logs an error and leaves Keycloak off (fail-closed, service
still boots). A token beginning `v4.` is a PASETO; anything else is offered
to Keycloak. An unmapped Keycloak role grants nothing. New
`auth::request_claims` and `auth::enforce_request` (async) are what the
middleware and extractors use; the sync `enforce` / `bearer_claims` remain
for the PASETO tests.

PASETO verification (`Verifier`, the boot key fetch, the rotation refresh)
moved behind the `paseto` feature, **on by default**, so a default build is
unchanged. `--no-default-features --features keycloak` builds a service with
no PASETO code. New `tests/keycloak_guard.rs` (DB-free) pushes real RS256
Keycloak-shaped tokens, served by a local OIDC provider
(`authentication_verifier::test_idp`), through the real guard. See
`agents/share/runbooks/keycloak-sso.md`.

### Added — opt-in registration of the four standard controls (T-26, PRO-P33)

`POST /api/plans/{pid}/controls/register-standard`: registers
`gate_readiness` (feedforward, fixed at 100%), `work_in_progress` and
`cycle_time_p85` (concurrent, caller-supplied — no default, since a WIP
limit and a cycle-time SLE are plan-specific commitments this service
has no basis to invent), and `budget_variance` (feedback, defaults to
a 10% tolerance, overridable). **Opt-in and per-plan, never
automatic** — a feedforward control's whole design intent is to be
able to block a write once something enforces that, so silently
registering one on every plan would be an unrequested behavioural
change. Idempotent per metric. Retrospectives are deliberately not
included — no metric exists for "a retrospective happened," and this
endpoint does not invent one. See `spec/13-tasks.md` T-26 for the full
decision record, including the verified (not assumed) fact that no
write path in this service currently enforces a feedforward control's
block at all.

### Added — per-user saved views (T-28l)

`POST`/`GET`/`DELETE /api/saved-views[/{pid}]`: a route-scoped filter/
sort/column preset, keyed by the caller's token `sub` and nothing else
identity-shaped (no email, no name). Gated by the required-auth
`AuthUser` extractor — a saved view with no owner makes no sense —
independent of the blanket `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`
flag. Every query is scoped to the caller's own `sub`; another user's
view is `404` on read or delete, not `403` — its existence is not
disclosed across users. `GET ?route=` narrows to one route (a saved
view is never applied outside the route it was saved from); omitted,
it lists all of the caller's own views.

Verified end to end with two real minted PASETO identities in a
dedicated test binary (`tests/saved_views.rs`) — needed because the
`verifier` is a process-wide `OnceLock`, the same reason
`tests/enforcement.rs` has its own binary.

### Added — intake demand forecast (T-28i)

`GET /api/proposals/forecast` answers "how many approved proposals in
the next N periods?" by reusing the exact throughput Monte-Carlo
behind `GET /plans/{pid}/forecast`
(`crate::tba::{throughput_history, forecast_items}`, verbatim, no new
pure logic) over the intake pipeline's own `proposal_approved` audit
events instead of a task board's completions. Same seed determinism,
same refusal below `MIN_THROUGHPUT_PERIODS`, and the response names
the history window (`from`/`to`/`periods`/`period_days`) it drew from.
`arrivals_per_period` (`proposals.created_at`) rides alongside for
context but does not feed the forecast.

`proposals` carries no `approved_at` column, so the approval instant
comes from the audit trail (`audit_logs` rows with action
`proposal_approved`) rather than the table itself. See
`spec/13-tasks.md` T-28i.

### Added — deterministic scenario generator (T-28h)

`POST /api/scenarios/generate` builds a **draft scenario** by scoring
every active plan (Smart Score, reusing the exact `Estate` evidence
gathering `GET /prioritization` already uses) and pricing it (budget
lines summed in the cap's currency), then greedily selecting by score
per unit cost within an optional budget cap. Every candidate gets a
rationale row: included with its score and cost, or excluded with
`no_score` / `foreign_currency` / `over_cap` / `must_include_conflict`.
A `must_include` candidate is force-included even when it alone
exceeds the cap — never silently dropped. The generated scenario is an
ordinary `draft`, so `evaluate`/`compare`/`commit`/`rollback` (T-28a)
apply unmodified. The selection itself
(`src/strategy.rs::generate_scenario`) is a pure function, unit-tested
for determinism directly.

Candidates are plans only, never proposals — Smart Score has no
defined evidence trail for a not-yet-promoted proposal. See
`spec/13-tasks.md` T-28h for this and a second scope note found while
writing the test: under the default weights every plan scores via
`momentum` alone (from `updated_at`), so a genuinely `no_score`
candidate needs `momentum` configured to `0`.

### Added — capacity-at-scale regression guard (T-28d)

`tests/requests/scale.rs::capacity_views_do_not_fan_out_with_scale`
seeds 60 plans with allocations across 40 shared people and proves
`GET /capacity`, `GET /capacity/utilization`, and `GET /at-a-glance`
each issue the same, bounded number of SeaORM driver round trips at a
5-plan baseline and at the full 60-plan scale — not timed, counted
directly via a tracing layer over SeaORM's own `#[instrument]` driver
spans (`sea_orm::driver::sqlx_postgres::*`), since no query-counting
test pattern existed anywhere in this repo and `pg_stat_statements` is
deliberately off on the family's test Postgres. Plus
`benches/service_bench.rs::bench_capacity_rollups`, a Criterion bench
over the pure rollup arithmetic (`visibility::summed_percent`,
`effort::utilization`) at the same scale.

This is a **regression guard, not a fix**: all three endpoints were
already query-bounded (confirmed by reading them directly) — the test
proves it and catches a future per-row query creeping in. See
`spec/13-tasks.md` T-28d for the tracing-callsite-caching mechanics the
counter had to account for.

### Added — skill-aware allocation (T-28c)

`allocations.skills_required` (JSONB, short tags) plus `GET
/api/plans/{pid}/skill-gap`, which resolves the assigned person's
skills live against the worker service by `EntityRef`
(`src/workers_client.rs`: lazy verify-on-read, TTL-cached in-process,
never persisted) and reports each required tag `covered`, `missing`,
or `unknown` (with a reason). A non-worker reference, an unconfigured
resolver, or a non-2xx/unreachable worker service all resolve
`unknown`, never `missing`.

The worker service carries no skills endpoint today (confirmed by
reading its `Worker`/`Assessment` models directly), so the resolver is
built against a documented, assumed contract
(`GET {base}/api/workers/{id}/skills`) rather than inventing that
feature inside the worker crate as a side effect of this task. See
`spec/13-tasks.md` T-28c for the full "decided rather than guessed"
note.

### Added — scenario rollback and evaluation provenance (T-28a)

`POST /api/scenarios/{pid}/rollback` un-commits a `committed` scenario
(status → `draft`, `committed_at` → `null`, audited as
`scenario_rolled_back`), refused `409` unless the scenario is
currently `committed` (a new `conflict()` helper). `GET
/api/scenarios/{pid}/evaluate` and `GET /api/scenarios/compare` now
carry `as_of` (when the read ran) and `inputs_read` — every live
`budget_line`/`risk`/`objective_link`/`proposal` row the evaluation
summed, each with its own `updated_at` — so two evaluations of the
same scenario that disagree can say why.

Scoped deliberately narrower than the task's original text, which
described rollback restoring "each member's funding state to what the
commit replaced": `commit_scenario` has never mutated a member's
`budget_lines`/`allocations`/any other row, only the scenario's own
`status`/`committed_at`, so there is no member funding state to
restore and none is invented. See `spec/13-tasks.md` T-28a for the
full "decided rather than guessed" note.

### Added — deadline-shift trigger and rescheduling (T-28g)

Two new automation triggers, narrowed to one field each like
`milestone_due` was: `plan_timeframe_changed` (fires from
`controllers::plans::update`) and `milestone_due_changed` (fires from a
**new** `PUT /api/plans/{pid}/milestones/{m_pid}` reschedule endpoint —
none existed before, a milestone could previously only be created and
completed). Two new actions: `propose_reschedule` (walks direct
`plan_dependencies` successors, computes each one's implied shift, and
writes a notification carrying the proposal — moves nothing) and
`shift_dependents` (applies the same computation instead of only
proposing it, without re-entering the engine).

`src/automation.rs` gains `SuccessorFact`/`ProposedShift`/
`propose_shifts()` (8 unit tests). `src/controllers/automation.rs`
gains `load_successors`/`act_propose_reschedule`/
`act_shift_dependents`; `apply_action` now threads the firing's
`action_index` through (bundled into a new `FiringContext` to stay
under clippy's argument-count lint), so `shift_dependents` can log one
`automation_runs` row per successor actually moved, in addition to the
one row the engine already logs for the action as a whole. 3 new
DB-gated request tests in `tests/requests/capabilities.rs`, including
one proving `shift_dependents` does not cascade into a second rule
watching the successor's own `plan_timeframe_changed`.

Two scope decisions recorded in `spec/13-tasks.md` T-28g rather than
guessed: every successor is proposed the **same** delta as the
predecessor's own shift, regardless of lag (a fixed lag does not change
how a delta propagates through it — lag only matters for the
already-violated check); and `shift_dependents` re-derives the shift
live rather than parsing back a prior `propose_reschedule`
notification, since `notifications.message` carries no structured
payload and re-deriving guarantees the two actions can never disagree.

`cargo test --lib` 406/406, DB-gated suite 88/88 (was 85), clippy
`-D warnings` clean, fmt clean.

### Added — phased budget baseline → EAC/ETC forecast (T-28b)

New pure module `src/financials.rs` (`Baseline`/`BaselinePeriod`/
`Forecast`/`EtcSource`/`ForecastAbsent`, `forecast()`/
`rollup_forecast()`, 10 unit tests) and HTTP surface
`src/controllers/financials.rs`:

- `POST`/`GET /api/plans/{pid}/budget-baselines` — approve a
  baseline version (planned cost per period, one currency, frozen at
  approval); append-only, a re-baseline needs a reason, the
  predecessor stays reproducible from its own version.
- `GET /api/plans/{pid}/financials/forecast` — `EAC = AC + ETC`, ETC
  from the plan's latest TPC cost-estimate-to-complete where
  recorded, else the baseline's own not-yet-elapsed periods; a plan
  with neither reports `null` with `no_currency_signal`, unchanged.
- `GET /api/financials/forecast?plan=&depth=` — the same forecast
  rolled over `parent_ref` (reuses `tba::walk_descendants` verbatim),
  one row **per currency**, never merged into one sum.

`GET /plans/{pid}/performance`'s `spi`/`cpi` stay `null` — a baseline
alone is not earned value — but the reason now distinguishes
`no_baseline` from a new `no_earned_value_signal` once a plan actually
has one, so the message stays true rather than becoming a stale lie.
See `spec/13-tasks.md` T-28b for the two scope decisions this made
explicit rather than guessing (why SPI/CPI still aren't wired, and how
the forecast's own currency is resolved).

Migration `m20260918_000001_budget_baselines`: `budget_baselines` +
`budget_baseline_periods`, both append-only (no `deleted_at`, matching
`phase_transitions`' precedent). 6 new DB-gated request tests in
`tests/requests/financials.rs`, verified against a real Postgres
(`scripts/ci-check.sh test-db`); `cargo test --lib` 400/400, clippy
`-D warnings` clean, fmt clean.

### Fixed — a hardcoded `spent_on` date aged out of its own test's window

`tests/requests/effort.rs::someone_on_leave_reports_null_not_zero_percent`
recorded a time entry at a fixed `"2026-08-20"` and read it back through
`GET /api/capacity/utilization?window_days=28`. That passed for exactly
28 calendar days after the date was written and then failed on every date after
— caught by this PR's own `db` CI job on 2026-09-18, 29 calendar days past the
hardcoded date, unrelated to anything this PR actually changed. Fixed
by computing `spent_on` and the leave period relative to
`chrono::Utc::now()` (mirroring `tests/requests/insights.rs`'s existing
pattern) rather than a fixed calendar date. Verified locally against a
real Postgres: `scripts/ci-check.sh test-db
project-portfolio-management/project-portfolio-management-service-with-rust`,
79/79 passing.

### Added — explainability pin: every derived GET discloses its inputs (T-28j)

`src/openapi.rs`'s test module gains
`every_derived_get_discloses_its_inputs_or_is_exempt_as_plain_crud`: walks
every `GET` operation in the hand-written OpenAPI document (the same
mechanical enumeration `spec_and_mounted_routes_agree_both_ways` already
uses) and asserts each one either carries a substantial description /
known disclosure marker ("null", "unmeasured", "withheld", "as_of",
"reason", "even at zero", …) or sits in a new `NON_DERIVED_GET` register
for a genuinely plain record read — the same register-shrinks-only
discipline `KNOWN_UNDOCUMENTED` already established, backstopped by
`non_derived_get_register_is_accurate`. Running it against the document
as it stood found two real gaps: `/api/plans/{pid}/time-entries` was
correctly plain (moved to the register), and `/api/reviews/consensus`
was a genuine miss (an aggregate verdict with undocumented `null`-on-
no-scores / `null`-on-tie behaviour), fixed with a real `description`
rather than loosening the test. `project-portfolio-management/spec/
02-scope.md` §2.3 gains the "no model-driven assistant inside the
service" refusal this task also calls for, cross-referencing the test
by name.

### Added — rollup attrition record (TBA-12, repo `tasks.md` PA-3)

`GET /api/plans/{pid}/rollup` gains an `attrition` key: a named,
ordered, parent-linked trail (`root_plan` → `walked_plans` →
`tasks_scanned`, then a `finished`/`work_in_progress`/`not_started`
three-way partition) explaining the `combined` figures' denominator
inside the response. `src/tba.rs`: `AttritionStep`,
`RollupAttritionInputs`, `ROLLUP_ATTRITION_STEP_LABELS`,
`ROLLUP_ATTRITION_PARTITION_PARENT`, `rollup_attrition_trail`, 6 new
unit tests. A **reinterpretation** of care-pathway's T-14g CONSORT
attrition trail (`agents/share/time-based-analysis.md` §8), not a
literal port: this endpoint's cohort is reached by a bounded
containment tree walk with a per-plan task-count cap, not a linear
status/window/suppression screen, so the step vocabulary names the
tree walk's own `truncated`/`revisits` and the per-plan `MAX_TASKS`
cap rather than reusing care-pathway's literal labels. See
`project-portfolio-management/spec/time-based-analysis.md` §7.5/§15/§16
for the full account, including why patient-flow's leg of PA-3 was
corrected rather than built (its `time-analysis` endpoint serves one
stay, not a cohort — no denominator to explain).

### Added — outbound signed webhooks as a relay sink (T-28m, root `tasks.md` EV-3)

`src/webhooks.rs`: a `WebhookSink` beside the relay's existing
`LoggingSink`/`FluvioSink`, delivering the outbox envelope to
operator-configured URLs (`PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS[_FILE]`,
JSON, per-target event-kind filter). Every delivery is signed
(`X-Mxi-Signature`, HMAC-SHA256 over the exact request body through the
shared `integrity-mac` crate's new `Domain::Webhook`) and carries
`X-Mxi-Event-Id` for receiver-side dedup. A `5xx`/transport failure is
retried with exponential backoff; a `4xx` is not. Delivery is
**best-effort and never blocks the durable bus**: `WebhookSink::send`
spawns each delivery and returns immediately, and `src/relay.rs`'s new
`CompositeSink` keeps the primary sink's failure as the only thing that
leaves an outbox row unpublished. Every delivery's final outcome is
recorded in a new `webhook_deliveries` table. No MAC key configured ⇒
webhook delivery refuses to start (logged, the primary relay is
unaffected) rather than send unsigned deliveries.

Verified live against a real Postgres and three local mock HTTP
receivers, not just unit-tested: the sent signature was independently
reproduced from the root key and the published HKDF derivation; a
`503,503,200` receiver was retried exactly 3 times and a `400` receiver
exactly once, both confirmed in the delivery log; the no-key refusal
left zero deliveries while the primary outbox row still published.
`cargo test --lib` 382/382 (was 366); `cargo fmt`/`clippy --all-targets
-D warnings` clean in this crate and its `migration/` subcrate;
DB-gated suite 80/80. See this crate's `spec/13-tasks.md` T-28m and
root `tasks.md` EV-3 for the full record.

### Fixed — two container-boot defects, found via the T-28o runbook exercise (2026-09-07)

Writing `agents/share/runbooks/first-deployment.md` (shared family guidance, not in this repository)'s
family go-live runbook (root `tasks.md` EV-4) meant actually building
and booting this crate from a fresh container against a real Postgres,
not reading the prior 2026-08-03 "verified end-to-end" claim as still
current. Two real, previously-undiscovered defects turned up:

1. `config/production.yaml`'s dead loco JWT `auth:` block —
   `secret: {{ get_env(name="JWT_SECRET") }}` had no `default` — crashed
   boot with no `JWT_SECRET` env var set, for a value this crate reads
   *nowhere* (`loco-rs` is built without the `auth` Cargo feature; this
   family issues PASETO v4.public, never JWT — `agents/share/jwt.md`).
   loco's own `Config.auth` field is `Option<Auth>`, so the fix is
   deleting the block entirely, not defaulting it. The identical defect
   was found and fixed in organization's, care-pathway's, and case's
   `production.yaml` too.
2. `Dockerfile` never `COPY`'d `benches/` — this crate's `Cargo.toml`
   declares `[[bench]] name = "service_bench"`, and Cargo refuses to
   parse the manifest at all without that path present, even for a
   `--bin`-only build. The identical gap existed in case-service's
   Dockerfile and (independently) in organization-service's.

Both fixed and re-verified: `podman build` from the repo root, `db
migrate` against a real Postgres, `start`, then
`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` toggled off→on against the
live container confirming `GET /api/plans` goes `200`→`401` while
`/_health`/`/metrics.prom` stay `200` throughout — exactly what the new
runbook documents. See root `tasks.md` EV-4 and this crate's own T-28o.

### Security — gate the oversight bulk-read endpoints as a privileged read (SEC-PPM-3)

`evidence_pack`, `auditor_trail`, `board_pack`, and `board_investments`
relied solely on the coarse `/api/*` blanket guard, so under the
built-in default ABAC policy any signed-in caller — not just
`svc`/`admin` — could pull the full audit trail or board investment
register. A new `authorize_oversight_read` helper (mirroring
case-service's `authorize_bulk`, SEC-G1) gates all four at
`Action::Destructive`, a no-op when
`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` is off. New assertions in
`tests/enforcement.rs` prove `403` for a plain-read-tier token and
`200` for `access=admin`/`svc=true`, verified against a real Postgres.
`cargo test --lib` 368/368; `cargo clippy --all-targets -- -D
warnings` / `cargo fmt --check` clean. See spec/index.md SEC-PPM-3.

### Fixed — doc drift: stale MSRV 1.95/N-3 reference (2026-09-06)

`Cargo.toml` already declares `rust-version = "1.96"` (matching
`ci/msrv.txt` and the repository's current **N-2** policy), but the
`[0.3.0] - 2026-08-26` entry below still says `"1.95"` and
"current-stable-minus-three" — the policy in effect when that entry
was written, since tightened. That entry is left as-written (a dated
record of what was true then); this entry is the correction, matching
the pattern already used for course-service (T-29) and
authentication-verifier (AV-3). No behaviour change — `Cargo.toml`,
`ci/msrv.txt`, and `scripts/ci-check.sh msrv` already agreed on 1.96
before this change.

### Fixed — `list`/`search`/`check-duplicates` could disclose a plan the caller's direct read denies (SEC-PPM-1)

`GET /{pid}` and `GET /{pid}/export` were the only handlers running a
record-level ABAC decision (`crate::auth::authorize_record`); `list`,
`search`, and `check_duplicates` returned every plan's pid/name (or
scored hit) unconditionally. With
`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` on and a policy that
**denies** read past some phase gate (`resource.stage`, PPM-3), those
three collection endpoints still surfaced a denied plan's existence and
name — a collection read disclosing more than the equivalent single
read (`agents/share/security.md` invariant 5). A new `readable_refs`
helper filters `list`/`search` rows through `authorize_record` per row
(keyed on the row's own `stage` column), omitting a denied one rather
than erroring the whole call; `check_duplicates` gained a
`MaybeAuthUser` extractor (it took none before) and applies the
identical per-candidate filter before scoring a hit. With enforcement
off (the default) or no deny rule configured, this is a no-op. New
DB-gated test (`tests/list_search_check_duplicates_authz.rs`) advances
one plan through gate 0 and proves a denied caller's
`list`/`search`/`check-duplicates` all omit it while still surfacing an
ungated plan, and that an unrestricted caller sees both everywhere. See
spec/index.md SEC-PPM-1 (including a correction to the task's original
"mask the lead/owner-org fields" framing — `PlanRef`/`ScoredRef` never
carried those fields to begin with, so the real, provable gap was
authorization filtering, not field redaction — the same finding
care-pathway-service's twin task CP-T2 made).

### Security — audit the oversight bulk-read/export endpoints (SEC-PPM-2)

`GET /api/auditor/evidence-pack` (bundles a period's decisions plus up to
2000 audit rows, JSON or CSV) and `GET /api/auditor/trail` (the
estate-wide audit-trail explorer) only ever *read* `audit_logs` to build
their response — neither wrote a row for the act of exporting/reading
it, unlike the posture `agents/share/bulk-import-export.md` §8 ("every
export is audited") and case-service's SEC-B8 require. Both handlers now
write an `audit_logs` row (actor, requested window, and — for the
evidence pack — format) before delivering the response, and a write
failure now refuses the response rather than silently under-recording
the disclosure. New DB-gated request tests
(`tests/requests/oversight.rs`) prove each endpoint writes exactly the
expected audit row, readable back via `GET /api/plans/audit/recent`.
See spec/index.md SEC-PPM-2.

### Added — control action → task conversion (T-26 remainder)

- **Control action → task conversion** (`POST
  /api/actions/{pid}/convert`): converts a `control_actions` row into a
  task on the control's own plan, in that plan's workflow-initial
  state, carrying the action's description as the task title and
  transactionally recording the task's opening transition — the same
  invariant `engineering::create_task` already upholds, so a converted
  task is analysable identically to a hand-created one. Stamps
  `converted_task_pid`; refuses a second conversion of the same action
  and a conversion of a closed one (`422`); an unknown action is `404`.
  Issue conversion is deliberately not offered: this service has no
  `issues` store yet (FR-14, still deferred), and the migration's own
  doc comment says actions convert into work stores that already
  exist, which issues are not — `converted_issue_pid` stays reserved,
  always `NULL`. Verified live against a fresh Postgres: 75/75 request
  tests (was 74, +1 for
  `converting_a_control_action_creates_a_task_on_the_plan`); `cargo
  fmt --check` / `cargo clippy --all-targets -- -D warnings` clean.

### Added — milestone-due date-arrival trigger (T-21, FR-32)

- A new `milestone_due` trigger kind fires when a milestone's own `due`
  date has arrived — the one dated field in the domain with
  unambiguous "arrived" semantics (a task's `due_at`/`start_at`/
  `finish_at` are each ambiguous about which one "the" date-arrival
  trigger would mean, so this ships narrowly on milestones rather than
  guessing a task-date convention).
- Unlike every other trigger, nobody's write makes a due date "arrive"
  — there is no event to hang the rule on — so this needed its own
  **exactly-once claim** rather than the existing fire-on-write path.
  New table `automation_milestone_fires (automation_pid, milestone_pid)`
  (migration `m20260902_000002_automation_milestone_fires`) with a
  `UNIQUE (automation_pid, milestone_pid)` constraint, claimed via
  `INSERT … ON CONFLICT DO NOTHING`. A suppressed conflict surfaces
  from sea-orm 2.0.2's `exec_with_returning` as `DbErr::RecordNotFound
  ("Failed to find inserted item")` — confirmed by running the claim
  twice against a real Postgres, not assumed from the crate source
  (which also defines a `RecordNotInserted` variant used by a
  different insert code path, `TryInsert`, not this one).
- New `POST /api/automations/milestones/sweep`
  (`sweep_milestone_due`): finds overdue, undone, non-deleted
  milestones (capped at `SWEEP_CAP`), and for each enabled
  `milestone_due` rule matching the milestone's plan, attempts the
  claim before applying the rule's actions — so a rule/milestone pair
  fires exactly once, ever, no matter how many times the sweep runs.
  The optional scheduler ticker now calls this sweep on every tick
  alongside the existing `sweep_due`; the endpoint also works
  standalone for a deployment driving both sweeps from external cron.
- `fire()`'s action-application logic is extracted into a shared
  `apply_rule_actions` helper so the write-triggered path and the new
  sweep log multi-action outcomes identically.
- Verified live: a new request test seeds one overdue and one
  far-future milestone, a matching `milestone_due` rule, sweeps twice,
  and confirms `fired: 1, already_claimed: 0` then `fired: 0,
  already_claimed: 1` — with exactly one `automation_runs` row
  throughout and the far-future milestone never firing.
  `cargo test --lib` 360/360 (was 358, +2); DB-gated suite 76/76 (was
  75, +1).
- **Field-change and SLE-breach triggers are deliberately not
  included** — both need a product decision (which fields count as
  "changed"; which SLE source and a once-only notification schema)
  that a mechanical implementation would otherwise have to invent. Left
  open in spec §13 T-21 pending that decision, on the same reasoning
  basis as the PRO-P33 controls-registration deferral.

### Added — multi-action automation rules (T-21, FR-32)

- A rule may now declare **more than one action**, applied in the
  array's declared order, with **each action's outcome logged
  separately**: `automations.action_kind`/`action_value` (one action)
  is replaced by `actions JSONB NOT NULL DEFAULT '[]'`
  (`[{"kind": …, "value": …}, …]`), and `automation_runs` gains
  `action_index` so a firing of an N-action rule writes N run rows
  instead of one silently overwriting the next. A `CHECK
  (jsonb_array_length(actions) > 0)` refuses an empty list even from a
  direct insert; migration `m20260902_000001_automation_multi_action`
  backfills every existing single-action rule into a one-element array.
- New pure `automation::validate_actions` (5 tests): validates every
  declared action with the existing per-kind `validate_action` and
  names the offending 0-based index on failure; capped at 20 actions
  per rule.
- `POST /api/automations`'s body changed from `action_kind`/
  `action_value` to `actions: [{kind, value}]` — a breaking change with
  **no back-compat shim**: there is no front-end consumer of this
  endpoint yet (repo `tasks.md` PRO-P20) and this service is pre-1.0
  with synthetic data only, so a clean cut was chosen over carrying two
  request shapes.
- Verified live against a fresh Postgres: a new request test seeds a
  two-action rule (`add_label` on an already-labelled plan, then
  `assign`) and confirms the first action logs `skipped` while the
  second still applies, proving one action's non-fatal outcome does
  not block the next. `cargo test --lib` 358/358 (was 353, +5);
  DB-gated suite 75/75 (was 74, +1).

### Added — OpenTelemetry OTLP export (PRO-H12 slice 7 of 7)

- Real OpenTelemetry OTLP export (`src/observability.rs`, repo
  `tasks.md` PRO-H12 slice 7 of 7 — the last): a `tracing-opentelemetry`
  bridge over an OTLP/gRPC span exporter, plus an OTLP/gRPC metric
  exporter feeding an `http.server.request.duration` histogram. On by
  default at `OTLP_ENDPOINT` (default `http://localhost:4317`); set it
  to the empty string to disable export and keep only local logging.
  Ported from case-service's slice 6; confirmed rather than assumed
  that this crate, like the other three loco-idiomatic registries, has
  exactly one router-construction surface, so `trace_mw` is layered
  once in `after_routes`. Proved end to end against a real in-process
  OTLP/gRPC collector (`tests/otlp_export.rs`, `tests/otlp_middleware.rs`),
  not merely against in-process SDK state. This closes repo
  `tasks.md` PRO-H12 — every entity registry in the family now
  exports real OTLP/gRPC traces and metrics.

## [0.3.0] - 2026-08-26

The project-management suite (entity spec §13 T-15 … T-27, landed
2026-08-25/26): custom workflows, the OKR engine, effort and time
tracking with per-person utilization, sprint ceremonies, project
phases, Flow Distribution, Total Project Control, the controls
register, and value realization / strategic performance — plus the
time-based-analysis completions (TBA-9/10/11) and the earlier
unreleased hardening below. Nine migrations
(`m20260825_000001` … `000006`, `m20260826_000001` … `000003`); the
embedded matcher moves to 0.2 for the `Plan.phase` wire addition.

### Added — custom workflows (T-15 / FR-26)

Pure `src/workflow.rs`; migration `m20260825_000005_workflows` (three
tables: `workflows`, `workflow_states`, `workflow_transitions`);
`src/controllers/workflow.rs` — `POST`/`GET /api/workflows`,
`DELETE /api/workflows/{pid}`, `GET /api/plans/{pid}/workflow`. The
task **create** and **move** paths now validate against the workflow in
force rather than a compile-time constant.

- **Resolution order:** the plan's own workflow, else the deployment
  default, else the built-in vocabulary — a plan with nothing
  configured behaves exactly as before. **An empty transition set means
  unconstrained**; constraint is opt-in.
- **Schema-enforced:** every state's `category` is `NOT NULL` + CHECK
  (an uncategorized state is impossible even by direct insert), and a
  partial unique index makes two initial states impossible.
- **Two defects found by testing, both fixed:** `done_at` was stamped
  on the literal string `"done"`, so a board whose final column was
  renamed (`shipped`) never stamped it — now stamped from the state's
  **category** on both paths. And the TBA flow classes were keyed on
  status names, so a custom vocabulary arrived with no classification;
  `workflow::default_flow_classes` now derives classes from the
  categories, overlaid with `tba::default_classes()` so the built-in
  `in_review` stays *necessary* non-value-adding rather than silently
  raising every untouched board's flow efficiency (pinned by a test).
- **Not built:** a workflow edit route (withdraw and re-register);
  issue workflows are resolvable but unused (no issues sub-resource).

### Added — the OKR engine (T-16 / FR-27)

Pure `src/okr.rs`; migration `m20260825_000006_key_results`
(`key_results` + `key_result_check_ins`); `src/controllers/okr.rs` —
`POST`/`GET /api/objectives/{pid}/key-results`,
`POST`/`GET /api/key-results/{pid}/check-ins`,
`GET /api/plans/{pid}/okr`.

- **Key results hang off `objectives`, not `goals[]`** — the spec was
  corrected first: `Goal` carries no identifier (addressable only by
  array position, so a `goal_id` binding would be orphaned by any
  reorder), while an objective has a `pid`, a `period` (the OKR cycle),
  and weighted plan alignment through `objective_links`. The plan score
  is weighted by that existing link weight, not a second notion of
  importance invented for OKRs.
- A `decrease` key result starts at its baseline and reads **0%**, not
  100%; a check-in moves `current_value` and leaves `start_value`
  untouched; an unmeasured objective is reported, never scored as
  zero; start-equals-target is refused at write. `maintain` without a
  tolerance and a `currency` metric without a currency code are
  CHECK-refused as well as handler-validated.

### Added — effort / time tracking (T-17 / FR-28)

Pure `src/effort.rs`; migration `m20260826_000001_effort`
(`time_entries`, `working_time_configs`, `non_working_periods`);
`src/controllers/effort.rs` — `POST`/`GET
/api/plans/{pid}/time-entries`, `GET /api/plans/{pid}/effort`,
`POST /api/working-time`, `POST /api/non-working`,
`GET /api/capacity/utilization`. Roll-ups per plan, task and assignee,
every one labelled **asserted**; uncategorized effort is reported
separately rather than folded into `opex` (which would flatter the
capitalizable share); an entry over 1440 minutes for one date is
refused — a day cannot hold more.

### Added — per-person utilization (T-24 / FR-35)

`GET /api/capacity/utilization?by=plan|team|person`, under the five
stated obligations of `agents/share/time-based-analysis.md` §7.1.

- Somebody on leave for the whole window reports `null` with
  `all_non_working`, **never 0%** — leave leaves the denominator, and
  0% would read as measured idleness. No declared capacity is its own
  reason (`no_declared_capacity`): an unknown denominator, not zero.
- Below the suppression floor the figure is withheld with its inputs
  still returned; team utilization sums numerator and denominator
  (never a mean of ratios); at or over 100% is flagged, not clamped.
- Per-person cycle time, throughput and flow efficiency remain
  **absent from every endpoint** — the family-wide refusal stands.

### Added — sprint ceremonies (T-18 / FR-29)

Migration `m20260826_000002_ceremonies` (`ceremonies` +
`sprint_commitments`); `src/controllers/ceremony.rs` — `POST`/`GET
/api/sprints/{pid}/ceremonies`, `POST /api/sprints/{pid}/commit`,
`GET /api/sprints/{pid}/commitment`. The retrospective already existed
as `sprint_notes`; this adds planning, daily and review, and the
commitment snapshot. **The commitment is written once** — a second
`commit` is refused (handler *and* partial unique index), because a
rewritable commitment would let mid-sprint scope look like scope
committed at the outset; the view **names** what was added and removed
afterwards. Every ceremony kind is reported even at zero, so a sprint
that never retrospected is a finding, not a missing row.

### Added — project phases (T-19 / FR-30)

Pure `src/phase.rs`; migration `m20260825_000003_phase_transitions`
(denormalized `plans.phase` with a CHECK, an append-only
`phase_transitions` log, no backfill); `src/controllers/phase.rs` —
`PUT /api/plans/{pid}/phase`, `GET /api/plans/{pid}/phase-history`.

- One-step advancement; a skip is `422` **and names the phase
  skipped**; a backward move needs an explicit reason; an unknown token
  is refused, not coerced. Per-phase durations partition the elapsed
  time exactly and survive unsorted input and clock skew. `DELETE` on
  the history is `405` — append-only expressed as an absent route.
- The matcher gains `PlanPhase` + `Plan.phase` (0.2), informational-only
  and pinned never-scored, following the `PlanStatus` precedent.
- **Known gap, recorded not hidden:** `plans.phase` is not a separate
  field in the integrity pre-image (its authoritative value rides in
  `data`, which *is* covered); closing it means a
  `RECORD_HASH_VERSION` bump that would read as a false tamper alarm
  estate-wide, so it waits for a change that needs one anyway.

### Added — Flow Distribution (T-20 / FR-31)

Pure `src/distribution.rs`; migration `m20260825_000004_flow_type`
(nullable CHECK-constrained `tasks.flow_type`);
`src/controllers/distribution.rs` —
`GET /api/plans/{pid}/flow-distribution` with the subtree rollup.

- **The work type is declared, not derived** — the spec was corrected
  first: the specified derivation read fields that do not exist
  (`tasks` has no `goal_id`; there is no `issues` table), and would
  have classified everything `unclassified` while appearing to work.
- `unclassified` is **not storable** (it is the *absence* of a
  declaration); a declared intended mix produces gaps only for the
  types it names; a malformed intent is warned about and ignored
  wholesale. **A bug the tests caught:** an out-of-range intended share
  produced a gap of ≈ −9.2e18 via unchecked arithmetic — now
  range-guarded where the arithmetic is.

### Added — Total Project Control (T-25 / FR-37)

Pure `src/tpc.rs` — DIPP (EMV / CEC), the progress index, banding, the
stored-vs-computed divergence, and currency-scoped triage; money in
minor units, ratios in basis points, **no float**. Migration
`m20260825_000001_total_project_control` with
`dipp_progress_index_ratio` as a Postgres **`GENERATED ALWAYS`** column
(`NULLIF` denominator, so a zero baseline is `NULL` rather than an
insert-time division error, and the ratio can never disagree with its
own inputs) and a CHECK refusing a negative cost-estimate-to-complete.
`src/controllers/tpc.rs` — `POST`/`GET /api/plans/{pid}/tpc`,
`GET /api/plans/{pid}/tpc/report`, `GET /api/tpc` (triage).

- `CEC = 0` reports `null` **with a reason**, never infinity; a
  negative EMV is not clamped and bands `value_destroying`; triage sets
  aside a foreign currency and an undefined DIPP rather than ranking
  them as zero; two plans with equal remaining value and cost rank
  identically however much has been sunk into either — the property
  the whole metric exists for. Mapping doc:
  `../spec/total-project-control/index.md`.

### Added — the controls register (T-26 / FR-38, FR-39)

Pure `src/controls.rs` — the three timings (feedforward / concurrent /
feedback) and the response each permits, standard validation against
the metrics the service actually produces, four comparators, and the
coverage rollup. Migration `m20260825_000002_controls` (three tables;
CHECKs on `timing`, `comparator`, `verdict`, `kind` — the `verdict`
CHECK keeps `unmeasured` a real third value, so a typo cannot fall
through as "not a fail"). `src/controllers/controls.rs` — `POST`/`GET
/api/plans/{pid}/controls`, `GET /api/plans/{pid}/controls/coverage`,
`GET /api/controls/coverage`, `DELETE /api/controls/{pid}`,
`POST`/`GET /api/controls/{pid}/readings`,
`POST /api/readings/{pid}/actions`.

- An unknown metric is refused at **registration**, not left
  permanently unmeasured; a valueless reading is `unmeasured`, not a
  pass, and is excluded from the pass rate; a failing reading with no
  action and no explicit `accept` appears as **unanswered**; every
  timing appears in coverage even at zero.
- **Open:** action → task/issue conversion (the `converted_*_pid`
  columns exist, always `NULL` today), and registering the controls
  that already exist in all but name (gate readiness, WIP limits, the
  SLE, retrospectives).

### Added — value realization + strategic performance (T-22 / T-23, FR-33 / FR-34 / FR-36)

Pure `src/value.rs`; migration `m20260826_000003_value`
(`business_case_targets`, `value_points`, `adoption_snapshots`,
`satisfaction_responses`); `src/controllers/value.rs` — `POST
/api/plans/{pid}/{business-case,value-points,adoption,satisfaction}`,
`GET /api/plans/{pid}/{value-realization,performance}`.

- **A plan with no value points is `unrealized`, never a total loss.**
  `approved_at` has no update path, and a second first-measurable value
  point is refused by a partial unique index — the Time-to-Value clock
  stops once, and Time to Value is a distribution (nearest-rank
  p50/p85), never a mean.
- Adoption refuses a zero denominator at write and stores its own
  `definition` and `window_days`; investment is actual cost, and
  **mixed currencies withhold the ROI** rather than adding pounds to
  euros (the `budget_lines` shape carries a currency per line). The
  evidence mix (`measured_share_basis_points`) is disclosed.
- Strategic performance: NPS always carries its response count and
  reports `null` with `no_responses` rather than zero; responses store
  a **role, never an identity**; **SPI and CPI report `null` with
  `no_baseline`, never `1.0`** — a plan without a phased budget
  baseline is unmeasured, not on plan. Not built: the phased baseline
  itself, NPV, Strategic Alignment Index, defect density — each needs
  an input the service does not hold.

### Added — `plan_phase_changed` automation trigger (T-21 partial / FR-32)

Wired into the phase controller; deliberately its **own** trigger
rather than folded into `plan_stage_changed` — the gate stage and the
project phase are separate ordered vocabularies, and one rule firing on
both would fire on the wrong kind of change half the time. Phase
filters validate against **phases**, not task statuses (pinned by
test); the phase change commits before the rule fires, so a failing
rule never undoes the operator's move. **Open:** field-change,
date-arrival and SLE-breach triggers, and multi-action rules.

### Added — request tests for the suite (T-27)

Fourteen `#[ignore]`d request tests (`tests/requests/workflow_phase.rs`,
`tests/requests/metrics_control.rs`, `tests/requests/effort.rs`),
taking the DB-gated suite from 47 to 61 — each one a check previously
performed only by hand against a running service, now in CI, with the
assertions chosen for the *refusals*: undefined is not zero, unmeasured
is not a pass, unclassified is not a feature, a skip names what it
skipped. **It immediately found a real gap:** `tasks.flow_type` existed
and Flow Distribution read it, but nothing could set it — the field was
never added to the task-create payload (the manual pass seeded rows
with `psql`). Fixed: `flow_type` is accepted and validated on create,
with `unclassified` deliberately not accepted as an input.

### Changed — embedded matcher 0.1 → 0.2

`project-portfolio-management-matcher` 0.2.0: `Plan.phase` /
`PlanPhase` (an additive, `#[serde(default)]` wire-format field, pinned
never-scored). No matching behaviour changes; stored payloads
round-trip unchanged.

### Added — cross-plan rollup (TBA-9)

`GET /api/plans/{pid}/rollup`: flow across a plan and everything it
contains. This closes the last open TBA task.

**The combined figures are the union of the subtree's tasks, not an
average of the children's ratios.** Averaging ratios weights a
five-task plan equally with a five-hundred-task one — the same error
the item-level analysis already rejects.

**The per-plan table always ships with it, and for a portfolio it is
usually the more useful half.** A rollup mixes boards whose teams mean
different things by `in_progress` — the classification is
deployment-local by design, and nothing forces two teams to agree — so
*which child differs* is a firmer finding than the combined number.
That is what the §17 open question was really asking, and it is now
recorded as resolved.

**The walk is bounded three ways, for three different reasons.** A
visited set, because a cycle in `parent_ref` would revisit nodes and
expand exponentially rather than merely loop: the write path refuses a
cycle, but a rollup that *trusts* that is one bulk import or one direct
`UPDATE` away from hanging the service. A depth cap and a node cap, so
one enormous portfolio cannot become an unbounded response. Neither is
silent — `truncated` reports a cap firing and `revisits` reports
containment that is not a tree, because a rollup that quietly covers
half an estate reads as if it covered all of it.

The walk itself is a **pure function over an adjacency map**, so the
controller loads the containment map in one query rather than one per
level, and the cycle, depth and cap behaviour is unit-tested without a
database — including a self-parent, a diamond, and a 59-deep chain.

### Added — Monte-Carlo delivery forecasting (TBA-11)

`GET /api/plans/{pid}/forecast` answers both delivery questions at
once, because quoting one without the other is how a forecast gets
misread: *how long will these N items take* and *how many will land in
N periods*.

**It samples throughput, not cycle time — and this corrected an error
in our own spec.** §17 had claimed "the cycle-time distribution is
exactly the input a Monte-Carlo 'when will these 20 items be done'
forecast needs". It is not, and it is the standard error in the field.
Cycle time answers a question about **one item** — which is precisely
what the service level expectation already reports. A **batch**
forecast needs the **throughput** distribution: how many items the team
actually finished per period. Building it from cycle times implicitly
assumes items are worked one at a time, so summing twenty cycle times
for a team running five in parallel is roughly five times too
pessimistic. The spec entry is struck through rather than deleted, so
the correction stays visible.

Three properties are load-bearing:

1. **The conservative percentile reverses between the two questions.**
   For *how long*, higher is more conservative — 85% of simulated runs
   finished by the p85. For *how many*, it is the **15th** percentile:
   "at least this many, with 85% confidence". Quoting the p85 there
   would promise the best case while sounding careful, so the field is
   named `at_least_items` for what it means rather than for the
   percentile it came from, and both responses repeat the direction.
2. **It is deterministic.** The seed is an input, fixed unless
   supplied. A forecast that changes every time you reload it is not
   one anybody will act on — and determinism is what made the whole
   simulation testable.
3. **It refuses rather than guessing.** Below six periods of history it
   returns a reason instead of a number; an all-zero history returns
   *"the honest answer is `never`, not a number"*; and a per-trial
   ceiling turns what would otherwise be an unbounded accumulation loop
   into a reported `trials_hit_ceiling`, so a percentile that is really
   a floor says so.

Sampling is with replacement, which is what makes the output a
distribution rather than a replay of the past in its original order. A
zero seed is replaced, since xorshift from a zero state emits only
zeroes and would collapse every trial to the same answer.

### Added — time-based-analysis flow gauges (TBA-10)

A default-off Prometheus gauge family, `ppm_flow_*`: flow efficiency,
p85 cycle time, work in progress, first-pass yield and the over-cap
column count per plan, refreshed by a background loop
(`PROJECT_PORTFOLIO_MANAGEMENT_FLOW_METRICS_SECS`, unset ⇒ the loop
never starts and the family never appears).

**Periodic refresh, not scrape-time computation** — computing on scrape
turns a 15-second scrape into a 15-second full-estate query on an
endpoint that needs no token, and updating on write cannot work because
these figures change as an item sits in a column, with no write to hang
an update on.

**Two bounds, because `/metrics.prom` is on the public allow-list.**
Per-plan series are **capped** (default 50, largest board first), and
small boards are **suppressed** (default floor 5): a flow efficiency
over two tasks describes two people's week, which §12.4 refuses to
measure, and reaching it by arithmetic through an unauthenticated
endpoint is the same thing. Neither bound is silent.

**Per-column occupancy is deliberately not exported.** It would be the
most useful detail and also five series per plan — the single biggest
cardinality contributor here. The over-cap *count* carries the
alertable fact ("a column on this plan is over its limit") in one
series, and the detail is one API call away.

**The p85 gauge inherits the service level expectation's refusal.**
Below `MIN_SLE_SAMPLE` finished items the SLE is null, and the gauge
stays absent rather than re-deciding the question with a number from
noise — rendering it as `0` would turn a refusal to forecast into a
claim of instant delivery.

The label is the plan **pid**, never its name (a rename would fork the
series); labelled series are reset each pass, so a plan that drops out
loses its series rather than keeping a stale value that looks live.

### Added — time-based analysis (TBA-1 … TBA-7)

The time dimension of delivery. `tasks` carried `status_changed_at` (when
the *current* status began) and `done_at`, so the moment a task moved
twice the first interval was gone — and the one question time-based
analysis exists to ask, *of the time this took, how much was somebody
actually working on it?*, could not be answered.

Flow efficiency in knowledge work typically measures **5–15%**, the same
order as Dr. R. C. Barker's finding that value-adding time is **8–14%**
of an NHS patient journey. That figure inverts the usual improvement
instinct: if an item is worked on 6% of its life, making the work 20%
faster improves delivery by about 1%, while removing half the waiting
improves it by nearly half. Velocity, utilization and story points all
measure the 6%. Full contract:
[`spec/time-based-analysis.md`](../spec/time-based-analysis.md).

- **`task_transitions`** — the durable, append-only status-transition
  log, written by the **existing** `POST /api/plans/{pid}/tasks` and
  `PATCH …/{t_pid}` calls, **in the same transaction** as the change
  that caused it. No new recording endpoint, and no edit or delete: a
  method that asks engineers to log hours gets logged hours, not true
  ones, and an editable flow log measures whatever the editor wanted.
- **A labelled backfill** — one synthetic transition per live task, so
  an existing board is analysable immediately, flagged `backfilled` and
  surfaced in every analysis. Writing it without the flag would have
  been the same code and a lie.
- **`src/tba.rs`** — the pure analysis: interval derivation, cycle
  versus lead time, per-status and per-category splits, rework and
  rolled first-pass yield, handoffs, nearest-rank percentiles, the
  service level expectation, constraint ranking, aging WIP, and Little's
  Law. No I/O and no clock read (`as_of` is a parameter), so all 24 of
  its tests run without a database.
- **Endpoints** — `GET /api/plans/{pid}/{time-analysis,constraints,aging-wip,flow,cumulative-flow}`,
  `GET /api/plans/{pid}/tasks/{t_pid}/{transitions,time-analysis}`, and
  `GET /api/flow-classes`. All read-only, OpenAPI-documented, behind the
  blanket guard.
- **`cumulative-flow`** — the board's composition sampled daily, added
  with the front-end view. It is the one figure here that cannot be
  assembled client-side: it needs every task's whole history at once,
  and an API that shipped the log to the browser to re-derive it would
  be sending far more data to compute what the server already indexes.
  Every status band is present at every sample including at zero, so a
  stacked chart never has to decide whether a missing band means zero;
  a task does not appear before it was created, and one whose history
  predates its first recorded transition reads as `todo` rather than
  vanishing and reappearing mid-chart.

Five decisions are load-bearing and pinned by tests rather than left to
comments:

1. **Cycle time and lead time are different numbers, and both are always
   returned.** An item that sat in `todo` for three weeks and was built
   in two calendar days has a cycle time of 2 calendar days and a lead time of 23. Quoting
   the first as "our delivery time" is a tenfold flattering misreport
   and the commonest error in flow reporting, so the API returns them
   together and the response says why.
2. **The statuses partition the lead time, not the cycle time.** The
   backlog dwell is real time the requester waited and has to land
   somewhere — time that belongs to no status is time a report can
   quietly lose. Flow efficiency is still measured against cycle time,
   since the team cannot be held to how long the backlog sat.
3. **An unclassified status counts against you.** A board column nobody
   classified falls back to `unnecessary_non_value_adding`, so adding a
   column cannot silently improve the flow efficiency. The `in_review`
   argument is real and local, so the whole map is overridable via
   `PROJECT_PORTFOLIO_MANAGEMENT_FLOW_CLASSES` — applied whole or not at
   all, and echoed on every response.
4. **Throughput never travels without first-pass yield.** A team whose
   throughput rises while yield falls is not going faster; it is
   shipping work back to itself.
5. **Nothing is per-person.** No per-assignee cycle time, throughput or
   flow efficiency — a stated refusal, not an unbuilt feature. It
   measures the wrong 6%, it is confounded by what the item was and who
   else was needed, and — because collection is a by-product of moving
   the card — it supplies the one reason anybody would have to distort
   the data. Handoff counts describe the item's journey.

There is no business-hours discounting and no clock pause: a weekend in
review really was a weekend in review, and working-hours arithmetic is
the standard way to make queues disappear from a report while the
customer still waits.






### Added — cargo-fuzz harness for the request-path logic (FUZZ-2)

A `fuzz/` [`cargo-fuzz`](https://rust-fuzz.github.io/book/) crate with
three coverage-guided libFuzzer targets. Until now the harnesses covered
only the dependency-light libraries; the services had none, despite
carrying the surface that actually faces the network.

- **`validate_json`** — the real request path: arbitrary bytes →
  `serde_json` → `Plan` → `validation::problems`. Never-panic,
  deterministic, and a **bounded problem report**.
- **`validate_built`** — the validator driven directly, building the
  `Plan` from raw bytes so the fuzzer controls array cardinality and
  entry contents without first having to learn JSON. A run of NUL bytes
  becomes a run of blank entries — the exact SEC-M8 shape.
- **`merge_plans`** — the merge fold over two arbitrary payloads:
  never-panic; the survivor keeps its `name`; deterministic; and
  **absorbing**, so a retried merge cannot inflate the record.

The sub-crate declares an empty `[workspace]` table: this crate is a
workspace root, so `fuzz/` would otherwise be pulled in as a member, and
a cargo-fuzz build needs its own sanitizer flags and lockfile. Nightly
only, so it is exempt from the repo MSRV. See
[`fuzz/README.md`](./fuzz/README.md).

### Fixed — an over-long array produced one `422` problem string per entry (SEC-M8)

`validation::problems` reported an over-long array's cardinality
violation once and then still walked **every** entry, so a payload with
ten thousand blank `keywords` or `tags` came back with ten thousand problem strings — which
the controller joins into a single `422` body. A small request bought a
large response.

Every per-entry loop now walks a new `inspected()` helper, which yields
at most `MAX_ARRAY_LEN` entries. The cardinality problem already rejects
the payload, so inspecting the tail decides nothing; bounding the
**report** is the same input-bounding rule (SEC-M1) as bounding the work.
The helper is named rather than inlined at each call site so a per-entry
loop added later without it reads as different from the ones that have
it. Pinned by a test.

Case landed this first as the reference; this is the roll-out
(repo `tasks.md` SEC-M8b).

Thirteen per-entry loops across `problems` and `push_size_cap_problems`
were involved, which is precisely why the cap is a named helper rather
than thirteen inline `.take(…)` calls.

### Fixed — the search index built a new Tantivy writer on every write

`SearchEngine::index_plan` (and `delete_*` / `clear`) called
`self.index.writer(WRITER_HEAP_MB)` per call. Tantivy's `IndexWriter`
allocates its whole 50 MB arena and spawns merge threads on
construction, so **every create, update, merge, and soft-delete paid
that setup synchronously**, on the request path. Measured at ~155 ms per
indexed document against a fresh index; holding one writer for the
process brings it to ~78 ms, the remainder being the durable commit and
reader reload that indexing-on-write inherently costs.

It was also a concurrency hazard, not only a slow one: an `IndexWriter`
holds the index directory's exclusive lock, so taking and releasing it
per call left two simultaneous writes able to collide on it. One owner
for the process cannot.

The engine now holds a `Mutex<IndexWriter>` created in `new()`. A
poisoned lock recovers the guard rather than failing for ever — the only
operations held across it are `delete_term` / `add_document` / `commit`,
and a permanently dead index would be the worse outcome.

Found by the new benchmark, which is the point of having one.

### Added — Criterion benchmarks

- `benches/service_bench.rs`, covering the CPU-bound halves of a request
  — the part a database benchmark hides behind I/O. Three groups:
  **validation** (every create and update pays it; the `oversized_arrays`
  case exercises the SEC-M1 input caps, because rejecting an abusive
  payload has to be cheap or the caps are not doing their job),
  **merge** (a whole-record fold, with a scaling case showing the cost
  sits in the collections it unions), and **search** (indexing one
  document — what every write pays synchronously — plus exact / fuzzy /
  phonetic retrieval and the `candidates` blocking query a duplicate
  check actually calls, against a populated index). The search group benchmarks the `kind` filter with and without a value, because `kind` narrows **retrieval** only and never gates matching — its cost belongs here and nowhere near the matcher.
- `criterion` is a new dev-dependency; test-only, so it is not in any
  release artefact.

### Added — declared MSRV (Rust 1.95)

- `Cargo.toml` now declares `rust-version = "1.95"`, the repository's
  **current stable minus three** floor
  (`spec/rust-msrv-n-minus-3/index.md`). Sourced from `ci/msrv.txt` and
  enforced by `scripts/ci-check.sh msrv`, which asserts the declared
  value matches that file and then compiles the crate — `--all-targets`,
  so benches and tests count — against the 1.95 toolchain. Behaviour is
  unchanged; what changes is that the floor is now a checked claim
  rather than an unstated assumption.

## [0.2.0] - 2026-08-05
### Added — pagination on the five operational sub-resource lists (PG-1, 2026-08-05)

Closes the sub-bullet left open when `GET /api/plans` gained pagination
(2026-08-01): `automations`, automation runs, the deadline queue,
delegations, and one inbox were still hard-capped at `LIST_CAP = 200`.

- **`GET /api/automations`, `GET /api/automations/runs`,
  `GET /api/scheduled-actions`, `GET /api/reviews`, and
  `GET /api/notifications` now take `?limit=`/`?offset=`**, reporting
  `X-Total-Count`/`X-Limit`/`X-Offset` (`agents/share/restful.md`) —
  the identical contract `GET /api/plans` already carries. Bodies stay
  bare arrays. Defaults reproduce the old cap (`LIST_CAP` = 200 on all
  five), `limit` clamps to 500, and an `offset` past 10 000 is a `400`.
- **The deadline queue's soonest-first `due_at` order holds under
  paging** — a page is a contiguous slice of the full sorted order, not
  a reshuffled one; a dedicated test assertion pins this rather than
  just re-checking the total count.
- **`controllers::pagination`** (new `src/controllers/mod.rs` module) —
  the `Page` struct and `with_page_headers` helper, promoted out of
  `controllers/plans.rs` so all six paginated collection reads (the
  original `/api/plans` + `/api/plans/search`, plus these five) share
  one implementation. `controllers/plans.rs` now imports from the
  shared module instead of carrying its own private copy; behaviour is
  unchanged (its own `list_and_search_are_paginated` test still passes
  untouched).
- DB-gated: two new tests in `tests/requests/capabilities.rs` —
  `automation_lists_are_paginated` (covers all three automation-side
  endpoints, including the soonest-first-under-paging pin) and
  `collaboration_lists_are_paginated` (delegations + one inbox).
- Front-end: `project-portfolio-management-ui-with-svelte` wires
  the consuming routes to `ApiClient.getPage()`/`listPage()` — see that
  crate's own `CHANGELOG.md`.

### Added — Durable event bus, real-broker sink (BUS-3, following BUS-1's case-service reference, 2026-08-03)

`FluvioSink` (`src/relay.rs`) — the Phase-3 relay's real-broker
`EventSink`, behind this crate's own `fluvio` Cargo feature (off by
default; `fluvio` 0.50). One producer per topic, partitioned by record
`pid` per `agents/share/event-bus.md` §7. New env vars:
`PROJECT_PORTFOLIO_MANAGEMENT_FLUVIO_ENDPOINT` (unset ⇒ unchanged
`LoggingSink` default) and `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TOPIC`
(default `mxi.plan.events` — the `plan` streaming-entity token, per
`src/streaming.rs::ENTITY`, not the `portfolio` token `src/auth.rs`
uses for ABAC action naming). An endpoint configured without the
`fluvio` feature refuses to start the relay rather than silently
falling back to `LoggingSink` — that fallback would mark outbox rows
`published_at` without ever reaching the broker the operator asked
for. `compose.fluvio.yaml` + `Dockerfile.fluvio-cli` provision a local
SC+SPU broker (`mxi-project-portfolio-management-fluvio-*` container
names) for opt-in manual runs (not part of any automated CI stage);
`tests/fluvio_relay.rs` is a feature-gated, `#[ignore]`d live-broker
round-trip, verified by compiling under `--features fluvio` rather
than an actual execution (no broker is stood up in this repo's CI).
This crate carries no `compliance/soup.tsv`, so no SOUP register
update was needed (unlike case).

### Added — Privacy: field masking + GDPR export (2026-08-02)

Repo tasks.md P-4 (as P-1/organization; lower sensitivity). The
thinnest of the four privacy modules in the family: most of a `Plan`
(`name`, `code`, `goals`, `status`, dates, `identifiers`, `tags`,
`relationships`, `parent_ref`) is operational content, not personal
data.

- `src/privacy.rs` — `mask_plan` + `export_plan`. `lead_ref` (the plan
  lead, a `person:`/`worker:` `EntityRef` — the most directly personal
  field on a `Plan`) is **dropped entirely** rather than partially
  shown: unlike a phone number or a provider name, a partial UUID has
  no "still recognizable" value when the plan is already identified by
  its `name`/`code`. `owner_org_id`/`owner_org_name` (the sponsoring
  organization — institutional, not personal, but still worth an
  obligation-driven redaction) are masked to their tail, the same
  treatment organization gives `telephone`/`email` and care-pathway
  gives `provider_name`/`provider_id`.
- `GET /api/plans/{pid}` was previously **unguarded** — no caller
  parameter, no ABAC check of any kind, even though `auth::authorize_record`
  + `auth::plan_resource_attrs` already existed and were wired into
  `PUT /{pid}` (PPM-3, the phase-gate `stage` attribute). Now honours
  the `mask` obligation the same way the other three entity services'
  `GET /{pid}` do.
- New `GET /api/plans/{pid}/masked` (always-redacted, no policy needed)
  and `GET /api/plans/{pid}/export` (the GDPR envelope, audited via the
  plain `AuditModel::record` — this crate has no HIPAA-style disclosure
  module, unlike case/care-pathway, so it follows organization's
  posture instead).
- DB-gated: `tests/masking.rs` (new, its own test binary from the
  start — case's P-3 found out the hard way that two masking-flavoured
  tests cannot share one binary's `policy()`/`require_auth()`
  `OnceLock`s) + 2 new tests in `tests/requests/plans.rs`
  (`masked_view_and_export_are_served`,
  `masked_view_and_export_are_404_for_unknown_pid`) + 6 new DB-free
  unit tests in `src/privacy.rs`.
- No SOUP register change (no new dependency; this crate carries no
  `compliance/soup.tsv` at all).
- **P-1 through P-4 are now complete** — every entity service in the
  family that carries personal or institutional data has field masking
  wired to the ABAC `mask` obligation.

### Added — Tantivy full-text/fuzzy/phonetic search (2026-08-02)

Repo tasks.md S-4: transfers the care-pathway/case Tantivy pattern
(S-2/S-3) whole — index module, streaming seam, reindex task with a
boot rebuild, duplicate detection blocked on the index instead of
scanning a capped 1000 rows. Portfolio adds one new wrinkle: the
optional `kind` label.

- `src/search/` — `PlanIndexSchema`/`PlanIndex` (`pid` STORED;
  `name`/`alternate_names`/`name_phonetic`/`identifiers`/`keywords`/
  `tags`/`goals`/`owner_org_name` TEXT; `code`/`owner_org_id`/`kind`/
  `status`/`active` STRING) and `SearchEngine` (`search_page`,
  `candidates`). A plan's defining attribute is what it is trying to
  achieve — goal titles are now searchable, alongside tags, the owner
  org, and every identifier scheme.
- `GET /api/plans/search?q=` is now Tantivy-backed with `?fuzzy=true`,
  `?phonetic=true`, and a new `?kind=` filter; `X-Total-Count` comes
  from Tantivy's `Count` collector rather than a SQL `COUNT(*)`.
  Replaces the Postgres `ILIKE` name search.
- **`kind` is a search filter, never a dedup gate.** `kind` is indexed
  as an exact-match field so `?kind=project` narrows a *search* — but
  `check-duplicates`' blocking query (`SearchEngine::candidates`)
  deliberately never applies it. The matcher is kind-agnostic by design
  (`project-portfolio-management-matcher` AGENTS.md: "do not reintroduce
  a kind gate — two plans with different kind labels may still be the
  same identity"), and this service's own golden rule 5 says the same:
  dedup / check-duplicates / merge are not scoped by kind. Gating the
  blocking query by kind would have silently reintroduced exactly the
  per-kind collection boundary the data model unification removed.
  `search::tests::candidates_ignore_kind` and the DB-gated
  `check_duplicates_blocks_on_identifier_alone_and_ignores_kind` pin
  this: a `Program`-labelled stored plan and a `Project`-labelled query
  still block against each other.
- `POST /api/plans/check-duplicates` now scores a **blocked** candidate
  set (fuzzy name, exact identifier, phonetic name — up to 200) from the
  index instead of an in-memory scan capped at 1000 rows. (The identical
  cap still backs `governance.rs`'s separate proposal-duplicate scan,
  which has no index of its own yet.)
- Both endpoints respond `503` (never a silent "no results") when the
  index is unavailable.
- `streaming.rs`'s `*_and_emit` seam indexes/deindexes best-effort after
  every commit, so no write path can skip it.
- `tasks/search.rs` — the `search_reindex` CLI task plus a
  rebuild-if-empty on boot.
- No SOUP register change — this crate carries no `compliance/soup.tsv`
  (personal-data sensitivity here is lower than case/care-pathway; see
  spec P-4).
- `.gitignore` gains `/data/` (the index's default on-disk path) — a
  derived, rebuildable artifact that must never be committed (a lesson
  from S-2, whose own `.gitignore` fix landed alongside S-3).
- DB-gated: `search_reaches_secondary_fields_tolerates_typos_and_filters_by_kind`,
  `check_duplicates_blocks_on_identifier_alone_and_ignores_kind`.

### Changed — loco-rs 1.0.1 (2026-08-02)

- **loco-rs 0.16 → 1.0.1**: sea-orm 1.1 → 2.0, sea-orm-migration → 2.0,
  sea-query → 1.0. No raw-`Statement` or `ExprTrait` fallout in this
  crate — it has neither.
- **loco's `ColType::PkAuto` now generates a 64-bit primary key.** 16
  entities move from `i32` to `i64`: `plans`, `audit_logs`,
  `merge_records`, and the governance/visibility/strategy phase tables
  (`proposals`, `gate_reviews`, `risks`, `budget_lines`,
  `plan_dependencies`, `milestones`, `allocations`,
  `report_definitions`, `ideas`, `scenarios`, `objectives`,
  `objective_links`, `benefits`) — plus the audit code that carries a
  row id (`compliance/audit_integrity.rs`'s `mismatched: Vec<i64>`, the
  `record_integrity.rs` test fixture). The 11 tables whose migrations
  write raw SQL (`id SERIAL PRIMARY KEY`) instead of the loco schema DSL
  — `event_outbox`, `insight_snapshots`, `reviews`, `automations`,
  `automation_runs`, `scheduled_actions`, `notifications`, `sprints`,
  `tasks`, `sprint_notes`, `devops_events` — stay `i32`.
- Also fixed three pre-existing `needless_borrows_for_generic_args`
  clippy findings unrelated to the width change but surfaced by the
  same `cargo clippy` run (`controllers/mod.rs`, `controllers/oversight.rs`
  passing `&reason` where `ErrorDetail::new` now takes it by value).
- No behavioural change; verified with the full DB-gated suite (38
  tests, unchanged count) against a freshly migrated Postgres 18.

### Added — pagination on the plan collection reads (2026-08-01)

- **`GET /api/plans` and `GET /api/plans/search` take `?limit=` and
  `?offset=`**, reporting `X-Total-Count` / `X-Limit` / `X-Offset`
  (`agents/share/restful.md`). Bodies stay bare arrays. Defaults
  reproduce the old caps (100 / 50), `limit` clamps to 500, and an
  `offset` past 10 000 is a `400`.
- **The `?parent=` scope reaches the count as well as the page**, so a
  child listing's total describes that parent's children rather than
  every plan. `list_paged` and `count_for` build the same predicate for
  exactly this reason — a total counting a different set from the page
  would be worse than no total.

**Still capped, not yet paged:** the operational sub-resource lists
(`automations`, automation runs, the deadline queue, delegations,
approvals) keep their `LIST_CAP` of 200. They are per-plan working
lists rather than the entity collection, and pagination there wants the
front-end screens that consume them; recorded in tasks.md PG-1 rather
than half-done here.

### Added — key rotation and policy hot-reload without a restart (2026-08-01)

AU-2, the loco-style half of the rollout (case was the reference; the
five axum-style services landed the same day as AU-1).

- **The verifier and the ABAC policy are now reloadable holders**
  (`ReloadableVerifier` / `ReloadablePolicy`) that the blanket guard
  **and** the bearer extractors read per request. They were boot-only
  `OnceLock` snapshots, so a rotated key set or an edited policy could
  not have reached a running process at all.
- **`spawn_key_refresh`** re-fetches `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` every
  `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_REFRESH_SECS` (default 3600; `0` disables; a no-op
  when the URL is unset). A failed fetch **keeps the current key set** —
  a transient auth-service outage must not lock every caller out.
- **`spawn_policy_watcher`** polls `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE`'s mtime every
  15 s and calls `reload_policy()`; a malformed edit falls back to the
  built-in default rather than leaving the service unprotected.
- **`tests/enforcement.rs`** — the activation proof, new here and in its
  own binary: `401` without a token, `403` for a valid token the default
  policy denies a write to, `200` for `access=write`.
- New environment variable: `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_REFRESH_SECS`.

### Fixed — the DB-gated suite ran for the first time (2026-08-01)

Both failures were in the tests; the service behaved correctly in each
case.

- **The workflow-automation test indexed an object as an array.**
  `GET /api/plans/{pid}/tasks` answers `{ "tasks": [...], "counts": {...} }`,
  so `moved[0]["assignee_ref"]` was `Null` and the assertion read as "the
  automation never fired". It had: the rule logged an `applied` run and
  the row carried the assignee. Now reads `moved["tasks"][0]`.
- **The burndown test hard-coded a sprint window that drifted into the
  past.** Burndown counts `done_at` stamps falling on or before each day
  in the window; the test completes a task *now*, and once "now" passed
  the fixed `ends_on` (2026-07-26) the completion stopped counting — the
  final point read 2 remaining instead of 1. The window is now relative
  to today (−6 / +7 calendar days), keeping today inside a sprint of 14 calendar days.

  Suite: 36/36 green vs Postgres 18; crate enrolled in
  `ci/db-suites.txt` (family CI list, not in this repository).


### Added

- 2026-07-22 — **Collaboration, automation, and prioritization
  capabilities** (spec §9.4a). Migration
  `m20260722_000001_capabilities` adds `reviews`, `automations`,
  `automation_runs`, `scheduled_actions`, and `notifications`.
  - **Collaborative review** — `POST`/`GET /api/reviews`,
    `/{pid}/respond`, `/{pid}/submit`, `DELETE /{pid}`, and
    `/api/reviews/consensus`. Reviewers are `EntityRef` URNs, never raw
    emails; `reviewer_scope` records the internal/external disclosure
    decision explicitly. Only a reviewer who **accepted** may submit, so
    an unanswered invitation never becomes evidence; consensus requires
    a **strict** majority (a tie or plurality reports none) and always
    reports what is still outstanding.
  - **Assignees** — `POST /api/plans/{pid}/tasks/{t_pid}/assign`
    (`null` unassigns, notifies the assignee) and
    `GET /api/assignees/workload`, which surfaces the `unassigned` pile
    rather than dropping it.
  - **Workflow automation** — `POST`/`GET /api/automations`,
    enable/disable, delete, and `GET /api/automations/runs`. Rules fire
    from a Kanban move and from a submitted plan review. Action shapes
    are validated at write time; a failing rule is logged as a `failed`
    run and never undoes the operator's move; actions are applied
    without re-entering the engine, so automations cannot cascade.
  - **Set and forget** — `POST`/`GET /api/scheduled-actions`,
    `/sweep`, and cancel. The sweep **claims** each due row with a
    conditional update, so a deadline fires exactly once even if the
    optional ticker (`PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULER_MINUTES`,
    default off) races a manual sweep. Sweeps are capped and say so.
  - **Data-driven prioritization** — `GET /api/plans/{pid}/smart-score`
    and `/api/prioritization`. The Smart Score is a renormalized
    weighted average over seven components with a full per-component
    breakdown; absent evidence is **dropped and disclosed** (`missing`
    + `coverage`), not scored zero, and a plan with no evidence scores
    `null` / `unscored` and sorts last. ROI is computed only within a
    single currency (no FX conversion, ever). Weights are tunable via
    `PROJECT_PORTFOLIO_MANAGEMENT_SMART_SCORE_WEIGHTS` (a complete
    10 000-basis-point map; anything else is warned about and ignored).
  - **Bird's-eye visibility** — `GET /api/lifecycle` and
    `/api/plans/{pid}/lifecycle`. Every phase is reported even at zero,
    items in an unresolvable phase are counted separately, and
    readiness is a five-check checklist that names each blocker.
  - Every mutation writes an `audit_logs` row; all four rule modules
    (`collaboration`, `automation`, `prioritization`, `lifecycle`) are
    pure and DB-free with 56 unit tests, plus six `#[ignore]`d request
    tests and OpenAPI coverage.
  - **Not built:** email / push transport (notifications are in-app
    only), a `votes` Smart Score component (a plan carries no link back
    to its originating idea), and record-level ABAC on the new
    endpoints — they sit behind the blanket guard only.

- 2026-07-22 — **Five new `kind` labels: `Practice`, `Process`,
  `Purpose`, `Pathway`, `Proposal`.** `parse_kind_label` accepts the
  singular and plural spellings of each (`practice`/`practices`, …), the
  OpenAPI `Plan.kind` enum lists them, and the `kind_target` /
  `collection` validation messages on `/api/proposals`,
  `/api/ideas/{pid}/convert`, and `/api/reports` name them. Matching is
  unchanged — still kind-agnostic. (The `Proposal` **label** on a plan is
  unrelated to the `proposals` intake pipeline resource.)

### Changed — BREAKING: work items unified into one recursive `plan` collection (2026-07-20)

- **`WorkItem` → `Plan` rename.** The registered entity is now a **plan**.
  The embedded matcher exposes `Plan`, `PlanKind`, `PlanIdentifier`,
  `PlanRelationship`, `PlanStatus`, and `MatchingEngine::match_plans`
  (was `match_work_items`); `Plan::new(name)` leaves `kind` `None`. The
  service DTO, controller (`controllers/plans.rs`), and model
  (`models/plans.rs`) follow.
- **Four collections → one `/api/plans` collection.** The four REST
  collections (`/api/portfolios`, `/api/projects`, `/api/products`,
  `/api/programs`) are replaced by a single `/api/plans` collection; the
  `{collection}` path segment is gone. Plan-scoped sub-resources moved to
  `/api/plans/{pid}/...` (tasks, sprints, milestones, allocations,
  gate-reviews, risks, budget-lines, objectives, benefits, schedule,
  governance, audit).
- **`kind` is now an optional label — the kind gate is removed.** `kind`
  is `Option<PlanKind>`, an optional Portfolio / Project / Product /
  Program descriptive/grouping label; it no longer fixes a collection and
  no longer gates matching. The matcher's hard kind gate ("R-GATE") is
  gone — **any two plans may match** regardless of kind
  (`MatchBreakdown.kind_gate_blocked` remains only as a vestigial,
  always-false field).
- **General containment via `parent_ref` + cycle check.** `portfolio_ref`
  is renamed `parent_ref`: any plan may contain any other plan (a
  recursive tree). A `parent_ref` that points a plan at itself or at one
  of its descendants is now rejected `422` (new containment-cycle check).
- **Schema.** One `plans` table (was `work_items`) with a **nullable**
  `kind` column and a `parent_pid` column (was `portfolio_pid`);
  migration `..._000001_plans`.
- **Merge is unscoped.** Merge is no longer collection/kind-scoped — any
  two plans may merge; it returns `422` only on a self-merge (equal pids)
  and `404` on an unknown pid (the former cross-kind rejection is gone).
- **Proposals.** The proposal `kind_target` is now an optional descriptive
  label (blank = none), validated via the `parse_kind_label` helper.

### Added — engineering moderate fits (2026-07-20)

- Story points on tasks + `GET .../velocity` (team-local; real
  completions only); env-configured WIP limits enforced on board moves
  (`PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMITS`); sprint retro/feedback
  notes with once-only action/feedback → task conversion; DevOps event
  ingest (`/api/devops/events`) + DORA-style metrics derived only from
  ingested events (MTTR over linked pairs; declared-cause change
  failure) + the deploy-event release register.

### Added — engineering-team features (2026-07-20)

- The spec-§13 **tasks** sub-resource (Kanban statuses, PATCH board
  move with true flow stamps — `status_changed_at` per move, first
  `done_at` kept; PUT refuses status changes), **sprints**, and the
  honest **burndown** (real completions only, derivation served).
- The last-24h **standup digest** (audit-derived) and the estate
  views: blocked-work aging, the `moscow:<band>` scope cut, the
  delivery-links panel (external tracker identifiers), and the
  milestone calendar (`milestones.kind`:
  milestone/demo/release/checkpoint).
- Migration `m20260720_000001_engineering`; tasks/sprints never feed
  the matcher (the partition rule).

### Added — oversight areas: board / auditor / compliance / CRO / CISO / regulator (2026-07-20)

- Thirteen derived-view endpoints (`controllers/oversight.rs`) + the
  `insight_snapshots` table: the period board pack + investments +
  stored trend snapshots (explicit POST or env-gated ticker), the
  audit-trail explorer + segregation-of-duties findings + evidence
  pack (JSON/CSV), compliance/security risk registers + conformance
  findings, the CRO heatmap (posture, concentration, hygiene, declared
  risk appetite or an honest absence), and the deliberately coarse
  regulator extract honouring the ABAC `mask` obligation.

### Added — executive moderate fits (2026-07-19)

- **Stage-gated funding tranches**: `budget_lines.gate` + `released_at`
  (migration `m20260719_000002`); a gated line is held (actuals `422`)
  until the work item's stage reaches the gate and the new
  `POST …/budget-lines/{line_pid}/release` succeeds (fail-closed
  `gate_reached`; audited). `financials/exposure` reports per-currency
  `held_minor`.
- **Technical-debt register**: `risks.category` (validated closed set)
  + `GET /api/technology/debt` — `tech_debt` risks, exposure-sorted.
- **Delivery-flow metrics**: `milestones.done_at` stamped on complete +
  `GET /api/technology/flow` — throughput/month + median lead time in calendar days;
  pre-stamp completions counted but never timed.
- **Strategic-alignment coverage**: `GET /api/executive/alignment` —
  aligned/unaligned per collection, unaligned spend per currency,
  ranked unaligned items (largest single-currency planned; disclosed
  heuristic).
- **Scenario comparison**: `GET /api/scenarios/compare?a=&b=` — two
  live evaluations side-by-side with per-currency deltas (b−a).

### Added — executive insight areas: CEO / CFO / CTO (2026-07-19)

- Seven read-only derived views over existing tables (no new
  migrations), ETag-conditional with `as_of`:
  `/api/executive/health` (per-portfolio RAG briefing),
  `/api/executive/decisions` (gate reviews, scenario commits, decided
  proposals, merges), `/api/executive/benefits` (per-currency target vs
  realized; honest null ratios), `/api/financials/variance` (by
  collection / category / portfolio; minor units; currencies never
  merged), `/api/financials/exposure` (per-currency totals, no FX),
  `/api/technology/dependency-risk` (fan-out / cross-portfolio /
  red-predecessor edges), `/api/technology/radar`
  (`tech:<name>[:<ring>]` tag convention, majority ring vote).
- Pure derivations live in `src/insights.rs` with DB-free unit tests;
  the RAG derivation is shared with `/at-a-glance`.

### Added

- 2026-07-18 — **PPM Phase C: strategy** (T-PPM-C; PPM-2/4/5/11).
  The idea funnel (capture / vote / dismiss / convert into a draft
  proposal, `provenance=idea` — completing idea → proposal → work
  item); what-if scenarios evaluated over live budgets, open risk
  exposure, and OKR alignment (per-currency saturating sums, budget
  cap + must-include violations; **infeasible commits refused**, the
  committed evaluation audited); the OKR objective registry with
  weighted (1–5) per-pair-upserting item mappings and
  per-collection alignment rollups; benefits with minor-unit
  financial targets or non-financial notes, accumulate-realize, and
  per-currency **ROI in basis points** against recorded budget
  actuals. Pure rules in `src/strategy.rs`; 3 unit + 4 DB-gated
  request tests vs Postgres 18.

- 2026-07-18 — **PPM Phase B: visibility** (T-PPM-B; PPM-6/7/8/9).
  Cross-item finish-start dependencies (cycle-refusing) + the
  portfolio schedule view (violations, memoized critical path,
  undated members); milestones with overdue flags; resource
  allocations over `person:`/`worker:` URNs + the per-person
  capacity rollup (summed percent over a window, > 100 % flagged);
  saved report definitions run synchronously as JSON or CSV
  (RFC-4180 escaping, row cap 1000); the ETag-conditional
  `/api/at-a-glance` dashboard (per-collection RAG — documented
  heuristic over materialized risks / overdue targets / budget
  overrun / exposure / schedule violations — stage distributions,
  and site tiles). Pure rules in `src/visibility.rs`; 7 unit + 5
  DB-gated request tests vs Postgres 18.

- 2026-07-18 — **PPM Phase A: the governance core** (T-PPM-A;
  PPM-1/3/10/12 from the entity roadmap). Work-intake `proposals`
  pipeline with matcher-backed duplicate-demand detection and
  promote-to-work-item (`provenance=intake`); strictly ordered
  phase-gate reviews (g0_concept…g5_benefits) advancing an
  operational `work_items.stage`, gate-lockable via the new
  `resource.stage` record-level ABAC (`auth::authorize_record`);
  risks (1–5 × 1–5 exposure, escalation); budget lines in integer
  minor units + ISO-4217 with per-currency planned/actual/variance;
  the per-item `/governance` summary. Pure rules in
  `src/governance.rs`; every mutation audited; OpenAPI `governance`
  tag; 4 unit + 5 DB-gated request tests, verified against
  Postgres 18.


### Fixed

- 2026-07-18 — **Unknown-pid reads returned 500, not 404.** loco 0.16's
  `IntoResponse` catch-all maps an unmapped `ModelError::EntityNotFound`
  to a 500, so `GET /…/{pid}` with an unknown pid crashed instead of
  404ing (the organization service was immune — its `http_err` helper
  already mapped it; the copy-adaptors dropped it). Controller lookups
  now route through a `model_not_found` mapping. Family-wide fix with
  per-crate request-test pins.


### Changed

- 2026-07-18 — **Subproject renamed**: `portfolio` →
  `project-portfolio-management` (directory, crate/package name, lib
  ident, env-var prefix `PORTFOLIO_*` → `PROJECT_PORTFOLIO_MANAGEMENT_*`,
  database names). The **domain language is unchanged**: the work-item
  kinds (portfolio / project / product / program), the `work_items`
  table, the API routes, and the matcher's `WorkItem` type keep their
  names — the rename repositions the *subproject* as a project
  portfolio management (PPM) product; see the feature roadmap in
  `../spec/15-roadmap.md`.


### Fixed

- 2026-07-18 — **Fresh-database `db migrate` failure.** The
  `…_000004_event_outbox` migration used the loco `create_table`
  helper, which pluralizes table names (`event_outbox` →
  `event_outboxes`); its own index DDL then failed and rolled back
  the entire fresh migrate (zero tables). Rewritten as explicit SQL
  creating exactly `event_outbox`; verified against a fresh
  Postgres 18 (all migrations apply, correct table names). Family-wide
  fix (case, care-pathway, organization, portfolio; patient-flow
  shipped with the explicit-SQL form).


### Security

- **SEC-G6: trailing slash can no longer downgrade a destructive POST.**
  `derive_action` classified `/merge` / `/deduplicate` / `/import` via
  `path.ends_with`, so a trailing slash (`POST …/merge/`) fell through to
  `Write` — a non-admin `access=write` caller could reach a destructive op.
  The path is now `trim_end_matches('/')`-normalized first. Test extended.

- **SEC-B6: relay claims outbox rows with `FOR UPDATE SKIP LOCKED`.** The
  Phase-3 relay drained via a plain unlocked `SELECT … WHERE published_at IS
  NULL`, so with more than one instance every relay would **double-ship** the
  same rows. `drain_once` now runs in a transaction and `unpublished` claims
  rows with `FOR UPDATE SKIP LOCKED` (a second relay skips locked rows; the
  lock releases on commit). Delivery stays at-least-once (consumers dedupe on
  `event_id`).

### Security — SEC-M1: input-size caps on the validation entrypoint (2026-07-13)

- `src/validation.rs` now rejects oversized `WorkItem` payloads before the
  record is stored or matched, closing a CPU/memory denial-of-service
  vector: the matcher runs `O(n·m)` string similarity (Jaro-Winkler /
  Soundex) and Jaccard over the payload's text fields and arrays, so a
  single huge string or huge array is a DoS (amplified by the
  check-duplicates scan). New named caps enforced (all problems collected,
  never aborting early, surfaced as `422`): `MAX_TEXT_LEN = 1024`
  Unicode scalar values per scalar text field (`name`, `code`,
  `owner_org_id`, `owner_org_name`, `lead_ref`, `portfolio_ref`,
  `start_date`, `target_date`, `in_language`); `MAX_ARRAY_LEN = 256`
  entries per array (`alternate_names`, `goals`, `keywords`, `tags`,
  `identifiers`, `same_as`, `relationships`); `MAX_ITEM_LEN = 512` per
  string entry inside an array (each entry, plus `goals[i].title`,
  `identifiers[i].value`, `relationships[i].work_item_id`). The `kind`
  discriminator is an enum, not free text, so it is not capped. New unit
  tests cover oversized single field, oversized array, oversized array
  item, and a within-caps large-but-valid record.

### Changed — event bus: audit now joins the outbox transaction (2026-07-09)

- Under the `outbox` transport, the `audit_logs` write now rides the
  **same transaction** as the entity mutation and its `event_outbox` row
  (`agents/share/event-bus.md` §3 — the three "can never disagree"). It
  was previously a best-effort side channel written *after* the
  transaction committed, so a crash or audit failure could leave a
  committed change + event with no audit row. `AuditModel::record` is now
  generic over `ConnectionTrait`; the `create/update/delete/merge_and_emit`
  functions own the audit write (strict/in-txn under `outbox`, best-effort
  logged under `memory`), and the `work_items` controller no longer audits
  separately. New DB-gated `tests/outbox_audit.rs` drives `create_and_emit`
  under `outbox` and asserts entity + event + audit all commit together.
  (The `merge_records` history row stays a best-effort side channel — it
  is merge metadata, not the §3 audit trail.)

### Added — authz: ABAC policy authorization inside the blanket guard (2026-07-05)

- ABAC authorization landed (supersedes the earlier per-crate
  roles/RBAC sketch; family contract:
  `agents/share/authorization-attributes.md`). When
  `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` is on, a verified PASETO token is further
  checked by the shared policy engine in `authentication-verifier`
  0.3: the request's action is derived from the HTTP method plus the
  crate's destructive named POSTs (`auth::DESTRUCTIVE_POST_SUFFIXES`
  — `/merge`, `/deduplicate`, `/import`; matched on path suffix across
  all four collections), and the policy is evaluated over the token's
  new `attrs` claim, first-match-wins, defaulting to allow-read /
  deny-mutation.
- New env vars `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY` (inline JSON) and
  `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE` (path); unset or unparsable ⇒
  `tracing::warn!` + the built-in default policy (`svc=true` ⇒
  everything; `access=admin` ⇒ destructive+write; `access=write` ⇒
  write) — the service always boots.
- `auth::enforce` now takes the HTTP method and the policy and returns
  `403` (deciding-rule reason) for a valid token the policy denies;
  `401` remains missing/bad credential. `require_auth_mw` in `app.rs`
  passes the request method and `auth::policy()`. DB-free unit tests
  pin the family §7 matrix. Flag off ⇒ behaviour-neutral.

### Added

- **Boot-time paseto-keys-over-HTTP fetch** (the spec §13 follow-up, done
  2026-07-04). New optional env var `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL`: when set
  (non-blank), `auth::init` — called from `App::after_routes`, before the
  app serves traffic — fetches the auth-service's published Ed25519 key
  set once over HTTP via `Verifier::from_paseto_keys_url` (the
  `authentication-verifier` crate's `fetch` feature, now enabled). On
  success the fetched key set **wins** over the `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS`
  env key set (`tracing::info!`); on failure the service logs a
  `tracing::warn!` and falls back to the env path, so it **always
  boots**. Unset/blank ⇒ prior behaviour unchanged (env key set, else
  empty reject-all). Fetch is once-at-boot only — no refresh loop
  (rotation-triggered refetch is tracked in spec §16). The seeding is
  idempotent (`OnceLock`), and the fetch-or-fallback helper
  (`auth::fetch_or`) is dependency-injected (URL / issuer / audience /
  fallback passed in) so tests cover it without the process global: a
  `#[tokio::test]` local ephemeral-port HTTP listener proves a token
  signed by the served key verifies via the fetch-built verifier, and a
  fast-failing URL (`http://127.0.0.1:1/`) proves fallback without
  panic. Existing env-key auth tests unchanged and green.

## [0.1.0] - 2026-06-18

### Added

- **Inaugural spec scaffold (spec-only — no code yet).** Documentation
  set for the loco.rs work-item registry **and** project-management tool:
  - `spec/index.md` — the §1–§18 single-source-of-truth service spec,
    mirroring the care-pathway service shape. Defines the **four matchable
    collections** (`portfolios`, `projects`, `products`, `programs`) — one
    JSONB row table per kind, sharing one parameterized controller core
    (the API DTO **is** `project_portfolio_management_matcher::WorkItem`, persisted verbatim,
    matched with no adapter); **within-kind matching only** (the matcher's
    R-GATE makes a project never match a product); the umbrella hierarchy
    (Projects / Products / Programs carry a `portfolio_ref` to their parent
    portfolio); the operational sub-resources (goals, tasks, issues) in
    their own tables keyed by the parent `(kind, pid)` and **excluded from
    the matcher payload** (goal titles bridge via `data.goals[]`); the
    derived timeline / burndown read views; CRUD + soft-delete + audit;
    embedded probabilistic + deterministic matching (`POST /match` /
    `/check-duplicates` / `/deduplicate`); real-time create duplicate
    detection (`409`) + review queue; record merge (`Replaces` link +
    transferred snapshot + `Merged` event, same-kind only); `ILIKE` name
    search; event streaming (durable-bus Phase 1 envelope); OpenAPI/Swagger;
    per-collection Prometheus metrics; offline PASETO v4.public verification
    + blanket `/api/*` enforcement (off by default, gated by
    `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`); cross-service entity links (write side); and
    bulk import/export (deferred).
  - `README.md` — user-facing intro, route table, quick start, status.
  - `CLAUDE.md` — one-line `@AGENTS.md` include.
  - `AGENTS.md` — agent guide (what this is, API surface, MVP scope,
    golden rules incl. four-kinds-one-core, within-kind matching, and the
    matcher-partition rule, intended layout).
  - `index.md` — documentation index + worked flow.
- **Auth model is PASETO v4.public + cookie sessions (spec-only).** The
  intended auth design is **server-side cookie sessions** for the human
  session plus **offline PASETO v4.public** verification for peers
  (verified against the auth-service's published **Ed25519 key**), and a
  **BFF** for the front-end so the browser holds no token. The
  `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` flag + enforcement semantics follow the family
  contract. Source of truth:
  `agents/share/authentication-sessions.md` (shared family guidance, not in this repository)
  (RS256/JWKS not used).
- **Adopts the cross-service-linking contract.** Portfolio is a
  participating service with an `entity_links` write-side table and
  `POST`/`GET`/`DELETE /api/{collection}/{pid}/links` emitting `linked`
  / `unlinked`; a work item / goal / task / issue can link to **any** index
  entity. Cross-service links are **not** a matcher signal (separate from
  within-payload `relationships`). Contract:
  `agents/share/cross-service-linking.md`.
- **Adopts the bulk-import/export contract** (deferred §13). Async
  `bg_pg` jobs, JSONL/CSV/Parquet, the five endpoints under
  `/api/{collection}/*`; stable upsert key = a deterministic external PM
  identifier (Jira / Asana / Trello / MS Project / GitHub Project / Linear /
  URI / UUID) or owner-scoped `code` or `pid`; keyless rows → dedupe →
  review queue (within-collection). Lead / person refs are personal data →
  export audited. Contract:
  [`agents/share/bulk-import-export.md`](../spec/bulk-import-export.md).

### Notes

- No Rust / Cargo crate has been generated; every `spec.md §13` task is
  unchecked. Next step is `loco new` (stripped of the auth starter) plus
  the four work-item tables + the shared CRUD MVP.
- The canonical `WorkItem` domain model is owned by the
  [portfolio entity spec §5](../spec/index.md); this crate spec references
  it.
- Copy-adapted from the (deleted) `plan` service template; the headline
  differences are the **four distinct matchable kinds** (vs plan's single
  `plan_type` field), the within-kind match **gate** (R-GATE), and the
  dropped `posts` / `comments` / `members` sub-resources (now deferred
  roadmap).

[Unreleased]: #unreleased
[0.3.0]: #030---2026-08-26
[0.2.0]: #020---2026-08-05
[0.1.0]: #010---2026-06-18

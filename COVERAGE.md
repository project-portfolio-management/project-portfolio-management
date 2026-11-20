# Test coverage

How much of the service workspace's code the tests run, measured with
[cargo-tarpaulin](https://github.com/xd009642/tarpaulin), and where tests are most needed. The numbers come from real
runs; what was not measured is said. **No threshold is set**: this page is the baseline, and a gate (a
`fail-under` figure, per crate or overall) is for the maintainers to decide.

## What is measured, and how

| | |
|---|---|
| Tool | cargo-tarpaulin 0.37.6 (published 2026-11-05), installed with `cargo install cargo-tarpaulin --version 0.37.6 --locked` |
| Code | the service workspace (`project-portfolio-management-service-with-rust`): the service, the domain, datetime and migration crates. The matcher and the UI are not measured |
| Metric | line coverage: lines run at least once over lines that can run |
| Engine | LLVM source-based coverage (`-C instrument-coverage`), on macOS and Linux alike |
| Configuration | [`tarpaulin.toml`](project-portfolio-management-service-with-rust/tarpaulin.toml) (no database) and [`tarpaulin-db.toml`](project-portfolio-management-service-with-rust/tarpaulin-db.toml) (the database suites too) |
| Reports | `target/coverage/` in the service directory (ignored): `tarpaulin-report.html`, `lcov.info`, `tarpaulin-report.json` |
| Summary | `python3 scripts/coverage-summary.py [--least N] [--crate NAME] [--markdown]`, totals by crate from the JSON report |

```bash
cd project-portfolio-management-service-with-rust
make coverage        # unit tests and the database-free integration tests (cargo test --workspace --lib; --test main)
make coverage-db     # the same and both database suites; needs Postgres and DB_URL (use a throwaway database)
```

Each section of a configuration file is one run, and tarpaulin merges the runs into one report: the unit tests
(`--workspace --lib`), the integration binary without a database (`--test main`), and in `tarpaulin-db.toml` the
request suites (`requests:: --ignored --test-threads=1`) and every other database suite (`--ignored
--test-threads=1 --skip requests:: --skip load_test::`), exactly as CI's `api-db` job splits them.

**Why LLVM and not ptrace on Linux.** macOS has only the LLVM engine, and using it on Linux too keeps the local and CI
figures comparable. More to the point, the suites that change process-wide state run each test in a child process
of the test binary (`tests/own_process.rs`): LLVM's runtime writes a profile from every process (`%m-%p`), while
ptrace sees a child only with `--follow-exec`, and slower.

**Left out of the denominator** (each with its reason in `tarpaulin.toml`): integration-test code (`tests/`,
`domain/tests/`), the Criterion benches, the cargo-fuzz targets, `migration/build.rs`, the generated
`src/openapi_bodies.rs`, the four unit-test modules kept in files of their own (`src/auth/tests.rs`,
`src/auth/area_tests.rs`, `src/tba/tests.rs`, `src/datetime/golden.rs`) and anything under `target/`. Test functions
inside a source file are left out by tarpaulin itself. A test, `tests/coverage_config.rs`, keeps the two files'
exclusion lists identical and refuses an exclusion that names nothing.

**Not measured:** the `keycloak` and `fluvio` features (built with default features only, so their code is not
compiled); the `keycloak_container` and `fluvio_relay` suites (they need Podman or a broker); `load_test` (it needs a
running service); doctests; branch coverage (tarpaulin does not implement it).

## Baseline (measured 2026-11-18)

### Work-in-progress measurement (2026-11-20)

The latest complete database trace map (`make coverage-db`, 2026-11-20, after commit `19cb59c7`) measures
**94.38% (50,342 of 53,340 lines)**: domain 98.23%, datetime 98.20%, migration 95.32%, service 93.18%. Every runner
passed: 8 migration, 27 datetime, 865 domain (one intentional ignore), 474 service-unit, 27 database-free
integration, 242 request and 95 other database tests. Tarpaulin exited 0. At this denominator, strictly exceeding 95%
needs 332 more covered lines. The detailed 2026-11-18 baseline below remains the last per-crate summary with timings.

**Correction.** An earlier run here reported `Test failed during run` and blamed a test that runs a failing child
process. The cause was the licence-file test failing on a checkout with agent worktrees under `.claude/worktrees/`,
and a run with a failing test drops that run's coverage, so every hit from the database-free integration tests was
lost. That test now lists what git would commit (`19cb59c7`), and the run exits 0. No threshold was added.

| | |
|---|---|
| Machine | Apple M4 Max (16 cores), macOS 27.2.1, shared with other sessions' builds (one-minute load 6 to 18) |
| Toolchain | rustc 1.100.0, debug profile with coverage instrumentation |
| Code | commit `508f60bc` plus this change |
| Database | PostgreSQL 18.6, a throwaway database created for the run and dropped after |
| Time | `make coverage`: 4 minutes, the instrumented build included; `make coverage-db` straight after: 9 minutes 46 seconds (the build reused; the request suites 343 s and the other database suites 166 s of it) |
| Tests | 455 + 17 + 849 + 7 unit tests, 26 database-free integration tests, 232 request tests, 92 other database tests: all passed |

| Crate | Without a database | With the database suites |
|---|---|---|
| domain (`domain/src`, 122 files) | 94.1% (10,795 of 11,470 lines) | **96.3%** (11,044 of 11,470) |
| datetime (`datetime/src`) | 72.3% (805 of 1,114) | **73.8%** (822 of 1,114) |
| migration (`migration/src`) | 19.3% (33 of 171) | **88.3%** (151 of 171) |
| service (`src`, 409 files) | 15.5% (6,295 of 40,517) | **91.2%** (36,951 of 40,517) |
| of which `src/controllers` (143 files) | 3.8% (1,247 of 33,013) | **92.0%** (30,376 of 33,013) |
| **All** | **33.7%** (17,928 of 53,272) | **91.9%** (48,968 of 53,272) |

How to read it:

- **The domain crate is tested by its own unit tests**: 94% without a database, and the database suites add only two
  points. Its largest modules: `openapi_more` 99.8% (1,995 lines), `mspdi` 90.6% (435), `tracker_import` 95.3% (424),
  `schedule_health` 99.5% (387), `cpm` 97.4% (346), `dhcw_import` 85.8% (260), `director_pack_text` 74.1% (247),
  `leveling` 92.7% (247), `bulk` 90.9% (208), `public_roadmap` 96.6% (176), `sandbox` 94.9% (176), `schedule_risk`
  90.3% (175).
- **The service is tested through its database suites.** Without them the controllers are at 3.8%: the handlers run
  only behind a real request. That is why the CI job runs both database suites, and why a figure from `make
  coverage` alone says nothing about the controllers.
- **The datetime crate is the least covered**, at 74%: `weekday.rs` 15%, `error.rs` 40%, `schema.rs` 50%,
  `duration.rs` 62%, `instant.rs` 63%, `zone.rs` 64%. What is missed was not examined line by line; much of it
  appears to be the chrono-compatible API kept for completeness (methods no caller uses yet, such as `Weekday`'s
  `succ` and `pred`), and the crate's own unit tests cover less than the service's tests do.
- A line counts once it runs; it says nothing about whether a test checks its result. Mutation testing
  ([`BENCHMARKS.md`](BENCHMARKS.md#mutation-testing-measured-2026-11-17)) is the measure of that, and has not been
  run on this workspace yet.

## Where tests are most needed

**The ten least-covered domain modules** (with the database suites; modules of any size):

| Module | Covered | Lines | Coverage |
|---|---|---|---|
| `domain/src/director_pack_text.rs` | 183 | 247 | 74.1% |
| `domain/src/validation.rs` | 93 | 111 | 83.8% |
| `domain/src/dhcw_import.rs` | 223 | 260 | 85.8% |
| `domain/src/joint_risk.rs` | 96 | 110 | 87.3% |
| `domain/src/extract.rs` | 68 | 76 | 89.5% |
| `domain/src/schedule_risk.rs` | 158 | 175 | 90.3% |
| `domain/src/deployment_moves.rs` | 19 | 21 | 90.5% |
| `domain/src/mspdi.rs` | 394 | 435 | 90.6% |
| `domain/src/bulk.rs` | 189 | 208 | 90.9% |
| `domain/src/ccpm_estimates.rs` | 50 | 55 | 90.9% |

Without the database suites the order differs (`director_pack_text` 60%, `start_control` 71%, `ccpm_estimates` 80%,
`deployment_moves` 81%, `tz` 81%, `scheduling_rules` 81%, `effort` 82%, `validation` 82%, `approvals` 83%,
`register_completeness` 85%): those modules are exercised by the request tests more than by their own unit tests.

**Service controllers no test reaches:** none. With the database suites every one of the 143 controller files has
lines covered. Without them six have none at all (`engineering/agile.rs`, `engineering/devops.rs`,
`engineering/estate.rs`, `visibility/allocations.rs`, `visibility/dashboard.rs`, `visibility/milestones.rs`), and
the rest are reached only through their inline unit tests and route registration. The least-covered controllers with
the database suites:

| Controller | Covered | Lines | Coverage |
|---|---|---|---|
| `src/controllers/docs.rs` | 6 | 10 | 60.0% |
| `src/controllers/automation.rs` | 465 | 678 | 68.6% |
| `src/controllers/components.rs` | 87 | 118 | 73.7% |
| `src/controllers/canvas.rs` | 202 | 258 | 78.3% |
| `src/controllers/roadmap_publish.rs` | 173 | 215 | 80.5% |
| `src/controllers/leveling.rs` | 155 | 190 | 81.6% |
| `src/controllers/task_bulk.rs` | 215 | 263 | 81.7% |
| `src/controllers/collaboration.rs` | 307 | 374 | 82.1% |
| `src/controllers/ledger.rs` | 243 | 293 | 82.9% |
| `src/controllers/sinks.rs` | 182 | 219 | 83.1% |

**Outside the controllers**, the service's least-covered modules of 20 lines or more: `scheduler.rs` 24%,
`auth/policy.rs` 37%, `relay.rs` 42%, `tasks/search.rs` 44%, `email.rs` 59%, `tba/constraints.rs` 62%,
`streaming.rs` 63%, `webhooks.rs` 64%, `app.rs` 65%, `cache.rs` 66%. Two files have no line run at all:
`src/bin/main.rs` (6 lines, the binary's entry point) and `src/workers/downloader.rs` (3 lines).

None of these tests were written in this change.

## In CI

The `coverage` job in [`ci.yml`](.github/workflows/ci.yml) runs `cargo tarpaulin --locked --config
tarpaulin-db.toml` on Ubuntu against a PostgreSQL 18 service container (as `api-db` does), prints the totals by
crate and the least-covered files, writes them to the job summary, and uploads `target/coverage/` as the
`coverage-api` artifact (30 days). It fails only if tarpaulin or a test fails, never on a figure.

It runs on every push and pull request, beside the other jobs, rather than weekly: locally the full run took under
ten minutes after the build, and CI's longest job (the UI, about 30 minutes) already sets the wall-clock time, so a
coverage job in parallel adds runner time but not waiting. It is **not** part of `ci-ok`: a failing test already fails
`api` or `api-db`, so the coverage job would only add a second, slower report of the same failure, and a fault in
tarpaulin itself (not in the code) would block merging. Make it a required check, and set a threshold, when a gate is
decided. **Not verified:** the job's run time and figures on a GitHub runner (it has not run in CI yet).

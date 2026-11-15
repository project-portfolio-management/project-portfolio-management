# Benchmarks

What has been measured, how, and what has **not**. Numbers here come from real
runs; where nothing was run, this page says so.

## The matcher (measured)

Criterion benchmarks for the matching engine: `project-portfolio-management-matcher-with-rust/benches/match_pair.rs`.

| | |
|---|---|
| Date | 2026-10-10 |
| Machine | Apple M4 Max, macOS 26.6.1 |
| Toolchain | rustc 1.99.1 (the pinned toolchain then; not re-run on 1.100.0) |
| Command | `cargo bench --bench match_pair -- --warm-up-time 1 --measurement-time 3` |
| Reported | the middle value of Criterion's range (time per call) |

| Benchmark | Time |
|---|---|
| `match_pair/identical_clone` | 98 ns |
| `match_pair/fuzzy_near_match` | 4.08 µs |
| `match_pair/unrelated_pair` | 3.73 µs |
| `deterministic/shared_jira_key` (short-circuit) | 108 ns |
| `deterministic/same_owner_code` (short-circuit) | 99 ns |
| `rank/10` candidates | 50 µs |
| `rank/100` candidates | 376 µs |
| `rank/1000` candidates | 3.80 ms |
| `config_variants/default`, `strict`, `lenient` | 4.04, 4.06, 4.17 µs |
| `field_arrays/goals/10`, `/100`, `/1000` | 6.8 µs, 50 µs, 1.19 ms |
| `field_arrays/relationships/10`, `/100` | 11.6 µs, 87.6 µs |

How to read it: the deterministic short-circuit is about forty times cheaper than
a probabilistic comparison; ranking scales close to linearly with the candidate
count (about 3.8 µs per candidate); and a plan with very long lists of goals,
relationships or keywords costs more, so the size of those lists matters
(see `spec/16-open-questions.md`, OQ-8, on whether to cap them).

These are microbenchmarks of the library on one machine. They say nothing about
the service under load.

## The service's CPU-bound work (measured 2026-11-15)

Criterion benchmarks for the pure halves of a request: `project-portfolio-management-service-with-rust/benches/service_bench.rs`.
Nothing here touches PostgreSQL; the database's share of a request is what the load test below measures.

| | |
|---|---|
| Date | 2026-11-15 |
| Machine | Apple M4 Max, macOS 27.2.1, shared with other work |
| Toolchain | rustc 1.100.0, release profile |
| Code | commit `c428fa6a` plus the five new groups |
| Command | `cargo bench --bench service_bench -- --warm-up-time 1 --measurement-time 3` |
| Reported | the middle value of Criterion's range (time per call) |

| Benchmark | Time | What it says |
|---|---|---|
| `validation/valid_payload` | 100 ns | every create and update |
| `validation/several_problems` | 174 ns | |
| `validation/oversized_arrays` | 6.9 µs | refusing an abusive body (SEC-M1) stays cheap |
| `merge/typical_pair` | 2.0 µs | |
| `merge/many_keywords_half_overlapping` | 1.47 ms | the cost is in the collections it unions |
| `search/index_one_document` | 99.6 ms | the per-document commit syncs to disk; group commit (CQ-10) shares it between writers |
| `search/exact`, `fuzzy`, `phonetic`, `exact_kind_filtered` | 26 µs, 174 µs, 10 µs, 28 µs | |
| `search/duplicate_candidates/10`, `/50` | 147 µs, 155 µs | |
| `capacity_rollups/summed_percent/40`, `/400` | 23 ns, 224 ns | |
| `capacity_rollups/utilization/40`, `/400` | 536 ns, 5.1 µs | |
| `cpm/analyze/100`, `/1000`, `/4000`, `/12667` | 203 µs, 2.19 ms, 10.4 ms, 53.1 ms | close to linear; the 12,667-task figure matches the 52 ms the earlier one-off probe gave, so of the 331 ms the whole request took (below) the calculation is about a sixth |
| `auth/enforce_valid_token_read`, `_write` | 25.8 µs, 25.9 µs | the Ed25519 signature check is nearly all of it; the ABAC decision is lost in the noise |
| `auth/refuse_no_token`, `auth/public_path` | 17.5 ns, 17.7 ns | a request with no token is refused before any cryptography |
| `cursor/first_page`, `signed_next_page`, `refuse_forged_cursor` | 356 ns, 488 ns, 199 ns | checking a signed cursor costs under half a microsecond |
| `public_roadmap/slug_from_title`, `public_id` | 236 ns, 11 ns | |
| `public_roadmap/source_hash_30_detail_lines` | 1.9 µs | |
| `public_roadmap/shown_translations_full_item` | 11.2 µs | hashing each field's source to decide `needs_review` |
| `public_roadmap/tag_and_detail_checks` | 1.5 µs | 20 tags and 30 detail lines |
| `dhcw_import/parse_example_spreadsheet`, `dry_run_example_spreadsheet` | 32.7 µs, 91.2 µs | 3 roadmaps and 15 items |

One run, on a machine doing other things: read these as orders of magnitude, and compare a change against a run
made the same day on the same machine.

## The service under load, every scenario, with login off and on (measured 2026-11-15)

[`scripts/load-test.mjs`](scripts/load-test.mjs) `seed`, `run`, `ramp` and `soak`, after RG-17 added the
portfolio-wide aggregates, search, the roadmap routes and the public reader, and login.

| | |
|---|---|
| Machine | Apple M4 Max, macOS 27.2.1. **The load tool, the service and PostgreSQL all on this one machine**, the database in a Podman virtual machine, and the machine shared with other work |
| Service | release build of commit `c428fa6a` plus this change; `container-dev` profile; rate limiting off; `PUBLIC_ROADMAPS=on`; log level `warn`. OTLP export was **on** (to a collector that was not there), because the run set `PROJECT_PORTFOLIO_MANAGEMENT_OTLP_ENDPOINT`, which is not read; the guide now says so |
| Login | off for the first run; **on** for the second (`REQUIRE_AUTH=true`, the published demonstration key, one PASETO token for every request but the public reader's) |
| Database | PostgreSQL 18, a fresh database: 200 plans of 50 tasks (10,000 tasks, 9,800 dependencies), 50 Product plans in one product family with a colour, each with a roadmap of 8 items (25 of the 50 roadmaps published), and one plan of 5,000 tasks |
| Seed | 103 seconds through the API (`seed --plans 200 --tasks 50 --large 5000`) |
| Command | `run --seconds 10 --concurrency 32 --public` (and `--token …` for login on) |

| Scenario | Req/s, login off | p50 / p95 ms, off | Req/s, login on | p50 / p95 ms, on | Errors (off, on) |
|---|---|---|---|---|---|
| list plans (first page) | 1886 | 15 / 28.8 | 1712 | 18 / 24.9 | 0, 0 |
| read one plan | 4659 | 6.2 / 9.5 | 3213 | 9.8 / 12.6 | 0, 0 |
| list a plan's tasks | 1790 | 17.2 / 22.9 | 1196 | 26.3 / 32.6 | 0, 0 |
| critical path of a plan | 4126 | 7.5 / 9.8 | 2803 | 11.1 / 14.9 | 0, 0 |
| issues register | 1162 | 10.8 / 93.8 | 1985 | 15.8 / 21.6 | 0, 0 |
| milestones across plans | 2498 | 9.8 / 26.8 | 8452 | 3.6 / 5.5 | 0, 0 |
| duplicate check | 1226 | 22.7 / 44.9 | 1336 | 23.1 / 30 | 0, 0 |
| create a task (write) | 1251 | 24.4 / 33 | 906 | 34.4 / 39.9 | 0, 0 |
| read a task | 2273 | 13.7 / 17.2 | 2537 | 12.3 / 15.4 | 0, 0 |
| at a glance (aggregate) | 827 | 37.8 / 49.2 | 928 | 32 / 51.4 | 0, 0 |
| executive health (aggregate) | 902 | 33.9 / 48.4 | 1068 | 29.7 / 34.1 | 0, 0 |
| lifecycle funnel (aggregate) | 1332 | 23.6 / 29.7 | 1488 | 21.2 / 25.5 | 0, 0 |
| prioritization (aggregate) | 993 | 31.7 / 39.6 | 1049 | 30 / 34.4 | 0, 0 |
| capacity (aggregate) | 2794 | 11.1 / 13 | 1834 | 18.1 / 25.9 | 0, 0 |
| risk heatmap (aggregate) | 1852 | 16.9 / 20.1 | 1622 | 18.4 / 27.8 | 0, 0 |
| portfolio money (aggregate) | 54 | 466.1 / 825.1 | 67 | 478.2 / 611 | 0, 0 |
| full-text plan search | 1903 | 15.2 / 27.5 | 2526 | 12.1 / 16.1 | 0, 0 |
| editors' roadmap list | 2397 | 12.7 / 18.9 | 3456 | 9.1 / 11.9 | 0, 0 |
| product families | 3673 | 7.9 / 13.8 | 3734 | 6.8 / 19.2 | 0, 0 |
| public reader: list (no token) | 4263 | 6.9 / 10.4 | 3615 | 7 / 17.8 | 0, 0 |
| public reader: one roadmap (no token) | 2416 | 12.2 / 19.4 | 2101 | 12.4 / 31.5 | 0, 0 |
| list tasks of a LARGE plan (5000) | 412 | 75.7 / 103.4 | 303 | 97.2 / 172.6 | 0, 0 |
| critical path of a LARGE plan (5000) | 218 | 133.4 / 224.2 | 254 | 117.9 / 163.9 | 0, 0 |

**Login costs nothing measurable here.** The two columns differ by about as much as two runs of the same thing do
on this machine; the benchmark above puts a token check at 26 µs, against requests of milliseconds.

**One aggregate is slow:** `GET /api/portfolio-money` answers 54 to 67 requests a second with a median near half a
second, where every other aggregate answers 800 to 2,800. With `--slo-p95-ms 250` the run names it and exits 1.
It is RG-18 in [`tasks.md`](tasks.md); not investigated yet.

**Memory** rose from 158 MB to 1,466 MB over the login-off run and from 34 MB to 1,534 MB over the login-on run.
A three-minute soak afterwards (`soak --minutes 3 --concurrency 16`, about 636,000 requests, no errors, worst p95
12.9 to 14.5 ms) held at **1,107 MB every minute**, so it reads as a high-water mark left by the large answers (the
5,000-task plan's task list and critical path), not a leak. Not run for hours.

**Where throughput stops growing** (`ramp --seconds 5 --max-concurrency 256 --slo-p95-ms 250`, login off): the
connections are doubled from 1 until two steps in a row fail to beat the best throughput by 10%, p95 passes the SLO,
or errors appear; the knee is the best step before that.

| Scenario | Knee (connections) | Req/s there | p95 ms there | Why it stopped |
|---|---|---|---|---|
| list plans (first page) | 32 | 2125 | 21.5 | throughput no higher than 2125 req/s past 32 connections |
| read one plan | 32 | 4770 | 8.5 | throughput no higher than 4770 req/s past 32 connections |
| create a task (write) | 128 | 1235 | 133 | p95 267.2 ms over the 250 ms SLO at 256 connections |
| at a glance (aggregate) | 16 | 744 | 29.4 | throughput no higher than 744 req/s past 16 connections |
| portfolio money (aggregate) | - | - | - | p95 420.7 ms over the 250 ms SLO at 1 connections |
| public reader: list (no token) | 16 | 3541 | 7.8 | throughput no higher than 3541 req/s past 16 connections |

Five-second steps on a shared machine are noisy (a first version of `ramp` that stopped at the first dip put two
of these knees at 2 connections); read the knee as "about this many", and the 32-connection table above as
throughput near the knee for most scenarios. Creating tasks keeps scaling to 128 connections because each waits on
the database, not the CPU.

## The service under load (measured, with strong caveats)

A repeatable test: [`scripts/load-test.mjs`](scripts/load-test.mjs) (`seed`, `run`, `soak`; needs Node and
[`oha`](https://github.com/hatoo/oha)). Recorded 2026-10-10.

| | |
|---|---|
| Machine | Apple M4 Max, macOS 26.6.1. **The load tool, the service and PostgreSQL all ran on this one machine**, the database in a Podman virtual machine |
| Service | release build of the service at commit `07453f0` plus the work in this commit; `container-dev` profile (**login and rate limiting off**, so the numbers are the service's work, not a 401 or 429) |
| Database | PostgreSQL 18 (Alpine), default settings, empty before seeding |
| Data | 300 plans of 100 tasks (30,000 tasks, 29,700 dependencies), one write-target plan, and one **large plan of 12,667 tasks** |
| Command | `node scripts/load-test.mjs run --seconds 20 --concurrency 32` (32 connections, 20 seconds per scenario) |

Results of the last of three runs (the others differ, see below):

| Scenario | Requests | Req/s | p50 ms | p95 ms | p99 ms | Slowest ms | Errors |
|---|---|---|---|---|---|---|---|
| list plans (first page) | 28060 | 1404 | 20.2 | 40.8 | 64.3 | 125 | 0 |
| read one plan | 27514 | 1377 | 13.5 | 66.7 | 80.4 | 148.1 | 0 |
| list a plan's tasks | 11327 | 568 | 35 | 153.3 | 186.3 | 247.5 | 0 |
| critical path of a plan | 1945 | 99 | 354.8 | 471.9 | 498.1 | 534.6 | 0 |
| issues register | 103414 | 5172 | 5.1 | 12.9 | 20 | 200.2 | 1 |
| milestones across plans | 123505 | 6176 | 5.1 | 6.6 | 7.7 | 113.9 | 0 |
| duplicate check | 11887 | 596 | 52.5 | 68.1 | 77.4 | 428.7 | 0 |
| create a task (write) | 16669 | 835 | 36.6 | 48.6 | 64.6 | 109 | 0 |
| read a task | 41696 | 2086 | 15 | 18.1 | 23.7 | 47.3 | 0 |
| list tasks of a LARGE plan (12667) | 320 | 18 | 1900.2 | 3254.6 | 3579.4 | 3872.5 | 122 |
| critical path of a LARGE plan (12667) | 210 | 12 | 2512 | 5759.6 | 6680 | 7967.9 | 178 |

`Errors` in the two LARGE rows are requests that did not finish before the tool's timeout; in the others an
error is a connection failure or a non-2xx answer (one each in the issues-register row, unexplained).

**How much to trust it.** Run to run, the same scenario moved by a factor of two to four (the critical-path row
gave 408, 239 and 99 requests a second in three runs), because the machine was shared with other work and the
database was growing between runs. Read the table as an order of magnitude: **an ordinary request on a
100-task plan answers in tens of milliseconds at 32 connections; a critical-path calculation takes a few
hundred.** Do not read it as capacity.

**The soak** (`soak --minutes 6 --concurrency 16`, a mixed read workload, about 800,000 requests): 4 errors;
the worst 95th percentile was 152 ms in the first minute and 95 ms in the last; the service's resident memory
stayed between 1,033 and 1,094 MB with no upward trend over the six minutes. That memory is high because it
had just served the large plan (it was 265 MB before the run); it did not fall afterwards.

### What the test found

1. **A very large plan is a cliff.** (*The critical-path figures in this item and in the two LARGE rows above were taken on a plan whose tasks had no start date, so most of the schedule was unplaced; they are withdrawn. The task-list figures stand. See "Before and after" below for the corrected critical-path numbers.*) With 12,667 tasks in one plan, listing its tasks returned about 400 KB
   and took roughly 1.9 seconds at the median under load (3.3 s at the 95th percentile), and the critical-path
   calculation took 2.5 seconds at the median and about 5.8 at the 95th percentile. Memory rose from 265 MB to
   about 1.2 GB. A plan of 100 tasks is two orders of magnitude cheaper. The task list of a plan was not paged when this was measured;
   it now is (RG-15, `limit`/`offset` and a 2,000-task ceiling), but that change was **not re-measured**.
   The critical-path cost is still open (RG-16 in [`tasks.md`](tasks.md)).
2. **Writing 60,000 rows through the API (30,000 tasks and 29,700 dependencies, 16 at a time) took 579
   seconds**, about 100 a second, much slower than the 600 to 800 task-creates a second the write scenario
   reaches against a small plan. The slowdown was not investigated.
3. A spot check that cannot be generalised: the first soak (before the write scenario was given its own plan)
   showed a 3.6-second 95th percentile and 1.7 GB, because the write scenario had grown the plan the reads
   used to 12,667 tasks. The test now keeps them apart.

### Before and after for the large plan (RG-15, RG-16)

Measured on the same machine and set-up as above, a fresh database holding one plan of 12,667 tasks chained
by 12,666 dependencies with the first task dated (so the schedule really places every task), one connection and
four connections, ten seconds each, the old build and the new build against the same data:

| Request | Before, 1 connection (p50) | After, 1 connection (p50) | Before, 4 (p50) | After, 4 (p50) |
|---|---|---|---|---|
| task list, one page of 100 | 262 ms (the old build ignored `limit` and sent all 475 KB) | **6 ms** | 320 ms | **9 ms** |
| critical path, whole plan | 879 ms | **331 ms** | 1,184 ms | **533 ms** |

Where the remaining 330 ms of one critical-path request goes (a timing probe, since removed): loading the
network from the database about 110 ms, the scheduling calculation itself about 55 ms, building the answer about
135 ms, writing it out about 20 ms, for a **10.8 MB** answer. The calculation is not the problem: the pure
function takes 52 ms for 12,667 tasks and 148 ms for 30,000 (`cargo test --release large_plan_timing --
--ignored --nocapture`). What was avoidable was a linear search inside the per-task loop (quadratic overall),
now a map lookup. Above 5,000 tasks the full answer is now refused with a 413 that names the two ways out (`?only=critical`, `?full=true`), so a 10.8 MB answer is never an accident. `GET …/cpm?only=critical` returns only the critical and near-critical tasks, but **in this
test every task is critical (it is a single chain), so it did not shrink the answer here**; it is covered by a
request test on a network with float. The earlier "2.5 s" figure for this request was taken at 32 connections on
a plan whose tasks had no start date, so it mostly measured loading and writing an answer of unplaced tasks;
please read this table instead.

### Not measured

- A database much larger than 300 plans; any other hardware; network latency between the tiers; the front end
  (the browser); rate limiting switched on (login on was measured on 2026-11-15, above); many different users; a long soak (hours or days);
  failure behaviour when the database is slow or down.

## Creating a plan, before and after moving index writes to the blocking pool (measured 2026-10-23)

CQ-10 moved the search-index write for a create from the async runtime to `spawn_blocking` (still awaited by the request). Release builds of commit `5ed2c1e` (before) and `00e91fc` (after), one `POST /api/plans`, 32 connections, 20 seconds, `oha`, `container-dev` profile (login and rate limiting off), OTLP export off, log level `warn`, PostgreSQL 18 in a Podman VM on the same Apple M4 Max as the client. The database was empty at first and grew across runs; the same body every time.

| Build | Run | Completed | Req/s | p50 ms | p95 ms | p99 ms | Errors |
|---|---|---|---|---|---|---|---|
| before | 1 | 237 | 12 | 3074 | 4562 | 5516 | 56 × 500 |
| before | 2 | 179 | 11 | 2771 | 8323 | 9172 | 51 × 500 |
| after | 1 | 126 | 8 | 3677 | 7884 | 8331 | 0 |
| after | 2 | 99 | 7 | 4311 | 7956 | 8553 | 0 |
| after | 3 | 128 | 8 | 3902 | 8343 | 8434 | 0 |
| after | 4 | 83 | 6 | 7856 | 8653 | 9225 | 0 |

(Earlier runs with debug logging and OTLP export on were discarded: they measured the logging.)

A third build adds **group commit** (CQ-10): writers add their document under the lock, then share one commit instead of each paying for its own, and each still returns only once its own write is searchable. Same method, with a fresh database (the others had grown):

| Build | Run | Completed | Req/s | p50 ms | p95 ms | p99 ms | Errors |
|---|---|---|---|---|---|---|---|
| group commit | 1 | 1118 | 57 | 526 | 608 | 2879 | 0 |
| group commit | 2 | 1313 | 67 | 371 | 453 | 4232 | 0 |
| group commit | 3 | 950 | 49 | 555 | 615 | 652 | 0 |
| group commit | 4 | 1048 | 54 | 560 | 695 | 720 | 0 |

That is about eight times the throughput and a p50 of about half a second instead of three or more seconds. The p99 is 0.65 to 4.2 s (two runs had a slow tail I did not explain), against 8 to 9 s. The comparison is not clean: the database was fresh for this build and had grown for the other two.

What the first two builds show: **p99 did not improve** (about 8 to 9 s after; 5.5 to 9 s before), and throughput fell, but the "before" build also answered about a quarter of its requests with `500` (nothing was logged at `warn`; the cause is not known), which the "after" build did not. A single create takes about 87 ms with no concurrency, so the limit is the per-create index `commit()` (it syncs to disk), not which thread runs it. Moving it off the runtime stops a blocked runtime from failing requests; it does not make creates faster. Batching commits is the change that would, and the third build does it.

What it does not show: only four to six runs, large variance, a growing database, one machine for everything. Treat the direction as indicative, not the figures.

## Limits that are not measurements

- Do not read the matcher numbers as request latency.
- The service's stated limits are those in the specification (list caps, rate limits;
  [`spec/07-non-functional-requirements.md`](spec/07-non-functional-requirements.md)), not measured
  throughput.

## Other timings that were observed (not benchmarks)

| What | Observed |
|---|---|
| The service's full Postgres-backed request suite, 160 tests, serial | about 3 minutes |
| The browser suite, 398 tests, 8 workers | about 1.5 min |

These are wall-clock times from the development machine on 2026-10-10, useful for
planning CI, not for comparing products.

## How to add a benchmark

Add it under `benches/`, run it with `cargo bench`, and record here the date,
the machine, the command and the numbers. A benchmark without a recorded run
does not belong on this page.

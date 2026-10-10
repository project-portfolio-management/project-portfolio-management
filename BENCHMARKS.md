# Benchmarks

What has been measured, how, and what has **not**. Numbers here come from real
runs; where nothing was run, this page says so.

## The matcher (measured)

Criterion benchmarks for the matching engine: `project-portfolio-management-matcher-with-rust/benches/match_pair.rs`.

| | |
|---|---|
| Date | 2026-10-10 |
| Machine | Apple M4 Max, macOS 26.6.1 |
| Toolchain | rustc 1.96.1 |
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

## The service and the interface (not measured)

- `project-portfolio-management-api-with-rust/benches/service_bench.rs` exists
  but its results are **not published here**; it has not been run for this page.
- No load test of the HTTP service, no database-size sweep and no front-end
  performance profile have been recorded. Do not read the matcher numbers as
  request latency.
- The service's stated limits are those in the specification (list caps, rate
  limits; [`spec/07-non-functional-requirements.md`](spec/07-non-functional-requirements.md)),
  not measured throughput.

## Other timings that were observed (not benchmarks)

| What | Observed |
|---|---|
| The service's full Postgres-backed request suite, 154 tests, serial | about 135 s |
| The browser suite, 398 tests, 8 workers | about 1.5 min |

These are wall-clock times from the development machine on 2026-10-10, useful for
planning CI, not for comparing products.

## How to add a benchmark

Add it under `benches/`, run it with `cargo bench`, and record here the date,
the machine, the command and the numbers. A benchmark without a recorded run
does not belong on this page.

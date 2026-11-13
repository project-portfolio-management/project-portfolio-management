# Load, soak and chaos testing

Four scripts put the service under load. None of them runs in CI except the chaos test, weekly. The only measured
results are in [`BENCHMARKS.md`](../../BENCHMARKS.md): `load-test.mjs run` and `soak` on one machine, on
2026-10-10. **No result has been recorded from `distributed-load-test.mjs` or `long-running-test.mjs`.**

| Script | What it does | Use it for |
| --- | --- | --- |
| [`scripts/load-test.mjs`](../../scripts/load-test.mjs) | `seed`, then `run` (each fixed scenario in turn with [oha](https://github.com/hatoo/oha)), or `soak` (a mixed read workload, latency and memory per minute) | The repeatable numbers in `BENCHMARKS.md` (RG-11) |
| [`scripts/distributed-load-test.mjs`](../../scripts/distributed-load-test.mjs) | `--workers` concurrent loops in **one Node process**, picking weighted scenarios (reads, a few writes) against plans it seeds | A mixed read and write load without oha |
| [`scripts/long-running-test.mjs`](../../scripts/long-running-test.mjs) | Once per `--interval`, runs each chosen scenario one time and records status, latency and (with `--pid`) RSS | A slow sampler over hours. It does not load the service |
| [`scripts/chaos-test.sh`](../../scripts/chaos-test.sh) | Random database pauses, killed connections and service restarts under load, then checks recovery and durability | Resilience (weekly in CI, `.github/workflows/chaos.yml`) |

All of them need Node 22 or later. Point the first three at a service with **login and rate limiting off** (the
`container-dev` profile; see `compose.yaml`) and a **throwaway database**: they create plans and tasks and leave
them. Otherwise the numbers measure a 401 or a 429.

```sh
APP_ENV=container-dev cargo run -- start        # in project-portfolio-management-service-with-rust
```

## `load-test.mjs`

`oha` is required (`cargo install oha`).

```sh
node scripts/load-test.mjs seed --base http://127.0.0.1:1241 --plans 100 --tasks 40 [--large 10000]
node scripts/load-test.mjs run  --base http://127.0.0.1:1241 --seconds 20 --concurrency 32 [--pid SERVICE_PID]
node scripts/load-test.mjs soak --base http://127.0.0.1:1241 --minutes 10 --concurrency 16 [--pid SERVICE_PID]
```

- `seed` writes the plan and task identifiers to `--state` (default `load-test-state.json`), which `run` and
  `soak` read.
- `run` measures each of its scenarios for `--seconds` **in turn**, so the whole run takes about the number of
  scenarios times `--seconds`.
- `run` prints a Markdown table to standard output and writes JSON to `--out` (default `load-test-results.json`).
- `soak` writes `soak-results.json` (or `--out`).
- The scenarios are in the script (`load-test.mjs`, the list after `run`).

## `distributed-load-test.mjs`

```sh
node scripts/distributed-load-test.mjs --base http://127.0.0.1:1241 --workers 4 --duration 300 [--scenarios default|heavy|read_only]
```

- **Concurrency.** The workers are async loops in one process, so the concurrency is `--workers`.
- **Seeding.** Each worker first seeds 10 plans with 5 tasks. Seeding is not counted, and the duration starts after it.
- **What a request measures.** Latency runs from sending the request to the end of its body. The percentiles are
  nearest-rank. Every response's real status is counted.
- **Scenario weights.** These are in `DEFAULT_SCENARIOS` in the script. `update_task` reads the task and sends it
  back with only the title changed, because `PUT` replaces every field.
- **Presets.** `read_only` keeps only the `GET` scenarios. `heavy` adds 500-plan pages and more critical-path
  calls, but on the same small seeded plans; for a large plan use `load-test.mjs seed --large`.
- **Reports.** A Markdown and a JSON report go to `--output-dir` (default `distributed-load-test-results/`, which git
  ignores).
- **Not built: several machines.** `--mode coordinator` and `--mode worker` exit with a message. To load from
  several machines, run the script on each machine and add up the reports by hand.

## `long-running-test.mjs`

```sh
node scripts/long-running-test.mjs --base http://127.0.0.1:1241 --duration 3600 --interval 60 \
  [--scenarios crud,read,search,aggregate] [--pid SERVICE_PID]
```

- **Seeding.** It seeds 50 plans with 10 tasks each, which are not counted.
- **Each interval.** It runs each scenario once, prints one progress line, and sleeps until the next interval.
- **Measurements.** Every request's real status is recorded, and its latency from send to the end of the body.
  A step that needs an earlier answer (for example the task of a plan whose create failed) is skipped, so it is
  not counted.
- **Reports.** Totals, the error rate, p50/p95/p99, per-interval counts and, with `--pid`, the service's RSS. They
  go to `--output-dir` (default `long-running-test-results/`, which git ignores).
- **Not measured:** database connections and cache consistency.

## `chaos-test.sh`

```sh
scripts/chaos-test.sh --seconds 60 --workers 8
```

It starts a PostgreSQL container with Podman, migrates it, starts the service and runs
[`scripts/chaos-test.mjs`](../../scripts/chaos-test.mjs).

- **Faults injected:** a database pause of 1 to 6 seconds, all database connections killed, and the service
  killed and restarted.
- **Checks:** `/_health/live` recovers within 30 seconds; every answer is a success, a deserved client error or
  a clean 500, 502, 503 or 504; every acknowledged write reads back; and at the end health and listings work.

## Reading and publishing numbers

[`BENCHMARKS.md`](../../BENCHMARKS.md) says what a published number must state:

- the machine;
- that the client, service and database share it;
- that login and rate limiting were off;
- that there is one user.

Treat the results as a floor for comparing versions, not as a capacity plan.

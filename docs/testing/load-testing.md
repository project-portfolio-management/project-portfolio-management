# Load, soak and chaos testing

Four scripts put the service under load. None of them runs in CI except the chaos test, weekly. The only measured
results are in [`BENCHMARKS.md`](../../BENCHMARKS.md): `load-test.mjs run` and `soak` on one machine, on
2026-10-10, and `run` (login off and on), `ramp` and `soak` on 2026-11-15. **No result has been recorded from `distributed-load-test.mjs` or `long-running-test.mjs`.**

| Script | What it does | Use it for |
| --- | --- | --- |
| [`scripts/load-test.mjs`](../../scripts/load-test.mjs) | `seed`, then `run` (each fixed scenario in turn with [oha](https://github.com/hatoo/oha)), `ramp` (each scenario at doubling connections, to its knee), or `soak` (a mixed read workload, latency and memory per minute) | The repeatable numbers in `BENCHMARKS.md` (RG-11) |
| [`scripts/distributed-load-test.mjs`](../../scripts/distributed-load-test.mjs) | `--workers` concurrent loops in **one Node process**, picking weighted scenarios (reads, a few writes) against plans it seeds | A mixed read and write load without oha |
| [`scripts/long-running-test.mjs`](../../scripts/long-running-test.mjs) | Once per `--interval`, runs each chosen scenario one time and records status, latency and (with `--pid`) RSS | A slow sampler over hours. It does not load the service |
| [`scripts/chaos-test.sh`](../../scripts/chaos-test.sh) | Random database pauses, killed connections and service restarts under load, then checks recovery and durability | Resilience (weekly in CI, `.github/workflows/chaos.yml`) |

All of them need Node 22 or later. Point the first three at a service with **rate limiting off** and a **throwaway
database**: they create plans and tasks and leave them. Otherwise the numbers measure a 429. Login may be on for
`load-test.mjs` if you pass `--token` (below); otherwise turn it off too, or the numbers measure a 401.

Another project on the same machine may hold port 1241 (a container published on `127.0.0.1:1241` wins over a
service listening on every address, silently, mid-run). Start the service on a port of its own with
`start --binding 127.0.0.1 --port 1251` and pass `--base http://127.0.0.1:1251`.

```sh
APP_ENV=container-dev cargo run -- start        # in project-portfolio-management-service-with-rust
```

## `load-test.mjs`

`oha` is required (`cargo install oha`).

```sh
node scripts/load-test.mjs seed --base http://127.0.0.1:1241 --plans 100 --tasks 40 [--large 10000]
node scripts/load-test.mjs run  --base http://127.0.0.1:1241 --seconds 20 --concurrency 32 [--pid SERVICE_PID]
node scripts/load-test.mjs soak --base http://127.0.0.1:1241 --minutes 10 --concurrency 16 [--pid SERVICE_PID]
node scripts/load-test.mjs ramp --base http://127.0.0.1:1241 --seconds 5 --max-concurrency 256 [--slo-p95-ms 250]
```

Every command takes `--token TOKEN`, sent as `Authorization: Bearer TOKEN` on every request but the public reader's,
to run with login on; against the demonstration key (`compose.yaml`'s `api-demo`),
`node --input-type=module -e "import {mintDemoToken} from './scripts/demo/token.mjs'; console.log(mintDemoToken())"`
mints one. `--public` adds the two public-reader scenarios, for a service with `PUBLIC_ROADMAPS=on`. `--only
"read one plan,at a glance"` keeps the scenarios whose name contains one of the fragments.

`run --slo-p95-ms N --max-error-rate R` names every scenario over either limit and exits 1, so a run can gate a
release. `ramp` doubles the connections from 1 on each scenario until errors appear, p95 passes `--slo-p95-ms`, or two
steps in a row fail to beat the best throughput by 10%, and reports the knee: the best step before that. It writes
`ramp-results.json` (or `--out`).

- `seed` writes the plan and task identifiers to `--state` (default `load-test-state.json`), which `run` and
  `soak` read.
- `run` measures each of its scenarios for `--seconds` **in turn**, so the whole run takes about the number of
  scenarios times `--seconds`.
- `run` prints a Markdown table to standard output and writes JSON to `--out` (default `load-test-results.json`).
- `soak` writes `soak-results.json` (or `--out`).
- The scenarios are in the script (`scenarios` in `load-test.mjs`): reads of one plan and its tasks and critical
  path, the issues and milestones registers, the duplicate check, a task write, seven portfolio-wide aggregates,
  full-text search, the editors' roadmap list, product families, and with `--public` the public reader; with
  `seed --large N`, a plan of N tasks.
- `seed` also makes one product family with a colour, holding every Product plan, each with a roadmap of eight
  items, every other one published.

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

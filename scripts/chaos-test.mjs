#!/usr/bin/env node
// Chaos test for the service: random faults while a mixed workload runs, then check what must still be true.
//
//   node scripts/chaos-test.mjs --service project-portfolio-management-service-with-rust/target/debug/project-portfolio-management \
//        --database-url postgres://loco:loco@127.0.0.1:5432/project_portfolio_management_test \
//        --service-dir project-portfolio-management-service-with-rust [--runtime podman] [--db-container mxi-project-portfolio-management-test-db] \
//        [--seconds 60] [--workers 8] [--seed 1] [--port 5199] [--self-test]
//
// Normally run through `scripts/chaos-test.sh`, which starts Postgres, migrates and builds first.
//
// Faults (all seeded, so a failing seed can be replayed; timing is not deterministic, so a replay is a
// second chance to see it, not a guarantee):
//   - pause the database container for 1-6 s (queries hang; connections stay open),
//   - terminate every client connection of the database (the pool is handed dead connections),
//   - kill -9 the service and start it again (a crash in the middle of requests).
// Not covered: a Postgres crash (the test database lives in RAM, so a restart would empty it), a full disk,
// network partitions between hosts, clock jumps, Redis (the container-dev profile uses the in-memory cache),
// and anything on a multi-replica deployment.
//
// What is checked:
//   1. The service answers `/_health` again within 30 s of every fault, and exits only when this script kills it.
//   2. Every response is either a success, a client error that the request deserves, or a clean 5xx
//      (500, 502, 503, 504) with a JSON or empty body. Anything else is a violation.
//   3. Every write that was acknowledged (2xx) is still there once the faults stop: acknowledged plans can be
//      read, and every acknowledged goal is in its plan, in order. A write that failed may or may not have
//      landed; the report counts those and does not call them a violation.
//   4. After the last fault the service is healthy and a listing answers 200 in under 2 s.
// Exit code 1 on any violation; the Markdown report and a JSON copy go to stdout and `--report`.

import { spawn, spawnSync } from "node:child_process";
import { writeFileSync } from "node:fs";

const args = Object.fromEntries(
  process.argv.slice(2).reduce((acc, a, i, all) => {
    if (a.startsWith("--")) acc.push([a.slice(2), all[i + 1] && !all[i + 1].startsWith("--") ? all[i + 1] : "true"]);
    return acc;
  }, []),
);
const need = (k) => args[k] ?? (console.error(`missing --${k}`), process.exit(2));
const service = need("service");
const databaseUrl = need("database-url");
const serviceDir = args["service-dir"] ?? "."; // where `config/` is: loco reads it from the working directory
const runtime = args.runtime ?? "podman";
const dbContainer = args["db-container"] ?? "mxi-project-portfolio-management-test-db";
const seconds = Number(args.seconds ?? 60);
const workers = Number(args.workers ?? 8);
const seed = Number(args.seed ?? Date.now() % 100000);
const port = Number(args.port ?? 5199);
const base = `http://127.0.0.1:${port}`;
const tag = `chaos-${seed}-${Date.now().toString(36)}`;

// A small seeded generator (mulberry32), so a run can be repeated.
let state = seed >>> 0;
const rand = () => {
  state = (state + 0x6d2b79f5) >>> 0;
  let t = state;
  t = Math.imul(t ^ (t >>> 15), t | 1);
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
};
const between = (lo, hi) => lo + Math.floor(rand() * (hi - lo + 1));
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

const violations = [];
const violate = (msg) => {
  if (violations.length < 50) violations.push(msg);
};
const counts = { ok: 0, client_error: 0, clean_5xx: 0, network_error: 0, timeout: 0 };
const faults = [];
const slowest = [];

// ---- the service under test -------------------------------------------------------------------
let child = null;
let expectedExit = false;
function startService() {
  expectedExit = false;
  child = spawn(service, ["start"], {
    cwd: serviceDir,
    env: {
      ...process.env,
      LOCO_ENV: "container-dev",
      DATABASE_URL: databaseUrl,
      PORT: String(port),
      PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH: "false",
    },
    stdio: ["ignore", "ignore", "pipe"],
  });
  let tail = "";
  child.stderr.on("data", (d) => (tail = (tail + d).slice(-2000)));
  const mine = child;
  child.on("exit", (code, signal) => {
    if (!expectedExit && mine === child) violate(`the service exited on its own (code ${code}, signal ${signal}): ${tail.slice(-400)}`);
  });
}
async function healthy(timeoutMs) {
  const end = Date.now() + timeoutMs;
  while (Date.now() < end) {
    try {
      const r = await fetch(`${base}/_health`, { signal: AbortSignal.timeout(2000) });
      if (r.ok) return true;
    } catch {
      /* not yet */
    }
    await sleep(250);
  }
  return false;
}
const container = (...a) => spawnSync(runtime, a, { encoding: "utf8" });

// ---- the workload ------------------------------------------------------------------------------
// Each worker owns its plans, so the order of its own goals is known and nothing else touches them.
const acknowledgedPlans = new Map(); // pid -> { goals: [titles acknowledged, in order] }
let unacknowledgedCreates = 0;

async function call(method, path, body) {
  const started = Date.now();
  let res;
  try {
    res = await fetch(base + path, {
      method,
      headers: { "content-type": "application/json" },
      body: body === undefined ? undefined : JSON.stringify(body),
      signal: AbortSignal.timeout(12000),
    });
  } catch (e) {
    const timedOut = e?.name === "TimeoutError";
    counts[timedOut ? "timeout" : "network_error"]++;
    return { failed: true };
  }
  const text = await res.text().catch(() => "");
  const ms = Date.now() - started;
  if (ms > 5000 && slowest.length < 20) slowest.push({ method, path, ms, status: res.status });
  if (res.ok) {
    counts.ok++;
    try {
      return { status: res.status, json: text ? JSON.parse(text) : null };
    } catch {
      violate(`${method} ${path}: ${res.status} with a body that is not JSON`);
      return { failed: true };
    }
  }
  if (res.status >= 500) {
    if (![500, 502, 503, 504].includes(res.status)) violate(`${method} ${path}: unexpected ${res.status}`);
    if (text && !/^[\[{]/.test(text.trim())) violate(`${method} ${path}: ${res.status} with a non-JSON body: ${text.slice(0, 120)}`);
    counts.clean_5xx++;
    return { failed: true, status: res.status };
  }
  counts.client_error++;
  violate(`${method} ${path}: a valid request got ${res.status}: ${text.slice(0, 160)}`);
  return { failed: true, status: res.status };
}

let stop = false;
async function worker(id) {
  const mine = [];
  let n = 0;
  while (!stop) {
    n++;
    const roll = rand();
    if (roll < 0.25 || mine.length === 0) {
      const name = `${tag} w${id} p${n}`;
      const r = await call("POST", "/api/plans", { kind: "Project", name });
      if (r.json?.pid) {
        acknowledgedPlans.set(r.json.pid, { name, goals: [] });
        mine.push(r.json.pid);
      } else unacknowledgedCreates++;
    } else if (roll < 0.6) {
      const pid = mine[between(0, mine.length - 1)];
      const title = `${tag} goal ${id}.${n}`;
      const r = await call("POST", `/api/plans/${pid}/goals`, { title });
      if (r.json) acknowledgedPlans.get(pid).goals.push(title);
    } else if (roll < 0.8) {
      await call("GET", `/api/plans/${mine[between(0, mine.length - 1)]}/goals`);
    } else if (roll < 0.9) {
      await call("GET", "/api/plans?limit=20");
    } else {
      await call("GET", `/api/plans/${mine[between(0, mine.length - 1)]}`);
    }
    await sleep(between(5, 40));
  }
}

// ---- the faults --------------------------------------------------------------------------------
async function recoversWithin30s(label) {
  if (!(await healthy(30000))) violate(`${label}: the service was not healthy again 30 s after the fault`);
}
async function chaos() {
  const end = Date.now() + seconds * 1000;
  while (Date.now() < end) {
    await sleep(between(2000, 6000));
    if (Date.now() >= end) break;
    const kind = ["pause_database", "drop_connections", "kill_service"][between(0, 2)];
    const at = new Date().toISOString();
    if (kind === "pause_database") {
      const ms = between(1000, 6000);
      container("pause", dbContainer);
      await sleep(ms);
      container("unpause", dbContainer);
      faults.push({ kind, at, detail: `${ms} ms` });
    } else if (kind === "drop_connections") {
      container(
        "exec", dbContainer, "psql", "-U", "loco", "-d", "postgres", "-c",
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'project_portfolio_management_test' AND pid <> pg_backend_pid();",
      );
      faults.push({ kind, at, detail: "every client connection" });
    } else {
      expectedExit = true;
      child.kill("SIGKILL");
      await sleep(between(500, 2000));
      startService();
      faults.push({ kind, at, detail: "kill -9, restart" });
    }
    await recoversWithin30s(kind);
  }
}

// ---- the checks afterwards ---------------------------------------------------------------------
async function verify() {
  if (!(await healthy(30000))) {
    violate("after the last fault the service never became healthy");
    return { checked: 0, missingPlans: 0, missingGoals: 0 };
  }
  let missingPlans = 0;
  let missingGoals = 0;
  for (const [pid, plan] of acknowledgedPlans) {
    const r = await fetch(`${base}/api/plans/${pid}/goals`, { signal: AbortSignal.timeout(10000) }).catch(() => null);
    if (!r || r.status === 404) {
      missingPlans++;
      violate(`acknowledged plan ${pid} (${plan.name}) is gone`);
      continue;
    }
    const goals = (await r.json()).map((g) => g.title);
    // Every acknowledged goal is present, in the order it was acknowledged. Extra goals are writes that
    // failed to answer but landed, which is allowed.
    let at = 0;
    for (const title of plan.goals) {
      const found = goals.indexOf(title, at);
      if (found < 0) {
        missingGoals++;
        violate(`acknowledged goal "${title}" is missing from plan ${pid} (or out of order)`);
      } else at = found + 1;
    }
  }
  const t = Date.now();
  const list = await fetch(`${base}/api/plans?limit=5`, { signal: AbortSignal.timeout(10000) }).catch(() => null);
  const listMs = Date.now() - t;
  if (!list?.ok) violate("a plan listing failed after the faults stopped");
  else if (listMs > 2000) violate(`a plan listing took ${listMs} ms after the faults stopped (limit 2000)`);
  return { checked: acknowledgedPlans.size, missingPlans, missingGoals, listMs };
}

// ---- run ---------------------------------------------------------------------------------------
startService();
if (!(await healthy(60000))) {
  console.error("the service did not start");
  child.kill("SIGKILL");
  process.exit(2);
}
const pool = Array.from({ length: workers }, (_, i) => worker(i));
await chaos();
stop = true;
await Promise.all(pool);
if (args["self-test"]) {
  // Prove the checks can fail: remove one acknowledged plan behind the service's back. The run must then
  // report it, so `--self-test` passes (exit 0) only if the violation was seen.
  const [victim] = acknowledgedPlans.keys();
  container("exec", dbContainer, "psql", "-U", "loco", "-d", "project_portfolio_management_test", "-c", `DELETE FROM plans WHERE pid = '${victim}';`);
}
const result = await verify();
expectedExit = true;
child.kill("SIGTERM");

const summary = { seed, seconds, workers, counts, faults: faults.length, unacknowledgedCreates, result, violations };
const lines = [
  `# Chaos test (seed ${seed}, ${seconds} s, ${workers} workers)`,
  "",
  "| | |",
  "|---|---|",
  `| Faults | ${["pause_database", "drop_connections", "kill_service"].map((k) => `${k} ${faults.filter((f) => f.kind === k).length}`).join(", ")} |`,
  `| Requests | ok ${counts.ok}, clean 5xx ${counts.clean_5xx}, network errors ${counts.network_error}, timeouts ${counts.timeout}, unexpected client errors ${counts.client_error} |`,
  `| Acknowledged plans checked | ${result.checked} (missing ${result.missingPlans}; missing goals ${result.missingGoals}) |`,
  `| Creates that got no answer | ${unacknowledgedCreates} (may or may not have landed; not a violation) |`,
  `| Plan listing after the faults | ${result.listMs ?? "n/a"} ms |`,
  `| Violations | ${violations.length} |`,
  ...violations.map((v) => `\n- ${v}`),
  slowest.length ? `\nSlow requests (over 5 s): ${JSON.stringify(slowest.slice(0, 5))}` : "",
];
console.log(lines.join("\n"));
if (args.report) writeFileSync(args.report, JSON.stringify(summary, null, 2));
if (args["self-test"]) {
  const seen = violations.some((v) => v.includes("is gone"));
  console.log(seen ? "\nself-test: the removed plan was reported, as it should be" : "\nself-test FAILED: a removed plan was not noticed");
  process.exit(seen ? 0 : 1);
}
process.exit(violations.length ? 1 : 0);

#!/usr/bin/env node
// Repeatable load and soak test for the service (RG-11).
//
//   node scripts/load-test.mjs seed  --base http://127.0.0.1:5150 --plans 200 --tasks 50 [--large 10000]
//   node scripts/load-test.mjs run   --base http://127.0.0.1:5150 --seconds 20 --concurrency 32 [--pid SERVICE_PID]
//   node scripts/load-test.mjs soak  --base ... --minutes 10 --concurrency 16 [--pid SERVICE_PID]
//
// Needs Node 22+ and `oha` (cargo install oha). Run the service WITHOUT login and rate limiting
// (the `container-dev` profile, see compose.yaml), so the numbers are the service's work and not a
// 401 or 429; say so when you publish them. Seeding goes through the public API, so it also proves
// that bulk creation works. `run` measures each scenario for a fixed time and prints a Markdown
// table plus JSON; `soak` repeats a mixed workload for a long time and reports latency and memory
// per minute, which is how a leak or a slowdown shows.
//
// What this does not do: it runs wherever you run it (client, service and database usually share
// one machine, which flatters the network and hurts the CPU), it uses one user, and it does not
// model your data. Treat the numbers as a floor for comparison between versions, not a capacity plan.

import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, existsSync } from "node:fs";

const args = Object.fromEntries(
  process.argv.slice(3).reduce((acc, a, i, all) => {
    if (a.startsWith("--")) acc.push([a.slice(2), all[i + 1] && !all[i + 1].startsWith("--") ? all[i + 1] : "true"]);
    return acc;
  }, []),
);
const command = process.argv[2];
const base = (args.base ?? "http://127.0.0.1:5150").replace(/\/$/, "");
const stateFile = args.state ?? "load-test-state.json";

async function api(method, path, body) {
  const res = await fetch(base + path, {
    method,
    headers: { "content-type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`${method} ${path} -> ${res.status} ${text.slice(0, 200)}`);
  return text ? JSON.parse(text) : null;
}

async function pool(items, concurrency, fn) {
  let next = 0;
  const workers = Array.from({ length: concurrency }, async () => {
    while (next < items.length) {
      const i = next++;
      await fn(items[i], i);
    }
  });
  await Promise.all(workers);
}

async function seed() {
  const plans = Number(args.plans ?? 100);
  const tasks = Number(args.tasks ?? 40);
  const started = Date.now();
  const state = { base, plans: [], tasks: 0, relations: 0 };
  await pool([...Array(plans).keys()], 16, async (i) => {
    const plan = await api("POST", "/api/plans", {
      kind: ["Portfolio", "Project", "Program", "Product"][i % 4],
      name: `Load plan ${i} ${["alpha", "beta", "gamma", "delta"][i % 4]} migration`,
      code: `LT-${i}`,
    });
    const pids = [];
    for (let t = 0; t < tasks; t++) {
      const task = await api("POST", `/api/plans/${plan.pid}/tasks`, {
        title: `Task ${t} of plan ${i}`,
        duration_days: 1 + ((t * 7 + i) % 9),
        start_date: t === 0 ? "2026-11-02" : undefined,
      });
      pids.push(task.pid);
      state.tasks++;
    }
    for (let t = 1; t < pids.length; t++) {
      await api("POST", `/api/plans/${plan.pid}/tasks/${pids[t - 1]}/relations`, { kind: "precedes", to_task_pid: pids[t] });
      state.relations++;
      if (t % 5 === 0 && t >= 3) {
        await api("POST", `/api/plans/${plan.pid}/tasks/${pids[t - 3]}/relations`, { kind: "precedes", to_task_pid: pids[t] }).catch(() => {});
      }
    }
    state.plans.push({ pid: plan.pid, tasks: pids.slice(0, 3) });
  });
  // A plan the write scenario can grow without changing what the read scenarios measure.
  const writePlan = await api("POST", "/api/plans", { kind: "Project", name: "Load write target" });
  state.write_plan = writePlan.pid;
  // Optionally one deliberately huge plan, measured on its own.
  const large = Number(args.large ?? 0);
  if (large > 0) {
    const plan = await api("POST", "/api/plans", { kind: "Project", name: "Load large plan" });
    const pids = [];
    await pool([...Array(large).keys()], 16, async (t) => {
      const task = await api("POST", `/api/plans/${plan.pid}/tasks`, { title: `Large ${t}`, duration_days: 1 + (t % 9) });
      pids[t] = task.pid;
    });
    await pool([...Array(large - 1).keys()], 16, async (t) => {
      await api("POST", `/api/plans/${plan.pid}/tasks/${pids[t]}/relations`, { kind: "precedes", to_task_pid: pids[t + 1] }).catch(() => {});
    });
    state.large = { pid: plan.pid, tasks: large };
  }
  state.seed_seconds = Math.round((Date.now() - started) / 1000);
  writeFileSync(stateFile, JSON.stringify(state, null, 1));
  console.log(`seeded ${state.plans.length} plans, ${state.tasks} tasks, ${state.relations} relations in ${state.seed_seconds}s -> ${stateFile}`);
}

function loadState() {
  if (!existsSync(stateFile)) throw new Error(`no ${stateFile}: run the seed command first`);
  return JSON.parse(readFileSync(stateFile, "utf8"));
}

function scenarios(state) {
  const p = state.plans[0].pid;
  const t = state.plans[0].tasks[1];
  const w = state.write_plan ?? p;
  const large = state.large
    ? [
        { name: `list tasks of a LARGE plan (${state.large.tasks})`, method: "GET", path: `/api/plans/${state.large.pid}/tasks`, large: true },
        { name: `critical path of a LARGE plan (${state.large.tasks})`, method: "GET", path: `/api/plans/${state.large.pid}/cpm`, large: true },
      ]
    : [];
  return [
    { name: "list plans (first page)", method: "GET", path: "/api/plans?limit=50" },
    { name: "read one plan", method: "GET", path: `/api/plans/${p}` },
    { name: "list a plan's tasks", method: "GET", path: `/api/plans/${p}/tasks` },
    { name: "critical path of a plan", method: "GET", path: `/api/plans/${p}/cpm` },
    { name: "issues register", method: "GET", path: "/api/issues" },
    { name: "milestones across plans", method: "GET", path: "/api/milestones" },
    { name: "duplicate check", method: "POST", path: "/api/plans/check-duplicates", body: { kind: "Project", name: "Load plan 7 beta migration" } },
    { name: "create a task (write)", method: "POST", path: `/api/plans/${w}/tasks`, body: { title: "Created under load", duration_days: 3 } },
    { name: "read a task", method: "GET", path: `/api/plans/${p}/tasks/${t}` },
    ...large,
  ];
}

function oha(scn, seconds, concurrency) {
  const a = ["--no-tui", "--output-format", "json", "-z", `${seconds}s`, "-c", String(concurrency), "-m", scn.method];
  if (scn.body) a.push("-T", "application/json", "-d", JSON.stringify(scn.body));
  a.push(base + scn.path);
  const out = spawnSync("oha", a, { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (out.status !== 0) throw new Error(`oha failed: ${out.stderr}`);
  const j = JSON.parse(out.stdout);
  const ms = (s) => (s === null || s === undefined ? null : Math.round(s * 10000) / 10);
  const total = Object.values(j.statusCodeDistribution ?? {}).reduce((x, y) => x + y, 0);
  const ok = Object.entries(j.statusCodeDistribution ?? {})
    .filter(([c]) => c.startsWith("2"))
    .reduce((x, [, n]) => x + n, 0);
  return {
    scenario: scn.name,
    requests: total,
    rps: Math.round(j.summary.requestsPerSec),
    p50_ms: ms(j.latencyPercentiles.p50),
    p95_ms: ms(j.latencyPercentiles.p95),
    p99_ms: ms(j.latencyPercentiles.p99),
    max_ms: ms(j.summary.slowest),
    // Requests still in flight when the timer ends are cut off by the tool, not failures of the service.
    errors:
      total - ok +
      Object.entries(j.errorDistribution ?? {})
        .filter(([why]) => !/deadline/i.test(why))
        .reduce((x, [, n]) => x + n, 0),
    status: j.statusCodeDistribution,
  };
}

function rssMb(pid) {
  if (!pid) return null;
  try {
    return Math.round(Number(execFileSync("ps", ["-o", "rss=", "-p", String(pid)], { encoding: "utf8" }).trim()) / 1024);
  } catch {
    return null;
  }
}

function table(rows) {
  const head = "| Scenario | Requests | Req/s | p50 ms | p95 ms | p99 ms | Slowest ms | Errors |\n|---|---|---|---|---|---|---|---|";
  const body = rows.map((r) => `| ${r.scenario} | ${r.requests} | ${r.rps} | ${r.p50_ms} | ${r.p95_ms} | ${r.p99_ms} | ${r.max_ms} | ${r.errors} |`);
  return [head, ...body].join("\n");
}

async function run() {
  const state = loadState();
  const seconds = Number(args.seconds ?? 15);
  const concurrency = Number(args.concurrency ?? 32);
  const before = rssMb(args.pid);
  const rows = scenarios(state).map((s) => {
    console.error(`running: ${s.name}`);
    return oha(s, seconds, concurrency);
  });
  const after = rssMb(args.pid);
  const result = { date: new Date().toISOString(), base, seconds, concurrency, plans: state.plans.length, tasks: state.tasks, rss_mb_before: before, rss_mb_after: after, rows };
  writeFileSync(args.out ?? "load-test-results.json", JSON.stringify(result, null, 1));
  console.log(table(rows));
  console.log(`\nservice memory (RSS): ${before ?? "n/a"} MB before, ${after ?? "n/a"} MB after`);
}

async function soak() {
  const state = loadState();
  const minutes = Number(args.minutes ?? 10);
  const concurrency = Number(args.concurrency ?? 16);
  const mix = scenarios(state).filter((s) => !s.name.includes("(write)") && !s.large).slice(0, 8);
  const per = [];
  for (let m = 0; m < minutes; m++) {
    const rows = mix.map((s) => oha(s, Math.max(1, Math.floor(60 / mix.length)), concurrency));
    const requests = rows.reduce((x, r) => x + r.requests, 0);
    const errors = rows.reduce((x, r) => x + r.errors, 0);
    const p95 = Math.max(...rows.map((r) => r.p95_ms));
    const worst = rows.find((r) => r.p95_ms === p95)?.scenario;
    const line = { minute: m + 1, requests, errors, worst_p95_ms: p95, worst_scenario: worst, rss_mb: rssMb(args.pid) };
    per.push(line);
    console.log(JSON.stringify(line));
  }
  writeFileSync(args.out ?? "soak-results.json", JSON.stringify({ date: new Date().toISOString(), base, minutes, concurrency, per }, null, 1));
  const first = per[0], last = per[per.length - 1];
  console.log(`\nminute 1 -> ${minutes}: worst p95 ${first.worst_p95_ms} -> ${last.worst_p95_ms} ms; RSS ${first.rss_mb ?? "n/a"} -> ${last.rss_mb ?? "n/a"} MB; errors ${per.reduce((x, p) => x + p.errors, 0)}`);
}

const commands = { seed, run, soak };
if (!commands[command]) {
  console.error("usage: load-test.mjs seed|run|soak [--base URL] [...]; see the header of this file");
  process.exit(2);
}
await commands[command]();

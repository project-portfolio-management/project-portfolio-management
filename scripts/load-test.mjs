#!/usr/bin/env node
// Repeatable load and soak test for the service (RG-11).
//
//   node scripts/load-test.mjs seed  --base http://127.0.0.1:1241 --plans 200 --tasks 50 [--large 10000]
//   node scripts/load-test.mjs run   --base http://127.0.0.1:1241 --seconds 20 --concurrency 32 [--pid SERVICE_PID]
//   node scripts/load-test.mjs soak  --base ... --minutes 10 --concurrency 16 [--pid SERVICE_PID]
//   node scripts/load-test.mjs ramp  --base ... --seconds 10 --max-concurrency 256 [--only NAME] [--slo-p95-ms 250]
//
// Every command takes `--token TOKEN` (sent as `Authorization: Bearer TOKEN`) to run with login on, and `run` and
// `ramp` take `--only TEXT` (scenarios whose name contains it, comma-separated for several). `run` takes
// `--slo-p95-ms N` and `--max-error-rate R` (a share, 0.001 is one in a thousand): a scenario over either is named
// and the command exits 1, so a run can gate a release. `ramp` doubles the connections from 1 up to
// `--max-concurrency` on each scenario and reports, per step, throughput and p95, and the knee: the last step before
// throughput stops growing (two steps in a row not 10% above the best) or p95 passes the SLO or errors appear. That is where to read capacity, not the
// fixed-concurrency table.
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
const base = (args.base ?? "http://127.0.0.1:1241").replace(/\/$/, "");
const stateFile = args.state ?? "load-test-state.json";

const token = args.token && args.token !== "true" ? args.token : null;
const authHeaders = token ? { authorization: `Bearer ${token}` } : {};

async function api(method, path, body) {
  const res = await fetch(base + path, {
    method,
    headers: { "content-type": "application/json", ...authHeaders },
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
  // One product family, with a colour, holding every Product plan, so the families scenario reads real rows.
  const family = await api("POST", "/api/plans", { kind: "Portfolio", name: "Load product family" });
  await api("PUT", `/api/plans/${family.pid}/roadmap-family`, { colour: "#325083" });
  state.family = family.pid;
  await pool([...Array(plans).keys()], 16, async (i) => {
    const kind = ["Portfolio", "Project", "Program", "Product"][i % 4];
    const plan = await api("POST", "/api/plans", {
      kind,
      name: `Load plan ${i} ${["alpha", "beta", "gamma", "delta"][i % 4]} migration`,
      code: `LT-${i}`,
      ...(kind === "Product" ? { parent_ref: family.pid } : {}),
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
    // Every Product plan gets a public product roadmap of eight items, every other one published, so the roadmap
    // scenarios read real rows.
    if (kind === "Product") {
      await api("PUT", `/api/plans/${plan.pid}/roadmap`, { title: `Load roadmap ${i}`, published: i % 8 === 3 }).catch(() => {});
      for (let r = 0; r < 8; r++) {
        await api("POST", `/api/plans/${plan.pid}/roadmap/items`, {
          title: `Item ${r} of roadmap ${i}`,
          description: "What the work is, in a sentence or two.",
          tags: ["Service X", `Theme ${r % 3}`],
          placement: ["now", "next", "later", "recently_delivered"][r % 4],
        }).catch(() => {});
      }
      state.roadmaps = (state.roadmaps ?? 0) + 1;
      if (i % 8 === 3) state.public_slug ??= `load-roadmap-${i}`;
    }
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
      // The first task is anchored to a date, otherwise the schedule has nothing to place and the
      // critical-path scenario would measure an empty answer.
      const task = await api("POST", `/api/plans/${plan.pid}/tasks`, { title: `Large ${t}`, duration_days: 1 + (t % 9), start_date: t === 0 ? "2026-11-02" : undefined });
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
    // Portfolio-wide reads: each spans every plan the caller may read, so they grow with the database.
    { name: "at a glance (aggregate)", method: "GET", path: "/api/at-a-glance" },
    { name: "executive health (aggregate)", method: "GET", path: "/api/executive/health" },
    { name: "lifecycle funnel (aggregate)", method: "GET", path: "/api/lifecycle" },
    { name: "prioritization (aggregate)", method: "GET", path: "/api/prioritization" },
    { name: "capacity (aggregate)", method: "GET", path: "/api/capacity" },
    { name: "risk heatmap (aggregate)", method: "GET", path: "/api/risk/heatmap" },
    { name: "portfolio money (aggregate)", method: "GET", path: "/api/portfolio-money" },
    { name: "full-text plan search", method: "GET", path: "/api/plans/search?q=migration" },
    { name: "editors' roadmap list", method: "GET", path: "/api/public-roadmaps?limit=50" },
    { name: "product families", method: "GET", path: "/api/roadmap-families?limit=50" },
    // The public reader answers 404 unless the deployment turns it on (PUBLIC_ROADMAPS=on); with --public.
    ...(args.public
      ? [
          { name: "public reader: list (no token)", method: "GET", path: "/api/public/roadmaps", anonymous: true },
          ...(state.public_slug ? [{ name: "public reader: one roadmap (no token)", method: "GET", path: `/api/public/roadmaps/${state.public_slug}`, anonymous: true }] : []),
        ]
      : []),
    ...large,
  ];
}

/// The scenarios `--only` names (a comma-separated list of name fragments), or all of them.
function chosen(state) {
  const all = scenarios(state);
  if (!args.only || args.only === "true") return all;
  const want = args.only.split(",").map((w) => w.trim().toLowerCase()).filter(Boolean);
  const out = all.filter((s) => want.some((w) => s.name.toLowerCase().includes(w)));
  if (out.length === 0) throw new Error(`--only ${args.only} names no scenario; they are: ${all.map((s) => s.name).join("; ")}`);
  return out;
}

function oha(scn, seconds, concurrency) {
  const a = ["--no-tui", "--output-format", "json", "-z", `${seconds}s`, "-c", String(concurrency), "-m", scn.method];
  if (scn.body) a.push("-T", "application/json", "-d", JSON.stringify(scn.body));
  if (token && !scn.anonymous) a.push("-H", `Authorization: Bearer ${token}`);
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
  const rows = chosen(state).map((s) => {
    console.error(`running: ${s.name}`);
    return oha(s, seconds, concurrency);
  });
  const after = rssMb(args.pid);
  const breaches = slo(rows);
  const result = { date: new Date().toISOString(), base, seconds, concurrency, login: Boolean(token), plans: state.plans.length, tasks: state.tasks, rss_mb_before: before, rss_mb_after: after, rows, breaches };
  writeFileSync(args.out ?? "load-test-results.json", JSON.stringify(result, null, 1));
  console.log(table(rows));
  console.log(`\nservice memory (RSS): ${before ?? "n/a"} MB before, ${after ?? "n/a"} MB after`);
  report(breaches);
}

/// The rows over `--slo-p95-ms` or `--max-error-rate`, each with why.
function slo(rows) {
  const p95 = args["slo-p95-ms"] ? Number(args["slo-p95-ms"]) : null;
  const rate = args["max-error-rate"] ? Number(args["max-error-rate"]) : null;
  const out = [];
  for (const r of rows) {
    if (p95 !== null && r.p95_ms > p95) out.push({ scenario: r.scenario, why: `p95 ${r.p95_ms} ms > ${p95} ms` });
    const share = r.requests ? r.errors / r.requests : 1;
    if (rate !== null && share > rate) out.push({ scenario: r.scenario, why: `error rate ${share.toFixed(4)} > ${rate}` });
  }
  return out;
}

function report(breaches) {
  if (breaches.length === 0) return;
  console.log(`\nover the limits:\n${breaches.map((b) => `- ${b.scenario}: ${b.why}`).join("\n")}`);
  process.exitCode = 1;
}

/// Double the connections from 1 to `--max-concurrency` on each scenario; the knee is the last step before
/// throughput grows by less than 10%, p95 passes `--slo-p95-ms`, or errors appear.
async function ramp() {
  const state = loadState();
  const seconds = Number(args.seconds ?? 10);
  const max = Number(args["max-concurrency"] ?? 256);
  const p95Limit = args["slo-p95-ms"] ? Number(args["slo-p95-ms"]) : Infinity;
  const steps = [];
  for (let c = 1; c <= max; c *= 2) steps.push(c);
  const out = [];
  for (const s of chosen(state).filter((x) => !x.large)) {
    // One short step is noisy on a shared machine, so a single dip does not stop the ramp: it stops on errors, on
    // p95 over the SLO, or after two steps in a row that do not beat the best throughput so far by 10%. The knee is
    // the best step before the stop.
    const rows = [];
    let best = null;
    let flat = 0;
    let why = `no limit reached by ${max} connections`;
    for (const c of steps) {
      console.error(`ramp: ${s.name} at ${c}`);
      const r = { concurrency: c, ...oha(s, seconds, c) };
      rows.push(r);
      if (r.errors > 0) {
        why = `errors at ${c} connection${c === 1 ? "" : "s"}`;
        break;
      }
      if (r.p95_ms > p95Limit) {
        why = `p95 ${r.p95_ms} ms over the ${p95Limit} ms SLO at ${c} connection${c === 1 ? "" : "s"}`;
        break;
      }
      if (best && r.rps < best.rps * 1.1) {
        flat += 1;
        if (flat === 2) {
          why = `throughput no higher than ${best.rps} req/s past ${best.concurrency} connections`;
          break;
        }
      } else {
        flat = 0;
      }
      if (!best || r.rps > best.rps) best = r;
    }
    const knee = best;
    out.push({ scenario: s.name, knee_concurrency: knee?.concurrency ?? null, knee_rps: knee?.rps ?? null, knee_p95_ms: knee?.p95_ms ?? null, stopped: why, steps: rows.map(({ concurrency, rps, p50_ms, p95_ms, p99_ms, errors }) => ({ concurrency, rps, p50_ms, p95_ms, p99_ms, errors })) });
    console.log(JSON.stringify(out.at(-1)));
  }
  writeFileSync(args.out ?? "ramp-results.json", JSON.stringify({ date: new Date().toISOString(), base, seconds, max_concurrency: max, login: Boolean(token), scenarios: out }, null, 1));
  console.log("\n| Scenario | Knee (connections) | Req/s there | p95 ms there | Why it stopped |\n|---|---|---|---|---|");
  for (const r of out) console.log(`| ${r.scenario} | ${r.knee_concurrency ?? "-"} | ${r.knee_rps ?? "-"} | ${r.knee_p95_ms ?? "-"} | ${r.stopped} |`);
}

async function soak() {
  const state = loadState();
  const minutes = Number(args.minutes ?? 10);
  const concurrency = Number(args.concurrency ?? 16);
  const mix = chosen(state).filter((s) => !s.name.includes("(write)") && !s.large && !s.anonymous).slice(0, 8);
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

const commands = { seed, run, soak, ramp };
if (!commands[command]) {
  console.error("usage: load-test.mjs seed|run|soak|ramp [--base URL] [...]; see the header of this file");
  process.exit(2);
}
await commands[command]();

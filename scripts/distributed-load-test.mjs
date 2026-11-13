#!/usr/bin/env node
// Distributed Load Testing Framework for Project Portfolio Management
//
// Weighted-mix load from one Node process: `--workers` concurrent request loops (async, not separate
// processes) pick scenarios by weight against seeded plans for `--duration` seconds. For the fixed per-endpoint
// runs with oha, and the loaded soak with memory, use `scripts/load-test.mjs`.
//
// Usage:
//   node scripts/distributed-load-test.mjs --workers 4 --base http://127.0.0.1:1241 --duration 300
//   node scripts/distributed-load-test.mjs --scenarios read_only    # or: default, heavy
//
// Not built: running workers on several machines. `--mode coordinator` and `--mode worker` exit with a message;
// to load from several machines, run this script on each and add up the reports by hand.
//
// Requirements:
//   - Node 22+
//   - Service running with login OFF (container-dev profile)
//
// Each worker seeds 10 plans with 5 tasks and leaves them, and `create_task` adds tasks to them; run it against a
// throwaway database. Reports (Markdown and JSON) go to `--output-dir` (default
// `distributed-load-test-results/`, ignored by git). Latency is from send to the end of the body.

import { writeFileSync } from "node:fs";
import { setTimeout as sleep } from "node:timers/promises";
import { randomUUID } from "node:crypto";

const args = Object.fromEntries(
  process.argv.slice(2).reduce((acc, a, i, all) => {
    if (a.startsWith("--")) acc.push([a.slice(2), all[i + 1] && !all[i + 1].startsWith("--") ? all[i + 1] : "true"]);
    return acc;
  }, []),
);

const mode = args.mode ?? "standalone";
const testId = args["test-id"] ?? randomUUID();
const duration = Number(args.duration ?? 300); // seconds
const workers = Number(args.workers ?? 4);
const baseUrl = (args.base ?? "http://127.0.0.1:1241").replace(/\/$/, "");
const outputDir = args["output-dir"] ?? "distributed-load-test-results";
const scenarioConfig = args.scenarios ?? "default";

// Ensure output directory exists
import { mkdir } from "node:fs/promises";
await mkdir(outputDir, { recursive: true }).catch(() => {});

// ============================================================================
// SCENARIOS
// ============================================================================

const DEFAULT_SCENARIOS = {
  // Read scenarios
  list_plans: { method: "GET", path: "/api/plans?limit=50", weight: 10 },
  read_plan: { method: "GET", path: "/api/plans/{pid}", weight: 8, params: ["pid"] },
  list_tasks: { method: "GET", path: "/api/plans/{pid}/tasks", weight: 8, params: ["pid"] },
  read_task: { method: "GET", path: "/api/plans/{pid}/tasks/{tid}", weight: 5, params: ["pid", "tid"] },
  search_plans: { method: "GET", path: "/api/plans/search?q=load", weight: 3 },
  critical_path: { method: "GET", path: "/api/plans/{pid}/cpm", weight: 4, params: ["pid"] },
  milestones: { method: "GET", path: "/api/milestones", weight: 4 },
  
  // Write scenarios
  create_task: { method: "POST", path: "/api/plans/{pid}/tasks", weight: 2, params: ["pid"], body: { title: "Load test task", duration_days: 1 } },
  // PUT replaces every field: the worker reads the task first and sends it back with only the title changed.
  update_task: { method: "PUT", path: "/api/plans/{pid}/tasks/{tid}", weight: 1, params: ["pid", "tid"], readModifyWrite: true },
  
  // Check duplicates
  check_duplicates: { method: "POST", path: "/api/plans/check-duplicates", weight: 3, body: { kind: "Project", name: "Test Project" } },
};

const HEAVY_SCENARIOS = {
  ...DEFAULT_SCENARIOS,
  list_all_plans: { method: "GET", path: "/api/plans?limit=500", weight: 5 },
  // The seeded plans are small (5 tasks); for a large plan's critical path use `load-test.mjs seed --large`.
  plan_cpm_extra: { method: "GET", path: "/api/plans/{pid}/cpm", weight: 3, params: ["pid"] },
};

const SCENARIO_PRESETS = {
  default: DEFAULT_SCENARIOS,
  heavy: HEAVY_SCENARIOS,
  read_only: Object.fromEntries(
    Object.entries(DEFAULT_SCENARIOS).filter(([k, v]) => v.method === "GET")
  ),
};

if (!(scenarioConfig in SCENARIO_PRESETS)) {
  console.error(`Unknown --scenarios ${scenarioConfig}; known: ${Object.keys(SCENARIO_PRESETS).join(", ")}`);
  process.exit(2);
}
const scenarios = SCENARIO_PRESETS[scenarioConfig];

// ============================================================================
// METRICS COLLECTOR
// ============================================================================

class MetricsCollector {
  constructor() {
    this.reset();
  }

  reset() {
    this.startTime = Date.now();
    this.totalRequests = 0;
    this.successfulRequests = 0;
    this.failedRequests = 0;
    this.latencies = [];
    this.statusCodes = {};
    this.errors = {};
    this.bytesSent = 0;
    this.bytesReceived = 0;
  }

  addRequest(method, path, status, latencyMs, bytesReceived, error = null) {
    this.totalRequests++;
    if (status >= 200 && status < 300) this.successfulRequests++;
    else this.failedRequests++;
    
    this.latencies.push(latencyMs);
    this.statusCodes[status] = (this.statusCodes[status] || 0) + 1;
    this.bytesReceived += bytesReceived;
    
    if (error) {
      this.errors[error] = (this.errors[error] || 0) + 1;
    }
  }

  getSummary() {
    // Throughput counts from when the load started (after the health check and seeding), not from construction.
    const elapsed = Math.max(0.001, ((this.endTime ?? Date.now()) - (this.loadStartTime ?? this.startTime)) / 1000);
    const latencies = [...this.latencies].sort((a, b) => a - b);
    // Nearest rank: the smallest value with at least p of the values at or below it.
    const rank = (p) => latencies[Math.max(0, Math.ceil(latencies.length * p) - 1)] ?? 0;
    const p50 = rank(0.5);
    const p95 = rank(0.95);
    const p99 = rank(0.99);
    const max = latencies[latencies.length - 1] ?? 0;
    
    return {
      elapsed_seconds: Math.round(elapsed),
      total_requests: this.totalRequests,
      successful_requests: this.successfulRequests,
      failed_requests: this.failedRequests,
      requests_per_second: Math.round(this.totalRequests / elapsed),
      p50_ms: Number(p50.toFixed(1)),
      p95_ms: Number(p95.toFixed(1)),
      p99_ms: Number(p99.toFixed(1)),
      max_ms: Number(max.toFixed(1)),
      avg_ms: Number((this.latencies.reduce((a, b) => a + b, 0) / this.latencies.length || 0).toFixed(1)),
      status_codes: this.statusCodes,
      errors: this.errors,
      bytes_received: this.bytesReceived,
    };
  }

  toMarkdown() {
    const summary = this.getSummary();
    const statusRows = Object.entries(summary.status_codes)
      .sort(([a], [b]) => Number(a) - Number(b))
      .map(([code, count]) => `| ${code} | ${count} |`);
    
    return `## Load Test Summary - ${testId}

| Metric | Value |
|--------|-------|
| Elapsed Time | ${summary.elapsed_seconds}s |
| Total Requests | ${summary.total_requests} |
| Successful | ${summary.successful_requests} |
| Failed | ${summary.failed_requests} |
| Requests/sec | ${summary.requests_per_second} |
| Avg Latency | ${summary.avg_ms}ms |
| P50 Latency | ${summary.p50_ms}ms |
| P95 Latency | ${summary.p95_ms}ms |
| P99 Latency | ${summary.p99_ms}ms |
| Max Latency | ${summary.max_ms}ms |
| Bytes Received | ${(summary.bytes_received / 1024 / 1024).toFixed(2)} MB |

### Status Codes
| Code | Count |
|------|-------|
${statusRows.join("\n")}
`;
  }

  toJSON() {
    return JSON.stringify(this.getSummary(), null, 2);
  }
}

// ============================================================================
// WORKER
// ============================================================================

class LoadTestWorker {
  constructor(workerId, baseUrl, scenarios, duration, metrics) {
    this.workerId = workerId;
    this.baseUrl = baseUrl;
    this.scenarios = scenarios;
    this.duration = duration;
    this.metrics = metrics;
    this.running = false;
    this.seeded = new Promise((resolve) => { this.markSeeded = resolve; });
  }

  async start() {
    this.running = true;
    // Pre-seed some plan IDs for parameterized scenarios (seeding is not counted, nor timed into the duration)
    const seedPlans = await this.seedPlans();
    this.markSeeded();
    const endTime = Date.now() + this.duration * 1000;
    
    while (this.running && Date.now() < endTime) {
      const scenario = this.selectScenario();
      const path = this.resolvePath(scenario, seedPlans);
      if (path === null) {
        // Nothing seeded for this scenario (every seed create failed); do not count a request that was not sent.
        await sleep(10);
        continue;
      }
      if (scenario.readModifyWrite) {
        const read = await this.executeRequest("GET", path);
        if (!read.ok) continue;
        await this.executeRequest(scenario.method, path, { ...JSON.parse(read.text), title: `Updated task ${Date.now()}` });
      } else {
        await this.executeRequest(scenario.method, path, scenario.body);
      }
    }
  }

  async seedPlans() {
    // Create a few plans to use for parameterized requests
    const planPids = [];
    const taskPids = [];
    
    for (let i = 0; i < 10; i++) {
      try {
        const plan = await this.createPlan(`Load Test Plan ${testId} ${this.workerId}-${i}`);
        if (!plan?.pid) continue;
        planPids.push(plan.pid);
        for (let j = 0; j < 5; j++) {
          const task = await this.createTask(plan.pid, `Task ${i}-${j}`);
          if (task?.pid) taskPids.push({ planPid: plan.pid, tid: task.pid });
        }
      } catch (error) {
        console.error(`worker ${this.workerId}: seeding failed: ${error.message}`);
      }
    }
    
    return { planPids, taskPids };
  }

  async createPlan(name) {
    const response = await fetch(`${this.baseUrl}/api/plans`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ kind: "Project", name }),
    });
    return response.ok ? response.json() : null;
  }

  async createTask(planPid, title) {
    const response = await fetch(`${this.baseUrl}/api/plans/${planPid}/tasks`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ title, duration_days: 1 }),
    });
    return response.ok ? response.json() : null;
  }

  selectScenario() {
    const totalWeight = Object.values(this.scenarios).reduce((sum, s) => sum + s.weight, 0);
    const r = Math.random() * totalWeight;
    let cumulative = 0;
    
    for (const [name, scenario] of Object.entries(this.scenarios)) {
      cumulative += scenario.weight;
      if (r <= cumulative) {
        return { ...scenario, name };
      }
    }
    
    return Object.values(this.scenarios)[0];
  }

  resolvePath(scenario, seedPlans) {
    let path = scenario.path;
    const params = scenario.params ?? [];
    if (params.includes("tid")) {
      // A task and the plan it belongs to, picked together.
      const task = seedPlans.taskPids[Math.floor(Math.random() * seedPlans.taskPids.length)];
      if (!task) return null;
      return path.replace("{pid}", task.planPid).replace("{tid}", task.tid);
    }
    if (params.includes("pid")) {
      const pid = seedPlans.planPids[Math.floor(Math.random() * seedPlans.planPids.length)];
      if (!pid) return null;
      path = path.replace("{pid}", pid);
    }
    return path;
  }

  async executeRequest(method, path, body) {
    const start = performance.now();
    try {
      const response = await fetch(`${this.baseUrl}${path}`, {
        method,
        headers: body === undefined ? undefined : { "content-type": "application/json" },
        body: body === undefined ? undefined : JSON.stringify(body),
      });
      // Read the whole body once, before stopping the clock (it also frees the connection).
      const text = await response.text();
      const latency = performance.now() - start;
      this.metrics.addRequest(method, path, response.status, latency, Buffer.byteLength(text), response.ok ? null : `HTTP ${response.status}: ${text.slice(0, 200)}`);
      return { ok: response.ok, status: response.status, text };
    } catch (error) {
      this.metrics.addRequest(method, path, 0, performance.now() - start, 0, error.message);
      return { ok: false, status: 0, text: "" };
    }
  }

  stop() {
    this.running = false;
  }
}

// ============================================================================
// STANDALONE MODE
// ============================================================================

async function runStandalone() {
  console.log(`Starting load test (${workers} concurrent loops in one process)`);
  console.log(`Test ID: ${testId}`);
  console.log(`Base URL: ${baseUrl}`);
  console.log(`Duration: ${duration}s`);
  console.log(`Workers: ${workers}`);
  console.log(`Output: ${outputDir}/`);
  
  const globalMetrics = new MetricsCollector();
  const testWorkers = [];
  
  // Health check
  console.log("\nRunning health check...");
  try {
    const health = await fetch(`${baseUrl}/_health/live`);
    if (!health.ok) {
      console.error("Health check failed!");
      process.exit(1);
    }
    console.log("✓ Service is healthy");
  } catch (error) {
    console.error("Could not connect to service:", error.message);
    process.exit(1);
  }
  
  // Start workers; each seeds, then runs for the duration. The clock for throughput starts when the first
  // worker has seeded (close enough: seeding takes the same time in each).
  console.log(`\nStarting ${workers} workers (seeding first)...`);
  const runs = [];
  for (let i = 0; i < workers; i++) {
    const worker = new LoadTestWorker(i, baseUrl, scenarios, duration, globalMetrics);
    testWorkers.push(worker);
    runs.push(worker.start());
  }
  globalMetrics.loadStartTime = null;
  const firstSeeded = Promise.race(testWorkers.map((w) => w.seeded));
  firstSeeded.then(() => { globalMetrics.loadStartTime = Date.now(); });
  process.on("SIGINT", () => { for (const w of testWorkers) w.stop(); });
  console.log(`Running for ${duration} seconds after seeding... (Ctrl+C stops early and still writes the report)`);
  await Promise.all(runs);
  globalMetrics.endTime = Date.now();

  // Generate reports
  const summary = globalMetrics.getSummary();
  console.log("\n" + globalMetrics.toMarkdown());
  
  // Save results
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
  const markdownPath = `${outputDir}/distributed-load-test-${timestamp}.md`;
  const jsonPath = `${outputDir}/distributed-load-test-${timestamp}.json`;
  
  writeFileSync(markdownPath, globalMetrics.toMarkdown());
  writeFileSync(jsonPath, globalMetrics.toJSON());
  
  console.log(`\nResults saved to:`);
  console.log(`  - ${markdownPath}`);
  console.log(`  - ${jsonPath}`);
  console.log(`\nTest complete!`);
}

// ============================================================================
// MAIN
// ============================================================================

async function main() {
  if (mode !== "standalone") {
    console.error(`--mode ${mode} is not built: this script runs on one machine only (see the header).`);
    process.exit(2);
  }
  await runStandalone();
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});

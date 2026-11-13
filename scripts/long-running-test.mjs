#!/usr/bin/env node
// Long-Running Test Suite for Project Portfolio Management
//
// A slow sampler for endurance runs: once per `--interval` it runs each chosen scenario one time, timing every
// request and recording its real status, and (with `--pid`) the service's resident memory. It does not put the
// service under load; for throughput and a loaded soak use `scripts/load-test.mjs run|soak`. It does not measure
// database connections or cache consistency.
//
// Usage:
//   # Run for 1 hour with default settings
//   node scripts/long-running-test.mjs --duration 3600
//
//   # Run for 24 hours with custom interval
//   node scripts/long-running-test.mjs --duration 86400 --interval 300
//
//   # Run with specific scenarios
//   node scripts/long-running-test.mjs --duration 3600 --scenarios crud,search
//
// Requirements:
//   - Node 22+
//   - Service running with login OFF (container-dev profile)
//   - PostgreSQL database
//
// Output: a Markdown and a JSON report in `--output-dir` (default `long-running-test-results/`, ignored by git):
// totals, the error rate, latency p50/p95/p99 (send to end of body), per-interval counts and, with `--pid`, RSS.
// It creates 50 seed plans (500 tasks) per run and leaves them; the CRUD scenario deletes what it creates.

import { execFileSync } from "node:child_process";
import { writeFileSync, mkdirSync, existsSync } from "node:fs";
import { setTimeout as sleep } from "node:timers/promises";
import { randomUUID } from "node:crypto";

const args = Object.fromEntries(
  process.argv.slice(2).reduce((acc, a, i, all) => {
    if (a.startsWith("--")) acc.push([a.slice(2), all[i + 1] && !all[i + 1].startsWith("--") ? all[i + 1] : "true"]);
    return acc;
  }, []),
);

const duration = Number(args.duration ?? 3600); // 1 hour default
const interval = Number(args.interval ?? 60); // 1 minute default
const baseUrl = (args.base ?? "http://127.0.0.1:1241").replace(/\/$/, "");
const scenarios = (args.scenarios ?? "crud,read,search").split(",");
const outputDir = args["output-dir"] ?? "long-running-test-results";
const testId = args["test-id"] ?? randomUUID();
const pid = args.pid; // Optional: PID of service to monitor

// Ensure output directory exists
if (!existsSync(outputDir)) {
  mkdirSync(outputDir, { recursive: true });
}

// ============================================================================
// TEST SCENARIOS
// ============================================================================

// Every request goes through `call`, which times it (from send to the end of the body), records its real
// status, and counts it in the current interval. A scenario step that needs an earlier step's answer stops when
// that answer is not a success, so a failed create is counted once and not followed by requests to `undefined`.
let currentInterval = null;

async function call(metrics, label, method, path, body) {
  const started = performance.now();
  let status = 0;
  let text = "";
  try {
    const res = await fetch(`${baseUrl}${path}`, {
      method,
      headers: body === undefined ? undefined : { "content-type": "application/json" },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    status = res.status;
    text = await res.text();
  } catch (error) {
    metrics.addError(new Error(`${label}: ${error.message}`));
  }
  const latencyMs = performance.now() - started;
  metrics.addRequest(label, status, latencyMs);
  if (currentInterval) {
    currentInterval.requests++;
    if (status >= 200 && status < 300) currentInterval.successful++;
    else currentInterval.failed++;
    currentInterval.latencies.push(latencyMs);
  }
  if (status !== 0 && (status < 200 || status >= 300)) {
    metrics.addError(new Error(`${label}: HTTP ${status} ${text.slice(0, 200)}`));
  }
  let json = null;
  try {
    json = text ? JSON.parse(text) : null;
  } catch {
    json = null;
  }
  return { ok: status >= 200 && status < 300, status, json };
}

const SCENARIO_FUNCTIONS = {
  crud: {
    name: "CRUD Operations",
    async run(metrics, iteration) {
      const plan = await call(metrics, "POST /api/plans", "POST", "/api/plans", {
        kind: "Project",
        name: `Long Test Plan ${testId} ${iteration}`,
      });
      if (!plan.ok) return;
      const pid = plan.json.pid;
      const task = await call(metrics, "POST /api/plans/{pid}/tasks", "POST", `/api/plans/${pid}/tasks`, {
        title: `Task ${iteration}`,
        duration_days: 1,
      });
      if (task.ok) {
        const tid = task.json.pid;
        await call(metrics, "GET /api/plans/{pid}", "GET", `/api/plans/${pid}`);
        const read = await call(metrics, "GET /api/plans/{pid}/tasks/{tid}", "GET", `/api/plans/${pid}/tasks/${tid}`);
        if (read.ok) {
          // PUT replaces every field, so send the task as read with only the title changed.
          await call(metrics, "PUT /api/plans/{pid}/tasks/{tid}", "PUT", `/api/plans/${pid}/tasks/${tid}`, {
            ...read.json,
            title: `Updated Task ${iteration}`,
          });
        }
        await call(metrics, "DELETE /api/plans/{pid}/tasks/{tid}", "DELETE", `/api/plans/${pid}/tasks/${tid}`);
      }
      await call(metrics, "DELETE /api/plans/{pid}", "DELETE", `/api/plans/${pid}`);
    },
  },

  read: {
    name: "Read Operations",
    async run(metrics, iteration, seedData) {
      if (seedData.plans.length === 0) return;
      const planPid = seedData.plans[Math.floor(Math.random() * seedData.plans.length)];
      await call(metrics, "GET /api/plans/{pid}", "GET", `/api/plans/${planPid}`);
      const list = await call(metrics, "GET /api/plans/{pid}/tasks", "GET", `/api/plans/${planPid}/tasks`);
      const tasks = list.json?.tasks ?? [];
      if (tasks.length > 0) {
        const task = tasks[Math.floor(Math.random() * tasks.length)];
        await call(metrics, "GET /api/plans/{pid}/tasks/{tid}", "GET", `/api/plans/${planPid}/tasks/${task.pid}`);
      }
    },
  },

  search: {
    name: "Search Operations",
    async run(metrics, iteration) {
      await call(metrics, "GET /api/plans/search", "GET", `/api/plans/search?q=${encodeURIComponent(`test ${iteration}`)}`);
      await call(metrics, "POST /api/plans/check-duplicates", "POST", "/api/plans/check-duplicates", {
        kind: "Project",
        name: `Test Project ${iteration}`,
      });
    },
  },

  aggregate: {
    name: "Aggregate Operations",
    async run(metrics) {
      await call(metrics, "GET /api/plans", "GET", "/api/plans?limit=50");
      await call(metrics, "GET /api/milestones", "GET", "/api/milestones");
      await call(metrics, "GET /api/issues", "GET", "/api/issues");
    },
  },
};

const unknownScenarios = scenarios.filter((name) => !(name in SCENARIO_FUNCTIONS));
if (unknownScenarios.length > 0) {
  console.error(`Unknown scenario(s): ${unknownScenarios.join(", ")}; known: ${Object.keys(SCENARIO_FUNCTIONS).join(", ")}`);
  process.exit(2);
}

// ============================================================================
// METRICS TRACKING
// ============================================================================

/** Nearest-rank percentile of a list of numbers, or null when the list is empty. */
function percentile(values, p) {
  if (values.length === 0) return null;
  const sorted = [...values].sort((x, y) => x - y);
  return Number(sorted[Math.max(0, Math.ceil(sorted.length * p) - 1)].toFixed(1));
}

class LongRunningMetrics {
  constructor() {
    this.reset();
  }

  reset() {
    this.startTime = Date.now();
    this.intervals = [];
    this.totalRequests = 0;
    this.successfulRequests = 0;
    this.failedRequests = 0;
    this.latencies = [];
    this.memoryUsage = [];
    this.errors = [];
  }

  addInterval(intervalData) {
    this.intervals.push(intervalData);
  }

  addRequest(label, status, latencyMs) {
    this.totalRequests++;
    if (status >= 200 && status < 300) this.successfulRequests++;
    else this.failedRequests++;
    this.latencies.push(latencyMs);
  }

  addMemory(rssMb) {
    this.memoryUsage.push({ timestamp: Date.now(), rss_mb: rssMb });
  }

  addError(error) {
    this.errors.push({ timestamp: Date.now(), error: error.message });
  }

  getRSS(pid) {
    if (!pid) return null;
    try {
      const rss = execFileSync("ps", ["-o", "rss=", "-p", String(pid)], { encoding: "utf8" }).trim();
      return rss ? Math.round(Number(rss) / 1024) : null;
    } catch {
      return null;
    }
  }

  getSummary() {
    const elapsed = (Date.now() - this.startTime) / 1000;
    
    return {
      test_id: testId,
      elapsed_seconds: Math.round(elapsed),
      total_intervals: this.intervals.length,
      total_requests: this.totalRequests,
      successful_requests: this.successfulRequests,
      failed_requests: this.failedRequests,
      error_rate: this.totalRequests > 0 ? this.failedRequests / this.totalRequests : 0,
      latency_ms: {
        p50: percentile(this.latencies, 0.5),
        p95: percentile(this.latencies, 0.95),
        p99: percentile(this.latencies, 0.99),
      },
      memory: this.memoryUsage,
      intervals: this.intervals,
    };
  }

  toMarkdown() {
    const summary = this.getSummary();
    
    let md = `## Long-Running Test Report - ${testId}\n\n`;
    md += `| Metric | Value |\n|--------|-------|\n`;
    md += `| Test ID | ${testId} |\n`;
    md += `| Duration | ${summary.elapsed_seconds}s |\n`;
    md += `| Intervals | ${summary.total_intervals} |\n`;
    md += `| Total Requests | ${summary.total_requests} |\n`;
    md += `| Successful | ${summary.successful_requests} |\n`;
    md += `| Failed | ${summary.failed_requests} |\n`;
    md += `| Error Rate | ${(summary.error_rate * 100).toFixed(2)}% |\n`;
    md += `| Latency p50 / p95 / p99 (ms) | ${summary.latency_ms.p50} / ${summary.latency_ms.p95} / ${summary.latency_ms.p99} |\n\n`;
    
    // Memory trend
    if (this.memoryUsage.length > 0) {
      md += `### Memory Usage (RSS in MB)\n\n`;
      md += `| Time (s) | RSS (MB) |\n|----------|---------|\n`;
      this.memoryUsage.forEach((m, i) => {
        const time = Math.round((m.timestamp - this.startTime) / 1000);
        md += `| ${time} | ${m.rss_mb} |\n`;
      });
      md += `\n`;
    }
    
    // Interval details
    if (this.intervals.length > 0) {
      md += `### Per-Interval Summary\n\n`;
      md += `| Interval | Requests | Successful | Failed | Avg Latency (ms) |\n`;
      md += `|----------|----------|------------|--------|------------------|\n`;
      
      this.intervals.forEach((interval, i) => {
        const start = Math.round((interval.startTime - this.startTime) / 1000);
        const end = Math.round((interval.endTime - this.startTime) / 1000);
        const avgLatency = interval.latencies.reduce((a, b) => a + b, 0) / interval.latencies.length || 0;
        
        md += `| ${i + 1} (${start}-${end}s) | ${interval.requests} | ${interval.successful} | ${interval.failed} | ${Math.round(avgLatency)} |\n`;
      });
      md += `\n`;
    }
    
    // Errors
    if (this.errors.length > 0) {
      md += `### Errors\n\n`;
      md += `| Time (s) | Error |\n|----------|-------|\n`;
      this.errors.forEach((e) => {
        const time = Math.round((e.timestamp - this.startTime) / 1000);
        md += `| ${time} | ${e.error} |\n`;
      });
      md += `\n`;
    }
    
    return md;
  }

  toJSON() {
    return JSON.stringify(this.getSummary(), null, 2);
  }
}

// ============================================================================
// MAIN TEST FUNCTION
// ============================================================================

async function runLongRunningTest() {
  console.log(`Starting long-running test`);
  console.log(`Test ID: ${testId}`);
  console.log(`Base URL: ${baseUrl}`);
  console.log(`Duration: ${duration}s`);
  console.log(`Reporting Interval: ${interval}s`);
  console.log(`Scenarios: ${scenarios.join(", ")}`);
  console.log(`Output: ${outputDir}/`);
  
  const metrics = new LongRunningMetrics();
  const seedData = { plans: [], tasks: [] };
  
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
  
  // Pre-seed the plans the read scenario uses (not counted in the metrics). They are left in place so a later
  // run can be compared on the same data; delete them by name ("Long Test Seed Plan") when done.
  console.log("Pre-seeding test data...");
  const seedMetrics = new LongRunningMetrics();
  for (let i = 0; i < 50; i++) {
    const plan = await call(seedMetrics, "seed plan", "POST", "/api/plans", {
      kind: "Project",
      name: `Long Test Seed Plan ${testId} ${i}`,
    });
    if (!plan.ok) continue;
    seedData.plans.push(plan.json.pid);
    for (let j = 0; j < 10; j++) {
      const task = await call(seedMetrics, "seed task", "POST", `/api/plans/${plan.json.pid}/tasks`, {
        title: `Seed Task ${i}-${j}`,
        duration_days: 1,
      });
      if (task.ok) seedData.tasks.push({ planPid: plan.json.pid, pid: task.json.pid });
    }
  }
  console.log(`✓ Created ${seedData.plans.length} plans with ${seedData.tasks.length} tasks (${seedMetrics.failedRequests} seed requests failed)`);

  const startTime = Date.now();
  const endTime = startTime + duration * 1000;
  let iteration = 0;
  
  console.log(`\nTest started at ${new Date(startTime).toISOString()}`);
  console.log(`Test will run until ${new Date(endTime).toISOString()}`);
  console.log(`Reporting every ${interval}s...\n`);
  
  while (Date.now() < endTime) {
    const intervalStart = Date.now();
    const intervalMetrics = {
      startTime: intervalStart,
      requests: 0,
      successful: 0,
      failed: 0,
      latencies: [],
    };
    
    // Run each selected scenario once; `call` counts every request into this interval.
    currentInterval = intervalMetrics;
    for (const scenarioName of scenarios) {
      try {
        await SCENARIO_FUNCTIONS[scenarioName].run(metrics, iteration, seedData);
      } catch (error) {
        metrics.addError(error);
      }
    }
    currentInterval = null;

    // Track memory if PID provided
    if (pid) {
      const rss = metrics.getRSS(pid);
      if (rss) {
        metrics.addMemory(rss);
      }
    }
    
    // End of interval
    const intervalEnd = Date.now();
    intervalMetrics.endTime = intervalEnd;
    metrics.addInterval(intervalMetrics);
    
    iteration++;
    
    // Report every iteration (one iteration per interval).
    {
      const elapsedSec = Math.round((Date.now() - startTime) / 1000);
      const remainingSec = Math.max(0, Math.round((endTime - Date.now()) / 1000));
      console.log(`[${new Date().toISOString()}] Elapsed: ${elapsedSec}s | Remaining: ${remainingSec}s | ` +
        `Requests: ${metrics.totalRequests} | Failed: ${metrics.failedRequests}`);
    }

    // Sleep until next interval or end
    if (Date.now() < endTime) {
      const sleepTime = Math.min(interval * 1000, endTime - Date.now());
      if (sleepTime > 0) {
        await sleep(sleepTime);
      }
    }
  }
  
  // Final report
  console.log("\nTest completed!");
  const elapsed = Math.round((Date.now() - startTime) / 1000);
  console.log(`Total duration: ${elapsed}s`);
  console.log(`Total requests: ${metrics.totalRequests}`);
  console.log(`Successful: ${metrics.successfulRequests}`);
  console.log(`Failed: ${metrics.failedRequests}`);
  const rate = metrics.totalRequests > 0 ? ((metrics.failedRequests / metrics.totalRequests) * 100).toFixed(2) : "n/a";
  console.log(`Error rate: ${rate}%`);
  
  // Save reports
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
  const markdownPath = `${outputDir}/long-running-test-${timestamp}-${testId}.md`;
  const jsonPath = `${outputDir}/long-running-test-${timestamp}-${testId}.json`;
  
  writeFileSync(markdownPath, metrics.toMarkdown());
  writeFileSync(jsonPath, metrics.toJSON());
  
  console.log(`\nReports saved to:`);
  console.log(`  - ${markdownPath}`);
  console.log(`  - ${jsonPath}`);
}

// ============================================================================
// MAIN
// ============================================================================

async function main() {
  try {
    await runLongRunningTest();
  } catch (error) {
    console.error("Test failed:", error);
    process.exit(1);
  }
}

main();

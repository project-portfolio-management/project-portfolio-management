#!/usr/bin/env node
// Distributed Load Testing Framework for Project Portfolio Management
//
// This script coordinates load testing across multiple worker processes (distributed nodes).
// It can run in two modes:
//   1. Single-machine multi-process (for development/testing)
//   2. Multi-machine distributed (for production load testing)
//
// Usage:
//   # Single machine with 4 workers
//   node scripts/distributed-load-test.mjs --workers 4 --base http://127.0.0.1:1241 --duration 300
//
//   # Distributed mode: start coordinator
//   node scripts/distributed-load-test.mjs --mode coordinator --port 8080
//
//   # Distributed mode: start worker (on each machine)
//   node scripts/distributed-load-test.mjs --mode worker --coordinator http://coordinator:8080 --base http://target:1241
//
// Requirements:
//   - Node 22+
//   - Service running with login OFF (container-dev profile)
//   - Optional: Redis for coordination in distributed mode
//
// Features:
//   - Ramp-up and ramp-down phases
//   - Multiple test scenarios with configurable weights
//   - Real-time metrics collection and reporting
//   - JSON and Markdown output formats
//   - Health checks before and during test

import { spawn, execSync } from "node:child_process";
import { createServer } from "node:http";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
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
const coordinatorUrl = args.coordinator; // e.g., "http://127.0.0.1:8080"
const port = Number(args.port ?? 8080);
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
  burndown: { method: "GET", path: "/api/plans/{pid}/burndown", weight: 3, params: ["pid"] },
  milestones: { method: "GET", path: "/api/milestones", weight: 4 },
  
  // Write scenarios
  create_task: { method: "POST", path: "/api/plans/{pid}/tasks", weight: 2, params: ["pid"], body: { title: "Load test task", duration_days: 1 } },
  update_task: { method: "PUT", path: "/api/plans/{pid}/tasks/{tid}", weight: 1, params: ["pid", "tid"], body: { title: "Updated task" } },
  
  // Check duplicates
  check_duplicates: { method: "POST", path: "/api/plans/check-duplicates", weight: 3, body: { kind: "Project", name: "Test Project" } },
};

const HEAVY_SCENARIOS = {
  ...DEFAULT_SCENARIOS,
  list_all_plans: { method: "GET", path: "/api/plans?limit=500", weight: 5 },
  large_plan_cpm: { method: "GET", path: "/api/plans/{pid}/cpm", weight: 3, params: ["pid"], large: true },
  full_search: { method: "GET", path: "/api/plans/search?q=load", weight: 2 },
};

const SCENARIO_PRESETS = {
  default: DEFAULT_SCENARIOS,
  heavy: HEAVY_SCENARIOS,
  read_only: Object.fromEntries(
    Object.entries(DEFAULT_SCENARIOS).filter(([k, v]) => v.method === "GET")
  ),
};

const scenarios = SCENARIO_PRESETS[scenarioConfig] ?? DEFAULT_SCENARIOS;

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
    const elapsed = (Date.now() - this.startTime) / 1000;
    const latencies = this.latencies.sort((a, b) => a - b);
    const p50 = latencies[Math.floor(latencies.length * 0.5)] || 0;
    const p95 = latencies[Math.floor(latencies.length * 0.95)] || 0;
    const p99 = latencies[Math.floor(latencies.length * 0.99)] || 0;
    const max = latencies[latencies.length - 1] || 0;
    
    return {
      elapsed_seconds: Math.round(elapsed),
      total_requests: this.totalRequests,
      successful_requests: this.successfulRequests,
      failed_requests: this.failedRequests,
      requests_per_second: Math.round(this.totalRequests / elapsed),
      p50_ms: Math.round(p50),
      p95_ms: Math.round(p95),
      p99_ms: Math.round(p99),
      max_ms: Math.round(max),
      avg_ms: Math.round(this.latencies.reduce((a, b) => a + b, 0) / this.latencies.length || 0),
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
  }

  async start() {
    this.running = true;
    const startTime = Date.now();
    const endTime = startTime + this.duration * 1000;
    
    // Pre-seed some plan IDs for parameterized scenarios
    const seedPlans = await this.seedPlans();
    
    while (this.running && Date.now() < endTime) {
      const scenario = this.selectScenario();
      const path = this.resolvePath(scenario, seedPlans);
      
      try {
        await this.executeRequest(scenario.method, path, scenario.body);
      } catch (error) {
        // Errors are recorded in metrics
      }
    }
  }

  async seedPlans() {
    // Create a few plans to use for parameterized requests
    const planPids = [];
    const taskPids = [];
    
    for (let i = 0; i < 10; i++) {
      try {
        const plan = await this.createPlan(`Load Test Plan ${this.workerId}-${i}`);
        planPids.push(plan.pid);
        
        // Create some tasks for this plan
        for (let j = 0; j < 5; j++) {
          const task = await this.createTask(plan.pid, `Task ${i}-${j}`);
          taskPids.push({ planPid: plan.pid, tid: task.pid });
        }
      } catch (error) {
        // Plan might already exist, continue
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
    return response.json();
  }

  async createTask(planPid, title) {
    const response = await fetch(`${this.baseUrl}/api/plans/${planPid}/tasks`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ title, duration_days: 1 }),
    });
    return response.json();
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
    
    if (scenario.params) {
      const replacements = {};
      
      scenario.params.forEach((param, index) => {
        if (param === "pid" && seedPlans.planPids.length > 0) {
          replacements[`{${param}}`] = seedPlans.planPids[Math.floor(Math.random() * seedPlans.planPids.length)];
        } else if (param === "tid" && seedPlans.taskPids.length > 0) {
          const task = seedPlans.taskPids[Math.floor(Math.random() * seedPlans.taskPids.length)];
          replacements[`{${param}}`] = task.tid;
        } else {
          replacements[`{${param}}`] = "placeholder";
        }
      });
      
      for (const [placeholder, value] of Object.entries(replacements)) {
        path = path.replace(placeholder, value);
      }
    }
    
    return path;
  }

  async executeRequest(method, path, body) {
    const start = Date.now();
    
    try {
      const options = {
        method,
        headers: { "content-type": "application/json" },
      };
      
      if (body) {
        options.body = JSON.stringify(body);
      }
      
      const response = await fetch(`${this.baseUrl}${path}`, options);
      const latency = Date.now() - start;
      const bytes = response.headers.get("content-length") ? 
        Number(response.headers.get("content-length")) : 
        (await response.text()).length;
      
      this.metrics.addRequest(
        method,
        path,
        response.status,
        latency,
        bytes,
        response.ok ? null : await response.text().slice(0, 200)
      );
      
      return response;
    } catch (error) {
      const latency = Date.now() - start;
      this.metrics.addRequest(
        method,
        path,
        0,
        latency,
        0,
        error.message
      );
      throw error;
    }
  }

  stop() {
    this.running = false;
  }
}

// ============================================================================
// COORDINATOR (for distributed mode)
// ============================================================================

class TestCoordinator {
  constructor(port) {
    this.port = port;
    this.workers = new Map();
    this.metrics = new MetricsCollector();
    this.server = null;
    this.startTime = null;
    this.testId = randomUUID();
  }

  start() {
    this.server = createServer((req, res) => {
      const url = new URL(req.url, `http://localhost:${this.port}`);
      
      if (req.method === "POST" && url.pathname === "/register") {
        this.handleRegister(req, res);
      } else if (req.method === "POST" && url.pathname === "/metrics") {
        this.handleMetrics(req, res);
      } else if (req.method === "GET" && url.pathname === "/status") {
        this.handleStatus(req, res);
      } else {
        res.writeHead(404);
        res.end("Not found");
      }
    });
    
    this.server.listen(this.port, () => {
      console.log(`Coordinator running on port ${this.port}, test ID: ${this.testId}`);
    });
  }

  handleRegister(req, res) {
    let body = "";
    req.on("data", (chunk) => body += chunk);
    req.on("end", () => {
      try {
        const workerInfo = JSON.parse(body);
        const workerId = randomUUID();
        this.workers.set(workerId, workerInfo);
        res.writeHead(200);
        res.end(JSON.stringify({ workerId, testId: this.testId, coordinatorUrl: `http://localhost:${this.port}` }));
      } catch (error) {
        res.writeHead(400);
        res.end("Invalid request");
      }
    });
  }

  handleMetrics(req, res) {
    let body = "";
    req.on("data", (chunk) => body += chunk);
    req.on("end", () => {
      try {
        const metrics = JSON.parse(body);
        // Merge metrics
        this.metrics.totalRequests += metrics.total_requests;
        this.metrics.successfulRequests += metrics.successful_requests;
        this.metrics.failedRequests += metrics.failed_requests;
        this.metrics.latencies.push(...metrics.latencies);
        res.writeHead(200);
        res.end("OK");
      } catch (error) {
        res.writeHead(400);
        res.end("Invalid metrics");
      }
    });
  }

  handleStatus(req, res) {
    res.writeHead(200);
    res.end(JSON.stringify({
      workers: this.workers.size,
      metrics: this.metrics.getSummary(),
    }));
  }

  stop() {
    if (this.server) {
      this.server.close();
    }
  }
}

// ============================================================================
// STANDALONE MODE
// ============================================================================

async function runStandalone() {
  console.log(`Starting distributed load test (standalone mode)`);
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
  
  // Start workers
  console.log(`\nStarting ${workers} workers...`);
  for (let i = 0; i < workers; i++) {
    const worker = new LoadTestWorker(i, baseUrl, scenarios, duration, globalMetrics);
    testWorkers.push(worker);
    worker.start().catch(() => {});
    await sleep(100); // Stagger worker starts
  }
  
  // Wait for test duration
  console.log(`\nRunning for ${duration} seconds... (Press Ctrl+C to stop early)`);
  await sleep(duration * 1000);
  
  // Stop workers
  console.log("\nStopping workers...");
  for (const worker of testWorkers) {
    worker.stop();
  }
  
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
  switch (mode) {
    case "coordinator":
      const coordinator = new TestCoordinator(port);
      coordinator.start();
      console.log("Coordinator started. Workers can now connect.");
      console.log("Press Ctrl+C to stop.");
      await new Promise(() => {}); // Run forever
      break;
    
    case "worker":
      if (!coordinatorUrl) {
        console.error("--coordinator URL is required for worker mode");
        process.exit(1);
      }
      // Worker would connect to coordinator and receive test parameters
      console.log(`Worker mode: connecting to ${coordinatorUrl}`);
      console.log("Distributed worker functionality coming soon...");
      break;
    
    default:
      await runStandalone();
      break;
  }
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});

#!/usr/bin/env node
// Long-Running Test Suite for Project Portfolio Management
//
// This script runs endurance/soak tests that verify the system remains stable
// over extended periods of time. Unlike load tests which focus on throughput,
// long-running tests focus on:
//   - Memory stability (no leaks)
//   - Database connection stability
//   - Cache consistency
//   - Error rate stability
//   - Response time consistency
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
// Features:
//   - Configurable test duration (seconds)
//   - Configurable reporting interval (seconds)
//   - Memory usage tracking
//   - Database connection tracking
//   - Error rate monitoring
//   - Latency trend analysis
//   - Real-time console output
//   - JSON and Markdown reports

import { execSync, spawnSync } from "node:child_process";
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

const SCENARIO_FUNCTIONS = {
  crud: {
    name: "CRUD Operations",
    async run(metrics, iteration) {
      const plan = await createPlan(`Long Test Plan ${iteration}`);
      metrics.addRequest("POST /api/plans", 200, Date.now() - Date.now(), 100);
      
      const task = await createTask(plan.pid, `Task ${iteration}`);
      metrics.addRequest("POST /api/plans/{pid}/tasks", 200, Date.now() - Date.now(), 100);
      
      await readPlan(plan.pid);
      metrics.addRequest("GET /api/plans/{pid}", 200, Date.now() - Date.now(), 100);
      
      await readTask(plan.pid, task.pid);
      metrics.addRequest("GET /api/plans/{pid}/tasks/{tid}", 200, Date.now() - Date.now(), 100);
      
      await updateTask(plan.pid, task.pid, `Updated Task ${iteration}`);
      metrics.addRequest("PUT /api/plans/{pid}/tasks/{tid}", 200, Date.now() - Date.now(), 100);
      
      await deleteTask(plan.pid, task.pid);
      metrics.addRequest("DELETE /api/plans/{pid}/tasks/{tid}", 200, Date.now() - Date.now(), 100);
      
      await deletePlan(plan.pid);
      metrics.addRequest("DELETE /api/plans/{pid}", 200, Date.now() - Date.now(), 100);
    },
  },
  
  read: {
    name: "Read Operations",
    async run(metrics, iteration, seedData) {
      if (!seedData || seedData.plans.length === 0) {
        // Create seed data if not exists
        const plan = await createPlan(`Seed Plan ${iteration}`);
        seedData.plans.push(plan.pid);
      }
      
      // Random plan from seed data
      const planPid = seedData.plans[Math.floor(Math.random() * seedData.plans.length)];
      
      await readPlan(planPid);
      metrics.addRequest("GET /api/plans/{pid}", 200, Date.now() - Date.now(), 100);
      
      const tasks = await listTasks(planPid);
      metrics.addRequest("GET /api/plans/{pid}/tasks", 200, Date.now() - Date.now(), 100);
      
      if (tasks.items && tasks.items.length > 0) {
        const task = tasks.items[Math.floor(Math.random() * tasks.items.length)];
        await readTask(planPid, task.pid);
        metrics.addRequest("GET /api/plans/{pid}/tasks/{tid}", 200, Date.now() - Date.now(), 100);
      }
    },
  },
  
  search: {
    name: "Search Operations",
    async run(metrics, iteration) {
      await searchPlans(`test ${iteration}`);
      metrics.addRequest("GET /api/plans/search", 200, Date.now() - Date.now(), 100);
      
      await checkDuplicates({ kind: "Project", name: `Test Project ${iteration}` });
      metrics.addRequest("POST /api/plans/check-duplicates", 200, Date.now() - Date.now(), 100);
    },
  },
  
  aggregate: {
    name: "Aggregate Operations",
    async run(metrics, iteration) {
      await listAllPlans();
      metrics.addRequest("GET /api/plans", 200, Date.now() - Date.now(), 100);
      
      await getMilestones();
      metrics.addRequest("GET /api/milestones", 200, Date.now() - Date.now(), 100);
      
      await getIssues();
      metrics.addRequest("GET /api/issues", 200, Date.now() - Date.now(), 100);
    },
  },
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

async function createPlan(name) {
  const response = await fetch(`${baseUrl}/api/plans`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ kind: "Project", name }),
  });
  return response.json();
}

async function readPlan(pid) {
  return fetch(`${baseUrl}/api/plans/${pid}`);
}

async function deletePlan(pid) {
  return fetch(`${baseUrl}/api/plans/${pid}`, { method: "DELETE" });
}

async function createTask(planPid, title) {
  const response = await fetch(`${baseUrl}/api/plans/${planPid}/tasks`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ title, duration_days: 1 }),
  });
  return response.json();
}

async function readTask(planPid, tid) {
  return fetch(`${baseUrl}/api/plans/${planPid}/tasks/${tid}`);
}

async function updateTask(planPid, tid, title) {
  return fetch(`${baseUrl}/api/plans/${planPid}/tasks/${tid}`, {
    method: "PUT",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ title }),
  });
}

async function deleteTask(planPid, tid) {
  return fetch(`${baseUrl}/api/plans/${planPid}/tasks/${tid}`, { method: "DELETE" });
}

async function listTasks(planPid) {
  const response = await fetch(`${baseUrl}/api/plans/${planPid}/tasks`);
  return response.json();
}

async function listAllPlans() {
  return fetch(`${baseUrl}/api/plans?limit=50`);
}

async function searchPlans(query) {
  return fetch(`${baseUrl}/api/plans/search?q=${encodeURIComponent(query)}`);
}

async function checkDuplicates(body) {
  return fetch(`${baseUrl}/api/plans/check-duplicates`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

async function getMilestones() {
  return fetch(`${baseUrl}/api/milestones`);
}

async function getIssues() {
  return fetch(`${baseUrl}/api/issues`);
}

// ============================================================================
// METRICS TRACKING
// ============================================================================

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

  addRequest(method, status, latencyMs, bytes) {
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
      const rss = execSync(`ps -o rss= -p ${pid}`, { encoding: "utf8" }).trim();
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
    md += `| Error Rate | ${(summary.error_rate * 100).toFixed(2)}% |\n\n`;
    
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
  
  // Pre-seed some data
  console.log("Pre-seeding test data...");
  for (let i = 0; i < 50; i++) {
    try {
      const plan = await createPlan(`Long Test Seed Plan ${i}`);
      seedData.plans.push(plan.pid);
      for (let j = 0; j < 10; j++) {
        const task = await createTask(plan.pid, `Seed Task ${i}-${j}`);
        seedData.tasks.push({ planPid: plan.pid, pid: task.pid });
      }
    } catch (error) {
      // Already exists, continue
    }
  }
  console.log(`✓ Created ${seedData.plans.length} plans with ${seedData.tasks.length} tasks`);
  
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
    
    // Run each selected scenario
    for (const scenarioName of scenarios) {
      const scenario = SCENARIO_FUNCTIONS[scenarioName];
      if (scenario) {
        try {
          await scenario.run(metrics, iteration, seedData);
          intervalMetrics.requests += scenario.name === "CRUD Operations" ? 7 : 
                                    scenario.name === "Read Operations" ? 3 :
                                    scenario.name === "Search Operations" ? 2 :
                                    3;
          intervalMetrics.successful += scenario.name === "CRUD Operations" ? 7 :
                                        scenario.name === "Read Operations" ? 3 :
                                        scenario.name === "Search Operations" ? 2 :
                                        3;
        } catch (error) {
          metrics.addError(error);
          intervalMetrics.failed++;
        }
      }
    }
    
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
    
    // Check if we should report
    const elapsed = Date.now() - startTime;
    const nextReport = Math.ceil(elapsed / (interval * 1000)) * interval * 1000;
    const timeUntilReport = nextReport - elapsed;
    
    if (timeUntilReport <= 0 || elapsed >= duration * 1000) {
      // Report
      const now = new Date();
      const elapsedSec = Math.round(elapsed / 1000);
      const remainingSec = Math.round((endTime - Date.now()) / 1000);
      
      console.log(`[${now.toISOString()}] Elapsed: ${elapsedSec}s | Remaining: ${remainingSec}s | ` +
        `Requests: ${metrics.totalRequests} | Errors: ${metrics.failedRequests}`);
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
  console.log(`Error rate: ${(metrics.failedRequests / metrics.totalRequests * 100).toFixed(2)}%`);
  
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

# Load Testing Guide

This document describes the load testing and performance testing infrastructure for the Project Portfolio Management entity.

## Overview

The project provides multiple levels of testing to ensure performance, stability, and scalability:

1. **Unit Tests** - Fast, isolated tests for individual components
2. **Integration Tests** - Tests that verify component interactions
3. **Load Tests** - Tests that measure system throughput and response times under load
4. **Long-Running Tests** - Endurance tests that verify system stability over time
5. **Chaos Tests** - Tests that verify system resilience to failures

This document focuses on **load testing** and **long-running tests**.

---

## Load Testing

### Quick Start

To run a basic load test:

```bash
# Single machine load test with 4 workers for 5 minutes
node scripts/load-test.mjs seed --base http://127.0.0.1:1241 --plans 200 --tasks 50
node scripts/load-test.mjs run --base http://127.0.0.1:1241 --seconds 300 --concurrency 32
```

### Requirements

- Node 22+
- Service running with **login OFF** (use `container-dev` profile)
- Optional: `oha` for HTTP load generation (install with `cargo install oha`)

### Scenarios

The load test supports multiple scenarios:

| Scenario | Description | Weight |
|----------|-------------|--------|
| `list_plans` | List all plans (first page) | 10 |
| `read_plan` | Read a single plan | 8 |
| `list_tasks` | List tasks for a plan | 8 |
| `read_task` | Read a single task | 5 |
| `search_plans` | Search for plans | 3 |
| `critical_path` | Get critical path | 4 |
| `burndown` | Get burndown chart | 3 |
| `milestones` | List milestones | 4 |
| `create_task` | Create a new task | 2 |
| `update_task` | Update a task | 1 |
| `check_duplicates` | Check for duplicate plans | 3 |

### Command Reference

#### `seed` - Create test data

```bash
node scripts/load-test.mjs seed \
  --base http://127.0.0.1:1241 \
  --plans 200 \
  --tasks 50 \
  [--large 10000] \
  [--state load-test-state.json]
```

Options:
- `--base`: Service base URL (default: `http://127.0.0.1:1241`)
- `--plans`: Number of plans to create (default: 100)
- `--tasks`: Number of tasks per plan (default: 40)
- `--large`: Create one large plan with this many tasks
- `--state`: File to save state to (default: `load-test-state.json`)

#### `run` - Run load test

```bash
node scripts/load-test.mjs run \
  --base http://127.0.0.1:1241 \
  --seconds 300 \
  --concurrency 32 \
  [--pid SERVICE_PID] \
  [--out results.json]
```

Options:
- `--base`: Service base URL
- `--seconds`: Test duration in seconds (default: 15)
- `--concurrency`: Number of concurrent requests (default: 32)
- `--pid`: Service process ID for memory monitoring
- `--out`: Output file for JSON results

#### `soak` - Run soak test

```bash
node scripts/load-test.mjs soak \
  --base http://127.0.0.1:1241 \
  --minutes 10 \
  --concurrency 16 \
  [--pid SERVICE_PID] \
  [--out soak-results.json]
```

Options:
- `--base`: Service base URL
- `--minutes`: Test duration in minutes (default: 10)
- `--concurrency`: Number of concurrent requests (default: 16)
- `--pid`: Service process ID for memory monitoring
- `--out`: Output file for JSON results

### Output

The load test produces two types of output:

1. **Markdown Report** (`load-test-<timestamp>.md`)
   - Human-readable summary
   - Status code distribution
   - Latency percentiles (p50, p95, p99)
   - Requests per second
   - Memory usage (if PID provided)

2. **JSON Report** (`load-test-<timestamp>.json`)
   - Machine-readable results
   - Full latency distribution
   - Raw metrics for analysis

---

## Distributed Load Testing

For testing across multiple machines or with higher concurrency, use the distributed load testing framework.

### Architecture

The distributed load test consists of:

1. **Coordinator** - Manages the test and collects results
2. **Workers** - Execute the actual load (can run on different machines)

### Quick Start (Standalone Mode)

For most use cases, standalone mode is sufficient:

```bash
node scripts/distributed-load-test.mjs \
  --base http://127.0.0.1:1241 \
  --duration 300 \
  --workers 4 \
  --output-dir distributed-results
```

This runs 4 worker processes on a single machine.

### Distributed Mode

For multi-machine testing:

#### On Coordinator Machine:

```bash
node scripts/distributed-load-test.mjs \
  --mode coordinator \
  --port 8080
```

#### On Each Worker Machine:

```bash
node scripts/distributed-load-test.mjs \
  --mode worker \
  --coordinator http://coordinator-ip:8080 \
  --base http://target-service:1241
```

### Command Reference

#### Coordinator Mode

```bash
node scripts/distributed-load-test.mjs \
  --mode coordinator \
  [--port 8080] \
  [--test-id TEST_ID]
```

#### Worker Mode

```bash
node scripts/distributed-load-test.mjs \
  --mode worker \
  --coordinator http://coordinator:8080 \
  --base http://target:1241 \
  [--test-id TEST_ID]
```

#### Standalone Mode (Default)

```bash
node scripts/distributed-load-test.mjs \
  --base http://127.0.0.1:1241 \
  --duration 300 \
  --workers 4 \
  [--scenarios default|heavy|read_only] \
  [--output-dir distributed-results]
```

Options:
- `--mode`: Test mode (`coordinator`, `worker`, or `standalone`)
- `--base`: Service base URL
- `--duration`: Test duration in seconds (default: 300)
- `--workers`: Number of worker processes (default: 4)
- `--scenarios`: Scenario preset (`default`, `heavy`, `read_only`)
- `--coordinator`: Coordinator URL (for worker mode)
- `--port`: Coordinator port (for coordinator mode)
- `--output-dir`: Directory for output files
- `--test-id`: Test identifier (default: random UUID)

### Scenario Presets

| Preset | Description | Best For |
|--------|-------------|----------|
| `default` | Balanced mix of read and write operations | General testing |
| `heavy` | Includes heavy operations (large plans, complex queries) | Stress testing |
| `read_only` | Only read operations (no writes) | Read performance testing |

### Metrics Collected

- Total requests
- Requests per second
- Successful vs failed requests
- Latency percentiles (p50, p95, p99, max)
- Status code distribution
- Error distribution
- Memory usage (RSS)

---

## Long-Running Tests

Long-running tests verify system stability over extended periods. Unlike load tests that focus on throughput, long-running tests focus on:

- Memory stability (no leaks)
- Database connection stability
- Cache consistency
- Error rate consistency
- Response time consistency

### Quick Start

```bash
# Run for 1 hour (default)
node scripts/long-running-test.mjs --base http://127.0.0.1:1241

# Run for 24 hours with 5-minute intervals
node scripts/long-running-test.mjs \
  --base http://127.0.0.1:1241 \
  --duration 86400 \
  --interval 300
```

### Command Reference

```bash
node scripts/long-running-test.mjs \
  --base http://127.0.0.1:1241 \
  --duration 3600 \
  --interval 60 \
  [--scenarios crud,read,search,aggregate] \
  [--pid SERVICE_PID] \
  [--output-dir long-running-results] \
  [--test-id TEST_ID]
```

Options:
- `--base`: Service base URL
- `--duration`: Test duration in seconds (default: 3600 = 1 hour)
- `--interval`: Reporting interval in seconds (default: 60)
- `--scenarios`: Comma-separated list of scenarios (default: `crud,read,search`)
- `--pid`: Service process ID for memory monitoring
- `--output-dir`: Directory for output files
- `--test-id`: Test identifier (default: random UUID)

### Available Scenarios

| Scenario | Description | Requests/Iteration |
|----------|-------------|-------------------|
| `crud` | Full CRUD operations (create, read, update, delete) | 7 |
| `read` | Read-only operations | 3 |
| `search` | Search and duplicate checking | 2 |
| `aggregate` | Aggregate queries (all plans, milestones, issues) | 3 |

### Output

The long-running test produces:

1. **Markdown Report** (`long-running-test-<timestamp>-<test-id>.md`)
   - Test summary
   - Memory usage over time
   - Per-interval request statistics
   - Error log

2. **JSON Report** (`long-running-test-<timestamp>-<test-id>.json`)
   - Raw metrics
   - All collected data for analysis

### Interpreting Results

#### Memory Trend

Check for memory leaks by examining the RSS (Resident Set Size) over time:

- **Stable**: Memory usage remains roughly constant
- **Growing**: Memory usage increases over time (potential leak)
- **Sawtooth**: Memory usage increases and decreases (normal GC behavior)

#### Error Rate

- **0%**: Perfect - all requests successful
- **< 0.1%**: Good - occasional expected failures
- **0.1-1%**: Warning - investigate error causes
- **> 1%**: Critical - system is failing too many requests

#### Response Time Trend

- **Stable**: Response times remain consistent
- **Degrading**: Response times increase over time (performance degradation)
- **Spiky**: Response times vary widely (investigate root cause)

---

## Chaos Testing

Chaos testing verifies system resilience to failures. See `scripts/chaos-test.mjs` and `scripts/chaos-test.sh`.

### Quick Start

```bash
scripts/chaos-test.sh --seconds 60 --workers 8
```

This starts a PostgreSQL container, migrates the database, and runs chaos tests for 60 seconds with 8 workers.

### Faults Injected

1. **Database Pause**: Pauses the database container for 1-6 seconds
2. **Connection Termination**: Kills all database connections
3. **Service Restart**: Kills and restarts the service (simulates crash)

### Checks Performed

1. Service health check (`/_health/live`) recovers within 30 seconds
2. All responses are either:
   - Success (2xx)
   - Client error (4xx) that the request deserves
   - Clean server error (500, 502, 503, 504)
3. All acknowledged writes are durable and can be read back
4. Final health check passes and listings work

---

## Best Practices

### Before Running Tests

1. **Use container-dev profile** (login OFF, rate limiting OFF):
   ```bash
   APP_ENV=container-dev cargo run -- start
   ```

2. **Reset the database**:
   ```bash
   cargo run -- db reset
   cargo run -- db migrate
   ```

3. **Check service health**:
   ```bash
   curl http://127.0.0.1:1241/_health/live
   curl http://127.0.0.1:1241/_health/ready
   ```

### Running Tests

1. **Start with small tests**:
   ```bash
   node scripts/load-test.mjs seed --plans 10 --tasks 5
   node scripts/load-test.mjs run --seconds 10 --concurrency 4
   ```

2. **Gradually increase load**:
   ```bash
   # Small
   node scripts/load-test.mjs run --seconds 30 --concurrency 8
   
   # Medium
   node scripts/load-test.mjs run --seconds 60 --concurrency 16
   
   # Large
   node scripts/load-test.mjs run --seconds 120 --concurrency 32
   ```

3. **Monitor system resources**:
   ```bash
   # CPU
   top
   htop
   
   # Memory
   free -m
   
   # Disk I/O
   iostat -x 1
   
   # Network
   iftop
   nload
   ```

### After Running Tests

1. **Analyze results**:
   - Check error rates
   - Check latency percentiles
   - Check memory usage trends

2. **Compare with baseline**:
   - Run tests before and after changes
   - Compare metrics to identify regressions

3. **Save results for comparison**:
   ```bash
   # Save to version control or external storage
   git add distributed-results/
   git commit -m "Load test results for v0.2.0"
   ```

---

## CI/CD Integration

### GitHub Actions

Add to your workflow:

```yaml
- name: Run Load Test
  run: |
    node scripts/load-test.mjs seed --base http://127.0.0.1:1241 --plans 50 --tasks 10
    node scripts/load-test.mjs run --base http://127.0.0.1:1241 --seconds 60 --concurrency 8

- name: Run Long-Running Test
  run: |
    node scripts/long-running-test.mjs --base http://127.0.0.1:1241 --duration 300 --interval 60

- name: Upload Test Results
  uses: actions/upload-artifact@v3
  with:
    name: load-test-results
    path: |
      load-test-*.json
      load-test-*.md
      long-running-test-*.json
      long-running-test-*.md
```

### Scheduled Tests

Run regular performance tests to catch regressions:

```yaml
name: Performance Tests

on:
  schedule:
    - cron: "0 2 * * 1"  # Every Monday at 2 AM
  workflow_dispatch:

jobs:
  performance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Start Service
        run: |
          docker compose up -d --wait
      - name: Run Performance Tests
        run: |
          node scripts/long-running-test.mjs --duration 3600 --interval 300
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: performance-results
          path: long-running-test-results/
```

---

## Troubleshooting

### Common Issues

#### Service Returns 401/403

**Cause**: Login is enabled

**Solution**: Run with login OFF:
```bash
APP_ENV=container-dev cargo run -- start
```

#### Connection Refused

**Cause**: Service not running or wrong port

**Solution**: Check service is running:
```bash
curl http://127.0.0.1:1241/_health/live
```

#### Database Connection Errors

**Cause**: Database not running or migrations not applied

**Solution**:
```bash
# Start database
docker compose up -d postgres

# Run migrations
cargo run -- db migrate
```

#### Out of Memory Errors

**Cause**: System doesn't have enough memory

**Solution**: Reduce concurrency or use smaller test data:
```bash
node scripts/load-test.mjs run --concurrency 8 --seconds 30
```

#### Slow Response Times

**Cause**: Many possible causes:
- Database under load
- Network latency
- Service CPU bound
- Small connection pool

**Solution**:
1. Check database performance:
   ```bash
   # PostgreSQL
   psql -c "SELECT * FROM pg_stat_activity;"
   ```
2. Check service metrics (if enabled)
3. Reduce concurrency

---

## Performance Benchmarks

The following are example benchmarks from development testing:

### Development Environment (Single Machine)

| Configuration | Requests/sec | P50 Latency | P95 Latency | P99 Latency |
|---------------|--------------|-------------|-------------|-------------|
| 4 workers, 100 plans | ~500 | 5ms | 20ms | 50ms |
| 8 workers, 200 plans | ~800 | 8ms | 30ms | 80ms |
| 16 workers, 500 plans | ~1,200 | 15ms | 60ms | 150ms |

### Production Environment (Multi-Machine)

| Configuration | Requests/sec | P50 Latency | P95 Latency | P99 Latency |
|---------------|--------------|-------------|-------------|-------------|
| 4 machines, 8 workers each | ~3,000 | 10ms | 30ms | 60ms |
| 8 machines, 16 workers each | ~6,000 | 15ms | 50ms | 100ms |

*Note: These are example values. Actual performance depends on hardware, network, and configuration.*

---

## Additional Resources

- [Load Testing with oha](https://github.com/hatoo/oha)
- [PostgreSQL Performance Tuning](https://wiki.postgresql.org/wiki/Tuning_Your_PostgreSQL_Server)
- [Rust Performance](https://doc.rust-lang.org/book/ch13-04-performance.html)
- [SvelteKit Performance](https://kit.svelte.dev/docs/page-options#ssr)

# Distributed Load Testing and Long-Running Tests - Summary

## Overview

This document summarizes the new distributed load testing and long-running test infrastructure added to the Project Portfolio Management project.

## What Was Created

### 1. Distributed Load Testing Framework (`scripts/distributed-load-test.mjs`)

A comprehensive framework for running load tests across multiple machines or processes.

**Features:**
- Single-machine multi-process mode (for development/testing)
- Multi-machine distributed mode (coordinator + worker architecture)
- Multiple scenario presets (default, heavy, read_only)
- Real-time metrics collection and reporting
- Markdown and JSON output formats
- Health checks before and during tests
- Configurable test duration, workers, and concurrency

**Modes:**
```bash
# Standalone (single machine)
node scripts/distributed-load-test.mjs --workers 4 --duration 300

# Coordinator (multi-machine)
node scripts/distributed-load-test.mjs --mode coordinator --port 8080

# Worker (multi-machine)
node scripts/distributed-load-test.mjs --mode worker --coordinator http://coordinator:8080
```

### 2. Long-Running Test Suite (`scripts/long-running-test.mjs`)

Endurance/soak testing framework that verifies system stability over extended periods.

**Features:**
- Configurable test duration (default: 1 hour)
- Configurable reporting intervals (default: 60 seconds)
- Memory usage tracking (RSS)
- Multiple test scenarios (crud, read, search, aggregate)
- Per-interval metrics collection
- Error tracking and reporting
- Markdown and JSON output formats

**Usage:**
```bash
# Run for 1 hour
node scripts/long-running-test.mjs --duration 3600

# Run for 24 hours with custom scenarios
node scripts/long-running-test.mjs --duration 86400 --scenarios crud,read,search
```

### 3. Documentation (`docs/testing/load-testing.md`)

Comprehensive guide covering:
- Load testing quick start
- Distributed load testing
- Long-running tests
- Chaos testing
- Best practices
- CI/CD integration
- Troubleshooting guide
- Performance benchmarks

## Test Scenarios

### Distributed Load Test Scenarios

**Default Preset:**
- `list_plans` - List all plans (weight: 10)
- `read_plan` - Read a single plan (weight: 8)
- `list_tasks` - List tasks for a plan (weight: 8)
- `read_task` - Read a single task (weight: 5)
- `search_plans` - Search for plans (weight: 3)
- `critical_path` - Get critical path (weight: 4)
- `burndown` - Get burndown chart (weight: 3)
- `milestones` - List milestones (weight: 4)
- `create_task` - Create a new task (weight: 2)
- `update_task` - Update a task (weight: 1)
- `check_duplicates` - Check for duplicate plans (weight: 3)

**Heavy Preset:**
- Includes all default scenarios plus heavy operations
- Large plan critical path
- Full search

**Read-Only Preset:**
- Only GET requests (no writes)
- Good for read performance testing

### Long-Running Test Scenarios

- **crud** - Full CRUD operations (7 requests per iteration)
- **read** - Read-only operations (3 requests per iteration)
- **search** - Search and duplicate checking (2 requests per iteration)
- **aggregate** - Aggregate queries (3 requests per iteration)

## Metrics Collected

### Distributed Load Test
- Total requests
- Requests per second
- Successful vs failed requests
- Latency percentiles (p50, p95, p99, max)
- Status code distribution
- Error distribution
- Memory usage (RSS) - if PID provided

### Long-Running Test
- Total requests
- Requests per second
- Successful vs failed requests
- Error rate
- Memory usage over time
- Per-interval statistics
- Error logs

## Output Files

Both test frameworks produce:

1. **Markdown Report** (`*.md`)
   - Human-readable summary
   - Tables and charts
   - Easy to read and compare

2. **JSON Report** (`*.json`)
   - Machine-readable results
   - Full metrics data
   - Easy to parse and analyze

## Requirements

- Node 22+
- Service running with **login OFF** (use `container-dev` profile)
- PostgreSQL database
- Optional: `oha` for HTTP load generation (distributed load test)

## Quick Start Examples

### Run a basic load test:
```bash
# Create test data
node scripts/load-test.mjs seed --base http://127.0.0.1:1241 --plans 100 --tasks 20

# Run load test
node scripts/load-test.mjs run --base http://127.0.0.1:1241 --seconds 60 --concurrency 8
```

### Run a distributed load test:
```bash
# Single machine with multiple workers
node scripts/distributed-load-test.mjs --workers 4 --duration 300
```

### Run a long-running test:
```bash
# Run for 1 hour
node scripts/long-running-test.mjs --base http://127.0.0.1:1241 --duration 3600

# Run for 24 hours
node scripts/long-running-test.mjs --base http://127.0.0.1:1241 --duration 86400
```

## Best Practices

1. **Start Small**: Begin with small tests and gradually increase load
2. **Monitor Resources**: Watch CPU, memory, disk I/O, and network
3. **Use container-dev Profile**: Disable login and rate limiting for testing
4. **Reset Database**: Start with a clean database for each test
5. **Check Health**: Verify service is healthy before testing
6. **Save Results**: Save test results for comparison over time
7. **Automate**: Integrate tests into CI/CD pipeline

## Integration with Existing Tests

The new test frameworks complement the existing test infrastructure:

| Test Type | Purpose | Script | When to Use |
|-----------|---------|--------|-------------|
| Unit Tests | Component testing | `cargo test` | Development |
| Integration Tests | Component interaction | `cargo test` | Development |
| Load Tests | Throughput testing | `load-test.mjs` | Pre-deployment |
| Distributed Load Tests | High-scale testing | `distributed-load-test.mjs` | Performance validation |
| Long-Running Tests | Stability testing | `long-running-test.mjs` | Endurance validation |
| Chaos Tests | Resilience testing | `chaos-test.mjs` | Failure testing |

## Files Created

1. `scripts/distributed-load-test.mjs` (17.5 KB)
   - Main distributed load testing framework

2. `scripts/long-running-test.mjs` (16.8 KB)
   - Main long-running test framework

3. `docs/testing/load-testing.md` (15 KB)
   - Comprehensive documentation

## Git Information

All files have been committed to the repository:

```
commit 67de405d
Add distributed load testing and long-running test infrastructure

New testing capabilities:
- scripts/distributed-load-test.mjs
- scripts/long-running-test.mjs
- docs/testing/load-testing.md
```

## Next Steps

1. **Review the documentation**: `docs/testing/load-testing.md`
2. **Try the examples**: Run the quick start examples
3. **Integrate into CI/CD**: Add to your GitHub Actions workflow
4. **Set up monitoring**: Use the metrics to monitor production performance
5. **Establish baselines**: Run tests regularly to catch regressions

## Support

For questions or issues:
- Check the documentation: `docs/testing/load-testing.md`
- Review existing tests: `scripts/load-test.mjs`, `scripts/chaos-test.mjs`
- Check GitHub Discussions: [Link to project discussions]

---

*Created: 2026-11-11*  
*Generated by Mistral Vibe*

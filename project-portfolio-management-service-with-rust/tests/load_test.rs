//! A load test in Rust against a **running** service: the Rust counterpart of
//! `scripts/load-test.mjs run` (which uses `oha` and covers more scenarios, and is
//! where the published numbers in `BENCHMARKS.md` come from).
//!
//! `CONCURRENCY` workers each send requests back to back, without waiting for one
//! another, for `DURATION` seconds. Each request is timed from send to the end of
//! its body. The test prints requests per second and p50/p95/p99/max per endpoint,
//! and fails on any 5xx, any transport error, or no successful request.
//!
//! ```sh
//! APP_ENV=container-dev cargo run -- start      # login and rate limiting off
//! PPM_LOAD_TEST_BASE=http://127.0.0.1:1241 PPM_LOAD_TEST_SECONDS=10 PPM_LOAD_TEST_CONCURRENCY=16 \
//!   cargo test --test load_test -- --ignored --nocapture
//! ```
//!
//! `#[ignore]`d, and skipped by CI's `api-db` job: it needs a service that is
//! already running, which CI does not start. Read-only: it creates nothing.
//! Client, service and database usually share one machine, so treat the figures
//! as a floor for comparing versions, not a capacity plan (`BENCHMARKS.md`).

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use reqwest::Client;

/// The read-only endpoints exercised, in turn, by each worker.
const PATHS: [&str; 4] = [
    "/api/plans?limit=10",
    "/api/plans?limit=50",
    "/api/milestones",
    "/api/issues",
];

/// A setting from the environment, or its default.
fn setting<T: std::str::FromStr>(name: &str, default: T) -> T {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// One request's outcome: the endpoint, the status (0 for a transport error) and the latency.
struct Sample {
    path: &'static str,
    status: u16,
    latency: Duration,
    /// For a transport error, what it was.
    error: Option<String>,
}

/// The value at `per_mille` (0 to 1000) of sorted durations, by nearest rank: the smallest value with at least that
/// share of the values at or below it.
fn percentile(sorted: &[Duration], per_mille: usize) -> Duration {
    if sorted.is_empty() {
        return Duration::ZERO;
    }
    let rank = (sorted.len() * per_mille).div_ceil(1000);
    sorted[rank.saturating_sub(1).min(sorted.len() - 1)]
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// One worker: requests back to back until `deadline`, cycling through `PATHS`.
async fn worker(client: Client, base: String, first: usize, deadline: Instant) -> Vec<Sample> {
    let mut samples = Vec::new();
    let mut i = first;
    while Instant::now() < deadline {
        let path = PATHS[i % PATHS.len()];
        i += 1;
        let started = Instant::now();
        let (status, error) = match client.get(format!("{base}{path}")).send().await {
            Ok(response) => {
                let status = response.status().as_u16();
                // Read the whole body: the latency includes it, and the connection is freed.
                match response.bytes().await {
                    Ok(_) => (status, None),
                    Err(e) => (0, Some(format!("{e:?}"))),
                }
            }
            Err(e) => (0, Some(format!("{e:?}"))),
        };
        samples.push(Sample {
            path,
            status,
            latency: started.elapsed(),
            error,
        });
    }
    samples
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a running service with login and rate limiting off; see the module docs"]
async fn read_endpoints_under_concurrent_load() {
    let base = setting("PPM_LOAD_TEST_BASE", "http://127.0.0.1:1241".to_string());
    let base = base.trim_end_matches('/').to_string();
    let seconds: u64 = setting("PPM_LOAD_TEST_SECONDS", 5);
    let concurrency: usize = setting("PPM_LOAD_TEST_CONCURRENCY", 8);

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .expect("an HTTP client");

    let health = client
        .get(format!("{base}/_health/ready"))
        .send()
        .await
        .unwrap_or_else(|e| panic!("no service at {base}: {e} (start one; see the module docs)"));
    assert!(
        health.status().is_success(),
        "{base}/_health/ready answered {}",
        health.status()
    );

    let started = Instant::now();
    let deadline = started + Duration::from_secs(seconds);
    let workers: Vec<_> = (0..concurrency)
        .map(|n| tokio::spawn(worker(client.clone(), base.clone(), n, deadline)))
        .collect();
    let mut samples = Vec::new();
    for w in workers {
        samples.extend(w.await.expect("a worker"));
    }
    let elapsed = started.elapsed();

    let total = samples.len();
    let ok = samples
        .iter()
        .filter(|s| (200..300).contains(&s.status))
        .count();
    let mut by_status: BTreeMap<u16, usize> = BTreeMap::new();
    let mut by_path: BTreeMap<&str, Vec<Duration>> = BTreeMap::new();
    for s in &samples {
        *by_status.entry(s.status).or_default() += 1;
        by_path.entry(s.path).or_default().push(s.latency);
    }

    let rate = f64::from(u32::try_from(total).unwrap_or(u32::MAX)) / elapsed.as_secs_f64();
    println!("\n{base}: {concurrency} workers for {seconds}s: {total} requests, {rate:.0}/s");
    println!("status counts (0 = transport error): {by_status:?}");
    println!("| endpoint | requests | p50 ms | p95 ms | p99 ms | max ms |");
    println!("|---|---|---|---|---|---|");
    for (path, latencies) in &mut by_path {
        latencies.sort_unstable();
        let max = latencies.last().copied().unwrap_or_default();
        println!(
            "| `{path}` | {} | {:.1} | {:.1} | {:.1} | {:.1} |",
            latencies.len(),
            ms(percentile(latencies, 500)),
            ms(percentile(latencies, 950)),
            ms(percentile(latencies, 990)),
            ms(max),
        );
    }

    let errors: BTreeMap<&str, usize> =
        samples
            .iter()
            .filter_map(|s| s.error.as_deref())
            .fold(BTreeMap::new(), |mut m, e| {
                *m.entry(e).or_default() += 1;
                m
            });
    for (error, n) in &errors {
        println!("transport error ({n}x): {error}");
    }

    assert!(ok > 0, "no request succeeded: {by_status:?}");
    let broken: usize = by_status
        .iter()
        .filter(|(status, _)| **status == 0 || **status >= 500)
        .map(|(_, n)| n)
        .sum();
    assert_eq!(broken, 0, "transport errors or 5xx answers: {by_status:?}");
}

#[test]
fn percentile_is_nearest_rank() {
    let d: Vec<Duration> = (1..=10).map(Duration::from_millis).collect();
    assert_eq!(percentile(&d, 500), Duration::from_millis(5));
    assert_eq!(percentile(&d, 950), Duration::from_millis(10));
    assert_eq!(percentile(&d, 100), Duration::from_millis(1));
    assert_eq!(percentile(&[], 500), Duration::ZERO);
}

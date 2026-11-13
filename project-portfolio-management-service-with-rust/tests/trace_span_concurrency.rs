//! The per-request tracing middleware (`observability::trace_mw`) under concurrent
//! requests on a multi-threaded runtime, with a `tracing` registry installed as the
//! global subscriber, as in the running service.
//!
//! A span entered with `Span::enter` and held across an `.await` stays "entered"
//! on whichever worker thread the task was on, so concurrent requests stack each
//! other's spans and the registry's reference counts go wrong. The registry then
//! panics inside the connection task, and the client sees the connection drop
//! (`IncompleteMessage`). The load test found this on 13 November 2026: 30 of about
//! 30,000 requests. This test sends enough concurrent requests through the
//! middleware to show it, and needs no database.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use axum::{Router, middleware, routing::get};
use tracing_subscriber::layer::SubscriberExt as _;

use project_portfolio_management::observability::trace_mw;

async fn handler() -> &'static str {
    // Yield so the request's task can move between worker threads, as a database call does.
    tokio::task::yield_now().await;
    tokio::time::sleep(Duration::from_micros(200)).await;
    tokio::task::yield_now().await;
    "ok"
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_requests_through_the_trace_middleware_all_complete() {
    // A registry with a layer that records span names, which is what the service's JSON log layer does.
    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(std::io::sink),
    );
    tracing::subscriber::set_global_default(subscriber)
        .expect("the only subscriber in this binary");

    let app = Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(trace_mw));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a port");
    let address = listener.local_addr().expect("an address");
    tokio::spawn(async move { axum::serve(listener, app).await.expect("serve") });

    let client = reqwest::Client::new();
    let failed = Arc::new(AtomicUsize::new(0));
    let workers: Vec<_> = (0..16)
        .map(|_| {
            let client = client.clone();
            let failed = failed.clone();
            tokio::spawn(async move {
                for _ in 0..300 {
                    let ok = match client.get(format!("http://{address}/")).send().await {
                        Ok(response) => {
                            response.status().is_success() && response.bytes().await.is_ok()
                        }
                        Err(_) => false,
                    };
                    if !ok {
                        failed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            })
        })
        .collect();
    for w in workers {
        w.await.expect("a worker");
    }
    assert_eq!(
        failed.load(Ordering::Relaxed),
        0,
        "requests whose connection dropped"
    );
}

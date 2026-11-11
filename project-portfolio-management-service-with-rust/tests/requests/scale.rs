//! T-28d: capacity-at-scale regression guard. Seeds a materially large
//! roster (60 plans, allocations across 40 shared people) and proves
//! `GET /capacity`, `GET /capacity/utilization`, and `GET /at-a-glance`
//! each run a **bounded** number of SQL statements — counted exactly by
//! [`project_portfolio_management::query_count`], which sees every
//! statement the app runs, `SeaORM`'s (its metric callback) and sqlx's
//! (`db::Counted`) alike, not timed (timing is noisy under load; a
//! statement count is exact and deterministic).
//!
//! It used to count `SeaORM`'s driver tracing spans only, which went
//! blind as the routes' statements moved to sqlx (SQ-1): a route whose
//! last `SeaORM` statement moved counted zero.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use std::future::Future;

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

/// Run `f`, returning its result plus the number of statements (`SeaORM`
/// and sqlx) the app ran meanwhile. The test is `#[serial]`, so nothing
/// else in this process runs statements while it is counted.
async fn count_queries<F, Fut, T>(f: F) -> (T, u64)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    let before = project_portfolio_management::query_count::total();
    let result = f().await;
    let after = project_portfolio_management::query_count::total();
    (result, after - before)
}

/// Create one plan and one allocation for it; returns the plan pid.
async fn seed_one(request: &axum_test::TestServer, name: &str, person: &str) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": "Project", "name": name }))
        .await
        .json();
    let pid = created["pid"].as_str().expect("pid").to_string();
    request
        .post(&format!("/api/plans/{pid}/allocations"))
        .json(&json!({ "person_ref": person, "percent": 20 }))
        .await
        .assert_status_ok();
    pid
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
// T-28d: query count for /capacity, /capacity/utilization, and
// /at-a-glance is identical at a small roster and at the full 60-plan
// / 40-person scale — proof the request-path queries are bulk reads,
// not a per-row fan-out that would grow with plan count.
async fn capacity_views_do_not_fan_out_with_scale() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let people: Vec<String> = (0..40)
            .map(|_| format!("worker:{}", uuid::Uuid::new_v4()))
            .collect();

        // A small roster first (5 plans): the baseline query count.
        let mut plan_pids = Vec::new();
        for i in 0..5 {
            plan_pids
                .push(seed_one(&request, &format!("Scale {i}"), &people[i % people.len()]).await);
        }

        let (_, capacity_small) =
            count_queries(|| async { request.get("/api/capacity").await }).await;
        let (_, utilization_small) =
            count_queries(|| async { request.get("/api/capacity/utilization?by=person").await })
                .await;
        let (_, glance_small) =
            count_queries(|| async { request.get("/api/at-a-glance").await }).await;
        assert!(
            capacity_small > 0,
            "the layer observed real /capacity queries"
        );
        assert!(
            utilization_small > 0,
            "the layer observed real /capacity/utilization queries"
        );
        assert!(
            glance_small > 0,
            "the layer observed real /at-a-glance queries"
        );

        // Scale up to 60 plans across the same 40 shared people.
        for i in 5..60 {
            plan_pids
                .push(seed_one(&request, &format!("Scale {i}"), &people[i % people.len()]).await);
        }
        assert_eq!(plan_pids.len(), 60);

        let (capacity_body, capacity_large) =
            count_queries(|| async { request.get("/api/capacity").await }).await;
        let (_, utilization_large) =
            count_queries(|| async { request.get("/api/capacity/utilization?by=person").await })
                .await;
        let (_, glance_large) =
            count_queries(|| async { request.get("/api/at-a-glance").await }).await;

        let capacity_json: Value = capacity_body.json();
        assert_eq!(
            capacity_json["people"].as_array().expect("people").len(),
            40,
            "all 40 shared people appear, each over multiple plans"
        );
        assert_eq!(
            capacity_small, capacity_large,
            "GET /capacity's query count must not grow with plan count \
             (was {capacity_small} at 5 plans, {capacity_large} at 60)"
        );
        assert_eq!(
            utilization_small, utilization_large,
            "GET /capacity/utilization's query count must not grow with plan count \
             (was {utilization_small} at 5 plans, {utilization_large} at 60)"
        );
        assert_eq!(
            glance_small, glance_large,
            "GET /at-a-glance's query count must not grow with plan count \
             (was {glance_small} at 5 plans, {glance_large} at 60)"
        );
    })
    .await;
}

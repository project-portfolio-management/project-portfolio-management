//! CQ-6: when the privacy guard cannot tell whether a plan is private, it refuses with `503`
//! instead of letting the request through to the handler, and the failure is counted. The lookup
//! is made to fail by renaming its table for the length of one request.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::metrics::Metrics;
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_failed_privacy_lookup_refuses_and_is_counted() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Lookup failure" }))
            .await
            .json();
        let pid = plan["pid"].as_str().unwrap().to_string();
        request
            .get(&format!("/api/plans/{pid}"))
            .await
            .assert_status_ok();

        let failed = || Metrics::global().visibility_lookup_failed_total.get();
        let before = failed();
        ctx.db
            .execute_unprepared(
                "ALTER TABLE plan_access_settings RENAME TO plan_access_settings_cq6",
            )
            .await
            .expect("rename");
        let res = request.get(&format!("/api/plans/{pid}")).await;
        ctx.db
            .execute_unprepared(
                "ALTER TABLE plan_access_settings_cq6 RENAME TO plan_access_settings",
            )
            .await
            .expect("rename back");

        assert_eq!(res.status_code(), 503, "{}", res.text());
        assert_eq!(res.json::<Value>()["error"], "unavailable");
        assert!(failed() - before >= 1.0, "the failure is counted");
        request
            .get(&format!("/api/plans/{pid}"))
            .await
            .assert_status_ok();
    })
    .await;
}

/// An aggregate that filters through `hidden_plans` used to answer `200` with fewer rows when the
/// lookup failed. It now answers `503`; a write is not turned into one.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_failed_lookup_under_a_list_answers_503() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Aggregate failure" }))
            .await
            .assert_status_success();
        request.get("/api/plans").await.assert_status_ok();

        let before = Metrics::global().visibility_lookup_failed_total.get();
        ctx.db
            .execute_unprepared(
                "ALTER TABLE plan_access_settings RENAME TO plan_access_settings_cq6",
            )
            .await
            .expect("rename");
        let res = request.get("/api/plans").await;
        ctx.db
            .execute_unprepared(
                "ALTER TABLE plan_access_settings_cq6 RENAME TO plan_access_settings",
            )
            .await
            .expect("rename back");

        assert_eq!(res.status_code(), 503, "{}", res.text());
        assert_eq!(res.json::<Value>()["error"], "unavailable");
        assert!(Metrics::global().visibility_lookup_failed_total.get() - before >= 1.0);
        request.get("/api/plans").await.assert_status_ok();
    })
    .await;
}

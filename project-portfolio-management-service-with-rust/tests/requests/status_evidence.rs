//! Evidence-backed status updates (BR-2): claims cite sources that are checked on save and resolved on read, and the
//! numbers they cite are counted as published.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use chrono::Days;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn claims_cite_checked_sources_and_the_numbers_are_counted_as_published() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let today = project_portfolio_management::tz::deployment_today();
        let on = |d: i64| {
            if d < 0 {
                today - Days::new(d.unsigned_abs())
            } else {
                today + Days::new(d.unsigned_abs())
            }
            .to_string()
        };
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Evidence programme" }))
            .await
            .json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let other: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Elsewhere programme" }))
            .await
            .json();
        let other = other["pid"].as_str().unwrap().to_string();
        let make = async |plan: &str, body: Value| -> String {
            let t: Value = request
                .post(&format!("/api/plans/{plan}/tasks"))
                .json(&body)
                .await
                .json();
            t["pid"].as_str().unwrap().to_string()
        };
        // one overdue, one due today (not overdue), one due in three days and blocked, one done long ago, one undated
        let late = make(&plan, json!({ "title": "Late survey", "due_date": on(-2) })).await;
        make(&plan, json!({ "title": "Due today", "due_date": on(0) })).await;
        make(
            &plan,
            json!({ "title": "Blocked pour", "due_date": on(3), "status": "blocked" }),
        )
        .await;
        make(
            &plan,
            json!({ "title": "Finished", "due_date": on(-30), "status": "done" }),
        )
        .await;
        make(&plan, json!({ "title": "Undated" })).await;
        let foreign = make(&other, json!({ "title": "Not ours" })).await;

        // ---- the published definitions
        let defs: Value = request.get("/api/status-metrics").await.json();
        let keys: Vec<&str> = defs["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m["key"].as_str().unwrap())
            .collect();
        assert_eq!(keys, ["overdue", "due_soon", "blocked", "no_due_date"]);

        // ---- a false reference is refused whole; so are a record of another plan, an unknown metric and a bad kind
        let post = |claims: Value| {
            request
                .post(&format!("/api/plans/{plan}/status-updates"))
                .json(&json!({ "status": "at_risk", "narrative": "Slipping.", "claims": claims }))
        };
        for bad in [
            json!([{ "text": "x", "sources": ["task:3f2c6b7e-0000-4000-8000-0000000000ff"] }]),
            json!([{ "text": "x", "sources": [format!("task:{foreign}")] }]),
            json!([{ "text": "x", "sources": ["metric:score"] }]),
            json!([{ "text": "x", "sources": ["plan:3f2c6b7e-0000-4000-8000-0000000000ff"] }]),
            json!([{ "text": " ", "sources": [] }]),
        ] {
            assert_eq!(post(bad.clone()).await.status_code(), 422, "{bad}");
        }
        let none: Value = request
            .get(&format!("/api/plans/{plan}/status-updates"))
            .await
            .json();
        assert!(
            none.as_array().unwrap().is_empty(),
            "a refused update is not stored"
        );

        // ---- a good one reads back with titles and the numbers
        let posted: Value = post(json!([
            { "text": "The survey is late", "sources": [format!("task:{late}"), "metric:overdue"] },
            { "text": "Dates are missing", "sources": ["metric:no_due_date"] },
            { "text": "Blocked work", "sources": ["metric:blocked", "metric:due_soon"] }
        ]))
        .await
        .json();
        let c = &posted["claims"];
        assert_eq!(c[0]["sources"][0]["title"], "Late survey");
        assert_eq!(
            (
                c[0]["sources"][1]["value"].as_u64(),
                c[0]["sources"][1]["known"].as_bool()
            ),
            (Some(1), Some(true)),
            "due today is not overdue"
        );
        assert_eq!(c[1]["sources"][0]["value"], 1);
        assert_eq!(c[2]["sources"][0]["value"], 1);
        assert_eq!(
            c[2]["sources"][1]["value"], 2,
            "due today and in three days; the done task is not counted"
        );
        assert!(
            c[0]["sources"][1]["definition"]
                .as_str()
                .unwrap()
                .contains("due today is not overdue")
                || c[0]["sources"][1]["definition"]
                    .as_str()
                    .unwrap()
                    .contains("A task due today is not overdue")
        );

        // ---- an update with no claims still posts, and the stored claims hold ids and never titles
        let plain: Value = request
            .post(&format!("/api/plans/{plan}/status-updates"))
            .json(&json!({ "status": "on_track", "narrative": "Fine." }))
            .await
            .json();
        assert!(plain["claims"].as_array().unwrap().is_empty());

        // ---- a source that is later deleted is unavailable, with no title
        request
            .delete(&format!("/api/plans/{plan}/tasks/{late}"))
            .await
            .assert_status_ok();
        let history: Value = request
            .get(&format!("/api/plans/{plan}/status-updates"))
            .await
            .json();
        let first = history
            .as_array()
            .unwrap()
            .iter()
            .find(|u| u["narrative"] == "Slipping.")
            .unwrap();
        let gone = &first["claims"][0]["sources"][0];
        assert_eq!(gone["state"], "unavailable");
        assert!(gone.get("title").is_none() && gone.get("pid").is_none());
        assert_eq!(
            first["claims"][0]["sources"][1]["value"], 0,
            "the count now has no overdue task"
        );
        let latest: Value = request
            .get(&format!("/api/plans/{plan}/status"))
            .await
            .json();
        assert_eq!(latest["latest"]["narrative"], "Fine.");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_plan_whose_open_tasks_have_no_dates_reports_overdue_as_unknown_not_zero() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Undated programme" })).await.json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/plans/{plan}/tasks")).json(&json!({ "title": "Somewhen" })).await.assert_status_ok();
        let posted: Value = request
            .post(&format!("/api/plans/{plan}/status-updates"))
            .json(&json!({ "status": "on_track", "narrative": "Unclear.", "claims": [{ "text": "Nothing overdue", "sources": ["metric:overdue"] }] }))
            .await
            .json();
        let s = &posted["claims"][0]["sources"][0];
        assert!(s["value"].is_null());
        assert_eq!(s["known"], false);
    })
    .await;
}

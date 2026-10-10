//! Issues register request tests (IR-1): raising, listing most serious first,
//! the closing rule, `resolved_on` handled by the service, a plan's issues stay
//! in that plan, and removal is soft.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer, name: &str) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": "Project", "name": name }))
        .await
        .json();
    created["pid"].as_str().expect("pid").to_string()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn issues_are_raised_ranked_closed_and_kept_in_their_plan() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Apollo").await;
        let other = plan(&request, "Other").await;

        // Raise three; defaults apply.
        let minor: Value = request
            .post(&format!("/api/plans/{p}/issues"))
            .json(
                &json!({ "title": "Typo in the report", "severity": "low", "kind": "bug",
                           "raised_on": "2026-09-01" }),
            )
            .await
            .json();
        assert_eq!(minor["status"], "open");
        assert_eq!(minor["resolved_on"], Value::Null);
        let big: Value = request
            .post(&format!("/api/plans/{p}/issues"))
            .json(
                &json!({ "title": "Build server is down", "severity": "critical",
                           "kind": "blocker", "owner_ref": "person:ada",
                           "raised_on": "2026-09-10", "due_date": "2026-09-01" }),
            )
            .await
            .json();
        // A due date before the raised date is refused, so that one did not exist.
        assert!(big.get("pid").is_none(), "{big}");
        let big: Value = request
            .post(&format!("/api/plans/{p}/issues"))
            .json(
                &json!({ "title": "Build server is down", "severity": "critical",
                           "kind": "blocker", "owner_ref": "person:ada",
                           "raised_on": "2026-09-10", "due_date": "2026-09-12" }),
            )
            .await
            .json();
        let big_pid = big["pid"].as_str().expect("pid").to_string();
        assert_eq!(big["overdue"], true);
        assert!(big["age_calendar_days"].as_i64().unwrap() > 0);

        // Most serious first.
        let list: Value = request.get(&format!("/api/plans/{p}/issues")).await.json();
        assert_eq!(list["total"], 2);
        assert_eq!(list["issues"][0]["title"], "Build server is down");
        assert_eq!(list["overdue"], 1);
        let filtered: Value = request
            .get(&format!("/api/plans/{p}/issues?severity=low"))
            .await
            .json();
        assert_eq!(filtered["total"], 1);
        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/issues?status=nonsense"))
                .await
                .status_code(),
            422
        );

        // Closing needs a resolution.
        let refused = request
            .put(&format!("/api/plans/{p}/issues/{big_pid}"))
            .json(
                &json!({ "title": "Build server is down", "severity": "critical",
                           "kind": "blocker", "status": "closed",
                           "raised_on": "2026-09-10", "due_date": "2026-09-12" }),
            )
            .await;
        assert_eq!(refused.status_code(), 422);
        let closed: Value = request
            .put(&format!("/api/plans/{p}/issues/{big_pid}"))
            .json(
                &json!({ "title": "Build server is down", "severity": "critical",
                           "kind": "blocker", "status": "closed",
                           "resolution": "Replaced the disk",
                           "raised_on": "2026-09-10", "due_date": "2026-09-12" }),
            )
            .await
            .json();
        assert_eq!(closed["status"], "closed");
        assert!(closed["resolved_on"].is_string());
        assert_eq!(closed["overdue"], false);

        // Reopening clears the resolved date.
        let reopened: Value = request
            .put(&format!("/api/plans/{p}/issues/{big_pid}"))
            .json(
                &json!({ "title": "Build server is down", "severity": "critical",
                           "kind": "blocker", "status": "open",
                           "raised_on": "2026-09-10" }),
            )
            .await
            .json();
        assert_eq!(reopened["resolved_on"], Value::Null);

        // Another plan cannot see or reach it.
        let others: Value = request
            .get(&format!("/api/plans/{other}/issues"))
            .await
            .json();
        assert_eq!(others["total"], 0);
        assert_eq!(
            request
                .get(&format!("/api/plans/{other}/issues/{big_pid}"))
                .await
                .status_code(),
            404
        );

        // Removal is soft: gone from the list and from direct reads.
        request
            .delete(&format!("/api/plans/{p}/issues/{big_pid}"))
            .await
            .assert_status_ok();
        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/issues/{big_pid}"))
                .await
                .status_code(),
            404
        );
        let after: Value = request.get(&format!("/api/plans/{p}/issues")).await.json();
        assert_eq!(after["total"], 1);

        // An unknown plan is 404.
        assert_eq!(
            request
                .get("/api/plans/00000000-0000-4000-8000-000000000000/issues")
                .await
                .status_code(),
            404
        );
    })
    .await;
}

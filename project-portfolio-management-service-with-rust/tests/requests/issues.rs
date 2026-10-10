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

        // The register spans plans, most serious first, with a summary.
        request
            .post(&format!("/api/plans/{other}/issues"))
            .json(
                &json!({ "title": "Licence expires", "severity": "high", "kind": "concern",
                           "raised_on": "2026-08-01" }),
            )
            .await
            .assert_status_ok();
        let all: Value = request.get("/api/issues").await.json();
        assert!(all["matching"].as_i64().unwrap() >= 3);
        let titles: Vec<&str> = all["issues"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i["title"].as_str().unwrap())
            .collect();
        let build = titles
            .iter()
            .position(|t| *t == "Build server is down")
            .unwrap();
        let licence = titles.iter().position(|t| *t == "Licence expires").unwrap();
        let typo = titles
            .iter()
            .position(|t| *t == "Typo in the report")
            .unwrap();
        assert!(build < licence && licence < typo, "{titles:?}");
        assert!(all["summary"]["by_severity"]["critical"].as_i64().unwrap() >= 1);
        let old: Value = request
            .get("/api/issues?min_age_days=60&open=true")
            .await
            .json();
        assert!(
            old["issues"]
                .as_array()
                .unwrap()
                .iter()
                .all(|i| i["age_calendar_days"].as_i64().unwrap() >= 60)
        );
        let scoped: Value = request
            .get(&format!("/api/issues?plan={other}"))
            .await
            .json();
        assert_eq!(scoped["matching"], 1);
        assert_eq!(
            request.get("/api/issues?severity=huge").await.status_code(),
            422
        );
        assert_eq!(
            request
                .get("/api/issues?min_age_days=-1")
                .await
                .status_code(),
            422
        );

        // Another plan cannot see or reach it.
        let others: Value = request
            .get(&format!("/api/plans/{other}/issues"))
            .await
            .json();
        assert_eq!(others["total"], 1);
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

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn an_issue_can_be_raised_from_a_risk_or_a_blocked_task_once() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Apollo").await;

        // From a risk: severity follows exposure (4 x 5 = 20 is critical); the risk is untouched.
        let risk: Value = request
            .post(&format!("/api/plans/{p}/risks"))
            .json(
                &json!({ "title": "Supplier fails", "description": "Single source",
                           "probability": 4, "impact": 5, "owner_ref": "person:3f2c6b7e-0000-4000-8000-00000000000a" }),
            )
            .await
            .json();
        let risk_pid = risk["pid"].as_str().unwrap_or_else(|| panic!("{risk}")).to_string();
        let raised: Value = request
            .post(&format!("/api/plans/{p}/issues/from-risk/{risk_pid}"))
            .await
            .json();
        assert_eq!(raised["kind"], "risk");
        assert_eq!(raised["severity"], "critical");
        assert_eq!(raised["title"], "Supplier fails");
        assert_eq!(raised["owner_ref"], "person:3f2c6b7e-0000-4000-8000-00000000000a");
        assert_eq!(raised["source_risk_pid"], risk_pid);
        let risks: Value = request.get(&format!("/api/plans/{p}/risks")).await.json();
        assert_eq!(risks[0]["status"], "open", "the risk is not changed");

        // A second attempt while the first is open is a conflict.
        assert_eq!(
            request
                .post(&format!("/api/plans/{p}/issues/from-risk/{risk_pid}"))
                .await
                .status_code(),
            409
        );
        // Unknown risk is 404.
        assert_eq!(
            request
                .post(&format!(
                    "/api/plans/{p}/issues/from-risk/00000000-0000-4000-8000-000000000000"
                ))
                .await
                .status_code(),
            404
        );

        // From a task: only a blocked one.
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Wire the gateway", "assignee_ref": "person:3f2c6b7e-0000-4000-8000-00000000000b" }))
            .await
            .json();
        let task_pid = task["pid"].as_str().expect("task pid").to_string();
        assert_eq!(
            request
                .post(&format!("/api/plans/{p}/issues/from-task/{task_pid}"))
                .await
                .status_code(),
            422
        );
        request
            .patch(&format!("/api/plans/{p}/tasks/{task_pid}"))
            .json(&json!({ "status": "blocked" }))
            .await
            .assert_status_ok();
        let blocker: Value = request
            .post(&format!("/api/plans/{p}/issues/from-task/{task_pid}"))
            .await
            .json();
        assert_eq!(blocker["kind"], "blocker");
        assert_eq!(blocker["title"], "Blocked: Wire the gateway");
        assert_eq!(blocker["owner_ref"], "person:3f2c6b7e-0000-4000-8000-00000000000b");
        assert_eq!(blocker["source_task_pid"], task_pid);
        assert_eq!(
            request
                .post(&format!("/api/plans/{p}/issues/from-task/{task_pid}"))
                .await
                .status_code(),
            409
        );

        // Once the first is closed, a new one may be raised from the same risk.
        let first = raised["pid"].as_str().unwrap();
        request
            .put(&format!("/api/plans/{p}/issues/{first}"))
            .json(
                &json!({ "title": "Supplier fails", "kind": "risk", "severity": "critical",
                           "status": "closed", "resolution": "Second supplier signed",
                           "raised_on": raised["raised_on"] }),
            )
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/issues/from-risk/{risk_pid}"))
            .await
            .assert_status_ok();
    })
    .await;
}

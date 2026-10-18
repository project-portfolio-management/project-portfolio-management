//! Task component request tests (PL-11): a task's component must belong to
//! its own plan, deleting a component in use is refused, the summary always
//! reconciles (every component reported, plus an `unassigned` bucket), and
//! the task list filters by component.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
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
async fn components_group_tasks_and_never_orphan_them() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Apollo").await;
        let other = plan(&request, "Other").await;

        let api: Value = request
            .post(&format!("/api/plans/{p}/components"))
            .json(&json!({ "name": "API" }))
            .await
            .json();
        let api = api["pid"].as_str().expect("component pid").to_string();
        request
            .post(&format!("/api/plans/{p}/components"))
            .json(&json!({ "name": "UI" }))
            .await
            .assert_status_ok();
        // Names are unique within a plan, case-insensitively.
        assert_eq!(
            request
                .post(&format!("/api/plans/{p}/components"))
                .json(&json!({ "name": "api" }))
                .await
                .status_code(),
            409
        );
        let foreign: Value = request
            .post(&format!("/api/plans/{other}/components"))
            .json(&json!({ "name": "Foreign" }))
            .await
            .json();
        let foreign = foreign["pid"].as_str().expect("pid").to_string();

        // A task's component must belong to the task's own plan.
        assert_eq!(
            request
                .post(&format!("/api/plans/{p}/tasks"))
                .json(&json!({ "title": "x", "component_pid": foreign }))
                .await
                .status_code(),
            422
        );
        for (title, c) in [("a", Some(&api)), ("b", Some(&api)), ("c", None)] {
            let mut body = json!({ "title": title });
            if let Some(c) = c {
                body["component_pid"] = json!(c);
            }
            request
                .post(&format!("/api/plans/{p}/tasks"))
                .json(&body)
                .await
                .assert_status_ok();
        }

        // Summary: every component reported (UI empty), unassigned last, totals reconcile.
        let sum: Value = request
            .get(&format!("/api/plans/{p}/components/summary"))
            .await
            .json();
        let buckets = sum["buckets"].as_array().expect("buckets");
        assert_eq!(buckets.len(), 3);
        assert_eq!(buckets[0]["total"], 2);
        assert_eq!(buckets[1]["total"], 0, "UI reported empty");
        assert_eq!(buckets[2]["name"], "unassigned");
        assert_eq!(buckets[2]["total"], 1);
        let total: i64 = buckets.iter().map(|b| b["total"].as_i64().unwrap()).sum();
        assert_eq!(total, sum["total"].as_i64().unwrap());

        // Filtering.
        let by: Value = request
            .get(&format!("/api/plans/{p}/tasks?component={api}"))
            .await
            .json();
        assert_eq!(by["tasks"].as_array().unwrap().len(), 2);
        let un: Value = request
            .get(&format!("/api/plans/{p}/tasks?component=unassigned"))
            .await
            .json();
        assert_eq!(un["tasks"].as_array().unwrap().len(), 1);
        assert!(
            request
                .get(&format!("/api/plans/{p}/tasks?component=nope"))
                .await
                .status_code()
                .is_client_error()
        );

        // Deleting a component in use is refused; an empty one can go.
        assert_eq!(
            request
                .delete(&format!("/api/plans/{p}/components/{api}"))
                .await
                .status_code(),
            409
        );
        let list: Value = request
            .get(&format!("/api/plans/{p}/components"))
            .await
            .json();
        let ui = list
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["name"] == "UI")
            .expect("UI")["pid"]
            .as_str()
            .unwrap()
            .to_string();
        request
            .delete(&format!("/api/plans/{p}/components/{ui}"))
            .await
            .assert_status_ok();
    })
    .await;
}

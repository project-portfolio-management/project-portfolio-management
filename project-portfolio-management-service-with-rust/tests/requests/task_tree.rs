//! Subtask roll-up request tests (RM-5): parent validation (same plan, no
//! cycle) and the derived view (latest due, estimate-weighted progress,
//! unestimated children named, spent time summed, `null` + reason when
//! nothing can be derived).
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

async fn task(request: &axum_test::TestServer, p: &str, body: Value) -> (u16, String) {
    let r = request
        .post(&format!("/api/plans/{p}/tasks"))
        .json(&body)
        .await;
    let code = r.status_code().as_u16();
    let v: Value = r.json();
    (code, v["pid"].as_str().unwrap_or_default().to_string())
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_parent_is_derived_from_its_subtasks_and_never_stored() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Apollo").await;
        let other = plan(&request, "Other").await;
        let (_, parent) = task(&request, &p, json!({ "title": "Parent" })).await;

        // A parent with no children: null + reason, not zero.
        let empty: Value = request
            .get(&format!("/api/plans/{p}/tasks/{parent}/rollup"))
            .await
            .json();
        assert_eq!(empty["rollup"]["progress_bp"]["reason"], "no_children");
        assert!(empty["rollup"]["progress_bp"]["value"].is_null());

        // Children: estimated + done, estimated + open, unestimated.
        let (_, c1) = task(&request, &p, json!({ "title": "C1", "parent_task_pid": parent, "points": 3, "due_date": "2026-10-03" })).await;
        let (_, c2) = task(&request, &p, json!({ "title": "C2", "parent_task_pid": parent, "points": 1, "due_date": "2026-10-09" })).await;
        let (_, c3) = task(&request, &p, json!({ "title": "C3", "parent_task_pid": parent })).await;
        request
            .patch(&format!("/api/plans/{p}/tasks/{c1}"))
            .json(&json!({ "status": "done" }))
            .await
            .assert_status_ok();
        for (task_pid, minutes) in [(&c1, 60), (&c3, 15)] {
            request
                .post(&format!("/api/plans/{p}/time-entries"))
                .json(&json!({
                    "task_pid": task_pid,
                    "actor_ref": "person:aaaaaaaa-0000-0000-0000-000000000001",
                    "spent_on": "2026-10-01", "minutes": minutes
                }))
                .await
                .assert_status_ok();
        }

        let r: Value = request
            .get(&format!("/api/plans/{p}/tasks/{parent}/rollup"))
            .await
            .json();
        let ru = &r["rollup"];
        assert_eq!(ru["descendants"], 3);
        assert_eq!(ru["done"], 1);
        assert_eq!(ru["due"]["value"], "2026-10-09", "latest descendant due");
        assert_eq!(ru["estimate_points"]["value"], 4);
        assert_eq!(ru["progress_bp"]["value"], 7_500, "3 of 4 estimated points are done");
        assert_eq!(ru["excluded_unestimated"], json!([c3]), "named, not counted as zero");
        assert_eq!(ru["spent_minutes"], 75);
        let _ = c2;

        // Nesting: a grandchild rolls up into the parent too.
        let (_, g) = task(&request, &p, json!({ "title": "G", "parent_task_pid": c2, "points": 4 })).await;
        let r2: Value = request
            .get(&format!("/api/plans/{p}/tasks/{parent}/rollup"))
            .await
            .json();
        assert_eq!(r2["rollup"]["descendants"], 4);
        assert_eq!(r2["rollup"]["estimate_points"]["value"], 8);

        // Parent validation: another plan's task, an unknown task, a cycle.
        let (_, foreign) = task(&request, &other, json!({ "title": "Foreign" })).await;
        let (code, _) = task(&request, &p, json!({ "title": "x", "parent_task_pid": foreign })).await;
        assert_eq!(code, 422, "the parent must be in the same plan");
        let put = |t: String, parent: String| {
            let (request, p) = (&request, p.clone());
            async move {
                request
                    .put(&format!("/api/plans/{p}/tasks/{t}"))
                    .json(&json!({ "title": "x", "parent_task_pid": parent }))
                    .await
                    .status_code()
                    .as_u16()
            }
        };
        assert_eq!(put(parent.clone(), g.clone()).await, 422, "a parent under its own grandchild is a cycle");
        assert_eq!(put(parent.clone(), parent.clone()).await, 422, "its own parent");
    })
    .await;
}

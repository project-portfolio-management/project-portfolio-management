//! Bulk task request tests (RM-9): one result per requested task, counts that
//! reconcile, `status` refused as a bulk field, a precedes relation honoured,
//! copy (fresh task, initial status, `copied_from`), and move (target
//! workflow re-validation with the mapping stated, subtasks refused).
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

async fn task(request: &axum_test::TestServer, p: &str, body: Value) -> String {
    let t: Value = request
        .post(&format!("/api/plans/{p}/tasks"))
        .json(&body)
        .await
        .json();
    t["pid"].as_str().expect("task pid").to_string()
}

fn counts(v: &Value) -> (i64, i64, i64) {
    (
        v["requested"].as_i64().unwrap(),
        v["ok"].as_i64().unwrap(),
        v["refused"].as_i64().unwrap(),
    )
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn bulk_edit_copy_and_move_report_every_item_and_reconcile() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let a = plan(&request, "A").await;
        let b = plan(&request, "B").await;
        // B has its own workflow, which lacks `todo` and `in_review`.
        let state = |key: &str, cat: &str, initial: bool, terminal: bool| {
            json!({ "key": key, "label": key, "category": cat, "is_initial": initial, "is_terminal": terminal })
        };
        request
            .post("/api/workflows")
            .json(&json!({ "name": "B board", "applies_to": "task", "plan_pid": b,
                "states": [state("backlog", "todo", true, false), state("active", "active", false, false), state("finished", "done", false, true)] }))
            .await
            .assert_status_ok();

        let t1 = task(&request, &a, json!({ "title": "T1", "due_date": "2026-10-05", "assignee_ref": "person:aaaaaaaa-0000-0000-0000-000000000001" })).await;
        let t2 = task(&request, &a, json!({ "title": "T2" })).await;
        let t3 = task(&request, &a, json!({ "title": "T3", "due_date": "2026-10-06" })).await;
        request
            .patch(&format!("/api/plans/{a}/tasks/{t2}"))
            .json(&json!({ "status": "in_review" }))
            .await
            .assert_status_ok();
        let comp: Value = request
            .post(&format!("/api/plans/{a}/components"))
            .json(&json!({ "name": "API" }))
            .await
            .json();
        let comp = comp["pid"].as_str().expect("component pid").to_string();
        let unknown = "3f2c6b7e-0000-4000-8000-000000000bad";

        // ---- edit: one result per id, an unknown id never aborts the rest.
        let edit: Value = request
            .post("/api/tasks/bulk")
            .json(&json!({ "ids": [t1, t2, unknown, t2], "set": { "component_pid": comp } }))
            .await
            .json();
        assert_eq!(counts(&edit), (3, 2, 1), "t2 listed twice is one request");
        // status is not bulk-editable; unknown fields refuse the whole call.
        for set in [json!({ "status": "done" }), json!({ "title": "x" }), json!({})] {
            assert_eq!(
                request.post("/api/tasks/bulk").json(&json!({ "ids": [t1], "set": set })).await.status_code(),
                422,
                "{set}"
            );
        }
        // A due date that would break a precedes relation refuses that task only.
        request
            .post(&format!("/api/plans/{a}/tasks/{t1}/relations"))
            .json(&json!({ "kind": "precedes", "to_task_pid": t3 }))
            .await
            .assert_status_ok();
        let due: Value = request
            .post("/api/tasks/bulk")
            .json(&json!({ "ids": [t1, t2], "set": { "due_date": "2026-10-07" } }))
            .await
            .json();
        assert_eq!(counts(&due), (2, 1, 1));
        let refused = due["results"].as_array().unwrap().iter().find(|r| r["result"] == "refused").unwrap();
        assert_eq!(refused["pid"], t1);
        assert!(refused["note"].as_str().unwrap().contains("precedes"));
        // A component of another plan is refused per task.
        let wrong: Value = request
            .post("/api/tasks/bulk")
            .json(&json!({ "ids": [t1], "set": { "component_pid": unknown } }))
            .await
            .json();
        assert_eq!(counts(&wrong), (1, 0, 1));

        // ---- copy: fresh tasks in B at its initial status, copied_from linked.
        let copy: Value = request
            .post("/api/tasks/copy")
            .json(&json!({ "ids": [t1, t2], "to_plan_pid": b }))
            .await
            .json();
        assert_eq!(counts(&copy), (2, 2, 0));
        let in_b: Value = request.get(&format!("/api/plans/{b}/tasks")).await.json();
        let b_tasks = in_b["tasks"].as_array().unwrap();
        assert_eq!(b_tasks.len(), 2);
        assert!(b_tasks.iter().all(|t| t["status"] == "backlog"), "initial state of B's workflow");
        assert!(b_tasks.iter().all(|t| t["assignee_ref"].is_null()), "the assignee is not copied by default");
        let copy_pid = b_tasks[0]["pid"].as_str().unwrap();
        let rel: Value = request
            .get(&format!("/api/plans/{b}/tasks/{copy_pid}/relations"))
            .await
            .json();
        assert!(format!("{rel}").contains("copied_from"));
        // The originals are untouched.
        let in_a: Value = request.get(&format!("/api/plans/{a}/tasks")).await.json();
        assert_eq!(in_a["tasks"].as_array().unwrap().len(), 3);

        // ---- move: the target workflow decides, and the result says so.
        let mv: Value = request
            .post("/api/tasks/move")
            .json(&json!({ "ids": [t2, t3], "to_plan_pid": b }))
            .await
            .json();
        assert_eq!(counts(&mv), (2, 2, 0));
        let notes: Vec<String> = mv["results"].as_array().unwrap().iter().map(|r| r["note"].as_str().unwrap_or_default().to_string()).collect();
        assert!(notes.iter().any(|n| n.contains("in_review") && n.contains("backlog")), "{notes:?}");
        let in_b: Value = request.get(&format!("/api/plans/{b}/tasks")).await.json();
        assert_eq!(in_b["tasks"].as_array().unwrap().len(), 4, "2 copies + 2 moved");
        // Moving into the plan it is already in is refused; the component link did not survive.
        let again: Value = request
            .post("/api/tasks/move")
            .json(&json!({ "ids": [t2], "to_plan_pid": b }))
            .await
            .json();
        assert_eq!(counts(&again), (1, 0, 1));
        let moved = in_b["tasks"].as_array().unwrap().iter().find(|t| t["pid"] == t2).unwrap();
        assert!(moved["component_pid"].is_null(), "plan-specific links are cleared");

        // A task with subtasks is refused until they are dealt with.
        let parent = task(&request, &a, json!({ "title": "Parent" })).await;
        task(&request, &a, json!({ "title": "Child", "parent_task_pid": parent })).await;
        let blocked: Value = request
            .post("/api/tasks/move")
            .json(&json!({ "ids": [parent], "to_plan_pid": b }))
            .await
            .json();
        assert_eq!(counts(&blocked), (1, 0, 1));
        assert!(blocked["results"][0]["note"].as_str().unwrap().contains("subtask"));

        // A target the caller cannot see, an empty list and an over-long list are 422.
        for body in [
            json!({ "ids": [t1], "to_plan_pid": unknown }),
            json!({ "ids": [], "to_plan_pid": b }),
        ] {
            assert_eq!(request.post("/api/tasks/move").json(&body).await.status_code(), 422, "{body}");
        }
    })
    .await;
}

//! Plan-template request tests (LT-2): a template captures shape and not
//! people or identity, dates become offsets from the start date given, tasks
//! restart at the workflow's initial state, and the template stands alone.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_template_keeps_shape_and_drops_people_identity_and_history() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        // ---- the source tree
        let root: Value = request
            .post("/api/plans")
            .json(&json!({
                "kind": "Project", "name": "Apollo", "start_date": "2026-10-05",
                "target_date": "2026-12-14", "code": "PROJ-1", "alternate_names": ["Moon shot"],
                "lead_ref": "person:3f2c6b7e-0000-4000-8000-000000000001",
                "keywords": ["space"]
            }))
            .await
            .json();
        let root = root["pid"].as_str().expect("root pid").to_string();
        let child: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Workstream", "parent_ref": root }))
            .await
            .json();
        let child = child["pid"].as_str().expect("child pid").to_string();
        // A workflow of the root's own, so we can see it travels.
        let state = |k: &str, c: &str, i: bool, t: bool| json!({ "key": k, "label": k, "category": c, "is_initial": i, "is_terminal": t });
        request
            .post("/api/workflows")
            .json(&json!({ "name": "Gated", "applies_to": "task", "plan_pid": root,
                "states": [state("backlog", "todo", true, false), state("shipped", "done", false, true)] }))
            .await
            .assert_status_ok();
        let comp: Value = request
            .post(&format!("/api/plans/{root}/components"))
            .json(&json!({ "name": "API" }))
            .await
            .json();
        let comp = comp["pid"].as_str().unwrap().to_string();
        let parent_task: Value = request
            .post(&format!("/api/plans/{root}/tasks"))
            .json(&json!({ "title": "Epic", "due_date": "2026-10-19", "points": 8, "component_pid": comp,
                           "assignee_ref": "person:3f2c6b7e-0000-4000-8000-000000000002" }))
            .await
            .json();
        let parent_task = parent_task["pid"].as_str().unwrap().to_string();
        request
            .post(&format!("/api/plans/{root}/tasks"))
            .json(&json!({ "title": "Story", "parent_task_pid": parent_task }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{child}/tasks"))
            .json(&json!({ "title": "Child task" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{root}/milestones"))
            .json(&json!({ "name": "Beta", "due": "2026-10-20" }))
            .await
            .assert_status_ok();

        // ---- save
        let saved: Value = request
            .post(&format!("/api/plans/{root}/save-as-template"))
            .json(&json!({ "name": "Standard project" }))
            .await
            .json();
        let tpl = saved["pid"].as_str().expect("template pid").to_string();
        assert_eq!(saved["plans"], 2);
        assert_eq!(saved["tasks"], 3);
        assert_eq!(saved["milestones"], 1);
        assert_eq!(saved["needs_start_date"], true);
        assert_eq!(
            request
                .post(&format!("/api/plans/{root}/save-as-template"))
                .json(&json!({ "name": "standard PROJECT" }))
                .await
                .status_code(),
            409,
            "template names are unique, case-insensitively"
        );
        let listed: Value = request.get("/api/plan-templates").await.json();
        assert_eq!(listed[0]["plans"], 2);

        // The template stands alone: editing the source afterwards changes nothing.
        let tasks: Value = request.get(&format!("/api/plans/{root}/tasks")).await.json();
        let epic = tasks["tasks"].as_array().unwrap().iter().find(|t| t["title"] == "Epic").unwrap()["pid"].as_str().unwrap().to_string();
        request
            .put(&format!("/api/plans/{root}/tasks/{epic}"))
            .json(&json!({ "title": "Epic (renamed)" }))
            .await
            .assert_status_ok();

        // ---- instantiate
        assert_eq!(
            request.post(&format!("/api/plans/from-template/{tpl}")).json(&json!({})).await.status_code(),
            422,
            "a dated template needs a start date"
        );
        // 2027-01-11 is 98 calendar days after the source start (2026-10-05).
        let made: Value = request
            .post(&format!("/api/plans/from-template/{tpl}"))
            .json(&json!({ "name": "Gemini", "start_date": "2027-01-11" }))
            .await
            .json();
        let new_root = made["root_pid"].as_str().expect("new root").to_string();
        assert_ne!(new_root, root);
        assert_eq!(made["plans"].as_array().unwrap().len(), 2);
        assert_eq!(made["tasks_created"], 3);
        assert_eq!(made["milestones_created"], 1);
        let new_child = made["plans"][1].as_str().unwrap().to_string();

        let r: Value = request.get(&format!("/api/plans/{new_root}")).await.json();
        assert_eq!(r["name"], "Gemini");
        assert_eq!(r["start_date"], "2027-01-11");
        assert_eq!(r["target_date"], "2027-03-22", "start + the captured 70-calendar-day offset");
        assert!(r["lead_ref"].is_null(), "no person is copied");
        assert!(r["code"].is_null(), "no owner-scoped code is copied");
        assert_eq!(r["alternate_names"], json!([]), "no former names are copied");
        assert_eq!(r["keywords"], json!(["space"]), "descriptive content is");
        let c: Value = request.get(&format!("/api/plans/{new_child}")).await.json();
        assert_eq!(c["parent_ref"], new_root, "shape is preserved under the new root");

        let nt: Value = request.get(&format!("/api/plans/{new_root}/tasks")).await.json();
        let nt = nt["tasks"].as_array().unwrap();
        assert_eq!(nt.len(), 2);
        let epic_new = nt.iter().find(|t| t["title"] == "Epic").expect("the template kept the original title");
        assert_eq!(epic_new["due_date"], "2027-01-25", "start + the task's offset (14 calendar days)");
        assert!(epic_new["assignee_ref"].is_null(), "no assignee is copied");
        assert_eq!(epic_new["status"], "backlog", "the workflow's initial state");
        assert!(epic_new["component_pid"].is_string(), "its component came along");
        let story = nt.iter().find(|t| t["title"] == "Story").unwrap();
        assert_eq!(story["parent_task_pid"], epic_new["pid"], "subtask nesting is preserved");
        let ms: Value = request.get(&format!("/api/plans/{new_root}/milestones")).await.json();
        assert_eq!(ms[0]["due"], "2027-01-26", "start + 15 calendar days");
        let wf: Value = request.get(&format!("/api/plans/{new_root}/workflow")).await.json();
        assert_eq!(wf["source"], "plan", "the root's own workflow travelled");

        // The source is untouched.
        let src: Value = request.get(&format!("/api/plans/{root}")).await.json();
        assert_eq!(src["code"], "PROJ-1");
        // Delete (POST twin) hides the template; plans made from it remain.
        request.post(&format!("/api/plan-templates/{tpl}/delete")).await.assert_status_ok();
        assert_eq!(request.get(&format!("/api/plan-templates/{tpl}")).await.status_code(), 404);
        request.get(&format!("/api/plans/{new_root}")).await.assert_status_ok();
    })
    .await;
}

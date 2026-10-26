//! Reverse references request test (BR-4): the meetings and lessons that point at one record of a plan.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_meeting_and_a_lesson_appear_under_the_task_they_point_at_and_the_count_is_the_list() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let mk = |name: &'static str| {
            let request = &request;
            async move {
                let p: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": name })).await.json();
                p["pid"].as_str().unwrap().to_string()
            }
        };
        let plan = mk("Reference plan").await;
        let other = mk("Another reference plan").await;
        let task_of = |plan: String, title: &'static str| {
            let request = &request;
            async move {
                let t: Value = request.post(&format!("/api/plans/{plan}/tasks")).json(&json!({ "title": title })).await.json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let task = task_of(plan.clone(), "Re-plan").await;
        let quiet = task_of(plan.clone(), "Untouched").await;
        let foreign = task_of(other.clone(), "Not ours").await;

        // A planned meeting (agenda link), a held meeting (outcome link), and a lesson, all at the task.
        let planned: Value = request.post(&format!("/api/plans/{plan}/meetings"))
            .json(&json!({ "title": "Gate 2", "scheduled_on": "2026-11-02",
                "agenda": [{ "text": "Is the schedule right?", "link": { "kind": "task", "pid": task } }] }))
            .await.json();
        let held_id = planned["pid"].as_str().unwrap().to_string();
        let second: Value = request.post(&format!("/api/plans/{plan}/meetings"))
            .json(&json!({ "title": "Weekly", "scheduled_on": "2026-11-09" })).await.json();
        let second = second["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/plans/{plan}/meetings/{second}/held"))
            .json(&json!({ "minutes": "Discussed.", "held_on": "2026-11-09",
                "outcomes": [{ "kind": "action", "text": "Redo the plan", "link": { "kind": "task", "pid": task } }] }))
            .await.assert_status_ok();
        let lesson: Value = request.post(&format!("/api/plans/{plan}/lessons"))
            .json(&json!({ "occasion": "closure", "category": "went_badly", "summary": "Re-planning came late",
                "link": { "kind": "task", "pid": task } }))
            .await.json();
        let lesson_id = lesson["pid"].as_str().unwrap().to_string();
        assert_eq!(lesson["link_kind"], "task");
        // A lesson naming another plan's record, or an unknown kind, is refused and stores nothing.
        for link in [json!({ "kind": "task", "pid": foreign }), json!({ "kind": "plan", "pid": task })] {
            let r = request.post(&format!("/api/plans/{plan}/lessons"))
                .json(&json!({ "occasion": "closure", "category": "went_well", "summary": "x", "link": link })).await;
            assert_eq!(r.status_code(), 422, "{link}");
        }

        let got: Value = request.get(&format!("/api/plans/{plan}/references?kind=task&pid={task}")).await.json();
        assert_eq!(got["target"]["title"], "Re-plan");
        assert_eq!(got["meetings"].as_array().unwrap().len(), 2, "{got}");
        assert_eq!(got["lessons"].as_array().unwrap().len(), 1);
        assert_eq!(got["lessons"][0]["lesson_pid"], lesson_id.as_str());
        assert_eq!(got["count"], 3, "the count is the list");
        assert!(got["meetings"].to_string().contains(&held_id));

        // A task nothing points at: an empty list and a zero count, not a 404.
        let none: Value = request.get(&format!("/api/plans/{plan}/references?kind=task&pid={quiet}")).await.json();
        assert_eq!((none["count"].as_i64(), none["meetings"].as_array().map(Vec::len)), (Some(0), Some(0)));

        // Another plan's record, an unknown kind and a made-up pid all answer as one that does not exist.
        for q in [format!("kind=task&pid={foreign}"), format!("kind=plan&pid={task}"), format!("kind=task&pid={}", uuid::Uuid::new_v4())] {
            assert_eq!(request.get(&format!("/api/plans/{plan}/references?{q}")).await.status_code(), 404, "{q}");
        }

        // A withdrawn lesson is no longer a reference.
        request.post(&format!("/api/plans/{plan}/lessons/{lesson_id}/withdraw")).await.assert_status_ok();
        let after: Value = request.get(&format!("/api/plans/{plan}/references?kind=task&pid={task}")).await.json();
        assert_eq!((after["lessons"].as_array().map(Vec::len), after["count"].as_i64()), (Some(0), Some(2)));
    })
    .await;
}

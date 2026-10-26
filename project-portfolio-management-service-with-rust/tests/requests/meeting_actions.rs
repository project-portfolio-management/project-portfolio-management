//! Action items from minutes to tasks (BR-3): preview judges, commit creates once, a retry creates nothing more.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const PRIYA: &str = "person:11111111-1111-1111-1111-111111111111";
const STRANGER: &str = "person:99999999-9999-9999-9999-999999999999";

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn proposed_tasks_are_verified_against_the_minutes_and_created_once() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Harbour works" })).await.json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        // Priya is a person of the plan because a task of the plan is hers.
        request
            .post(&format!("/api/plans/{plan}/tasks"))
            .json(&json!({ "title": "Survey", "assignee_ref": PRIYA }))
            .await
            .assert_status_ok();

        // ---- a meeting that is not held has nothing to propose against
        let planned: Value = request
            .post(&format!("/api/plans/{plan}/meetings"))
            .json(&json!({ "title": "Site review", "scheduled_on": "2026-11-02" }))
            .await
            .json();
        let mid = planned["pid"].as_str().unwrap().to_string();
        let base = format!("/api/plans/{plan}/meetings/{mid}/task-proposals");
        let one = json!({ "items": [{ "title": "X", "excerpt": "Priya will order the racks", "outcome": 0 }] });
        assert_eq!(request.post(&format!("{base}/preview")).json(&one).await.status_code(), 409);

        request
            .post(&format!("/api/plans/{plan}/meetings/{mid}/held"))
            .json(&json!({
                "minutes": "Reviewed the quay.\nPriya will order the racks by the tenth.\nSomeone must tell the sponsor.",
                "held_on": "2026-11-03",
                "outcomes": [
                    { "kind": "action", "text": "Order the racks" },
                    { "kind": "note", "text": "Quay is sound" },
                    { "kind": "action", "text": "Tell the sponsor" }
                ]
            }))
            .await
            .assert_status_ok();

        // ---- preview: one ready, and each way of being dropped is named; nothing is written
        let items = json!({ "items": [
            { "title": "Order the racks", "excerpt": "priya will ORDER the racks", "outcome": 0, "assignee_ref": PRIYA, "due_on": "2026-11-10" },
            { "title": "Invented", "excerpt": "The sponsor approved the budget", "outcome": 2 },
            { "title": "On a note", "excerpt": "Reviewed the quay", "outcome": 1 },
            { "title": "Far away", "excerpt": "Someone must tell the sponsor", "outcome": 2, "due_on": "2030-01-01" },
            { "title": "Stranger", "excerpt": "Someone must tell the sponsor", "outcome": 2, "assignee_ref": STRANGER }
        ]});
        let seen: Value = request.post(&format!("{base}/preview")).json(&items).await.json();
        assert_eq!(seen["counts"], json!({ "ready": 1, "dropped": 4 }));
        let reasons: Vec<&str> = seen["items"].as_array().unwrap().iter().map(|i| i["reason"].as_str().unwrap_or("")).collect();
        assert_eq!(reasons, ["", "excerpt_not_in_minutes", "outcome_not_an_action", "due_outside_window", "assignee_not_in_plan"]);
        let tasks: Value = request.get(&format!("/api/plans/{plan}/tasks")).await.json();
        assert_eq!(tasks["tasks"].as_array().unwrap().len(), 1, "a preview writes nothing");

        // ---- commit refuses the whole request when one item is not ready, and writes nothing
        assert_eq!(request.post(&format!("{base}/commit")).json(&items).await.status_code(), 422);
        assert_eq!(request.get(&format!("/api/plans/{plan}/tasks")).await.json::<Value>()["tasks"].as_array().unwrap().len(), 1);

        // ---- commit with the good ones; a retry under the same key creates nothing more
        let good = json!({ "items": [
            { "title": "Order the racks", "excerpt": "priya will ORDER the racks", "outcome": 0, "assignee_ref": PRIYA, "due_on": "2026-11-10", "points": 3 },
            { "title": "Tell the sponsor", "excerpt": "Someone must tell the sponsor", "outcome": 2 }
        ]});
        let first = request.post(&format!("{base}/commit")).add_header("Idempotency-Key", "mins-1").json(&good).await;
        first.assert_status_ok();
        let first: Value = first.json();
        assert_eq!(first["created"].as_array().unwrap().len(), 2);
        let retry = request.post(&format!("{base}/commit")).add_header("Idempotency-Key", "mins-1").json(&good).await;
        assert_eq!(retry.json::<Value>(), first);
        let tasks: Value = request.get(&format!("/api/plans/{plan}/tasks")).await.json();
        assert_eq!(tasks["tasks"].as_array().unwrap().len(), 3, "the retry created nothing");
        let made = tasks["tasks"].as_array().unwrap().iter().find(|t| t["title"] == "Order the racks").unwrap();
        assert_eq!((made["assignee_ref"].as_str(), made["due_date"].as_str(), made["points"].as_i64()), (Some(PRIYA), Some("2026-11-10"), Some(3)));

        // ---- the meeting shows which task came from which outcome
        let got: Value = request.get(&format!("/api/plans/{plan}/meetings/{mid}")).await.json();
        assert_eq!(got["outcomes"][0]["task_pid"], first["created"][0]["task_pid"]);
        assert!(got["outcomes"][1]["task_pid"].is_null());
        assert_eq!(got["outcomes"][2]["task_pid"], first["created"][1]["task_pid"]);

        // ---- another commit, under another key, cannot make a second task of the same outcome
        let again = request.post(&format!("{base}/commit")).add_header("Idempotency-Key", "mins-2").json(&good).await;
        assert_eq!(again.status_code(), 422);
        assert_eq!(request.get(&format!("/api/plans/{plan}/tasks")).await.json::<Value>()["tasks"].as_array().unwrap().len(), 3);
    })
    .await;
}

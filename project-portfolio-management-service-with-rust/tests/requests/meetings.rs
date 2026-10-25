//! Meetings request test (NC-14): plan a meeting with an agenda linked to the plan's records, hold it with minutes
//! and outcomes, print it as a PDF, and see a held meeting refuse a second record.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_meeting_is_planned_held_linked_and_printed_and_a_held_one_is_a_record() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Gate programme" })).await.json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let other: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Other programme" })).await.json();
        let other = other["pid"].as_str().unwrap().to_string();
        let task: Value = request
            .post(&format!("/api/plans/{plan}/tasks"))
            .json(&json!({ "title": "Re-plan" }))
            .await
            .json();
        let task = task["pid"].as_str().unwrap().to_string();
        let foreign: Value = request
            .post(&format!("/api/plans/{other}/tasks"))
            .json(&json!({ "title": "Not ours" }))
            .await
            .json();
        let foreign = foreign["pid"].as_str().unwrap().to_string();

        // ---- planned, with an agenda that points at the plan's own task
        let planned: Value = request
            .post(&format!("/api/plans/{plan}/meetings"))
            .json(&json!({
                "title": "Gate 2 review", "scheduled_on": "2026-11-02",
                "agenda": [
                    { "text": "Is the schedule still right?", "link": { "kind": "task", "pid": task } },
                    { "text": "Open risks" }
                ]
            }))
            .await
            .json();
        let mid = planned["pid"].as_str().unwrap().to_string();
        assert_eq!(planned["status"], "planned");

        // ---- a link to another plan's record, a bad kind and a missing title are refused
        for body in [
            json!({ "title": "X", "scheduled_on": "2026-11-02", "agenda": [{ "text": "t", "link": { "kind": "task", "pid": foreign } }] }),
            json!({ "title": "X", "scheduled_on": "2026-11-02", "agenda": [{ "text": "t", "link": { "kind": "plan", "pid": task } }] }),
            json!({ "title": " ", "scheduled_on": "2026-11-02" }),
        ] {
            assert_eq!(request.post(&format!("/api/plans/{plan}/meetings")).json(&body).await.status_code(), 422, "{body}");
        }

        // ---- the agenda reads back with the linked record named
        let got: Value = request.get(&format!("/api/plans/{plan}/meetings/{mid}")).await.json();
        assert_eq!(got["agenda"][0]["link"]["name"], "Re-plan");
        assert!(got["minutes"].is_null() && got["outcomes"].as_array().unwrap().is_empty());
        // A meeting is read under its own plan only.
        assert_eq!(request.get(&format!("/api/plans/{other}/meetings/{mid}")).await.status_code(), 404);

        // ---- held: minutes are required; outcomes link too
        let no_minutes = request
            .post(&format!("/api/plans/{plan}/meetings/{mid}/held"))
            .json(&json!({ "minutes": "" }))
            .await;
        assert_eq!(no_minutes.status_code(), 422);
        let held = json!({
            "minutes": "We reviewed the schedule.\nIt slips two weeks.",
            "held_on": "2026-11-03",
            "outcomes": [
                { "kind": "decision", "text": "Accept the slip", "link": { "kind": "task", "pid": task } },
                { "kind": "action", "text": "Tell the sponsor" }
            ]
        });
        let done: Value = request.post(&format!("/api/plans/{plan}/meetings/{mid}/held")).json(&held).await.json();
        assert_eq!(done["status"], "held");
        let got: Value = request.get(&format!("/api/plans/{plan}/meetings/{mid}")).await.json();
        assert_eq!(got["held_on"], "2026-11-03");
        assert_eq!(got["outcomes"][0]["kind"], "decision");
        assert_eq!(got["outcomes"].as_array().unwrap().len(), 2);

        // ---- a held meeting is a record: not held again, not cancelled
        assert_eq!(request.post(&format!("/api/plans/{plan}/meetings/{mid}/held")).json(&held).await.status_code(), 409);
        assert_eq!(request.post(&format!("/api/plans/{plan}/meetings/{mid}/cancel")).await.status_code(), 409);

        // ---- the PDF carries what the JSON shows, and two renders are identical
        let pdf = request.get(&format!("/api/plans/{plan}/meetings/{mid}/pdf")).await;
        pdf.assert_status_ok();
        assert_eq!(pdf.header("content-type"), "application/pdf");
        let bytes = pdf.as_bytes().to_vec();
        assert!(bytes.starts_with(b"%PDF"));
        let text = project_portfolio_management::pdf::text_of(&bytes);
        for want in ["Is the schedule still right?", "about task: Re-plan", "It slips two weeks.", "[decision] Accept the slip", "Held on: 2026-11-03"] {
            assert!(text.contains(want), "{want} missing from {text}");
        }
        let again = request.get(&format!("/api/plans/{plan}/meetings/{mid}/pdf")).await;
        assert_eq!(again.as_bytes().to_vec(), bytes);

        // ---- a second meeting is cancelled; the list is newest first
        let second: Value = request
            .post(&format!("/api/plans/{plan}/meetings"))
            .json(&json!({ "title": "Retro", "scheduled_on": "2026-12-01" }))
            .await
            .json();
        let second = second["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/plans/{plan}/meetings/{second}/cancel")).await.assert_status_ok();
        let list: Value = request.get(&format!("/api/plans/{plan}/meetings")).await.json();
        let titles: Vec<&str> = list.as_array().unwrap().iter().map(|m| m["title"].as_str().unwrap()).collect();
        assert_eq!(titles, ["Retro", "Gate 2 review"]);
        assert_eq!(list[0]["status"], "cancelled");
    })
    .await;
}

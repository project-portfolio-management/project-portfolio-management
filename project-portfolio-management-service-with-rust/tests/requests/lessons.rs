//! The lessons-learned register (NC-8): lessons recorded at a gate and at closure, shown at intake for a similar
//! proposal with the match evidence, and not shown for an unrelated one.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axum::http::StatusCode;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

fn proposal(title: &str) -> Value {
    json!({
        "title": title,
        "summary": "Intake",
        "kind_target": "projects",
        "sponsor_ref": format!("organization:{}", uuid::Uuid::new_v4()),
        "requested_minor": 1_000_000_i64,
        "currency": "GBP",
    })
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_similar_proposal_is_shown_the_lessons_of_a_past_plan_with_the_match_evidence() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let past: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Data warehouse migration" }))
            .await
            .json();
        let past_pid = past["pid"].as_str().expect("pid").to_string();
        let other: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Office relocation" }))
            .await
            .json();
        let other_pid = other["pid"].as_str().expect("pid").to_string();

        let url = format!("/api/plans/{past_pid}/lessons");
        let gate: Value = request
            .post(&url)
            .json(&json!({ "occasion": "g2_definition", "category": "went_badly",
                           "summary": "Cut-over rehearsal was left too late" }))
            .await
            .json();
        request
            .post(&url)
            .json(&json!({ "occasion": "closure", "category": "recommendation",
                           "summary": "Rehearse the cut-over twice", "detail": "Once early, once at freeze." }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{other_pid}/lessons"))
            .json(&json!({ "occasion": "closure", "category": "went_well", "summary": "Movers were punctual" }))
            .await
            .assert_status_ok();

        // Refusals: an occasion that is neither a gate nor closure, a category, an empty summary.
        for bad in [
            json!({ "occasion": "kickoff", "category": "went_well", "summary": "x" }),
            json!({ "occasion": "closure", "category": "fun", "summary": "x" }),
            json!({ "occasion": "closure", "category": "went_well", "summary": "  " }),
        ] {
            request
                .post(&url)
                .json(&bad)
                .await
                .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        }

        let listed: Value = request.get(&url).await.json();
        assert_eq!(listed.as_array().unwrap().len(), 2);

        let similar: Value = request
            .post("/api/proposals")
            .json(&proposal("Data warehouse migration"))
            .await
            .json();
        let similar_pid = similar["pid"].as_str().expect("pid");
        let hits: Value = request
            .get(&format!("/api/proposals/{similar_pid}/lessons"))
            .await
            .json();
        let hits = hits.as_array().expect("array");
        assert_eq!(hits.len(), 1, "only the similar plan: {hits:?}");
        assert_eq!(hits[0]["plan_pid"], past_pid.as_str());
        assert!(hits[0]["match"]["score"].as_f64().unwrap() > 0.5);
        assert!(hits[0]["match"]["breakdown"].is_object(), "the evidence");
        assert_eq!(hits[0]["lessons"].as_array().unwrap().len(), 2);

        let unrelated: Value = request
            .post("/api/proposals")
            .json(&proposal("Staff canteen refit"))
            .await
            .json();
        let none: Value = request
            .get(&format!("/api/proposals/{}/lessons", unrelated["pid"].as_str().unwrap()))
            .await
            .json();
        assert!(none.as_array().unwrap().is_empty());

        // A withdrawn lesson is no longer shown, and the withdrawal is audited.
        request
            .post(&format!("{url}/{}/withdraw", gate["pid"].as_str().unwrap()))
            .await
            .assert_status_ok();
        let after: Value = request
            .get(&format!("/api/proposals/{similar_pid}/lessons"))
            .await
            .json();
        assert_eq!(after[0]["lessons"].as_array().unwrap().len(), 1);
        let audit: Value = request.get(&format!("/api/plans/{past_pid}/audit")).await.json();
        let text = audit.to_string();
        assert!(text.contains("lesson_recorded") && text.contains("lesson_withdrawn"), "{text}");
    })
    .await;
}

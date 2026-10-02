//! Custom-field request tests (RM-1): typed validation, kind scoping,
//! required fields that block task creation, atomic writes, equality
//! search, and withdrawn definitions that never hide stored values.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer, kind: &str, name: &str) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": kind, "name": name }))
        .await
        .json();
    created["pid"].as_str().expect("pid").to_string()
}

async fn status_of(request: &axum_test::TestServer, body: Value) -> u16 {
    request
        .put("/api/custom-fields/values")
        .json(&body)
        .await
        .status_code()
        .as_u16()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn custom_fields_are_typed_scoped_required_and_searchable() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        // Definitions: invalid ones are refused; a duplicate key is a 409.
        assert_eq!(
            request
                .post("/api/custom-fields")
                .json(&json!({ "key": "Bad Key", "label": "x", "type": "nope", "applies_to": "plan" }))
                .await
                .status_code(),
            422
        );
        for def in [
            json!({ "key": "cost_centre", "label": "Cost centre", "type": "text", "applies_to": "plan" }),
            json!({ "key": "risk_class", "label": "Risk class", "type": "list", "applies_to": "plan",
                    "options": ["low", "high"], "kinds": ["Project"] }),
            json!({ "key": "regions", "label": "Regions", "type": "multi_list", "applies_to": "plan",
                    "options": ["emea", "apac", "amer"] }),
            json!({ "key": "go_live", "label": "Go live", "type": "date", "applies_to": "plan" }),
            json!({ "key": "estimate", "label": "Estimate", "type": "integer", "applies_to": "task", "required": true }),
        ] {
            request.post("/api/custom-fields").json(&def).await.assert_status_ok();
        }
        assert_eq!(
            request
                .post("/api/custom-fields")
                .json(&json!({ "key": "cost_centre", "label": "dup", "type": "text", "applies_to": "plan" }))
                .await
                .status_code(),
            409
        );

        let alpha = plan(&request, "Project", "Alpha").await;
        let beta = plan(&request, "Project", "Beta").await;
        let prog = plan(&request, "Program", "Prog").await;

        // Typed validation.
        for bad in [
            json!({ "cost_centre": 5 }),
            json!({ "risk_class": "medium" }),
            json!({ "regions": ["emea", "mars"] }),
            json!({ "go_live": "03/10/2026" }),
            json!({ "no_such_field": "x" }),
        ] {
            assert_eq!(
                status_of(&request, json!({ "applies_to": "plan", "subject_pid": alpha, "values": bad })).await,
                422,
                "{bad}"
            );
        }
        // Atomic: one bad value among good ones writes nothing.
        assert_eq!(
            status_of(&request, json!({ "applies_to": "plan", "subject_pid": alpha,
                "values": { "cost_centre": "CC-1", "risk_class": "medium" } })).await,
            422
        );
        let none: Value = request
            .get(&format!("/api/custom-fields/values?applies_to=plan&subject_pid={alpha}"))
            .await
            .json();
        assert!(none["values"].as_object().unwrap().is_empty(), "nothing was written");

        // Kind scoping: risk_class is for Projects only.
        assert_eq!(
            status_of(&request, json!({ "applies_to": "plan", "subject_pid": prog, "values": { "risk_class": "low" } })).await,
            422
        );

        // Set values and search them.
        for (p, cc, regions) in [(&alpha, "CC-1", json!(["emea", "apac"])), (&beta, "CC-2", json!(["amer"]))] {
            assert_eq!(
                status_of(&request, json!({ "applies_to": "plan", "subject_pid": p,
                    "values": { "cost_centre": cc, "regions": regions, "risk_class": "high" } })).await,
                200
            );
        }
        let hit: Value = request
            .get("/api/custom-fields/search?applies_to=plan&key=cost_centre&value=CC-1")
            .await
            .json();
        assert_eq!(hit["pids"], json!([alpha]), "exactly the matching plan");
        let multi: Value = request
            .get("/api/custom-fields/search?applies_to=plan&key=regions&value=apac")
            .await
            .json();
        assert_eq!(multi["count"], 1, "multi_list means contains");

        // A required task field blocks task creation until supplied.
        assert_eq!(
            request
                .post(&format!("/api/plans/{alpha}/tasks"))
                .json(&json!({ "title": "needs estimate" }))
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .post(&format!("/api/plans/{alpha}/tasks"))
                .json(&json!({ "title": "bad estimate", "custom": { "estimate": "lots" } }))
                .await
                .status_code(),
            422
        );
        let task: Value = request
            .post(&format!("/api/plans/{alpha}/tasks"))
            .json(&json!({ "title": "estimated", "custom": { "estimate": 5 } }))
            .await
            .json();
        let t = task["pid"].as_str().expect("task pid").to_string();
        let tv: Value = request
            .get(&format!("/api/custom-fields/values?applies_to=task&subject_pid={t}"))
            .await
            .json();
        assert_eq!(tv["values"]["estimate"]["value"], 5);
        assert_eq!(
            status_of(&request, json!({ "applies_to": "task", "subject_pid": t, "values": { "estimate": null } })).await,
            422,
            "a required field cannot be cleared"
        );

        // Withdrawing a definition keeps the stored value readable.
        let defs: Value = request.get("/api/custom-fields?applies_to=plan").await.json();
        let cc = defs
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["key"] == "cost_centre")
            .expect("def")["pid"]
            .as_str()
            .unwrap()
            .to_string();
        request.delete(&format!("/api/custom-fields/{cc}")).await.assert_status_ok();
        let after: Value = request
            .get(&format!("/api/custom-fields/values?applies_to=plan&subject_pid={alpha}"))
            .await
            .json();
        assert_eq!(after["values"]["cost_centre"]["value"], "CC-1");
        assert_eq!(after["values"]["cost_centre"]["definition"], "withdrawn");
        assert_eq!(
            status_of(&request, json!({ "applies_to": "plan", "subject_pid": alpha, "values": { "cost_centre": "x" } })).await,
            422,
            "no new values for a withdrawn field"
        );
    })
    .await;
}

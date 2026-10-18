//! `?include=` request tests (RM-12): the bare plan is unchanged without it;
//! with it the named sub-resources are embedded; an unknown name is 422
//! listing the allowed ones; a disabled module's sub-resource is omitted and
//! *listed*.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn include_embeds_named_sub_resources_and_lists_what_it_omits() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        request
            .post(&format!("/api/plans/{p}/risks"))
            .json(&json!({ "title": "Supplier failure", "probability": 3, "impact": 4 }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Ship" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/milestones"))
            .json(&json!({ "name": "Beta", "due": "2026-10-20" }))
            .await
            .assert_status_ok();

        // Unchanged without include: the bare plan.
        let bare: Value = request.get(&format!("/api/plans/{p}")).await.json();
        assert_eq!(bare["name"], "Apollo");
        assert!(bare.get("included").is_none());

        // With include.
        let inc: Value = request
            .get(&format!("/api/plans/{p}?include=risks,tasks,milestones"))
            .await
            .json();
        assert_eq!(inc["plan"]["name"], "Apollo");
        assert_eq!(inc["included"]["risks"].as_array().unwrap().len(), 1);
        assert_eq!(inc["included"]["tasks"][0]["title"], "Ship");
        assert_eq!(inc["included"]["milestones"][0]["name"], "Beta");
        assert!(inc["omitted"].as_array().unwrap().is_empty());
        // Only what was asked for.
        assert!(inc["included"].get("time_entries").is_none());

        // An unknown name is refused, listing the allowed ones.
        let bad = request
            .get(&format!("/api/plans/{p}?include=risks,nope"))
            .await;
        assert_eq!(bad.status_code(), 422);
        assert!(
            bad.text().contains("nope") && bad.text().contains("milestones"),
            "{}",
            bad.text()
        );

        // A disabled module is omitted and listed, never silently missing.
        request
            .put(&format!("/api/plans/{p}/modules"))
            .json(&json!({ "modules": ["tasks"] }))
            .await
            .assert_status_ok();
        let gated: Value = request
            .get(&format!("/api/plans/{p}?include=risks,tasks"))
            .await
            .json();
        assert!(gated["included"].get("risks").is_none());
        assert_eq!(gated["included"]["tasks"].as_array().unwrap().len(), 1);
        assert_eq!(
            gated["omitted"],
            json!([{ "name": "risks", "reason": "module_disabled" }])
        );
    })
    .await;
}

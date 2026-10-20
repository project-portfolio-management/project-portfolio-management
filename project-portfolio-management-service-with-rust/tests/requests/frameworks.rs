//! Frameworks (ways of working and governance): records, plan links, and inheritance down the plan tree.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_project_follows_its_own_ways_of_working_or_inherits_its_programmes() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let make_plan = |name: &'static str, parent: Option<String>| {
            let request = &request;
            async move {
                let mut body = json!({ "kind": "Project", "name": name });
                if let Some(p) = parent {
                    body["parent_ref"] = json!(p);
                }
                let plan: Value = request.post("/api/plans").json(&body).await.json();
                plan["pid"].as_str().expect("pid").to_string()
            }
        };
        let programme = make_plan("WoW programme", None).await;
        let project = make_plan("WoW project", Some(programme.clone())).await;
        let effective = |plan: &str| {
            let (request, path) = (&request, format!("/api/plans/{plan}/ways-of-working"));
            async move { request.get(&path).await.json::<Value>() }
        };

        let none = effective(&project).await;
        assert_eq!(none["source"], "none");
        assert!(none["ways_of_working"].is_null());
        assert!(none["reason"].as_str().expect("a reason, not a default").contains("declares"));

        // An organisation-wide record, linked from the programme.
        let org: Value = request
            .post("/api/ways-of-working")
            .json(&json!({
                "name": "Two-week Scrum", "methodology": "scrum", "cadence_days": 14, "wip_limit": 5,
                "definition_of_done": "Reviewed and deployed",
                "ceremonies": [{ "name": "Retrospective", "cadence": "every sprint", "purpose": "Improve" }]
            }))
            .await
            .json();
        let org_pid = org["pid"].as_str().expect("pid").to_string();
        assert_eq!(org["scope"], "organization");
        let linked: Value = request
            .put(&format!("/api/plans/{programme}/ways-of-working"))
            .json(&json!({ "framework_pid": org_pid }))
            .await
            .json();
        assert_eq!(linked["source"], "own");

        let inherited = effective(&project).await;
        assert_eq!(inherited["source"], "inherited");
        assert_eq!(inherited["from_plan_pid"], programme.as_str());
        assert_eq!(inherited["ways_of_working"]["name"], "Two-week Scrum");
        assert_eq!(inherited["ways_of_working"]["ceremonies"][0]["name"], "Retrospective");

        // The project makes its own, which wins; unlinking returns it to the programme's.
        let own: Value = request
            .post("/api/ways-of-working")
            .json(&json!({ "name": "Kanban", "methodology": "kanban", "wip_limit": 3, "plan_pid": project }))
            .await
            .json();
        assert_eq!(own["scope"], "plan");
        let mine = effective(&project).await;
        assert_eq!((mine["source"].as_str(), mine["ways_of_working"]["name"].as_str()), (Some("own"), Some("Kanban")));
        request
            .put(&format!("/api/plans/{project}/ways-of-working"))
            .json(&json!({ "framework_pid": null }))
            .await
            .assert_status_ok();
        assert_eq!(effective(&project).await["source"], "inherited");

        // Listing: the organisation's and the project's own; `?plan=` narrows.
        let all: Value = request.get("/api/ways-of-working").await.json();
        assert!(all.as_array().expect("list").len() >= 2);
        let only: Value = request.get(&format!("/api/ways-of-working?plan={project}")).await.json();
        assert_eq!(only.as_array().map(Vec::len), Some(1));
        assert_eq!(only[0]["name"], "Kanban");

        // Refusals.
        for (body, status, why) in [
            (json!({ "name": "", "methodology": "scrum" }), 422, "a blank name"),
            (json!({ "name": "X", "methodology": "chaos" }), 422, "an unknown methodology"),
            (json!({ "name": "X", "cadence_days": 0 }), 422, "a cadence out of range"),
            (json!({ "name": "two-week scrum" }), 409, "the name is taken (case-insensitive)"),
            (json!({ "name": "X", "plan_pid": "00000000-0000-4000-8000-000000000000" }), 404, "an unknown plan"),
        ] {
            assert_eq!(request.post("/api/ways-of-working").json(&body).await.status_code(), status, "{why}");
        }
        assert_eq!(
            request
                .put(&format!("/api/plans/{project}/ways-of-working"))
                .json(&json!({ "framework_pid": "00000000-0000-4000-8000-000000000000" }))
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request.post(&format!("/api/ways-of-working/{org_pid}/delete")).await.status_code(),
            409,
            "a record plans follow cannot be deleted"
        );

        // Unlink the programme, then the record can go, and the project is back to none.
        request
            .put(&format!("/api/plans/{programme}/ways-of-working"))
            .json(&json!({ "framework_pid": null }))
            .await
            .assert_status_ok();
        request.post(&format!("/api/ways-of-working/{org_pid}/delete")).await.assert_status_ok();
        assert_eq!(request.get(&format!("/api/ways-of-working/{org_pid}")).await.status_code(), 404);
        assert_eq!(effective(&project).await["source"], "none");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn governance_is_followed_separately_from_ways_of_working() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let make_plan = |name: &'static str, parent: Option<String>| {
            let request = &request;
            async move {
                let mut body = json!({ "kind": "Project", "name": name });
                if let Some(p) = parent {
                    body["parent_ref"] = json!(p);
                }
                let plan: Value = request.post("/api/plans").json(&body).await.json();
                plan["pid"].as_str().expect("pid").to_string()
            }
        };
        let programme = make_plan("Gov programme", None).await;
        let project = make_plan("Gov project", Some(programme.clone())).await;

        let none: Value = request.get(&format!("/api/plans/{project}/governance-framework")).await.json();
        assert_eq!(none["source"], "none");
        assert!(none["governance_framework"].is_null());

        let steering: Value = request
            .post("/api/governance-frameworks")
            .json(&json!({
                "name": "Programme board", "decision_model": "steering_committee",
                "review_cadence_days": 28, "reporting_cadence_days": 7,
                "escalation_path": [
                    { "level": 1, "body": "Programme manager", "trigger": "A risk is red for a week" },
                    { "level": 2, "body": "Sponsor", "trigger": "A decision is blocked for two weeks" }
                ],
                "decision_rights": [{ "decision": "Scope change", "decides": "Sponsor", "consulted": "Architecture" }],
                "forums": [{ "name": "Board", "cadence": "monthly" }]
            }))
            .await
            .json();
        let steering_pid = steering["pid"].as_str().expect("pid").to_string();
        assert_eq!(steering["decision_model"], "steering_committee");
        assert_eq!(steering["escalation_path"][1]["body"], "Sponsor");

        request
            .put(&format!("/api/plans/{programme}/governance-framework"))
            .json(&json!({ "framework_pid": steering_pid }))
            .await
            .assert_status_ok();
        let inherited: Value = request.get(&format!("/api/plans/{project}/governance-framework")).await.json();
        assert_eq!(inherited["source"], "inherited");
        assert_eq!(inherited["governance_framework"]["name"], "Programme board");
        // The two families do not leak into each other.
        let wow: Value = request.get(&format!("/api/plans/{project}/ways-of-working")).await.json();
        assert_eq!(wow["source"], "none", "governance does not set ways of working");
        let wow_list: Value = request.get("/api/ways-of-working").await.json();
        assert!(!wow_list.to_string().contains("Programme board"), "a governance record is not in the ways-of-working list");
        assert_eq!(request.get(&format!("/api/ways-of-working/{steering_pid}")).await.status_code(), 404);
        assert_eq!(
            request
                .put(&format!("/api/plans/{project}/ways-of-working"))
                .json(&json!({ "framework_pid": steering_pid }))
                .await
                .status_code(),
            422,
            "a governance record cannot be followed as ways of working"
        );

        // Refusals: a field of the other family, a misspelling, a bad value, a repeated level.
        for (body, why) in [
            (json!({ "name": "X", "wip_limit": 3 }), "a ways-of-working field on governance"),
            (json!({ "name": "X", "decison_model": "sponsor" }), "a misspelt field"),
            (json!({ "name": "X", "decision_model": "coin_toss" }), "an unknown decision model"),
            (json!({ "name": "X", "escalation_path": [{ "level": 1, "body": "a", "trigger": "t" }, { "level": 1, "body": "b", "trigger": "t" }] }), "a repeated level"),
        ] {
            assert_eq!(request.post("/api/governance-frameworks").json(&body).await.status_code(), 422, "{why}");
        }
        assert_eq!(
            request
                .post("/api/governance-frameworks")
                .json(&json!({ "name": "programme BOARD" }))
                .await
                .status_code(),
            409,
            "names are unique per family and owner, ignoring case"
        );
        // The same name in the other family is fine.
        request.post("/api/ways-of-working").json(&json!({ "name": "Programme board" })).await.assert_status_ok();
    })
    .await;
}

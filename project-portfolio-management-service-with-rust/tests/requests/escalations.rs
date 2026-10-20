//! Escalations: raised at a level of the governance escalation path, acknowledged, taken up a level, resolved or
//! withdrawn, with an append-only history; and the link from a red tolerance to an escalation.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
#[allow(clippy::too_many_lines)] // one escalation walked through its whole life
async fn a_project_escalates_along_its_governance_path_and_the_history_is_kept() {
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
        let programme = make_plan("Esc programme", None).await;
        let project = make_plan("Esc project", Some(programme.clone())).await;
        let url = format!("/api/plans/{project}/escalations");

        // With no governance framework, any level 1 to 9 is allowed and nothing is filled in.
        let free: Value = request.post(&url).json(&json!({ "title": "Early warning", "level": 5 })).await.json();
        assert_eq!((free["level"].as_i64(), free["escalated_to"].is_null()), (Some(5), true));
        assert_eq!(request.post(&url).json(&json!({ "title": "x", "level": 10 })).await.status_code(), 422);
        request.post(&format!("{url}/{}/withdraw", free["pid"].as_str().expect("pid"))).json(&json!({})).await.assert_status_ok();

        // The programme's governance framework declares the path; the project inherits it.
        let framework: Value = request
            .post("/api/governance-frameworks")
            .json(&json!({
                "name": "Escalation path",
                "escalation_path": [
                    { "level": 1, "body": "Programme manager", "trigger": "Red for a week" },
                    { "level": 2, "body": "Sponsor", "trigger": "Blocked for two weeks" },
                    { "level": 4, "body": "Board", "trigger": "Needs funding" }
                ]
            }))
            .await
            .json();
        request
            .put(&format!("/api/plans/{programme}/governance-framework"))
            .json(&json!({ "framework_pid": framework["pid"] }))
            .await
            .assert_status_ok();

        // The level defaults to the first, its body is filled in, and a level off the path is refused.
        let raised: Value = request
            .post(&url)
            .json(&json!({ "title": "Cost is beyond tolerance", "reason_kind": "tolerance", "tolerance_dimension": "cost", "respond_by": "2099-01-01" }))
            .await
            .json();
        let e = raised["pid"].as_str().expect("pid").to_string();
        assert_eq!((raised["level"].as_i64(), raised["escalated_to"].as_str(), raised["status"].as_str()), (Some(1), Some("Programme manager"), Some("open")));
        assert_eq!(request.post(&url).json(&json!({ "title": "x", "level": 3 })).await.status_code(), 422, "level 3 is not on the path");
        for (body, why) in [
            (json!({ "title": "" }), "a blank title"),
            (json!({ "title": "x", "reason_kind": "mood" }), "an unknown reason"),
            (json!({ "title": "x", "reason_kind": "risk", "tolerance_dimension": "cost" }), "a dimension on a risk escalation"),
            (json!({ "title": "x", "owner_ref": "nobody" }), "a bad owner"),
            (json!({ "title": "x", "respond_by": "2001-01-01" }), "a past date"),
        ] {
            assert_eq!(request.post(&url).json(&body).await.status_code(), 422, "{why}");
        }

        // Acknowledge once; again is a conflict. Taking it up needs a reason and skips the level the path lacks.
        request.post(&format!("{url}/{e}/acknowledge")).json(&json!({ "note": "Seen" })).await.assert_status_ok();
        assert_eq!(request.post(&format!("{url}/{e}/acknowledge")).json(&json!({})).await.status_code(), 409);
        assert_eq!(request.post(&format!("{url}/{e}/raise-level")).json(&json!({ "note": "short" })).await.status_code(), 422, "a reason is required");
        let up: Value = request.post(&format!("{url}/{e}/raise-level")).json(&json!({ "note": "No answer in a week" })).await.json();
        assert_eq!((up["level"].as_i64(), up["escalated_to"].as_str(), up["status"].as_str()), (Some(2), Some("Sponsor"), Some("open")), "back to open at the new level");
        let up: Value = request.post(&format!("{url}/{e}/raise-level")).json(&json!({ "note": "Needs a funding decision" })).await.json();
        assert_eq!((up["level"].as_i64(), up["escalated_to"].as_str()), (Some(4), Some("Board")), "level 3 does not exist");
        assert_eq!(request.post(&format!("{url}/{e}/raise-level")).json(&json!({ "note": "Nowhere further to go" })).await.status_code(), 409, "the top");

        // Resolve with an outcome and a written response; after that nothing moves.
        assert_eq!(request.post(&format!("{url}/{e}/resolve")).json(&json!({ "outcome": "maybe", "response": "A long enough answer" })).await.status_code(), 422);
        assert_eq!(request.post(&format!("{url}/{e}/resolve")).json(&json!({ "outcome": "decided", "response": "short" })).await.status_code(), 422);
        let done: Value = request
            .post(&format!("{url}/{e}/resolve"))
            .json(&json!({ "outcome": "decided", "response": "Funding approved from contingency" }))
            .await
            .json();
        assert_eq!((done["status"].as_str(), done["outcome"].as_str()), (Some("resolved"), Some("decided")));
        for action in ["acknowledge", "raise-level", "resolve", "withdraw"] {
            assert_eq!(request.post(&format!("{url}/{e}/{action}")).json(&json!({ "note": "a note long enough", "outcome": "decided", "response": "a response long enough" })).await.status_code(), 409, "{action}");
        }

        // The history is the whole story, in order.
        let one: Value = request.get(&format!("{url}/{e}")).await.json();
        let events: Vec<&str> = one["events"].as_array().expect("events").iter().filter_map(|x| x["event"].as_str()).collect();
        assert_eq!(events, ["raised", "acknowledged", "level_raised", "level_raised", "resolved"]);
        assert_eq!(one["events"][3]["level"], 4);

        // Lists and the register.
        let live: Value = request.get(&format!("{url}?status=live")).await.json();
        assert_eq!(live["escalations"].as_array().map(Vec::len), Some(0));
        let all: Value = request.get(&url).await.json();
        assert_eq!(all["escalations"].as_array().map(Vec::len), Some(2), "the withdrawn early warning and the resolved one");
        assert_eq!(request.get(&format!("{url}?status=nonsense")).await.status_code(), 422);
        let register: Value = request.get("/api/escalations?status=resolved").await.json();
        assert!(register["escalations"].as_array().expect("list").iter().any(|x| x["pid"] == e.as_str() && x["plan_name"] == "Esc project"));
        assert_eq!(request.get(&format!("{url}/00000000-0000-4000-8000-000000000000")).await.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_red_tolerance_with_no_escalation_is_flagged_until_someone_escalates_it() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Needs escalating" })).await.json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        request.post(&format!("/api/plans/{p}/risks")).json(&json!({ "title": "Vendor", "probability": 5, "impact": 5 })).await.assert_status_ok();
        let tolerance = |picture: &Value| picture["dimensions"].as_array().expect("dims").iter().find(|d| d["dimension"] == "risk").cloned().expect("risk");
        let set: Value = request.put(&format!("/api/plans/{p}/tolerances/risk")).json(&json!({ "limit": 10 })).await.json();
        let risk = tolerance(&set);
        assert_eq!((risk["status"].as_str(), risk["escalation_needed"].as_bool(), risk["open_escalations"].as_i64()), (Some("red"), Some(true), Some(0)));
        assert_eq!(set["escalation_needed"], true);

        let raised: Value = request
            .post(&format!("/api/plans/{p}/escalations"))
            .json(&json!({ "title": "Vendor risk is beyond tolerance", "reason_kind": "tolerance", "tolerance_dimension": "risk" }))
            .await
            .json();
        let after: Value = request.get(&format!("/api/plans/{p}/tolerances")).await.json();
        let risk = tolerance(&after);
        assert_eq!((risk["escalation_needed"].as_bool(), risk["open_escalations"].as_i64()), (Some(false), Some(1)));
        assert_eq!(after["escalation_needed"], false);

        // Once the escalation is answered, a still-red dimension needs a new one.
        request
            .post(&format!("/api/plans/{p}/escalations/{}/resolve", raised["pid"].as_str().expect("pid")))
            .json(&json!({ "outcome": "guidance_given", "response": "Mitigate it first, then report back" }))
            .await
            .assert_status_ok();
        let again: Value = request.get(&format!("/api/plans/{p}/tolerances")).await.json();
        assert_eq!(tolerance(&again)["escalation_needed"], true);
    })
    .await;
}

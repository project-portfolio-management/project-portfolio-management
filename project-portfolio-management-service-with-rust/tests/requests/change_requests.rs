//! Change requests (IR-4, IR-5): drafting, the whole state machine, linking to issues and risks, the
//! forecast agreeing with the what-if sandbox, a baseline taken because of one, and the portfolio view.
//! Login is off here, so the identity checks (who may decide) are in `tests/change_requests.rs`.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const BOB: &str = "person:3f2c6b7e-0000-4000-8000-0000000000b1";

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn a_change_request_goes_from_draft_to_baseline_and_every_wrong_step_is_refused() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let mk = |title: &'static str, days: i64, start: Option<&'static str>| {
            let (request, p) = (&request, p.clone());
            async move {
                let t: Value = request.post(&format!("/api/plans/{p}/tasks"))
                    .json(&json!({ "title": title, "duration_days": days, "start_date": start })).await.json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let a = mk("A", 3, Some("2026-11-02")).await;
        let b = mk("B", 4, None).await;
        request.post(&format!("/api/plans/{p}/tasks/{a}/relations")).json(&json!({ "kind": "precedes", "to_task_pid": b })).await.assert_status_ok();
        let base = format!("/api/plans/{p}/change-requests");
        let edits = json!([{ "op": "duration", "task_pid": b, "duration_days": 12 }]);

        // Refusals when drafting name every fault.
        let bad = request.post(&base).json(&json!({ "title": " ", "description": "", "reason": "", "kind": "huge",
            "impact_cost_minor": 100, "impact_schedule_days": 99999 })).await;
        assert_eq!(bad.status_code(), 422);
        for word in ["title", "description", "reason", "kind", "currency", "3650"] {
            assert!(bad.text().contains(word), "{word} not named in {}", bad.text());
        }
        assert_eq!(request.post(&base).json(&json!({ "title": "t", "description": "d", "reason": "r", "edits": [] })).await.status_code(), 422, "empty edits");

        // A good draft.
        let cr: Value = request.post(&base).json(&json!({
            "title": "Slip B by eight days", "description": "Supplier is late", "reason": "Delivery moved",
            "kind": "schedule", "impact_schedule_days": 8, "impact_cost_minor": 250_000, "cost_currency": "GBP",
            "edits": edits })).await.json();
        let c = cr["pid"].as_str().unwrap().to_string();
        assert_eq!(cr["status"], "draft");
        assert_eq!(cr["impact"]["schedule_business_days"], 8);
        assert_eq!(cr["impact"]["has_forecast_edits"], true);
        let one = format!("{base}/{c}");

        // Wrong steps from a draft.
        for step in ["review", "implement"] {
            assert_eq!(request.post(&format!("{one}/{step}")).await.status_code(), 409, "{step} from a draft");
        }
        assert_eq!(request.post(&format!("{one}/decision")).json(&json!({ "decision": "approve", "reason": "long enough reason" })).await.status_code(), 409);
        assert_eq!(request.post(&format!("{one}/baseline")).await.status_code(), 409, "not approved");
        assert_eq!(request.post(&format!("{one}/submit")).json(&json!({ "approver_ref": "nobody" })).await.status_code(), 422, "not a person reference");
        assert_eq!(request.get(&format!("{one}/preview")).await.status_code(), 200);

        // The forecast is the sandbox's own answer for the same edits.
        let direct: Value = request.post(&format!("/api/plans/{p}/sandbox/preview")).json(&json!({ "edits": edits })).await.json();
        let via: Value = request.get(&format!("{one}/preview")).await.json();
        assert_eq!(via["label"], "forecast");
        assert_eq!(via["forecast"], direct, "the change request's forecast is the what-if answer");
        assert_eq!(via["stated_impact"]["schedule_business_days"], 8);

        // Edit the draft; then submit.
        let edited: Value = request.put(&one).json(&json!({ "title": "Slip B by eight days (revised)", "description": "Supplier is late",
            "reason": "Delivery moved", "kind": "schedule", "impact_schedule_days": 8, "edits": edits })).await.json();
        assert_eq!(edited["title"], "Slip B by eight days (revised)");
        assert_eq!(edited["impact"]["cost_minor"], Value::Null, "an edit replaces the whole content");
        let sub: Value = request.post(&format!("{one}/submit")).json(&json!({ "approver_ref": BOB })).await.json();
        assert_eq!(sub["status"], "submitted");
        assert_eq!(sub["approver_ref"], BOB);
        assert_eq!(request.put(&one).json(&json!({ "title": "x", "description": "d", "reason": "r" })).await.status_code(), 409, "only a draft is editable");
        assert_eq!(request.post(&format!("{one}/submit")).json(&json!({ "approver_ref": BOB })).await.status_code(), 409, "already submitted");

        // Linking an issue and a risk of the plan.
        let issue: Value = request.post(&format!("/api/plans/{p}/issues")).json(&json!({ "title": "Supplier late", "kind": "concern" })).await.json();
        let issue = issue["pid"].as_str().unwrap().to_string();
        let risk: Value = request.post(&format!("/api/plans/{p}/risks")).json(&json!({ "title": "Late delivery", "probability": 3, "impact": 3 })).await.json();
        let risk = risk["pid"].as_str().unwrap().to_string();
        let other: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Other" })).await.json();
        let other_issue: Value = request.post(&format!("/api/plans/{}/issues", other["pid"].as_str().unwrap())).json(&json!({ "title": "Elsewhere" })).await.json();
        assert_eq!(request.put(&format!("{one}/links")).json(&json!({ "issue_pids": [other_issue["pid"]] })).await.status_code(), 422, "an issue of another plan");
        assert_eq!(request.put(&format!("{one}/links")).json(&json!({ "risk_pids": ["00000000-0000-4000-8000-000000000000"] })).await.status_code(), 422, "an unknown risk");
        let linked: Value = request.put(&format!("{one}/links")).json(&json!({ "issue_pids": [issue], "risk_pids": [risk] })).await.json();
        assert_eq!(linked["issue_pids"], json!([issue]));
        assert_eq!(linked["risk_pids"], json!([risk]));
        let back: Value = request.get(&format!("/api/plans/{p}/issues/{issue}")).await.json();
        assert_eq!(back["change_request_pid"], c.as_str(), "the issue points back");
        // A second request cannot claim the same issue.
        let cr2: Value = request.post(&base).json(&json!({ "title": "Second", "description": "d", "reason": "r" })).await.json();
        let c2 = cr2["pid"].as_str().unwrap().to_string();
        assert_eq!(request.put(&format!("{base}/{c2}/links")).json(&json!({ "issue_pids": [issue] })).await.status_code(), 409);
        // Unlinking clears the pointer.
        request.put(&format!("{one}/links")).json(&json!({})).await.assert_status_ok();
        let cleared: Value = request.get(&format!("/api/plans/{p}/issues/{issue}")).await.json();
        assert_eq!(cleared["change_request_pid"], Value::Null);
        request.put(&format!("{one}/links")).json(&json!({ "issue_pids": [issue], "risk_pids": [risk, risk] })).await.assert_status_ok();

        // Decisions need a real reason, and a known word.
        assert_eq!(request.post(&format!("{one}/decision")).json(&json!({ "decision": "approve", "reason": "ok" })).await.status_code(), 422);
        assert_eq!(request.post(&format!("{one}/decision")).json(&json!({ "decision": "maybe", "reason": "long enough reason" })).await.status_code(), 422);
        let review: Value = request.post(&format!("{one}/review")).await.json();
        assert_eq!(review["status"], "under_review");
        let approved: Value = request.post(&format!("{one}/decision")).json(&json!({ "decision": "approve", "reason": "Within contingency; PMO agrees" })).await.json();
        assert_eq!(approved["status"], "approved");
        assert_eq!(approved["decision_reason"], "Within contingency; PMO agrees");
        assert!(approved["decided_on"].is_string());
        assert_eq!(request.post(&format!("{one}/decision")).json(&json!({ "decision": "reject", "reason": "changed my mind now" })).await.status_code(), 409, "decided once");

        // A baseline because of it: a person chooses to, once, and it names the request.
        let before = request.get(&format!("/api/plans/{p}/schedule-baselines")).await.json::<Value>();
        assert_eq!(before.as_array().unwrap().len(), 0, "nothing re-baselined automatically");
        let taken: Value = request.post(&format!("{one}/baseline")).await.json();
        assert_eq!(taken["baseline_version"], 1);
        let baselines: Value = request.get(&format!("/api/plans/{p}/schedule-baselines")).await.json();
        assert!(baselines[0]["reason"].as_str().unwrap().contains(&c), "{baselines}");
        assert_eq!(request.post(&format!("{one}/baseline")).await.status_code(), 409, "once");

        // Implemented; then nothing more can happen to it.
        let done: Value = request.post(&format!("{one}/implement")).await.json();
        assert_eq!(done["status"], "implemented");
        assert!(done["implemented_on"].is_string());
        for step in ["withdraw", "review", "implement"] {
            assert_eq!(request.post(&format!("{one}/{step}")).await.status_code(), 409, "{step} after implementing");
        }
        assert_eq!(request.put(&format!("{one}/links")).json(&json!({})).await.status_code(), 409, "a closed request is not relinked");

        // Rejection and withdrawal paths.
        let r: Value = request.post(&base).json(&json!({ "title": "R", "description": "d", "reason": "r" })).await.json();
        let r = format!("{base}/{}", r["pid"].as_str().unwrap());
        request.post(&format!("{r}/submit")).json(&json!({ "approver_ref": BOB })).await.assert_status_ok();
        let rej: Value = request.post(&format!("{r}/decision")).json(&json!({ "decision": "reject", "reason": "No budget this year" })).await.json();
        assert_eq!(rej["status"], "rejected");
        assert_eq!(request.post(&format!("{r}/withdraw")).await.status_code(), 409);
        let w: Value = request.post(&format!("{base}/{c2}/withdraw")).await.json();
        assert_eq!(w["status"], "withdrawn");

        // The plan's list and the portfolio view.
        let all: Value = request.get(&base).await.json();
        assert_eq!(all["total"], 3);
        let open: Value = request.get(&format!("{base}?open=true")).await.json();
        assert_eq!(open["total"], 0, "implemented, rejected and withdrawn are not open");
        assert_eq!(request.get(&format!("{base}?status=nonsense")).await.status_code(), 422);
        let folio: Value = request.get("/api/change-requests").await.json();
        assert_eq!(folio["summary"]["by_status"]["implemented"], 1);
        assert_eq!(folio["summary"]["by_status"]["rejected"], 1);
        assert_eq!(folio["summary"]["by_status"]["withdrawn"], 1);
        let c3: Value = request.post(&base).json(&json!({ "title": "Still open", "description": "d", "reason": "r" })).await.json();
        let _ = c3;
        let folio2: Value = request.get("/api/change-requests?open=true").await.json();
        assert_eq!(folio2["change_requests"].as_array().unwrap().len(), 1);
        assert_eq!(folio2["summary"]["oldest_open"]["title"], "Still open");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_director_pack_summary_says_nothing_in_words_then_counts_what_is_recorded() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let url = "/api/director-pack/issues-and-changes";
        let empty: Value = request.get(url).await.json();
        assert_eq!(empty["available"], false);
        assert!(empty["reason"].as_str().is_some());
        assert_eq!(empty["issues"]["statement"], "No issues recorded.");
        assert_eq!(
            empty["change_requests"]["statement"],
            "No change requests recorded."
        );
        assert_eq!(empty["issues"]["total"], 0);

        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        request
            .post(&format!("/api/plans/{p}/issues"))
            .json(&json!({ "title": "Build down", "severity": "critical", "status": "open" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/change-requests"))
            .json(&json!({ "title": "Move go-live", "description": "d", "reason": "r" }))
            .await
            .assert_status_ok();

        let s: Value = request.get(url).await.json();
        assert_eq!(s["available"], true);
        assert_eq!(s["issues"]["open"], 1);
        assert_eq!(s["issues"]["open_by_severity"]["critical"], 1);
        assert_eq!(s["issues"]["statement"], "1 open, 1 of them critical.");
        assert_eq!(s["change_requests"]["by_status"]["draft"], 1);
        assert_eq!(s["change_requests"]["awaiting_decision"], 0);

        for bad in ["0", "367", "x"] {
            assert_eq!(
                request
                    .get(&format!("{url}?window_days={bad}"))
                    .await
                    .status_code(),
                if bad == "x" { 400 } else { 422 },
                "{bad}"
            );
        }
        assert_eq!(
            request
                .get(&format!("{url}?window_days=7"))
                .await
                .status_code(),
            200
        );
    })
    .await;
}

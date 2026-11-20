//! Oversight bulk-read/export request tests (SEC-PPM-2): `GET
//! /api/auditor/evidence-pack` and `GET /api/auditor/trail` each write an
//! `audit_logs` row for the act of reading/exporting, not just for the
//! rows they read — the same posture `agents/share/bulk-import-export.md`
//! §8 requires of a bulk export.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

/// Every `action` recorded in `/api/plans/audit/recent`.
fn recorded_actions(rows: &Value) -> Vec<&str> {
    rows.as_array()
        .expect("audit rows array")
        .iter()
        .map(|r| r["action"].as_str().expect("action"))
        .collect()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn evidence_pack_export_is_audited() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/auditor/evidence-pack?format=json").await;
        assert_eq!(response.status_code(), 200, "evidence pack should succeed");

        let audit: Value = request.get("/api/plans/audit/recent").await.json();
        let actions = recorded_actions(&audit);
        assert!(
            actions.contains(&"oversight_evidence_pack_exported"),
            "the export act itself should be audited, got {actions:?}"
        );
        let row = audit
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["action"] == "oversight_evidence_pack_exported")
            .expect("the exported-audit row");
        assert_eq!(row["snapshot"]["format"], "json");
        assert!(
            row["snapshot"]["from"].is_string() && row["snapshot"]["to"].is_string(),
            "the audit row should name the requested window, got {row}"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn auditor_trail_read_is_audited() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/auditor/trail?limit=10").await;
        assert_eq!(response.status_code(), 200, "auditor trail should succeed");

        let audit: Value = request.get("/api/plans/audit/recent").await.json();
        let actions = recorded_actions(&audit);
        assert!(
            actions.contains(&"oversight_auditor_trail_read"),
            "the read act itself should be audited, got {actions:?}"
        );
        let row = audit
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["action"] == "oversight_auditor_trail_read")
            .expect("the read-audit row");
        assert_eq!(row["snapshot"]["limit"], 10);
        request
            .get("/api/auditor/trail?actor=nobody&action=plan_created&entity=00000000-0000-0000-0000-000000000000&from=2020-01-01&to=2030-01-01T00:00:00Z")
            .await
            .assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn compliance_and_risk_views_name_each_material_finding() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({
                "kind": "Project",
                "name": "Overdue governed plan",
                "target_date": "2020-01-01"
            }))
            .await
            .json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let escalated: Value = request
            .post(&format!("/api/plans/{plan}/risks"))
            .json(&json!({ "title": "Escalated ownerless", "probability": 5, "impact": 5 }))
            .await
            .json();
        request
            .post(&format!(
                "/api/plans/{plan}/risks/{}/escalate",
                escalated["pid"].as_str().unwrap()
            ))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{plan}/risks"))
            .json(&json!({
                "title": "Review overdue",
                "probability": 4,
                "impact": 4,
                "review_date": "2020-01-01"
            }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{plan}/gate-reviews"))
            .json(&json!({ "gate": "g0_concept", "decision": "approved" }))
            .await
            .assert_status_ok();
        request
            .post("/api/plans")
            .json(&json!({
                "kind": "Project",
                "name": "Overdue unreviewed plan",
                "target_date": "2020-01-01"
            }))
            .await
            .assert_status_ok();

        let findings: Value = request
            .get("/api/compliance/findings?review_days=1")
            .await
            .json();
        let rules: Vec<&str> = findings["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|row| row["rule"].as_str())
            .collect();
        for expected in [
            "overdue_item_without_recent_gate_review",
            "escalated_risk_without_owner",
            "risk_past_review_date",
            "gate_approval_without_approver",
        ] {
            assert!(rules.contains(&expected), "missing {expected}: {rules:?}");
        }

        // SAFETY: this database test is serial and the risk view reads the
        // setting per request rather than caching it.
        unsafe {
            std::env::set_var(
                "PROJECT_PORTFOLIO_MANAGEMENT_RISK_APPETITE",
                r#"{"max_open_exposure":1,"max_item_exposure":1}"#,
            );
        }
        let heatmap: Value = request.get("/api/risk/heatmap").await.json();
        assert!(
            heatmap["breaches"]
                .as_array()
                .is_some_and(|rows| rows.len() >= 2)
        );
        unsafe {
            std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_RISK_APPETITE");
        }
    })
    .await;
}

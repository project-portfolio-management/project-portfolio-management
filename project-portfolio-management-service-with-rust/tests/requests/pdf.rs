//! PDF request tests (PL-10): both documents render, are byte-identical when
//! stamped, never carry the people references the masked view hides, and
//! refuse a malformed stamp.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use project_portfolio_management_service::pdf::text_of;
use serde_json::{Value, json};
use serial_test::serial;

const AS_OF: &str = "2026-10-03T12:00:00Z";

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn pdfs_render_deterministically_and_never_leak_what_the_masked_view_hides() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo (phase 1)", "keywords": ["space"],
                           "lead_ref": "person:3f2c6b7e-0000-4000-8000-000000000001",
                           "owner_org_id": "organization:3f2c6b7e-0000-4000-8000-0000000000aa",
                           "owner_org_name": "Acme Corporation Ltd" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Ship", "assignee_ref": "person:3f2c6b7e-0000-4000-8000-000000000002" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/milestones"))
            .json(&json!({ "name": "Beta", "due": "2026-10-20" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/risks"))
            .json(&json!({ "title": "Supplier", "probability": 3, "impact": 4,
                           "owner_ref": "person:3f2c6b7e-0000-4000-8000-000000000003" }))
            .await
            .assert_status_ok();

        // ---- plan summary
        let url = format!("/api/plans/{p}/summary.pdf?as_of={AS_OF}");
        let first = request.get(&url).await;
        first.assert_status_ok();
        let h = first.headers();
        assert_eq!(h.get("content-type").unwrap(), "application/pdf");
        assert_eq!(h.get("x-content-type-options").unwrap(), "nosniff");
        assert!(h.get("content-disposition").unwrap().to_str().unwrap().starts_with("attachment;"));
        let bytes = first.as_bytes().to_vec();
        assert!(bytes.starts_with(b"%PDF-1.4"), "a real PDF");
        let text = text_of(&bytes);
        assert!(text.contains("PLAN SUMMARY: Apollo \\(phase 1\\)"), "{text}");
        assert!(text.contains(&format!("As of: {}", "2026-10-03T12:00:00+00:00")), "the stamp is printed: {text}");
        assert!(text.contains("Tasks: 1 total") && text.contains("Risks recorded: 1"), "{text}");
        assert!(text.contains("Beta due 2026-10-20"));
        // Nothing the masked view hides reaches the text layer.
        for secret in ["3f2c6b7e-0000-4000-8000-000000000001", "3f2c6b7e-0000-4000-8000-000000000002",
                       "3f2c6b7e-0000-4000-8000-000000000003", "Acme Corporation Ltd", "person:"] {
            assert!(!text.contains(secret), "{secret} leaked into the PDF: {text}");
        }
        // Deterministic: the same stamp and data give the same bytes.
        assert_eq!(request.get(&url).await.as_bytes().to_vec(), bytes);
        // A malformed stamp and an unknown plan.
        assert_eq!(request.get(&format!("/api/plans/{p}/summary.pdf?as_of=yesterday")).await.status_code(), 422);
        assert_eq!(request.get("/api/plans/3f2c6b7e-0000-4000-8000-000000000bad/summary.pdf").await.status_code(), 404);

        // ---- board pack
        let pack_url = format!("/api/board/pack.pdf?from=2020-01-01&to=2030-12-31&as_of={AS_OF}");
        let pack = request.get(&pack_url).await;
        pack.assert_status_ok();
        let pack_bytes = pack.as_bytes().to_vec();
        assert!(pack_bytes.starts_with(b"%PDF-1.4"));
        let pack_text = text_of(&pack_bytes);
        assert!(pack_text.contains("BOARD PACK") && pack_text.contains("Window: 2020-01-01"), "{pack_text}");
        assert!(pack_text.contains("health_now:"), "{pack_text}");
        assert_eq!(request.get(&pack_url).await.as_bytes().to_vec(), pack_bytes, "stamped ⇒ byte-identical");
        assert_eq!(request.get("/api/board/pack.pdf?as_of=nope").await.status_code(), 422);
    })
    .await;
}

//! The access review (RG-9): standing grants only, oldest first, flagged by age in calendar days,
//! filtered by plan, and available as a spreadsheet.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const ADA: &str = "person:3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "person:3f2c6b7e-0000-4000-8000-00000000000b";

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_review_lists_only_standing_grants_oldest_first() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let record = |person: &str, access: &str, action: &str, on: &str| {
            json!({ "person_ref": person, "access": access, "action": action,
                    "recorded_on": on, "note": "=SUM(1)" })
        };
        for body in [
            record(ADA, "repo write", "granted", "2025-01-10"),
            record(BOB, "repo write", "granted", "2026-10-01"),
            record(BOB, "wiki editor", "granted", "2025-06-01"),
            record(BOB, "wiki editor", "revoked", "2026-02-01"),
        ] {
            request
                .post(&format!("/api/plans/{p}/access-records"))
                .json(&body)
                .await
                .assert_status_ok();
        }

        let review: Value = request
            .get(&format!("/api/access-review?plan={p}"))
            .await
            .json();
        assert_eq!(review["standing"], 2, "{review}");
        assert_eq!(review["review_due"], 1);
        let grants = review["grants"].as_array().unwrap();
        assert_eq!(grants[0]["person_ref"], ADA, "oldest first");
        assert_eq!(grants[0]["review_due"], true);
        assert!(grants[0]["age_calendar_days"].as_i64().unwrap() > 365);
        assert_eq!(grants[1]["person_ref"], BOB);
        assert_eq!(grants[1]["review_due"], false);

        // A stricter threshold flags both.
        let strict: Value = request
            .get(&format!("/api/access-review?plan={p}&older_than_days=0"))
            .await
            .json();
        assert_eq!(strict["review_due"], 2);

        // The spreadsheet defuses a formula and leaves a decision column.
        let csv = request
            .get(&format!("/api/access-review?plan={p}&format=csv"))
            .await;
        csv.assert_status_ok();
        let text = csv.text();
        assert!(text.starts_with("plan_pid,person_ref,access,"), "{text}");
        assert!(text.contains("decision (keep/revoke)"));
        assert!(text.contains("'=SUM(1)"), "formula not defused: {text}");
        assert!(!text.contains(",=SUM(1)"));

        // Bad parameters are refused.
        assert_eq!(
            request
                .get("/api/access-review?older_than_days=-1")
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .get("/api/access-review?format=xml")
                .await
                .status_code(),
            422
        );
    })
    .await;
}

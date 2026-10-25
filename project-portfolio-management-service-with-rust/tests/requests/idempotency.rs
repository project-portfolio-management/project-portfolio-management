//! Idempotency keys (BR-1): a retried `POST` with the same `Idempotency-Key` returns the first answer and writes
//! nothing twice; a different request under the same key is refused; a refused request releases its key.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn count_named(request: &axum_test::TestServer, name: &str) -> usize {
    let list: Value = request.get("/api/plans").await.json();
    list.as_array()
        .unwrap()
        .iter()
        .filter(|p| p["name"] == name)
        .count()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_retried_post_returns_the_first_answer_and_writes_once() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let body = json!({ "kind": "Project", "name": "Idempotent plan" });

        // ---- without a key two identical posts make two plans (nothing changes for callers who send none)
        request
            .post("/api/plans")
            .json(&body)
            .await
            .assert_status_ok();
        request
            .post("/api/plans")
            .json(&body)
            .await
            .assert_status_ok();
        assert_eq!(count_named(&request, "Idempotent plan").await, 2);

        // ---- with a key the second is a replay
        let body = json!({ "kind": "Project", "name": "Keyed plan" });
        let first = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "key-1")
            .json(&body)
            .await;
        first.assert_status_ok();
        assert!(
            first.maybe_header("idempotent-replayed").is_none(),
            "the first answer is not a replay"
        );
        let first_json: Value = first.json();
        let again = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "key-1")
            .json(&body)
            .await;
        again.assert_status_ok();
        assert_eq!(again.header("idempotent-replayed"), "true");
        let again_json: Value = again.json();
        assert_eq!(first_json["pid"], again_json["pid"], "the same answer");
        assert_eq!(
            count_named(&request, "Keyed plan").await,
            1,
            "one plan, not two"
        );

        // ---- the same key for a different request is refused, and writes nothing
        let other = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "key-1")
            .json(&json!({ "kind": "Project", "name": "A different plan" }))
            .await;
        assert_eq!(other.status_code(), 409);
        assert_eq!(other.json::<Value>()["error"], "idempotency_key_reused");
        assert_eq!(count_named(&request, "A different plan").await, 0);

        // ---- the same key on another path is a different request too
        let elsewhere = request
            .post(&format!(
                "/api/plans/{}/tasks",
                first_json["pid"].as_str().unwrap()
            ))
            .add_header("Idempotency-Key", "key-1")
            .json(&json!({ "title": "t" }))
            .await;
        assert_eq!(elsewhere.status_code(), 409);

        // ---- a refused request releases its key, so the corrected request runs
        let bad = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "key-2")
            .json(&json!({ "kind": "Project", "name": "" }))
            .await;
        assert!(
            bad.status_code().is_client_error(),
            "an empty name is refused"
        );
        let fixed = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "key-2")
            .json(&json!({ "kind": "Project", "name": "Corrected plan" }))
            .await;
        fixed.assert_status_ok();
        assert!(fixed.maybe_header("idempotent-replayed").is_none());
        assert_eq!(count_named(&request, "Corrected plan").await, 1);

        // ---- a write that is not a plan: a meeting, retried, is one meeting
        let plan = first_json["pid"].as_str().unwrap().to_string();
        let meeting = json!({ "title": "Retried meeting", "scheduled_on": "2026-12-01" });
        for _ in 0..3 {
            request
                .post(&format!("/api/plans/{plan}/meetings"))
                .add_header("Idempotency-Key", "meeting-1")
                .json(&meeting)
                .await
                .assert_status_ok();
        }
        let meetings: Value = request
            .get(&format!("/api/plans/{plan}/meetings"))
            .await
            .json();
        assert_eq!(meetings.as_array().unwrap().len(), 1);

        // ---- a request body over 1 MiB cannot carry a key
        let huge = json!({ "kind": "Project", "name": "x".repeat(1_100_000) });
        let refused = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "key-3")
            .json(&huge)
            .await;
        assert_eq!(refused.status_code(), 422);
        assert_eq!(
            refused.json::<Value>()["error"],
            "idempotency_body_too_large"
        );

        // ---- a GET ignores the header
        request
            .get("/api/plans")
            .add_header("Idempotency-Key", "key-1")
            .await
            .assert_status_ok();
    })
    .await;
}

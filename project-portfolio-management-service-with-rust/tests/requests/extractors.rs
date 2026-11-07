//! A bad body, path or query answers a problem whose `detail` says which part was wrong and quotes nothing the caller
//! sent (spec §8.3a): the handlers take `axa::extract`'s extractors, not Axum's.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

/// The marker each probe sends, which no answer may echo.
const MARKER: &str = "zq-secret-7731";

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with `cargo test -- --ignored`"]
async fn a_rejected_body_path_or_query_is_a_problem_that_quotes_nothing() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let created: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Extractors" }))
            .await
            .json();
        let p = created["pid"].as_str().expect("pid").to_string();

        let check = |response: axum_test::TestResponse, status: u16, detail: &str| {
            assert_eq!(
                response.status_code().as_u16(),
                status,
                "{}",
                response.text()
            );
            assert_eq!(
                response.header("content-type"),
                "application/problem+json",
                "{}",
                response.text()
            );
            let text = response.text();
            assert!(
                !text.contains(MARKER),
                "the answer quotes the request: {text}"
            );
            let body: Value = response.json();
            assert_eq!(body["status"], status);
            assert_eq!(body["detail"], detail, "{body}");
        };

        // A body that is not JSON.
        check(
            request
                .post("/api/plans")
                .content_type("application/json")
                .bytes(format!("{{\"name\": \"{MARKER}").into())
                .await,
            400,
            "The request body is not valid JSON",
        );
        // A path value that is not a UUID.
        check(
            request
                .put(&format!("/api/skill-pools/{MARKER}"))
                .json(&json!({}))
                .await,
            400,
            "A value in the URL is not valid",
        );
        // A query value that is not a UUID.
        check(
            request
                .get(&format!("/api/plans/{p}/burndown?sprint={MARKER}"))
                .await,
            400,
            "A query parameter is not valid",
        );
    })
    .await;
}

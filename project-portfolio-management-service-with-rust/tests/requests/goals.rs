//! Goal sub-resource request tests (FR-12): goals are addressed by position
//! and every write goes through the plan, so the matchable payload and the
//! sub-resource view cannot diverge.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn goals_are_added_replaced_and_removed_by_position_through_the_plan() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Lunar lander" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let path = format!("/api/plans/{p}/goals");

        let none: Value = request.get(&path).await.json();
        assert_eq!(none, json!([]), "a new plan has no goals");

        let one: Value = request
            .post(&path)
            .json(&json!({ "title": "Land on the Moon", "target_date": "2030-01-01" }))
            .await
            .json();
        assert_eq!(one[0]["index"], 0);
        assert_eq!(one[0]["title"], "Land on the Moon");
        let two: Value = request
            .post(&path)
            .json(&json!({ "title": "Return safely", "status": "NotStarted" }))
            .await
            .json();
        assert_eq!(two.as_array().map(Vec::len), Some(2));
        assert_eq!(two[1]["index"], 1);

        // The plan itself holds the same goals: one source of truth.
        let stored: Value = request.get(&format!("/api/plans/{p}")).await.json();
        assert_eq!(stored["goals"].as_array().map(Vec::len), Some(2));
        assert_eq!(stored["goals"][1]["title"], "Return safely");

        let replaced: Value = request
            .put(&format!("{path}/1"))
            .json(&json!({ "title": "Return crew safely", "status": "InProgress" }))
            .await
            .json();
        assert_eq!(replaced[1]["title"], "Return crew safely");
        assert_eq!(replaced[1]["status"], "InProgress");

        assert_eq!(
            request
                .post(&path)
                .json(&json!({ "title": "   " }))
                .await
                .status_code(),
            422,
            "a blank title is refused"
        );
        assert_eq!(
            request
                .get(&path)
                .await
                .json::<Value>()
                .as_array()
                .map(Vec::len),
            Some(2),
            "the refused goal was not stored"
        );
        assert_eq!(
            request
                .put(&format!("{path}/9"))
                .json(&json!({ "title": "Nowhere" }))
                .await
                .status_code(),
            404
        );
        assert_eq!(
            request
                .post(&format!("{path}/9/delete"))
                .await
                .status_code(),
            404
        );
        let unknown = "00000000-0000-4000-8000-000000000000";
        assert_eq!(
            request
                .get(&format!("/api/plans/{unknown}/goals"))
                .await
                .status_code(),
            404
        );

        // CF-2: two people read the list; the first removes goal 0, so the second's
        // "goal 0" is now a different goal, and their If-Match is refused rather than
        // deleting it.
        let read = request.get(&path).await;
        let tag = read.header("etag").to_str().expect("etag").to_string();
        assert!(tag.starts_with("W/\""), "{tag}");
        let stale = request
            .put(&format!("{path}/0"))
            .add_header(
                axum::http::header::IF_MATCH,
                axum::http::HeaderValue::from_str("W/\"0000000000000000\"").unwrap(),
            )
            .json(&json!({ "title": "Edited from an old read" }))
            .await;
        assert_eq!(stale.status_code(), 412, "{}", stale.text());
        let first = request
            .post(&format!("{path}/0/delete"))
            .add_header(
                axum::http::header::IF_MATCH,
                axum::http::HeaderValue::from_str(&tag).unwrap(),
            )
            .await;
        first.assert_status_ok();
        assert_ne!(
            first.header("etag").to_str().unwrap(),
            tag,
            "the list changed, so its tag did"
        );
        let second = request
            .delete(&format!("{path}/0"))
            .add_header(
                axum::http::header::IF_MATCH,
                axum::http::HeaderValue::from_str(&tag).unwrap(),
            )
            .await;
        assert_eq!(second.status_code(), 412, "{}", second.text());
        // Removing the first goal moved the second down to position 0, and the 412 deleted nothing.
        let rest: Value = request.get(&path).await.json();
        assert_eq!(rest.as_array().map(Vec::len), Some(1));
        assert_eq!(rest[0]["index"], 0);
        assert_eq!(rest[0]["title"], "Return crew safely");
        let rest: Value = request.delete(&format!("{path}/0")).await.json();
        assert_eq!(rest, json!([]));
    })
    .await;
}

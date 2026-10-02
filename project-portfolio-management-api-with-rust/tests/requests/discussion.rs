//! Comment and reaction request tests (LT-4): threads on several subject
//! kinds, notification fan-out (the owner, never the author, once), edits
//! that keep history, soft delete, and idempotent reactions with every
//! kind counted.
//!
//! Runs with authentication off, so the author-only rule is pinned by the
//! unit test `discussion::tests::only_the_author_may_change_a_comment`.
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const ASSIGNEE: &str = "person:3f2c6b7e-0000-4000-8000-0000000000aa";

async fn plan(request: &axum_test::TestServer) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": "Project", "name": "Apollo" }))
        .await
        .json();
    created["pid"].as_str().expect("pid").to_string()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// A comment on a task notifies the assignee once; the thread is ordered;
// an edit keeps the previous body; a delete hides the comment.
async fn comments_thread_notify_edit_and_delete() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request).await;
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Draft", "assignee_ref": ASSIGNEE }))
            .await
            .json();
        let t = task["pid"].as_str().expect("task pid").to_string();

        let c1: Value = request
            .post("/api/comments")
            .json(&json!({ "subject_kind": "task", "subject_pid": t, "body": "first" }))
            .await
            .json();
        request
            .post("/api/comments")
            .json(&json!({ "subject_kind": "task", "subject_pid": t, "body": "second" }))
            .await
            .assert_status_ok();
        let thread: Value = request
            .get(&format!("/api/comments?subject_kind=task&subject_pid={t}"))
            .await
            .json();
        let bodies: Vec<&str> = thread
            .as_array()
            .expect("thread")
            .iter()
            .map(|c| c["body"].as_str().expect("body"))
            .collect();
        assert_eq!(bodies, ["first", "second"], "oldest first");

        let inbox: Value = request
            .get(&format!("/api/notifications?recipient={ASSIGNEE}"))
            .await
            .json();
        assert_eq!(
            inbox.as_array().expect("inbox").len(),
            2,
            "one per comment for the assignee, never a duplicate within a comment"
        );

        let c1 = c1["pid"].as_str().expect("comment pid");
        let edited: Value = request
            .put(&format!("/api/comments/{c1}"))
            .json(&json!({ "body": "first (edited)" }))
            .await
            .json();
        assert_eq!(edited["body"], "first (edited)");
        assert!(edited["edited_at"].is_string());
        let history: Value = request
            .get(&format!("/api/comments/{c1}/history"))
            .await
            .json();
        assert_eq!(history[0]["previous_body"], "first", "edit history kept");

        // The POST twin of DELETE.
        request
            .post(&format!("/api/comments/{c1}/delete"))
            .await
            .assert_status_ok();
        let after: Value = request
            .get(&format!("/api/comments?subject_kind=task&subject_pid={t}"))
            .await
            .json();
        assert_eq!(after.as_array().expect("thread").len(), 1);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// Invalid input is refused; an unknown subject is 404; every subject
// kind we can create here accepts a comment.
async fn comment_validation_and_subject_resolution() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request).await;
        let unknown = "3f2c6b7e-0000-4000-8000-000000000bad";
        for (body, want) in [
            (
                json!({ "subject_kind": "nope", "subject_pid": p, "body": "x" }),
                422,
            ),
            (
                json!({ "subject_kind": "plan", "subject_pid": p, "body": "  " }),
                422,
            ),
            (
                json!({ "subject_kind": "plan", "subject_pid": unknown, "body": "x" }),
                404,
            ),
            (
                json!({ "subject_kind": "task", "subject_pid": p, "body": "x" }),
                404,
            ),
        ] {
            assert_eq!(
                request
                    .post("/api/comments")
                    .json(&body)
                    .await
                    .status_code(),
                want,
                "{body}"
            );
        }
        for kind in ["plan"] {
            request
                .post("/api/comments")
                .json(&json!({ "subject_kind": kind, "subject_pid": p, "body": "ok" }))
                .await
                .assert_status_ok();
        }
        let idea: Value = request
            .post("/api/ideas")
            .json(&json!({ "title": "An idea" }))
            .await
            .json();
        request
            .post("/api/comments")
            .json(&json!({ "subject_kind": "idea", "subject_pid": idea["pid"], "body": "ok" }))
            .await
            .assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// A repeat reaction is idempotent; every kind is counted (zero included);
// removing is idempotent; an unknown kind is refused.
async fn reactions_are_idempotent_and_every_kind_is_counted() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request).await;
        let react = json!({ "subject_kind": "plan", "subject_pid": p, "kind": "heart" });
        let first: Value = request.post("/api/reactions").json(&react).await.json();
        let again: Value = request.post("/api/reactions").json(&react).await.json();
        assert_eq!(first["created"], true);
        assert_eq!(again["created"], false, "a repeat writes nothing");

        let got: Value = request
            .get(&format!("/api/reactions?subject_kind=plan&subject_pid={p}"))
            .await
            .json();
        assert_eq!(got["counts"]["heart"], 1);
        assert_eq!(
            got["counts"]["celebrate"], 0,
            "unused kind reported at zero"
        );
        assert_eq!(got["mine"], json!(["heart"]));

        assert_eq!(
            request
                .post("/api/reactions")
                .json(&json!({ "subject_kind": "plan", "subject_pid": p, "kind": "poop" }))
                .await
                .status_code(),
            422
        );
        let remove = format!("/api/reactions?subject_kind=plan&subject_pid={p}&kind=heart");
        request.delete(&remove).await.assert_status_ok();
        // Idempotent, and the POST twin behaves the same.
        request
            .post("/api/reactions/remove")
            .json(&react)
            .await
            .assert_status_ok();
        let after: Value = request
            .get(&format!("/api/reactions?subject_kind=plan&subject_pid={p}"))
            .await
            .json();
        assert_eq!(after["counts"]["heart"], 0);
    })
    .await;
}

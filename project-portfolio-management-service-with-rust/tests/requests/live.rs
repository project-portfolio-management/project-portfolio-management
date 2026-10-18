//! Live-update request tests (PL-3): an event reaches a connected client,
//! a reconnect with `Last-Event-ID` is replayed, other plans' events are not
//! delivered, an idle stream closes, and an unknown plan is 404.
//!
//! The stream is infinite by design, so the tests use the `limit` / `idle_ms`
//! parameters to make it finite. `#[ignore]`d: needs PostgreSQL.

use std::time::Duration;

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer, name: &str) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": "Project", "name": name }))
        .await
        .json();
    created["pid"].as_str().expect("pid").to_string()
}

async fn task(request: &axum_test::TestServer, p: &str, title: &str) -> String {
    let t: Value = request
        .post(&format!("/api/plans/{p}/tasks"))
        .json(&json!({ "title": title }))
        .await
        .json();
    t["pid"].as_str().expect("task pid").to_string()
}

async fn move_after(request: &axum_test::TestServer, p: &str, t: &str, status: &str) {
    tokio::time::sleep(Duration::from_millis(150)).await;
    request
        .patch(&format!("/api/plans/{p}/tasks/{t}"))
        .json(&json!({ "status": status }))
        .await
        .assert_status_ok();
}

fn event_id(body: &str) -> u64 {
    body.lines()
        .find_map(|l| l.strip_prefix("id: ").or_else(|| l.strip_prefix("id:")))
        .and_then(|v| v.trim().parse().ok())
        .expect("an id: line")
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_connected_client_sees_a_move_and_a_reconnect_is_replayed() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Apollo").await;
        let other = plan(&request, "Other").await;
        let t = task(&request, &p, "Ship").await;
        let o = task(&request, &other, "Elsewhere").await;
        let live = format!("/api/plans/{p}/live");

        // A client connected before the move sees it.
        let (resp, ()) = tokio::join!(
            request.get(&format!("{live}?limit=1&idle_ms=5000")),
            move_after(&request, &p, &t, "in_progress"),
        );
        resp.assert_status_ok();
        assert!(
            resp.headers()
                .get("content-type")
                .is_some_and(|v| v.to_str().unwrap().starts_with("text/event-stream"))
        );
        let body = resp.text();
        assert!(
            body.contains("event: task_moved") || body.contains("event:task_moved"),
            "{body}"
        );
        assert!(body.contains(&t), "the payload names the task: {body}");
        let id = event_id(&body);

        // Reconnecting with Last-Event-ID = id - 1 replays that event.
        let replay = request
            .get(&format!("{live}?limit=1&idle_ms=2000"))
            .add_header(
                axum::http::HeaderName::from_static("last-event-id"),
                axum::http::HeaderValue::from_str(&(id - 1).to_string()).unwrap(),
            )
            .await;
        assert_eq!(event_id(&replay.text()), id, "the missed event is replayed");
        // Caught up: nothing is owed, so an idle stream closes empty.
        let caught_up = request
            .get(&format!("{live}?limit=1&idle_ms=300"))
            .add_header(
                axum::http::HeaderName::from_static("last-event-id"),
                axum::http::HeaderValue::from_str(&id.to_string()).unwrap(),
            )
            .await;
        caught_up.assert_status_ok();
        assert!(
            !caught_up.text().contains("event:"),
            "nothing new: {}",
            caught_up.text()
        );

        // Another plan's events are not delivered here.
        let (isolated, ()) = tokio::join!(
            request.get(&format!("{live}?limit=1&idle_ms=600")),
            move_after(&request, &other, &o, "in_progress"),
        );
        assert!(!isolated.text().contains("event:"), "{}", isolated.text());

        // A task created and a comment added are announced too.
        let (resp, ()) = tokio::join!(
            request.get(&format!("{live}?limit=2&idle_ms=5000")),
            async {
                tokio::time::sleep(Duration::from_millis(150)).await;
                let t2 = task(&request, &p, "Another").await;
                request
                    .post("/api/comments")
                    .json(&json!({ "subject_kind": "task", "subject_pid": t2, "body": "hi" }))
                    .await
                    .assert_status_ok();
            }
        );
        let text = resp.text();
        assert!(
            text.contains("task_created") && text.contains("comment_added"),
            "{text}"
        );

        // An unknown plan is 404 before any event.
        assert_eq!(
            request
                .get("/api/plans/3f2c6b7e-0000-4000-8000-000000000bad/live?limit=1&idle_ms=100")
                .await
                .status_code(),
            404
        );
    })
    .await;
}

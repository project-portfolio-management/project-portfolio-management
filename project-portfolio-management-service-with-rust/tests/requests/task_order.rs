//! Sibling order of tasks: `PUT /api/plans/{pid}/task-order` sets the order of one parent's children, and task
//! lists come back in it. The body must name exactly the current children.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn siblings_are_reordered_per_parent_and_the_body_must_be_exact() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Work breakdown" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let make = |title: &str, parent: Option<&str>| {
            let body = json!({ "title": title, "parent_task_pid": parent });
            let (request, path) = (&request, format!("/api/plans/{p}/tasks"));
            async move {
                let t: Value = request.post(&path).json(&body).await.json();
                t["pid"].as_str().expect("task pid").to_string()
            }
        };
        let a = make("A", None).await;
        let b = make("B", None).await;
        let c = make("C", None).await;
        let a1 = make("A1", Some(&a)).await;
        let a2 = make("A2", Some(&a)).await;
        let order_url = format!("/api/plans/{p}/task-order");
        let titles = |parent: Option<String>| {
            let (request, path) = (&request, format!("/api/plans/{p}/tasks"));
            async move {
                let list: Value = request.get(&path).await.json();
                list["tasks"]
                    .as_array()
                    .expect("tasks")
                    .iter()
                    .filter(|t| t["parent_task_pid"].as_str().map(str::to_string) == parent)
                    .map(|t| t["title"].as_str().expect("title").to_string())
                    .collect::<Vec<_>>()
            }
        };

        assert_eq!(titles(None).await, ["A", "B", "C"], "by age until ordered");
        request
            .put(&order_url)
            .json(&json!({ "order": [c, a, b] }))
            .await
            .assert_status_ok();
        assert_eq!(titles(None).await, ["C", "A", "B"]);
        assert_eq!(
            titles(Some(a.clone())).await,
            ["A1", "A2"],
            "another parent is untouched"
        );

        request
            .put(&order_url)
            .json(&json!({ "parent_task_pid": a, "order": [a2, a1] }))
            .await
            .assert_status_ok();
        assert_eq!(titles(Some(a.clone())).await, ["A2", "A1"]);
        assert_eq!(
            titles(None).await,
            ["C", "A", "B"],
            "the top level is untouched"
        );

        // A new sibling has no position, so it follows the positioned ones.
        make("D", None).await;
        assert_eq!(titles(None).await, ["C", "A", "B", "D"]);

        for (body, status, why) in [
            (json!({ "order": [c, a, b] }), 409, "a child missing (D)"),
            (json!({ "order": [c, a, b, b] }), 422, "a task twice"),
            (
                json!({ "parent_task_pid": a, "order": [a2, a1, b] }),
                409,
                "a task of another parent",
            ),
            (
                json!({ "parent_task_pid": "00000000-0000-4000-8000-000000000000", "order": [] }),
                422,
                "an unknown parent",
            ),
        ] {
            assert_eq!(
                request.put(&order_url).json(&body).await.status_code(),
                status,
                "{why}"
            );
        }
        assert_eq!(
            titles(None).await,
            ["C", "A", "B", "D"],
            "refusals change nothing"
        );
        assert_eq!(
            request
                .put("/api/plans/00000000-0000-4000-8000-000000000000/task-order")
                .json(&json!({ "order": [] }))
                .await
                .status_code(),
            404
        );
    })
    .await;
}

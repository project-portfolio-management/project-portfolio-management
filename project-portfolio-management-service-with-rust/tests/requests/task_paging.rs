//! The plan's task list pages (RG-15): a page of tasks, counts and total over every matching task,
//! the standard headers, a stable order, and the old shape (no parameters) still working.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_task_list_pages_and_keeps_counts_over_everything() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let mut pids = Vec::new();
        for i in 0..5 {
            let t: Value = request
                .post(&format!("/api/plans/{p}/tasks"))
                .json(&json!({ "title": format!("T{i}") }))
                .await
                .json();
            pids.push(t["pid"].as_str().unwrap().to_string());
        }
        request
            .patch(&format!("/api/plans/{p}/tasks/{}", pids[0]))
            .json(&json!({ "status": "done" }))
            .await
            .assert_status_ok();

        // No parameters: the old shape, everything (5 is under the cap), not truncated.
        let all: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        assert_eq!(all["tasks"].as_array().unwrap().len(), 5);
        assert_eq!(all["total"], 5);
        assert_eq!(all["truncated"], false);
        assert_eq!(all["counts"]["done"], 1);
        assert_eq!(all["counts"]["todo"], 4);

        // A page: two tasks, but the counts and total still cover all five.
        let r = request.get(&format!("/api/plans/{p}/tasks?limit=2")).await;
        assert_eq!(r.header("x-total-count"), "5");
        assert_eq!(r.header("x-limit"), "2");
        assert_eq!(r.header("x-offset"), "0");
        let page: Value = r.json();
        assert_eq!(page["tasks"].as_array().unwrap().len(), 2);
        assert_eq!(page["total"], 5);
        assert_eq!(page["truncated"], true);
        assert_eq!(
            page["counts"]["done"], 1,
            "counts are over every task, not the page"
        );
        assert_eq!(page["tasks"][0]["title"], "T0");

        // The last page, and past the end.
        let last: Value = request
            .get(&format!("/api/plans/{p}/tasks?limit=2&offset=4"))
            .await
            .json();
        assert_eq!(last["tasks"].as_array().unwrap().len(), 1);
        assert_eq!(last["tasks"][0]["title"], "T4");
        assert_eq!(last["truncated"], false);
        let past: Value = request
            .get(&format!("/api/plans/{p}/tasks?offset=9000"))
            .await
            .json();
        assert!(past["tasks"].as_array().unwrap().is_empty());
        assert_eq!(past["total"], 5);

        // Walking the pages yields every task once, in order.
        let mut titles = Vec::new();
        for offset in [0, 2, 4] {
            let page: Value = request
                .get(&format!("/api/plans/{p}/tasks?limit=2&offset={offset}"))
                .await
                .json();
            for t in page["tasks"].as_array().unwrap() {
                titles.push(t["title"].as_str().unwrap().to_string());
            }
        }
        assert_eq!(titles, ["T0", "T1", "T2", "T3", "T4"]);

        // Bad parameters.
        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/tasks?limit=abc"))
                .await
                .status_code(),
            400
        );
        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/tasks?offset=999999999"))
                .await
                .status_code(),
            400
        );
    })
    .await;
}

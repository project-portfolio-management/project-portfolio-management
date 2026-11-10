//! The plan's task list pages (RG-15): a page of tasks, counts and total over every matching task,
//! a signed next link that walks every task once in a stable order, and the old shape (no parameters)
//! still working.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with `cargo test -- --ignored`"]
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
        assert!(r.maybe_header("x-total-count").is_none());
        let page: Value = r.json();
        assert_eq!(page["tasks"].as_array().unwrap().len(), 2);
        assert_eq!(page["total"], 5);
        assert_eq!(page["truncated"], true);
        assert_eq!(page["limit"], 2);
        assert_eq!(
            page["counts"]["done"], 1,
            "counts are over every task, not the page"
        );
        assert_eq!(page["tasks"][0]["title"], "T0");

        // Walking the next links yields every task once, in order, and the last page has no next.
        let mut titles = Vec::new();
        let mut url = Some(format!("/api/plans/{p}/tasks?limit=2"));
        let mut pages = 0;
        while let Some(at) = url {
            let r = request.get(&at).await;
            r.assert_status_ok();
            let page: Value = r.json();
            for t in page["tasks"].as_array().unwrap() {
                titles.push(t["title"].as_str().unwrap().to_string());
            }
            url = r.maybe_header("link").map(|l| {
                let l = l.to_str().unwrap();
                l.strip_prefix('<')
                    .unwrap()
                    .split_once(">; rel=\"next\"")
                    .unwrap()
                    .0
                    .to_string()
            });
            assert_eq!(
                page["truncated"],
                url.is_some(),
                "truncated says a next page follows"
            );
            pages += 1;
        }
        assert_eq!(pages, 3);
        assert_eq!(titles, ["T0", "T1", "T2", "T3", "T4"]);

        // Bad parameters: refused, never clamped or ignored.
        for (query, status) in [
            ("limit=abc", 422),
            ("limit=2001", 422),
            ("cursor=4", 422),
            ("offset=4", 400),
        ] {
            assert_eq!(
                request
                    .get(&format!("/api/plans/{p}/tasks?{query}"))
                    .await
                    .status_code(),
                status,
                "{query}"
            );
        }
        // A cursor from one plan's list does not read another's.
        let other: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Gemini" }))
            .await
            .json();
        let first = request.get(&format!("/api/plans/{p}/tasks?limit=2")).await;
        let link = first.header("link");
        let cursor = link
            .to_str()
            .unwrap()
            .split_once("cursor=")
            .unwrap()
            .1
            .split_once('>')
            .unwrap()
            .0;
        assert_eq!(
            request
                .get(&format!(
                    "/api/plans/{}/tasks?limit=2&cursor={cursor}",
                    other["pid"].as_str().unwrap()
                ))
                .await
                .status_code(),
            422
        );
    })
    .await;
}

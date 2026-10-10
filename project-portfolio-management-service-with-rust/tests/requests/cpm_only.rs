//! `GET /api/plans/{pid}/cpm?only=critical` (RG-16): the critical and near-critical tasks and the
//! dependencies between them, for plans too large to read whole.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn only_critical_keeps_the_critical_tasks_and_their_dependencies() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let mk = |title: &'static str, days: i64, start: Option<&'static str>| {
            let (request, p) = (&request, p.clone());
            async move {
                let t: Value = request
                    .post(&format!("/api/plans/{p}/tasks"))
                    .json(&json!({ "title": title, "duration_days": days, "start_date": start }))
                    .await
                    .json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        // A (3 days) -> B (10 days) -> D (2 days) is the long path; A -> C (1 day) -> D has float.
        let a = mk("A", 3, Some("2026-11-02")).await;
        let b = mk("B", 10, None).await;
        let c = mk("C", 1, None).await;
        let d = mk("D", 2, None).await;
        for (from, to) in [(&a, &b), (&b, &d), (&a, &c), (&c, &d)] {
            request
                .post(&format!("/api/plans/{p}/tasks/{from}/relations"))
                .json(&json!({ "kind": "precedes", "to_task_pid": to }))
                .await
                .assert_status_ok();
        }

        let all: Value = request
            .get(&format!("/api/plans/{p}/cpm?near=0"))
            .await
            .json();
        assert_eq!(all["only"], "all");
        assert_eq!(all["nodes"].as_array().unwrap().len(), 4);
        assert_eq!(all["nodes_total"], 4);
        let all_edges = all["edges"].as_array().unwrap().len();
        assert_eq!(all_edges, 4);

        let only: Value = request
            .get(&format!("/api/plans/{p}/cpm?near=0&only=critical"))
            .await
            .json();
        assert_eq!(only["only"], "critical");
        assert_eq!(
            only["nodes_total"], 4,
            "how many there were before filtering"
        );
        let titles: Vec<&str> = only["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n["title"].as_str().unwrap())
            .collect();
        assert_eq!(titles.len(), 3, "{titles:?}");
        assert!(titles.contains(&"A") && titles.contains(&"B") && titles.contains(&"D"));
        assert!(!titles.contains(&"C"), "C has float");
        // Only the two dependencies between critical tasks remain.
        assert_eq!(only["edges"].as_array().unwrap().len(), 2);
        // The chains and the finish are the same either way.
        assert_eq!(only["project_finish"], all["project_finish"]);
        assert_eq!(only["critical_chains"], all["critical_chains"]);

        // A wide near-critical band brings C back.
        let wide: Value = request
            .get(&format!("/api/plans/{p}/cpm?near=100&only=critical"))
            .await
            .json();
        assert_eq!(wide["nodes"].as_array().unwrap().len(), 4);

        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/cpm?only=nope"))
                .await
                .status_code(),
            422
        );
        let _ = (c, d);
    })
    .await;
}

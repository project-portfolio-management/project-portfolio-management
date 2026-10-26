//! Starter templates request test (BR-13): five are listed, each instantiates to the plans, tasks and milestones it
//! promises, and none can be deleted, revised or saved over.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::starter_templates::starters;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn five_starters_are_listed_instantiate_and_are_not_changed_in_place() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let listed: Value = request.get("/api/plan-templates").await.json();
        let listed = listed.as_array().unwrap();
        assert_eq!(
            listed.iter().filter(|t| t["starter"] == true).count(),
            5,
            "{listed:?}"
        );

        for s in starters() {
            let pid = s.pid.to_string();
            let want_plans = s.snapshot.nodes.len();
            let want_tasks: usize = s.snapshot.nodes.iter().map(|n| n.tasks.len()).sum();
            let want_milestones: usize = s.snapshot.nodes.iter().map(|n| n.milestones.len()).sum();
            let entry = listed
                .iter()
                .find(|t| t["pid"] == pid.as_str())
                .unwrap_or_else(|| panic!("{} is listed", s.name));
            assert_eq!(
                (entry["name"].as_str(), entry["plans"].as_u64()),
                (Some(s.name), Some(want_plans as u64))
            );

            // A start date is required, since a starter carries its dates as offsets.
            let url = format!("/api/plans/from-template/{pid}");
            assert_eq!(
                request.post(&url).json(&json!({})).await.status_code(),
                422,
                "{}",
                s.name
            );

            let made: Value = request
                .post(&url)
                .json(&json!({ "start_date": "2027-01-04", "name": format!("{} (test)", s.name) }))
                .await
                .json();
            assert_eq!(
                made["plans"].as_array().map(Vec::len),
                Some(want_plans),
                "{}: {made}",
                s.name
            );
            assert_eq!(
                made["tasks_created"].as_u64(),
                Some(want_tasks as u64),
                "{}",
                s.name
            );
            assert_eq!(
                made["milestones_created"].as_u64(),
                Some(want_milestones as u64),
                "{}",
                s.name
            );

            // The root plan reads back with its tasks at the initial state and dates counted from the start date.
            let root = made["root_pid"].as_str().unwrap();
            let got: Value = request.get(&format!("/api/plans/{root}")).await.json();
            assert_eq!(got["name"], format!("{} (test)", s.name).as_str());
            assert_eq!(got["start_date"], "2027-01-04");
            let tasks: Value = request
                .get(&format!("/api/plans/{root}/tasks"))
                .await
                .json();
            let tasks = tasks["tasks"].as_array().unwrap();
            assert_eq!(tasks.len(), s.snapshot.nodes[0].tasks.len());
            assert!(tasks.iter().all(|t| t["status"] == "todo"), "{}", s.name);

            // Not changed in place: delete, revise and a saved copy under its name are each refused.
            for r in [
                request
                    .post(&format!("/api/plan-templates/{pid}/delete"))
                    .await,
                request.delete(&format!("/api/plan-templates/{pid}")).await,
                request
                    .post(&format!("/api/plan-templates/{pid}/revise"))
                    .json(&json!({ "from_plan": root }))
                    .await,
            ] {
                assert_eq!(r.status_code(), 422, "{}", s.name);
            }
            assert_eq!(
                request
                    .post(&format!("/api/plans/{root}/save-as-template"))
                    .json(&json!({ "name": s.name.to_uppercase() }))
                    .await
                    .status_code(),
                409,
                "{}: a saved template cannot take a starter's name",
                s.name
            );
        }

        // Still five, after every delete was refused; and a copy saved under another name is an ordinary template.
        let after: Value = request.get("/api/plan-templates").await.json();
        assert_eq!(
            after
                .as_array()
                .unwrap()
                .iter()
                .filter(|t| t["starter"] == true)
                .count(),
            5
        );
    })
    .await;
}

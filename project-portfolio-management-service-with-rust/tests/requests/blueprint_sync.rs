//! Blueprint sync request test (NC-12): a template is revised; two plans made from it see the same changes; one
//! applies them, the other has edited a task and is refused that change until it says to overwrite.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn tasks_of(
    request: &axum_test::TestServer,
    plan: &str,
) -> Vec<(String, String, Option<i64>)> {
    let list: Value = request
        .get(&format!("/api/plans/{plan}/tasks"))
        .await
        .json();
    list["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .map(|t| {
            (
                t["pid"].as_str().unwrap().to_string(),
                t["title"].as_str().unwrap().to_string(),
                t["points"].as_i64(),
            )
        })
        .collect()
}

fn ids(diff: &Value) -> Vec<String> {
    diff["changes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["id"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_template_change_is_applied_to_one_plan_and_refused_for_another_that_edited_the_task() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        // ---- the source plan, saved as a template
        let source: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Playbook" }))
            .await
            .json();
        let source = source["pid"].as_str().unwrap().to_string();
        for (title, points) in [("Plan", 1), ("Build", 3), ("Ship", 2)] {
            request
                .post(&format!("/api/plans/{source}/tasks"))
                .json(&json!({ "title": title, "points": points }))
                .await
                .assert_status_ok();
        }
        let saved: Value = request
            .post(&format!("/api/plans/{source}/save-as-template"))
            .json(&json!({ "name": "Playbook template" }))
            .await
            .json();
        let tpl = saved["pid"].as_str().unwrap().to_string();

        // ---- two plans made from it; Y's people change Build's points
        let mut made = Vec::new();
        for name in ["Plan X", "Plan Y"] {
            let m: Value = request
                .post(&format!("/api/plans/from-template/{tpl}"))
                .json(&json!({ "name": name }))
                .await
                .json();
            made.push(m["root_pid"].as_str().unwrap().to_string());
        }
        let (x, y) = (made[0].clone(), made[1].clone());
        let build_y = tasks_of(&request, &y)
            .await
            .into_iter()
            .find(|t| t.1 == "Build")
            .unwrap()
            .0;
        request
            .put(&format!("/api/plans/{y}/tasks/{build_y}"))
            .json(&json!({ "title": "Build", "points": 8 }))
            .await
            .assert_status_ok();

        // ---- nothing has changed yet
        let diff: Value = request
            .get(&format!("/api/plans/{x}/template-diff"))
            .await
            .json();
        assert!(ids(&diff).is_empty());
        assert_eq!(diff["template_version"], 1);

        // ---- the template is revised: Build is re-estimated, Ship is dropped, Review is new
        let build_src = tasks_of(&request, &source)
            .await
            .into_iter()
            .find(|t| t.1 == "Build")
            .unwrap()
            .0;
        let ship_src = tasks_of(&request, &source)
            .await
            .into_iter()
            .find(|t| t.1 == "Ship")
            .unwrap()
            .0;
        request
            .put(&format!("/api/plans/{source}/tasks/{build_src}"))
            .json(&json!({ "title": "Build", "points": 5 }))
            .await
            .assert_status_ok();
        request
            .delete(&format!("/api/plans/{source}/tasks/{ship_src}"))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{source}/tasks"))
            .json(&json!({ "title": "Review", "points": 1 }))
            .await
            .assert_status_ok();
        let revised: Value = request
            .post(&format!("/api/plan-templates/{tpl}/revise"))
            .json(&json!({ "from_plan": source }))
            .await
            .json();
        assert_eq!(revised["version"], 2);

        // ---- both plans see three changes waiting; Y's edited task is marked
        let waiting: Value = request
            .get(&format!("/api/plan-templates/{tpl}/plans"))
            .await
            .json();
        let waiting = waiting.as_array().unwrap();
        assert_eq!(waiting.len(), 2);
        assert!(
            waiting
                .iter()
                .all(|p| p["changes_waiting"] == 3 && p["template_version"] == 2)
        );
        let dx: Value = request
            .get(&format!("/api/plans/{x}/template-diff"))
            .await
            .json();
        let dy: Value = request
            .get(&format!("/api/plans/{y}/template-diff"))
            .await
            .json();
        assert_eq!(ids(&dx), ["change:1", "remove:2", "add:3"]);
        assert_eq!(ids(&dy), ids(&dx));
        let edited = |d: &Value| -> Vec<bool> {
            d["changes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c["locally_edited"].as_bool().unwrap())
                .collect()
        };
        assert_eq!(edited(&dx), [false, false, false]);
        assert_eq!(
            edited(&dy),
            [true, false, false],
            "only Build was edited by Y's people"
        );

        // ---- X takes everything
        let applied: Value = request
            .post(&format!("/api/plans/{x}/template-sync"))
            .json(&json!({ "accept": ["change:1", "remove:2", "add:3"] }))
            .await
            .json();
        assert_eq!(applied["applied"].as_array().unwrap().len(), 3);
        assert_eq!(applied["remaining"], 0);
        let tx = tasks_of(&request, &x).await;
        let titles: Vec<&str> = tx.iter().map(|t| t.1.as_str()).collect();
        assert!(
            titles.contains(&"Review") && !titles.contains(&"Ship"),
            "{titles:?}"
        );
        assert_eq!(tx.iter().find(|t| t.1 == "Build").unwrap().2, Some(5));
        let dx: Value = request
            .get(&format!("/api/plans/{x}/template-diff"))
            .await
            .json();
        assert!(ids(&dx).is_empty());
        assert_eq!(dx["synced_version"], 2);

        // ---- Y asks for the edited task's change too: refused, the other applied, Build untouched
        let partial: Value = request
            .post(&format!("/api/plans/{y}/template-sync"))
            .json(&json!({ "accept": ["change:1", "add:3"] }))
            .await
            .json();
        assert_eq!(partial["applied"], json!(["add:3"]));
        assert_eq!(partial["refused"][0]["id"], "change:1");
        assert_eq!(
            partial["remaining"], 2,
            "the refused change and the one never asked for"
        );
        let ty = tasks_of(&request, &y).await;
        assert_eq!(
            ty.iter().find(|t| t.1 == "Build").unwrap().2,
            Some(8),
            "Y's own edit stands"
        );
        assert!(ty.iter().any(|t| t.1 == "Review"));
        assert!(
            ty.iter().any(|t| t.1 == "Ship"),
            "removal was not asked for"
        );

        // ---- naming the overwrite applies it
        let forced: Value = request
            .post(&format!("/api/plans/{y}/template-sync"))
            .json(&json!({ "accept": ["change:1"], "overwrite_local": ["change:1"] }))
            .await
            .json();
        assert_eq!(forced["overwritten_local_edits"], json!(["change:1"]));
        assert_eq!(forced["remaining"], 1);
        let ty = tasks_of(&request, &y).await;
        assert_eq!(ty.iter().find(|t| t.1 == "Build").unwrap().2, Some(5));

        // ---- bad requests
        assert_eq!(
            request
                .post(&format!("/api/plans/{y}/template-sync"))
                .json(&json!({ "accept": ["add:99"] }))
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .post(&format!("/api/plans/{y}/template-sync"))
                .json(&json!({ "accept": [], "overwrite_local": ["remove:2"] }))
                .await
                .status_code(),
            422
        );
        // A plan not made from a template has nothing to sync.
        assert_eq!(
            request
                .get(&format!("/api/plans/{source}/template-diff"))
                .await
                .status_code(),
            404
        );
        // A revision that reshapes the tree is refused.
        request
            .put(&format!("/api/plans/{source}"))
            .json(&json!({ "name": "Renamed" }))
            .await;
        let reshaped = request
            .post(&format!("/api/plan-templates/{tpl}/revise"))
            .json(&json!({ "from_plan": source }))
            .await;
        assert_eq!(reshaped.status_code(), 422);
    })
    .await;
}

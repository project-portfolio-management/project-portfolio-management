//! BR-15: the CSV the UI builds from pasted spreadsheet rows (`Task ID,Name,Due Date,Notes`, ids like `paste-1a2b3c4d`)
//! goes through the Asana import: a dry run writes nothing, applying creates the tasks, and pasting the same rows again
//! updates them and creates none.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const CSV: &str = "Task ID,Name,Due Date,Notes\n\
paste-0a1b2c3d,\"Plan, then \"\"go\"\"\",2026-10-05,first draft\n\
paste-4e5f6a7b,Book room,2026-10-06,\n";

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn pasted_rows_import_once_and_a_second_paste_updates_instead_of_duplicating() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Paste" }))
            .await
            .json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let post = |dry: bool| {
            let (request, plan) = (&request, plan.clone());
            async move {
                request
                    .post(&format!("/api/plans/{plan}/import/asana?dry_run={dry}"))
                    .bytes(CSV.as_bytes().to_vec().into())
                    .await
            }
        };
        let count = || {
            let (request, plan) = (&request, plan.clone());
            async move {
                let t: Value = request
                    .get(&format!("/api/plans/{plan}/tasks"))
                    .await
                    .json();
                t["tasks"].as_array().unwrap().len()
            }
        };

        // The dry run says what it would do and writes nothing.
        let r: Value = post(true).await.json();
        assert_eq!(
            (
                r["dry_run"].as_bool(),
                r["tasks_created"].as_i64(),
                r["tasks_updated"].as_i64()
            ),
            (Some(true), Some(2), Some(0)),
            "{r}"
        );
        assert!(
            r["unmapped"].as_array().unwrap().is_empty(),
            "nothing is left out: {r}"
        );
        assert_eq!(count().await, 0);

        // Applying creates both, with the quoted title read back whole and the date kept.
        let r: Value = post(false).await.json();
        assert_eq!(
            (r["tasks_created"].as_i64(), r["tasks_updated"].as_i64()),
            (Some(2), Some(0)),
            "{r}"
        );
        let t: Value = request
            .get(&format!("/api/plans/{plan}/tasks"))
            .await
            .json();
        let tasks = t["tasks"].as_array().unwrap();
        let planned = tasks
            .iter()
            .find(|x| x["title"] == "Plan, then \"go\"")
            .expect("the quoted title");
        assert_eq!(
            planned["deadline"], "2026-10-05",
            "the import stores the date as the deadline"
        );

        // The same paste again: updated, not created, and still two tasks.
        let r: Value = post(false).await.json();
        assert_eq!(
            (r["tasks_created"].as_i64(), r["tasks_updated"].as_i64()),
            (Some(0), Some(2)),
            "{r}"
        );
        assert_eq!(count().await, 2);
    })
    .await;
}

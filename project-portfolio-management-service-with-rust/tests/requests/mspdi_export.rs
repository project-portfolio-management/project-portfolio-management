//! MS Project export (PJ-10): a plan exports as MSPDI XML that the importer reads back, UIDs survive a round
//! trip, sibling order and dependencies come out, and the file says what it does not carry.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const MSPDI: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Project xmlns="http://schemas.microsoft.com/project">
  <Name>Round trip</Name><MinutesPerDay>480</MinutesPerDay>
  <Tasks>
    <Task><UID>0</UID><Name>Round trip</Name><Summary>1</Summary></Task>
    <Task><UID>10</UID><Name>Design</Name><OutlineLevel>1</OutlineLevel><Summary>1</Summary></Task>
    <Task><UID>20</UID><Name>Draft &amp; sketch</Name><OutlineLevel>2</OutlineLevel><Start>2026-10-05T08:00:00</Start><Duration>PT24H0M0S</Duration></Task>
    <Task><UID>30</UID><Name>Review</Name><OutlineLevel>2</OutlineLevel><Duration>PT32H0M0S</Duration>
      <PredecessorLink><PredecessorUID>20</PredecessorUID><Type>1</Type><LinkLag>4800</LinkLag></PredecessorLink></Task>
    <Task><UID>40</UID><Name>Sign-off</Name><OutlineLevel>1</OutlineLevel><Milestone>1</Milestone><Duration>PT0H0M0S</Duration>
      <PredecessorLink><PredecessorUID>30</PredecessorUID><Type>3</Type></PredecessorLink></Task>
  </Tasks>
</Project>"#;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_plan_exports_as_an_ms_project_file_that_reads_back() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let make = |name: &'static str| {
            let request = &request;
            async move {
                let plan: Value = request
                    .post("/api/plans")
                    .json(&json!({ "kind": "Project", "name": name }))
                    .await
                    .json();
                plan["pid"].as_str().expect("pid").to_string()
            }
        };
        let source = make("Round trip").await;
        request
            .post(&format!("/api/plans/{source}/import/mspdi?dry_run=false"))
            .content_type("application/xml")
            .text(MSPDI)
            .await
            .assert_status_ok();

        // A task made here, not imported, after Sign-off in the list, and moved to the front of the top level.
        let tasks: Value = request
            .get(&format!("/api/plans/{source}/tasks"))
            .await
            .json();
        let top: Vec<String> = tasks["tasks"]
            .as_array()
            .expect("tasks")
            .iter()
            .filter(|t| t["parent_task_pid"].is_null())
            .map(|t| t["pid"].as_str().expect("pid").to_string())
            .collect();
        let made: Value = request
            .post(&format!("/api/plans/{source}/tasks"))
            .json(&json!({ "title": "Added here", "duration_days": 2 }))
            .await
            .json();
        let added = made["pid"].as_str().expect("pid").to_string();
        let mut order = vec![added];
        order.extend(top);
        request
            .put(&format!("/api/plans/{source}/task-order"))
            .json(&json!({ "order": order }))
            .await
            .assert_status_ok();

        let response = request
            .get(&format!("/api/plans/{source}/export/mspdi"))
            .await;
        response.assert_status_ok();
        assert_eq!(
            response.header("content-type"),
            "application/xml; charset=utf-8"
        );
        assert!(
            response
                .header("content-disposition")
                .to_str()
                .expect("header")
                .contains("Round-trip.xml")
        );
        let xml = response.text();
        assert!(
            xml.contains("Not carried:"),
            "the file says what it leaves out"
        );
        assert!(xml.contains("Draft &amp; sketch"), "escaped");
        // Imported UIDs survive; the new task gets one above them; the new task is first.
        for uid in [10, 20, 30, 40] {
            assert!(xml.contains(&format!("<UID>{uid}</UID>")), "UID {uid}");
        }
        assert!(
            xml.contains("<UID>41</UID>"),
            "a fresh UID above the largest"
        );
        assert!(
            xml.find("Added here") < xml.find("Design"),
            "stored sibling order"
        );

        // The importer reads our own file: nothing unmapped, every task and link present.
        let target = make("Round trip copy").await;
        let report: Value = request
            .post(&format!("/api/plans/{target}/import/mspdi?dry_run=false"))
            .content_type("application/xml")
            .text(xml)
            .await
            .json();
        assert_eq!(report["tasks_in_file"], 5, "{report}");
        assert_eq!(report["tasks_created"], 5, "{report}");
        assert_eq!(report["links_in_file"], 2, "{report}");
        assert_eq!(report["links_created"], 2, "{report}");
        assert_eq!(report["unmapped"], json!([]), "{report}");

        assert_eq!(
            request
                .get("/api/plans/00000000-0000-4000-8000-000000000000/export/mspdi")
                .await
                .status_code(),
            404
        );
    })
    .await;
}

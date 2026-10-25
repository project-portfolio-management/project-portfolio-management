//! Many MSPDI files in one job (NC-13): three related projects are imported at once into three new plans; the
//! resources they share are listed once, an enterprise field is stored through the caller's mapping, a field with no
//! mapping is reported, and a bad file stops the job before anything is written.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;
use std::fmt::Write;

/// One project: two tasks (the second waits on the first), the named resources assigned to the first task, and the
/// enterprise fields `Cost centre` (mapped by the test) and `Region` (not).
fn project(name: &str, resources: &[(i64, &str)], cost_centre: &str) -> String {
    let (mut res, mut asg) = (String::new(), String::new());
    for (u, n) in resources {
        write!(res, "<Resource><UID>{u}</UID><Name>{n}</Name></Resource>").unwrap();
        write!(
            asg,
            "<Assignment><TaskUID>1</TaskUID><ResourceUID>{u}</ResourceUID></Assignment>"
        )
        .unwrap();
    }
    format!(
        "<Project xmlns=\"http://schemas.microsoft.com/project\"><Name>{name}</Name><MinutesPerDay>480</MinutesPerDay>\
         <ExtendedAttributes>\
           <ExtendedAttribute><FieldID>111</FieldID><FieldName>Enterprise Text1</FieldName><Alias>Cost centre</Alias></ExtendedAttribute>\
           <ExtendedAttribute><FieldID>222</FieldID><FieldName>Enterprise Text2</FieldName><Alias>Region</Alias></ExtendedAttribute>\
         </ExtendedAttributes>\
         <Tasks>\
           <Task><UID>1</UID><Name>{name} design</Name><OutlineLevel>1</OutlineLevel><Start>2026-11-02T08:00:00</Start><Duration>PT40H0M0S</Duration>\
             <ExtendedAttribute><FieldID>111</FieldID><Value>{cost_centre}</Value></ExtendedAttribute>\
             <ExtendedAttribute><FieldID>222</FieldID><Value>North</Value></ExtendedAttribute></Task>\
           <Task><UID>2</UID><Name>{name} build</Name><OutlineLevel>1</OutlineLevel><Duration>PT80H0M0S</Duration>\
             <PredecessorLink><PredecessorUID>1</PredecessorUID><Type>1</Type></PredecessorLink>\
             <ExtendedAttribute><FieldID>111</FieldID><Value>{cost_centre}</Value></ExtendedAttribute></Task>\
         </Tasks><Resources>{res}</Resources><Assignments>{asg}</Assignments></Project>"
    )
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn three_projects_import_in_one_job_with_their_shared_resources_listed_once() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        request
            .post("/api/custom-fields")
            .json(&json!({ "key": "cost_centre", "label": "Cost centre", "type": "text", "applies_to": "task" }))
            .await
            .assert_status_ok();
        let parent: Value = request.post("/api/plans").json(&json!({ "kind": "Program", "name": "Estate" })).await.json();
        let parent = parent["pid"].as_str().unwrap().to_string();
        let body = json!({
            "parent_pid": parent,
            "field_map": { "cost centre": "cost_centre" },
            "projects": [
                { "xml": project("Alpha", &[(1, "Ana Lopez"), (2, "Bo Chen")], "CC-1") },
                { "xml": project("Beta", &[(5, "ana  lopez"), (6, "Cy Dunn")], "CC-2") },
                { "xml": project("Gamma", &[(9, "Bo Chen")], "CC-3"), "name": "Gamma (renamed)" }
            ]
        });

        // ---- the default is a dry run: nothing is written, and the pool is merged
        let dry: Value = request.post("/api/import/mspdi-batch").json(&body).await.json();
        assert_eq!(dry["dry_run"], true);
        let pool = dry["resource_pool"].as_array().unwrap();
        let names: Vec<&str> = pool.iter().map(|r| r["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["Ana Lopez", "Bo Chen", "Cy Dunn"], "each resource once");
        assert_eq!(pool[0]["projects"], json!(["Alpha", "Beta"]));
        assert_eq!(pool[1]["projects"], json!(["Alpha", "Gamma (renamed)"]));
        assert_eq!(dry["projects"][0]["outcome"], "would_import");
        assert_eq!(dry["projects"][0]["plan_would_be_created"], true);
        assert_eq!(dry["projects"][0]["custom_fields"]["values_to_store"], 2);
        let left = &dry["projects"][0]["custom_fields"]["fields_left_out"];
        assert_eq!(left[0]["field"], "Region");
        assert_eq!(left[0]["values"], 1);
        let kids: Value = request.get(&format!("/api/plans?parent={parent}")).await.json();
        assert_eq!(kids.as_array().unwrap().len(), 0, "a dry run creates nothing");

        // ---- applied: three plans under the programme, tasks imported, the field stored
        let done: Value = request.post("/api/import/mspdi-batch?dry_run=false").json(&body).await.json();
        assert_eq!(done["failed_at"], Value::Null, "{done}");
        let plans: Vec<String> = done["projects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                assert_eq!(p["outcome"], "imported", "{p}");
                assert_eq!(p["plan_created"], true);
                assert_eq!(p["report"]["tasks_created"], 2);
                assert_eq!(p["report"]["links_created"], 1);
                assert_eq!(p["custom_fields"]["values_stored"], 2);
                p["plan_pid"].as_str().unwrap().to_string()
            })
            .collect();
        let kids: Value = request.get(&format!("/api/plans?parent={parent}")).await.json();
        let kid_names: Vec<&str> = kids.as_array().unwrap().iter().map(|k| k["name"].as_str().unwrap()).collect();
        assert_eq!(kid_names.len(), 3);
        assert!(kid_names.contains(&"Gamma (renamed)") && kid_names.contains(&"Alpha"), "{kid_names:?}");
        let tasks: Value = request.get(&format!("/api/plans/{}/tasks", plans[1])).await.json();
        let first = tasks["tasks"].as_array().unwrap().iter().find(|t| t["title"] == "Beta design").unwrap();
        assert!(first["description"].as_str().unwrap().contains("lopez"), "the resource name is kept on the task: {first}");
        let values: Value = request
            .get(&format!("/api/custom-fields/values?applies_to=task&subject_pid={}", first["pid"].as_str().unwrap()))
            .await
            .json();
        assert_eq!(values["values"]["cost_centre"]["value"], "CC-2", "{values}");

        // ---- importing again into the same plans changes nothing (the single-file importer is idempotent)
        let again: Value = request
            .post("/api/import/mspdi-batch?dry_run=false")
            .json(&json!({
                "field_map": { "cost centre": "cost_centre" },
                "projects": [{ "plan_pid": plans[0], "xml": project("Alpha", &[(1, "Ana Lopez")], "CC-1") }]
            }))
            .await
            .json();
        assert_eq!(again["projects"][0]["plan_created"], false);
        assert_eq!(again["projects"][0]["report"]["tasks_created"], 0);
        assert_eq!(again["projects"][0]["report"]["tasks_updated"], 2);

        // ---- a mapping to a field that does not exist is reported, not stored
        let bad_map: Value = request
            .post("/api/import/mspdi-batch")
            .json(&json!({
                "field_map": { "cost centre": "no_such_field" },
                "projects": [{ "plan_pid": plans[0], "xml": project("Alpha", &[], "CC-1") }]
            }))
            .await
            .json();
        let rejected = &bad_map["projects"][0]["custom_fields"]["values_rejected"];
        assert_eq!(rejected.as_array().unwrap().len(), 2);
        assert!(rejected[0]["reason"].as_str().unwrap().contains("unknown custom field"), "{rejected}");

        // ---- one bad file stops the job before anything is written
        let before: Value = request.get(&format!("/api/plans?parent={parent}")).await.json();
        let broken = request
            .post("/api/import/mspdi-batch?dry_run=false")
            .json(&json!({ "projects": [{ "xml": project("Delta", &[], "x") }, { "xml": "<nope/>" }] }))
            .await;
        assert_eq!(broken.status_code(), 422);
        assert!(broken.text().contains("project 2"));
        let after: Value = request.get(&format!("/api/plans?parent={parent}")).await.json();
        assert_eq!(before, after);
        // And an empty job is refused.
        assert_eq!(request.post("/api/import/mspdi-batch").json(&json!({ "projects": [] })).await.status_code(), 422);
    })
    .await;
}

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

/// A project with pay: `Ana Lopez` at `ana_rate` an hour with a calendar that is off on Fridays, a fixed cost on
/// the first task, an enterprise field, and optionally a link to a task of another file.
fn pay_project(name: &str, ana_rate: &str, bo_rate: &str, cross: Option<(&str, i64)>) -> String {
    let link = cross.map_or(String::new(), |(file, uid)| {
        format!(
            "<PredecessorLink><PredecessorUID>{uid}</PredecessorUID><Type>1</Type><CrossProject>1</CrossProject>\
             <CrossProjectName>{file}</CrossProjectName></PredecessorLink>"
        )
    });
    format!(
        "<Project xmlns=\"http://schemas.microsoft.com/project\"><Name>{name}</Name><CurrencyCode>GBP</CurrencyCode>\
         <MinutesPerDay>480</MinutesPerDay>\
         <ExtendedAttributes><ExtendedAttribute><FieldID>111</FieldID><FieldName>Enterprise Text1</FieldName><Alias>Grant code</Alias></ExtendedAttribute></ExtendedAttributes>\
         <Calendars><Calendar><UID>5</UID><IsBaseCalendar>0</IsBaseCalendar><WeekDays>\
           <WeekDay><DayType>6</DayType><DayWorking>0</DayWorking></WeekDay></WeekDays></Calendar></Calendars>\
         <Tasks>\
           <Task><UID>1</UID><Name>{name} design</Name><OutlineLevel>1</OutlineLevel><Duration>PT40H0M0S</Duration>\
             <FixedCost>300</FixedCost><FixedCostAccrual>2</FixedCostAccrual>\
             <ExtendedAttribute><FieldID>111</FieldID><Value>42</Value></ExtendedAttribute></Task>\
           <Task><UID>2</UID><Name>{name} build</Name><OutlineLevel>1</OutlineLevel><Duration>PT80H0M0S</Duration>{link}\
             <ExtendedAttribute><FieldID>111</FieldID><Value>7</Value></ExtendedAttribute></Task>\
         </Tasks>\
         <Resources>\
           <Resource><UID>1</UID><Name>Ana Lopez</Name><StandardRate>{ana_rate}</StandardRate><StandardRateFormat>2</StandardRateFormat>\
             <OvertimeRate>75</OvertimeRate><OvertimeRateFormat>2</OvertimeRateFormat><CostPerUse>20</CostPerUse><CalendarUID>5</CalendarUID></Resource>\
           <Resource><UID>2</UID><Name>Bo Chen</Name><StandardRate>{bo_rate}</StandardRate><StandardRateFormat>2</StandardRateFormat></Resource>\
         </Resources></Project>"
    )
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn files_link_to_each_other_and_carry_rates_costs_and_new_fields() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let body = json!({
            "create_fields": true,
            "field_map": { "grant code": "grant_code" },
            "resource_map": { "ana lopez": { "role": "developer" } },
            "rates_effective_from": "2026-11-01",
            "projects": [
                { "xml": pay_project("Alpha", "50", "40", None) },
                { "xml": pay_project("Beta", "50", "45", Some(("C:\\Plans\\Alpha.mpp", 2))) },
                { "xml": pay_project("Gamma", "50", "40", Some(("Zeta.mpp", 1))) }
            ]
        });

        // ---- a dry run says what it would do, and writes nothing
        let dry: Value = request
            .post("/api/import/mspdi-batch")
            .json(&body)
            .await
            .json();
        assert_eq!(
            dry["custom_field_definitions"][0]["outcome"],
            "would_declare"
        );
        assert_eq!(
            dry["custom_field_definitions"][0]["type"], "integer",
            "42 and 7 are whole numbers"
        );
        assert_eq!(dry["projects"][0]["custom_fields"]["values_to_store"], 2);
        assert_eq!(dry["cross_project_links"]["would_create"], 1);
        let unresolved = dry["cross_project_links"]["unresolved"].as_array().unwrap();
        assert_eq!(unresolved.len(), 1);
        assert!(
            unresolved[0]["reason"]
                .as_str()
                .unwrap()
                .contains("Zeta.mpp"),
            "{unresolved:?}"
        );
        let terms = dry["resource_terms"].as_array().unwrap();
        let ana = &terms[0]["parts"];
        assert!(
            ana.as_array()
                .unwrap()
                .iter()
                .any(|p| p["what"] == "standard_rate" && p["outcome"] == "would_store")
        );
        assert!(
            ana.as_array()
                .unwrap()
                .iter()
                .any(|p| p["what"] == "cost_per_use" && p["outcome"] == "not_stored")
        );
        assert!(
            ana.as_array()
                .unwrap()
                .iter()
                .any(|p| p["what"] == "calendar"
                    && p["reason"].as_str().unwrap().contains("1111000")),
            "a person's calendar is theirs to set, and its weekdays are shown: {ana}"
        );
        let bo = terms[1]["parts"].as_array().unwrap();
        assert!(
            bo.iter().any(|p| p["reason"]
                .as_str()
                .unwrap()
                .contains("disagree on: standard rate")),
            "{bo:?}"
        );
        let defs: Value = request
            .get("/api/custom-fields?applies_to=task")
            .await
            .json();
        assert!(
            !defs.to_string().contains("grant_code"),
            "a dry run declares nothing"
        );
        let tables: Value = request.get("/api/cost-rate-tables").await.json();
        assert_eq!(tables["tables"].as_array().unwrap().len(), 0);

        // ---- applied
        let done: Value = request
            .post("/api/import/mspdi-batch?dry_run=false")
            .json(&body)
            .await
            .json();
        assert_eq!(done["failed_at"], Value::Null, "{done}");
        assert_eq!(done["custom_field_definitions"][0]["outcome"], "declared");
        assert_eq!(
            done["cross_project_links"]["created"], 1,
            "{}",
            done["cross_project_links"]
        );
        let plans: Vec<String> = done["projects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["plan_pid"].as_str().unwrap().to_string())
            .collect();
        for p in done["projects"].as_array().unwrap() {
            assert_eq!(p["costs"]["stored"], 1, "{p}");
            assert_eq!(p["custom_fields"]["values_stored"], 2);
        }
        // The link runs from Alpha's task 2 to Beta's task 2, across plans.
        let tasks_of = |plan: String| {
            let request = &request;
            async move {
                let t: Value = request
                    .get(&format!("/api/plans/{plan}/tasks"))
                    .await
                    .json();
                t["tasks"].as_array().unwrap().clone()
            }
        };
        let alpha = tasks_of(plans[0].clone()).await;
        let beta = tasks_of(plans[1].clone()).await;
        let a2 = alpha.iter().find(|t| t["title"] == "Alpha build").unwrap()["pid"]
            .as_str()
            .unwrap()
            .to_string();
        let b2 = beta.iter().find(|t| t["title"] == "Beta build").unwrap()["pid"]
            .as_str()
            .unwrap()
            .to_string();
        let deps: Value = request.get("/api/dependencies").await.json();
        let found = deps.to_string();
        assert!(
            found.contains(&a2) && found.contains(&b2),
            "no dependency joins the two plans: {found}"
        );
        // The role's rate tables hold Ana's rates, once, in GBP; Bo's disagreement stored nothing.
        let tables: Value = request
            .get("/api/cost-rate-tables?role=developer")
            .await
            .json();
        let tables = tables["tables"].as_array().unwrap();
        let names: Vec<&str> = tables.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["standard", "overtime"]);
        assert_eq!(tables[0]["currency"], "GBP");
        assert_eq!(tables[0]["rates"][0]["per_hour_minor"], 5000);
        assert_eq!(tables[0]["rates"][0]["effective_from"], "2026-11-01");
        assert_eq!(tables[1]["rates"][0]["per_hour_minor"], 7500);
        // The fixed cost sits on the first task.
        let a1 = alpha.iter().find(|t| t["title"] == "Alpha design").unwrap()["pid"]
            .as_str()
            .unwrap()
            .to_string();
        let costs: Value = request
            .get(&format!("/api/plans/{}/tasks/{a1}/costs", plans[0]))
            .await
            .json();
        assert_eq!(costs["costs"][0]["amount_minor"], 30000);
        assert_eq!(costs["costs"][0]["accrual"], "end");
        // The new field holds the whole number.
        let v: Value = request
            .get(&format!(
                "/api/custom-fields/values?applies_to=task&subject_pid={a1}"
            ))
            .await
            .json();
        assert_eq!(v["values"]["grant_code"]["value"], 42, "{v}");

        // ---- again: nothing is doubled
        let mut same = body.clone();
        for (i, p) in same["projects"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            p["plan_pid"] = json!(plans[i]);
        }
        let again: Value = request
            .post("/api/import/mspdi-batch?dry_run=false")
            .json(&same)
            .await
            .json();
        assert_eq!(again["failed_at"], Value::Null, "{again}");
        assert_eq!(again["cross_project_links"]["created"], 0);
        assert_eq!(again["cross_project_links"]["already_present"], 1);
        assert!(
            again["custom_field_definitions"]
                .as_array()
                .unwrap()
                .is_empty(),
            "already declared"
        );
        let ana = again["resource_terms"][0]["parts"].as_array().unwrap();
        assert!(
            ana.iter()
                .any(|p| p["what"] == "standard_rate" && p["outcome"] == "already_there"),
            "{ana:?}"
        );
        assert_eq!(
            again["projects"][0]["costs"]["stored"], 0,
            "the fixed cost is there already"
        );
        let tables: Value = request
            .get("/api/cost-rate-tables?role=developer")
            .await
            .json();
        assert_eq!(tables["tables"][0]["rates"].as_array().unwrap().len(), 1);
    })
    .await;
}

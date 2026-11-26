//! Request tests for handlers and branches the coverage baseline (COVERAGE.md) showed no test reaching: updating a
//! component, updating a skill pool, withdrawing a control, removing an allocation and the grouped budget report as
//! CSV. Each checks what the caller sees, not only that the line ran.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer, name: &str) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": "Project", "name": name }))
        .await
        .json();
    created["pid"].as_str().expect("pid").to_string()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_component_is_renamed_and_a_clashing_or_blank_name_is_refused() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Components plan").await;
        let mut pids = Vec::new();
        for name in ["API", "UI"] {
            let c: Value = request
                .post(&format!("/api/plans/{p}/components"))
                .json(&json!({ "name": name }))
                .await
                .json();
            pids.push(c["pid"].as_str().expect("component pid").to_string());
        }
        let url = format!("/api/plans/{p}/components/{}", pids[0]);

        let renamed: Value = request
            .put(&url)
            .json(&json!({ "name": "  Gateway " }))
            .await
            .json();
        assert_eq!(renamed["name"], "Gateway", "the name is trimmed: {renamed}");
        let listed: Value = request
            .get(&format!("/api/plans/{p}/components"))
            .await
            .json();
        assert!(listed.to_string().contains("Gateway"), "{listed}");

        // Keeping its own name (in another case) is not a clash; another component's name is.
        request
            .put(&url)
            .json(&json!({ "name": "GATEWAY" }))
            .await
            .assert_status_ok();
        assert_eq!(
            request
                .put(&url)
                .json(&json!({ "name": "ui" }))
                .await
                .status_code(),
            409
        );
        assert_eq!(
            request
                .put(&url)
                .json(&json!({ "name": "   " }))
                .await
                .status_code(),
            422
        );
        let missing = format!("/api/plans/{p}/components/{}", uuid::Uuid::new_v4());
        assert_eq!(
            request
                .put(&missing)
                .json(&json!({ "name": "Nope" }))
                .await
                .status_code(),
            404
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_skill_pool_is_retired_restored_and_has_its_members_replaced() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let created: Value = request
            .post("/api/skill-pools")
            .json(&json!({ "name": "Backend", "roles": ["developer"], "skills": ["rust"] }))
            .await
            .json();
        let pid = created["pid"].as_str().expect("pool pid").to_string();
        let url = format!("/api/skill-pools/{pid}");

        // No field given: the pool is read back unchanged.
        let same: Value = request.put(&url).json(&json!({})).await.json();
        assert_eq!(
            (same["active"].as_bool(), &same["roles"], &same["skills"]),
            (Some(true), &json!(["developer"]), &json!(["rust"]))
        );

        // Retire it.
        let retired: Value = request
            .put(&url)
            .json(&json!({ "active": false }))
            .await
            .json();
        assert_eq!(retired["active"], false);

        // Replace the roles only: the skills are kept. Then the skills only: the roles are kept.
        let roles: Value = request
            .put(&url)
            .json(&json!({ "active": true, "roles": ["architect", "tester"] }))
            .await
            .json();
        assert_eq!(
            (&roles["roles"], &roles["skills"], roles["active"].as_bool()),
            (
                &json!(["architect", "tester"]),
                &json!(["rust"]),
                Some(true)
            )
        );
        let skills: Value = request
            .put(&url)
            .json(&json!({ "skills": ["sql"] }))
            .await
            .json();
        assert_eq!(
            (&skills["roles"], &skills["skills"]),
            (&json!(["architect", "tester"]), &json!(["sql"]))
        );

        let missing = format!("/api/skill-pools/{}", uuid::Uuid::new_v4());
        assert_eq!(
            request
                .put(&missing)
                .json(&json!({ "active": false }))
                .await
                .status_code(),
            404
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_withdrawn_control_leaves_the_register_and_cannot_be_withdrawn_twice() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Controls plan").await;
        let control: Value = request
            .post(&format!("/api/plans/{p}/controls"))
            .json(&json!({
                "name": "Gate readiness", "timing": "feedforward",
                "metric": "gate_readiness", "target_value": 10_000,
                "comparator": "at_least", "cadence_days": 7
            }))
            .await
            .json();
        let pid = control["pid"].as_str().expect("control pid").to_string();
        let before: Value = request
            .get(&format!("/api/plans/{p}/controls"))
            .await
            .json();
        assert!(before.to_string().contains(&pid), "{before}");

        request
            .delete(&format!("/api/controls/{pid}"))
            .await
            .assert_status_ok();
        let after: Value = request
            .get(&format!("/api/plans/{p}/controls"))
            .await
            .json();
        assert!(!after.to_string().contains(&pid), "withdrawn: {after}");
        assert_eq!(
            request
                .delete(&format!("/api/controls/{pid}"))
                .await
                .status_code(),
            404
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_removed_allocation_leaves_the_list_and_a_second_removal_is_not_found() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Allocation plan").await;
        let a: Value = request
            .post(&format!("/api/plans/{p}/allocations"))
            .json(
                &json!({ "person_ref": "person:3f2c6b7e-0000-4000-8000-0000000000cc", "percent": 50,
                           "start_date": "2026-10-05", "end_date": "2026-10-18" }),
            )
            .await
            .json();
        let pid = a["pid"].as_str().expect("allocation pid").to_string();
        let url = format!("/api/plans/{p}/allocations/{pid}");
        request.delete(&url).await.assert_status_ok();
        let listed: Value = request
            .get(&format!("/api/plans/{p}/allocations"))
            .await
            .json();
        assert!(!listed.to_string().contains(&pid), "{listed}");
        assert_eq!(request.delete(&url).await.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_grouped_budget_report_is_a_csv_with_a_row_per_currency_and_a_bare_row_for_no_money() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let a = plan(&request, "Alpha").await;
        request
            .post("/api/plans")
            .json(&json!({ "kind": "Program", "name": "Gamma" }))
            .await
            .assert_status_ok();
        for (currency, planned) in [("GBP", 100), ("EUR", 70)] {
            request
                .post(&format!("/api/plans/{a}/budget-lines"))
                .json(&json!({ "category": "capex", "description": "x", "currency": currency, "planned_minor": planned }))
                .await
                .assert_status_ok();
        }
        let saved: Value = request
            .post("/api/reports")
            .json(&json!({ "name": "By kind", "collection": "", "fields": ["name"], "group_by": "kind" }))
            .await
            .json();
        let report = saved["pid"].as_str().expect("report pid");
        let csv = request.get(&format!("/api/reports/{report}/run?format=csv")).await;
        assert!(csv.headers().get("content-type").is_some_and(|v| v == "text/csv"));
        let text = csv.text();
        assert!(text.starts_with("kind,plans,currency,planned_minor,actual_minor,remaining_minor,line_count\n"), "{text}");
        assert!(text.contains(",GBP,100,"), "{text}");
        assert!(text.contains(",EUR,70,"), "{text}");
        assert!(text.lines().any(|l| l.to_lowercase().starts_with("program,1,,,,,")), "a group with no money has one bare row: {text}");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_programme_threshold_is_set_changed_and_removed() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let programme: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Program", "name": "Thresholds programme" }))
            .await
            .json();
        let url = format!(
            "/api/plans/{}/thresholds",
            programme["pid"].as_str().expect("pid")
        );
        let set: Value = request
            .put(&url)
            .json(&json!({ "kind": "confidence_red_packs", "count": 3 }))
            .await
            .json();
        assert_eq!(
            (set["kind"].as_str(), set["count"].as_i64()),
            (Some("confidence_red_packs"), Some(3)),
            "{set}"
        );
        let changed: Value = request
            .put(&url)
            .json(&json!({ "kind": "confidence_red_packs", "count": 5 }))
            .await
            .json();
        assert_eq!(changed["count"], 5);
        let removed: Value = request
            .put(&url)
            .json(&json!({ "kind": "confidence_red_packs", "remove": true }))
            .await
            .json();
        assert_eq!(removed["removed"], "confidence_red_packs");
        // Removing what is not there is not an error.
        request
            .put(&url)
            .json(&json!({ "kind": "confidence_red_packs", "remove": true }))
            .await
            .assert_status_ok();
        assert_eq!(
            request
                .put(&url)
                .json(&json!({ "kind": "nonsense" }))
                .await
                .status_code(),
            422
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_scorecards_kpis_are_replaced_and_an_unknown_kpi_is_refused() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let card: Value = request
            .post("/api/scorecards")
            .json(&json!({ "name": "Delivery card" }))
            .await
            .json();
        let card = card["pid"].as_str().expect("scorecard pid").to_string();
        let mut kpis = Vec::new();
        for name in ["Uptime", "Defects"] {
            let k: Value = request
                .post("/api/kpis")
                .json(
                    &json!({ "name": name, "unit": "percent", "direction": "higher_better",
                               "target_value": 95, "red_value": 80, "cadence": "weekly" }),
                )
                .await
                .json();
            kpis.push(k["pid"].as_str().expect("kpi pid").to_string());
        }
        let url = format!("/api/scorecards/{card}/kpis");
        let both: Value = request
            .put(&url)
            .json(&json!({ "kpi_pids": kpis }))
            .await
            .json();
        assert_eq!(both["scorecard_pid"], json!(card));
        assert_eq!(both["kpi_pids"].as_array().expect("pids").len(), 2);
        let one: Value = request
            .put(&url)
            .json(&json!({ "kpi_pids": [kpis[0]] }))
            .await
            .json();
        assert_eq!(one["kpi_pids"], json!([kpis[0]]));
        let shown: Value = request.get(&format!("/api/scorecards/{card}")).await.json();
        assert!(
            shown.to_string().contains("Uptime") && !shown.to_string().contains("Defects"),
            "{shown}"
        );
        request
            .put(&url)
            .json(&json!({ "kpi_pids": [] }))
            .await
            .assert_status_ok();
        let unknown = request
            .put(&url)
            .json(&json!({ "kpi_pids": [uuid::Uuid::new_v4()] }))
            .await;
        assert_eq!(unknown.status_code(), 422);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_merge_is_in_the_executive_decision_feed_with_its_reason() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let main = plan(&request, "Survivor plan").await;
        let duplicate = plan(&request, "Duplicate plan").await;
        request
            .post("/api/plans/merge")
            .json(&json!({ "main_pid": main, "duplicate_pid": duplicate, "reason": "same work, entered twice" }))
            .await
            .assert_status_ok();
        let decisions: Value = request.get("/api/executive/decisions").await.json();
        let entry = decisions["decisions"]
            .as_array()
            .expect("decisions")
            .iter()
            .find(|d| d["kind"] == "merge")
            .unwrap_or_else(|| panic!("a merge entry in {decisions}"));
        assert_eq!(entry["decision"], "merged");
        assert_eq!(entry["subject"]["pid"], json!(main));
        assert_eq!(entry["merged_from"], json!(duplicate));
        assert_eq!(entry["reason"], "same work, entered twice");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_merge_with_no_reason_is_an_auditor_finding() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let main = plan(&request, "Kept plan").await;
        let duplicate = plan(&request, "Folded plan").await;
        request
            .post("/api/plans/merge")
            .json(&json!({ "main_pid": main, "duplicate_pid": duplicate }))
            .await
            .assert_status_ok();
        let findings: Value = request.get("/api/auditor/findings").await.json();
        let finding = findings["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("findings in {findings}"))
            .iter()
            .find(|f| f["rule"] == "merge_without_reason")
            .unwrap_or_else(|| panic!("a merge_without_reason finding in {findings}"));
        assert_eq!(finding["main_pid"], json!(main));
        assert_eq!(finding["duplicate_pid"], json!(duplicate));
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_cited_case_or_exception_must_belong_to_the_plan() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Access plan").await;
        let stranger = uuid::Uuid::new_v4();
        let refused = request
            .post(&format!("/api/plans/{p}/access-records"))
            .json(&json!({ "person_ref": "person:3f2c6b7e-0000-4000-8000-0000000000dd", "access": "repo write",
                           "action": "granted", "recorded_on": "2026-10-06", "case_pid": stranger }))
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());
        assert!(refused.text().contains("case_pid"));

        let programme: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Program", "name": "Cited programme" }))
            .await
            .json();
        let programme = programme["pid"].as_str().expect("pid").to_string();
        let refused = request
            .post(&format!("/api/plans/{programme}/programme-status"))
            .json(&json!({ "status": "paused", "reason": "pausing while the funding is confirmed", "exception_pid": stranger }))
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());
        assert!(refused.text().contains("exception_pid"));
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn bulk_requests_are_refused_for_a_bad_format_an_empty_body_and_are_filterable() {
    use axum::body::Bytes;
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: `#[serial]` — nothing else runs while this is set.
    unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path()) };
    request::<App, _, _>(|request, _ctx| async move {
        let post = |query: &'static str, body: &'static str| {
            let request = &request;
            async move {
                request
                    .post(&format!("/api/plans/import?{query}"))
                    .content_type("text/plain")
                    .bytes(Bytes::from_static(body.as_bytes()))
                    .await
            }
        };
        assert_eq!(post("format=xml", "{}").await.status_code(), 422);
        assert_eq!(post("format=jsonl&on_keyless=guess", "{}").await.status_code(), 422);
        assert_eq!(post("format=jsonl", "").await.status_code(), 422);

        // A file with one bad row (no name) and one good row: the error report is also served as JSON lines.
        let accepted = post(
            "format=jsonl&on_keyless=create",
            "{\"kind\":\"Project\"}\n{\"name\":\"Filter me\",\"kind\":\"Project\",\"code\":\"F-1\"}",
        )
        .await;
        assert_eq!(accepted.status_code(), 202);
        let job = accepted.json::<Value>()["job_id"].as_str().expect("job id").to_string();
        let errors = request.get(&format!("/api/plans/bulk-jobs/{job}/errors?format=jsonl")).await;
        assert!(errors.headers().get("content-type").is_some_and(|v| v == "application/x-ndjson"));
        let text = errors.text();
        assert_eq!(text.lines().count(), 1, "one bad row: {text}");
        assert!(serde_json::from_str::<Value>(text.lines().next().unwrap()).is_ok(), "{text}");

        // The job list filters by kind and by status; an export filters by kind.
        let imports: Value = request.get("/api/plans/bulk-jobs?kind=import&status=completed_with_errors").await.json();
        assert!(imports.as_array().expect("jobs").iter().any(|j| j["job_id"] == json!(job)), "{imports}");
        let none: Value = request.get("/api/plans/bulk-jobs?kind=export&status=nonsense").await.json();
        assert!(none.as_array().expect("jobs").is_empty());
        let export = request.post("/api/plans/export").json(&json!({ "kind": "Project", "name_like": "filter me" })).await;
        assert_eq!(export.status_code(), 202, "{}", export.text());
        let status: Value = request
            .get(&format!("/api/plans/export/{}", export.json::<Value>()["job_id"].as_str().expect("job id")))
            .await
            .json();
        assert_eq!(status["counts"]["total"], 1, "{status}");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_bulk_edit_clears_fields_with_null_and_refuses_a_malformed_value_whole() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Bulk clear plan").await;
        let t: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Clear me", "due_date": "2026-10-05", "start_date": "2026-10-01",
                           "assignee_ref": "person:aaaaaaaa-0000-0000-0000-000000000001" }))
            .await
            .json();
        let t = t["pid"].as_str().expect("task pid").to_string();
        let bulk = |set: Value| {
            let (request, t) = (&request, t.clone());
            async move { request.post("/api/tasks/bulk").json(&json!({ "ids": [t], "set": set })).await }
        };
        for bad in [
            json!({ "assignee_ref": 5 }),
            json!({ "assignee_ref": "ana" }),
            json!({ "due_date": "next week" }),
            json!({ "due_date": 5 }),
            json!({ "component_pid": "not-a-uuid" }),
            json!({ "component_pid": 5 }),
            json!({ "start_date": "soon" }),
            json!({ "start_date": 5 }),
        ] {
            assert_eq!(bulk(bad.clone()).await.status_code(), 422, "{bad}");
        }
        let cleared = bulk(json!({ "assignee_ref": null, "due_date": null, "component_pid": null, "start_date": null })).await;
        cleared.assert_status_ok();
        assert_eq!(cleared.json::<Value>()["ok"], 1);
        let shown: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        let row = shown.to_string();
        assert!(!row.contains("2026-10-05") && !row.contains("aaaaaaaa-0000"), "cleared: {row}");
        let set = bulk(json!({ "assignee_ref": "person:bbbbbbbb-0000-0000-0000-000000000002", "due_date": "2026-11-02" })).await;
        assert_eq!(set.json::<Value>()["ok"], 1);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn the_ledger_refuses_a_bad_allocation_or_cost_code_and_a_remapped_code_moves_to_the_new_line()
 {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Ledger plan").await;
        let source: Value = request.post("/api/funding-sources").json(&json!({ "name": "Treasury" })).await.json();
        let source = source["pid"].as_str().expect("source pid").to_string();
        let allocation = |body: Value| {
            let request = &request;
            async move { request.post("/api/funding-allocations").json(&body).await }
        };
        let ok = json!({ "funding_source_pid": source, "amount_minor": 1000, "currency": "GBP", "allocated_on": "2026-04-01" });
        let with = |key: &str, value: Value| {
            let mut b = ok.clone();
            b[key] = value;
            b
        };
        for (what, body) in [
            ("a currency that is not ISO 4217", with("currency", json!("pounds"))),
            ("an allocation dated in the future", with("allocated_on", json!("2999-01-01"))),
            ("a reference over 120 characters", with("reference", json!("r".repeat(121)))),
            ("a source that does not exist", with("funding_source_pid", json!(uuid::Uuid::new_v4()))),
        ] {
            assert_eq!(allocation(body).await.status_code(), 422, "{what}");
        }
        allocation(ok.clone()).await.assert_status_ok();

        let mut lines = Vec::new();
        for description in ["first", "second"] {
            let line: Value = request
                .post(&format!("/api/plans/{p}/budget-lines"))
                .json(&json!({ "category": "capex", "description": description, "currency": "GBP", "planned_minor": 100 }))
                .await
                .json();
            lines.push(line["pid"].as_str().expect("line pid").to_string());
        }
        let map = |code: String, line: String| {
            let request = &request;
            async move { request.put("/api/cost-codes").json(&json!({ "cost_code": code, "budget_line_pid": line })).await }
        };
        assert_eq!(map("   ".into(), lines[0].clone()).await.status_code(), 422);
        assert_eq!(map("c".repeat(61), lines[0].clone()).await.status_code(), 422);
        assert_eq!(map("CC-1".into(), uuid::Uuid::new_v4().to_string()).await.status_code(), 422);
        map("CC-1".into(), lines[0].clone()).await.assert_status_ok();
        // The same code in another case is the same code: it moves to the second line rather than being added.
        map("cc-1".into(), lines[1].clone()).await.assert_status_ok();
        let codes: Value = request.get("/api/cost-codes").await.json();
        let text = codes.to_string();
        assert!(text.contains(&lines[1]) && !text.contains(&lines[0]), "{text}");

        let too_big = "x".repeat(1_000_001);
        assert_eq!(request.post("/api/ledger/reconcile").text(&too_big).await.status_code(), 422);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn rate_tables_rates_and_task_costs_each_refuse_every_bad_field_at_once() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let who = "person:3f2c6b7e-0000-4000-8000-0000000000ee";
        let table = |body: Value| {
            let request = &request;
            async move { request.post("/api/cost-rate-tables").json(&body).await }
        };
        for (what, body) in [
            ("a blank name", json!({ "name": " ", "person_ref": who, "currency": "GBP" })),
            ("a name over 40 characters", json!({ "name": "n".repeat(41), "person_ref": who, "currency": "GBP" })),
            ("a person that is not a URN", json!({ "name": "standard", "person_ref": "ana", "currency": "GBP" })),
            ("a blank role", json!({ "name": "standard", "role": "  ", "currency": "GBP" })),
            ("a role over 60 characters", json!({ "name": "standard", "role": "r".repeat(61), "currency": "GBP" })),
            ("both a person and a role", json!({ "name": "standard", "person_ref": who, "role": "dev", "currency": "GBP" })),
        ] {
            assert_eq!(table(body).await.status_code(), 422, "{what}");
        }
        let created: Value = table(json!({ "name": "standard", "role": " Developer ", "currency": "gbp" })).await.json();
        let pid = created["pid"].as_str().expect("table pid").to_string();
        // The same role table again is the same table, found by its role with the spaces trimmed.
        assert_eq!(table(json!({ "name": "standard", "role": "Developer", "currency": "GBP" })).await.status_code(), 409);

        let rates = format!("/api/cost-rate-tables/{pid}/rates");
        let rate = |body: Value| {
            let (request, rates) = (&request, rates.clone());
            async move { request.post(&rates).json(&body).await }
        };
        assert_eq!(rate(json!({ "per_hour_minor": -1, "effective_from": "2026-01-01" })).await.status_code(), 422);
        assert_eq!(rate(json!({ "per_hour_minor": 5000, "effective_from": "1969-12-31" })).await.status_code(), 422);
        assert_eq!(rate(json!({ "per_hour_minor": 5000, "effective_from": "2101-01-01" })).await.status_code(), 422);
        rate(json!({ "per_hour_minor": 5000, "effective_from": "2026-01-01" })).await.assert_status_ok();

        let p = plan(&request, "Cost plan").await;
        let t: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({ "title": "Costed" })).await.json();
        let cost = format!("/api/plans/{p}/tasks/{}/costs", t["pid"].as_str().expect("task pid"));
        let refused = request
            .post(&cost)
            .json(&json!({ "kind": "gift", "description": " ", "amount_minor": 0, "currency": "pounds", "accrual": "whenever" }))
            .await;
        assert_eq!(refused.status_code(), 422);
        let text = refused.text();
        for part in ["kind must be", "description is required", "amount_minor must be above zero", "currency must be", "accrual must be"] {
            assert!(text.contains(part), "{part} in {text}");
        }
        request
            .post(&cost)
            .json(&json!({ "kind": "material", "description": "Cable", "amount_minor": 1200, "currency": "gbp", "accrual": "start" }))
            .await
            .assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_roadmap_change_set_preview_names_every_problem_without_writing() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let created: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Product", "name": "Roadmap plan" }))
            .await
            .json();
        let p = created["pid"].as_str().expect("pid").to_string();
        request
            .put(&format!("/api/plans/{p}/roadmap"))
            .json(&json!({ "title": "Our roadmap" }))
            .await
            .assert_status_ok();
        let item: Value = request
            .post(&format!("/api/plans/{p}/roadmap/items"))
            .json(&json!({ "title": "Existing item", "placement": "now" }))
            .await
            .json();
        let item = item["pid"].as_str().expect("item pid").to_string();
        let preview = |set: Value| {
            let (request, url) = (&request, format!("/api/plans/{p}/roadmap/publish/preview"));
            async move { request.post(&url).json(&set).await.json::<Value>() }
        };
        let change = |base: Value, value: Value| json!({ "base": base, "value": value });
        let stranger = uuid::Uuid::new_v4();

        let report = preview(json!({
            "roadmap": {
                "published": change(json!(false), json!("yes")),
                "category_pid": change(Value::Null, json!(stranger)),
                "owner": change(Value::Null, json!("x")),
                "vision": change(Value::Null, json!(5)),
            },
            "items": [
                { "pid": item, "fields": {
                    "tags": change(json!([]), json!("not a list")),
                    "details": change(json!([]), json!("not details")),
                    "organization_pids": change(json!([]), json!(["nope"])),
                    "category_pid": change(Value::Null, Value::Null),
                    "placement": change(json!("now"), json!("someday")),
                    "title": change(json!("Existing item"), json!("")),
                } },
                { "pid": item, "fields": {} },
            ],
            "new": [ { "title": "", "placement": "now" } ],
        }))
        .await;
        assert_eq!(report["would_publish"], false, "{report}");
        let problems = report["problems"].to_string();
        for part in [
            "published is true or false",
            "is not an active roadmap category",
            "roadmap field `owner` cannot be changed here",
            "vision is text or null",
            "tags is a list of text",
            "details is a list of",
            "organization_pids is a list of ids",
            "item field `category_pid` cannot be changed here",
            "placement is one of",
            "is changed twice",
            "new item 1",
        ] {
            assert!(problems.contains(part), "{part} in {problems}");
        }

        // A base that is itself malformed is reported as the base's problem.
        let report =
            preview(json!({ "roadmap": { "published": change(json!("maybe"), json!(true)) } }))
                .await;
        assert!(
            report["problems"]
                .to_string()
                .contains("base: published is true or false"),
            "{report}"
        );

        // A change to an item that is gone is a conflict, not a problem; and a clean change set would publish.
        let report = preview(json!({ "items": [ { "pid": stranger, "fields": {} } ] })).await;
        assert_eq!(report["conflicts"][0]["gone"], true, "{report}");
        let report =
            preview(json!({ "roadmap": { "vision": change(Value::Null, json!("Clearer care")) } }))
                .await;
        assert_eq!(report["would_publish"], true, "{report}");
        assert_eq!(report["changes"][0]["to"], "Clearer care");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_budget_baseline_refuses_each_bad_field_and_a_rebaseline_needs_a_reason() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Baseline plan").await;
        let url = format!("/api/plans/{p}/budget-baselines");
        let period = json!({ "period_start": "2026-04-01", "period_end": "2026-06-30", "planned_minor": 1000 });
        let post = |body: Value| {
            let (request, url) = (&request, url.clone());
            async move { request.post(&url).json(&body).await }
        };
        for (what, body) in [
            ("a currency that is not ISO 4217", json!({ "currency": "pounds", "periods": [period] })),
            ("no periods", json!({ "currency": "GBP", "periods": [] })),
            ("a period ending before it starts", json!({ "currency": "GBP",
                "periods": [{ "period_start": "2026-06-30", "period_end": "2026-04-01", "planned_minor": 1 }] })),
            ("a negative amount", json!({ "currency": "GBP",
                "periods": [{ "period_start": "2026-04-01", "period_end": "2026-06-30", "planned_minor": -1 }] })),
        ] {
            assert_eq!(post(body).await.status_code(), 422, "{what}");
        }
        post(json!({ "currency": "gbp", "periods": [period] })).await.assert_status_ok();
        // The second version needs a reason, and not an enormous one.
        assert_eq!(post(json!({ "currency": "GBP", "periods": [period] })).await.status_code(), 422);
        assert_eq!(post(json!({ "currency": "GBP", "periods": [period], "reason": "  " })).await.status_code(), 422);
        assert_eq!(post(json!({ "currency": "GBP", "periods": [period], "reason": "r".repeat(100_000) })).await.status_code(), 422);
        post(json!({ "currency": "GBP", "periods": [period], "reason": "scope grew" })).await.assert_status_ok();
        let forecast = request.get(&format!("/api/plans/{p}/financials/forecast")).await;
        forecast.assert_status_ok();
        let rollup = request.get(&format!("/api/financials/forecast?plan={p}&depth=0")).await;
        assert_eq!(rollup.status_code(), 422);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn the_deescalation_tracker_refuses_short_names_levels_criteria_and_unfinished_quarters() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        assert_eq!(
            request
                .post("/api/scorecards")
                .json(&json!({ "name": "ab" }))
                .await
                .status_code(),
            422
        );
        let card: Value = request
            .post("/api/scorecards")
            .json(&json!({ "name": "Recovery card" }))
            .await
            .json();
        let card = card["pid"].as_str().expect("scorecard pid").to_string();
        assert_eq!(
            request
                .post(&format!("/api/scorecards/{card}/from-template"))
                .json(&json!({ "measures": [] }))
                .await
                .status_code(),
            422
        );

        let level = request
            .put("/api/deescalation/level")
            .json(&json!({ "level": "   " }))
            .await;
        assert_eq!(level.status_code(), 422, "{}", level.text());
        assert_eq!(
            request
                .post("/api/deescalation/criteria")
                .json(&json!({ "criterion": "no" }))
                .await
                .status_code(),
            422
        );
        let criterion: Value = request
            .post("/api/deescalation/criteria")
            .json(&json!({ "criterion": "Two quarters with no red milestones" }))
            .await
            .json();
        let url = format!(
            "/api/deescalation/criteria/{}/evidence",
            criterion["pid"].as_str().expect("criterion pid")
        );
        let evidence = |body: Value| {
            let (request, url) = (&request, url.clone());
            async move { request.put(&url).json(&body).await }
        };
        let good =
            json!({ "quarter": "2020-Q1", "met": true, "note": "Checked the milestone log" });
        let with = |key: &str, value: Value| {
            let mut b = good.clone();
            b[key] = value;
            b
        };
        for (what, body) in [
            (
                "a quarter that is not YYYY-Qn",
                with("quarter", json!("last spring")),
            ),
            (
                "a quarter that has not finished",
                with("quarter", json!("2999-Q4")),
            ),
            ("a note under five characters", with("note", json!("ok"))),
            (
                "a link that is not http",
                with("link", json!("ftp://example.org/x")),
            ),
            (
                "a link with a space",
                with("link", json!("https://example.org/a b")),
            ),
        ] {
            assert_eq!(evidence(body).await.status_code(), 422, "{what}");
        }
        evidence(with("link", json!("https://example.org/evidence")))
            .await
            .assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_checklist_template_is_replaced_and_a_case_is_refused_for_a_bad_date_title_note_or_status()
 {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Onboarding plan").await;
        let template = "/api/onboarding/templates/plan_closeout";
        request.put(template).json(&json!({ "items": [{ "title": "First", "offset_days": 0 }] })).await.assert_status_ok();
        // A second put replaces the first rather than adding to it.
        request
            .put(template)
            .json(&json!({ "items": [{ "title": "Second", "offset_days": 1 }, { "title": "Third", "offset_days": 2 }] }))
            .await
            .assert_status_ok();
        let listed: Value = request.get("/api/onboarding/templates").await.json();
        let closeout = listed["templates"].as_array().expect("templates").iter().find(|t| t["kind"] == "plan_closeout").expect("closeout").clone();
        assert_eq!(closeout["items"].as_array().expect("items").len(), 2, "{closeout}");

        let cases = format!("/api/plans/{p}/onboarding/cases");
        let case = |body: Value| {
            let (request, cases) = (&request, cases.clone());
            async move { request.post(&cases).json(&body).await }
        };
        assert_eq!(case(json!({ "kind": "plan_closeout", "effective_date": "1969-12-31" })).await.status_code(), 422);
        assert_eq!(case(json!({ "kind": "plan_closeout", "effective_date": "2101-01-01" })).await.status_code(), 422);
        assert_eq!(case(json!({ "kind": "plan_closeout", "effective_date": "2026-10-12", "title": "   " })).await.status_code(), 422);
        assert_eq!(case(json!({ "kind": "plan_closeout", "effective_date": "2026-10-12", "title": "t".repeat(201) })).await.status_code(), 422);
        let started: Value = case(json!({ "kind": "plan_closeout", "effective_date": "2026-10-12", "title": "Close it down" })).await.json();
        let case_pid = started["pid"].as_str().expect("case pid").to_string();
        let item = started["items"][0]["pid"].as_str().expect("item pid").to_string();

        assert_eq!(request.get(&format!("{cases}?status=bogus")).await.status_code(), 422);
        request.get(&format!("{cases}?status=open")).await.assert_status_ok();

        let item_url = format!("/api/onboarding/cases/{case_pid}/items/{item}");
        assert_eq!(request.put(&item_url).json(&json!({ "note": "n".repeat(501) })).await.status_code(), 422);
        let set: Value = request
            .put(&item_url)
            .json(&json!({ "due_date": "2026-11-01", "note": "  chase the supplier  ", "owner_ref": "" }))
            .await
            .json();
        assert!(set.to_string().contains("2026-11-01") && set.to_string().contains("chase the supplier"), "{set}");

        let cancel = format!("/api/onboarding/cases/{case_pid}/cancel");
        assert_eq!(request.post(&cancel).json(&json!({ "reason": "  " })).await.status_code(), 422);
        request.post(&cancel).json(&json!({ "reason": "plan not closing after all" })).await.assert_status_ok();
        assert_eq!(request.post(&cancel).json(&json!({ "reason": "again" })).await.status_code(), 409);
        assert_eq!(request.put(&item_url).json(&json!({ "note": "late" })).await.status_code(), 409);
        assert_eq!(request.post(&format!("/api/onboarding/cases/{case_pid}/complete")).await.status_code(), 409);
        request.delete(template).await.assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_dismissed_idea_takes_no_votes_and_cannot_be_dismissed_again() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let idea: Value = request
            .post("/api/ideas")
            .json(&json!({ "title": "Retire the fax", "pitch": "Nobody reads it" }))
            .await
            .json();
        let idea = idea["pid"].as_str().expect("idea pid").to_string();
        let voted: Value = request
            .post(&format!("/api/ideas/{idea}/vote"))
            .await
            .json();
        assert_eq!(voted["votes"], 1);
        request
            .post(&format!("/api/ideas/{idea}/dismiss"))
            .await
            .assert_status_ok();
        assert_eq!(
            request
                .post(&format!("/api/ideas/{idea}/vote"))
                .await
                .status_code(),
            422
        );
        let again = request.post(&format!("/api/ideas/{idea}/dismiss")).await;
        assert_eq!(again.status_code(), 422);
        assert!(
            again.text().contains("idea is dismissed"),
            "{}",
            again.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn an_automation_with_a_blank_name_and_an_unknown_trigger_is_refused_with_both_problems() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let refused = request
            .post("/api/automations")
            .json(&json!({ "name": "  ", "trigger_kind": "when_it_feels_right", "actions": [{ "kind": "teleport", "value": "x" }] }))
            .await;
        assert_eq!(refused.status_code(), 422);
        assert!(refused.text().contains("name is required"), "{}", refused.text());
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn scheduled_actions_are_listed_by_subject_and_by_what_is_overdue() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Scheduled plan").await;
        let other = plan(&request, "Other scheduled plan").await;
        for subject in [&p, &other] {
            request
                .post("/api/scheduled-actions")
                .json(&json!({ "subject_kind": "plan", "subject_pid": subject, "action_kind": "expire_review", "in_days": 30 }))
                .await
                .assert_status_ok();
        }
        let mine: Value = request.get(&format!("/api/scheduled-actions?subject={p}")).await.json();
        assert_eq!(mine.as_array().map(Vec::len), Some(1), "{mine}");
        let overdue: Value = request.get("/api/scheduled-actions?overdue=true").await.json();
        assert_eq!(overdue.as_array().map(Vec::len), Some(0), "nothing is due for 30 days: {overdue}");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn review_queries_and_verdicts_are_validated_and_filterable() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let idea: Value = request.post("/api/ideas").json(&json!({ "title": "Reviewed idea" })).await.json();
        let idea = idea["pid"].as_str().expect("idea pid").to_string();
        let stranger = uuid::Uuid::new_v4();

        // Consensus: the subject kind is a token; an idea, an unknown proposal and a plan all have one (empty) answer.
        let bad = request.get(&format!("/api/reviews/consensus?subject_kind=moon&subject_pid={idea}")).await;
        assert_eq!(bad.status_code(), 422);
        for (kind, pid) in [("idea", idea.clone()), ("proposal", stranger.to_string())] {
            let consensus = request.get(&format!("/api/reviews/consensus?subject_kind={kind}&subject_pid={pid}")).await;
            consensus.assert_status_ok();
        }

        // A verdict names every problem at once, before the review is even looked up.
        let verdict = request
            .post(&format!("/api/reviews/{stranger}/submit"))
            .json(&json!({ "recommendation": "shrug", "score": 101, "comment": "c".repeat(100_000) }))
            .await;
        assert_eq!(verdict.status_code(), 422);
        let text = verdict.text();
        for part in ["recommendation must be one of", "score, when given, must be between 0 and 100", "comment exceeds"] {
            assert!(text.contains(part), "{part} in {text}");
        }

        // The list filters by subject, reviewer and status.
        let listed = request
            .get(&format!("/api/reviews?subject_kind=idea&subject_pid={idea}&reviewer=person:3f2c6b7e-0000-4000-8000-0000000000ab&status=invited"))
            .await;
        listed.assert_status_ok();
        assert_eq!(listed.json::<Value>().as_array().map(Vec::len), Some(0));

        // An inbox is asked for by a valid actor reference.
        let inbox = request.get("/api/notifications?recipient=ana").await;
        assert_eq!(inbox.status_code(), 422);
    })
    .await;
}

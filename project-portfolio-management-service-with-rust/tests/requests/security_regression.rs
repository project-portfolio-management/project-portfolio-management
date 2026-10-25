//! Security regression tests (BR-6): one test per class of defect a security review of a comparable product found,
//! each written to fail if the guard it checks were removed.
//!
//! **Nested ids belong to their parent.** A record of plan B must not be read, changed or deleted through plan A's
//! path. The review found a nested route that never compared the child's parent with the path's parent (a
//! cross-tenant edit and delete). Here every nested write and read on the main collections is tried with plan B's
//! record under plan A's path and must answer `404`; plan B's records are then checked unchanged.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn made(request: &axum_test::TestServer, path: &str, body: Value) -> String {
    let r: Value = request.post(path).json(&body).await.json();
    r["pid"]
        .as_str()
        .unwrap_or_else(|| panic!("{path}: {r}"))
        .to_string()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_record_of_one_plan_is_not_reachable_through_another_plans_path() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let a = made(&request, "/api/plans", json!({ "kind": "Project", "name": "Plan A" })).await;
        let b = made(&request, "/api/plans", json!({ "kind": "Project", "name": "Plan B" })).await;
        // Plan B's records.
        let task = made(&request, &format!("/api/plans/{b}/tasks"), json!({ "title": "B task" })).await;
        let component = made(&request, &format!("/api/plans/{b}/components"), json!({ "name": "B component" })).await;
        let milestone = made(&request, &format!("/api/plans/{b}/milestones"), json!({ "name": "B milestone", "due": "2026-12-01" })).await;
        let risk = made(&request, &format!("/api/plans/{b}/risks"), json!({ "title": "B risk", "probability": 3, "impact": 3 })).await;
        let issue = made(&request, &format!("/api/plans/{b}/issues"), json!({ "title": "B issue" })).await;
        let meeting = made(&request, &format!("/api/plans/{b}/meetings"), json!({ "title": "B meeting", "scheduled_on": "2026-12-02" })).await;
        let lesson = made(
            &request,
            &format!("/api/plans/{b}/lessons"),
            json!({ "occasion": "closure", "category": "went_well", "summary": "B lesson" }),
        )
        .await;

        // (method, path under plan A with B's id, body). Every one must be 404.
        let cases: Vec<(&str, String, Value)> = vec![
            ("GET", format!("/api/plans/{a}/tasks/{task}"), json!(null)),
            ("PUT", format!("/api/plans/{a}/tasks/{task}"), json!({ "title": "hijacked" })),
            ("DELETE", format!("/api/plans/{a}/tasks/{task}"), json!(null)),
            ("GET", format!("/api/plans/{a}/tasks/{task}/costs"), json!(null)),
            ("POST", format!("/api/plans/{a}/tasks/{task}/costs"), json!({ "kind": "fixed", "description": "x", "amount_minor": 100, "currency": "GBP", "accrual": "end" })),
            ("PUT", format!("/api/plans/{a}/tasks/{task}/estimate"), json!({ "optimistic_days": 1, "likely_days": 2, "pessimistic_days": 3 })),
            ("PUT", format!("/api/plans/{a}/components/{component}"), json!({ "name": "hijacked" })),
            ("DELETE", format!("/api/plans/{a}/components/{component}"), json!(null)),
            ("PUT", format!("/api/plans/{a}/milestones/{milestone}"), json!({ "due": "2027-01-01" })),
            ("POST", format!("/api/plans/{a}/milestones/{milestone}/complete"), json!({})),
            ("PUT", format!("/api/plans/{a}/risks/{risk}"), json!({ "mitigation": "hijacked" })),
            ("GET", format!("/api/plans/{a}/issues/{issue}"), json!(null)),
            ("PUT", format!("/api/plans/{a}/issues/{issue}"), json!({ "title": "hijacked" })),
            ("DELETE", format!("/api/plans/{a}/issues/{issue}"), json!(null)),
            ("GET", format!("/api/plans/{a}/meetings/{meeting}"), json!(null)),
            ("GET", format!("/api/plans/{a}/meetings/{meeting}/pdf"), json!(null)),
            ("POST", format!("/api/plans/{a}/meetings/{meeting}/cancel"), json!(null)),
            ("POST", format!("/api/plans/{a}/meetings/{meeting}/held"), json!({ "minutes": "hijacked" })),
            ("POST", format!("/api/plans/{a}/lessons/{lesson}/withdraw"), json!(null)),
        ];
        let mut offenders = Vec::new();
        for (method, path, body) in &cases {
            let response = match *method {
                "GET" => request.get(path).await,
                "DELETE" => request.delete(path).await,
                "PUT" => request.put(path).json(body).await,
                _ if body.is_null() => request.post(path).await,
                _ => request.post(path).json(body).await,
            };
            if response.status_code() != 404 {
                offenders.push(format!("{method} {path} answered {}", response.status_code()));
            }
        }
        assert!(offenders.is_empty(), "plan B's records are reachable through plan A:\n{}", offenders.join("\n"));

        // And nothing of plan B changed.
        let t: Value = request.get(&format!("/api/plans/{b}/tasks/{task}")).await.json();
        assert_eq!(t["title"], "B task");
        let m: Value = request.get(&format!("/api/plans/{b}/meetings/{meeting}")).await.json();
        assert_eq!(m["status"], "planned");
        let i: Value = request.get(&format!("/api/plans/{b}/issues/{issue}")).await.json();
        assert_eq!(i["title"], "B issue");
        let l: Value = request.get(&format!("/api/plans/{b}/lessons")).await.json();
        assert_eq!(l.as_array().unwrap().len(), 1, "the lesson was not withdrawn");
    })
    .await;
}

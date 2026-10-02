//! Per-plan feature-module request tests (RM-7): a disabled module's
//! plan-scoped routes answer 404 while the data survives, an unconfigured
//! plan behaves as before, the personal preset exposes only tasks and time,
//! the toggle is audited, and a deployment default can narrow every plan.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use project_portfolio_management_service::models::_entities::audit_logs;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// A plan nobody configured exposes everything; switching `risks` off
// hides its routes (all methods) with 404 module_disabled but leaves every
// other route, the plan record and the stored risk alone; re-enabling
// restores the risk; each change is audited.
async fn a_disabled_module_hides_its_routes_but_never_its_data() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let p = plan(&request, "Apollo").await;
        let before: Value = request.get(&format!("/api/plans/{p}/modules")).await.json();
        assert_eq!(before["source"], "default", "unconfigured ⇒ the default");
        assert_eq!(
            before["modules"].as_array().unwrap().len(),
            6,
            "all modules"
        );

        request
            .post(&format!("/api/plans/{p}/risks"))
            .json(&json!({ "title": "Supplier failure", "probability": 3, "impact": 4 }))
            .await
            .assert_status_ok();

        let put: Value = request
            .put(&format!("/api/plans/{p}/modules"))
            .json(&json!({ "modules": ["tasks", "time"] }))
            .await
            .json();
        assert_eq!(put["source"], "plan");
        for (method, path) in [("get", "risks"), ("post", "risks"), ("get", "budget-lines")] {
            let url = format!("/api/plans/{p}/{path}");
            let r = match method {
                "get" => request.get(&url).await,
                _ => request.post(&url).json(&json!({})).await,
            };
            assert_eq!(r.status_code(), 404, "{method} {path}");
            let body: Value = r.json();
            assert_eq!(body["error"], "module_disabled");
        }
        // Everything else is untouched.
        request
            .get(&format!("/api/plans/{p}"))
            .await
            .assert_status_ok();
        request
            .get(&format!("/api/plans/{p}/tasks"))
            .await
            .assert_status_ok();
        request
            .get(&format!("/api/plans/{p}/time-entries"))
            .await
            .assert_status_ok();
        request
            .get(&format!("/api/plans/{p}/audit"))
            .await
            .assert_status_ok();

        // Reset ⇒ the risk written earlier is intact.
        let reset: Value = request
            .put(&format!("/api/plans/{p}/modules"))
            .json(&json!({ "reset": true }))
            .await
            .json();
        assert_eq!(reset["source"], "default");
        let risks: Value = request.get(&format!("/api/plans/{p}/risks")).await.json();
        assert!(
            format!("{risks}").contains("Supplier failure"),
            "data survived: {risks}"
        );

        let audit = audit_logs::Entity::find()
            .filter(audit_logs::Column::EntityPid.eq(uuid::Uuid::parse_str(&p).unwrap()))
            .filter(audit_logs::Column::Action.eq("plan_modules_changed"))
            .all(&ctx.db)
            .await
            .expect("audit");
        assert_eq!(audit.len(), 2, "set and reset are both audited");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// The personal preset at creation exposes only tasks and time; bad input
// is refused; an unknown plan is 404.
async fn the_personal_preset_exposes_only_tasks_and_time() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let created: Value = request
            .post("/api/plans?modules_preset=personal")
            .json(&json!({ "kind": "Project", "name": "My own work" }))
            .await
            .json();
        let p = created["pid"].as_str().expect("pid").to_string();
        for ok in ["tasks", "time-entries"] {
            request
                .get(&format!("/api/plans/{p}/{ok}"))
                .await
                .assert_status_ok();
        }
        for hidden in ["risks", "budget-lines", "milestones", "gate-reviews"] {
            assert_eq!(
                request
                    .get(&format!("/api/plans/{p}/{hidden}"))
                    .await
                    .status_code(),
                404,
                "{hidden}"
            );
        }
        let view: Value = request.get(&format!("/api/plans/{p}/modules")).await.json();
        assert_eq!(view["source"], "plan");
        assert_eq!(view["modules"], json!(["tasks", "time"]));

        // A bad preset creates no plan at all.
        assert_eq!(
            request
                .post("/api/plans?modules_preset=nope")
                .json(&json!({ "kind": "Project", "name": "Never created" }))
                .await
                .status_code(),
            422
        );
        let all: Value = request.get("/api/plans").await.json();
        assert!(!format!("{all}").contains("Never created"));

        for body in [
            json!({ "modules": ["tasks", "nope"] }),
            json!({ "preset": "enterprise" }),
            json!({}),
            json!({ "modules": ["tasks"], "reset": true }),
        ] {
            assert_eq!(
                request
                    .put(&format!("/api/plans/{p}/modules"))
                    .json(&body)
                    .await
                    .status_code(),
                422,
                "{body}"
            );
        }
        assert_eq!(
            request
                .get("/api/plans/3f2c6b7e-0000-4000-8000-000000000bad/modules")
                .await
                .status_code(),
            404
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// A deployment default narrows every unconfigured plan, and a plan's own
// choice overrides it.
async fn the_deployment_default_applies_until_a_plan_chooses() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request, "Apollo").await;
        // SAFETY: `#[serial]` — no other test runs while this is set, and
        // it is removed before the closure ends.
        unsafe {
            std::env::set_var(
                "PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES",
                "tasks,milestones",
            );
        }
        let narrowed = request
            .get(&format!("/api/plans/{p}/risks"))
            .await
            .status_code();
        let milestones = request
            .get(&format!("/api/plans/{p}/milestones"))
            .await
            .status_code();
        request
            .put(&format!("/api/plans/{p}/modules"))
            .json(&json!({ "modules": ["risks"] }))
            .await
            .assert_status_ok();
        let own = request
            .get(&format!("/api/plans/{p}/risks"))
            .await
            .status_code();
        let own_milestones = request
            .get(&format!("/api/plans/{p}/milestones"))
            .await
            .status_code();
        unsafe {
            std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES");
        }
        assert_eq!(narrowed, 404, "risks is outside the narrowed default");
        assert_eq!(milestones, 200, "milestones is inside it");
        assert_eq!(own, 200, "the plan's own set overrides the default");
        assert_eq!(own_milestones, 404);
    })
    .await;
}

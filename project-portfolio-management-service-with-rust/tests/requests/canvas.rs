//! Strategy-canvas request tests (LT-1): every built-in template
//! round-trips with all its boxes reported (empty ones at zero), an
//! unknown box is refused, promotion writes the target plus an audit row
//! naming the source item and is refused a second time, and deleting a
//! canvas soft-deletes its items.
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

async fn canvas(request: &axum_test::TestServer, template: &str, plan_pid: Option<&str>) -> String {
    let mut body = json!({ "template": template, "title": format!("{template} canvas") });
    if let Some(p) = plan_pid {
        body["plan_pid"] = json!(p);
    }
    let created: Value = request.post("/api/canvases").json(&body).await.json();
    created["pid"].as_str().expect("canvas pid").to_string()
}

async fn item(request: &axum_test::TestServer, canvas: &str, key: &str, body: &str) -> String {
    let created: Value = request
        .post(&format!("/api/canvases/{canvas}/items"))
        .json(&json!({ "box": key, "body": body }))
        .await
        .json();
    created["pid"].as_str().expect("item pid").to_string()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// Each built-in template accepts an item in every one of its boxes and
// reports every box, with the right counts.
async fn every_template_round_trips_with_all_boxes_reported() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let templates: Value = request.get("/api/canvas-templates").await.json();
        let templates = templates.as_array().expect("templates");
        assert!(templates.len() >= 7);
        for t in templates {
            let key = t["key"].as_str().expect("key");
            let boxes = t["boxes"].as_array().expect("boxes");
            let c = canvas(&request, key, None).await;
            // Fill every box except the last.
            for b in &boxes[..boxes.len() - 1] {
                item(&request, &c, b["key"].as_str().expect("box"), "note").await;
            }
            let got: Value = request.get(&format!("/api/canvases/{c}")).await.json();
            let reported = got["boxes"].as_array().expect("boxes");
            assert_eq!(reported.len(), boxes.len(), "{key}: every box reported");
            assert_eq!(
                reported.last().expect("last")["count"],
                0,
                "{key}: empty box at zero"
            );
            assert_eq!(reported[0]["count"], 1);
        }
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// An unknown template, an unknown box, a blank body and a foreign
// owner scheme are all refused 422; another canvas's item is a 404.
async fn invalid_canvases_and_items_are_refused() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        assert_eq!(
            request
                .post("/api/canvases")
                .json(&json!({ "template": "nope", "title": "x" }))
                .await
                .status_code(),
            422
        );
        let c = canvas(&request, "swot", None).await;
        let other = canvas(&request, "swot", None).await;
        for body in [
            json!({ "box": "problem", "body": "wrong template box" }),
            json!({ "box": "strengths", "body": "   " }),
            json!({ "box": "strengths", "body": "ok", "owner_ref": "email:a@b.c" }),
        ] {
            assert_eq!(
                request
                    .post(&format!("/api/canvases/{c}/items"))
                    .json(&body)
                    .await
                    .status_code(),
                422,
                "{body}"
            );
        }
        let it = item(&request, &c, "strengths", "ok").await;
        assert_eq!(
            request
                .delete(&format!("/api/canvases/{other}/items/{it}"))
                .await
                .status_code(),
            404,
            "an item is addressable only through its own canvas"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// Promotion creates the target and an audit row naming the source item,
// links the item back, and is refused (409) the second time.
async fn promotion_writes_the_target_audits_the_source_and_is_not_repeatable() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let p = plan(&request, "Apollo").await;
        let c = canvas(&request, "swot", Some(&p)).await;

        // Idea.
        let it = item(
            &request,
            &c,
            "opportunities",
            "Open a new market\nlong detail",
        )
        .await;
        let promoted: Value = request
            .post(&format!("/api/canvases/{c}/items/{it}/promote"))
            .json(&json!({ "target": "idea" }))
            .await
            .json();
        assert_eq!(promoted["kind"], "idea");
        let idea_pid = promoted["pid"].as_str().expect("idea pid").to_string();
        let ideas: Value = request.get("/api/ideas").await.json();
        assert!(
            ideas
                .as_array()
                .expect("ideas")
                .iter()
                .any(|i| i["pid"] == idea_pid && i["title"] == "Open a new market"),
            "title is the item's first line"
        );
        let rows = audit_logs::Entity::find()
            .filter(audit_logs::Column::EntityPid.eq(uuid::Uuid::parse_str(&idea_pid).unwrap()))
            .filter(audit_logs::Column::Action.eq("idea_from_canvas_item"))
            .all(&ctx.db)
            .await
            .expect("audit");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].snapshot.as_ref().expect("snapshot")["canvas_item_pid"],
            it
        );
        assert_eq!(
            request
                .post(&format!("/api/canvases/{c}/items/{it}/promote"))
                .json(&json!({ "target": "objective" }))
                .await
                .status_code(),
            409,
            "never promoted twice"
        );

        // Objective.
        let it2 = item(&request, &c, "strengths", "Strong brand").await;
        request
            .post(&format!("/api/canvases/{c}/items/{it2}/promote"))
            .json(&json!({ "target": "objective" }))
            .await
            .assert_status_ok();

        // Risk: probability / impact are required, never defaulted.
        let it3 = item(&request, &c, "threats", "Supplier failure").await;
        assert_eq!(
            request
                .post(&format!("/api/canvases/{c}/items/{it3}/promote"))
                .json(&json!({ "target": "risk" }))
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .post(&format!("/api/canvases/{c}/items/{it3}/promote"))
                .json(&json!({ "target": "risk", "probability": 9, "impact": 2 }))
                .await
                .status_code(),
            422
        );
        let risk: Value = request
            .post(&format!("/api/canvases/{c}/items/{it3}/promote"))
            .json(&json!({ "target": "risk", "probability": 3, "impact": 4 }))
            .await
            .json();
        assert_eq!(risk["kind"], "risk");
        let risks: Value = request.get(&format!("/api/plans/{p}/risks")).await.json();
        assert!(format!("{risks}").contains("Supplier failure"));

        // A free-standing canvas cannot promote to a risk.
        let free = canvas(&request, "swot", None).await;
        let fi = item(&request, &free, "threats", "x").await;
        assert_eq!(
            request
                .post(&format!("/api/canvases/{free}/items/{fi}/promote"))
                .json(&json!({ "target": "risk", "probability": 1, "impact": 1 }))
                .await
                .status_code(),
            422
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// Deleting a canvas hides it and its items; the list no longer shows it.
async fn deleting_a_canvas_soft_deletes_its_items() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let c = canvas(&request, "lean", None).await;
        item(&request, &c, "problem", "p").await;
        item(&request, &c, "solution", "s").await;
        request
            .delete(&format!("/api/canvases/{c}"))
            .await
            .assert_status_ok();
        assert_eq!(request.get(&format!("/api/canvases/{c}")).await.status_code(), 404);
        let live = project_portfolio_management_service::models::_entities::canvas_items::Entity::find()
            .filter(
                project_portfolio_management_service::models::_entities::canvas_items::Column::CanvasPid
                    .eq(uuid::Uuid::parse_str(&c).unwrap()),
            )
            .filter(
                project_portfolio_management_service::models::_entities::canvas_items::Column::DeletedAt
                    .is_null(),
            )
            .all(&ctx.db)
            .await
            .expect("items");
        assert!(live.is_empty(), "items soft-deleted with their canvas");
    })
    .await;
}

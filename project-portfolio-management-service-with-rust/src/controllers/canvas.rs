//! Strategy canvases (LT-1): create a canvas from a built-in template,
//! add / edit / remove items in its boxes, and **promote** an item into
//! an idea, a risk or an objective. Pure rules and the templates live in
//! [`crate::canvas`]; every mutation audits.

use super::conflict;
use super::{db_err, unprocessable};
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::governance::{find_item, valid_ref};
use crate::auth::MaybeAuthUser;
use crate::canvas::{self as rules, PromoteTarget};
use crate::governance as gov_rules;
use crate::models::_entities::{canvas_items, canvases, ideas, objectives, risks};
use crate::models::audit_logs::Model as AuditModel;
use crate::validation::MAX_TEXT_LEN;

async fn find_canvas(ctx: &AppContext, pid: &str) -> Result<canvases::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    canvases::Entity::find()
        .filter(canvases::Column::Pid.eq(pid))
        .filter(canvases::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// An item that belongs to `canvas` (a pid from another canvas is `404`).
async fn find_item_of(
    ctx: &AppContext,
    canvas: &canvases::Model,
    item_pid: &str,
) -> Result<canvas_items::Model> {
    let pid = Uuid::parse_str(item_pid).map_err(|_| Error::NotFound)?;
    canvas_items::Entity::find()
        .filter(canvas_items::Column::Pid.eq(pid))
        .filter(canvas_items::Column::CanvasPid.eq(canvas.pid))
        .filter(canvas_items::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

async fn live_items(ctx: &AppContext, canvas_pid: Uuid) -> Result<Vec<canvas_items::Model>> {
    canvas_items::Entity::find()
        .filter(canvas_items::Column::CanvasPid.eq(canvas_pid))
        .filter(canvas_items::Column::DeletedAt.is_null())
        .order_by_asc(canvas_items::Column::Position)
        .order_by_asc(canvas_items::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// `GET /api/canvas-templates` — every built-in template and its boxes.
#[debug_handler]
async fn list_templates() -> Result<Response> {
    format::json(rules::TEMPLATES)
}

/// `POST /api/canvases` body.
#[derive(Debug, Deserialize)]
struct CanvasPayload {
    template: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    /// Optional plan scope; absent ⇒ a free-standing canvas.
    #[serde(default)]
    plan_pid: Option<String>,
}

/// `POST /api/canvases`.
#[debug_handler]
async fn create_canvas(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<CanvasPayload>,
) -> Result<Response> {
    let mut problems = rules::canvas_problems(&payload.template, &payload.title);
    if payload
        .description
        .as_deref()
        .is_some_and(|d| d.chars().count() > rules::MAX_BODY_LEN)
    {
        problems.push(format!(
            "description: exceeds {} characters",
            rules::MAX_BODY_LEN
        ));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let plan_pid = match payload.plan_pid.as_deref() {
        Some(p) => Some(find_item(&ctx, p).await?.pid),
        None => None,
    };
    let row = canvases::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan_pid),
        template: ActiveValue::set(payload.template.clone()),
        title: ActiveValue::set(payload.title.clone()),
        description: ActiveValue::set(payload.description.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(&ctx.db, row.pid, "canvas_created", caller.actor(), None)
        .await
        .ok();
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/canvases?plan=&template=` query.
#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    plan: Option<String>,
    #[serde(default)]
    template: Option<String>,
}

/// `GET /api/canvases` — live canvases, optionally by plan / template.
#[debug_handler]
async fn list_canvases(
    State(ctx): State<AppContext>,
    Query(params): Query<ListParams>,
) -> Result<Response> {
    let mut query = canvases::Entity::find().filter(canvases::Column::DeletedAt.is_null());
    if let Some(plan) = params.plan.as_deref() {
        let pid = Uuid::parse_str(plan).map_err(|_| unprocessable("plan must be a UUID"))?;
        query = query.filter(canvases::Column::PlanPid.eq(pid));
    }
    if let Some(template) = params.template {
        query = query.filter(canvases::Column::Template.eq(template));
    }
    let rows = query
        .order_by_asc(canvases::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows)
}

/// `GET /api/canvases/{pid}` — the canvas, its template's boxes (**all of
/// them, empty ones at count 0**), and its items.
#[debug_handler]
async fn get_canvas(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let canvas = find_canvas(&ctx, &pid).await?;
    let items = live_items(&ctx, canvas.pid).await?;
    let boxes = rules::summarize(&canvas.template, items.iter().map(|i| i.box_key.as_str()));
    format::json(serde_json::json!({
        "canvas": canvas,
        "boxes": boxes,
        "items": items,
    }))
}

/// `DELETE /api/canvases/{pid}` — soft-delete the canvas and its items.
#[debug_handler]
async fn delete_canvas(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &pid).await?;
    let canvas_pid = canvas.pid;
    let now: sea_orm::prelude::DateTimeWithTimeZone = chrono::Utc::now().into();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    canvas_items::Entity::update_many()
        .col_expr(canvas_items::Column::DeletedAt, Expr::value(now))
        .filter(canvas_items::Column::CanvasPid.eq(canvas_pid))
        .filter(canvas_items::Column::DeletedAt.is_null())
        .exec(&txn)
        .await
        .map_err(db_err)?;
    let mut active: canvases::ActiveModel = canvas.into();
    active.deleted_at = ActiveValue::set(Some(now));
    active.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, canvas_pid, "canvas_deleted", caller.actor(), None)
        .await
        .map_err(db_err_model)?;
    txn.commit().await.map_err(db_err)?;
    format::empty_json()
}

fn db_err_model(e: ModelError) -> Error {
    Error::Model(e)
}

/// `POST …/items` body.
#[derive(Debug, Deserialize)]
struct ItemPayload {
    #[serde(rename = "box")]
    box_key: String,
    body: String,
    #[serde(default)]
    owner_ref: Option<String>,
}

fn owner_problem(owner: Option<&str>) -> Option<String> {
    owner
        .filter(|o| !valid_ref(o, &["worker", "person"]))
        .map(|_| "owner_ref must be a worker:/person: URN".to_string())
}

/// `POST /api/canvases/{pid}/items`.
#[debug_handler]
async fn add_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<ItemPayload>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &pid).await?;
    let mut problems = rules::item_problems(&canvas.template, &payload.box_key, &payload.body);
    problems.extend(owner_problem(payload.owner_ref.as_deref()));
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = live_items(&ctx, canvas.pid).await?;
    if existing.len() >= rules::MAX_ITEMS {
        return Err(unprocessable(&format!(
            "a canvas holds at most {} items",
            rules::MAX_ITEMS
        )));
    }
    let position = existing
        .iter()
        .filter(|i| i.box_key == payload.box_key)
        .map(|i| i.position)
        .max()
        .map_or(0, |p| p + 1);
    let row = canvas_items::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        canvas_pid: ActiveValue::set(canvas.pid),
        box_key: ActiveValue::set(payload.box_key.clone()),
        body: ActiveValue::set(payload.body.clone()),
        owner_ref: ActiveValue::set(payload.owner_ref.clone()),
        position: ActiveValue::set(position),
        promoted_to_kind: ActiveValue::set(None),
        promoted_to_pid: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(&ctx.db, row.pid, "canvas_item_added", caller.actor(), None)
        .await
        .ok();
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `PUT …/items/{item_pid}` body — every field optional.
#[derive(Debug, Deserialize)]
struct ItemUpdate {
    #[serde(default, rename = "box")]
    box_key: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    owner_ref: Option<String>,
    #[serde(default)]
    position: Option<i32>,
}

/// `PUT /api/canvases/{pid}/items/{item_pid}` — edit or move an item.
#[debug_handler]
async fn update_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, item_pid)): Path<(String, String)>,
    Json(payload): Json<ItemUpdate>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &pid).await?;
    let item = find_item_of(&ctx, &canvas, &item_pid).await?;
    let box_key = payload
        .box_key
        .clone()
        .unwrap_or_else(|| item.box_key.clone());
    let body = payload.body.clone().unwrap_or_else(|| item.body.clone());
    let mut problems = rules::item_problems(&canvas.template, &box_key, &body);
    problems.extend(owner_problem(payload.owner_ref.as_deref()));
    if payload.position.is_some_and(|p| p < 0) {
        problems.push("position must be ≥ 0".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let item_pid = item.pid;
    let mut active: canvas_items::ActiveModel = item.into();
    active.box_key = ActiveValue::set(box_key);
    active.body = ActiveValue::set(body);
    if let Some(owner) = payload.owner_ref {
        active.owner_ref = ActiveValue::set(Some(owner));
    }
    if let Some(position) = payload.position {
        active.position = ActiveValue::set(position);
    }
    let row = active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        item_pid,
        "canvas_item_updated",
        caller.actor(),
        None,
    )
    .await
    .ok();
    format::json(row)
}

/// `DELETE /api/canvases/{pid}/items/{item_pid}` — soft-delete one item.
#[debug_handler]
async fn delete_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, item_pid)): Path<(String, String)>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &pid).await?;
    let item = find_item_of(&ctx, &canvas, &item_pid).await?;
    let item_pid = item.pid;
    let mut active: canvas_items::ActiveModel = item.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        item_pid,
        "canvas_item_deleted",
        caller.actor(),
        None,
    )
    .await
    .ok();
    format::empty_json()
}

/// `POST …/items/{item_pid}/promote` body.
#[derive(Debug, Deserialize)]
struct PromotePayload {
    /// `idea` | `risk` | `objective`.
    target: String,
    /// Risk only — 1–5. Never defaulted: the author must say.
    #[serde(default)]
    probability: Option<i32>,
    /// Risk only — 1–5.
    #[serde(default)]
    impact: Option<i32>,
}

/// `POST /api/canvases/{pid}/items/{item_pid}/promote` — turn an item
/// into an idea, risk or objective. One transaction: the target row, the
/// item's back-link and **both** audit rows commit together or not at
/// all. An item already promoted is `409`, never promoted twice.
#[debug_handler]
#[allow(clippy::too_many_lines)] // one match over three targets, one transaction
async fn promote_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, item_pid)): Path<(String, String)>,
    Json(payload): Json<PromotePayload>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &pid).await?;
    let item = find_item_of(&ctx, &canvas, &item_pid).await?;
    let target = PromoteTarget::parse(&payload.target)
        .ok_or_else(|| unprocessable("target must be one of: idea, risk, objective"))?;
    if let (Some(kind), Some(to)) = (&item.promoted_to_kind, item.promoted_to_pid) {
        return Err(conflict(&format!("already promoted to {kind} {to}")));
    }
    let title = rules::title_from_body(&item.body, MAX_TEXT_LEN);
    if title.is_empty() {
        return Err(unprocessable("the item has no text to derive a title from"));
    }
    let actor = caller.actor();
    let new_pid = Uuid::new_v4();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    match target {
        PromoteTarget::Idea => {
            ideas::ActiveModel {
                pid: ActiveValue::set(new_pid),
                title: ActiveValue::set(title),
                pitch: ActiveValue::set(Some(item.body.clone())),
                tags: ActiveValue::set(serde_json::json!([canvas.template])),
                votes: ActiveValue::set(0),
                status: ActiveValue::set("open".to_string()),
                converted_proposal_pid: ActiveValue::set(None),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?;
        }
        PromoteTarget::Objective => {
            objectives::ActiveModel {
                pid: ActiveValue::set(new_pid),
                title: ActiveValue::set(title),
                description: ActiveValue::set(Some(item.body.clone())),
                period: ActiveValue::set(None),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?;
        }
        PromoteTarget::Risk => {
            let plan_pid = canvas.plan_pid.ok_or_else(|| {
                unprocessable("a risk needs a plan; this canvas is free-standing")
            })?;
            let (Some(probability), Some(impact)) = (payload.probability, payload.impact) else {
                return Err(unprocessable(
                    "probability and impact (1–5) are required to promote to a risk",
                ));
            };
            gov_rules::risk_exposure(probability, impact).map_err(|e| unprocessable(&e))?;
            risks::ActiveModel {
                pid: ActiveValue::set(new_pid),
                plan_pid: ActiveValue::set(plan_pid),
                title: ActiveValue::set(title),
                description: ActiveValue::set(Some(item.body.clone())),
                probability: ActiveValue::set(probability),
                impact: ActiveValue::set(impact),
                status: ActiveValue::set("open".to_string()),
                owner_ref: ActiveValue::set(item.owner_ref.clone()),
                mitigation: ActiveValue::set(None),
                review_date: ActiveValue::set(None),
                escalated_at: ActiveValue::set(None),
                category: ActiveValue::set(None),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?;
        }
    }
    let source_pid = item.pid;
    let mut active: canvas_items::ActiveModel = item.into();
    active.promoted_to_kind = ActiveValue::set(Some(target.as_str().to_string()));
    active.promoted_to_pid = ActiveValue::set(Some(new_pid));
    active.update(&txn).await.map_err(db_err)?;
    // The target's audit row names its source, so provenance survives
    // even if the canvas is later deleted.
    AuditModel::record(
        &txn,
        new_pid,
        &format!("{}_from_canvas_item", target.as_str()),
        actor,
        Some(serde_json::json!({
            "canvas_pid": canvas.pid.to_string(),
            "canvas_item_pid": source_pid.to_string(),
            "template": canvas.template,
        })),
    )
    .await
    .map_err(db_err_model)?;
    AuditModel::record(
        &txn,
        source_pid,
        "canvas_item_promoted",
        actor,
        Some(serde_json::json!({ "to_kind": target.as_str(), "to_pid": new_pid.to_string() })),
    )
    .await
    .map_err(db_err_model)?;
    txn.commit().await.map_err(db_err)?;
    format::json(serde_json::json!({
        "kind": target.as_str(),
        "pid": new_pid.to_string(),
    }))
}

/// The canvas routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/canvas-templates", get(list_templates))
        .add("/canvases", post(create_canvas))
        .add("/canvases", get(list_canvases))
        .add("/canvases/{pid}", get(get_canvas))
        .add("/canvases/{pid}", delete(delete_canvas))
        .add("/canvases/{pid}/items", post(add_item))
        .add("/canvases/{pid}/items/{item_pid}", put(update_item))
        .add("/canvases/{pid}/items/{item_pid}", delete(delete_item))
        .add(
            "/canvases/{pid}/items/{item_pid}/promote",
            post(promote_item),
        )
}

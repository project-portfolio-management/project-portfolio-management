//! Strategy canvases (LT-1): create a canvas from a built-in template,
//! add / edit / remove items in its boxes, and **promote** an item into
//! an idea, a risk or an objective. Pure rules and the templates live in
//! [`crate::canvas`]; every mutation audits.

use super::conflict;
use super::unprocessable;
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::{find_item, valid_ref};
use crate::auth::MaybeAuthUser;
use crate::canvas::{self as rules, PromoteTarget};
use crate::governance as gov_rules;
use crate::models::_entities::{canvas_items, canvases};
use crate::models::audit_logs::Model as AuditModel;
use crate::validation::MAX_TEXT_LEN;

/// Loads a live (not soft-deleted) canvas by public id, `404` when absent, withdrawn, or on a plan the caller cannot read.
async fn find_canvas(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<canvases::Model> {
    let pid = super::parse_pid(pid)?;
    let canvas: canvases::Model =
        sqlx::query_as("SELECT * FROM canvases WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    super::plan_access::require_readable_if_any(ctx, caller, canvas.plan_pid).await?;
    Ok(canvas)
}

/// An item that belongs to `canvas` (a pid from another canvas is `404`).
async fn find_item_of(
    ctx: &AppContext,
    canvas: &canvases::Model,
    item_pid: &str,
) -> Result<canvas_items::Model> {
    let pid = super::parse_pid(item_pid)?;
    sqlx::query_as(
        "SELECT * FROM canvas_items WHERE pid = $1 AND canvas_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .bind(canvas.pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)
}

/// Live items of a canvas in display order (position, then insertion order as a tie-break).
async fn live_items(ctx: &AppContext, canvas_pid: Uuid) -> Result<Vec<canvas_items::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM canvas_items WHERE canvas_pid = $1 AND deleted_at IS NULL ORDER BY position, id",
    )
    .bind(canvas_pid)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// `GET /api/canvas-templates` — every built-in template and its boxes.
#[debug_handler]
async fn list_templates() -> Result<Response> {
    format::json(rules::TEMPLATES)
}

/// `POST /api/canvases` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CanvasPayload {
    /// Built-in template key; must be one of the templates in [`crate::canvas`].
    template: String,
    /// Canvas title; required.
    title: String,
    /// Optional longer description; absent means none.
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
    super::plan_access::require_readable_if_any(&ctx, &caller, plan_pid).await?;
    let row_pid = Uuid::new_v4();
    let mut tx = crate::db::begin(&ctx).await?;
    // The columns `SeaORM`'s insert set, no more.
    sqlx::query(
        "INSERT INTO canvases (pid, plan_pid, template, title, description, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL)",
    )
    .bind(row_pid)
    .bind(plan_pid)
    .bind(&payload.template)
    .bind(&payload.title)
    .bind(&payload.description)
    .execute(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row_pid, "canvas_created", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/canvases?plan=&template=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// Optional plan public id filter; absent lists canvases of every scope.
    #[serde(default)]
    plan: Option<String>,
    /// Optional template key filter.
    #[serde(default)]
    template: Option<String>,
}

/// `GET /api/canvases` — live canvases, optionally by plan / template. A canvas on a plan the caller cannot read is left out.
#[debug_handler]
async fn list_canvases(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<ListParams>,
) -> Result<Response> {
    let mut query = sqlx::QueryBuilder::new("SELECT * FROM canvases WHERE deleted_at IS NULL");
    if let Some(plan) = params.plan.as_deref() {
        let pid = Uuid::parse_str(plan).map_err(|_| unprocessable("plan must be a UUID"))?;
        query.push(" AND plan_pid = ").push_bind(pid);
    }
    if let Some(template) = params.template {
        query.push(" AND template = ").push_bind(template);
    }
    query.push(" ORDER BY id");
    let rows: Vec<canvases::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let rows = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .keep(rows);
    format::json(rows)
}

/// `GET /api/canvases/{pid}` — the canvas, its template's boxes (**all of
/// them, empty ones at count 0**), and its items.
#[debug_handler]
async fn get_canvas(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &caller, &pid).await?;
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
    let canvas = find_canvas(&ctx, &caller, &pid).await?;
    let canvas_pid = canvas.pid;
    let now = chrono::DateTime::<chrono::FixedOffset>::from(chrono::Utc::now());
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "UPDATE canvas_items SET deleted_at = $1 WHERE canvas_pid = $2 AND deleted_at IS NULL",
    )
    .bind(now)
    .bind(canvas_pid)
    .execute(tx.conn())
    .await?;
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>("UPDATE canvases SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(now)
        .bind(canvas.id)
        .fetch_one(tx.conn())
        .await?;
    // Passed on (an outage stays `503`), as the `SeaORM` error was.
    AuditModel::record_on(
        tx.conn(),
        canvas_pid,
        "canvas_deleted",
        caller.actor(),
        None,
    )
    .await?;
    tx.commit().await?;
    format::empty_json()
}

/// `POST …/items` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ItemPayload {
    /// Key of the box (a template section) the item sits in; must exist in the canvas's template.
    #[serde(rename = "box")]
    box_key: String,
    /// Item text; validated by `rules::item_problems`.
    body: String,
    /// Optional owner as a `worker:` / `person:` URN; absent means unowned.
    #[serde(default)]
    owner_ref: Option<String>,
}

/// Returns a problem message when an owner reference is present but is not a `worker:` / `person:` URN, else `None`.
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
    let canvas = find_canvas(&ctx, &caller, &pid).await?;
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
    // Append to the end of the target box: one past the highest existing position there.
    let position = existing
        .iter()
        .filter(|i| i.box_key == payload.box_key)
        .map(|i| i.position)
        .max()
        .map_or(0, |p| p + 1);
    let row_pid = Uuid::new_v4();
    let mut tx = crate::db::begin(&ctx).await?;
    // The columns `SeaORM`'s insert set, no more.
    sqlx::query(
        "INSERT INTO canvas_items
             (pid, canvas_pid, box_key, body, owner_ref, position, promoted_to_kind, promoted_to_pid, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, NULL, NULL, NULL)",
    )
    .bind(row_pid)
    .bind(canvas.pid)
    .bind(&payload.box_key)
    .bind(&payload.body)
    .bind(&payload.owner_ref)
    .bind(position)
    .execute(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "canvas_item_added",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `PUT …/items/{item_pid}` body — every field optional.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ItemUpdate {
    /// Move the item to another box of the same template; absent keeps its box.
    #[serde(default, rename = "box")]
    box_key: Option<String>,
    /// Replacement text; absent keeps it.
    #[serde(default)]
    body: Option<String>,
    /// Replacement owner URN; absent keeps it.
    #[serde(default)]
    owner_ref: Option<String>,
    /// New position within the box; absent keeps it.
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
    let canvas = find_canvas(&ctx, &caller, &pid).await?;
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
    // By id, the box and body always and the owner and position when given, as `SeaORM`'s update named; no row
    // back is an error, as its was.
    let mut update = sqlx::QueryBuilder::new("UPDATE canvas_items SET box_key = ");
    update.push_bind(box_key).push(", body = ").push_bind(body);
    if let Some(owner) = payload.owner_ref {
        update.push(", owner_ref = ").push_bind(owner);
    }
    if let Some(position) = payload.position {
        update.push(", position = ").push_bind(position);
    }
    update
        .push(" WHERE id = ")
        .push_bind(item.id)
        .push(" RETURNING *");
    let mut tx = crate::db::begin(&ctx).await?;
    let row: canvas_items::Model = update.build_query_as().fetch_one(tx.conn()).await?;
    AuditModel::record_on(
        tx.conn(),
        item_pid,
        "canvas_item_updated",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `DELETE /api/canvases/{pid}/items/{item_pid}` — soft-delete one item.
#[debug_handler]
async fn delete_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, item_pid)): Path<(String, String)>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &caller, &pid).await?;
    let item = find_item_of(&ctx, &canvas, &item_pid).await?;
    let item_pid = item.pid;
    let now = chrono::DateTime::<chrono::FixedOffset>::from(chrono::Utc::now());
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE canvas_items SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(now)
    .bind(item.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        item_pid,
        "canvas_item_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `POST …/items/{item_pid}/promote` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PromotePayload {
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
#[expect(clippy::too_many_lines)] // one match over three targets, one transaction
async fn promote_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, item_pid)): Path<(String, String)>,
    Json(payload): Json<PromotePayload>,
) -> Result<Response> {
    let canvas = find_canvas(&ctx, &caller, &pid).await?;
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
    let mut tx = crate::db::begin(&ctx).await?;
    // Each insert lists the columns `SeaORM`'s set, no more.
    match target {
        PromoteTarget::Idea => {
            sqlx::query(
                "INSERT INTO ideas (pid, title, pitch, tags, votes, status, converted_proposal_pid, deleted_at)
                 VALUES ($1, $2, $3, $4, 0, 'open', NULL, NULL)",
            )
            .bind(new_pid)
            .bind(&title)
            .bind(&item.body)
            .bind(serde_json::json!([canvas.template]))
            .execute(tx.conn())
            .await?;
        }
        PromoteTarget::Objective => {
            sqlx::query(
                "INSERT INTO objectives (pid, title, description, period, deleted_at)
                 VALUES ($1, $2, $3, NULL, NULL)",
            )
            .bind(new_pid)
            .bind(&title)
            .bind(&item.body)
            .execute(tx.conn())
            .await?;
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
            sqlx::query(
                "INSERT INTO risks
                     (pid, plan_pid, title, description, probability, impact, status, owner_ref, mitigation,
                      review_date, escalated_at, category, deleted_at)
                 VALUES ($1, $2, $3, $4, $5, $6, 'open', $7, NULL, NULL, NULL, NULL, NULL)",
            )
            .bind(new_pid)
            .bind(plan_pid)
            .bind(&title)
            .bind(&item.body)
            .bind(probability)
            .bind(impact)
            .bind(&item.owner_ref)
            .execute(tx.conn())
            .await?;
        }
    }
    let source_pid = item.pid;
    // By id, the two columns `SeaORM`'s update named; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE canvas_items SET promoted_to_kind = $1, promoted_to_pid = $2 WHERE id = $3 RETURNING id",
    )
    .bind(target.as_str())
    .bind(new_pid)
    .bind(item.id)
    .fetch_one(tx.conn())
    .await?;
    // The target's audit row names its source, so provenance survives
    // even if the canvas is later deleted. Both audit errors are passed on (an outage stays `503`), as the
    // `SeaORM` errors were.
    AuditModel::record_on(
        tx.conn(),
        new_pid,
        &format!("{}_from_canvas_item", target.as_str()),
        actor,
        Some(serde_json::json!({
            "canvas_pid": canvas.pid.to_string(),
            "canvas_item_pid": source_pid.to_string(),
            "template": canvas.template,
        })),
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        source_pid,
        "canvas_item_promoted",
        actor,
        Some(serde_json::json!({ "to_kind": target.as_str(), "to_pid": new_pid.to_string() })),
    )
    .await?;
    tx.commit().await?;
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

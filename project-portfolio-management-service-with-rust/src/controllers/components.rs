//! Task components (PL-11): named groupings of a plan's tasks. Pure rules
//! in [`crate::components`]. A task's `component_pid` is validated in the
//! task create/update path (it must name a component of the same plan);
//! deleting a component that still has tasks is refused, never silently
//! orphaning them.

use super::conflict;
use super::unprocessable;
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::{find_item, valid_ref};
use crate::auth::MaybeAuthUser;
use crate::components as rules;
use crate::models::_entities::components;
use crate::models::audit_logs::Model as AuditModel;

/// Whether a lead reference is a `worker:` or `person:` URN; people are held as references, never names.
fn lead_ok(s: &str) -> bool {
    valid_ref(s, &["worker", "person"])
}

/// A plan's components that are not deleted, in creation order.
async fn live_components(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<components::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM components WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(plan_pid)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// One live component of the given plan by id; `404` when it is missing, deleted or belongs to another plan.
async fn find_component(ctx: &AppContext, plan_pid: Uuid, pid: &str) -> Result<components::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as(
        "SELECT * FROM components WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .bind(plan_pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)
}

/// `POST/PUT` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ComponentPayload {
    /// Component name; unique within the plan, ignoring case.
    name: String,
    /// Reference to the lead as a `worker:` or `person:` URN; absent means no lead.
    #[serde(default)]
    lead_ref: Option<String>,
}

/// `POST /api/plans/{pid}/components`.
#[debug_handler]
async fn create_component(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<ComponentPayload>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let problems = rules::problems(&payload.name, payload.lead_ref.as_deref(), lead_ok);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = live_components(&ctx, plan.pid).await?;
    if existing.len() >= rules::MAX_PER_PLAN {
        return Err(unprocessable(&format!(
            "a plan holds at most {} components",
            rules::MAX_PER_PLAN
        )));
    }
    if existing
        .iter()
        .any(|c| c.name.to_lowercase() == payload.name.trim().to_lowercase())
    {
        return Err(conflict(
            "a component with this name already exists in the plan",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: components::Model = sqlx::query_as(
        "INSERT INTO components (pid, plan_pid, name, lead_ref, deleted_at)
         VALUES ($1, $2, $3, $4, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(payload.name.trim())
    .bind(&payload.lead_ref)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "component_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/plans/{pid}/components`.
#[debug_handler]
async fn list_components(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    format::json(live_components(&ctx, plan.pid).await?)
}

/// `PUT /api/plans/{pid}/components/{c_pid}` — rename / change the lead.
#[debug_handler]
async fn update_component(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, c_pid)): Path<(String, String)>,
    Json(payload): Json<ComponentPayload>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_component(&ctx, plan.pid, &c_pid).await?;
    let problems = rules::problems(&payload.name, payload.lead_ref.as_deref(), lead_ok);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let clash = live_components(&ctx, plan.pid)
        .await?
        .into_iter()
        .any(|c| c.pid != row.pid && c.name.to_lowercase() == payload.name.trim().to_lowercase());
    if clash {
        return Err(conflict(
            "a component with this name already exists in the plan",
        ));
    }
    let comp_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: components::Model =
        sqlx::query_as("UPDATE components SET name = $1, lead_ref = $2 WHERE id = $3 RETURNING *")
            .bind(payload.name.trim())
            .bind(&payload.lead_ref)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        comp_pid,
        "component_updated",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `DELETE /api/plans/{pid}/components/{c_pid}` — refused (409) while any
/// live task still names it.
#[debug_handler]
async fn delete_component(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, c_pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_component(&ctx, plan.pid, &c_pid).await?;
    let in_use: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM tasks WHERE component_pid = $1 AND deleted_at IS NULL",
    )
    .bind(row.pid)
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    if in_use > 0 {
        return Err(conflict(&format!(
            "{in_use} live task(s) still belong to this component; move them first"
        )));
    }
    let comp_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE components SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::Utc::now().fixed_offset())
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        comp_pid,
        "component_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `GET /api/plans/{pid}/components/summary` — open / done / total per
/// component, every component reported (empty ones at zero), plus an
/// `unassigned` bucket; the totals always sum to the plan's task count.
#[debug_handler]
async fn summary(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let comps: Vec<(Uuid, String)> = live_components(&ctx, plan.pid)
        .await?
        .into_iter()
        .map(|c| (c.pid, c.name))
        .collect();
    let facts: Vec<(Option<Uuid>, bool)> = sqlx::query_as(
        "SELECT component_pid, done_at IS NOT NULL FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let buckets = rules::summarize(&comps, &facts);
    format::json(serde_json::json!({
        "buckets": buckets,
        "total": facts.len(),
    }))
}

/// The component routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/components", post(create_component))
        .add("/plans/{pid}/components", get(list_components))
        .add("/plans/{pid}/components/summary", get(summary))
        .add("/plans/{pid}/components/{c_pid}", put(update_component))
        .add("/plans/{pid}/components/{c_pid}", delete(delete_component))
}

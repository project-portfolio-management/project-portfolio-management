//! Task components (PL-11): named groupings of a plan's tasks. Pure rules
//! in [`crate::components`]. A task's `component_pid` is validated in the
//! task create/update path (it must name a component of the same plan);
//! deleting a component that still has tasks is refused, never silently
//! orphaning them.

use super::conflict;
use super::{db_err, unprocessable};
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::Deserialize;
use uuid::Uuid;

use super::governance::{find_item, valid_ref};
use crate::auth::MaybeAuthUser;
use crate::components as rules;
use crate::models::_entities::{components, tasks};
use crate::models::audit_logs::Model as AuditModel;

fn lead_ok(s: &str) -> bool {
    valid_ref(s, &["worker", "person"])
}

async fn live_components(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<components::Model>> {
    components::Entity::find()
        .filter(components::Column::PlanPid.eq(plan_pid))
        .filter(components::Column::DeletedAt.is_null())
        .order_by_asc(components::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

async fn find_component(ctx: &AppContext, plan_pid: Uuid, pid: &str) -> Result<components::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    components::Entity::find()
        .filter(components::Column::Pid.eq(pid))
        .filter(components::Column::PlanPid.eq(plan_pid))
        .filter(components::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `POST/PUT` body.
#[derive(Debug, Deserialize)]
struct ComponentPayload {
    name: String,
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
    let row = components::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        name: ActiveValue::set(payload.name.trim().to_string()),
        lead_ref: ActiveValue::set(payload.lead_ref.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(&ctx.db, row.pid, "component_created", caller.actor(), None)
        .await
        .ok();
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
    let mut active: components::ActiveModel = row.into();
    active.name = ActiveValue::set(payload.name.trim().to_string());
    active.lead_ref = ActiveValue::set(payload.lead_ref.clone());
    let row = active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(&ctx.db, comp_pid, "component_updated", caller.actor(), None)
        .await
        .ok();
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
    let in_use = tasks::Entity::find()
        .filter(tasks::Column::ComponentPid.eq(row.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    if in_use > 0 {
        return Err(conflict(&format!(
            "{in_use} live task(s) still belong to this component; move them first"
        )));
    }
    let comp_pid = row.pid;
    let mut active: components::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(&ctx.db, comp_pid, "component_deleted", caller.actor(), None)
        .await
        .ok();
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
    let rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let facts: Vec<(Option<Uuid>, bool)> = rows
        .iter()
        .map(|t| (t.component_pid, t.done_at.is_some()))
        .collect();
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

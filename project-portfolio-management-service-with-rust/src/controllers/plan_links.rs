//! Typed links on a plan, or on one of its tasks (LT-8). Pure rules in [`crate::plan_links`]. The
//! service stores a link and returns it; it never fetches or renders one.

use super::{db_err, unprocessable};
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::plan_links;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_links as rules;

/// The live links of a plan (`task` is `None`) or of one of its tasks.
async fn live(ctx: &AppContext, plan: Uuid, task: Option<Uuid>) -> Result<Vec<plan_links::Model>> {
    plan_links::Entity::find()
        .filter(plan_links::Column::PlanPid.eq(plan))
        .filter(match task {
            Some(t) => plan_links::Column::TaskPid.eq(t),
            None => plan_links::Column::TaskPid.is_null(),
        })
        .filter(plan_links::Column::DeletedAt.is_null())
        .order_by_asc(plan_links::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// `POST /api/plans/{pid}/links` body.
#[derive(Debug, Deserialize)]
struct LinkBody {
    kind: String,
    url: String,
    label: String,
    /// The work-tracking system this points into (`jira`, `trello`, `asana`, `linear`, `monday`, ...).
    #[serde(default)]
    system: Option<String>,
    /// That system's own identifier for the thing (a Jira key, a board id).
    #[serde(default)]
    external_id: Option<String>,
}

/// `POST /api/plans/{pid}/links`.
#[debug_handler]
async fn add_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<LinkBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    add(&ctx, &caller, plan.pid, None, b).await
}

/// `POST /api/plans/{pid}/tasks/{tpid}/links`.
#[debug_handler]
async fn add_task_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, tpid)): Path<(String, String)>,
    Json(b): Json<LinkBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let task = super::engineering::find_task(&ctx, plan.pid, &tpid).await?;
    add(&ctx, &caller, plan.pid, Some(task.pid), b).await
}

async fn add(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    task: Option<Uuid>,
    b: LinkBody,
) -> Result<Response> {
    let problems = rules::problems(
        &b.kind,
        &b.url,
        &b.label,
        b.system.as_deref(),
        b.external_id.as_deref(),
    );
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = live(ctx, plan, task).await?;
    if existing.len() >= rules::MAX_PER_PLAN {
        return Err(unprocessable(&format!(
            "{} holds at most {} links",
            if task.is_some() { "a task" } else { "a plan" },
            rules::MAX_PER_PLAN
        )));
    }
    if existing.iter().any(|l| l.url == b.url) {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "this URL is already linked from the plan"),
        ));
    }
    let row = plan_links::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan),
        task_pid: ActiveValue::set(task),
        kind: ActiveValue::set(b.kind.clone()),
        url: ActiveValue::set(b.url.clone()),
        label: ActiveValue::set(b.label.trim().to_string()),
        system: ActiveValue::set(b.system.clone()),
        external_id: ActiveValue::set(b.external_id.clone()),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(&ctx.db, row.pid, "plan_link_added", caller.actor(), None)
        .await
        .ok();
    format::json(row)
}

/// `GET /api/plans/{pid}/links` query: narrow to one system and/or kind.
#[derive(Debug, Deserialize)]
struct ListQuery {
    #[serde(default)]
    system: Option<String>,
    #[serde(default)]
    kind: Option<String>,
}

/// `GET /api/plans/{pid}/links?system=&kind=`.
#[debug_handler]
async fn list_links(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    filtered(&ctx, plan.pid, None, &q).await
}

/// `GET /api/plans/{pid}/tasks/{tpid}/links?system=&kind=`.
#[debug_handler]
async fn list_task_links(
    State(ctx): State<AppContext>,
    Path((pid, tpid)): Path<(String, String)>,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let task = super::engineering::find_task(&ctx, plan.pid, &tpid).await?;
    filtered(&ctx, plan.pid, Some(task.pid), &q).await
}

async fn filtered(
    ctx: &AppContext,
    plan: Uuid,
    task: Option<Uuid>,
    q: &ListQuery,
) -> Result<Response> {
    let mut rows = live(ctx, plan, task).await?;
    if let Some(system) = q.system.as_deref() {
        if !rules::SYSTEMS.contains(&system) {
            return Err(unprocessable(&format!(
                "unknown system {system:?} (allowed: {:?})",
                rules::SYSTEMS
            )));
        }
        rows.retain(|l| l.system.as_deref() == Some(system));
    }
    if let Some(kind) = q.kind.as_deref() {
        rows.retain(|l| l.kind == kind);
    }
    format::json(rows)
}

/// `POST /api/plans/{pid}/links/{lpid}/delete` (and `DELETE`).
#[debug_handler]
async fn remove_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, lpid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    remove(&ctx, &caller, plan.pid, None, &lpid).await
}

/// `POST /api/plans/{pid}/tasks/{tpid}/links/{lpid}/delete` (and `DELETE`).
#[debug_handler]
async fn remove_task_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, tpid, lpid)): Path<(String, String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let task = super::engineering::find_task(&ctx, plan.pid, &tpid).await?;
    remove(&ctx, &caller, plan.pid, Some(task.pid), &lpid).await
}

async fn remove(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    task: Option<Uuid>,
    lpid: &str,
) -> Result<Response> {
    let lpid = Uuid::parse_str(lpid).map_err(|_| Error::NotFound)?;
    let row = plan_links::Entity::find()
        .filter(plan_links::Column::Pid.eq(lpid))
        .filter(plan_links::Column::PlanPid.eq(plan))
        .filter(match task {
            Some(t) => plan_links::Column::TaskPid.eq(t),
            None => plan_links::Column::TaskPid.is_null(),
        })
        .filter(plan_links::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut a: plan_links::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(&ctx.db, lpid, "plan_link_removed", caller.actor(), None)
        .await
        .ok();
    format::empty_json()
}

/// The plan-link routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/links", post(add_link))
        .add("/plans/{pid}/links", get(list_links))
        .add("/plans/{pid}/links/{lpid}", delete(remove_link))
        .add("/plans/{pid}/links/{lpid}/delete", post(remove_link))
        .add("/plans/{pid}/tasks/{tpid}/links", post(add_task_link))
        .add("/plans/{pid}/tasks/{tpid}/links", get(list_task_links))
        .add(
            "/plans/{pid}/tasks/{tpid}/links/{lpid}",
            delete(remove_task_link),
        )
        .add(
            "/plans/{pid}/tasks/{tpid}/links/{lpid}/delete",
            post(remove_task_link),
        )
}

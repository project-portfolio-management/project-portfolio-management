//! Typed links on a plan, or on one of its tasks (LT-8). Pure rules in [`crate::plan_links`]. The
//! service stores a link and returns it; it never fetches or renders one.

use super::unprocessable;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::plan_links;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_links as rules;

/// The live links of a plan (`task` is `None`) or of one of its tasks.
async fn live(ctx: &AppContext, plan: Uuid, task: Option<Uuid>) -> Result<Vec<plan_links::Model>> {
    let mut q = sqlx::QueryBuilder::new("SELECT * FROM plan_links WHERE plan_pid = ");
    q.push_bind(plan);
    push_task(&mut q, task);
    q.push(" AND deleted_at IS NULL ORDER BY id");
    Ok(q.build_query_as().fetch_all(crate::db::pool(ctx)).await?)
}

/// The task condition: one task's links, or (`task` is `None`) the plan's own, whose `task_pid` is `NULL`.
fn push_task(q: &mut sqlx::QueryBuilder<sqlx::Postgres>, task: Option<Uuid>) {
    match task {
        Some(t) => {
            q.push(" AND task_pid = ").push_bind(t);
        }
        None => {
            q.push(" AND task_pid IS NULL");
        }
    }
}

/// `POST /api/plans/{pid}/links` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LinkBody {
    /// Kind of link, validated against the allowed kinds in the plan-link rules.
    kind: String,
    /// The linked URL; must be unique among the live links of the same plan or task, otherwise 409.
    url: String,
    /// Human label for the link; stored trimmed.
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

/// Shared create path for plan and task links. Validates the fields, caps links per plan or task, rejects a duplicate URL with 409, then inserts and audits `plan_link_added` in one transaction. `task` is None for a plan-level link.
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
    let mut tx = crate::db::begin(ctx).await?;
    let row: plan_links::Model = sqlx::query_as(
        "INSERT INTO plan_links
             (pid, plan_pid, task_pid, kind, url, label, system, external_id, created_by, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan)
    .bind(task)
    .bind(&b.kind)
    .bind(&b.url)
    .bind(b.label.trim())
    .bind(&b.system)
    .bind(&b.external_id)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "plan_link_added", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/plans/{pid}/links` query: narrow to one system and/or kind.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListQuery {
    /// Restrict to links of this system; an unknown system is a 422.
    #[serde(default)]
    system: Option<String>,
    /// Restrict to links of this kind.
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

/// Shared list path: live links of the plan (or of one task when `task` is set), optionally narrowed by system and kind.
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

/// `GET /api/plans/{pid}/task-link-counts`: how many live links each of the plan's tasks has, so a list of tasks can
/// say which ones link out without asking for every task's links. Tasks with none are absent.
#[debug_handler]
async fn task_link_counts(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let tasks: Vec<Option<Uuid>> = sqlx::query_scalar(
        "SELECT task_pid FROM plan_links
         WHERE plan_pid = $1 AND task_pid IS NOT NULL AND deleted_at IS NULL",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut counts: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
    for t in tasks.into_iter().flatten() {
        *counts.entry(t.to_string()).or_insert(0) += 1;
    }
    format::json(serde_json::json!({ "plan_pid": plan.pid, "counts": counts }))
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

/// Shared delete path: soft-deletes one live link of the plan (or task) and audits `plan_link_removed`; `NotFound` when it does not match.
async fn remove(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    task: Option<Uuid>,
    lpid: &str,
) -> Result<Response> {
    let lpid = super::parse_pid(lpid)?;
    let mut q = sqlx::QueryBuilder::new("SELECT * FROM plan_links WHERE pid = ");
    q.push_bind(lpid).push(" AND plan_pid = ").push_bind(plan);
    push_task(&mut q, task);
    q.push(" AND deleted_at IS NULL LIMIT 1");
    let row: plan_links::Model = q
        .build_query_as()
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(ctx).await?;
    // An update that finds no row is an error, as it was.
    sqlx::query("UPDATE plan_links SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::DateTime::<chrono::FixedOffset>::from(
            chrono::Utc::now(),
        ))
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(tx.conn(), lpid, "plan_link_removed", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
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
        .add("/plans/{pid}/task-link-counts", get(task_link_counts))
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

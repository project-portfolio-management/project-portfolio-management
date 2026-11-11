//! Hammock activities (PJ-7). Rules in [`crate::hammock`]; this is the wiring.
//!
//! * `POST …/tasks/{pid}/hammock` turns a task into a hammock (or back): a derived
//!   span, so it must carry no duration, work, constraint or dependency;
//! * `POST …/hammocks/{pid}/links` and `DELETE …/links/{member}` link it to the
//!   tasks (or hammocks) whose span it follows;
//! * `GET …/hammocks` lists each with the span derived from its links.
//!
//! A hammock is **not in the dependency network**, so it can never drive another
//! task's float. The span is derived on read from the same analysis the critical
//! path uses.

use super::unprocessable;
use std::collections::HashMap;

use crate::prelude::*;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::find_live_task as find_task;
use crate::auth::MaybeAuthUser;
use crate::hammock::{self as rules, Span};
use crate::models::_entities::{hammock_links, tasks};
use crate::models::audit_logs::Model as AuditModel;

/// Request body for turning a task into a hammock or back.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct EnableBody {
    /// True makes the task a hammock; false returns it to an ordinary task. Asking for the state it already has is a no-op.
    enabled: bool,
}

/// `POST /api/plans/{pid}/tasks/{t_pid}/hammock` — make a task a hammock, or an
/// ordinary task again. A hammock is a derived span, so becoming one needs the task
/// to carry no duration, work, constraint or dependency (the answer lists what to
/// clear). Becoming ordinary again removes its links. Audited.
#[debug_handler]
async fn set_hammock(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<EnableBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    if task.is_hammock == b.enabled {
        return format::json(
            serde_json::json!({ "pid": task.pid, "is_hammock": task.is_hammock, "changed": false }),
        );
    }
    if b.enabled {
        let mut blockers = Vec::new();
        if task.duration_days.is_some() {
            blockers.push("duration");
        }
        if task.work_minutes.is_some() {
            blockers.push("work");
        }
        if task.constraint_kind.is_some() {
            blockers.push("constraint");
        }
        let has_dep = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM plan_dependencies WHERE predecessor_pid = $1 OR successor_pid = $1 LIMIT 1",
        )
        .bind(task.pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await?
        .is_some()
            || sqlx::query_scalar::<_, i64>(
                "SELECT id FROM task_relations
                 WHERE deleted_at IS NULL AND kind = $1 AND (from_task_pid = $2 OR to_task_pid = $2)
                 LIMIT 1",
            )
            .bind("precedes")
            .bind(task.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .is_some();
        if has_dep {
            blockers.push("dependencies");
        }
        if !blockers.is_empty() {
            return Err(unprocessable(&format!(
                "a hammock is a derived span, not work: clear its {} first",
                blockers.join(", ")
            )));
        }
    }
    // Links, flag and audit row change together or not at all.
    let mut tx = crate::db::begin(&ctx).await?;
    if !b.enabled {
        sqlx::query("DELETE FROM hammock_links WHERE hammock_pid = $1")
            .bind(task.pid)
            .execute(tx.conn())
            .await?;
    }
    // A hammock's start is derived, so a stored planned start must not linger.
    let row: tasks::Model = if b.enabled {
        sqlx::query_as(
            "UPDATE tasks SET is_hammock = $1, start_date = $2 WHERE id = $3 RETURNING *",
        )
        .bind(b.enabled)
        .bind(None::<chrono::NaiveDate>)
        .bind(task.id)
        .fetch_one(tx.conn())
        .await?
    } else {
        sqlx::query_as("UPDATE tasks SET is_hammock = $1 WHERE id = $2 RETURNING *")
            .bind(b.enabled)
            .bind(task.id)
            .fetch_one(tx.conn())
            .await?
    };
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        if b.enabled {
            "hammock_enabled"
        } else {
            "hammock_disabled"
        },
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan.pid })),
    )
    .await?;
    tx.commit().await?;
    format::json(
        serde_json::json!({ "pid": row.pid, "is_hammock": row.is_hammock, "changed": true }),
    )
}

/// Request body for linking a task or hammock under a hammock.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LinkBody {
    /// Task or hammock of the same plan whose dates the hammock spans.
    member_pid: Uuid,
}

/// `POST /api/plans/{pid}/hammocks/{h_pid}/links` — link a hammock to a task (or
/// another hammock) of the plan. A hammock cannot link to itself or close a cycle.
#[debug_handler]
async fn add_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, h_pid)): Path<(String, String)>,
    Json(b): Json<LinkBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let hammock = find_task(&ctx, plan.pid, &h_pid).await?;
    if !hammock.is_hammock {
        return Err(unprocessable(
            "this task is not a hammock: make it one first",
        ));
    }
    let member = find_task(&ctx, plan.pid, &b.member_pid.to_string())
        .await
        .map_err(|_| unprocessable("a hammock links to a task of the same plan"))?;
    // One link at a time per plan: two simultaneous A→B and B→A cannot both pass the cycle check.
    let mut tx = crate::db::begin(&ctx).await?;
    super::advisory_lock_on(&mut tx, "hammock", plan.pid).await?;
    let all: Vec<(Uuid, Uuid)> =
        sqlx::query_as("SELECT hammock_pid, member_pid FROM hammock_links")
            .fetch_all(tx.conn())
            .await?;
    if all.contains(&(hammock.pid, member.pid)) {
        return Err(unprocessable("this link already exists"));
    }
    if rules::would_cycle(&all, hammock.pid, member.pid) {
        return Err(unprocessable(
            "a hammock cannot link to itself or create a cycle of hammocks",
        ));
    }
    let row: hammock_links::Model = sqlx::query_as(
        "INSERT INTO hammock_links (pid, hammock_pid, member_pid) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(hammock.pid)
    .bind(member.pid)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "hammock_link_added",
        caller.actor(),
        Some(serde_json::json!({ "hammock_pid": hammock.pid, "member_pid": member.pid })),
    )
    .await?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid }))
}

/// `DELETE /api/plans/{pid}/hammocks/{h_pid}/links/{member}` — remove a link; the
/// span is recomputed on the next read.
#[debug_handler]
async fn remove_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, h_pid, member)): Path<(String, String, String)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let hammock = find_task(&ctx, plan.pid, &h_pid).await?;
    let member = super::parse_pid(&member)?;
    let link: hammock_links::Model = sqlx::query_as(
        "SELECT * FROM hammock_links WHERE hammock_pid = $1 AND member_pid = $2 LIMIT 1",
    )
    .bind(hammock.pid)
    .bind(member)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("DELETE FROM hammock_links WHERE id = $1")
        .bind(link.id)
        .execute(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        link.pid,
        "hammock_link_removed",
        caller.actor(),
        Some(serde_json::json!({ "hammock_pid": hammock.pid, "member_pid": member })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// The derived span of every hammock of a plan, from an analysis already made.
pub(crate) async fn spans_for(
    ctx: &AppContext,
    plan: Uuid,
    analysis: Option<&crate::cpm::CpmResult>,
) -> Result<Vec<serde_json::Value>> {
    let rows: Vec<tasks::Model> = sqlx::query_as(
        "SELECT * FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(plan)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let hammocks: Vec<&tasks::Model> = rows.iter().filter(|t| t.is_hammock).collect();
    if hammocks.is_empty() {
        return Ok(Vec::new());
    }
    let pids: Vec<Uuid> = hammocks.iter().map(|h| h.pid).collect();
    let links: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT hammock_pid, member_pid FROM hammock_links WHERE hammock_pid = ANY($1) ORDER BY id",
    )
    .bind(pids.clone())
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let spans: HashMap<Uuid, (NaiveDate, NaiveDate)> = analysis
        .map(|a| {
            a.nodes
                .iter()
                .map(|n| (n.pid, (n.early_start, n.early_finish)))
                .collect()
        })
        .unwrap_or_default();
    let derived = rules::resolve(&pids, &links, &spans);
    let title: HashMap<Uuid, &str> = rows.iter().map(|t| (t.pid, t.title.as_str())).collect();
    Ok(hammocks
        .iter()
        .map(|h| {
            serde_json::json!({
                "pid": h.pid, "title": h.title,
                "span": derived.get(&h.pid).cloned().unwrap_or(Span::Unscheduled { reason: "no_links" }),
                "links": links.iter().filter(|(x, _)| *x == h.pid).map(|(_, m)| serde_json::json!({
                    "pid": m, "title": title.get(m).copied(), "is_hammock": pids.contains(m) })).collect::<Vec<_>>(),
            })
        })
        .collect())
}

/// `GET /api/plans/{pid}/hammocks` — each hammock with the span derived from its links.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let net = super::cpm::load_network(&ctx, plan.pid).await?;
    let analysis = net.analyze(0).ok();
    let hammocks = spans_for(&ctx, plan.pid, analysis.as_ref()).await?;
    format::json(serde_json::json!({
        "hammocks": hammocks,
        "note": "A hammock's start is the earliest start, and its finish the latest finish, of the tasks it links to. It is not work and is not in the dependency network, so it never affects another task's float. A hammock with no links, or whose links are all unscheduled, has no span.",
    }))
}

/// The hammock routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks/{t_pid}/hammock", post(set_hammock))
        .add("/plans/{pid}/hammocks", get(list))
        .add("/plans/{pid}/hammocks/{h_pid}/links", post(add_link))
        .add(
            "/plans/{pid}/hammocks/{h_pid}/links/{member}",
            delete(remove_link),
        )
}

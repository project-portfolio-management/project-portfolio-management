//! Task relations (LT-10): create, list and remove typed relations between
//! tasks, and the three behaviours that make them more than labels —
//! `blocks` stops the blocked task closing, `precedes` constrains due dates,
//! `duplicates` closes the duplicate when the primary closes. Pure rules in
//! [`crate::task_relations`].
//!
//! Every refusal names the relation that caused it. A relation to a task in
//! a plan the caller cannot read is **hidden** from them (counted, never
//! shown), and deleting nothing here ever removes a task.

use super::conflict;
use super::unprocessable;
use crate::prelude::*;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::task_relations as rules;

/// Load a task that is not soft-deleted by pid, or `NotFound`.
async fn live_task(ctx: &AppContext, pid: Uuid) -> Result<tasks::Model> {
    live_task_or_none(ctx, pid).await?.ok_or(Error::NotFound)
}

/// A task that is not soft-deleted, by pid.
async fn live_task_or_none(ctx: &AppContext, pid: Uuid) -> Result<Option<tasks::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM tasks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// All live relations, optionally of one kind, in creation order.
async fn live_relations_of(
    ctx: &AppContext,
    kind: Option<&str>,
) -> Result<Vec<task_relations::Model>> {
    Ok(match kind {
        Some(k) => {
            sqlx::query_as(
                "SELECT * FROM task_relations WHERE deleted_at IS NULL AND kind = $1 ORDER BY id",
            )
            .bind(k)
            .fetch_all(crate::db::pool(ctx))
            .await?
        }
        None => {
            sqlx::query_as("SELECT * FROM task_relations WHERE deleted_at IS NULL ORDER BY id")
                .fetch_all(crate::db::pool(ctx))
                .await?
        }
    })
}

/// Live relations of `kind` pointing at `to`, unordered (as the `SeaORM` read was).
async fn live_relations_to(
    ctx: &AppContext,
    to: Uuid,
    kind: &str,
) -> std::result::Result<Vec<task_relations::Model>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM task_relations WHERE to_task_pid = $1 AND kind = $2 AND deleted_at IS NULL",
    )
    .bind(to)
    .bind(kind)
    .fetch_all(crate::db::pool(ctx))
    .await
}

/// Open tasks that block `task_pid` (a live `blocks` relation pointing at it
/// from a task that is not finished): `(blocker pid, title)`.
pub(crate) async fn open_blockers(ctx: &AppContext, task_pid: Uuid) -> Result<Vec<(Uuid, String)>> {
    let rels = live_relations_to(ctx, task_pid, "blocks").await?;
    let mut out = Vec::new();
    for r in rels {
        let blocker: Option<(Uuid, String)> = sqlx::query_as(
            "SELECT pid, title FROM tasks
             WHERE pid = $1 AND deleted_at IS NULL AND done_at IS NULL
             LIMIT 1",
        )
        .bind(r.from_task_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        if let Some(b) = blocker {
            out.push(b);
        }
    }
    Ok(out)
}

/// `blocks`: refuse finishing `task` while a blocker is open.
///
/// A blocker in a plan hidden from the caller still blocks, but is not named.
pub(crate) async fn ensure_not_blocked(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    task_pid: Uuid,
) -> Result<()> {
    let blockers = open_blockers(ctx, task_pid).await?;
    if blockers.is_empty() {
        return Ok(());
    }
    let scope = super::plan_access::HiddenScope::of(ctx, caller).await?;
    let names: Vec<String> = blockers
        .iter()
        .map(|(pid, title)| {
            if scope.node(pid) {
                "a task you cannot see".to_string()
            } else {
                format!("{title} ({pid})")
            }
        })
        .collect();
    Err(unprocessable(&format!(
        "blocked by an open task via a `blocks` relation: {}",
        names.join(", ")
    )))
}

/// `precedes`: refuse a new due date that breaks an existing relation.
pub(crate) async fn check_due_change(
    ctx: &AppContext,
    task_pid: Uuid,
    new_due: Option<NaiveDate>,
) -> Result<()> {
    for r in live_relations_of(ctx, Some("precedes")).await? {
        let (pred, succ) = if r.from_task_pid == task_pid {
            (Some(new_due), None)
        } else if r.to_task_pid == task_pid {
            (None, Some(new_due))
        } else {
            continue;
        };
        let other_pid = if r.from_task_pid == task_pid {
            r.to_task_pid
        } else {
            r.from_task_pid
        };
        let Some(other) = live_task_or_none(ctx, other_pid).await? else {
            continue;
        };
        let (pred_due, succ_due) = match (pred, succ) {
            (Some(p), _) => (p, other.due_date),
            (_, Some(s)) => (other.due_date, s),
            _ => continue,
        };
        if rules::precedes_violated(pred_due, succ_due) {
            return Err(unprocessable(&format!(
                "the `precedes` relation {} -> {} requires the successor to be due after the predecessor",
                r.from_task_pid, r.to_task_pid
            )));
        }
    }
    Ok(())
}

/// `duplicates`: when `primary` is finished, close every open task that is a
/// duplicate of it, writing a transition row for each. Best-effort per
/// duplicate: one failure never undoes the primary's move.
pub(crate) async fn close_duplicates(ctx: &AppContext, primary: Uuid, actor: Option<&str>) {
    let Ok(rels) = live_relations_to(ctx, primary, "duplicates").await else {
        return;
    };
    for r in rels {
        close_one(ctx, r.from_task_pid, primary, actor).await.ok();
    }
}

/// Close one duplicate task as done in its plan's workflow, with a transition row and an audit entry in a single transaction. Does nothing when it is already done; fails if the workflow has no done state.
async fn close_one(ctx: &AppContext, dup: Uuid, primary: Uuid, actor: Option<&str>) -> Result<()> {
    let task = live_task(ctx, dup).await?;
    if task.done_at.is_some() {
        return Ok(());
    }
    let workflow = super::workflow::in_force(ctx, task.plan_pid, "task").await?;
    let Some(done) = workflow
        .states
        .iter()
        .find(|s| crate::workflow::is_done(&workflow, &s.key))
    else {
        return Ok(());
    };
    let now = chrono::Utc::now();
    let (from, assignee, plan_pid) = (
        task.status.clone(),
        task.assignee_ref.clone(),
        task.plan_pid,
    );
    let at = chrono::DateTime::<chrono::FixedOffset>::from(now);
    let mut tx = crate::db::begin(ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    sqlx::query_scalar::<_, i32>(
        "UPDATE tasks SET status = $1, status_changed_at = $2, done_at = $3 WHERE id = $4 RETURNING id",
    )
    .bind(&done.key)
    .bind(at)
    .bind(Some(at))
    .bind(task.id)
    .fetch_one(tx.conn())
    .await?;
    // The move in the transition log, as `crate::tba::insert_transition_on` writes it.
    sqlx::query(
        "INSERT INTO task_transitions
             (pid, task_pid, plan_pid, from_status, to_status, at, actor_ref, assignee_ref, backfilled)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, false)",
    )
    .bind(Uuid::new_v4())
    .bind(dup)
    .bind(plan_pid)
    .bind(Some(from))
    .bind(&done.key)
    .bind(at)
    .bind(actor)
    .bind(assignee)
    .execute(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        dup,
        "task_closed_as_duplicate",
        actor,
        Some(serde_json::json!({ "primary": primary })),
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

/// `POST …/relations` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RelationPayload {
    /// Relation kind, validated against the allowed kinds when the request is handled.
    kind: String,
    /// The task the relation points at; must be a live task.
    to_task_pid: Uuid,
}

/// `POST /api/plans/{pid}/tasks/{t_pid}/relations`.
#[debug_handler]
async fn create_relation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(p): Json<RelationPayload>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let from = super::parse_pid(&t_pid)?;
    let from_task = live_task(&ctx, from).await?;
    if from_task.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let problems = rules::problems(&p.kind, from, p.to_task_pid);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let to_task = live_task(&ctx, p.to_task_pid)
        .await
        .map_err(|_| unprocessable("to_task_pid names no live task"))?;
    // A relation acts on the other task, so it must be one the caller can read: the same answer as no such task.
    super::plan_access::require_readable(&ctx, &caller, to_task.plan_pid)
        .await
        .map_err(|_| unprocessable("to_task_pid names no live task"))?;
    let same_kind = live_relations_of(&ctx, Some(&p.kind)).await?;
    let duplicate = same_kind.iter().any(|r| {
        (r.from_task_pid == from && r.to_task_pid == p.to_task_pid)
            || (rules::is_symmetric(&p.kind)
                && r.from_task_pid == p.to_task_pid
                && r.to_task_pid == from)
    });
    if duplicate {
        return Err(conflict("this relation already exists"));
    }
    if rules::is_directed_acyclic(&p.kind) {
        let edges: Vec<(Uuid, Uuid)> = same_kind
            .iter()
            .map(|r| (r.from_task_pid, r.to_task_pid))
            .collect();
        if rules::creates_cycle(&edges, from, p.to_task_pid) {
            return Err(unprocessable(&format!(
                "a `{}` relation here would form a cycle",
                p.kind
            )));
        }
    }
    if p.kind == "precedes" && rules::precedes_violated(from_task.due_date, to_task.due_date) {
        return Err(unprocessable(
            "`precedes` requires the successor to be due after the predecessor",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: task_relations::Model = sqlx::query_as(
        "INSERT INTO task_relations (pid, from_task_pid, to_task_pid, kind, deleted_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(from)
    .bind(p.to_task_pid)
    .bind(&p.kind)
    .bind(None::<chrono::DateTime<chrono::FixedOffset>>)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "task_relation_added",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/plans/{pid}/tasks/{t_pid}/relations` — relations in both
/// directions. Those whose other task sits in a plan the caller cannot read
/// are omitted and counted in `hidden`.
#[debug_handler]
async fn list_relations(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let task = live_task(&ctx, super::parse_pid(&t_pid)?).await?;
    if task.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let rels: Vec<task_relations::Model> = sqlx::query_as(
        "SELECT * FROM task_relations
         WHERE deleted_at IS NULL AND (from_task_pid = $1 OR to_task_pid = $1)
         ORDER BY id",
    )
    .bind(task.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut shown = Vec::new();
    let mut hidden = 0usize;
    for r in rels {
        let other_pid = if r.from_task_pid == task.pid {
            r.to_task_pid
        } else {
            r.from_task_pid
        };
        let Some(other) = live_task_or_none(&ctx, other_pid).await? else {
            continue; // the other task is gone: so is the relation's meaning
        };
        let stage: Option<String> = sqlx::query_scalar::<_, Option<String>>(
            "SELECT stage FROM plans WHERE pid = $1 LIMIT 1",
        )
        .bind(other.plan_pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await?
        .flatten();
        let private_hidden = super::plan_access::hidden_plans(&ctx, &[other.plan_pid], &caller)
            .await
            .contains(&other.plan_pid);
        let readable = !private_hidden
            && crate::auth::authorize_record(
                &caller,
                authentication_verifier::Action::Read,
                &crate::auth::plan_resource_attrs(stage.as_deref()),
            )
            .is_ok();
        if !readable {
            hidden += 1;
            continue;
        }
        shown.push(serde_json::json!({
            "pid": r.pid,
            "kind": r.kind,
            "direction": if r.from_task_pid == task.pid { "outgoing" } else { "incoming" },
            "other_task": { "pid": other.pid, "title": other.title, "plan_pid": other.plan_pid,
                            "status": other.status, "due_date": other.due_date },
        }));
    }
    format::json(serde_json::json!({ "relations": shown, "hidden": hidden }))
}

/// `DELETE /api/plans/{pid}/tasks/{t_pid}/relations/{r_pid}`.
#[debug_handler]
async fn delete_relation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid, r_pid)): Path<(String, String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let task = live_task(&ctx, super::parse_pid(&t_pid)?).await?;
    if task.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let r_pid = super::parse_pid(&r_pid)?;
    let row: task_relations::Model = sqlx::query_as(
        "SELECT * FROM task_relations
         WHERE pid = $1 AND deleted_at IS NULL AND (from_task_pid = $2 OR to_task_pid = $2)
         LIMIT 1",
    )
    .bind(r_pid)
    .bind(task.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let deleted_at = chrono::DateTime::<chrono::FixedOffset>::from(chrono::Utc::now());
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE task_relations SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(Some(deleted_at))
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        r_pid,
        "task_relation_removed",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// The task-relation routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add(
            "/plans/{pid}/tasks/{t_pid}/relations",
            post(create_relation),
        )
        .add("/plans/{pid}/tasks/{t_pid}/relations", get(list_relations))
        .add(
            "/plans/{pid}/tasks/{t_pid}/relations/{r_pid}",
            delete(delete_relation),
        )
}

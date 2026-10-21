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
use super::{db_err, unprocessable};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{plans, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::task_relations as rules;

async fn live_task(ctx: &AppContext, pid: Uuid) -> Result<tasks::Model> {
    tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

async fn live_relations_of(
    ctx: &AppContext,
    kind: Option<&str>,
) -> Result<Vec<task_relations::Model>> {
    let mut q = task_relations::Entity::find().filter(task_relations::Column::DeletedAt.is_null());
    if let Some(k) = kind {
        q = q.filter(task_relations::Column::Kind.eq(k));
    }
    q.order_by_asc(task_relations::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// Open tasks that block `task_pid` (a live `blocks` relation pointing at it
/// from a task that is not finished): `(blocker pid, title)`.
pub(crate) async fn open_blockers(ctx: &AppContext, task_pid: Uuid) -> Result<Vec<(Uuid, String)>> {
    let rels = task_relations::Entity::find()
        .filter(task_relations::Column::ToTaskPid.eq(task_pid))
        .filter(task_relations::Column::Kind.eq("blocks"))
        .filter(task_relations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = Vec::new();
    for r in rels {
        let blocker = tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(r.from_task_pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::DoneAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?;
        if let Some(b) = blocker {
            out.push((b.pid, b.title));
        }
    }
    Ok(out)
}

/// `blocks`: refuse finishing `task` while a blocker is open.
pub(crate) async fn ensure_not_blocked(ctx: &AppContext, task_pid: Uuid) -> Result<()> {
    let blockers = open_blockers(ctx, task_pid).await?;
    if blockers.is_empty() {
        return Ok(());
    }
    let names: Vec<String> = blockers
        .iter()
        .map(|(pid, title)| format!("{title} ({pid})"))
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
        let Some(other) = tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(other_pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
        else {
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
    let Ok(rels) = task_relations::Entity::find()
        .filter(task_relations::Column::ToTaskPid.eq(primary))
        .filter(task_relations::Column::Kind.eq("duplicates"))
        .filter(task_relations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
    else {
        return;
    };
    for r in rels {
        close_one(ctx, r.from_task_pid, primary, actor).await.ok();
    }
}

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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let mut active: tasks::ActiveModel = task.into();
    active.status = ActiveValue::set(done.key.clone());
    active.status_changed_at = ActiveValue::set(now.into());
    active.done_at = ActiveValue::set(Some(now.into()));
    active.update(&txn).await.map_err(db_err)?;
    crate::tba::transition_row(
        dup,
        plan_pid,
        Some(from),
        done.key.clone(),
        now,
        actor.map(ToString::to_string),
        assignee,
    )
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        dup,
        "task_closed_as_duplicate",
        actor,
        Some(serde_json::json!({ "primary": primary })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    Ok(())
}

/// `POST …/relations` body.
#[derive(Debug, Deserialize)]
struct RelationPayload {
    kind: String,
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
    let from = Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?;
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
    let row = task_relations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        from_task_pid: ActiveValue::set(from),
        to_task_pid: ActiveValue::set(p.to_task_pid),
        kind: ActiveValue::set(p.kind.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "task_relation_added",
        caller.actor(),
        None,
    )
    .await;
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
    let task = live_task(&ctx, Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?).await?;
    if task.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let rels = task_relations::Entity::find()
        .filter(task_relations::Column::DeletedAt.is_null())
        .filter(
            sea_orm::Condition::any()
                .add(task_relations::Column::FromTaskPid.eq(task.pid))
                .add(task_relations::Column::ToTaskPid.eq(task.pid)),
        )
        .order_by_asc(task_relations::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut shown = Vec::new();
    let mut hidden = 0usize;
    for r in rels {
        let other_pid = if r.from_task_pid == task.pid {
            r.to_task_pid
        } else {
            r.from_task_pid
        };
        let Some(other) = tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(other_pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
        else {
            continue; // the other task is gone: so is the relation's meaning
        };
        let stage = plans::Entity::find()
            .filter(plans::Column::Pid.eq(other.plan_pid))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .and_then(|p| p.stage);
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
    let task = live_task(&ctx, Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?).await?;
    if task.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let r_pid = Uuid::parse_str(&r_pid).map_err(|_| Error::NotFound)?;
    let row = task_relations::Entity::find()
        .filter(task_relations::Column::Pid.eq(r_pid))
        .filter(task_relations::Column::DeletedAt.is_null())
        .filter(
            sea_orm::Condition::any()
                .add(task_relations::Column::FromTaskPid.eq(task.pid))
                .add(task_relations::Column::ToTaskPid.eq(task.pid)),
        )
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut active: task_relations::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        r_pid,
        "task_relation_removed",
        caller.actor(),
        None,
    )
    .await;
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

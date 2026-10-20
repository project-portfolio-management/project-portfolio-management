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

use super::{db_err, unprocessable};
use std::collections::HashMap;

use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::ConnectionTrait;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::hammock::{self as rules, Span};
use crate::models::_entities::{hammock_links, plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;

async fn find_task(ctx: &AppContext, plan: Uuid, raw: &str) -> Result<tasks::Model> {
    let pid = Uuid::parse_str(raw).map_err(|_| Error::NotFound)?;
    tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(pid))
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

#[derive(Debug, Deserialize)]
struct EnableBody {
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
        let has_dep = plan_dependencies::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(plan_dependencies::Column::PredecessorPid.eq(task.pid))
                    .add(plan_dependencies::Column::SuccessorPid.eq(task.pid)),
            )
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .is_some()
            || task_relations::Entity::find()
                .filter(task_relations::Column::DeletedAt.is_null())
                .filter(task_relations::Column::Kind.eq("precedes"))
                .filter(
                    sea_orm::Condition::any()
                        .add(task_relations::Column::FromTaskPid.eq(task.pid))
                        .add(task_relations::Column::ToTaskPid.eq(task.pid)),
                )
                .one(&ctx.db)
                .await
                .map_err(db_err)?
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    if !b.enabled {
        hammock_links::Entity::delete_many()
            .filter(hammock_links::Column::HammockPid.eq(task.pid))
            .exec(&txn)
            .await
            .map_err(db_err)?;
    }
    let mut a: tasks::ActiveModel = task.clone().into();
    a.is_hammock = ActiveValue::set(b.enabled);
    // A hammock's start is derived, so a stored planned start must not linger.
    if b.enabled {
        a.start_date = ActiveValue::set(None);
    }
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        if b.enabled {
            "hammock_enabled"
        } else {
            "hammock_disabled"
        },
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan.pid })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    format::json(
        serde_json::json!({ "pid": row.pid, "is_hammock": row.is_hammock, "changed": true }),
    )
}

#[derive(Debug, Deserialize)]
struct LinkBody {
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    txn.execute_unprepared(&format!(
        "SELECT pg_advisory_xact_lock(hashtext('hammock:{}'))",
        plan.pid
    ))
    .await
    .map_err(db_err)?;
    let all: Vec<(Uuid, Uuid)> = hammock_links::Entity::find()
        .all(&txn)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|l| (l.hammock_pid, l.member_pid))
        .collect();
    if all.contains(&(hammock.pid, member.pid)) {
        return Err(unprocessable("this link already exists"));
    }
    if rules::would_cycle(&all, hammock.pid, member.pid) {
        return Err(unprocessable(
            "a hammock cannot link to itself or create a cycle of hammocks",
        ));
    }
    let row = hammock_links::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        hammock_pid: ActiveValue::set(hammock.pid),
        member_pid: ActiveValue::set(member.pid),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "hammock_link_added",
        caller.actor(),
        Some(serde_json::json!({ "hammock_pid": hammock.pid, "member_pid": member.pid })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
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
    let member = Uuid::parse_str(&member).map_err(|_| Error::NotFound)?;
    let link = hammock_links::Entity::find()
        .filter(hammock_links::Column::HammockPid.eq(hammock.pid))
        .filter(hammock_links::Column::MemberPid.eq(member))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    hammock_links::Entity::delete_by_id(link.id)
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        link.pid,
        "hammock_link_removed",
        caller.actor(),
        Some(serde_json::json!({ "hammock_pid": hammock.pid, "member_pid": member })),
    )
    .await
    .ok();
    format::empty_json()
}

/// The derived span of every hammock of a plan, from an analysis already made.
pub(crate) async fn spans_for(
    ctx: &AppContext,
    plan: Uuid,
    analysis: Option<&crate::cpm::CpmResult>,
) -> Result<Vec<serde_json::Value>> {
    let rows: Vec<tasks::Model> = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .order_by_asc(tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let hammocks: Vec<&tasks::Model> = rows.iter().filter(|t| t.is_hammock).collect();
    if hammocks.is_empty() {
        return Ok(Vec::new());
    }
    let pids: Vec<Uuid> = hammocks.iter().map(|h| h.pid).collect();
    let links: Vec<(Uuid, Uuid)> = hammock_links::Entity::find()
        .filter(hammock_links::Column::HammockPid.is_in(pids.clone()))
        .order_by_asc(hammock_links::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|l| (l.hammock_pid, l.member_pid))
        .collect();
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

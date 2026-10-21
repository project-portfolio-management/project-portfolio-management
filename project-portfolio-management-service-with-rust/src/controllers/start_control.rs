//! Start control (EX-8). Rules in [`crate::start_control`].
//!
//! Moving a programme into delivery (its phase to `executing`) is checked against a limit on programmes in
//! active delivery and against the constraint pool (the drum, CC-5): if the places are full, or that pool is
//! short of supply in the months ahead, the move is refused (422 `start_refused`), the programme joins the
//! queue, and it starts only when asked again with room, or with an override that gives a reason. The check
//! is also readable without moving anything: `GET /api/plans/{pid}/start-check`.
//!
//! The limit and the pool are the deployment's: `PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT`, and the drum.

use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{Datelike, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    allocations, plans, resource_roles, skill_pool_members as members, skill_pools,
    start_overrides as overrides, start_queue_entries as queue,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::start_control::{self as rules, Refusal};

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/start-check", get(start_check))
        .add("/plans/{pid}/start-queue/withdraw", post(withdraw))
        .add("/start-queue", get(list_queue))
}

/// What the check found for a programme.
pub(crate) struct Evaluation {
    pub in_delivery: Vec<(Uuid, String)>,
    pub limit: Option<usize>,
    pub limit_unusable: bool,
    pub refusals: Vec<Refusal>,
    pub pool: Option<(Uuid, String)>,
    pub demand: Value,
}

/// Evaluate a start of `plan` now.
#[allow(clippy::too_many_lines)]
pub(crate) async fn evaluate(ctx: &AppContext, plan: &plans::Model) -> Result<Evaluation> {
    let today = Utc::now().date_naive();
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let statuses = super::programme_gates::status_map(ctx, today).await?;
    let in_delivery: Vec<(Uuid, String)> = all
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && p.pid != plan.pid)
        .filter(|p| {
            rules::in_delivery(
                p.phase.as_deref(),
                statuses.get(&p.pid).copied().unwrap_or("active"),
            )
        })
        .map(|p| (p.pid, p.name.clone()))
        .collect();
    let (limit, limit_unusable) = rules::limit();

    // The constraint pool and its short months, from the capacity heatmap over every plan.
    let everyone: HashSet<Uuid> = all.iter().map(|p| p.pid).collect();
    let first = today.with_day(1).unwrap_or(today);
    let heat = super::capacity_heatmap::heatmap(ctx, &everyone, first, 6).await?;
    let pool = super::drum::constraint_pool(ctx, &heat).await?;
    let short: Vec<String> = pool
        .as_ref()
        .and_then(|(pid, _)| {
            heat["pools"].as_array()?.iter().find(|p| {
                p["pool_pid"].as_str().and_then(|s| Uuid::parse_str(s).ok()) == Some(*pid)
            })
        })
        .into_iter()
        .flat_map(|p| p["cells"].as_array().cloned().unwrap_or_default())
        .filter(|c| c["gap_fte"].as_f64().is_some_and(|g| g < 0.0))
        .filter_map(|c| c["month"].as_str().map(str::to_string))
        .collect();
    let refusals = rules::refusals(
        in_delivery.len(),
        limit,
        pool.as_ref().map(|(_, n)| (n.as_str(), short.as_slice())),
    );

    // This programme's demand per skill pool and month (firm allocations on it and the plans under it).
    let mut tree = vec![plan.pid];
    let mut frontier = vec![plan.pid];
    for _ in 0..10 {
        let kids: Vec<Uuid> = all
            .iter()
            .filter(|p| {
                p.parent_pid.is_some_and(|x| frontier.contains(&x)) && !tree.contains(&p.pid)
            })
            .map(|p| p.pid)
            .collect();
        if kids.is_empty() {
            break;
        }
        tree.extend(kids.iter().copied());
        frontier = kids;
    }
    let pool_names: HashMap<Uuid, String> = skill_pools::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let pool_of_role: HashMap<String, Uuid> = members::Entity::find()
        .filter(members::Column::Kind.eq("role"))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|m| (m.value.to_lowercase(), m.pool_pid))
        .collect();
    let role_of: HashMap<String, String> = resource_roles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|r| (r.person_ref, r.role.to_lowercase()))
        .collect();
    let mut milli: BTreeMap<(String, String), i64> = BTreeMap::new();
    for a in allocations::Entity::find()
        .filter(allocations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    {
        if !tree.contains(&a.plan_pid) || a.booking == "proposed" {
            continue;
        }
        let role = a
            .role
            .clone()
            .filter(|r| !r.trim().is_empty())
            .map(|r| r.to_lowercase())
            .or_else(|| role_of.get(&a.person_ref).cloned());
        let Some(pool_pid) = role.and_then(|r| pool_of_role.get(&r).copied()) else {
            continue;
        };
        for m in 0..6u32 {
            let ms = crate::capacity_heatmap::add_months(first, m);
            let me = crate::capacity_heatmap::add_months(first, m + 1)
                .pred_opt()
                .unwrap_or(ms);
            let (s, e) = (
                a.start_date.unwrap_or(ms).max(ms),
                a.end_date.unwrap_or(me).min(me),
            );
            if s > e {
                continue;
            }
            let days = (e - s).num_days() + 1;
            let month_days = (me - ms).num_days() + 1;
            *milli
                .entry((
                    pool_names.get(&pool_pid).cloned().unwrap_or_default(),
                    ms.format("%Y-%m").to_string(),
                ))
                .or_default() += i64::from(a.percent) * 10 * days / month_days;
        }
    }
    let mut by_pool: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for ((pool_name, month), v) in milli {
        #[allow(clippy::cast_precision_loss)]
        by_pool
            .entry(pool_name)
            .or_default()
            .push(json!({ "month": month, "demand_fte": v as f64 / 1000.0 }));
    }
    let demand = json!(
        by_pool
            .into_iter()
            .map(|(p, months)| json!({ "pool": p, "months": months }))
            .collect::<Vec<_>>()
    );
    Ok(Evaluation {
        in_delivery,
        limit,
        limit_unusable,
        refusals,
        pool,
        demand,
    })
}

async fn waiting(ctx: &AppContext) -> Result<Vec<queue::Model>> {
    queue::Entity::find()
        .filter(queue::Column::Status.eq("waiting"))
        .order_by_asc(queue::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))
}

/// `GET /api/plans/{pid}/start-check` — what a start would meet now. Changes nothing.
#[debug_handler]
async fn start_check(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let e = evaluate(&ctx, &plan).await?;
    let q = waiting(&ctx).await?;
    let pids: Vec<Uuid> = q.iter().map(|x| x.plan_pid).collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &pids).await;
    format::json(json!({
        "plan_pid": plan.pid,
        "wip": { "limit": e.limit, "limit_setting_unusable": e.limit_unusable, "in_delivery": e.in_delivery.len() },
        "constraint_pool": e.pool.as_ref().map(|(_, n)| n),
        "demand_by_pool": e.demand,
        "would_be_refused": !e.refusals.is_empty(),
        "refusals": e.refusals,
        "queue_position": q.iter().filter(|x| readable.contains(&x.plan_pid)).position(|x| x.plan_pid == plan.pid).map(|i| i + 1),
        "note": "A programme starts when its phase moves to executing. It is refused when the limit on programmes in active delivery (paused and stopped ones do not count) is full, or when the constraint pool (the drum) is short of supply in a month of the next six; then it joins the queue, and starts later when there is room or with an override that gives a reason. Demand is this programme's firm allocations by skill pool and month, in FTE.",
    }))
}

/// The gate for a move into delivery. Allows it, or refuses with 422 `start_refused`, queueing the programme.
///
/// # Errors
///
/// 422 when refused and no override reason was given; a database error otherwise.
#[allow(clippy::too_many_lines)]
pub(crate) async fn gate(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: &plans::Model,
    override_reason: Option<&str>,
) -> Result<()> {
    if plan.kind.as_deref() != Some("Program") {
        return Ok(());
    }
    let today = Utc::now().date_naive();
    let status = super::programme_gates::status_map(ctx, today)
        .await?
        .get(&plan.pid)
        .copied()
        .unwrap_or("active");
    if rules::in_delivery(plan.phase.as_deref(), status) {
        return Ok(());
    }
    let e = evaluate(ctx, plan).await?;
    let entry = queue::Entity::find()
        .filter(queue::Column::PlanPid.eq(plan.pid))
        .filter(queue::Column::Status.eq("waiting"))
        .one(&ctx.db)
        .await
        .map_err(|x| db_err(&x))?;
    let started = |entry: Option<queue::Model>| async move {
        if let Some(q) = entry {
            let mut a: queue::ActiveModel = q.into();
            a.status = ActiveValue::set("started".to_string());
            a.resolved_on = ActiveValue::set(Some(today));
            a.update(&ctx.db).await.map_err(|x| db_err(&x))?;
        }
        Ok::<(), Error>(())
    };
    if e.refusals.is_empty() {
        return started(entry).await;
    }
    let text = serde_json::to_string(&e.refusals).unwrap_or_default();
    if let Some(r) = override_reason
        .map(str::trim)
        .filter(|r| r.chars().count() >= 10)
    {
        overrides::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            reason: ActiveValue::set(r.to_string()),
            refusals: ActiveValue::set(text),
            decided_by: ActiveValue::set(caller.actor().map(str::to_string)),
            decided_on: ActiveValue::set(today),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|x| db_err(&x))?;
        AuditModel::record_or_log(
            &ctx.db,
            plan.pid,
            "start_override",
            caller.actor(),
            Some(json!({ "refusals": e.refusals })),
        )
        .await;
        return started(entry).await;
    }
    let position = if let Some(q) = entry {
        let mut a: queue::ActiveModel = q.clone().into();
        a.refusals = ActiveValue::set(text);
        a.update(&ctx.db).await.map_err(|x| db_err(&x))?;
        q.id
    } else {
        queue::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            status: ActiveValue::set("waiting".to_string()),
            refusals: ActiveValue::set(text),
            queued_on: ActiveValue::set(today),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|x| db_err(&x))?
        .id
    };
    // The place among the programmes the caller may read: a private plan is not counted for anyone else.
    let waiting_now = waiting(ctx).await?;
    let pids: Vec<Uuid> = waiting_now.iter().map(|x| x.plan_pid).collect();
    let readable = super::plan_access::readable_plans(ctx, caller, &pids).await;
    let place = waiting_now
        .iter()
        .filter(|x| readable.contains(&x.plan_pid))
        .position(|x| x.id == position)
        .map_or(1, |i| i + 1);
    let why = e
        .refusals
        .iter()
        .map(|r| match r {
            Refusal::LimitReached { in_delivery, limit } => format!(
                "{in_delivery} of {limit} places for programmes in active delivery are taken"
            ),
            Refusal::ConstraintPoolOver { pool, months } => format!(
                "the {pool} pool is short of supply in {}",
                months.join(", ")
            ),
        })
        .collect::<Vec<_>>()
        .join("; ");
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "start_refused",
        caller.actor(),
        Some(json!({ "refusals": e.refusals, "queue_position": place })),
    )
    .await;
    Err(Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new(
            "start_refused",
            format!(
                "this programme cannot start yet: {why}. It is number {place} in the queue; send start_override_reason (10 or more characters) to start anyway."
            ),
        ),
    ))
}

/// `POST /api/plans/{pid}/start-queue/withdraw` — leave the queue.
#[debug_handler]
async fn withdraw(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let q = queue::Entity::find()
        .filter(queue::Column::PlanPid.eq(plan.pid))
        .filter(queue::Column::Status.eq("waiting"))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let mut a: queue::ActiveModel = q.into();
    a.status = ActiveValue::set("withdrawn".to_string());
    a.resolved_on = ActiveValue::set(Some(Utc::now().date_naive()));
    a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "start_queue_withdrawn",
        caller.actor(),
        None,
    )
    .await;
    format::json(json!({ "withdrawn": plan.pid }))
}

/// `GET /api/start-queue` — programmes waiting to start that you can read, in the order they asked.
#[debug_handler]
async fn list_queue(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let q = waiting(&ctx).await?;
    let pids: Vec<Uuid> = q.iter().map(|x| x.plan_pid).collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &pids).await;
    let names: HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    // A private plan is nowhere: the places count only the programmes you may read.
    let list: Vec<Value> = q
        .iter()
        .filter(|x| readable.contains(&x.plan_pid))
        .enumerate()
        .map(|(i, x)| json!({
            "position": i + 1, "plan_pid": x.plan_pid, "name": names.get(&x.plan_pid),
            "queued_on": x.queued_on, "refusals": serde_json::from_str::<Value>(&x.refusals).unwrap_or_default(),
        }))
        .collect();
    format::json(
        json!({ "queue": list, "note": "In the order the programmes asked to start; the order never changes. A place counts only the programmes you may read." }),
    )
}

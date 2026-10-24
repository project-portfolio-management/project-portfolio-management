//! Deployment-date change control (EX-5). Rules in [`crate::deployment_moves`].
//!
//! * `POST /api/plans/{pid}/deployment-moves`: ask to move the go-live task of a deployment plan. Allowed
//!   only where the plan belongs to a programme delivered as a central platform with local deployment (EX-2).
//!   The knock-on is worked out **now**, by running the schedule network (across linked plans, as the caller
//!   may read them) with the moved date and comparing it with the network as it stands; the labour cost of
//!   the knock-on is recorded against the requester. **The schedule does not change.**
//! * `POST …/{move}/decision`: the named approver approves (the go-live date is then set) or refuses, with a
//!   reason. Both are audited.
//! * `GET /api/plans/{pid}/deployment-moves`: the requests, and the knock-on cost by requester.

use std::collections::{BTreeMap, HashMap};

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::db_err;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cpm::{self, Options};
use crate::deployment_moves as rules;
use crate::models::_entities::{deployment_moves as moves, plans, programme_profiles, tasks};
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/deployment-moves", post(request_move))
        .add("/plans/{pid}/deployment-moves", get(list))
        .add(
            "/plans/{pid}/deployment-moves/{mpid}/decision",
            post(decide),
        )
}

#[derive(Debug, Deserialize)]
struct NewMove {
    task_pid: Uuid,
    new_start: NaiveDate,
    reason: String,
    /// `person:<id>` or `role:<name>`; defaults to `PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_APPROVER`.
    #[serde(default)]
    approver_ref: Option<String>,
}

/// The programme a plan belongs to: itself or its nearest ancestor of kind Program.
async fn programme_of(ctx: &AppContext, plan: &plans::Model) -> Result<Option<plans::Model>> {
    let mut cur = plan.clone();
    for _ in 0..64 {
        if cur.kind.as_deref() == Some("Program") {
            return Ok(Some(cur));
        }
        let Some(parent) = cur.parent_pid else {
            return Ok(None);
        };
        let Some(p) = plans::Entity::find()
            .filter(plans::Column::Pid.eq(parent))
            .filter(plans::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
        else {
            return Ok(None);
        };
        cur = p;
    }
    Ok(None)
}

fn roles_of(caller: &MaybeAuthUser) -> Vec<String> {
    caller
        .claims()
        .and_then(|c| c.attrs.get("role").cloned())
        .unwrap_or_default()
}

fn view(m: &moves::Model) -> Value {
    json!({
        "pid": m.pid, "plan_pid": m.plan_pid, "programme_pid": m.programme_pid, "task_pid": m.task_pid,
        "from_start": m.from_start, "to_start": m.to_start, "reason": m.reason,
        "requested_by": m.requested_by, "organization_pid": m.organization_pid,
        "approver_ref": m.approver_ref, "status": m.status, "decided_by": m.decided_by,
        "decided_on": m.decided_on, "decision_reason": m.decision_reason,
        "knock_on": serde_json::from_str::<Value>(&m.knock_on).unwrap_or_default(),
        "knock_on_cost": serde_json::from_str::<Value>(&m.knock_on_cost).unwrap_or_default(),
    })
}

/// `POST /api/plans/{pid}/deployment-moves`
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn request_move(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<NewMove>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let Some(programme) = programme_of(&ctx, &plan).await? else {
        return Err(unprocessable("this plan is not part of a programme"));
    };
    let model = programme_profiles::Entity::find()
        .filter(programme_profiles::Column::PlanPid.eq(programme.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .and_then(|p| p.delivery_model);
    if model.as_deref() != Some(rules::CONTROLLED_MODEL) {
        return Err(unprocessable(
            "a deployment date is under change control only in a programme delivered as a central platform with local deployment (set it with PUT /api/plans/{pid}/delivery-model)",
        ));
    }
    if b.reason.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a reason of at least 10 characters is required",
        ));
    }
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(b.task_pid))
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or_else(|| unprocessable("task_pid is not a task of this deployment"))?;
    let Some(from) = task.start_date else {
        return Err(unprocessable("the go-live task has no start date to move"));
    };
    if from == b.new_start {
        return Err(unprocessable("new_start is the date it already has"));
    }
    let approver = b
        .approver_ref
        .clone()
        .or_else(rules::default_approver)
        .ok_or_else(|| unprocessable("name an approver (approver_ref), or have the deployment set PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_APPROVER"))?;
    if let Some(why) = rules::approver_problem(&approver, caller.actor()) {
        return Err(unprocessable(&why));
    }
    if moves::Entity::find()
        .filter(moves::Column::TaskPid.eq(task.pid))
        .filter(moves::Column::Status.eq("requested"))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .is_some()
    {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "conflict",
                "a move of this go-live is already waiting for a decision",
            ),
        ));
    }

    // The knock-on: the network across linked plans (as the caller may read them) as it stands, and with the
    // moved date. Nothing is written.
    let net = super::cpm::load_network_cross(&ctx, plan.pid, &caller).await?;
    let cycle = |cpm::CpmError::Cycle(_)| unprocessable("the dependencies contain a cycle");
    let before = net.analyze(0).map_err(cycle)?;
    let mut nodes = net.nodes.clone();
    for n in &mut nodes {
        if n.pid == task.pid {
            n.planned_start = Some(b.new_start);
        }
    }
    let after = cpm::analyze_with(
        &nodes,
        &net.edges,
        &net.plan_cal,
        &net.own,
        Options {
            near_critical_days: 0,
            ..Options::default()
        },
    )
    .map_err(cycle)?;
    let titles: HashMap<Uuid, (&str, Uuid)> = net
        .rows
        .iter()
        .map(|t| (t.pid, (t.title.as_str(), t.plan_pid)))
        .collect();
    let knock_on: Vec<Value> = cpm::diff(&before, &after)
        .into_iter()
        .filter(|m| m.pid != task.pid)
        .map(|m| {
            let (title, plan_of) = titles
                .get(&m.pid)
                .copied()
                .or_else(|| net.external.get(&m.pid).map(|e| (e.title.as_str(), e.plan_pid)))
                .unwrap_or(("", Uuid::nil()));
            json!({ "task_pid": m.pid, "title": title, "plan_pid": plan_of, "other_deployment": plan_of != plan.pid,
                "moved": m.moved, "start_days": m.start_days, "finish_days": m.finish_days })
        })
        .collect();
    let b_cost = super::cost_rates::labour_totals(&ctx, &net, Some(&before)).await?;
    let a_cost = super::cost_rates::labour_totals(&ctx, &net, Some(&after)).await?;
    let mut currencies: Vec<&String> = b_cost
        .by_currency
        .keys()
        .chain(a_cost.by_currency.keys())
        .collect();
    currencies.sort();
    currencies.dedup();
    let cost: Vec<Value> = currencies
        .iter()
        .map(|c| {
            let (x, y) = (
                b_cost.by_currency.get(*c).copied().unwrap_or(0),
                a_cost.by_currency.get(*c).copied().unwrap_or(0),
            );
            json!({ "currency": c, "delta_minor": y - x })
        })
        .collect();
    let row = moves::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        programme_pid: ActiveValue::set(programme.pid),
        task_pid: ActiveValue::set(task.pid),
        from_start: ActiveValue::set(from),
        to_start: ActiveValue::set(b.new_start),
        reason: ActiveValue::set(b.reason.trim().to_string()),
        requested_by: ActiveValue::set(caller.actor().map(str::to_string)),
        organization_pid: ActiveValue::set(super::organizations::caller_organizations(&caller).first().copied()),
        approver_ref: ActiveValue::set(approver),
        status: ActiveValue::set("requested".to_string()),
        knock_on: ActiveValue::set(json!(knock_on).to_string()),
        knock_on_cost: ActiveValue::set(json!({ "labour": cost, "unrated_minutes_before": b_cost.unrated_minutes, "unrated_minutes_after": a_cost.unrated_minutes }).to_string()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(&ctx.db, row.pid, "deployment_move_requested", caller.actor(), Some(json!({ "task": task.pid, "from": from, "to": b.new_start, "knock_on": knock_on.len() })))
        .await;
    format::json(view(&row))
}

#[derive(Debug, Deserialize)]
struct Decision {
    /// `approve` or `refuse`.
    decision: String,
    reason: String,
}

/// `POST /api/plans/{pid}/deployment-moves/{mpid}/decision`
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, mpid)): Path<(String, String)>,
    Json(b): Json<Decision>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mpid = super::parse_pid(&mpid)?;
    let m = moves::Entity::find()
        .filter(moves::Column::Pid.eq(mpid))
        .filter(moves::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if !["approve", "refuse"].contains(&b.decision.as_str()) {
        return Err(unprocessable("decision must be approve or refuse"));
    }
    if b.reason.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a decision needs a reason of at least 10 characters",
        ));
    }
    if m.status != "requested" {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "this move has been decided"),
        ));
    }
    if !rules::may_decide(
        &m.approver_ref,
        caller.actor(),
        &roles_of(&caller),
        m.requested_by.as_deref(),
    ) {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new(
                "forbidden",
                "only the named approver decides a move, and never the person who asked for it",
            ),
        ));
    }
    let approve = b.decision == "approve";
    if approve {
        let task = tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(m.task_pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .ok_or(Error::NotFound)?;
        if task.start_date != Some(m.from_start) {
            return Err(Error::CustomError(
                StatusCode::CONFLICT,
                ErrorDetail::new(
                    "stale_move",
                    "the go-live date has changed since this was asked; the knock-on no longer holds: ask again",
                ),
            ));
        }
        let mut a: tasks::ActiveModel = task.into();
        a.start_date = ActiveValue::set(Some(m.to_start));
        a.update(&ctx.db).await.map_err(db_err)?;
        AuditModel::record_or_log(
            &ctx.db,
            m.task_pid,
            "deployment_go_live_moved",
            caller.actor(),
            Some(json!({ "from": m.from_start, "to": m.to_start, "move": m.pid })),
        )
        .await;
    }
    let mut a: moves::ActiveModel = m.into();
    a.status = ActiveValue::set(if approve { "approved" } else { "refused" }.to_string());
    a.decided_by = ActiveValue::set(caller.actor().map(str::to_string));
    a.decided_on = ActiveValue::set(Some(crate::tz::deployment_today()));
    a.decision_reason = ActiveValue::set(Some(b.reason.trim().to_string()));
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        if approve {
            "deployment_move_approved"
        } else {
            "deployment_move_refused"
        },
        caller.actor(),
        None,
    )
    .await;
    format::json(view(&row))
}

/// `GET /api/plans/{pid}/deployment-moves`
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = moves::Entity::find()
        .filter(moves::Column::PlanPid.eq(plan.pid))
        .order_by_desc(moves::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    // The knock-on cost recorded against each requester (all requests, whatever their outcome).
    let mut by: BTreeMap<String, (usize, BTreeMap<String, i64>)> = BTreeMap::new();
    for r in &rows {
        let e = by
            .entry(r.requested_by.clone().unwrap_or_default())
            .or_default();
        e.0 += 1;
        let cost: Value = serde_json::from_str(&r.knock_on_cost).unwrap_or_default();
        for l in cost["labour"].as_array().into_iter().flatten() {
            *e.1.entry(l["currency"].as_str().unwrap_or("").to_string())
                .or_default() += l["delta_minor"].as_i64().unwrap_or(0);
        }
    }
    format::json(json!({
        "moves": rows.iter().map(view).collect::<Vec<_>>(),
        "cost_by_requester": by.into_iter().map(|(who, (n, c))| json!({ "requested_by": who, "requests": n,
            "knock_on_labour": c.into_iter().map(|(cur, d)| json!({ "currency": cur, "delta_minor": d })).collect::<Vec<_>>() })).collect::<Vec<_>>(),
        "note": "A move changes nothing until its approver approves it. The knock-on was worked out when it was asked, by running the schedule network (across linked plans you may read) with the moved date.",
    }))
}

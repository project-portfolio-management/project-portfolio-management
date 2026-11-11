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

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cpm::{self, Options};
use crate::deployment_moves as rules;
use crate::models::_entities::{deployment_moves as moves, plans, tasks};
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

/// Body of `POST /api/plans/{pid}/deployment-moves`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewMove {
    /// The go-live task to move.
    task_pid: Uuid,
    /// The requested new start date for the task.
    new_start: NaiveDate,
    /// Why the move is wanted; at least 10 characters after trimming.
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
        let Some(p) = sqlx::query_as::<_, plans::Model>(
            "SELECT * FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(parent)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        else {
            return Ok(None);
        };
        cur = p;
    }
    Ok(None)
}

/// The caller's `role` attribute values from the token; empty when unauthenticated or none.
fn roles_of(caller: &MaybeAuthUser) -> Vec<String> {
    caller
        .claims()
        .and_then(|c| c.attrs.get("role").cloned())
        .unwrap_or_default()
}

/// Render a move request with its knock-on effects and cost (stored as JSON text; an unreadable value shows as null).
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
    let model: Option<String> = sqlx::query_scalar::<_, Option<String>>(
        "SELECT delivery_model FROM programme_profiles WHERE plan_pid = $1 LIMIT 1",
    )
    .bind(programme.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .flatten();
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
    let task: tasks::Model = sqlx::query_as(
        "SELECT * FROM tasks WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(b.task_pid)
    .bind(plan.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
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
    if sqlx::query_scalar::<_, i64>(
        "SELECT id FROM deployment_moves WHERE task_pid = $1 AND status = $2 LIMIT 1",
    )
    .bind(task.pid)
    .bind("requested")
    .fetch_optional(crate::db::pool(&ctx))
    .await?
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
    let mut tx = crate::db::begin(&ctx).await?;
    let row: moves::Model = sqlx::query_as(
        "INSERT INTO deployment_moves
             (pid, plan_pid, programme_pid, task_pid, from_start, to_start, reason, requested_by,
              organization_pid, approver_ref, status, knock_on, knock_on_cost)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(programme.pid)
    .bind(task.pid)
    .bind(from)
    .bind(b.new_start)
    .bind(b.reason.trim())
    .bind(caller.actor())
    .bind(super::organizations::caller_organizations(&caller).first().copied())
    .bind(approver)
    .bind("requested")
    .bind(json!(knock_on).to_string())
    .bind(json!({ "labour": cost, "unrated_minutes_before": b_cost.unrated_minutes, "unrated_minutes_after": a_cost.unrated_minutes }).to_string())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "deployment_move_requested", caller.actor(), Some(json!({ "task": task.pid, "from": from, "to": b.new_start, "knock_on": knock_on.len() }))).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row))
}

/// Body of `POST /api/plans/{pid}/deployment-moves/{mpid}/decision`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Decision {
    /// `approve` or `refuse`.
    decision: String,
    /// Why the decision was made; at least 10 characters after trimming.
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
    let m: moves::Model =
        sqlx::query_as("SELECT * FROM deployment_moves WHERE pid = $1 AND plan_pid = $2 LIMIT 1")
            .bind(mpid)
            .bind(plan.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
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
    // The go-live date, the move and both audit rows are one change.
    let mut tx = crate::db::begin(&ctx).await?;
    if approve {
        let task: tasks::Model =
            sqlx::query_as("SELECT * FROM tasks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
                .bind(m.task_pid)
                .fetch_optional(tx.conn())
                .await?
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
        // `fetch_one`: an update that finds no row is an error, as before SQ-1.
        sqlx::query_scalar::<_, i32>("UPDATE tasks SET start_date = $1 WHERE id = $2 RETURNING id")
            .bind(Some(m.to_start))
            .bind(task.id)
            .fetch_one(tx.conn())
            .await?;
        AuditModel::record_on(
            tx.conn(),
            m.task_pid,
            "deployment_go_live_moved",
            caller.actor(),
            Some(json!({ "from": m.from_start, "to": m.to_start, "move": m.pid })),
        )
        .await
        .map_err(super::audit_failed)?;
    }
    let row: moves::Model = sqlx::query_as(
        "UPDATE deployment_moves
         SET status = $1, decided_by = $2, decided_on = $3, decision_reason = $4
         WHERE id = $5
         RETURNING *",
    )
    .bind(if approve { "approved" } else { "refused" })
    .bind(caller.actor())
    .bind(Some(crate::tz::deployment_today()))
    .bind(Some(b.reason.trim()))
    .bind(m.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        if approve {
            "deployment_move_approved"
        } else {
            "deployment_move_refused"
        },
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row))
}

/// `GET /api/plans/{pid}/deployment-moves`
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows: Vec<moves::Model> =
        sqlx::query_as("SELECT * FROM deployment_moves WHERE plan_pid = $1 ORDER BY id DESC")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
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

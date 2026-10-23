//! Delivery confidence (DP-2), the exceptions it opens (DP-3) and strategic aims with the portfolio
//! map (DP-4). Rules in [`crate::delivery_confidence`].
//!
//! A stated confidence is **append-only** and sits beside the derived RAG; it is never blended with it.
//! Setting `amber_red` or `red` opens an exception and starts a clock; a worsening while one is open
//! is recorded without restarting it; only a recorded decision closes it. The map and the exception
//! lists are built here once and used both by the live routes (for the plans the caller may read) and by
//! the director pack (for plans that are neither private nor marked).

use super::refuse;
use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{DateTime, NaiveDate, Utc};
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use super::unprocessable_entity as unprocessable;
use crate::auth::{MaybeAuthUser, require_auth};
use crate::delivery_confidence::{self as rules, Assessment, Parts};
use crate::models::_entities::{
    budget_lines, delivery_confidence_assessments as assessments,
    delivery_exceptions as exceptions, objective_links, objectives, plans, risks, strategic_aims,
};
use crate::models::audit_logs::Model as AuditModel;

fn is_admin(caller: &MaybeAuthUser) -> bool {
    super::plan_access::is_admin(caller.claims())
}

/// Whether the caller holds `confidence=independent` (or is an administrator): only they may record an
/// independent assessment. With login off everyone is trusted, as for every other write.
fn may_assess_independently(caller: &MaybeAuthUser) -> bool {
    if !require_auth() {
        return true;
    }
    is_admin(caller)
        || caller
            .claims()
            .and_then(|c| c.attrs.get("confidence"))
            .is_some_and(|v| v.iter().any(|x| x == "independent"))
}

// ---------------------------------------------------------------------------------------------------
// DP-2: delivery confidence
// ---------------------------------------------------------------------------------------------------

fn assessment_json(a: &assessments::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": a.pid, "plan_pid": a.plan_pid, "value": a.value, "kind": a.kind,
        "rationale": a.rationale, "assessor": a.assessor, "assessed_on": a.assessed_on,
    })
}

#[derive(Debug, Deserialize)]
struct AssessBody {
    value: String,
    kind: String,
    rationale: String,
    /// Defaults to today; not in the future.
    #[serde(default)]
    assessed_on: Option<NaiveDate>,
}

fn exception_json(e: &exceptions::Model, now: DateTime<Utc>, window: i64) -> serde_json::Value {
    let parts = Parts {
        cause: e.cause.as_deref(),
        decision_needed: e.decision_needed.as_deref(),
        owner_ref: e.owner_ref.as_deref(),
        decide_by: e.decide_by,
    };
    let opened: DateTime<Utc> = e.opened_at.into();
    let completed: Option<DateTime<Utc>> = e.completed_at.map(Into::into);
    let closed: Option<DateTime<Utc>> = e.closed_at.map(Into::into);
    // A closed exception is judged at its completion; an open one by the clock now.
    let breach = rules::breached(opened, completed, closed.unwrap_or(now), window);
    serde_json::json!({
        "pid": e.pid, "plan_pid": e.plan_pid, "status": e.status, "opened_at": e.opened_at,
        "opened_value": e.opened_value, "worst_value": e.worst_value, "worsened_count": e.worsened_count,
        "cause": e.cause, "decision_needed": e.decision_needed, "owner_ref": e.owner_ref,
        "owner_role": e.owner_role, "decide_by": e.decide_by, "completed_at": e.completed_at,
        "complete": rules::missing(&parts).is_empty(), "missing": rules::missing(&parts),
        "breached": breach, "window_hours": window,
        "hours_open": (closed.unwrap_or(now) - opened).num_hours(),
        "closed_at": e.closed_at, "decision": e.decision, "decided_by": e.decided_by,
    })
}

/// `PUT /api/plans/{pid}/delivery-confidence` — append a stated assessment (it never replaces one).
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn set_confidence(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<AssessBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let problems = rules::assessment_problems(&b.value, &b.kind, &b.rationale);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let today = crate::tz::utc_today();
    let on = b.assessed_on.unwrap_or(today);
    if on > today {
        return Err(unprocessable("assessed_on must not be in the future"));
    }
    if b.kind == "independent" && !may_assess_independently(&caller) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "an independent assessment needs access=admin or confidence=independent",
        ));
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let tx = ctx.db.begin().await.map_err(db_err)?;
    let previous = assessments::Entity::find()
        .filter(assessments::Column::PlanPid.eq(plan.pid))
        .order_by_desc(assessments::Column::Id)
        .one(&tx)
        .await
        .map_err(db_err)?;
    let row = assessments::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        value: ActiveValue::set(b.value.clone()),
        kind: ActiveValue::set(b.kind.clone()),
        rationale: ActiveValue::set(b.rationale.trim().to_string()),
        assessor: ActiveValue::set(Some(actor.clone())),
        assessed_on: ActiveValue::set(on),
        ..Default::default()
    }
    .insert(&tx)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &tx,
        row.pid,
        "delivery_confidence_set",
        Some(&actor),
        Some(serde_json::json!({
            "plan_pid": plan.pid, "value": b.value, "kind": b.kind,
            "previous": previous.as_ref().map(|p| p.value.clone()),
        })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    // DP-3: reaching amber-red or red opens an exception; a worsening while one is open is recorded and
    // does not restart its clock; improving never closes it (only a decision does).
    let mut exception = None;
    if rules::opens_exception(&b.value) {
        let open = exceptions::Entity::find()
            .filter(exceptions::Column::PlanPid.eq(plan.pid))
            .filter(exceptions::Column::Status.eq("open"))
            .one(&tx)
            .await
            .map_err(db_err)?;
        let now = Utc::now();
        exception = Some(if let Some(open) = open {
            if rules::worsens(&open.worst_value, &b.value) {
                let mut a: exceptions::ActiveModel = open.clone().into();
                a.worst_value = ActiveValue::set(b.value.clone());
                a.worsened_count = ActiveValue::set(open.worsened_count + 1);
                let updated = a.update(&tx).await.map_err(db_err)?;
                AuditModel::record(
                    &tx,
                    updated.pid,
                    "delivery_exception_worsened",
                    Some(&actor),
                    Some(serde_json::json!({ "from": open.worst_value, "to": b.value })),
                )
                .await
                .map_err(|e| Error::string(&e.to_string()))?;
                updated
            } else {
                open
            }
        } else {
            let opened = exceptions::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                status: ActiveValue::set("open".to_string()),
                opened_at: ActiveValue::set(now.into()),
                opened_by: ActiveValue::set(Some(actor.clone())),
                opened_value: ActiveValue::set(b.value.clone()),
                worst_value: ActiveValue::set(b.value.clone()),
                worsened_count: ActiveValue::set(0),
                ..Default::default()
            }
            .insert(&tx)
            .await
            .map_err(db_err)?;
            AuditModel::record(
                &tx,
                opened.pid,
                "delivery_exception_opened",
                Some(&actor),
                Some(serde_json::json!({ "plan_pid": plan.pid, "value": b.value })),
            )
            .await
            .map_err(|e| Error::string(&e.to_string()))?;
            opened
        });
    }
    tx.commit().await.map_err(db_err)?;
    format::json(serde_json::json!({
        "assessment": assessment_json(&row),
        "exception": exception.map(|e| exception_json(&e, Utc::now(), rules::window_hours())),
    }))
}

/// `GET /api/plans/{pid}/delivery-confidence` — the whole history, oldest first, and what the map shows.
#[debug_handler]
async fn get_confidence(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = assessments::Entity::find()
        .filter(assessments::Column::PlanPid.eq(plan.pid))
        .order_by_asc(assessments::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let history: Vec<Assessment> = rows.iter().map(lite).collect();
    let period = Utc::now().format("%Y-%m").to_string();
    let shown = rules::shown(&history).map(|s| {
        serde_json::json!({
            "value": s.value, "kind": s.kind, "assessed_on": s.assessed_on,
            "stale": rules::is_stale(s.assessed_on, &period),
            "marked_self": s.kind == "self",
        })
    });
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "shown": shown.clone().unwrap_or_else(|| serde_json::json!({ "value": "not_assessed" })),
        "history": rows.iter().map(assessment_json).collect::<Vec<_>>(),
        "note": "Stated by a person and kept beside the derived RAG, never blended with it. No assessment is not_assessed, not green. The history is append-only.",
    }))
}

fn lite(a: &assessments::Model) -> Assessment {
    Assessment {
        value: a.value.clone(),
        kind: a.kind.clone(),
        assessed_on: a.assessed_on,
    }
}

// ---------------------------------------------------------------------------------------------------
// DP-3: exceptions
// ---------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ExceptionQuery {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    breached: Option<bool>,
}

/// `GET /api/exceptions[?status&breached]` — across the plans you can read.
#[debug_handler]
async fn list_exceptions(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ExceptionQuery>,
) -> Result<Response> {
    if q.status
        .as_deref()
        .is_some_and(|s| !matches!(s, "open" | "closed"))
    {
        return Err(unprocessable("status must be open or closed"));
    }
    let rows = exceptions::Entity::find()
        .order_by_desc(exceptions::Column::OpenedAt)
        .limit(1000)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut plans: Vec<Uuid> = rows.iter().map(|r| r.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    let (now, window) = (Utc::now(), rules::window_hours());
    let shown: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| readable.contains(&r.plan_pid))
        .filter(|r| q.status.as_ref().is_none_or(|s| &r.status == s))
        .map(|r| exception_json(r, now, window))
        .filter(|v| q.breached.is_none_or(|b| v["breached"] == b))
        .collect();
    format::json(serde_json::json!({ "exceptions": shown, "window_hours": window }))
}

pub(crate) async fn find_exception(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<exceptions::Model> {
    let pid = super::parse_pid(pid)?;
    let row = exceptions::Entity::find()
        .filter(exceptions::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    // The same answer as an exception that does not exist when its plan is hidden from the caller.
    if !super::plan_access::readable_plans(ctx, caller, &[row.plan_pid])
        .await
        .contains(&row.plan_pid)
    {
        return Err(Error::NotFound);
    }
    Ok(row)
}

#[derive(Debug, Deserialize)]
struct CompleteBody {
    #[serde(default)]
    cause: Option<String>,
    #[serde(default)]
    decision_needed: Option<String>,
    #[serde(default)]
    owner_ref: Option<String>,
    /// The owner's role, which is what a pack shows instead of the person.
    #[serde(default)]
    owner_role: Option<String>,
    #[serde(default)]
    decide_by: Option<NaiveDate>,
}

/// `PUT /api/exceptions/{pid}` — fill in the cause, the decision needed, the owner and the decide-by date.
/// Fields left out are kept; the exception is complete when all four are there.
#[debug_handler]
async fn complete_exception(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<CompleteBody>,
) -> Result<Response> {
    let row = find_exception(&ctx, &caller, &pid).await?;
    if row.status != "open" {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this exception is closed",
        ));
    }
    let clean = |s: &Option<String>| {
        s.as_ref()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    };
    let mut a: exceptions::ActiveModel = row.clone().into();
    if let Some(v) = clean(&b.cause) {
        a.cause = ActiveValue::set(Some(v));
    }
    if let Some(v) = clean(&b.decision_needed) {
        a.decision_needed = ActiveValue::set(Some(v));
    }
    if let Some(v) = clean(&b.owner_ref) {
        a.owner_ref = ActiveValue::set(Some(v));
    }
    if let Some(v) = clean(&b.owner_role) {
        a.owner_role = ActiveValue::set(Some(v));
    }
    if let Some(d) = b.decide_by {
        a.decide_by = ActiveValue::set(Some(d));
    }
    let tx = ctx.db.begin().await.map_err(db_err)?;
    let merged = a.update(&tx).await.map_err(db_err)?;
    let parts = Parts {
        cause: merged.cause.as_deref(),
        decision_needed: merged.decision_needed.as_deref(),
        owner_ref: merged.owner_ref.as_deref(),
        decide_by: merged.decide_by,
    };
    let done = rules::missing(&parts).is_empty();
    let saved = if done && merged.completed_at.is_none() {
        let mut a: exceptions::ActiveModel = merged.into();
        a.completed_at = ActiveValue::set(Some(Utc::now().into()));
        a.update(&tx).await.map_err(db_err)?
    } else {
        merged
    };
    AuditModel::record(
        &tx,
        saved.pid,
        "delivery_exception_updated",
        caller.actor(),
        Some(serde_json::json!({ "complete": done })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(db_err)?;
    format::json(exception_json(&saved, Utc::now(), rules::window_hours()))
}

#[derive(Debug, Deserialize)]
struct DecisionBody {
    decision: String,
}

/// `POST /api/exceptions/{pid}/decision` — record the decision, which closes it.
#[debug_handler]
async fn decide_exception(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<DecisionBody>,
) -> Result<Response> {
    let row = find_exception(&ctx, &caller, &pid).await?;
    if row.threshold_kind.is_some() {
        return Err(unprocessable(
            "this exception was opened by a threshold: it needs a pause, reset, stop or recorded override (POST /api/exceptions/{pid}/threshold-decision)",
        ));
    }
    if b.decision.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a decision of at least 10 characters is required",
        ));
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let tx = ctx.db.begin().await.map_err(db_err)?;
    let done = exceptions::Entity::update_many()
        .col_expr(exceptions::Column::Status, Expr::value("closed"))
        .col_expr(exceptions::Column::ClosedAt, Expr::value(Utc::now()))
        .col_expr(
            exceptions::Column::Decision,
            Expr::value(b.decision.trim().to_string()),
        )
        .col_expr(exceptions::Column::DecidedBy, Expr::value(actor.clone()))
        .filter(exceptions::Column::Pid.eq(row.pid))
        .filter(exceptions::Column::Status.eq("open"))
        .exec(&tx)
        .await
        .map_err(db_err)?;
    if done.rows_affected == 0 {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this exception is already closed",
        ));
    }
    AuditModel::record(
        &tx,
        row.pid,
        "delivery_exception_closed",
        Some(&actor),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(db_err)?;
    let now = exceptions::Entity::find()
        .filter(exceptions::Column::Pid.eq(row.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    format::json(exception_json(&now, Utc::now(), rules::window_hours()))
}

// ---------------------------------------------------------------------------------------------------
// DP-4: strategic aims and the portfolio map
// ---------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct AimBody {
    strategic: bool,
}

/// `PUT /api/objectives/{pid}/strategic-aim` — mark or unmark an objective as a strategic aim.
#[debug_handler]
async fn mark_aim(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<AimBody>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let objective = objectives::Entity::find()
        .filter(objectives::Column::Pid.eq(pid))
        .filter(objectives::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let tx = ctx.db.begin().await.map_err(db_err)?;
    let existing = strategic_aims::Entity::find()
        .filter(strategic_aims::Column::ObjectivePid.eq(objective.pid))
        .one(&tx)
        .await
        .map_err(db_err)?;
    match (b.strategic, existing) {
        (true, None) => {
            strategic_aims::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                objective_pid: ActiveValue::set(objective.pid),
                marked_by: ActiveValue::set(caller.actor().map(str::to_string)),
                ..Default::default()
            }
            .insert(&tx)
            .await
            .map_err(db_err)?;
        }
        (false, Some(row)) => {
            // Unmarking also clears the primary flag of every programme that pointed at it.
            objective_links::Entity::update_many()
                .col_expr(objective_links::Column::IsPrimary, Expr::value(false))
                .filter(objective_links::Column::ObjectivePid.eq(objective.pid))
                .exec(&tx)
                .await
                .map_err(db_err)?;
            row.delete(&tx).await.map_err(db_err)?;
        }
        _ => {}
    }
    AuditModel::record(
        &tx,
        objective.pid,
        if b.strategic {
            "strategic_aim_marked"
        } else {
            "strategic_aim_unmarked"
        },
        caller.actor(),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(db_err)?;
    format::json(serde_json::json!({ "objective_pid": objective.pid, "strategic": b.strategic }))
}

#[derive(Debug, Deserialize)]
struct PrimaryBody {
    objective_pid: Uuid,
}

/// `PUT /api/plans/{pid}/primary-aim` — the programme's one primary strategic aim. It links the
/// programme to the aim if it is not already, and makes any earlier primary an ordinary link.
#[debug_handler]
async fn set_primary(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<PrimaryBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let is_aim = strategic_aims::Entity::find()
        .filter(strategic_aims::Column::ObjectivePid.eq(b.objective_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .is_some();
    if !is_aim {
        return Err(unprocessable(
            "that objective is not marked as a strategic aim",
        ));
    }
    let tx = ctx.db.begin().await.map_err(db_err)?;
    objective_links::Entity::update_many()
        .col_expr(objective_links::Column::IsPrimary, Expr::value(false))
        .filter(objective_links::Column::PlanPid.eq(plan.pid))
        .exec(&tx)
        .await
        .map_err(db_err)?;
    let existing = objective_links::Entity::find()
        .filter(objective_links::Column::PlanPid.eq(plan.pid))
        .filter(objective_links::Column::ObjectivePid.eq(b.objective_pid))
        .one(&tx)
        .await
        .map_err(db_err)?;
    if let Some(link) = existing {
        let mut a: objective_links::ActiveModel = link.into();
        a.is_primary = ActiveValue::set(true);
        a.update(&tx).await.map_err(db_err)?;
    } else {
        objective_links::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            objective_pid: ActiveValue::set(b.objective_pid),
            plan_pid: ActiveValue::set(plan.pid),
            weight: ActiveValue::set(3),
            is_primary: ActiveValue::set(true),
            ..Default::default()
        }
        .insert(&tx)
        .await
        .map_err(db_err)?;
    }
    AuditModel::record(
        &tx,
        plan.pid,
        "primary_aim_set",
        caller.actor(),
        Some(serde_json::json!({ "objective_pid": b.objective_pid })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(db_err)?;
    format::json(serde_json::json!({ "plan_pid": plan.pid, "primary_aim": b.objective_pid }))
}

/// One count out of a previous pack's map: the confidence value shown for a programme.
fn previous_value(body: Option<&str>, plan: Uuid) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(body?).ok()?;
    let groups = v.get("portfolio_map")?.get("groups")?.as_array()?;
    for g in groups {
        for p in g.get("programmes")?.as_array()? {
            if p.get("plan_pid")?.as_str()? == plan.to_string() {
                return p
                    .get("confidence")?
                    .get("value")?
                    .as_str()
                    .map(str::to_string);
            }
        }
    }
    None
}

/// Build the portfolio map for the programmes among `include`, as of `today` for the pack `period`.
/// `previous_body` is the previous issued pack, for the trend.
///
/// # Errors
///
/// When a database read fails.
#[allow(clippy::too_many_lines)]
pub(crate) async fn build_map(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    period: &str,
    previous_body: Option<&str>,
) -> Result<serde_json::Value> {
    let today = crate::tz::utc_today();
    let all_plans = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let programmes: Vec<&plans::Model> = all_plans
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    let risk_rows = risks::Entity::find().all(&ctx.db).await.map_err(db_err)?;
    let budget_rows = budget_lines::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let deps = crate::models::visibility::all_dependencies(&ctx.db).await?;
    let scoped: Vec<plans::Model> = all_plans
        .iter()
        .filter(|p| include.contains(&p.pid))
        .cloned()
        .collect();
    let rag = super::insights::rag_by_pid(&scoped, &risk_rows, &budget_rows, &deps, today);
    let aims: HashMap<Uuid, String> = {
        let marked: HashSet<Uuid> = strategic_aims::Entity::find()
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|a| a.objective_pid)
            .collect();
        objectives::Entity::find()
            .filter(objectives::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .filter(|o| marked.contains(&o.pid))
            .map(|o| (o.pid, o.title))
            .collect()
    };
    let links = objective_links::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let statuses = super::programme_gates::status_map(ctx, today).await?;
    let (profiles, org_names) = super::programme_profile::all(ctx, include).await?;
    let history = assessments::Entity::find()
        .order_by_asc(assessments::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut by_plan: HashMap<Uuid, Vec<Assessment>> = HashMap::new();
    for h in &history {
        by_plan.entry(h.plan_pid).or_default().push(lite(h));
    }
    // group key: Some(aim pid) or None for "no strategic aim".
    let mut groups: BTreeMap<Option<Uuid>, Vec<serde_json::Value>> = BTreeMap::new();
    for p in &programmes {
        let mine: Vec<&objective_links::Model> = links
            .iter()
            .filter(|l| l.plan_pid == p.pid && aims.contains_key(&l.objective_pid))
            .collect();
        let primary = mine.iter().find(|l| l.is_primary).map(|l| l.objective_pid);
        let others: Vec<String> = mine
            .iter()
            .filter(|l| Some(l.objective_pid) != primary)
            .filter_map(|l| aims.get(&l.objective_pid).cloned())
            .collect();
        let sh = rules::shown(by_plan.get(&p.pid).map_or(&[][..], Vec::as_slice));
        let derived = rag.get(&p.pid).copied();
        let value = sh.as_ref().map(|s| s.value.clone());
        let disagreement = match (&value, derived.and_then(rules::rank_of_rag)) {
            (Some(v), Some(d)) if rules::disagree(v, d) => Some("derived"),
            _ => None,
        };
        let self_gap = {
            let h = by_plan.get(&p.pid).map_or(&[][..], Vec::as_slice);
            let ind = h.iter().rev().find(|a| a.kind == "independent");
            let own = h.iter().rev().find(|a| a.kind == "self");
            match (ind, own) {
                (Some(i), Some(s)) => {
                    rules::rank(&s.value).is_some_and(|r| rules::disagree(&i.value, r))
                }
                _ => false,
            }
        };
        let confidence = match &sh {
            Some(s) => serde_json::json!({
                "value": s.value, "kind": s.kind, "marked_self": s.kind == "self",
                "assessed_on": s.assessed_on, "stale": rules::is_stale(s.assessed_on, period),
            }),
            None => serde_json::json!({ "value": "not_assessed" }),
        };
        let prev = previous_value(previous_body, p.pid);
        let trend = if previous_body.is_none() {
            "no_earlier_pack"
        } else {
            rules::trend(prev.as_deref(), value.as_deref())
        };
        groups.entry(primary).or_default().push(serde_json::json!({
            "plan_pid": p.pid, "name": p.name, "stage": p.stage, "phase": p.phase,
            "programme_status": statuses.get(&p.pid).copied().unwrap_or("active"),
            "delivery": super::programme_profile::view(profiles.get(&p.pid), &org_names),
            "confidence": confidence, "derived_rag": derived,
            "trend": trend,
            "disagreement": { "with_derived": disagreement.is_some(), "independent_vs_self": self_gap },
            "also_serves": others,
            "reason_no_primary": (primary.is_none() && !mine.is_empty())
                .then_some("linked to strategic aims but none is marked primary"),
        }));
    }
    let mut out = Vec::new();
    let mut titled: Vec<(&Uuid, &String)> = aims.iter().collect();
    titled.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
    for (pid, title) in titled {
        let progs = groups.remove(&Some(*pid)).unwrap_or_default();
        out.push(serde_json::json!({ "aim_pid": pid, "aim": title, "programmes": progs }));
    }
    let none = groups.remove(&None).unwrap_or_default();
    out.push(serde_json::json!({ "aim_pid": null, "aim": "no strategic aim", "programmes": none }));
    let total: usize = out
        .iter()
        .map(|g| g["programmes"].as_array().map_or(0, Vec::len))
        .sum();
    Ok(serde_json::json!({
        "period": period,
        "programmes_total": total,
        "groups": out,
        "note": "Programmes are plans of kind Program. A programme appears once, under its primary strategic aim; other aims it serves are named. Delivery confidence is the stated value (latest independent, else latest self, marked), beside the derived RAG, never blended. Not assessed is not green. Trend compares with the previous issued pack.",
    }))
}

/// `GET /api/portfolio-map[?period=YYYY-MM]` — the map for the programmes you can read.
#[derive(Debug, Deserialize)]
struct MapQuery {
    #[serde(default)]
    period: Option<String>,
}

#[debug_handler]
async fn portfolio_map(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<MapQuery>,
) -> Result<Response> {
    let period = q
        .period
        .unwrap_or_else(|| Utc::now().format("%Y-%m").to_string());
    if !crate::director_pack::valid_period(&period) {
        return Err(unprocessable("period must be YYYY-MM"));
    }
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let map = build_map(&ctx, &readable, &period, None).await?;
    format::json(map)
}

/// The exceptions section of a pack, over the plans in `include`.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn exceptions_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    period: &str,
    now: DateTime<Utc>,
) -> Result<serde_json::Value> {
    let names: HashMap<Uuid, String> = plans::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let rows: Vec<exceptions::Model> = exceptions::Entity::find()
        .order_by_asc(exceptions::Column::OpenedAt)
        .all(ctx_db(ctx))
        .await
        .map_err(db_err)?
        .into_iter()
        .filter(|r| include.contains(&r.plan_pid))
        .collect();
    let window = rules::window_hours();
    let start = rules::period_start(period);
    let item = |r: &exceptions::Model| {
        let j = exception_json(r, now, window);
        // The pack names the owner by role and never by person.
        serde_json::json!({
            "plan_pid": r.plan_pid, "plan": names.get(&r.plan_pid), "opened_at": r.opened_at,
            "worst_value": r.worst_value, "worsened_count": r.worsened_count,
            "cause": r.cause, "decision_needed": r.decision_needed,
            "owner_role": r.owner_role, "decide_by": r.decide_by,
            "complete": j["complete"], "breached": j["breached"], "hours_open": j["hours_open"],
            "decision": r.decision, "closed_at": r.closed_at,
        })
    };
    let open: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| r.status == "open")
        .map(item)
        .collect();
    let breached: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| exception_json(r, now, window)["breached"] == true)
        .map(item)
        .collect();
    let closed: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| {
            r.status == "closed"
                && r.closed_at.is_some_and(|c| {
                    let d = DateTime::<Utc>::from(c).date_naive();
                    start.is_some_and(|s| d >= s && d.format("%Y-%m").to_string() == period)
                })
        })
        .map(item)
        .collect();
    Ok(serde_json::json!({
        "window_hours": window,
        "open": open, "breached": breached, "closed_in_period": closed,
        "note": "An exception opens when delivery confidence reaches amber-red or red. It is complete with a cause, the decision needed, a named owner and a decide-by date, and closes only when a decision is recorded. Breached means it was not complete within the window counted from when it opened. Owners are shown by role, never by person.",
    }))
}

fn ctx_db(ctx: &AppContext) -> &sea_orm::DatabaseConnection {
    &ctx.db
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/delivery-confidence", put(set_confidence))
        .add("/plans/{pid}/delivery-confidence", get(get_confidence))
        .add("/exceptions", get(list_exceptions))
        .add("/exceptions/{pid}", put(complete_exception))
        .add("/exceptions/{pid}/decision", post(decide_exception))
        .add("/objectives/{pid}/strategic-aim", put(mark_aim))
        .add("/plans/{pid}/primary-aim", put(set_primary))
        .add("/portfolio-map", get(portfolio_map))
}

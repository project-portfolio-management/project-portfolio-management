//! Project escalations: a project referring something upward at a level of its governance escalation path. Rules
//! in [`crate::escalations`]; the path comes from the governance framework in force for the plan
//! ([`super::frameworks::escalation_path`]).
//!
//! * `POST/GET /api/plans/{pid}/escalations`: raise one, list a plan's;
//! * `GET /api/plans/{pid}/escalations/{epid}`: one, with its history;
//! * `POST …/{epid}/acknowledge`, `…/raise-level`, `…/resolve`, `…/withdraw`: move it on;
//! * `GET /api/escalations`: the register across the plans the caller can read.
//!
//! The history (`escalation_events`) is append-only, enforced by a database trigger. Nothing here notifies anyone.

use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::governance::find_item;
use super::{conflict, db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::escalations::{self as rules, Action};
use crate::models::_entities::{escalation_events, escalations, plans};
use crate::models::audit_logs::Model as AuditModel;

fn today() -> NaiveDate {
    chrono::Utc::now().date_naive()
}

pub(crate) fn view(e: &escalations::Model, now: NaiveDate) -> Value {
    json!({
        "pid": e.pid, "plan_pid": e.plan_pid, "title": e.title, "description": e.description,
        "reason_kind": e.reason_kind, "tolerance_dimension": e.tolerance_dimension,
        "level": e.level, "escalated_to": e.escalated_to, "owner_ref": e.owner_ref,
        "status": e.status, "raised_by": e.raised_by, "raised_on": e.raised_on,
        "respond_by": e.respond_by, "outcome": e.outcome, "response": e.response, "resolved_on": e.resolved_on,
        "age_days": rules::age_days(e.raised_on, e.resolved_on.unwrap_or(now)),
        "overdue": rules::is_overdue(&e.status, e.respond_by, now),
    })
}

async fn path_of(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
) -> Result<Vec<(u32, String)>> {
    Ok(super::frameworks::escalation_path(ctx, caller, plan)
        .await?
        .into_iter()
        .map(|e| (e.level, e.body))
        .collect())
}

async fn find_escalation(ctx: &AppContext, plan: Uuid, epid: &str) -> Result<escalations::Model> {
    let pid = Uuid::parse_str(epid).map_err(|_| Error::NotFound)?;
    escalations::Entity::find()
        .filter(escalations::Column::Pid.eq(pid))
        .filter(escalations::Column::PlanPid.eq(plan))
        .filter(escalations::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

async fn event<C: ConnectionTrait>(
    db: &C,
    esc: Uuid,
    event: &str,
    level: i32,
    actor: Option<&str>,
    note: Option<String>,
) -> Result<()> {
    escalation_events::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        escalation_pid: ActiveValue::set(esc),
        event: ActiveValue::set(event.to_string()),
        level: ActiveValue::set(level),
        actor: ActiveValue::set(actor.map(str::to_string)),
        note: ActiveValue::set(note),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// `POST /api/plans/{pid}/escalations` body.
#[derive(Debug, Deserialize)]
struct Input {
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default = "other")]
    reason_kind: String,
    #[serde(default)]
    tolerance_dimension: Option<String>,
    /// The level of the escalation path; the first level of the path (else 1) when absent.
    #[serde(default)]
    level: Option<i32>,
    #[serde(default)]
    escalated_to: Option<String>,
    #[serde(default)]
    owner_ref: Option<String>,
    #[serde(default)]
    respond_by: Option<NaiveDate>,
}

fn other() -> String {
    "other".to_string()
}

fn trimmed(s: Option<&String>) -> Option<String> {
    s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// Tell the people an escalation concerns, in their inbox: the person who raised it and the person it names as
/// responsible, but never the person who just acted. `tell_raiser` is false when the escalation has only just been
/// raised. Best effort: a failure to notify never fails the step, and a recipient who has switched the `escalation`
/// kind off is skipped. In-app only; this service sends no email.
async fn tell(
    ctx: &AppContext,
    e: &escalations::Model,
    actor: Option<&str>,
    tell_raiser: bool,
    what: &str,
) {
    let mut people: Vec<String> = Vec::new();
    if tell_raiser && let Some(sub) = e.raised_by.as_deref() {
        people.push(format!("person:{sub}"));
    }
    people.extend(e.owner_ref.clone());
    people.sort();
    people.dedup();
    for who in people {
        let is_actor = actor.is_some_and(|a| {
            who.strip_prefix("person:") == Some(a) || who.strip_prefix("worker:") == Some(a)
        });
        if is_actor {
            continue;
        }
        crate::models::capabilities::notify(
            &ctx.db,
            &who,
            "plan",
            e.plan_pid,
            "escalation",
            &format!("Escalation \"{}\" {what}", e.title),
        )
        .await
        .ok();
    }
}

/// The fields of a new escalation, already validated and trimmed.
pub(crate) struct New {
    pub title: String,
    pub description: Option<String>,
    pub reason_kind: String,
    pub tolerance_dimension: Option<String>,
    /// The level of the escalation path; the first level (else 1) when absent.
    pub level: Option<i32>,
    pub escalated_to: Option<String>,
    pub owner_ref: Option<String>,
    pub respond_by: Option<NaiveDate>,
}

/// Insert an escalation at a level of the plan's governance path, with its first history entry and an audit record.
pub(crate) async fn insert(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    n: New,
) -> Result<escalations::Model> {
    let path = path_of(ctx, caller, plan).await?;
    let level = n.level.unwrap_or_else(|| {
        path.first()
            .and_then(|(l, _)| i32::try_from(*l).ok())
            .unwrap_or(1)
    });
    let default_to = rules::check_level(level, &path).map_err(|m| unprocessable(&m))?;
    let row = escalations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan),
        title: ActiveValue::set(n.title),
        description: ActiveValue::set(n.description),
        reason_kind: ActiveValue::set(n.reason_kind),
        tolerance_dimension: ActiveValue::set(n.tolerance_dimension),
        level: ActiveValue::set(level),
        escalated_to: ActiveValue::set(n.escalated_to.or(default_to)),
        owner_ref: ActiveValue::set(n.owner_ref),
        status: ActiveValue::set("open".to_string()),
        raised_by: ActiveValue::set(caller.actor().map(str::to_string)),
        raised_on: ActiveValue::set(today()),
        respond_by: ActiveValue::set(n.respond_by),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    event(&ctx.db, row.pid, "raised", level, caller.actor(), None).await?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "escalation_raised",
        caller.actor(),
        Some(json!({ "plan_pid": plan, "level": level, "reason_kind": row.reason_kind })),
    )
    .await;
    tell(ctx, &row, caller.actor(), false, "has been raised with you").await;
    Ok(row)
}

#[debug_handler]
async fn raise(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Input>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let now = today();
    let problems = rules::problems(&rules::Draft {
        title: &b.title,
        description: b.description.as_deref(),
        reason_kind: &b.reason_kind,
        tolerance_dimension: b.tolerance_dimension.as_deref(),
        escalated_to: b.escalated_to.as_deref(),
        owner_ref: b.owner_ref.as_deref(),
        respond_by: b.respond_by,
        today: now,
    });
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let row = insert(
        &ctx,
        &caller,
        plan.pid,
        New {
            title: b.title.trim().to_string(),
            description: trimmed(b.description.as_ref()),
            reason_kind: b.reason_kind.clone(),
            tolerance_dimension: b.tolerance_dimension.clone(),
            level: b.level,
            escalated_to: trimmed(b.escalated_to.as_ref()),
            owner_ref: trimmed(b.owner_ref.as_ref()),
            respond_by: b.respond_by,
        },
    )
    .await?;
    format::json(view(&row, now))
}

/// `GET /api/plans/{pid}/escalations?status=` query.
#[derive(Debug, Deserialize)]
struct StatusQuery {
    #[serde(default)]
    status: Option<String>,
}

fn status_filter(raw: Option<&str>) -> Result<Option<Vec<&'static str>>> {
    Ok(match raw {
        None => None,
        Some("live") => Some(vec!["open", "acknowledged"]),
        Some(s) => Some(vec![
            rules::STATUSES
                .iter()
                .find(|x| **x == s)
                .copied()
                .ok_or_else(|| {
                    unprocessable("status must be open, acknowledged, resolved, withdrawn or live")
                })?,
        ]),
    })
}

#[debug_handler]
async fn list_plan(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<StatusQuery>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let wanted = status_filter(q.status.as_deref())?;
    let mut query = escalations::Entity::find()
        .filter(escalations::Column::PlanPid.eq(plan.pid))
        .filter(escalations::Column::DeletedAt.is_null())
        .order_by_desc(escalations::Column::Id);
    if let Some(w) = wanted {
        query = query.filter(escalations::Column::Status.is_in(w));
    }
    let now = today();
    let rows = query.all(&ctx.db).await.map_err(db_err)?;
    format::json(json!({
        "plan_pid": plan.pid,
        "escalations": rows.iter().map(|e| view(e, now)).collect::<Vec<_>>(),
    }))
}

#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    Path((pid, epid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let e = find_escalation(&ctx, plan.pid, &epid).await?;
    let events = escalation_events::Entity::find()
        .filter(escalation_events::Column::EscalationPid.eq(e.pid))
        .order_by_asc(escalation_events::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = view(&e, today());
    out["events"] = json!(events
        .iter()
        .map(|x| json!({ "event": x.event, "level": x.level, "actor": x.actor, "note": x.note, "at": x.created_at }))
        .collect::<Vec<_>>());
    format::json(out)
}

/// A note on an action.
#[derive(Debug, Deserialize)]
struct NoteBody {
    #[serde(default)]
    note: Option<String>,
}

/// `POST …/resolve` body.
#[derive(Debug, Deserialize)]
struct ResolveBody {
    outcome: String,
    response: String,
}

/// Apply `action` and record the event in one transaction.
#[allow(clippy::too_many_arguments)] // the whole step: who, what, to which row, with what change
async fn step(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    e: escalations::Model,
    action: Action,
    event_name: &str,
    note: Option<String>,
    change: impl FnOnce(&mut escalations::ActiveModel),
) -> Result<Response> {
    let status = rules::apply(&e.status, action).map_err(|m| conflict(&m))?;
    let pid = e.pid;
    let mut a: escalations::ActiveModel = e.into();
    a.status = ActiveValue::set(status.to_string());
    change(&mut a);
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    event(&txn, pid, event_name, row.level, caller.actor(), note).await?;
    txn.commit().await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        pid,
        &format!("escalation_{event_name}"),
        caller.actor(),
        None,
    )
    .await;
    let what = match event_name {
        "acknowledged" => "has been acknowledged",
        "level_raised" => "has been taken up a level",
        "resolved" => "has been resolved",
        _ => "has been withdrawn",
    };
    tell(ctx, &row, caller.actor(), true, what).await;
    format::json(view(&row, today()))
}

#[debug_handler]
async fn acknowledge(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, epid)): Path<(String, String)>,
    Json(b): Json<NoteBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let e = find_escalation(&ctx, plan.pid, &epid).await?;
    step(
        &ctx,
        &caller,
        e,
        Action::Acknowledge,
        "acknowledged",
        trimmed(b.note.as_ref()),
        |_| {},
    )
    .await
}

#[debug_handler]
async fn raise_level(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, epid)): Path<(String, String)>,
    Json(b): Json<NoteBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let e = find_escalation(&ctx, plan.pid, &epid).await?;
    let note = trimmed(b.note.as_ref()).filter(|n| n.chars().count() >= rules::RESPONSE_MIN);
    let Some(note) = note else {
        return Err(unprocessable(&format!(
            "say why it is going up a level (a note of at least {} characters)",
            rules::RESPONSE_MIN
        )));
    };
    let path = path_of(&ctx, &caller, plan.pid).await?;
    let (level, body) = rules::next_level(e.level, &path).map_err(|m| conflict(&m))?;
    step(
        &ctx,
        &caller,
        e,
        Action::RaiseLevel,
        "level_raised",
        Some(note),
        move |a| {
            a.level = ActiveValue::set(level);
            if let Some(body) = body {
                a.escalated_to = ActiveValue::set(Some(body));
            }
        },
    )
    .await
}

#[debug_handler]
async fn resolve(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, epid)): Path<(String, String)>,
    Json(b): Json<ResolveBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let e = find_escalation(&ctx, plan.pid, &epid).await?;
    let mut problems = Vec::new();
    if !rules::OUTCOMES.contains(&b.outcome.as_str()) {
        problems.push(format!("outcome must be one of {:?}", rules::OUTCOMES));
    }
    let response = b.response.trim().to_string();
    if response.chars().count() < rules::RESPONSE_MIN || response.chars().count() > 10_000 {
        problems.push(format!(
            "response is {} to 10000 characters",
            rules::RESPONSE_MIN
        ));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let outcome = b.outcome.clone();
    let on = today();
    step(
        &ctx,
        &caller,
        e,
        Action::Resolve,
        "resolved",
        Some(response.clone()),
        move |a| {
            a.outcome = ActiveValue::set(Some(outcome));
            a.response = ActiveValue::set(Some(response));
            a.resolved_on = ActiveValue::set(Some(on));
        },
    )
    .await
}

#[debug_handler]
async fn withdraw(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, epid)): Path<(String, String)>,
    Json(b): Json<NoteBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let e = find_escalation(&ctx, plan.pid, &epid).await?;
    step(
        &ctx,
        &caller,
        e,
        Action::Withdraw,
        "withdrawn",
        trimmed(b.note.as_ref()),
        |_| {},
    )
    .await
}

/// `GET /api/escalations?status=&plan=&overdue=` query.
#[derive(Debug, Deserialize)]
struct RegisterQuery {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    plan: Option<String>,
    #[serde(default)]
    overdue: Option<bool>,
}

/// `GET /api/escalations`: the register across the plans the caller can read, newest first.
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<RegisterQuery>,
) -> Result<Response> {
    let wanted = status_filter(q.status.as_deref())?;
    let mut query = escalations::Entity::find()
        .filter(escalations::Column::DeletedAt.is_null())
        .order_by_desc(escalations::Column::Id);
    if let Some(w) = wanted {
        query = query.filter(escalations::Column::Status.is_in(w));
    }
    if let Some(p) = q.plan.as_deref() {
        let plan = Uuid::parse_str(p).map_err(|_| unprocessable("plan must be a plan pid"))?;
        query = query.filter(escalations::Column::PlanPid.eq(plan));
    }
    let rows = query.all(&ctx.db).await.map_err(db_err)?;
    let mut plan_pids: Vec<Uuid> = rows.iter().map(|e| e.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plan_pids).await;
    let names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(plan_pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let now = today();
    let mut counts = std::collections::BTreeMap::new();
    let mut overdue = 0_u64;
    let mut out = Vec::new();
    for e in rows.iter().filter(|e| readable.contains(&e.plan_pid)) {
        let late = rules::is_overdue(&e.status, e.respond_by, now);
        if q.overdue == Some(true) && !late {
            continue;
        }
        *counts.entry(e.status.clone()).or_insert(0_u64) += 1;
        overdue += u64::from(late);
        let mut v = view(e, now);
        v["plan_name"] = json!(names.get(&e.plan_pid));
        out.push(v);
    }
    format::json(json!({ "escalations": out, "counts": counts, "overdue": overdue }))
}

/// The escalation routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/escalations", get(register))
        .add("/plans/{pid}/escalations", post(raise))
        .add("/plans/{pid}/escalations", get(list_plan))
        .add("/plans/{pid}/escalations/{epid}", get(get_one))
        .add(
            "/plans/{pid}/escalations/{epid}/acknowledge",
            post(acknowledge),
        )
        .add(
            "/plans/{pid}/escalations/{epid}/raise-level",
            post(raise_level),
        )
        .add("/plans/{pid}/escalations/{epid}/resolve", post(resolve))
        .add("/plans/{pid}/escalations/{epid}/withdraw", post(withdraw))
}

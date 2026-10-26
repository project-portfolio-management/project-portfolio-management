//! Status updates on a cadence (NC-4). Rules in [`crate::status_updates`].
//!
//! * `PUT`/`GET /api/plans/{pid}/status-cadence`: how often the plan expects an update (1 to 92 days). Setting it
//!   restarts the clock;
//! * `POST`/`GET /api/plans/{pid}/status-updates`: post an on-track / at-risk / off-track call with a narrative;
//!   the history, newest first. Append-only, audited;
//! * `GET /api/plans/{pid}/status`: the latest update, the cadence and where the plan stands against it;
//! * `GET /api/status-updates/due`: the plans the caller can read whose update is due, most overdue first;
//! * `POST /api/status-updates/remind`: tell the person who last posted (else who set the cadence) once per due
//!   date. The scheduler ticker does the same on its own when it is switched on.
//! * `GET /api/status-metrics`: the definitions of the numbers a claim may cite (BR-2);
//! * [`pack_section`]: the director pack's view, as at its cut-off.

use std::collections::HashSet;

use chrono::{DateTime, NaiveDate, Utc};
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, sea_query::Expr};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::{db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::meetings::Link;
use crate::models::_entities::{
    plan_status_cadences as cadences, plans, status_updates as updates,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::status_evidence::{self as evidence, Claim, Counts, Source, TaskFact};
use crate::status_updates::{self as rules, Standing};

fn update_view(u: &updates::Model) -> Value {
    json!({
        "pid": u.pid, "plan_pid": u.plan_pid, "status": u.status, "narrative": u.narrative,
        "posted_on": u.posted_on, "posted_by": u.posted_by, "posted_at": u.created_at,
    })
}

/// The plan's open-task numbers as of today (BR-2): "open" is a workflow state that is not done.
async fn counts_of(ctx: &AppContext, plan: Uuid) -> Result<Counts> {
    let workflow = super::workflow::in_force(ctx, plan, "task").await?;
    let rows = crate::models::_entities::tasks::Entity::find()
        .filter(crate::models::_entities::tasks::Column::PlanPid.eq(plan))
        .filter(crate::models::_entities::tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let facts: Vec<TaskFact> = rows
        .iter()
        .map(|t| TaskFact {
            due: t.due_date,
            open: !crate::workflow::is_done(&workflow, &t.status),
            blocked: t.status == "blocked",
        })
        .collect();
    Ok(evidence::counts(&facts, crate::tz::deployment_today()))
}

/// An update with its claims' sources resolved for the reader. Sources are stored as ids; a record that is gone, or
/// not a record of the plan, is `unavailable` with no title; a metric carries its current value (`null` is unknown)
/// and its definition. The numbers are counted once per request, and only when a claim cites one.
async fn view_with_evidence(
    ctx: &AppContext,
    u: &updates::Model,
    counts: &mut Option<Counts>,
) -> Result<Value> {
    let claims: Vec<Claim> = serde_json::from_value(u.claims.clone()).unwrap_or_default();
    let mut shown = Vec::with_capacity(claims.len());
    for c in &claims {
        let mut sources = Vec::with_capacity(c.sources.len());
        for s in &c.sources {
            sources.push(match evidence::parse(s) {
                Ok(Source::Record { kind, pid }) => {
                    let link = Link { kind: kind.clone(), pid };
                    match super::meetings::describe(&ctx.db, u.plan_pid, &link).await? {
                        Some(title) => json!({ "source": s, "state": "ok", "kind": kind, "pid": pid, "title": title }),
                        None => json!({ "source": s, "state": "unavailable" }),
                    }
                }
                Ok(Source::Metric(key)) => {
                    if counts.is_none() {
                        *counts = Some(counts_of(ctx, u.plan_pid).await?);
                    }
                    let value = counts.as_ref().and_then(|c| c.value(&key));
                    let def = evidence::METRICS.iter().find(|m| m.key == key);
                    json!({
                        "source": s, "state": "ok", "kind": "metric", "key": key,
                        "value": value, "known": value.is_some(),
                        "definition": def.map(|d| d.definition),
                    })
                }
                Err(_) => json!({ "source": s, "state": "unavailable" }),
            });
        }
        shown.push(json!({ "text": c.text, "sources": sources }));
    }
    let mut v = update_view(u);
    v["claims"] = json!(shown);
    Ok(v)
}

/// `GET /api/status-metrics`: how each number a claim may cite is counted. A number that cannot be known is `null`
/// in a claim's source, never zero.
#[debug_handler]
async fn metrics() -> Result<Response> {
    format::json(json!({
        "due_soon_days": evidence::DUE_SOON_DAYS,
        "metrics": evidence::METRICS.iter().map(|m| json!({
            "key": m.key, "label": m.label, "definition": m.definition,
        })).collect::<Vec<_>>(),
        "note": "A status is chosen by a person; the service computes no health score.",
    }))
}

async fn latest_before<C: ConnectionTrait>(
    db: &C,
    plan: Uuid,
    at_or_before: Option<DateTime<Utc>>,
) -> Result<Option<updates::Model>> {
    let mut q = updates::Entity::find().filter(updates::Column::PlanPid.eq(plan));
    if let Some(t) = at_or_before {
        q = q.filter(updates::Column::CreatedAt.lte(t));
    }
    q.order_by_desc(updates::Column::Id)
        .one(db)
        .await
        .map_err(db_err)
}

fn standing_of(c: &cadences::Model, last: Option<&updates::Model>, today: NaiveDate) -> Standing {
    rules::standing(c.cadence_days, c.set_on, last.map(|u| u.posted_on), today)
}

/// `PUT /api/plans/{pid}/status-cadence` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CadenceBody {
    /// How often an update is expected, in days (1 to 92).
    cadence_days: i32,
}

/// `PUT /api/plans/{pid}/status-cadence`.
#[debug_handler]
async fn put_cadence(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(body): Json<CadenceBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if !rules::cadence_ok(body.cadence_days) {
        return Err(unprocessable(&format!(
            "cadence_days is {} to {}",
            rules::MIN_CADENCE_DAYS,
            rules::MAX_CADENCE_DAYS
        )));
    }
    let today = crate::tz::deployment_today();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let existing = cadences::Entity::find()
        .filter(cadences::Column::PlanPid.eq(plan.pid))
        .one(&txn)
        .await
        .map_err(db_err)?;
    let before = existing.as_ref().map(|e| e.cadence_days);
    let who = caller.actor().map(str::to_string);
    let row = if let Some(e) = existing {
        let mut a: cadences::ActiveModel = e.into();
        a.cadence_days = ActiveValue::set(body.cadence_days);
        a.set_on = ActiveValue::set(today);
        a.set_by = ActiveValue::set(who);
        a.reminded_for = ActiveValue::set(None);
        a.update(&txn).await.map_err(db_err)?
    } else {
        cadences::ActiveModel {
            plan_pid: ActiveValue::set(plan.pid),
            cadence_days: ActiveValue::set(body.cadence_days),
            set_on: ActiveValue::set(today),
            set_by: ActiveValue::set(who),
            reminded_for: ActiveValue::set(None),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?
    };
    AuditModel::record(
        &txn,
        plan.pid,
        "status_cadence_set",
        caller.actor(),
        Some(json!({ "before": before, "after": row.cadence_days })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    let last = latest_before(&ctx.db, plan.pid, None).await?;
    format::json(cadence_view(&row, last.as_ref(), today))
}

fn cadence_view(c: &cadences::Model, last: Option<&updates::Model>, today: NaiveDate) -> Value {
    json!({
        "plan_pid": c.plan_pid, "cadence_days": c.cadence_days, "set_on": c.set_on, "set_by": c.set_by,
        "standing": standing_of(c, last, today),
    })
}

/// `GET /api/plans/{pid}/status-cadence` — how often the plan expects an update and where it stands against that today: the date the next is due, whether it is due or overdue, and by how many days; `404` when no cadence is set.
#[debug_handler]
async fn get_cadence(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = cadences::Entity::find()
        .filter(cadences::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let last = latest_before(&ctx.db, plan.pid, None).await?;
    format::json(cadence_view(
        &row,
        last.as_ref(),
        crate::tz::deployment_today(),
    ))
}

/// `POST /api/plans/{pid}/status-updates` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdateBody {
    /// `on_track`, `at_risk` or `off_track`.
    status: String,
    /// What is going on, in words.
    narrative: String,
    /// Optional claims, each a sentence with sources (`task:<pid>`, `risk:<pid>`, `issue:<pid>`, `milestone:<pid>`,
    /// `metric:<key>`). Every source is checked on save.
    #[serde(default)]
    claims: Vec<Claim>,
}

/// `POST /api/plans/{pid}/status-updates`.
#[debug_handler]
async fn post_update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(body): Json<UpdateBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut found = rules::update_problems(&body.status, &body.narrative);
    found.extend(evidence::problems(&body.claims));
    if !found.is_empty() {
        return Err(unprocessable(&found.join("; ")));
    }
    for (i, c) in body.claims.iter().enumerate() {
        for s in &c.sources {
            if let Ok(Source::Record { kind, pid }) = evidence::parse(s)
                && super::meetings::describe(&ctx.db, plan.pid, &Link { kind, pid })
                    .await?
                    .is_none()
            {
                return Err(unprocessable(&format!(
                    "claim {i}: {s} is not a record of this plan"
                )));
            }
        }
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = updates::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        status: ActiveValue::set(body.status.clone()),
        narrative: ActiveValue::set(body.narrative.trim().to_string()),
        posted_on: ActiveValue::set(crate::tz::deployment_today()),
        posted_by: ActiveValue::set(caller.actor().map(str::to_string)),
        claims: ActiveValue::set(json!(body.claims)),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan.pid,
        "status_update_posted",
        caller.actor(),
        Some(json!({ "update": row.pid, "status": row.status, "claims": body.claims.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(view_with_evidence(&ctx, &row, &mut None).await?)
}

/// `GET /api/plans/{pid}/status-updates`: the history, newest first.
#[debug_handler]
async fn list_updates(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = updates::Entity::find()
        .filter(updates::Column::PlanPid.eq(plan.pid))
        .order_by_desc(updates::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut counts = None;
    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        out.push(view_with_evidence(&ctx, r, &mut counts).await?);
    }
    format::json(out)
}

/// `GET /api/plans/{pid}/status` — the latest update, the plan's cadence and where it stands against it (today, in the
/// deployment's time zone). A plan with no cadence has no due date; one with no update says so.
#[debug_handler]
async fn get_status(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let today = crate::tz::deployment_today();
    let last = latest_before(&ctx.db, plan.pid, None).await?;
    let cadence = cadences::Entity::find()
        .filter(cadences::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(json!({
        "plan_pid": plan.pid,
        "as_of": today,
        "latest": match last.as_ref() {
            Some(u) => Some(view_with_evidence(&ctx, u, &mut None).await?),
            None => None,
        },
        "cadence_days": cadence.as_ref().map(|c| c.cadence_days),
        "standing": cadence.as_ref().map(|c| standing_of(c, last.as_ref(), today)),
    }))
}

/// `GET /api/status-updates/due` — the plans the caller can read whose next update is due, most overdue first.
#[debug_handler]
async fn due(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let today = crate::tz::deployment_today();
    let all = cadences::Entity::find()
        .order_by_asc(cadences::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let pids: Vec<Uuid> = all.iter().map(|c| c.plan_pid).collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &pids).await;
    let names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let mut out = Vec::new();
    for c in all.iter().filter(|c| readable.contains(&c.plan_pid)) {
        let last = latest_before(&ctx.db, c.plan_pid, None).await?;
        let s = standing_of(c, last.as_ref(), today);
        if s.due {
            out.push((
                s.days_overdue,
                json!({
                    "plan_pid": c.plan_pid, "plan_name": names.get(&c.plan_pid),
                    "cadence_days": c.cadence_days, "standing": s,
                    "latest_status": last.as_ref().map(|u| u.status.clone()),
                    "last_update_on": last.as_ref().map(|u| u.posted_on),
                }),
            ));
        }
    }
    out.sort_by_key(|(late, _)| std::cmp::Reverse(*late));
    format::json(out.into_iter().map(|(_, v)| v).collect::<Vec<_>>())
}

/// What a reminder sweep did.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Reminded {
    /// Reminders sent.
    pub sent: usize,
    /// Due plans with nobody to tell (no one posted and the cadence was set by no addressable person).
    pub unaddressed: usize,
}

/// Remind, once per due date, the person who last posted (else who set the cadence) that an update is due. The
/// reminder is claimed with a conditional update before it is sent, so two sweepers send one.
///
/// # Errors
///
/// When a database statement fails.
pub async fn remind_due(ctx: &AppContext) -> Result<Reminded> {
    let today = crate::tz::deployment_today();
    let mut out = Reminded::default();
    let all = cadences::Entity::find()
        .order_by_asc(cadences::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let live: HashSet<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .filter(plans::Column::Pid.is_in(all.iter().map(|c| c.plan_pid).collect::<Vec<_>>()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    for c in all.iter().filter(|c| live.contains(&c.plan_pid)) {
        let last = latest_before(&ctx.db, c.plan_pid, None).await?;
        let s = standing_of(c, last.as_ref(), today);
        if !s.due || c.reminded_for.is_some_and(|r| r >= s.due_on) {
            continue;
        }
        let who = last
            .as_ref()
            .and_then(|u| u.posted_by.clone())
            .or_else(|| c.set_by.clone())
            .map(|sub| format!("person:{sub}"))
            .filter(|r| crate::notification_prefs::recipient_sub(r).is_some());
        let txn = ctx.db.begin().await.map_err(db_err)?;
        let claimed = cadences::Entity::update_many()
            .col_expr(cadences::Column::RemindedFor, Expr::value(s.due_on))
            .filter(cadences::Column::Id.eq(c.id))
            .filter(
                sea_orm::Condition::any()
                    .add(cadences::Column::RemindedFor.is_null())
                    .add(cadences::Column::RemindedFor.lt(s.due_on)),
            )
            .exec(&txn)
            .await
            .map_err(db_err)?;
        if claimed.rows_affected != 1 {
            continue;
        }
        AuditModel::record(
            &txn,
            c.plan_pid,
            "status_update_reminded",
            None,
            Some(json!({ "due_on": s.due_on, "to": who })),
        )
        .await
        .map_err(super::audit_failed)?;
        txn.commit().await.map_err(db_err)?;
        let Some(who) = who else {
            out.unaddressed += 1;
            continue;
        };
        let message = format!(
            "A status update is due{} for this plan (every {} days).",
            if s.overdue {
                format!(" and {} day(s) overdue", s.days_overdue)
            } else {
                String::new()
            },
            c.cadence_days
        );
        crate::models::capabilities::notify(
            &ctx.db,
            &who,
            "plan",
            c.plan_pid,
            "status_update_due",
            &message,
        )
        .await?;
        out.sent += 1;
    }
    Ok(out)
}

/// `POST /api/status-updates/remind`.
#[debug_handler]
async fn remind(State(ctx): State<AppContext>) -> Result<Response> {
    let r = remind_due(&ctx).await?;
    format::json(json!({ "sent": r.sent, "unaddressed": r.unaddressed }))
}

/// The director pack's view: for each included plan with a cadence, its latest update **as at the cut-off** and
/// where it stood against its cadence on the cut-off's date. A plan whose cadence was set after the cut-off is not in
/// it. `without_cadence` counts the included plans that have none.
///
/// # Errors
///
/// When a database read fails.
#[expect(
    clippy::implicit_hasher,
    reason = "the pack passes a plain `HashSet`; public only so the request test can call it"
)]
pub async fn pack_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    cut_off: DateTime<Utc>,
) -> Result<Value> {
    let day = cut_off.date_naive();
    let all = cadences::Entity::find()
        .order_by_asc(cadences::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let with: Vec<&cadences::Model> = all
        .iter()
        .filter(|c| include.contains(&c.plan_pid) && c.set_on <= day)
        .collect();
    let names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(with.iter().map(|c| c.plan_pid).collect::<Vec<_>>()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let mut rows = Vec::new();
    for c in &with {
        let last = latest_before(&ctx.db, c.plan_pid, Some(cut_off)).await?;
        rows.push(json!({
            "plan_pid": c.plan_pid, "plan_name": names.get(&c.plan_pid),
            "cadence_days": c.cadence_days,
            "latest": last.as_ref().map(|u| json!({
                "status": u.status, "narrative": u.narrative, "posted_on": u.posted_on })),
            "standing": rules::standing(c.cadence_days, c.set_on, last.as_ref().map(|u| u.posted_on), day),
        }));
    }
    rows.sort_by(|a, b| {
        let key = |v: &Value| {
            (
                std::cmp::Reverse(v["standing"]["days_overdue"].as_i64().unwrap_or(0)),
                v["plan_name"].as_str().unwrap_or_default().to_string(),
            )
        };
        key(a).cmp(&key(b))
    });
    let overdue = rows
        .iter()
        .filter(|r| r["standing"]["overdue"] == true)
        .count();
    Ok(json!({
        "as_at": cut_off,
        "plans": rows,
        "overdue": overdue,
        "without_cadence": include.len().saturating_sub(with.len()),
        "basis": "The latest update posted on or before the cut-off, and the cadence as it is now.",
    }))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/status-cadence", put(put_cadence))
        .add("/plans/{pid}/status-cadence", get(get_cadence))
        .add("/plans/{pid}/status-updates", post(post_update))
        .add("/plans/{pid}/status-updates", get(list_updates))
        .add("/plans/{pid}/status", get(get_status))
        .add("/status-metrics", get(metrics))
        .add("/status-updates/due", get(due))
        .add("/status-updates/remind", post(remind))
}

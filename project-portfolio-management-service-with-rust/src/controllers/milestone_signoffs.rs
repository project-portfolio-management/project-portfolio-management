//! Milestones met only with partner sign-off (EX-4). The rules are [`crate::milestone_signoff`].
//!
//! * `PUT /api/plans/{pid}/milestones/{m_pid}/signoff-required` sets the organizations whose sign-off a
//!   milestone needs (audited);
//! * `POST …/signoffs` records an organization's sign-off, or withdraws it with a reason (append-only,
//!   audited). Only a caller of that organization (the `organization` attribute), or an administrator,
//!   signs or withdraws for it;
//! * `GET /api/milestone-signoffs` is the portfolio view: delivered, met, and the delivered-not-met gap.
//!
//! Delivered keeps its meaning and its date; met is separate. Both are derived on read.

use super::db_err;
use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{NaiveDate, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::milestone_signoff::{self as rules, Assessment, Required};
use crate::models::_entities::{
    milestone_signoff_requirements as reqs, milestone_signoffs as events, milestones, organizations,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add(
            "/plans/{pid}/milestones/{m_pid}/signoff-required",
            put(set_required),
        )
        .add("/plans/{pid}/milestones/{m_pid}/signoffs", post(act))
        .add("/milestone-signoffs", get(portfolio))
}

/// Each milestone's assessment, given when it was delivered (milestones with no requirement are absent).
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn assessments(
    ctx: &AppContext,
    delivered: &HashMap<Uuid, Option<NaiveDate>>,
) -> Result<HashMap<Uuid, Assessment>> {
    let pids: Vec<Uuid> = delivered.keys().copied().collect();
    if pids.is_empty() {
        return Ok(HashMap::new());
    }
    let required = reqs::Entity::find()
        .filter(reqs::Column::MilestonePid.is_in(pids.clone()))
        .order_by_asc(reqs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if required.is_empty() {
        return Ok(HashMap::new());
    }
    let orgs: HashMap<Uuid, String> = organizations::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|o| (o.pid, o.name))
        .collect();
    // The sign-off now in force per milestone and organization: the latest event, if it is a sign.
    let mut in_force: HashMap<(Uuid, Uuid), Option<NaiveDate>> = HashMap::new();
    for e in events::Entity::find()
        .filter(events::Column::MilestonePid.is_in(pids))
        .order_by_asc(events::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        in_force.insert(
            (e.milestone_pid, e.organization_pid),
            (e.action == "sign").then_some(e.acted_on),
        );
    }
    let mut by: HashMap<Uuid, Vec<Required>> = HashMap::new();
    for r in required {
        by.entry(r.milestone_pid).or_default().push(Required {
            organization: r.organization_pid,
            name: orgs.get(&r.organization_pid).cloned().unwrap_or_default(),
            signed_on: in_force
                .get(&(r.milestone_pid, r.organization_pid))
                .copied()
                .flatten(),
        });
    }
    Ok(by
        .into_iter()
        .map(|(m, req)| {
            let d = delivered.get(&m).copied().flatten();
            (m, rules::assess(&req, d))
        })
        .collect())
}

/// An assessment as JSON.
pub(crate) fn assessment_json(a: &Assessment) -> Value {
    json!({
        "state": a.state, "delivered_on": a.delivered_on, "met_on": a.met_on,
        "outstanding": a.outstanding,
    })
}

fn is_admin(caller: &MaybeAuthUser) -> bool {
    caller.holds_or_open(|c| crate::auth::has_attr(c, "access", "admin"))
}

#[derive(Debug, Deserialize)]
struct Required_ {
    organization_pids: Vec<Uuid>,
}

async fn the_milestone(ctx: &AppContext, pid: &str, m_pid: &str) -> Result<milestones::Model> {
    let plan = super::governance::find_item(ctx, pid).await?;
    let m_pid = Uuid::parse_str(m_pid).map_err(|_| Error::NotFound)?;
    let m = vis_models::find_milestone(&ctx.db, m_pid).await?;
    if m.plan_pid == plan.pid {
        Ok(m)
    } else {
        Err(Error::NotFound)
    }
}

/// `PUT /api/plans/{pid}/milestones/{m_pid}/signoff-required`
#[debug_handler]
async fn set_required(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    Json(b): Json<Required_>,
) -> Result<Response> {
    let m = the_milestone(&ctx, &pid, &m_pid).await?;
    let wanted: Vec<Uuid> = b
        .organization_pids
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if wanted.len() > 20 {
        return Err(unprocessable(
            "a milestone may require at most 20 organizations",
        ));
    }
    let found = organizations::Entity::find()
        .filter(organizations::Column::Pid.is_in(wanted.clone()))
        .filter(organizations::Column::Active.eq(true))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if found.len() != wanted.len() {
        return Err(unprocessable("every organization must exist and be active"));
    }
    reqs::Entity::delete_many()
        .filter(reqs::Column::MilestonePid.eq(m.pid))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    for o in &wanted {
        reqs::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            milestone_pid: ActiveValue::set(m.pid),
            organization_pid: ActiveValue::set(*o),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?;
    }
    AuditModel::record_or_log(
        &ctx.db,
        m.pid,
        "milestone_signoff_required_set",
        caller.actor(),
        Some(json!({ "organizations": wanted })),
    )
    .await;
    format::json(json!({ "milestone_pid": m.pid, "organization_pids": wanted }))
}

#[derive(Debug, Deserialize)]
struct Act {
    organization_pid: Uuid,
    /// `sign` or `withdraw`.
    action: String,
    #[serde(default)]
    reason: Option<String>,
}

/// `POST /api/plans/{pid}/milestones/{m_pid}/signoffs`
#[debug_handler]
async fn act(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    Json(b): Json<Act>,
) -> Result<Response> {
    let m = the_milestone(&ctx, &pid, &m_pid).await?;
    if !["sign", "withdraw"].contains(&b.action.as_str()) {
        return Err(unprocessable("action must be sign or withdraw"));
    }
    let required = reqs::Entity::find()
        .filter(reqs::Column::MilestonePid.eq(m.pid))
        .filter(reqs::Column::OrganizationPid.eq(b.organization_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    if required.is_none() {
        return Err(unprocessable(
            "that organization is not required to sign this milestone",
        ));
    }
    if !is_admin(&caller)
        && !super::organizations::caller_organizations(&caller).contains(&b.organization_pid)
    {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new(
                "forbidden",
                "only a person of that organization, or an administrator, signs for it",
            ),
        ));
    }
    let reason = b
        .reason
        .as_deref()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(str::to_string);
    if b.action == "withdraw"
        && reason
            .as_ref()
            .is_none_or(|r| r.chars().count() < 5 || r.chars().count() > 500)
    {
        return Err(unprocessable(
            "a withdrawal needs a reason of 5 to 500 characters",
        ));
    }
    let row = events::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        milestone_pid: ActiveValue::set(m.pid),
        organization_pid: ActiveValue::set(b.organization_pid),
        action: ActiveValue::set(b.action.clone()),
        reason: ActiveValue::set(reason.clone()),
        acted_on: ActiveValue::set(Utc::now().date_naive()),
        signed_by: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        m.pid,
        if b.action == "sign" {
            "milestone_signed_off"
        } else {
            "milestone_signoff_withdrawn"
        },
        caller.actor(),
        Some(json!({ "organization": b.organization_pid, "reason": reason })),
    )
    .await;
    format::json(json!({ "pid": row.pid, "action": row.action, "acted_on": row.acted_on }))
}

/// The portfolio view over `include` (the plans that may be read): every milestone that requires sign-off.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn summary(ctx: &AppContext, include: &HashSet<Uuid>) -> Result<Value> {
    let pids: Vec<Uuid> = reqs::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| r.milestone_pid)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let rows: Vec<milestones::Model> = if pids.is_empty() {
        Vec::new()
    } else {
        milestones::Entity::find()
            .filter(milestones::Column::Pid.is_in(pids))
            .filter(milestones::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .filter(|m| include.contains(&m.plan_pid))
            .collect()
    };
    let today = Utc::now().date_naive();
    let derived = super::milestones::derive(ctx, &rows, today, 14, 5, false).await?;
    let mut list: Vec<Value> = Vec::new();
    let mut states: Vec<rules::State> = Vec::new();
    for (_, _, v) in &derived.json {
        if v["signoff"].is_null() {
            continue;
        }
        if let Ok(st) = serde_json::from_value::<rules::State>(v["signoff"]["state"].clone()) {
            states.push(st);
        }
        list.push(json!({
            "milestone_pid": v["pid"], "plan_pid": v["plan_pid"], "name": v["name"], "due": v["due"],
            "signoff": v["signoff"],
        }));
    }
    Ok(json!({
        "totals": rules::totals(&states),
        "milestones": list,
        "note": "Delivered is the delivery organization saying the milestone is done (marked done, or its linked tasks are all done) and keeps its date. Met is delivered with every required organization's sign-off in force, and has its own date: the later of the delivery and the last sign-off. A milestone with a sign-off outstanding is delivered-not-met, and the gap is listed. The met share is of those delivered. A sign-off withdrawn is outstanding again. The state is as now, not as at a past date.",
    }))
}

/// The pack's section.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext, include: &HashSet<Uuid>) -> Result<Value> {
    summary(ctx, include).await
}

/// `GET /api/milestone-signoffs` — for the plans you can read.
#[debug_handler]
async fn portfolio(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let all: Vec<Uuid> = milestones::Entity::find()
        .filter(milestones::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|m| m.plan_pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    format::json(summary(&ctx, &readable).await?)
}

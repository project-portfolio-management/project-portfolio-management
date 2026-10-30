//! Skill pools, capacity commitments and the capacity heatmap by pool and month (DP-10). Rules in
//! [`crate::capacity_heatmap`].
//!
//! **Partner organizations (EX-3) are not built.** A commitment carries the partner's name as a label;
//! none means the organization's own. Because there is no organization attribute to base a policy on,
//! the partners' names are shown only when `PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS` is
//! `1`/`true`; by default partner supply is shown combined and the number of partners is stated.

use super::refuse;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{Datelike, NaiveDate};
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::capacity_heatmap::{self as rules, Inputs};
use crate::models::_entities::{
    allocations, capacity_commitments as commitments, organizations, peer_demand_sends, plans,
    resource_roles, skill_pool_members as members, skill_pools as pools,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::portfolio_resources::{self as pr, Grain, GroupBy, Person};

fn db_err(e: &sea_orm::DbErr) -> Error {
    let t = e.to_string();
    if t.contains("skill_pools_peer_pool_ref") {
        return refuse(
            StatusCode::CONFLICT,
            "conflict",
            "another pool is already linked to that workforce-service pool",
        );
    }
    if t.contains("spm_once") {
        return refuse(
            StatusCode::CONFLICT,
            "conflict",
            "a role or skill is in at most one pool; it is already in another",
        );
    }
    if t.contains("skill_pools_name") {
        return refuse(
            StatusCode::CONFLICT,
            "conflict",
            "a pool with that name exists",
        );
    }
    Error::string(&t)
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/skill-pools", get(list_pools))
        .add("/skill-pools", post(create_pool))
        .add("/skill-pools/{pid}", put(update_pool))
        .add("/skill-pools/{pid}/peer-link", put(set_peer_link))
        .add("/capacity-commitments", get(list_commitments))
        .add("/capacity-commitments", put(set_commitment))
        .add("/capacity-heatmap", get(live))
}

fn show_partners() -> bool {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS")
        .is_ok_and(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "yes"))
}

// ---- pools ----------------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewPool {
    name: String,
    #[serde(default)]
    roles: Vec<String>,
    #[serde(default)]
    skills: Vec<String>,
}

async fn set_members(
    tx: &sea_orm::DatabaseTransaction,
    pool: Uuid,
    roles: &[String],
    skills: &[String],
) -> Result<()> {
    members::Entity::delete_many()
        .filter(members::Column::PoolPid.eq(pool))
        .exec(tx)
        .await
        .map_err(|e| db_err(&e))?;
    for (kind, values) in [("role", roles), ("skill", skills)] {
        let mut seen = BTreeSet::new();
        for v in values {
            let v = v.trim();
            if v.is_empty() || !seen.insert(v.to_lowercase()) {
                continue;
            }
            members::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                pool_pid: ActiveValue::set(pool),
                kind: ActiveValue::set(kind.to_string()),
                value: ActiveValue::set(v.to_string()),
                ..Default::default()
            }
            .insert(tx)
            .await
            .map_err(|e| db_err(&e))?;
        }
    }
    Ok(())
}

fn pool_json(p: &pools::Model, ms: &[members::Model]) -> serde_json::Value {
    let of = |k: &str| -> Vec<&str> {
        let mut v: Vec<&str> = ms
            .iter()
            .filter(|m| m.pool_pid == p.pid && m.kind == k)
            .map(|m| m.value.as_str())
            .collect();
        v.sort_unstable();
        v
    };
    serde_json::json!({ "pid": p.pid, "name": p.name, "active": p.active, "roles": of("role"), "skills": of("skill"), "peer_pool_ref": p.peer_pool_ref })
}

/// `POST /api/skill-pools` — a pool, with the roles and skills it groups.
#[debug_handler]
async fn create_pool(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewPool>,
) -> Result<Response> {
    let name = b.name.trim().to_string();
    if name.chars().count() < 2 || name.chars().count() > 120 {
        return Err(unprocessable("name must be 2 to 120 characters"));
    }
    let tx = ctx.db.begin().await.map_err(|e| db_err(&e))?;
    let row = pools::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(name),
        active: ActiveValue::set(true),
        ..Default::default()
    }
    .insert(&tx)
    .await
    .map_err(|e| db_err(&e))?;
    set_members(&tx, row.pid, &b.roles, &b.skills).await?;
    AuditModel::record(&tx, row.pid, "skill_pool_created", caller.actor(), None)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let ms = members::Entity::find()
        .filter(members::Column::PoolPid.eq(row.pid))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(pool_json(&row, &ms))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdatePool {
    #[serde(default)]
    active: Option<bool>,
    #[serde(default)]
    roles: Option<Vec<String>>,
    #[serde(default)]
    skills: Option<Vec<String>>,
}

/// `PUT /api/skill-pools/{pid}` — retire or restore a pool, or replace its roles and skills.
#[debug_handler]
async fn update_pool(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<Uuid>,
    Json(b): Json<UpdatePool>,
) -> Result<Response> {
    let row = pools::Entity::find()
        .filter(pools::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let tx = ctx.db.begin().await.map_err(|e| db_err(&e))?;
    let mut a: pools::ActiveModel = row.into();
    if let Some(active) = b.active {
        a.active = ActiveValue::set(active);
    }
    let row = a.update(&tx).await.map_err(|e| db_err(&e))?;
    if b.roles.is_some() || b.skills.is_some() {
        let current = members::Entity::find()
            .filter(members::Column::PoolPid.eq(pid))
            .all(&tx)
            .await
            .map_err(|e| db_err(&e))?;
        let keep = |k: &str| -> Vec<String> {
            current
                .iter()
                .filter(|m| m.kind == k)
                .map(|m| m.value.clone())
                .collect()
        };
        let roles = b.roles.clone().unwrap_or_else(|| keep("role"));
        let skills = b.skills.clone().unwrap_or_else(|| keep("skill"));
        set_members(&tx, pid, &roles, &skills).await?;
    }
    AuditModel::record(&tx, pid, "skill_pool_updated", caller.actor(), None)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let ms = members::Entity::find()
        .filter(members::Column::PoolPid.eq(pid))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(pool_json(&row, &ms))
}

/// `GET /api/skill-pools`.
#[debug_handler]
async fn list_pools(State(ctx): State<AppContext>) -> Result<Response> {
    let mut ps = pools::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    ps.sort_by_key(|p| p.name.to_lowercase());
    let ms = members::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(
        serde_json::json!({ "skill_pools": ps.iter().map(|p| pool_json(p, &ms)).collect::<Vec<_>>() }),
    )
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PeerLink {
    /// The workforce service's id for the pool that means the same skills (a lower-case hyphenated UUID);
    /// `null` removes the link.
    peer_pool_ref: Option<String>,
}

/// `PUT /api/skill-pools/{pid}/peer-link` — link the pool to the workforce service's pool, or remove the link
/// (FB-1). A link is set by a person, never matched by name; one PPM pool per WPM pool. While linked, the pool's
/// supply on the heatmap is the workforce service's and its demand commitments are sent there; unlinked, it is as
/// before. Audited with no figure. Removing or changing a link withdraws every figure sent to the old WPM pool
/// (`fte_centi: 0` per programme and month, best effort, FB-5), listed in `peer_demand_withdrawals`.
#[debug_handler]
async fn set_peer_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: axum::http::HeaderMap,
    Path(pid): Path<Uuid>,
    Json(b): Json<PeerLink>,
) -> Result<Response> {
    let peer = b.peer_pool_ref.map(|t| t.trim().to_string());
    if let Some(t) = &peer
        && !crate::wpm_bridge::peer_pool_ref_ok(t)
    {
        return Err(unprocessable(
            "peer_pool_ref must be the workforce service's pool id: a lower-case hyphenated UUID, or null",
        ));
    }
    let row = pools::Entity::find()
        .filter(pools::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let action = if peer.is_some() {
        "skill_pool_peer_linked"
    } else {
        "skill_pool_peer_unlinked"
    };
    let before = row
        .peer_pool_ref
        .clone()
        .filter(|o| Some(o) != peer.as_ref());
    let tx = ctx.db.begin().await.map_err(|e| db_err(&e))?;
    let mut a: pools::ActiveModel = row.into();
    a.peer_pool_ref = ActiveValue::set(peer);
    let row = a.update(&tx).await.map_err(|e| db_err(&e))?;
    AuditModel::record(&tx, pid, action, caller.actor(), None)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let ms = members::Entity::find()
        .filter(members::Column::PoolPid.eq(pid))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut out = pool_json(&row, &ms);
    if let Some(old) = before {
        out["peer_demand_withdrawals"] = withdraw_on_unlink(&ctx, &headers, pid, old).await?;
    }
    format::json(out)
}

/// The most withdrawals one unlink sends (FB-5); beyond it nothing more is sent and the rows keep their outcome.
const WITHDRAWALS_MAX: u64 = 2_400;

/// When a pool's link to the workforce service is removed or changed (FB-5), withdraw from the WPM pool it was linked
/// to every figure PPM sent there: for each programme and month whose latest send carried a figure above zero, send
/// `fte_centi: 0` with a new event id and the time of the unlink. Best effort, in order, after the unlink has been
/// stored and without holding it up; each one is recorded in `peer_demand_sends` like any other send. With no base
/// URL or no token, each is recorded as not sent and the figure stays in WPM.
async fn withdraw_on_unlink(
    ctx: &AppContext,
    headers: &axum::http::HeaderMap,
    pool: Uuid,
    old: String,
) -> Result<serde_json::Value> {
    use crate::wpm_bridge::{self as bridge, DemandPlan};
    let rows = peer_demand_sends::Entity::find()
        .filter(peer_demand_sends::Column::PoolPid.eq(pool))
        .filter(peer_demand_sends::Column::PeerPoolRef.eq(&old))
        .filter(peer_demand_sends::Column::FteCenti.gt(0))
        .order_by_asc(peer_demand_sends::Column::Month)
        .order_by_asc(peer_demand_sends::Column::PlanPid)
        .limit(WITHDRAWALS_MAX)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let unlinked_at = chrono::Utc::now();
    let base = bridge::base_url();
    let bearer = bridge::bearer_of(headers);
    let decided = bridge::plan_demand(0, base.is_some(), bearer.is_some());
    let mut answer = Vec::with_capacity(rows.len());
    let mut sends = Vec::new();
    for r in &rows {
        let event = bridge::peer_event_id(pool, r.plan_pid, r.month, unlinked_at);
        let (outcome, detail, centi) = match &decided {
            DemandPlan::Send(c) => ("started", bridge::started_detail(*c), Some(*c)),
            DemandPlan::Skip(o, d) => (*o, *d, None),
        };
        record_send(
            &ctx.db,
            pool,
            r.plan_pid,
            r.month,
            &old,
            centi,
            &event,
            unlinked_at,
            outcome,
            detail,
        )
        .await?;
        answer.push(serde_json::json!({
            "plan_pid": r.plan_pid, "month": r.month.format("%Y-%m").to_string(),
            "outcome": outcome, "detail": detail, "fte_centi": centi, "peer_event_id": event,
        }));
        if let (DemandPlan::Send(c), Some(base), Some(bearer)) = (&decided, &base, &bearer) {
            sends.push(bridge::DemandSend {
                base: base.clone(),
                bearer: bearer.clone(),
                peer_pool_ref: old.clone(),
                plan_pid: r.plan_pid,
                month: r.month,
                fte_centi: *c,
                peer_event_id: event,
                sent_at: unlinked_at,
            });
        }
    }
    if !sends.is_empty() {
        let db = ctx.db.clone();
        tokio::spawn(async move {
            for send in sends {
                let (outcome, detail) = bridge::send_demand(&send).await;
                if let Err(e) = record_send(
                    &db,
                    pool,
                    send.plan_pid,
                    send.month,
                    &send.peer_pool_ref,
                    Some(send.fte_centi),
                    &send.peer_event_id,
                    send.sent_at,
                    outcome,
                    &detail,
                )
                .await
                {
                    tracing::warn!(error = %e, "could not record the outcome of a withdrawal sent to the workforce service");
                }
            }
        });
    }
    Ok(serde_json::Value::Array(answer))
}

// ---- commitments ----------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CommitBody {
    pool_pid: Uuid,
    plan_pid: Uuid,
    #[serde(default)]
    organization: Option<String>,
    /// An organization record (EX-3); its name then becomes the label.
    #[serde(default)]
    organization_pid: Option<Uuid>,
    kind: String,
    /// `YYYY-MM`.
    month: String,
    committed_fte: f64,
    #[serde(default)]
    delivered_fte: Option<f64>,
    #[serde(default)]
    note: Option<String>,
}

fn milli(f: f64) -> Option<i32> {
    if !f.is_finite() || !(0.0..=1000.0).contains(&f) {
        return None;
    }
    #[expect(clippy::cast_possible_truncation)]
    Some((f * 1000.0).round() as i32)
}

/// `PUT /api/capacity-commitments` — set capacity promised to (supply) or work owed on (demand) a programme
/// for a pool and a month. One per pool, programme, organization, kind and month; setting it again replaces it.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn set_commitment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: axum::http::HeaderMap,
    Json(b): Json<CommitBody>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if !["supply", "demand"].contains(&b.kind.as_str()) {
        problems.push("kind must be supply or demand");
    }
    let month = rules::parse_month(&b.month);
    if month.is_none() {
        problems.push("month must be YYYY-MM");
    }
    let committed = milli(b.committed_fte);
    if committed.is_none() {
        problems.push("committed_fte must be a number from 0 to 1000");
    }
    let delivered = match b.delivered_fte {
        None => None,
        Some(d) => {
            let m = milli(d);
            if m.is_none() {
                problems.push("delivered_fte must be a number from 0 to 1000");
            }
            m
        }
    };
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let (month, committed) = (month.unwrap_or_default(), committed.unwrap_or_default());
    let plan = super::governance::find_item(&ctx, &b.plan_pid.to_string()).await?;
    // The plan is named in the body, so the plan guard on the path does not see it: a programme the
    // caller may not read is a 404 here too.
    if !super::plan_access::readable_plans(&ctx, &caller, &[plan.pid])
        .await
        .contains(&plan.pid)
    {
        return Err(Error::NotFound);
    }
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(
            "a commitment is to a programme (a plan of kind Program)",
        ));
    }
    let pool = pools::Entity::find()
        .filter(pools::Column::Pid.eq(b.pool_pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let Some(pool) = pool.filter(|p| p.active) else {
        return Err(unprocessable("pool_pid is not an active skill pool"));
    };
    // An organization record, if one is named (by pid, or by a label that matches one); else a label.
    let mut org_pid: Option<Uuid> = None;
    let mut label = b
        .organization
        .as_deref()
        .map(str::trim)
        .filter(|o| !o.is_empty())
        .map(str::to_string);
    if let Some(op) = b.organization_pid {
        let o = organizations::Entity::find()
            .filter(organizations::Column::Pid.eq(op))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .filter(|o| o.active)
            .ok_or_else(|| unprocessable("organization_pid is not an active organization"))?;
        org_pid = Some(o.pid);
        label = Some(o.name);
    } else if let Some(l) = &label
        && let Some(o) = organizations::Entity::find()
            .filter(sea_orm::sea_query::Expr::cust_with_values(
                "lower(name) = lower($1)",
                [l.clone()],
            ))
            .filter(organizations::Column::Active.eq(true))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
    {
        org_pid = Some(o.pid);
        label = Some(o.name);
    }
    let org = label;
    let mut find = commitments::Entity::find()
        .filter(commitments::Column::PoolPid.eq(b.pool_pid))
        .filter(commitments::Column::PlanPid.eq(plan.pid))
        .filter(commitments::Column::Kind.eq(&b.kind))
        .filter(commitments::Column::Month.eq(month));
    find = match &org {
        None => find.filter(commitments::Column::Organization.is_null()),
        Some(o) => find.filter(sea_orm::sea_query::Expr::cust_with_values(
            "lower(organization) = lower($1)",
            [o.clone()],
        )),
    };
    let existing = find.one(&ctx.db).await.map_err(|e| db_err(&e))?;
    let note = b
        .note
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty());
    let actor = caller.actor().map(str::to_string);
    let row = if let Some(e) = existing {
        let mut a: commitments::ActiveModel = e.into();
        a.organization_pid = ActiveValue::set(org_pid);
        a.committed_milli_fte = ActiveValue::set(committed);
        a.delivered_milli_fte = ActiveValue::set(delivered);
        a.note = ActiveValue::set(note);
        a.updated_by = ActiveValue::set(actor);
        a.update(&ctx.db).await.map_err(|e| db_err(&e))?
    } else {
        commitments::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            pool_pid: ActiveValue::set(b.pool_pid),
            plan_pid: ActiveValue::set(plan.pid),
            organization: ActiveValue::set(org),
            organization_pid: ActiveValue::set(org_pid),
            kind: ActiveValue::set(b.kind.clone()),
            month: ActiveValue::set(month),
            committed_milli_fte: ActiveValue::set(committed),
            delivered_milli_fte: ActiveValue::set(delivered),
            note: ActiveValue::set(note),
            updated_by: ActiveValue::set(actor),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    };
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "capacity_commitment_set",
        caller.actor(),
        Some(serde_json::json!({ "pool": b.pool_pid, "kind": b.kind, "month": b.month, "committed_milli_fte": committed })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    let mut out = commitment_json(&row);
    if b.kind == "demand"
        && let Some(peer) = pool.peer_pool_ref.clone()
    {
        out["peer_demand_send"] =
            start_demand_send(&ctx, &headers, pool.pid, peer, plan.pid, month).await?;
    }
    format::json(out)
}

/// Send a programme's demand on a linked pool for a month to the workforce service (FB-3), without holding up or
/// failing the write that asked for it. The figure is the programme's total demand commitments on the pool for the
/// month (every organization's), because WPM keeps one figure per programme, pool and month; a total of zero is sent
/// as `fte_centi: 0`, which withdraws the figure there (FB-5). The outcome is recorded
/// in `peer_demand_sends` (the latest per pool, programme and month; an older version never overwrites a newer
/// one); the caller's token is held in memory for the one send and never stored, so a failed send is not retried:
/// setting the commitment again sends again.
async fn start_demand_send(
    ctx: &AppContext,
    headers: &axum::http::HeaderMap,
    pool: Uuid,
    peer: String,
    plan: Uuid,
    month: NaiveDate,
) -> Result<serde_json::Value> {
    use crate::wpm_bridge::{self as bridge, DemandPlan};
    let total: i64 = commitments::Entity::find()
        .filter(commitments::Column::PoolPid.eq(pool))
        .filter(commitments::Column::PlanPid.eq(plan))
        .filter(commitments::Column::Kind.eq("demand"))
        .filter(commitments::Column::Month.eq(month))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .iter()
        .map(|c| i64::from(c.committed_milli_fte))
        .sum();
    let written_at = chrono::Utc::now();
    let event = bridge::peer_event_id(pool, plan, month, written_at);
    let base = bridge::base_url();
    let bearer = bridge::bearer_of(headers);
    let decided = bridge::plan_demand(total, base.is_some(), bearer.is_some());
    let (outcome, detail, centi) = match &decided {
        DemandPlan::Send(c) => ("started", bridge::started_detail(*c), Some(*c)),
        DemandPlan::Skip(o, d) => (*o, *d, None),
    };
    record_send(
        &ctx.db, pool, plan, month, &peer, centi, &event, written_at, outcome, detail,
    )
    .await?;
    if let (DemandPlan::Send(c), Some(base), Some(bearer)) = (decided, base, bearer) {
        let send = bridge::DemandSend {
            base,
            bearer,
            peer_pool_ref: peer.clone(),
            plan_pid: plan,
            month,
            fte_centi: c,
            peer_event_id: event.clone(),
            sent_at: written_at,
        };
        let db = ctx.db.clone();
        tokio::spawn(async move {
            let (outcome, detail) = bridge::send_demand(&send).await;
            if let Err(e) = record_send(
                &db,
                pool,
                plan,
                month,
                &send.peer_pool_ref,
                Some(send.fte_centi),
                &send.peer_event_id,
                written_at,
                outcome,
                &detail,
            )
            .await
            {
                tracing::warn!(error = %e, "could not record the outcome of a demand sent to the workforce service");
            }
        });
    }
    Ok(
        serde_json::json!({ "outcome": outcome, "detail": detail, "fte_centi": centi, "peer_event_id": event }),
    )
}

#[expect(clippy::too_many_arguments)]
async fn record_send(
    db: &sea_orm::DatabaseConnection,
    pool: Uuid,
    plan: Uuid,
    month: NaiveDate,
    peer: &str,
    centi: Option<i64>,
    event: &str,
    version_at: chrono::DateTime<chrono::Utc>,
    outcome: &str,
    detail: &str,
) -> Result<()> {
    use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
    db.execute_raw(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "INSERT INTO peer_demand_sends (pid, pool_pid, plan_pid, month, peer_pool_ref, fte_centi, peer_event_id, version_at, outcome, detail, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now(), now()) \
         ON CONFLICT (pool_pid, plan_pid, month) DO UPDATE SET peer_pool_ref = EXCLUDED.peer_pool_ref, fte_centi = EXCLUDED.fte_centi, \
         peer_event_id = EXCLUDED.peer_event_id, version_at = EXCLUDED.version_at, outcome = EXCLUDED.outcome, detail = EXCLUDED.detail, updated_at = now() \
         WHERE peer_demand_sends.version_at <= EXCLUDED.version_at",
        [
            Uuid::new_v4().into(),
            pool.into(),
            plan.into(),
            month.into(),
            peer.to_string().into(),
            centi.into(),
            event.to_string().into(),
            version_at.into(),
            outcome.to_string().into(),
            detail.to_string().into(),
        ],
    ))
    .await
    .map_err(|e| db_err(&e))?;
    Ok(())
}

fn commitment_json(c: &commitments::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": c.pid, "pool_pid": c.pool_pid, "plan_pid": c.plan_pid, "organization": c.organization, "organization_pid": c.organization_pid,
        "kind": c.kind, "month": c.month.format("%Y-%m").to_string(),
        "committed_fte": f64::from(c.committed_milli_fte) / 1000.0,
        "delivered_fte": c.delivered_milli_fte.map(|d| f64::from(d) / 1000.0), "note": c.note,
    })
}

/// `GET /api/capacity-commitments` — those on programmes you can read, at most 1000.
#[debug_handler]
async fn list_commitments(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let rows = commitments::Entity::find()
        .order_by_asc(commitments::Column::Month)
        .limit(1000)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let plans_of: Vec<Uuid> = rows
        .iter()
        .map(|r| r.plan_pid)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans_of).await;
    // A caller who belongs to an organization (the `organization` attribute) sees only that organization's
    // commitments (EX-3): a partner reads its own, not another's and not the organization's own.
    let mine = super::organizations::caller_organizations(&caller);
    let belongs = caller
        .claims()
        .is_some_and(|c| c.attrs.contains_key("organization"));
    let sends: HashMap<(Uuid, Uuid, NaiveDate), peer_demand_sends::Model> =
        peer_demand_sends::Entity::find()
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .map(|s| ((s.pool_pid, s.plan_pid, s.month), s))
            .collect();
    let linked: HashSet<Uuid> = pools::Entity::find()
        .filter(pools::Column::PeerPoolRef.is_not_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    format::json(serde_json::json!({
        "commitments": rows.iter()
            .filter(|r| readable.contains(&r.plan_pid))
            .filter(|r| !belongs || r.organization_pid.is_some_and(|o| mine.contains(&o)))
            .map(|r| {
                let mut j = commitment_json(r);
                let sent = sends.get(&(r.pool_pid, r.plan_pid, r.month));
                if r.kind == "demand" && (linked.contains(&r.pool_pid) || sent.is_some()) {
                    j["peer_demand_send"] = sent.map_or(serde_json::Value::Null, |s| serde_json::json!({
                        "outcome": s.outcome, "detail": s.detail, "fte_centi": s.fte_centi,
                        "peer_event_id": s.peer_event_id, "at": s.updated_at,
                    }));
                }
                j
            }).collect::<Vec<_>>(),
    }))
}

// ---- the heatmap ----------------------------------------------------------------------------

#[derive(Default)]
struct Cell {
    inputs: Inputs,
    partners: BTreeSet<String>,
    by_org: BTreeMap<String, (i64, Option<i64>)>,
}

/// The heatmap: for each pool and each of `months` months from `from`, demand, supply (own and
/// partners'), the gap, and a word, shape and colour. Allocations and commitments on plans outside
/// `include` (private or marked) are not read.
///
/// # Errors
///
/// When a database read fails, or the window is too long.
#[expect(clippy::too_many_lines)]
pub(crate) async fn heatmap(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    from: NaiveDate,
    months: u32,
    read: &crate::wpm_bridge::SupplyRead,
) -> Result<serde_json::Value> {
    let last = rules::add_months(from, months).pred_opt().unwrap_or(from);
    let periods = pr::periods(from, last, Grain::Month).map_err(|m| unprocessable(&m))?;
    let allocs: Vec<pr::Alloc> = allocations::Entity::find()
        .filter(allocations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .iter()
        .filter(|a| include.contains(&a.plan_pid))
        .map(super::portfolio_resources::to_alloc)
        .collect();
    let role_of: HashMap<String, String> = resource_roles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|r| (r.person_ref, r.role))
        .collect();
    let ranges = super::availability::all_ranges(ctx).await?;
    let mut who: BTreeSet<&str> = role_of.keys().map(String::as_str).collect();
    who.extend(
        allocs
            .iter()
            .filter(|a| !a.proposed)
            .filter_map(|a| a.person.as_deref()),
    );
    let people: Vec<Person> = who
        .iter()
        .map(|p| Person {
            person: (*p).to_string(),
            role: role_of.get(*p).cloned(),
            ranges: ranges.get(*p).cloned().unwrap_or_default(),
        })
        .collect();
    let plan = pr::plan(&periods, &allocs, &people, GroupBy::Role);

    let all_pools = pools::Entity::find()
        .filter(pools::Column::Active.eq(true))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let pool_members = members::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    // Linked pools take their supply from the workforce service (FB-2): asked now or as last received.
    let linked: Vec<(Uuid, String)> = all_pools
        .iter()
        .filter_map(|p| p.peer_pool_ref.clone().map(|r| (p.pid, r)))
        .collect();
    let starts: Vec<NaiveDate> = periods.iter().map(|p| p.from).collect();
    let peer = crate::wpm_bridge::supply_for(&linked, &starts, read).await;
    let peer_ref_of: HashMap<Uuid, String> = linked.into_iter().collect();
    let pool_of_role: HashMap<String, Uuid> = pool_members
        .iter()
        .filter(|m| m.kind == "role")
        .map(|m| (m.value.to_lowercase(), m.pool_pid))
        .collect();

    // Cells keyed by (pool or none, month index).
    let mut cells: BTreeMap<(Option<Uuid>, usize), Cell> = BTreeMap::new();
    for (mi, pr_rows) in plan.periods.iter().enumerate() {
        let days = pr_rows.period.days();
        for row in &pr_rows.rows {
            let pool = pool_of_role.get(&row.key.to_lowercase()).copied();
            let c = cells.entry((pool, mi)).or_default();
            c.inputs.own_demand += rules::milli_fte(row.demand_percent_days, days);
            if row.headcount > 0
                && let Some(s) = row.supply_percent_days
            {
                *c.inputs.own_supply.get_or_insert(0) += rules::milli_fte(s, days);
            }
        }
    }
    let by_month: HashMap<NaiveDate, usize> = periods
        .iter()
        .enumerate()
        .map(|(i, p)| (p.from, i))
        .collect();
    let rows = commitments::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    for r in rows.iter().filter(|r| include.contains(&r.plan_pid)) {
        let Some(&mi) = by_month.get(&r.month) else {
            continue;
        };
        let c = cells.entry((Some(r.pool_pid), mi)).or_default();
        let committed = i64::from(r.committed_milli_fte);
        let delivered = r.delivered_milli_fte.map(i64::from);
        match (r.kind.as_str(), &r.organization) {
            ("demand", _) => c.inputs.partner_demand += committed,
            ("supply", None) => *c.inputs.own_supply.get_or_insert(0) += committed,
            ("supply", Some(org)) => {
                *c.inputs.partner_committed.get_or_insert(0) += committed;
                if let Some(d) = delivered {
                    *c.inputs.partner_delivered.get_or_insert(0) += d;
                }
                c.partners.insert(org.to_lowercase());
                let e = c.by_org.entry(org.clone()).or_insert((0, None));
                e.0 += committed;
                if let Some(d) = delivered {
                    *e.1.get_or_insert(0) += d;
                }
            }
            _ => {}
        }
    }
    #[expect(clippy::cast_precision_loss)] // milli-FTE counts are tiny
    let fte = |m: i64| -> f64 { (m as f64) / 1000.0 };
    let names = show_partners();
    let mut pool_list: Vec<(Option<Uuid>, String)> = all_pools
        .iter()
        .map(|p| (Some(p.pid), p.name.clone()))
        .collect();
    pool_list.sort_by_key(|(_, n)| n.to_lowercase());
    if cells.keys().any(|(p, _)| p.is_none()) {
        pool_list.push((None, "no pool assigned".to_string()));
    }
    let month_names: Vec<String> = periods
        .iter()
        .map(|p| p.from.format("%Y-%m").to_string())
        .collect();
    let mut out_pools = Vec::new();
    let mut table = Vec::new();
    for (pid, name) in &pool_list {
        let mut out_cells = Vec::new();
        for (mi, month) in month_names.iter().enumerate() {
            let cell = cells.remove(&(*pid, mi)).unwrap_or_default();
            let here = rules::verdict(&cell.inputs);
            let reading = pid.and_then(|p| peer.get(&p)).and_then(|r| r.get(mi));
            let v = reading.map_or_else(
                || here.clone(),
                |r| rules::linked_verdict(&cell.inputs, r.milli),
            );
            let i = &cell.inputs;
            let mut j = serde_json::json!({
                "month": month, "state": v.state, "word": v.word, "shape": v.shape, "colour": v.colour,
                "demand_fte": fte(v.demand), "own_demand_fte": fte(i.own_demand), "partner_demand_fte": fte(i.partner_demand),
                "supply_fte": v.supply.map(fte), "own_supply_fte": i.own_supply.map(fte),
                "partner_supply_committed_fte": i.partner_committed.map(fte),
                "partner_supply_delivered_fte": i.partner_delivered.map(fte),
                "gap_fte": v.gap.map(fte), "partner_delivery_short": v.partner_delivery_short,
                "partners": cell.partners.len(),
                "supply_source": if reading.is_some() { "wpm" } else { "here" },
            });
            if let Some(r) = reading {
                // The workforce service owns this pool's supply; what was entered here is shown beside it, never added.
                j["entered_here_supply_fte"] = here.supply.map(fte).into();
                j["peer_supply"] = serde_json::json!({
                    "source": "wpm", "status": r.status, "supply_fte": r.milli.map(fte),
                    "as_of": r.as_of, "age_seconds": r.as_of.map(|t| (chrono::Utc::now() - t).num_seconds().max(0)),
                    "reason": r.reason,
                });
            }
            if names && !cell.by_org.is_empty() {
                j["by_organization"] = cell
                    .by_org
                    .iter()
                    .map(|(o, (c, d))| serde_json::json!({ "organization": o, "committed_fte": fte(*c), "delivered_fte": d.map(fte) }))
                    .collect::<Vec<_>>()
                    .into();
            }
            table.push(serde_json::json!({
                "pool": name, "month": month, "word": v.word, "shape": v.shape, "colour": v.colour,
                "demand_fte": fte(v.demand), "supply_fte": v.supply.map(fte), "gap_fte": v.gap.map(fte),
                "supply_source": if reading.is_some() { "wpm" } else { "here" },
            }));
            out_cells.push(j);
        }
        let peer_pool_ref = pid.and_then(|p| peer_ref_of.get(&p).cloned());
        out_pools.push(serde_json::json!({ "pool_pid": pid, "name": name, "peer_pool_ref": peer_pool_ref, "cells": out_cells }));
    }
    Ok(serde_json::json!({
        "months": month_names,
        "pools": out_pools,
        "table": table,
        "partner_names_shown": names,
        "note": "Demand is the committed allocations on plans you may read (by the role they ask for) plus work partners owe; supply is the organization's own people with the pool's roles, read from their availability, plus partners' commitments. Everything is in FTE. A pool with no supplier and no partner commitment is no supply recorded, not zero capacity. Shortfall is supply under demand; tight is supply under 110 % of demand. Proposed bookings are not demand. Skills are not used: a role maps to at most one pool, so nothing is counted twice. Partner organizations are labels until they exist as records, and partner names are hidden unless the deployment shows them. No person appears. A pool linked to the workforce service takes its supply from there (source wpm, with the time it was taken): what was entered here for it is shown as entered_here_supply_fte and never added; when the workforce service cannot be reached the last figure your own token fetched is shown with its age (never one fetched for another caller), and with none the supply is unknown, never zero.",
    }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct HeatQuery {
    #[serde(default)]
    from: Option<String>,
    #[serde(default)]
    months: Option<u32>,
}

/// `GET /api/capacity-heatmap[?from=YYYY-MM][&months=6]` — for the plans you can read.
#[debug_handler]
async fn live(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: axum::http::HeaderMap,
    Query(q): Query<HeatQuery>,
) -> Result<Response> {
    let months = q.months.unwrap_or(6);
    if !(1..=24).contains(&months) {
        return Err(unprocessable("months must be 1 to 24"));
    }
    let from = match &q.from {
        None => chrono::Utc::now()
            .date_naive()
            .with_day(1)
            .unwrap_or_default(),
        Some(f) => rules::parse_month(f).ok_or_else(|| unprocessable("from must be YYYY-MM"))?,
    };
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let read =
        crate::wpm_bridge::SupplyRead::live(caller.actor(), crate::wpm_bridge::bearer_of(&headers));
    format::json(heatmap(&ctx, &readable, from, months, &read).await?)
}

/// The pack's heatmap: six months from the month of the cut-off.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    cut_off: chrono::DateTime<chrono::Utc>,
) -> Result<serde_json::Value> {
    let from = cut_off.date_naive().with_day(1).unwrap_or_default();
    heatmap(ctx, include, from, 6, &crate::wpm_bridge::SupplyRead::Kept).await
}

//! Skill pools, capacity commitments and the capacity heatmap by pool and month (DP-10). Rules in
//! [`crate::capacity_heatmap`].
//!
//! **Partner organizations (EX-3) are not built.** A commitment carries the partner's name as a label;
//! none means the organization's own. Because there is no organization attribute to base a policy on,
//! the partners' names are shown only when `PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS` is
//! `1`/`true`; by default partner supply is shown combined and the number of partners is stated.

use super::refuse;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::prelude::*;
use axum::http::StatusCode;
use chrono::{Datelike, NaiveDate};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::capacity_heatmap::{self as rules, Inputs};
use crate::models::_entities::{
    allocations, capacity_commitments as commitments, organizations, peer_demand_sends,
    skill_pool_members as members, skill_pools as pools,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::portfolio_resources::{self as pr, Grain, GroupBy, Person};

/// Map a database error to a response: the three unique-index violations (peer link, role or skill in two pools, pool name) become `409`; anything else is an internal error carrying the text.
fn db_err(e: &sqlx::Error) -> Error {
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

/// Whether partner organization names may appear on the heatmap; off unless the deployment sets `PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS` to a true-like value.
fn show_partners() -> bool {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS")
        .is_ok_and(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "yes"))
}

// ---- pools ----------------------------------------------------------------------------------

/// Body of `POST /api/skill-pools`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewPool {
    /// Pool name, 2 to 120 characters after trimming; a duplicate name is refused with `409`.
    name: String,
    /// Resource role names the pool groups; a role belongs to at most one pool. Empty by default.
    #[serde(default)]
    roles: Vec<String>,
    /// Skill names the pool groups; a skill belongs to at most one pool. Empty by default.
    #[serde(default)]
    skills: Vec<String>,
}

/// Replace a pool's members with the given roles and skills inside the caller's transaction. Values are trimmed, blanks dropped and case-insensitive duplicates within a kind ignored.
async fn set_members(
    tx: &mut impl crate::db::Session,
    pool: Uuid,
    roles: &[String],
    skills: &[String],
) -> Result<()> {
    sqlx::query("DELETE FROM skill_pool_members WHERE pool_pid = $1")
        .bind(pool)
        .execute(tx.ex())
        .await
        .map_err(|e| db_err(&e))?;
    for (kind, values) in [("role", roles), ("skill", skills)] {
        let mut seen = BTreeSet::new();
        for v in values {
            let v = v.trim();
            if v.is_empty() || !seen.insert(v.to_lowercase()) {
                continue;
            }
            sqlx::query(
                "INSERT INTO skill_pool_members (pid, pool_pid, kind, value) VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::new_v4())
            .bind(pool)
            .bind(kind)
            .bind(v)
            .execute(tx.ex())
            .await
            .map_err(|e| db_err(&e))?;
        }
    }
    Ok(())
}

/// A pool's members.
async fn members_of(ctx: &AppContext, pool: Uuid) -> Result<Vec<members::Model>> {
    sqlx::query_as("SELECT * FROM skill_pool_members WHERE pool_pid = $1")
        .bind(pool)
        .fetch_all(crate::db::pool(ctx))
        .await
        .map_err(|e| db_err(&e))
}

/// A pool by pid.
async fn find_pool(ctx: &AppContext, pid: Uuid) -> Result<Option<pools::Model>> {
    sqlx::query_as("SELECT * FROM skill_pools WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await
        .map_err(|e| db_err(&e))
}

/// Render a pool and its roles and skills (sorted, so the output is stable) as the API's JSON shape; `peer_pool_ref` is null when unlinked.
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
    let mut tx = crate::db::begin(&ctx).await.map_err(|e| db_err(&e))?;
    let row: pools::Model = sqlx::query_as(
        "INSERT INTO skill_pools (pid, name, active) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(name)
    .bind(true)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    set_members(&mut tx, row.pid, &b.roles, &b.skills).await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "skill_pool_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let ms = members_of(&ctx, row.pid).await?;
    format::json(pool_json(&row, &ms))
}

/// Body of `PUT /api/skill-pools/{pid}`. Every field is optional; an absent one leaves that part unchanged.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdatePool {
    /// `false` retires the pool, `true` restores it; absent leaves it as is.
    #[serde(default)]
    active: Option<bool>,
    /// Replacement role list; absent keeps the current roles.
    #[serde(default)]
    roles: Option<Vec<String>>,
    /// Replacement skill list; absent keeps the current skills.
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
    let row = find_pool(&ctx, pid).await?.ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await.map_err(|e| db_err(&e))?;
    // With no `active` given, the row is read back unchanged (as `SeaORM`'s empty update did).
    let row: pools::Model = if let Some(active) = b.active {
        sqlx::query_as("UPDATE skill_pools SET active = $1 WHERE id = $2 RETURNING *")
            .bind(active)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await
            .map_err(|e| db_err(&e))?
    } else {
        sqlx::query_as("SELECT * FROM skill_pools WHERE id = $1 LIMIT 1")
            .bind(row.id)
            .fetch_one(tx.conn())
            .await
            .map_err(|e| db_err(&e))?
    };
    if b.roles.is_some() || b.skills.is_some() {
        let current: Vec<members::Model> =
            sqlx::query_as("SELECT * FROM skill_pool_members WHERE pool_pid = $1")
                .bind(pid)
                .fetch_all(tx.conn())
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
        set_members(&mut tx, pid, &roles, &skills).await?;
    }
    AuditModel::record_on(tx.conn(), pid, "skill_pool_updated", caller.actor(), None)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let ms = members_of(&ctx, pid).await?;
    format::json(pool_json(&row, &ms))
}

/// `GET /api/skill-pools`.
#[debug_handler]
async fn list_pools(State(ctx): State<AppContext>) -> Result<Response> {
    let mut ps: Vec<pools::Model> = sqlx::query_as("SELECT * FROM skill_pools")
        .fetch_all(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?;
    ps.sort_by_key(|p| p.name.to_lowercase());
    let ms: Vec<members::Model> = sqlx::query_as("SELECT * FROM skill_pool_members")
        .fetch_all(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?;
    format::json(
        serde_json::json!({ "skill_pools": ps.iter().map(|p| pool_json(p, &ms)).collect::<Vec<_>>() }),
    )
}

/// Body of `PUT /api/skill-pools/{pid}/peer-link`.
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
    let row = find_pool(&ctx, pid).await?.ok_or(Error::NotFound)?;
    let action = if peer.is_some() {
        "skill_pool_peer_linked"
    } else {
        "skill_pool_peer_unlinked"
    };
    let before = row
        .peer_pool_ref
        .clone()
        .filter(|o| Some(o) != peer.as_ref());
    let mut tx = crate::db::begin(&ctx).await.map_err(|e| db_err(&e))?;
    let row: pools::Model =
        sqlx::query_as("UPDATE skill_pools SET peer_pool_ref = $1 WHERE id = $2 RETURNING *")
            .bind(peer)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await
            .map_err(|e| db_err(&e))?;
    AuditModel::record_on(tx.conn(), pid, action, caller.actor(), None)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let ms = members_of(&ctx, pid).await?;
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
    let rows: Vec<peer_demand_sends::Model> = sqlx::query_as(
        "SELECT * FROM peer_demand_sends
         WHERE pool_pid = $1 AND peer_pool_ref = $2 AND fte_centi > $3
         ORDER BY month, plan_pid
         LIMIT $4",
    )
    .bind(pool)
    .bind(&old)
    .bind(0_i64)
    .bind(i64::try_from(WITHDRAWALS_MAX).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(ctx))
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
            ctx,
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
        let ctx = ctx.clone();
        tokio::spawn(async move {
            for send in sends {
                let (outcome, detail) = bridge::send_demand(&send).await;
                if let Err(e) = record_send(
                    &ctx,
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

/// Body of `PUT /api/capacity-commitments`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CommitBody {
    /// The skill pool the commitment is for; it must be active.
    pool_pid: Uuid,
    /// The programme (a plan of kind Program) the commitment is to or from.
    plan_pid: Uuid,
    /// Partner label; absent or blank means the organization's own. If it matches an active organization record (case-insensitive) that record is linked.
    #[serde(default)]
    organization: Option<String>,
    /// An organization record (EX-3); its name then becomes the label.
    #[serde(default)]
    organization_pid: Option<Uuid>,
    /// Either `supply` (capacity promised) or `demand` (work owed).
    kind: String,
    /// `YYYY-MM`.
    month: String,
    /// Committed full-time equivalents, 0 to 1000; stored in thousandths.
    committed_fte: f64,
    /// Delivered FTE, 0 to 1000; absent means no delivery reported yet, not zero.
    #[serde(default)]
    delivered_fte: Option<f64>,
    /// Free-text note; blank is stored as absent.
    #[serde(default)]
    note: Option<String>,
}

/// Convert FTE to thousandths, rounded; `None` for a non-finite value or one outside 0 to 1000.
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
    let pool = find_pool(&ctx, b.pool_pid).await?;
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
        let o = sqlx::query_as::<_, organizations::Model>(
            "SELECT * FROM organizations WHERE pid = $1 LIMIT 1",
        )
        .bind(op)
        .fetch_optional(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?
        .filter(|o| o.active)
        .ok_or_else(|| unprocessable("organization_pid is not an active organization"))?;
        org_pid = Some(o.pid);
        label = Some(o.name);
    } else if let Some(l) = &label
        && let Some(o) = sqlx::query_as::<_, organizations::Model>(
            "SELECT * FROM organizations WHERE lower(name) = lower($1) AND active = $2 LIMIT 1",
        )
        .bind(l)
        .bind(true)
        .fetch_optional(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?
    {
        org_pid = Some(o.pid);
        label = Some(o.name);
    }
    let org = label;
    // Compare labels case-insensitively so a differently cased label updates the same commitment.
    let existing: Option<commitments::Model> = match &org {
        None => sqlx::query_as(
            "SELECT * FROM capacity_commitments
             WHERE pool_pid = $1 AND plan_pid = $2 AND kind = $3 AND month = $4 AND organization IS NULL
             LIMIT 1",
        )
        .bind(b.pool_pid)
        .bind(plan.pid)
        .bind(&b.kind)
        .bind(month)
        .fetch_optional(crate::db::pool(&ctx))
        .await,
        Some(o) => sqlx::query_as(
            "SELECT * FROM capacity_commitments
             WHERE pool_pid = $1 AND plan_pid = $2 AND kind = $3 AND month = $4
               AND lower(organization) = lower($5)
             LIMIT 1",
        )
        .bind(b.pool_pid)
        .bind(plan.pid)
        .bind(&b.kind)
        .bind(month)
        .bind(o)
        .fetch_optional(crate::db::pool(&ctx))
        .await,
    }
    .map_err(|e| db_err(&e))?;
    let note = b
        .note
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty());
    let actor = caller.actor().map(str::to_string);
    // The commitment and its audit row in one transaction (before SQ-1 batch 5 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await.map_err(|e| db_err(&e))?;
    let row: commitments::Model = if let Some(e) = existing {
        sqlx::query_as(
            "UPDATE capacity_commitments
             SET organization_pid = $1, committed_milli_fte = $2, delivered_milli_fte = $3, note = $4,
                 updated_by = $5
             WHERE id = $6
             RETURNING *",
        )
        .bind(org_pid)
        .bind(committed)
        .bind(delivered)
        .bind(note)
        .bind(actor)
        .bind(e.id)
        .fetch_one(tx.conn())
        .await
        .map_err(|e| db_err(&e))?
    } else {
        sqlx::query_as(
            "INSERT INTO capacity_commitments
                 (pid, pool_pid, plan_pid, organization, organization_pid, kind, month, committed_milli_fte,
                  delivered_milli_fte, note, updated_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(b.pool_pid)
        .bind(plan.pid)
        .bind(org)
        .bind(org_pid)
        .bind(&b.kind)
        .bind(month)
        .bind(committed)
        .bind(delivered)
        .bind(note)
        .bind(actor)
        .fetch_one(tx.conn())
        .await
        .map_err(|e| db_err(&e))?
    };
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "capacity_commitment_set",
        caller.actor(),
        Some(serde_json::json!({ "pool": b.pool_pid, "kind": b.kind, "month": b.month, "committed_milli_fte": committed })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
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
    let total: i64 = sqlx::query_scalar::<_, i32>(
        "SELECT committed_milli_fte FROM capacity_commitments
         WHERE pool_pid = $1 AND plan_pid = $2 AND kind = $3 AND month = $4",
    )
    .bind(pool)
    .bind(plan)
    .bind("demand")
    .bind(month)
    .fetch_all(crate::db::pool(ctx))
    .await
    .map_err(|e| db_err(&e))?
    .into_iter()
    .map(i64::from)
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
        ctx, pool, plan, month, &peer, centi, &event, written_at, outcome, detail,
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
        let ctx = ctx.clone();
        tokio::spawn(async move {
            let (outcome, detail) = bridge::send_demand(&send).await;
            if let Err(e) = record_send(
                &ctx,
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

/// Record the outcome of a demand figure sent (or not sent) to the workforce service in `peer_demand_sends`. One row per pool, programme and month; the upsert only replaces a row whose `version_at` is not newer, so a late outcome never overwrites a later send.
#[expect(clippy::too_many_arguments)]
async fn record_send(
    ctx: &AppContext,
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
    sqlx::query(
        "INSERT INTO peer_demand_sends (pid, pool_pid, plan_pid, month, peer_pool_ref, fte_centi, peer_event_id, version_at, outcome, detail, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now(), now()) \
         ON CONFLICT (pool_pid, plan_pid, month) DO UPDATE SET peer_pool_ref = EXCLUDED.peer_pool_ref, fte_centi = EXCLUDED.fte_centi, \
         peer_event_id = EXCLUDED.peer_event_id, version_at = EXCLUDED.version_at, outcome = EXCLUDED.outcome, detail = EXCLUDED.detail, updated_at = now() \
         WHERE peer_demand_sends.version_at <= EXCLUDED.version_at",
    )
    .bind(Uuid::new_v4())
    .bind(pool)
    .bind(plan)
    .bind(month)
    .bind(peer)
    .bind(centi)
    .bind(event)
    .bind(version_at)
    .bind(outcome)
    .bind(detail)
    .execute(crate::db::pool(ctx))
    .await
    .map_err(|e| db_err(&e))?;
    Ok(())
}

/// Render a commitment with FTE converted back from thousandths; `delivered_fte` is null when none was reported.
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
    let rows: Vec<commitments::Model> =
        sqlx::query_as("SELECT * FROM capacity_commitments ORDER BY month LIMIT $1")
            .bind(1000_i64)
            .fetch_all(crate::db::pool(&ctx))
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
        sqlx::query_as::<_, peer_demand_sends::Model>("SELECT * FROM peer_demand_sends")
            .fetch_all(crate::db::pool(&ctx))
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .map(|s| ((s.pool_pid, s.plan_pid, s.month), s))
            .collect();
    // Pools linked to the workforce service, so a demand commitment shows its send state even before a send is recorded.
    let linked: HashSet<Uuid> = sqlx::query_scalar::<_, Uuid>(
        "SELECT pid FROM skill_pools WHERE peer_pool_ref IS NOT NULL",
    )
    .fetch_all(crate::db::pool(&ctx))
    .await
    .map_err(|e| db_err(&e))?
    .into_iter()
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

/// One pool and month of the heatmap while it is being accumulated.
#[derive(Default)]
struct Cell {
    /// The demand and supply figures fed to the verdict rules.
    inputs: Inputs,
    /// Lower-cased partner names with supply in this cell, used only for a count unless names are shown.
    partners: BTreeSet<String>,
    /// Per partner label: committed and (if any reported) delivered milli-FTE; shown only when partner names are enabled.
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
    let allocs: Vec<pr::Alloc> = sqlx::query_as::<_, allocations::Model>(
        "SELECT * FROM allocations WHERE deleted_at IS NULL",
    )
    .fetch_all(crate::db::pool(ctx))
    .await
    .map_err(|e| db_err(&e))?
    .iter()
    .filter(|a| include.contains(&a.plan_pid))
    .map(super::portfolio_resources::to_alloc)
    .collect();
    let role_of: HashMap<String, String> =
        sqlx::query_as::<_, (String, String)>("SELECT person_ref, role FROM resource_roles")
            .fetch_all(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .collect();
    let ranges = super::availability::all_ranges(ctx).await?;
    // Everyone with a role or a non-proposed booking: proposed bookings are not demand and must not add people.
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

    let all_pools: Vec<pools::Model> =
        sqlx::query_as("SELECT * FROM skill_pools WHERE active = $1")
            .bind(true)
            .fetch_all(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?;
    let pool_members: Vec<members::Model> = sqlx::query_as("SELECT * FROM skill_pool_members")
        .fetch_all(crate::db::pool(ctx))
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
    let rows: Vec<commitments::Model> = sqlx::query_as("SELECT * FROM capacity_commitments")
        .fetch_all(crate::db::pool(ctx))
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
    // Demand with no matching pool gets its own row so it is never silently dropped.
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

/// Query of `GET /api/capacity-heatmap`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct HeatQuery {
    /// First month as `YYYY-MM`; absent means the current month.
    #[serde(default)]
    from: Option<String>,
    /// Number of months, 1 to 24; absent means 6.
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
    let all: Vec<Uuid> = sqlx::query_scalar("SELECT pid FROM plans WHERE deleted_at IS NULL")
        .fetch_all(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?;
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

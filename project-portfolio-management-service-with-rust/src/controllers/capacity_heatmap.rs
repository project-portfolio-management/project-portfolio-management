//! Skill pools, capacity commitments and the capacity heatmap by pool and month (DP-10). Rules in
//! [`crate::capacity_heatmap`].
//!
//! **Partner organizations (EX-3) are not built.** A commitment carries the partner's name as a label;
//! none means the organization's own. Because there is no organization attribute to base a policy on,
//! the partners' names are shown only when `PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS` is
//! `1`/`true`; by default partner supply is shown combined and the number of partners is stated.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{Datelike, NaiveDate};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::capacity_heatmap::{self as rules, Inputs};
use crate::models::_entities::{
    allocations, capacity_commitments as commitments, plans, resource_roles,
    skill_pool_members as members, skill_pools as pools,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::portfolio_resources::{self as pr, Grain, GroupBy, Person};

fn refuse(status: StatusCode, code: &str, message: &str) -> Error {
    Error::CustomError(status, ErrorDetail::new(code, message))
}

fn unprocessable(message: &str) -> Error {
    refuse(
        StatusCode::UNPROCESSABLE_ENTITY,
        "unprocessable_entity",
        message,
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    let t = e.to_string();
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
        .add("/capacity-commitments", get(list_commitments))
        .add("/capacity-commitments", put(set_commitment))
        .add("/capacity-heatmap", get(live))
}

fn show_partners() -> bool {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS")
        .is_ok_and(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "yes"))
}

// ---- pools ----------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct NewPool {
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
    serde_json::json!({ "pid": p.pid, "name": p.name, "active": p.active, "roles": of("role"), "skills": of("skill") })
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

#[derive(Debug, Deserialize)]
struct UpdatePool {
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

// ---- commitments ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CommitBody {
    pool_pid: Uuid,
    plan_pid: Uuid,
    #[serde(default)]
    organization: Option<String>,
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
    #[allow(clippy::cast_possible_truncation)]
    Some((f * 1000.0).round() as i32)
}

/// `PUT /api/capacity-commitments` — set capacity promised to (supply) or work owed on (demand) a programme
/// for a pool and a month. One per pool, programme, organization, kind and month; setting it again replaces it.
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn set_commitment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
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
    if !pool.is_some_and(|p| p.active) {
        return Err(unprocessable("pool_pid is not an active skill pool"));
    }
    let org = b
        .organization
        .as_deref()
        .map(str::trim)
        .filter(|o| !o.is_empty())
        .map(str::to_string);
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
    format::json(commitment_json(&row))
}

fn commitment_json(c: &commitments::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": c.pid, "pool_pid": c.pool_pid, "plan_pid": c.plan_pid, "organization": c.organization,
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
    format::json(serde_json::json!({
        "commitments": rows.iter().filter(|r| readable.contains(&r.plan_pid)).map(commitment_json).collect::<Vec<_>>(),
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
#[allow(clippy::too_many_lines)]
pub(crate) async fn heatmap(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    from: NaiveDate,
    months: u32,
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
    #[allow(clippy::cast_precision_loss)] // milli-FTE counts are tiny
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
            let v = rules::verdict(&cell.inputs);
            let i = &cell.inputs;
            let mut j = serde_json::json!({
                "month": month, "state": v.state, "word": v.word, "shape": v.shape, "colour": v.colour,
                "demand_fte": fte(v.demand), "own_demand_fte": fte(i.own_demand), "partner_demand_fte": fte(i.partner_demand),
                "supply_fte": v.supply.map(fte), "own_supply_fte": i.own_supply.map(fte),
                "partner_supply_committed_fte": i.partner_committed.map(fte),
                "partner_supply_delivered_fte": i.partner_delivered.map(fte),
                "gap_fte": v.gap.map(fte), "partner_delivery_short": v.partner_delivery_short,
                "partners": cell.partners.len(),
            });
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
            }));
            out_cells.push(j);
        }
        out_pools.push(serde_json::json!({ "pool_pid": pid, "name": name, "cells": out_cells }));
    }
    Ok(serde_json::json!({
        "months": month_names,
        "pools": out_pools,
        "table": table,
        "partner_names_shown": names,
        "note": "Demand is the committed allocations on plans you may read (by the role they ask for) plus work partners owe; supply is the organization's own people with the pool's roles, read from their availability, plus partners' commitments. Everything is in FTE. A pool with no supplier and no partner commitment is no supply recorded, not zero capacity. Shortfall is supply under demand; tight is supply under 110 % of demand. Proposed bookings are not demand. Skills are not used: a role maps to at most one pool, so nothing is counted twice. Partner organizations are labels until they exist as records, and partner names are hidden unless the deployment shows them. No person appears.",
    }))
}

#[derive(Debug, Deserialize)]
struct HeatQuery {
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
    format::json(heatmap(&ctx, &readable, from, months).await?)
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
    heatmap(ctx, include, from, 6).await
}

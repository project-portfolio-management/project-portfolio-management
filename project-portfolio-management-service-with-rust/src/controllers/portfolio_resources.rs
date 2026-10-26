//! Portfolio resource management (SC-7): demand against supply by role or
//! skill and period across plans, role-only **placeholders** resolved to named
//! people, and planned against actual hours. The rules are
//! [`crate::portfolio_resources`]; this is the wiring.
//!
//! A caller who may not manage people (`access=admin` or `role=resource_manager`
//! — the same line as availability) sees **roles and totals only**: no person
//! refs anywhere, and unplanned time is shown per plan, not per person.

use super::db_err;
use super::forbidden;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{allocations, resource_roles, scenarios, time_entries};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::portfolio_resources::{
    self as rules, Actual, Alloc, Grain, GroupBy, Person, fte, is_placeholder,
};

/// Time entries read per call.
const ROW_CAP: u64 = 20_000;

/// Whether the caller may see and manage people's resource detail.
pub(crate) fn may_manage_people(caller: &MaybeAuthUser) -> bool {
    caller.holds_or_open(|c| {
        let has = |key: &str, want: &str| {
            c.attrs
                .get(key)
                .is_some_and(|v| v.iter().any(|a| a == want))
        };
        has("access", "admin") || has("role", "resource_manager")
    })
}

pub(crate) fn to_alloc(a: &allocations::Model) -> Alloc {
    Alloc {
        plan: a.plan_pid,
        person: (!is_placeholder(&a.person_ref)).then(|| a.person_ref.clone()),
        role: a.role.clone().filter(|r| !r.trim().is_empty()),
        skills: serde_json::from_value(a.skills_required.clone()).unwrap_or_default(),
        percent: a.percent,
        start: a.start_date,
        end: a.end_date,
        proposed: a.booking == "proposed",
    }
}

/// The live allocations the caller may read, optionally limited to a scenario's
/// plans, and the scope that was applied.
async fn scoped_allocations(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    scenario: Option<&str>,
) -> Result<(Vec<allocations::Model>, serde_json::Value)> {
    let mut rows = vis_models::all_allocations_with_placeholders(&ctx.db).await?;
    let mut scope = serde_json::json!({ "kind": "portfolio" });
    if let Some(raw) = scenario {
        let pid = super::parse_pid(raw)?;
        let s = scenarios::Entity::find()
            .filter(scenarios::Column::Pid.eq(pid))
            .filter(scenarios::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .ok_or(Error::NotFound)?;
        let members: HashSet<Uuid> = s
            .members
            .get("plan_pids")
            .and_then(|v| serde_json::from_value::<Vec<Uuid>>(v.clone()).ok())
            .unwrap_or_default()
            .into_iter()
            .collect();
        rows.retain(|a| members.contains(&a.plan_pid));
        scope = serde_json::json!({ "kind": "scenario", "scenario_pid": s.pid,
                                     "name": s.name, "plans": members.len() });
    }
    let mut plan_pids: Vec<Uuid> = rows.iter().map(|a| a.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = super::plan_access::readable_plans(ctx, caller, &plan_pids).await;
    rows.retain(|a| readable.contains(&a.plan_pid));
    Ok((rows, scope))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PlanParams {
    #[serde(default)]
    from: Option<NaiveDate>,
    #[serde(default)]
    to: Option<NaiveDate>,
    /// `week` or `month` (default month).
    #[serde(default)]
    period: Option<String>,
    /// `role` (default) or `skill`.
    #[serde(default)]
    group_by: Option<String>,
    /// Limit demand to a scenario's plans (what-if).
    #[serde(default)]
    scenario: Option<String>,
}

/// `GET /api/resource-plan` — demand against supply by role (or skill) and
/// period, in FTE.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn resource_plan(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<PlanParams>,
) -> Result<Response> {
    let today = crate::tz::deployment_today();
    let from = q.from.unwrap_or(today);
    let to = q.to.unwrap_or(from + chrono::Days::new(89));
    let grain = match q.period.as_deref() {
        None => Grain::Month,
        Some(p) => Grain::parse(p).ok_or_else(|| unprocessable("period must be week or month"))?,
    };
    let group = match q.group_by.as_deref() {
        None | Some("role") => GroupBy::Role,
        Some("skill") => GroupBy::Skill,
        Some(_) => return Err(unprocessable("group_by must be role or skill")),
    };
    let periods = rules::periods(from, to, grain).map_err(|m| unprocessable(&m))?;
    let (rows, scope) = scoped_allocations(&ctx, &caller, q.scenario.as_deref()).await?;
    let allocs: Vec<Alloc> = rows.iter().map(to_alloc).collect();

    // Supply: everyone with a primary role, plus named people with demand in
    // scope and no role (the unclassified row).
    let roles: HashMap<String, String> = resource_roles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| (r.person_ref, r.role))
        .collect();
    let all_ranges = super::availability::all_ranges(&ctx).await?;
    let mut who: BTreeSet<&str> = roles.keys().map(String::as_str).collect();
    who.extend(
        allocs
            .iter()
            .filter(|a| !a.proposed)
            .filter_map(|a| a.person.as_deref()),
    );
    let people: Vec<Person> = if group == GroupBy::Role {
        who.iter()
            .map(|p| Person {
                person: (*p).to_string(),
                role: roles.get(*p).cloned(),
                ranges: all_ranges.get(*p).cloned().unwrap_or_default(),
            })
            .collect()
    } else {
        Vec::new()
    };
    let result = rules::plan(&periods, &allocs, &people, group);

    let see = may_manage_people(&caller);
    let mut by_role: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for p in &people {
        by_role
            .entry(
                p.role
                    .clone()
                    .unwrap_or_else(|| rules::UNCLASSIFIED.to_string()),
            )
            .or_default()
            .push(&p.person);
    }
    let periods_json: Vec<serde_json::Value> = result
        .periods
        .iter()
        .map(|pr| {
            let days = pr.period.days();
            let rows: Vec<serde_json::Value> = pr
                .rows
                .iter()
                .map(|r| {
                    let supply = r.supply_percent_days;
                    serde_json::json!({
                        "key": r.key,
                        "demand_fte": fte(r.demand_percent_days, days),
                        "named_fte": fte(r.named_percent_days, days),
                        "placeholder_fte": fte(r.placeholder_percent_days, days),
                        "proposed_fte": fte(r.proposed_percent_days, days),
                        "supply_fte": supply.map(|s| fte(s, days)),
                        // Negative is a shortage; computed from exact percent-days.
                        "gap_fte": supply.map(|s| fte(s - r.demand_percent_days, days)),
                        "headcount": r.headcount,
                        "demand_percent_days": r.demand_percent_days,
                        "placeholder_percent_days": r.placeholder_percent_days,
                        "supply_percent_days": supply,
                    })
                })
                .collect();
            serde_json::json!({
                "from": pr.period.from, "to": pr.period.to, "days": days,
                "rows": rows,
                "total_demand_fte": fte(pr.total_demand_percent_days, days),
                "total_placeholder_fte": fte(pr.total_placeholder_percent_days, days),
                "total_proposed_fte": fte(pr.total_proposed_percent_days, days),
                "total_demand_percent_days": pr.total_demand_percent_days,
            })
        })
        .collect();
    let placeholders: Vec<serde_json::Value> = rows
        .iter()
        .filter(|a| is_placeholder(&a.person_ref))
        .map(|a| {
            serde_json::json!({ "allocation_pid": a.pid, "plan_pid": a.plan_pid, "role": a.role,
                "percent": a.percent, "start_date": a.start_date, "end_date": a.end_date,
                "booking": a.booking })
        })
        .collect();
    format::json(serde_json::json!({
        "scope": scope,
        "from": from, "to": to,
        "period": if grain == Grain::Week { "week" } else { "month" },
        "group_by": if group == GroupBy::Role { "role" } else { "skill" },
        "overlapping": group == GroupBy::Skill,
        "periods": periods_json,
        "by_plan": result.by_plan.iter().map(|s| serde_json::json!({
            "plan_pid": s.plan, "demand_percent_days": s.demand_percent_days })).collect::<Vec<_>>(),
        "placeholders": placeholders,
        "people_by_role": if see && group == GroupBy::Role { serde_json::json!(by_role) } else { serde_json::Value::Null },
        "masked": !see,
        "note": "FTE is percent-days divided by 100 × the days in the period, so a gap is always full-time equivalents (supply minus demand; negative is a shortage), with headcount a separate field. Demand is committed allocations (placeholders included, shown apart as placeholder_fte); proposed bookings are a separate series and never enter demand or the gap. Supply is the people whose primary role it is (PUT /api/resource-roles), read per day from their availability; people with no role and demand with no role are the unclassified row. Skills are demand-only and overlap (an allocation counts fully toward each of its tags). Days are calendar days on both sides. A caller who may not manage people sees roles and totals only.",
    }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct HoursParams {
    #[serde(default)]
    from: Option<NaiveDate>,
    #[serde(default)]
    to: Option<NaiveDate>,
}

/// `GET /api/resource-plan/hours` — planned against actual hours.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn hours(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<HoursParams>,
) -> Result<Response> {
    let today = crate::tz::deployment_today();
    let to = q.to.unwrap_or(today);
    let from = q.from.unwrap_or(to - chrono::Days::new(27));
    if to < from {
        return Err(unprocessable("to is before from"));
    }
    if (to - from).num_days() > rules::MAX_DAYS {
        return Err(unprocessable("the window may span at most ten years"));
    }
    let (rows, _) = scoped_allocations(&ctx, &caller, None).await?;
    let in_window = |a: &&allocations::Model| {
        a.start_date.is_none_or(|s| s <= to) && a.end_date.is_none_or(|e| e >= from)
    };
    let live: Vec<&allocations::Model> = rows.iter().filter(in_window).collect();
    let allocs: Vec<Alloc> = live.iter().map(|a| to_alloc(a)).collect();
    let any: HashSet<(Uuid, String)> = live
        .iter()
        .filter(|a| !is_placeholder(&a.person_ref))
        .map(|a| (a.plan_pid, a.person_ref.clone()))
        .collect();

    let entries = time_entries::Entity::find()
        .filter(time_entries::Column::DeletedAt.is_null())
        .filter(time_entries::Column::SpentOn.gte(from))
        .filter(time_entries::Column::SpentOn.lte(to))
        .order_by_asc(time_entries::Column::Id)
        .limit(ROW_CAP + 1)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let truncated = entries.len() as u64 > ROW_CAP;
    let mut plan_pids: Vec<Uuid> = entries.iter().map(|e| e.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plan_pids).await;
    let actuals: Vec<Actual> = entries
        .iter()
        .take(usize::try_from(ROW_CAP).unwrap_or(usize::MAX))
        .filter(|e| readable.contains(&e.plan_pid))
        .map(|e| Actual {
            plan: e.plan_pid,
            person: e.actor_ref.clone(),
            minutes: e.minutes,
        })
        .collect();
    let table = rules::hours(from, to, &allocs, &any, &actuals);
    let see = may_manage_people(&caller);

    let mut plans: BTreeMap<Uuid, Vec<&rules::HoursRow>> = BTreeMap::new();
    for r in &table {
        plans.entry(r.plan).or_default().push(r);
    }
    let mut total_planned = 0_i64;
    let mut total_actual: Option<i64> = None;
    let mut out = Vec::new();
    let mut unplanned = Vec::new();
    for (plan, rs) in &plans {
        let planned: i64 = rs
            .iter()
            .filter(|r| r.person.is_some())
            .map(|r| r.planned_minutes)
            .sum();
        let placeholder: i64 = rs
            .iter()
            .filter(|r| r.person.is_none())
            .map(|r| r.planned_minutes)
            .sum();
        let actual: Option<i64> = rs
            .iter()
            .filter_map(|r| r.actual_minutes)
            .fold(None, |acc, m| Some(acc.unwrap_or(0) + m));
        total_planned += planned;
        if let Some(a) = actual {
            total_actual = Some(total_actual.unwrap_or(0) + a);
        }
        let unplanned_rows: Vec<&&rules::HoursRow> = rs.iter().filter(|r| r.unplanned).collect();
        if !unplanned_rows.is_empty() {
            let minutes: i64 = unplanned_rows.iter().filter_map(|r| r.actual_minutes).sum();
            unplanned.push(if see {
                serde_json::json!({ "plan_pid": plan, "minutes": minutes,
                    "people": unplanned_rows.iter().map(|r| serde_json::json!({
                        "person_ref": r.person, "minutes": r.actual_minutes })).collect::<Vec<_>>() })
            } else {
                serde_json::json!({ "plan_pid": plan, "minutes": minutes,
                    "people_count": unplanned_rows.len() })
            });
        }
        let mut v = serde_json::json!({
            "plan_pid": plan,
            "planned_minutes": planned,
            "placeholder_planned_minutes": placeholder,
            "actual_minutes": actual,
            "actual_reason": if actual.is_none() { Some("no_actuals") } else { None },
            "variance_minutes": actual.map(|a| a - planned),
        });
        if see {
            v["people"] = serde_json::json!(
                rs.iter()
                    .filter(|r| r.person.is_some())
                    .map(|r| serde_json::json!({
                "person_ref": r.person, "planned_minutes": r.planned_minutes,
                "actual_minutes": r.actual_minutes,
                "actual_reason": if r.actual_minutes.is_none() { Some("no_actuals") } else { None },
                "unplanned": r.unplanned }))
                    .collect::<Vec<_>>()
            );
        }
        out.push(v);
    }
    format::json(serde_json::json!({
        "from": from, "to": to,
        "plans": out,
        "unplanned_actuals": unplanned,
        "total_planned_minutes": total_planned,
        "total_actual_minutes": total_actual,
        "total_actual_reason": if total_actual.is_none() { Some("no_actuals") } else { None },
        "truncated": truncated,
        "masked": !see,
        "note": "Planned is a named, committed allocation's percent of an 8-hour day on each Monday to Friday of the window (placeholder demand is reported apart); actual is recorded time entries, whether or not the week is approved. A plan with nothing recorded is null with no_actuals, never zero hours. Time recorded by someone with no allocation on that plan in the window is listed under unplanned_actuals, not folded in and not dropped.",
    }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RoleBody {
    person_ref: String,
    /// The primary role, or `null` to clear it.
    #[serde(default)]
    role: Option<String>,
}

/// `PUT /api/resource-roles` — set (or clear) a person's primary role.
#[debug_handler]
async fn set_role(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<RoleBody>,
) -> Result<Response> {
    if !may_manage_people(&caller) {
        return Err(forbidden(
            "setting a role needs access=admin or role=resource_manager",
        ));
    }
    if !super::governance::valid_ref(&b.person_ref, &["person", "worker"]) {
        return Err(unprocessable("person_ref must be a person:/worker: URN"));
    }
    let role = b.role.as_deref().map(str::trim).filter(|r| !r.is_empty());
    if let Some(r) = role
        && r.chars().count() > 60
    {
        return Err(unprocessable("role is at most 60 characters"));
    }
    let existing = resource_roles::Entity::find()
        .filter(resource_roles::Column::PersonRef.eq(&b.person_ref))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let was = existing.as_ref().map(|e| e.role.clone());
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let pid = match (existing, role) {
        (Some(e), Some(r)) => {
            let pid = e.pid;
            let mut a: resource_roles::ActiveModel = e.into();
            a.role = ActiveValue::set(r.to_string());
            a.update(&txn).await.map_err(db_err)?;
            pid
        }
        (Some(e), None) => {
            let pid = e.pid;
            resource_roles::Entity::delete_by_id(e.id)
                .exec(&txn)
                .await
                .map_err(db_err)?;
            pid
        }
        (None, Some(r)) => {
            resource_roles::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                person_ref: ActiveValue::set(b.person_ref.clone()),
                role: ActiveValue::set(r.to_string()),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?
            .pid
        }
        (None, None) => {
            return format::json(serde_json::json!({ "person_ref": b.person_ref, "role": null }));
        }
    };
    AuditModel::record(
        &txn,
        pid,
        "resource_role_set",
        caller.actor(),
        Some(serde_json::json!({ "person_ref": b.person_ref, "from": was, "to": role })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(serde_json::json!({ "person_ref": b.person_ref, "role": role }))
}

/// `GET /api/resource-roles` — everyone's primary role (a manager), or just
/// the caller's own.
#[debug_handler]
async fn list_roles(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let mut rows = resource_roles::Entity::find()
        .order_by_asc(resource_roles::Column::PersonRef)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if !may_manage_people(&caller) {
        let me = caller
            .claims()
            .map(|c| format!("person:{}", c.sub))
            .unwrap_or_default();
        rows.retain(|r| r.person_ref == me);
    }
    format::json(
        rows.iter()
            .map(|r| serde_json::json!({ "person_ref": r.person_ref, "role": r.role }))
            .collect::<Vec<_>>(),
    )
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ResolveBody {
    person_ref: String,
    /// Resolve even though it would put the person over capacity.
    #[serde(default)]
    confirm: bool,
}

/// `POST /api/plans/{pid}/allocations/{a_pid}/resolve` — give a placeholder a
/// named person. A manager's call; if it would put the person over capacity on
/// any day (and the booking is committed) the answer is 409 with the numbers,
/// and only `confirm=true` goes ahead (audited).
#[debug_handler]
async fn resolve_placeholder(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, a_pid)): Path<(String, String)>,
    Json(b): Json<ResolveBody>,
) -> Result<Response> {
    if !may_manage_people(&caller) {
        return Err(forbidden(
            "resolving a placeholder needs access=admin or role=resource_manager",
        ));
    }
    if !super::governance::valid_ref(&b.person_ref, &["person", "worker"]) {
        return Err(unprocessable("person_ref must be a person:/worker: URN"));
    }
    let item = super::governance::find_item(&ctx, &pid).await?;
    let a_pid = super::parse_pid(&a_pid)?;
    let allocation = vis_models::find_allocation(&ctx.db, a_pid).await?;
    if allocation.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    if !is_placeholder(&allocation.person_ref) {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "this allocation already names a person"),
        ));
    }
    let mut over = false;
    if allocation.booking == "committed" {
        let today = crate::tz::deployment_today();
        let from = allocation.start_date.unwrap_or(today);
        let to = allocation.end_date.unwrap_or(from + chrono::Days::new(365));
        let mut allocs: Vec<crate::availability::Alloc> = vis_models::all_allocations(&ctx.db)
            .await?
            .iter()
            .filter(|a| a.person_ref == b.person_ref)
            .map(|a| crate::availability::Alloc {
                percent: a.percent,
                start: a.start_date,
                end: a.end_date,
            })
            .collect();
        allocs.push(crate::availability::Alloc {
            percent: allocation.percent,
            start: allocation.start_date,
            end: allocation.end_date,
        });
        let ranges = super::availability::ranges_for(&ctx, &b.person_ref).await?;
        let load = crate::availability::load(from, to, &allocs, &ranges);
        over = load.days_over > 0;
        if over && !b.confirm {
            return Err(Error::CustomError(
                StatusCode::CONFLICT,
                ErrorDetail::new(
                    "over_capacity",
                    format!(
                        "giving this placeholder to that person would put them over capacity on {} calendar day(s) (peak {}% against a capacity of {}%); repeat the request with confirm=true to do it anyway",
                        load.days_over, load.peak_demand, load.min_capacity
                    ),
                ),
            ));
        }
    }
    let was = allocation.person_ref.clone();
    let row_pid = allocation.pid;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    // Compare and set (CF-6): the write names the placeholder it saw, so a second resolver that read the same row
    // changes nothing and is told so, rather than replacing the first person.
    let changed = allocations::Entity::update_many()
        .col_expr(
            allocations::Column::PersonRef,
            sea_orm::sea_query::Expr::value(b.person_ref.clone()),
        )
        .filter(allocations::Column::Pid.eq(row_pid))
        .filter(allocations::Column::PersonRef.eq(was.clone()))
        .exec(&txn)
        .await
        .map_err(db_err)?
        .rows_affected;
    if changed == 0 {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "this allocation already names a person"),
        ));
    }
    AuditModel::record(
        &txn,
        row_pid,
        "allocation_resolved",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": b.person_ref, "over_capacity": over })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(
        serde_json::json!({ "pid": row_pid, "person_ref": b.person_ref, "over_capacity": over }),
    )
}

/// The portfolio resource routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/resource-plan", get(resource_plan))
        .add("/resource-plan/hours", get(hours))
        .add("/resource-roles", get(list_roles))
        .add("/resource-roles", put(set_role))
        .add(
            "/plans/{pid}/allocations/{a_pid}/resolve",
            post(resolve_placeholder),
        )
}

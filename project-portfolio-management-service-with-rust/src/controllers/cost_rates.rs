//! Cost rates, task costs and labour cost (PJ-2). The maths is
//! [`crate::cost_rates`]; this is the wiring.
//!
//! * `POST/GET /api/cost-rate-tables`, `POST …/{pid}/rates`,
//!   `GET /api/cost-rates/lookup` — named rate tables per person or role, each
//!   rate **effective from a date** and append-only;
//! * `POST/GET/DELETE …/tasks/{pid}/costs` — material and fixed-cost expenses with
//!   an accrual rule;
//! * `GET …/cost-plan` — the planned cost by period: labour (the work profile, PJ-5,
//!   priced at the rate in force on each day) plus accrued expenses, **per
//!   currency**;
//! * `POST …/labour-costs/record`, `GET …/labour-costs` — the cost of recorded
//!   time, **snapshotted with the rate used**, so a later rate change never
//!   rewrites an actual.
//!
//! **Rates are finance data.** Every route here is available only to a caller the
//! policy admits to the `finance` area (`access=admin` or `finance=cost_rates`,
//! see [`crate::auth::service_rules`]; or when authentication is off, the
//! deployment's posture for every write); anyone else gets `404`, and no rate or
//! cost appears in any other response.

use super::conflict;
use super::db_err;
use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::find_live_task as find_task;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cost_rates::{self as rules, Accrual, Rate};
use crate::models::_entities::{
    cost_rate_tables, cost_rates, resource_roles, task_costs, time_entries, time_entry_costs,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::work_profile as wp;

/// Whether the caller may see or change rates and the costs derived from them.
pub(crate) fn may_see_rates(caller: &MaybeAuthUser) -> bool {
    crate::auth::area_allows(caller, "finance", authentication_verifier::Action::Read)
}

/// A route a caller without the finance attribute must not know exists.
fn finance_only(caller: &MaybeAuthUser) -> Result<()> {
    if may_see_rates(caller) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// The rates that apply to a person, from their own table or their role's.
struct Applicable {
    table_pid: Uuid,
    currency: String,
    source: &'static str,
    rates: Vec<Rate>,
}

async fn rates_of(
    ctx: &AppContext,
    table: &cost_rate_tables::Model,
    source: &'static str,
) -> Result<Applicable> {
    let rows = cost_rates::Entity::find()
        .filter(cost_rates::Column::TablePid.eq(table.pid))
        .order_by_asc(cost_rates::Column::EffectiveFrom)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    Ok(Applicable {
        table_pid: table.pid,
        currency: table.currency.clone(),
        source,
        rates: rows
            .iter()
            .map(|r| Rate {
                effective_from: r.effective_from,
                per_hour_minor: r.per_hour_minor,
            })
            .collect(),
    })
}

/// The rate table for a person: their own named table, else their primary role's.
async fn applicable(ctx: &AppContext, person: &str, name: &str) -> Result<Option<Applicable>> {
    if let Some(t) = cost_rate_tables::Entity::find()
        .filter(cost_rate_tables::Column::Name.eq(name))
        .filter(cost_rate_tables::Column::PersonRef.eq(person))
        .filter(cost_rate_tables::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
    {
        return Ok(Some(rates_of(ctx, &t, "person").await?));
    }
    let role = resource_roles::Entity::find()
        .filter(resource_roles::Column::PersonRef.eq(person))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .map(|r| r.role);
    let Some(role) = role else {
        return Ok(None);
    };
    match cost_rate_tables::Entity::find()
        .filter(cost_rate_tables::Column::Name.eq(name))
        .filter(cost_rate_tables::Column::Role.eq(role))
        .filter(cost_rate_tables::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
    {
        Some(t) => Ok(Some(rates_of(ctx, &t, "role").await?)),
        None => Ok(None),
    }
}

// ───────────────────────── rate tables and rates ─────────────────────────

#[derive(Debug, Deserialize)]
struct TableBody {
    name: String,
    #[serde(default)]
    person_ref: Option<String>,
    #[serde(default)]
    role: Option<String>,
    currency: String,
}

/// `POST /api/cost-rate-tables` — a named rate table for one person or one role.
#[debug_handler]
async fn create_table(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<TableBody>,
) -> Result<Response> {
    finance_only(&caller)?;
    let name = b.name.trim();
    let mut problems = Vec::new();
    if name.is_empty() || name.chars().count() > 40 {
        problems
            .push("name is required (up to 40 characters), e.g. standard or overtime".to_string());
    }
    match (&b.person_ref, &b.role) {
        (Some(p), None) if !super::governance::valid_ref(p, &["person", "worker"]) => {
            problems.push("person_ref must be a person:/worker: URN".to_string());
        }
        (None, Some(r)) if r.trim().is_empty() || r.chars().count() > 60 => {
            problems.push("role must be 1 to 60 characters".to_string());
        }
        (Some(_), None) | (None, Some(_)) => {}
        _ => problems.push("name exactly one of person_ref and role".to_string()),
    }
    let currency = b.currency.trim().to_ascii_uppercase();
    if currency.len() != 3 || !currency.chars().all(|c| c.is_ascii_uppercase()) {
        problems.push("currency must be a 3-letter code such as GBP".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut q = cost_rate_tables::Entity::find()
        .filter(cost_rate_tables::Column::Name.eq(name))
        .filter(cost_rate_tables::Column::DeletedAt.is_null());
    q = match (&b.person_ref, &b.role) {
        (Some(p), _) => q.filter(cost_rate_tables::Column::PersonRef.eq(p.as_str())),
        (_, Some(r)) => q.filter(cost_rate_tables::Column::Role.eq(r.trim())),
        _ => q,
    };
    if q.one(&ctx.db).await.map_err(db_err)?.is_some() {
        return Err(conflict(
            "that person or role already has a table with that name",
        ));
    }
    let row = cost_rate_tables::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(name.to_string()),
        person_ref: ActiveValue::set(b.person_ref.clone()),
        role: ActiveValue::set(b.role.as_ref().map(|r| r.trim().to_string())),
        currency: ActiveValue::set(currency),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "cost_rate_table_created",
        caller.actor(),
        Some(
            serde_json::json!({ "name": row.name, "person_ref": row.person_ref,
            "role": row.role, "currency": row.currency }),
        ),
    )
    .await;
    format::json(serde_json::json!({ "pid": row.pid }))
}

#[derive(Debug, Deserialize)]
struct ListTables {
    #[serde(default)]
    person_ref: Option<String>,
    #[serde(default)]
    role: Option<String>,
}

/// `GET /api/cost-rate-tables` — the tables with their rates, oldest first.
#[debug_handler]
async fn list_tables(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListTables>,
) -> Result<Response> {
    finance_only(&caller)?;
    let mut query =
        cost_rate_tables::Entity::find().filter(cost_rate_tables::Column::DeletedAt.is_null());
    if let Some(p) = &q.person_ref {
        query = query.filter(cost_rate_tables::Column::PersonRef.eq(p.as_str()));
    }
    if let Some(r) = &q.role {
        query = query.filter(cost_rate_tables::Column::Role.eq(r.as_str()));
    }
    let tables = query
        .order_by_asc(cost_rate_tables::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = Vec::new();
    for t in &tables {
        let rates = rates_of(&ctx, t, "table").await?;
        out.push(serde_json::json!({
            "pid": t.pid, "name": t.name, "person_ref": t.person_ref, "role": t.role,
            "currency": t.currency,
            "rates": rates.rates.iter().map(|r| serde_json::json!({
                "effective_from": r.effective_from, "per_hour_minor": r.per_hour_minor })).collect::<Vec<_>>(),
        }));
    }
    format::json(serde_json::json!({ "tables": out }))
}

#[derive(Debug, Deserialize)]
struct RateBody {
    effective_from: NaiveDate,
    /// Minor units of the table's currency per hour.
    per_hour_minor: i64,
}

/// `POST /api/cost-rate-tables/{pid}/rates` — add a rate, effective from a date.
/// Rates are append-only: there is no edit and no delete, and a date can carry
/// one rate (409), so what a day cost is what it was said to cost.
#[debug_handler]
async fn add_rate(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<RateBody>,
) -> Result<Response> {
    finance_only(&caller)?;
    let pid = super::parse_pid(&pid)?;
    let table = cost_rate_tables::Entity::find()
        .filter(cost_rate_tables::Column::Pid.eq(pid))
        .filter(cost_rate_tables::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if b.per_hour_minor < 0 {
        return Err(unprocessable("per_hour_minor must not be negative"));
    }
    if b.effective_from < NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default()
        || chrono::Datelike::year(&b.effective_from) > 2100
    {
        return Err(unprocessable(
            "effective_from must be between 1970 and 2100",
        ));
    }
    if cost_rates::Entity::find()
        .filter(cost_rates::Column::TablePid.eq(table.pid))
        .filter(cost_rates::Column::EffectiveFrom.eq(b.effective_from))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .is_some()
    {
        return Err(conflict(
            "that table already has a rate effective from that date; add a later date instead",
        ));
    }
    let row = cost_rates::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        table_pid: ActiveValue::set(table.pid),
        effective_from: ActiveValue::set(b.effective_from),
        per_hour_minor: ActiveValue::set(b.per_hour_minor),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        table.pid,
        "cost_rate_added",
        caller.actor(),
        Some(serde_json::json!({ "effective_from": row.effective_from, "per_hour_minor": row.per_hour_minor,
            "currency": table.currency })),
    )
    .await;
    format::json(serde_json::json!({ "pid": row.pid, "table_pid": table.pid,
        "effective_from": row.effective_from, "per_hour_minor": row.per_hour_minor, "currency": table.currency }))
}

#[derive(Debug, Deserialize)]
struct LookupParams {
    person_ref: String,
    date: NaiveDate,
    #[serde(default)]
    table: Option<String>,
}

/// `GET /api/cost-rates/lookup` — the rate in force for a person on a date: their
/// own table's, else their role's. No rate is `null` with `no_rate`, never zero.
#[debug_handler]
async fn lookup(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<LookupParams>,
) -> Result<Response> {
    finance_only(&caller)?;
    let name = q.table.as_deref().unwrap_or("standard");
    let found = applicable(&ctx, &q.person_ref, name).await?;
    let hit = found.as_ref().and_then(|a| {
        rules::rate_on(&a.rates, q.date).map(|rate| {
            let from = a.rates.iter().filter(|r| r.effective_from <= q.date).map(|r| r.effective_from).max();
            serde_json::json!({ "per_hour_minor": rate, "currency": a.currency, "effective_from": from,
                "table_pid": a.table_pid, "source": a.source })
        })
    });
    format::json(serde_json::json!({
        "person_ref": q.person_ref, "date": q.date, "table": name,
        "rate": hit,
        "reason": if hit.is_none() { Some("no_rate") } else { None },
    }))
}

// ───────────────────────── task costs ─────────────────────────

#[derive(Debug, Deserialize)]
struct TaskCostBody {
    /// `material` or `fixed`.
    kind: String,
    description: String,
    amount_minor: i64,
    currency: String,
    /// `start`, `prorated` or `end`.
    accrual: String,
}

/// `POST /api/plans/{pid}/tasks/{t_pid}/costs` — a material or fixed-cost expense.
#[debug_handler]
async fn add_task_cost(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<TaskCostBody>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    let currency = b.currency.trim().to_ascii_uppercase();
    let mut problems = Vec::new();
    if !["material", "fixed"].contains(&b.kind.as_str()) {
        problems.push("kind must be material or fixed".to_string());
    }
    if b.description.trim().is_empty() || b.description.chars().count() > 200 {
        problems.push("description is required (up to 200 characters)".to_string());
    }
    if b.amount_minor <= 0 {
        problems.push("amount_minor must be above zero".to_string());
    }
    if currency.len() != 3 || !currency.chars().all(|c| c.is_ascii_uppercase()) {
        problems.push("currency must be a 3-letter code such as GBP".to_string());
    }
    if Accrual::parse(&b.accrual).is_none() {
        problems.push("accrual must be start, prorated or end".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let row = task_costs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        task_pid: ActiveValue::set(task.pid),
        kind: ActiveValue::set(b.kind.clone()),
        description: ActiveValue::set(b.description.trim().to_string()),
        amount_minor: ActiveValue::set(b.amount_minor),
        currency: ActiveValue::set(currency),
        accrual: ActiveValue::set(b.accrual.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "task_cost_added",
        caller.actor(),
        Some(serde_json::json!({ "task_pid": task.pid, "kind": row.kind, "amount_minor": row.amount_minor,
            "currency": row.currency, "accrual": row.accrual })),
    )
    .await;
    format::json(serde_json::json!({ "pid": row.pid }))
}

/// `GET /api/plans/{pid}/tasks/{t_pid}/costs` — the task's expenses.
#[debug_handler]
async fn list_task_costs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    let rows = task_costs::Entity::find()
        .filter(task_costs::Column::TaskPid.eq(task.pid))
        .filter(task_costs::Column::DeletedAt.is_null())
        .order_by_asc(task_costs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(
        serde_json::json!({ "costs": rows.iter().map(|c| serde_json::json!({
        "pid": c.pid, "kind": c.kind, "description": c.description,
        "amount_minor": c.amount_minor, "currency": c.currency, "accrual": c.accrual })).collect::<Vec<_>>() }),
    )
}

/// `DELETE /api/plans/{pid}/tasks/{t_pid}/costs/{c_pid}` — soft-delete an expense.
#[debug_handler]
async fn delete_task_cost(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid, c_pid)): Path<(String, String, String)>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    let c_pid = super::parse_pid(&c_pid)?;
    let row = task_costs::Entity::find()
        .filter(task_costs::Column::Pid.eq(c_pid))
        .filter(task_costs::Column::TaskPid.eq(task.pid))
        .filter(task_costs::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let row_pid = row.pid;
    let mut a: task_costs::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(&ctx.db, row_pid, "task_cost_removed", caller.actor(), None).await;
    format::empty_json()
}

// ───────────────────────── the planned cost ─────────────────────────

fn money_buckets(
    per_day: &[(NaiveDate, i64)],
    periods: &[crate::portfolio_resources::Period],
) -> Vec<serde_json::Value> {
    wp::bucket(per_day, periods)
        .0
        .iter()
        .map(|b| serde_json::json!({ "from": b.from, "to": b.to, "minor": b.minutes }))
        .collect()
}

/// Minor units by day.
type DayMoney = BTreeMap<NaiveDate, i64>;

fn add_days(into: &mut BTreeMap<NaiveDate, i64>, days: &[(NaiveDate, i64)]) {
    for (d, m) in days {
        *into.entry(*d).or_default() += m;
    }
}

#[derive(Debug, Deserialize)]
struct CostPlanParams {
    #[serde(flatten)]
    window: super::assignments::ProfileParams,
    /// The rate table to price labour with (default `standard`).
    #[serde(default)]
    table: Option<String>,
}

/// `GET /api/plans/{pid}/cost-plan` — the planned cost by period, per currency.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn cost_plan(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<CostPlanParams>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let table = q.table.clone().unwrap_or_else(|| "standard".to_string());
    let net = super::cpm::load_network(&ctx, plan.pid).await?;
    let analysis = net.analyze(0).ok();
    let work = super::assignments::plan_work(&ctx, plan.pid).await?;
    let work_by: HashMap<Uuid, &super::assignments::TaskWork> =
        work.iter().map(|w| (w.pid, w)).collect();

    // Everything that will be placed on a day, to choose the window from.
    let mut every_day: Vec<(NaiveDate, i64)> = Vec::new();
    let mut labour: Vec<(Uuid, String, String, rules::Priced, Applicable)> = Vec::new();
    let mut unrated: Vec<serde_json::Value> = Vec::new();
    let mut applicable_cache: HashMap<String, Option<usize>> = HashMap::new();
    let mut tables: Vec<Applicable> = Vec::new();
    for w in &work {
        if w.unplaced.is_some() {
            continue;
        }
        let Some(person) = &w.assignee else { continue };
        let ix = if let Some(ix) = applicable_cache.get(person) {
            *ix
        } else {
            let found = applicable(&ctx, person, &table).await?;
            let ix = found.map(|a| {
                tables.push(a);
                tables.len() - 1
            });
            applicable_cache.insert(person.clone(), ix);
            ix
        };
        every_day.extend(w.per_day.iter().map(|(d, _)| (*d, 0)));
        match ix {
            None => unrated.push(serde_json::json!({ "pid": w.pid, "title": w.title,
                "work_minutes": w.work_minutes, "reason": "no_rate" })),
            Some(i) => {
                let a = &tables[i];
                let priced = rules::price_days(&w.per_day, |d| rules::rate_on(&a.rates, d));
                if priced.unrated_minutes > 0 {
                    unrated.push(serde_json::json!({ "pid": w.pid, "title": w.title,
                        "work_minutes": priced.unrated_minutes, "reason": "no_rate_on_some_days" }));
                }
                labour.push((
                    w.pid,
                    w.title.clone(),
                    a.currency.clone(),
                    priced,
                    Applicable {
                        table_pid: a.table_pid,
                        currency: a.currency.clone(),
                        source: a.source,
                        rates: Vec::new(),
                    },
                ));
            }
        }
    }
    // Work with an assignee but none placed, or no assignee at all, is said too.
    let unassigned: Vec<serde_json::Value> = work
        .iter()
        .filter(|w| w.assignee.is_none())
        .map(|w| serde_json::json!({ "pid": w.pid, "title": w.title, "work_minutes": w.work_minutes, "reason": "no_assignee" }))
        .collect();

    // Expenses, accrued over each task's business days.
    let costs = task_costs::Entity::find()
        .filter(task_costs::Column::DeletedAt.is_null())
        .filter(
            task_costs::Column::TaskPid.is_in(net.rows.iter().map(|t| t.pid).collect::<Vec<_>>()),
        )
        .order_by_asc(task_costs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut fixed: Vec<(task_costs::Model, Vec<(NaiveDate, i64)>)> = Vec::new();
    let mut unplaced: Vec<serde_json::Value> = Vec::new();
    for c in &costs {
        let ix = net.rows.iter().position(|t| t.pid == c.task_pid);
        let days = ix.and_then(|i| super::assignments::working_days_of(&net, analysis.as_ref(), i));
        let parts = days.as_ref().and_then(|d| {
            rules::accrue(
                c.amount_minor,
                d,
                Accrual::parse(&c.accrual).unwrap_or(Accrual::Prorated),
            )
        });
        match parts {
            Some(p) if !p.is_empty() => {
                every_day.extend(p.iter().map(|(d, _)| (*d, 0)));
                fixed.push((c.clone(), p));
            }
            _ => unplaced.push(
                serde_json::json!({ "cost_pid": c.pid, "task_pid": c.task_pid,
                "amount_minor": c.amount_minor, "currency": c.currency,
                "reason": if days.is_none() { "unscheduled" } else { "no_working_days" } }),
            ),
        }
    }
    if labour.is_empty() && fixed.is_empty() {
        return format::json(serde_json::json!({
            "plan_pid": plan.pid, "currencies": [], "tasks": [],
            "unrated": unrated, "unassigned": unassigned, "unplaced": unplaced,
            "reason": "no_costs",
            "note": "Nothing could be costed: no task with work has a rate, and no expense can be placed on a day.",
        }));
    }
    let refs: Vec<&(NaiveDate, i64)> = every_day.iter().collect();
    let (from, to, grain, periods) = super::assignments::window(&q.window, &refs)?;

    let mut by_currency: BTreeMap<String, (DayMoney, DayMoney)> = BTreeMap::new();
    let mut task_rows: BTreeMap<Uuid, serde_json::Value> = BTreeMap::new();
    let titles: HashMap<Uuid, &str> = net.rows.iter().map(|t| (t.pid, t.title.as_str())).collect();
    for (pid, title, currency, priced, _) in &labour {
        add_days(
            &mut by_currency.entry(currency.clone()).or_default().0,
            &priced.days,
        );
        let entry = task_rows
            .entry(*pid)
            .or_insert_with(|| serde_json::json!({ "pid": pid, "title": title, "fixed": [] }));
        entry["labour"] = serde_json::json!({ "currency": currency, "total_minor": priced.total_minor,
            "unrated_minutes": priced.unrated_minutes, "buckets": money_buckets(&priced.days, &periods) });
        let _ = work_by.get(pid);
    }
    for (c, parts) in &fixed {
        add_days(
            &mut by_currency.entry(c.currency.clone()).or_default().1,
            parts,
        );
        let entry = task_rows.entry(c.task_pid).or_insert_with(|| {
            serde_json::json!({
            "pid": c.task_pid, "title": titles.get(&c.task_pid), "fixed": [] })
        });
        if let Some(arr) = entry["fixed"].as_array_mut() {
            arr.push(
                serde_json::json!({ "cost_pid": c.pid, "kind": c.kind, "description": c.description,
                "currency": c.currency, "amount_minor": c.amount_minor, "accrual": c.accrual,
                "buckets": money_buckets(parts, &periods) }),
            );
        }
    }
    let currencies: Vec<serde_json::Value> = by_currency
        .iter()
        .map(|(cur, (lab, fix))| {
            let lab_days: Vec<(NaiveDate, i64)> = lab.iter().map(|(d, m)| (*d, *m)).collect();
            let fix_days: Vec<(NaiveDate, i64)> = fix.iter().map(|(d, m)| (*d, *m)).collect();
            let mut both = lab.clone();
            for (d, m) in fix {
                *both.entry(*d).or_default() += m;
            }
            let both_days: Vec<(NaiveDate, i64)> = both.iter().map(|(d, m)| (*d, *m)).collect();
            serde_json::json!({
                "currency": cur,
                "labour": { "total_minor": lab.values().sum::<i64>(), "buckets": money_buckets(&lab_days, &periods) },
                "fixed": { "total_minor": fix.values().sum::<i64>(), "buckets": money_buckets(&fix_days, &periods) },
                "total": { "total_minor": both.values().sum::<i64>(), "buckets": money_buckets(&both_days, &periods) },
            })
        })
        .collect();
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "from": from, "to": to,
        "period": match grain { wp::Grain::Day => "day", wp::Grain::Week => "week", wp::Grain::Month => "month" },
        "table": table,
        "currencies": currencies,
        "tasks": task_rows.into_values().collect::<Vec<_>>(),
        "unrated": unrated, "unassigned": unassigned, "unplaced": unplaced,
        "note": "Labour is each task's work (spread by its contour over its business days) priced day by day at the rate in force that day, from the assignee's own rate table or their role's; daily costs sum exactly to the rounded total. Expenses accrue over the task's business days by their rule (start, prorated, end) in whole minor units that sum to the amount. Work with no rate (or on days before the first rate) is listed under unrated and costs nothing here — unknown, never zero. Each currency stands alone and is never added to another. Derived on read.",
    }))
}

/// A plan's planned labour cost by currency, for a network already in hand (the
/// what-if sandbox prices the original and the edited copy).
pub(crate) struct LabourTotals {
    /// Total minor units per currency, each priced day by day at the rate in force.
    pub by_currency: BTreeMap<String, i64>,
    /// Minutes of work that no rate covers (unknown, never zero).
    pub unrated_minutes: i64,
}

pub(crate) async fn labour_totals(
    ctx: &AppContext,
    net: &super::cpm::Network,
    analysis: Option<&crate::cpm::CpmResult>,
) -> Result<LabourTotals> {
    let mut out = LabourTotals {
        by_currency: BTreeMap::new(),
        unrated_minutes: 0,
    };
    let mut cache: HashMap<String, Option<Applicable>> = HashMap::new();
    for w in super::assignments::work_of(net, analysis) {
        if w.unplaced.is_some() {
            continue;
        }
        let Some(person) = &w.assignee else {
            out.unrated_minutes += w.work_minutes;
            continue;
        };
        if !cache.contains_key(person) {
            let a = applicable(ctx, person, "standard").await?;
            cache.insert(person.clone(), a);
        }
        match cache.get(person).and_then(Option::as_ref) {
            None => out.unrated_minutes += w.work_minutes,
            Some(a) => {
                let priced = rules::price_days(&w.per_day, |d| rules::rate_on(&a.rates, d));
                out.unrated_minutes += priced.unrated_minutes;
                *out.by_currency.entry(a.currency.clone()).or_default() += priced.total_minor;
            }
        }
    }
    Ok(out)
}

// ───────────────────────── labour cost actuals ─────────────────────────

#[derive(Debug, Deserialize)]
struct RecordParams {
    #[serde(default)]
    table: Option<String>,
}

/// `POST /api/plans/{pid}/labour-costs/record` — price every recorded time entry
/// that has no recorded cost yet, at the rate in force the day it was worked, and
/// store the rate **with** the cost. Entries already recorded are never touched:
/// a later rate change never rewrites an actual. Entries with no rate are listed
/// and left unrecorded (cost unknown, not zero).
#[debug_handler]
async fn record_labour(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<RecordParams>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let table = q.table.unwrap_or_else(|| "standard".to_string());
    let entries = time_entries::Entity::find()
        .filter(time_entries::Column::PlanPid.eq(plan.pid))
        .filter(time_entries::Column::DeletedAt.is_null())
        .order_by_asc(time_entries::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let done: HashSet<Uuid> = time_entry_costs::Entity::find()
        .filter(time_entry_costs::Column::PlanPid.eq(plan.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|c| c.time_entry_pid)
        .collect();
    let mut cache: HashMap<String, Option<Applicable>> = HashMap::new();
    let (mut recorded, mut already) = (0_usize, 0_usize);
    let mut no_rate = Vec::new();
    for e in &entries {
        if done.contains(&e.pid) {
            already += 1;
            continue;
        }
        if !cache.contains_key(&e.actor_ref) {
            let a = applicable(&ctx, &e.actor_ref, &table).await?;
            cache.insert(e.actor_ref.clone(), a);
        }
        let hit = cache
            .get(&e.actor_ref)
            .and_then(Option::as_ref)
            .and_then(|a| {
                rules::rate_on(&a.rates, e.spent_on).map(|r| (r, a.table_pid, a.currency.clone()))
            });
        let Some((rate, table_pid, currency)) = hit else {
            no_rate.push(
                serde_json::json!({ "time_entry_pid": e.pid, "minutes": e.minutes,
                "spent_on": e.spent_on, "reason": "no_rate" }),
            );
            continue;
        };
        time_entry_costs::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            time_entry_pid: ActiveValue::set(e.pid),
            plan_pid: ActiveValue::set(plan.pid),
            person_ref: ActiveValue::set(e.actor_ref.clone()),
            table_pid: ActiveValue::set(table_pid),
            spent_on: ActiveValue::set(e.spent_on),
            minutes: ActiveValue::set(e.minutes),
            per_hour_minor: ActiveValue::set(rate),
            currency: ActiveValue::set(currency),
            cost_minor: ActiveValue::set(rules::labour_cost_minor(e.minutes, rate)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?;
        recorded += 1;
    }
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "labour_costs_recorded",
        caller.actor(),
        Some(
            serde_json::json!({ "recorded": recorded, "already_recorded": already,
            "no_rate": no_rate.len(), "table": table }),
        ),
    )
    .await;
    format::json(serde_json::json!({
        "recorded": recorded, "already_recorded": already, "no_rate": no_rate,
        "note": "A recorded cost keeps the rate it was priced at. Recording again never changes one; it prices only entries that have none yet.",
    }))
}

/// `GET /api/plans/{pid}/labour-costs` — the recorded labour cost, per currency
/// and never across, with the entries still unpriced counted apart.
#[debug_handler]
async fn labour_costs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let snaps = time_entry_costs::Entity::find()
        .filter(time_entry_costs::Column::PlanPid.eq(plan.pid))
        .order_by_asc(time_entry_costs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let entries = time_entries::Entity::find()
        .filter(time_entries::Column::PlanPid.eq(plan.pid))
        .filter(time_entries::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let priced: HashSet<Uuid> = snaps.iter().map(|s| s.time_entry_pid).collect();
    let unpriced: Vec<&time_entries::Model> = entries
        .iter()
        .filter(|e| !priced.contains(&e.pid))
        .collect();
    let mut by_cur: BTreeMap<&str, (i64, i64, usize)> = BTreeMap::new();
    for s in &snaps {
        let e = by_cur.entry(&s.currency).or_default();
        e.0 += s.cost_minor;
        e.1 += s.minutes;
        e.2 += 1;
    }
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "currencies": by_cur.iter().map(|(c, (cost, minutes, n))| serde_json::json!({
            "currency": c, "cost_minor": cost, "minutes": minutes, "entries": n })).collect::<Vec<_>>(),
        "entries": snaps.iter().map(|s| serde_json::json!({
            "time_entry_pid": s.time_entry_pid, "person_ref": s.person_ref, "spent_on": s.spent_on,
            "minutes": s.minutes, "per_hour_minor": s.per_hour_minor, "currency": s.currency,
            "cost_minor": s.cost_minor })).collect::<Vec<_>>(),
        "unpriced": { "entries": unpriced.len(), "minutes": unpriced.iter().map(|e| e.minutes).sum::<i64>(),
                      "reason": if unpriced.is_empty() { None } else { Some("not_recorded_or_no_rate") } },
        "note": "Each entry's cost carries the rate it was priced at. Entries not yet recorded, or with no rate when recorded, are counted under unpriced: their cost is unknown, not zero. Currencies are never added together.",
    }))
}

/// The cost-rate routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/cost-rate-tables", post(create_table))
        .add("/cost-rate-tables", get(list_tables))
        .add("/cost-rate-tables/{pid}/rates", post(add_rate))
        .add("/cost-rates/lookup", get(lookup))
        .add("/plans/{pid}/tasks/{t_pid}/costs", post(add_task_cost))
        .add("/plans/{pid}/tasks/{t_pid}/costs", get(list_task_costs))
        .add(
            "/plans/{pid}/tasks/{t_pid}/costs/{c_pid}",
            delete(delete_task_cost),
        )
        .add("/plans/{pid}/cost-plan", get(cost_plan))
        .add("/plans/{pid}/labour-costs/record", post(record_labour))
        .add("/plans/{pid}/labour-costs", get(labour_costs))
}

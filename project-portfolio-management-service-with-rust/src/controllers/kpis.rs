//! KPIs: standing measures with a target and a red line, and their readings.
//! The rules are [`crate::kpi`]; this is the wiring.
//!
//! **Status is derived on read** from the latest reading and never stored, so a
//! corrected reading corrects every figure that rests on it. A KPI with no
//! reading is `no_data`, not red and not green. Readings are append-only: a
//! correction is a newer reading for the same day.

use super::{db_err, unprocessable};
use std::collections::HashMap;

use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::kpi::{self, Direction, Status, Summary};
use crate::models::_entities::{kpi_readings, kpis, objectives};
use crate::models::audit_logs::Model as AuditModel;

const CADENCES: &[&str] = &["daily", "weekly", "monthly", "quarterly"];

/// Problems with the fields shared by create and update.
fn field_problems(
    name: &str,
    unit: Option<&str>,
    cadence: Option<&str>,
    owner_ref: Option<&str>,
) -> Vec<String> {
    let mut out = Vec::new();
    if name.trim().is_empty() || name.len() > crate::validation::MAX_TEXT_LEN {
        out.push("name is required (and capped)".to_string());
    }
    if unit.is_some_and(|u| u.chars().count() > 40) {
        out.push("unit is at most 40 characters".to_string());
    }
    if cadence.is_some_and(|c| !CADENCES.contains(&c)) {
        out.push("cadence must be daily, weekly, monthly or quarterly".to_string());
    }
    if owner_ref.is_some_and(|o| !super::governance::valid_ref(o, &["person", "worker"])) {
        out.push("owner_ref must be a person:/worker: URN".to_string());
    }
    out
}

async fn objective_exists(ctx: &AppContext, pid: Uuid) -> Result<bool> {
    Ok(objectives::Entity::find()
        .filter(objectives::Column::Pid.eq(pid))
        .filter(objectives::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .is_some())
}

/// The two latest readings of a KPI, newest first.
fn latest_two(readings: &[kpi_readings::Model], kpi_pid: Uuid) -> Vec<&kpi_readings::Model> {
    let mut mine: Vec<&kpi_readings::Model> =
        readings.iter().filter(|r| r.kpi_pid == kpi_pid).collect();
    mine.sort_by_key(|r| std::cmp::Reverse((r.observed_on, r.id)));
    mine.truncate(2);
    mine
}

/// One KPI as the API shows it: the definition and everything derived.
fn view(
    k: &kpis::Model,
    readings: &[kpi_readings::Model],
    as_of: NaiveDate,
) -> (Status, serde_json::Value) {
    let direction = Direction::parse(&k.direction).unwrap_or(Direction::HigherBetter);
    let two = latest_two(readings, k.pid);
    let latest = two.first().copied();
    let previous = two.get(1).copied();
    let st = kpi::status_of(
        direction,
        k.target_value,
        k.red_value,
        latest.map(|r| r.value),
    );
    let count = readings.iter().filter(|r| r.kpi_pid == k.pid).count();
    (
        st,
        serde_json::json!({
            "kpi": k,
            "status": st,
            "latest": latest.map(|r| serde_json::json!({
                "value": r.value, "observed_on": r.observed_on, "note": r.note })),
            "trend": kpi::trend(direction, previous.map(|r| r.value), latest.map(|r| r.value)),
            "stale": kpi::stale(k.cadence.as_deref(), latest.map(|r| r.observed_on), as_of),
            "readings": count,
        }),
    )
}

async fn load_readings(ctx: &AppContext, pids: &[Uuid]) -> Result<Vec<kpi_readings::Model>> {
    if pids.is_empty() {
        return Ok(Vec::new());
    }
    kpi_readings::Entity::find()
        .filter(kpi_readings::Column::KpiPid.is_in(pids.to_vec()))
        .order_by_asc(kpi_readings::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

async fn find_kpi(ctx: &AppContext, caller: &MaybeAuthUser, raw: &str) -> Result<kpis::Model> {
    let pid = super::parse_pid(raw)?;
    let k = kpis::Entity::find()
        .filter(kpis::Column::Pid.eq(pid))
        .filter(kpis::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    // A KPI on a plan the caller cannot read is not there for them.
    if let Some(plan) = k.plan_pid
        && !super::plan_access::readable_plans(ctx, caller, &[plan])
            .await
            .contains(&plan)
    {
        return Err(Error::NotFound);
    }
    Ok(k)
}

/// `POST /api/kpis` body.
#[derive(Debug, Deserialize)]
struct CreateBody {
    /// The plan it measures; omit for a portfolio-level KPI.
    #[serde(default)]
    plan_pid: Option<Uuid>,
    /// The objective it supports.
    #[serde(default)]
    objective_pid: Option<Uuid>,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    unit: Option<String>,
    /// `higher_better` or `lower_better`.
    direction: String,
    target_value: i64,
    /// The line beyond which it is red; on the worse side of the target.
    red_value: i64,
    #[serde(default)]
    cadence: Option<String>,
    #[serde(default)]
    owner_ref: Option<String>,
}

/// `POST /api/kpis` — define a KPI.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<CreateBody>,
) -> Result<Response> {
    let mut problems = field_problems(
        &b.name,
        b.unit.as_deref(),
        b.cadence.as_deref(),
        b.owner_ref.as_deref(),
    );
    let direction = Direction::parse(&b.direction);
    match direction {
        None => problems.push("direction must be higher_better or lower_better".to_string()),
        Some(d) => {
            if let Some(p) = kpi::threshold_problem(d, b.target_value, b.red_value) {
                problems.push(p.to_string());
            }
        }
    }
    if let Some(o) = b.objective_pid
        && !objective_exists(&ctx, o).await?
    {
        problems.push("objective_pid is not an objective".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    if let Some(plan) = b.plan_pid {
        super::governance::find_item(&ctx, &plan.to_string()).await?;
    }
    let row = kpis::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(b.plan_pid),
        objective_pid: ActiveValue::set(b.objective_pid),
        name: ActiveValue::set(b.name.trim().to_string()),
        description: ActiveValue::set(b.description.clone()),
        unit: ActiveValue::set(b.unit.clone()),
        direction: ActiveValue::set(b.direction.clone()),
        target_value: ActiveValue::set(b.target_value),
        red_value: ActiveValue::set(b.red_value),
        cadence: ActiveValue::set(b.cadence.clone()),
        owner_ref: ActiveValue::set(b.owner_ref.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(&ctx.db, row.pid, "kpi_defined", caller.actor(), None).await;
    format::json(serde_json::json!({ "pid": row.pid }))
}

#[derive(Debug, Deserialize)]
struct ListParams {
    /// Only this plan's KPIs.
    #[serde(default)]
    plan: Option<Uuid>,
    /// Only KPIs supporting this objective.
    #[serde(default)]
    objective: Option<Uuid>,
    /// Only this status: `green`, `amber`, `red`, `no_data`.
    #[serde(default)]
    status: Option<String>,
    /// The date staleness is judged against (default today).
    #[serde(default)]
    as_of: Option<NaiveDate>,
}

/// `GET /api/kpis` — KPIs with their derived status, red first, then amber, then
/// no data, then green; and a summary of the whole (unfiltered by status) set.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let wanted = match q.status.as_deref() {
        None => None,
        Some("green") => Some(Status::Green),
        Some("amber") => Some(Status::Amber),
        Some("red") => Some(Status::Red),
        Some("no_data") => Some(Status::NoData),
        Some(_) => return Err(unprocessable("status must be green, amber, red or no_data")),
    };
    let as_of = q.as_of.unwrap_or_else(crate::tz::utc_today);
    let mut query = kpis::Entity::find().filter(kpis::Column::DeletedAt.is_null());
    if let Some(p) = q.plan {
        query = query.filter(kpis::Column::PlanPid.eq(p));
    }
    if let Some(o) = q.objective {
        query = query.filter(kpis::Column::ObjectivePid.eq(o));
    }
    let mut rows = query
        .order_by_asc(kpis::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    // Plan KPIs on plans the caller cannot read are left out.
    let mut plans: Vec<Uuid> = rows.iter().filter_map(|k| k.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    rows.retain(|k| k.plan_pid.is_none_or(|p| readable.contains(&p)));
    let pids: Vec<Uuid> = rows.iter().map(|k| k.pid).collect();
    let readings = load_readings(&ctx, &pids).await?;
    let mut summary = Summary::default();
    let mut shown: Vec<(Status, serde_json::Value)> = Vec::new();
    for k in &rows {
        let (st, v) = view(k, &readings, as_of);
        summary.add(st);
        if wanted.is_none_or(|w| w == st) {
            shown.push((st, v));
        }
    }
    let rank = |s: Status| match s {
        Status::Red => 0,
        Status::Amber => 1,
        Status::NoData => 2,
        Status::Green => 3,
    };
    shown.sort_by_key(|(s, _)| rank(*s));
    format::json(serde_json::json!({
        "as_of": as_of,
        "kpis": shown.into_iter().map(|(_, v)| v).collect::<Vec<_>>(),
        "summary": summary,
        "note": "Status is derived from the latest reading: green at or better than the target, red at or worse than the red line, amber between. A KPI with no reading is no_data, not red and not green. stale is set when a KPI with a cadence has no reading within twice that interval. Summary counts every KPI visible to the caller, whatever the status filter.",
    }))
}

/// `GET /api/kpis/{pid}` — one KPI with its derived status and its readings.
#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let k = find_kpi(&ctx, &caller, &pid).await?;
    let readings = load_readings(&ctx, &[k.pid]).await?;
    let (_, mut v) = view(&k, &readings, crate::tz::utc_today());
    let mut history: Vec<&kpi_readings::Model> = readings.iter().collect();
    history.sort_by_key(|r| std::cmp::Reverse((r.observed_on, r.id)));
    v["history"] = serde_json::json!(
        history
            .iter()
            .map(|r| serde_json::json!({
        "pid": r.pid, "observed_on": r.observed_on, "value": r.value,
        "note": r.note, "actor": r.actor }))
            .collect::<Vec<_>>()
    );
    format::json(v)
}

/// `PUT /api/kpis/{pid}` body. The `direction` and the plan are not editable:
/// changing which way is good would silently re-colour every past reading.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateBody {
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    unit: Option<String>,
    target_value: i64,
    red_value: i64,
    #[serde(default)]
    cadence: Option<String>,
    #[serde(default)]
    owner_ref: Option<String>,
    #[serde(default)]
    objective_pid: Option<Uuid>,
}

/// `PUT /api/kpis/{pid}` — edit a KPI's name, thresholds, cadence and owner.
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<UpdateBody>,
) -> Result<Response> {
    let k = find_kpi(&ctx, &caller, &pid).await?;
    let mut problems = field_problems(
        &b.name,
        b.unit.as_deref(),
        b.cadence.as_deref(),
        b.owner_ref.as_deref(),
    );
    if let Some(d) = Direction::parse(&k.direction)
        && let Some(p) = kpi::threshold_problem(d, b.target_value, b.red_value)
    {
        problems.push(p.to_string());
    }
    if let Some(o) = b.objective_pid
        && !objective_exists(&ctx, o).await?
    {
        problems.push("objective_pid is not an objective".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let was = serde_json::json!({ "name": k.name, "target_value": k.target_value,
        "red_value": k.red_value, "cadence": k.cadence, "owner_ref": k.owner_ref });
    let mut a: kpis::ActiveModel = k.into();
    a.name = ActiveValue::set(b.name.trim().to_string());
    a.description = ActiveValue::set(b.description.clone());
    a.unit = ActiveValue::set(b.unit.clone());
    a.target_value = ActiveValue::set(b.target_value);
    a.red_value = ActiveValue::set(b.red_value);
    a.cadence = ActiveValue::set(b.cadence.clone());
    a.owner_ref = ActiveValue::set(b.owner_ref.clone());
    a.objective_pid = ActiveValue::set(b.objective_pid);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "kpi_updated",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": { "name": row.name, "target_value": row.target_value,
            "red_value": row.red_value, "cadence": row.cadence, "owner_ref": row.owner_ref } })),
    )
    .await;
    format::json(row)
}

/// `DELETE /api/kpis/{pid}` — soft-delete a KPI; its readings are kept.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let k = find_kpi(&ctx, &caller, &pid).await?;
    let row_pid = k.pid;
    let mut a: kpis::ActiveModel = k.into();
    a.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(&ctx.db, row_pid, "kpi_deleted", caller.actor(), None).await;
    format::empty_json()
}

/// `POST /api/kpis/{pid}/readings` body.
#[derive(Debug, Deserialize)]
struct ReadingBody {
    value: i64,
    /// The day it was observed (default today; not in the future).
    #[serde(default)]
    observed_on: Option<NaiveDate>,
    #[serde(default)]
    note: Option<String>,
}

/// `POST /api/kpis/{pid}/readings` — append a reading. The latest by observed
/// day (then by entry order) is the one the status rests on, so a correction is
/// simply a newer reading for the same day.
#[debug_handler]
async fn add_reading(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ReadingBody>,
) -> Result<Response> {
    let k = find_kpi(&ctx, &caller, &pid).await?;
    let today = crate::tz::utc_today();
    let on = b.observed_on.unwrap_or(today);
    if on > today {
        return Err(unprocessable("observed_on is in the future"));
    }
    if on < NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default() {
        return Err(unprocessable("observed_on is too far in the past"));
    }
    if b.note
        .as_deref()
        .is_some_and(|n| n.len() > crate::validation::MAX_TEXT_LEN)
    {
        return Err(unprocessable("note is too long"));
    }
    let row = kpi_readings::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        kpi_pid: ActiveValue::set(k.pid),
        observed_on: ActiveValue::set(on),
        value: ActiveValue::set(b.value),
        note: ActiveValue::set(b.note.clone()),
        actor: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        k.pid,
        "kpi_reading_added",
        caller.actor(),
        Some(serde_json::json!({ "value": b.value, "observed_on": on })),
    )
    .await;
    let readings = load_readings(&ctx, &[k.pid]).await?;
    let (_, v) = view(&k, &readings, today);
    format::json(serde_json::json!({ "reading_pid": row.pid, "kpi": v }))
}

/// `GET /api/kpis/{pid}/readings` — every reading, newest first.
#[debug_handler]
async fn list_readings(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let k = find_kpi(&ctx, &caller, &pid).await?;
    let mut readings = load_readings(&ctx, &[k.pid]).await?;
    readings.sort_by_key(|r| std::cmp::Reverse((r.observed_on, r.id)));
    let by_day: HashMap<NaiveDate, usize> = readings.iter().fold(HashMap::new(), |mut m, r| {
        *m.entry(r.observed_on).or_default() += 1;
        m
    });
    format::json(serde_json::json!({
        "readings": readings.iter().map(|r| serde_json::json!({
            "pid": r.pid, "observed_on": r.observed_on, "value": r.value, "note": r.note,
            "actor": r.actor, "superseded": by_day.get(&r.observed_on).copied().unwrap_or(1) > 1
                && readings.iter().find(|x| x.observed_on == r.observed_on).map(|x| x.pid) != Some(r.pid),
        })).collect::<Vec<_>>(),
    }))
}

/// The KPI routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/kpis", post(create))
        .add("/kpis", get(list))
        .add("/kpis/{pid}", get(get_one))
        .add("/kpis/{pid}", put(update))
        .add("/kpis/{pid}", delete(remove))
        .add("/kpis/{pid}/readings", post(add_reading))
        .add("/kpis/{pid}/readings", get(list_readings))
}

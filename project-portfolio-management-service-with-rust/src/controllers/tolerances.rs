//! Project tolerances: how far a plan may drift on time, cost, risk and resources, and a red/amber/green view of
//! how much of each limit is used. Rules in [`crate::tolerances`]; inheritance in [`crate::inherit`].
//!
//! * `GET /api/plans/{pid}/tolerances`: the four dimensions in force for a plan (its own limit, else the nearest
//!   ancestor's), each with what was measured, how it was measured, and the colour.
//! * `PUT /api/plans/{pid}/tolerances/{dimension}`: set the plan's own limit for one dimension.
//! * `DELETE` (and a POST twin) `/api/plans/{pid}/tolerances/{dimension}`: remove it, so the plan inherits again.
//! * `GET /api/tolerances?status=&dimension=`: every plan the caller can read that has any tolerance, worst first.
//!
//! A limit with no usable measurement is `unknown`, never green. A figure that depends on other plans (the
//! over-booking of the plan's people) counts only the plans the caller can read, so it can differ between callers;
//! the answer says so.

use std::collections::HashMap;

use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::governance::find_item;
use super::{db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::inherit::{Source, resolve};
use crate::models::_entities::{allocations, escalations, plan_tolerances, plans, risks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::plans::Model as PlanModel;
use crate::tolerances::{self as rules, Dimension, Status};

/// Most plans the portfolio view measures in one request.
const BOARD_CAP: u64 = 200;
/// How far ahead over-booking is looked for, in days.
const BOOKING_WINDOW_DAYS: u64 = 90;

/// A measurement, or why there is none.
struct Measure {
    value: Option<i64>,
    reason: Option<String>,
}

impl Measure {
    const fn of(value: i64) -> Self {
        Self {
            value: Some(value),
            reason: None,
        }
    }
    fn none(reason: &str) -> Self {
        Self {
            value: None,
            reason: Some(reason.to_string()),
        }
    }
}

async fn measure_time(ctx: &AppContext, plan: &plans::Model) -> Result<Measure> {
    let target = plan
        .to_plan()
        .ok()
        .and_then(|p| p.target_date)
        .and_then(|d| NaiveDate::parse_from_str(d.get(..10).unwrap_or(&d), "%Y-%m-%d").ok());
    let Some(target) = target else {
        return Ok(Measure::none(
            "the plan has no target_date to hold the forecast against",
        ));
    };
    let net = super::cpm::load_network(ctx, plan.pid).await?;
    let Ok(result) = net.analyze(0) else {
        return Ok(Measure::none(
            "the schedule has a dependency cycle, so there is no forecast finish",
        ));
    };
    Ok(result.project_finish.map_or_else(
        || Measure::none("no task could be scheduled, so there is no forecast finish"),
        |finish| Measure::of(rules::days_late(finish, target)),
    ))
}

async fn measure_cost(ctx: &AppContext, plan: Uuid) -> Result<Measure> {
    let forecast = super::financials::forecast_of(ctx, plan).await?;
    let Some(eac) = forecast.estimate_at_completion_minor else {
        return Ok(Measure::none(
            "there is no estimate at completion (no budget baseline or cost-to-complete observation)",
        ));
    };
    let Some(baseline) = super::financials::latest_baseline(ctx, plan).await? else {
        return Ok(Measure::none(
            "there is no approved budget baseline to hold the estimate against",
        ));
    };
    let total = baseline
        .periods
        .iter()
        .fold(0_i64, |a, p| a.saturating_add(p.planned_minor));
    Ok(rules::overrun_bps(eac, total).map_or_else(
        || Measure::none("the approved baseline totals zero, so an overrun has no percentage"),
        Measure::of,
    ))
}

async fn measure_risk(ctx: &AppContext, plan: Uuid) -> Result<Measure> {
    let rows = risks::Entity::find()
        .filter(risks::Column::PlanPid.eq(plan))
        .filter(risks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if rows.is_empty() {
        return Ok(Measure::none(
            "no risks are recorded for the plan, which is not the same as no risk",
        ));
    }
    let worst = rows
        .iter()
        .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
        .map(|r| rules::risk_score(r.probability, r.impact))
        .max()
        .unwrap_or(0);
    Ok(Measure::of(i64::from(worst)))
}

/// Over-booking of the plan's named, committed people over the next 90 days, across `readable` allocations.
fn measure_resources(readable: &[allocations::Model], plan: Uuid, today: NaiveDate) -> Measure {
    let named = |a: &allocations::Model| {
        !crate::portfolio_resources::is_placeholder(&a.person_ref) && a.booking != "proposed"
    };
    let mut people: Vec<&str> = readable
        .iter()
        .filter(|a| a.plan_pid == plan && named(a))
        .map(|a| a.person_ref.as_str())
        .collect();
    people.sort_unstable();
    people.dedup();
    if people.is_empty() {
        return Measure::none("no named, committed people are booked on the plan");
    }
    let to = today + chrono::Days::new(BOOKING_WINDOW_DAYS - 1);
    let worst = people
        .iter()
        .map(|person| {
            let bookings: Vec<rules::Booking> = readable
                .iter()
                .filter(|a| a.person_ref == *person && named(a))
                .map(|a| (a.start_date, a.end_date, a.percent))
                .collect();
            rules::peak_overbooking(&bookings, today, to)
        })
        .max()
        .unwrap_or(0);
    Measure::of(worst)
}

/// The live allocations the caller may read (their plans are readable).
async fn readable_allocations(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
) -> Result<Vec<allocations::Model>> {
    let rows = crate::models::visibility::all_allocations_with_placeholders(&ctx.db).await?;
    let mut pids: Vec<Uuid> = rows.iter().map(|a| a.plan_pid).collect();
    pids.sort();
    pids.dedup();
    let readable = super::plan_access::readable_plans(ctx, caller, &pids).await;
    Ok(rows
        .into_iter()
        .filter(|a| readable.contains(&a.plan_pid))
        .collect())
}

/// Every tolerance row of `plans`, keyed by `(plan, dimension)`.
async fn rows_for(
    ctx: &AppContext,
    plan_pids: Vec<Uuid>,
) -> Result<HashMap<(Uuid, String), plan_tolerances::Model>> {
    Ok(plan_tolerances::Entity::find()
        .filter(plan_tolerances::Column::PlanPid.is_in(plan_pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| ((r.plan_pid, r.dimension.clone()), r))
        .collect())
}

/// The four dimensions in force for one plan, measured and coloured.
async fn picture(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: &plans::Model,
    allocs: &[allocations::Model],
) -> Result<Value> {
    let ancestors = PlanModel::ancestor_pids(&ctx.db, plan.pid)
        .await
        .map_err(Error::Model)?;
    let mut chain = vec![plan.pid];
    chain.extend(&ancestors);
    let rows = rows_for(ctx, chain).await?;
    let today = chrono::Utc::now().date_naive();
    // Live escalations that cite a tolerance dimension, so a red one nobody has escalated is visible.
    let mut escalated: HashMap<String, usize> = HashMap::new();
    for e in escalations::Entity::find()
        .filter(escalations::Column::PlanPid.eq(plan.pid))
        .filter(escalations::Column::DeletedAt.is_null())
        .filter(escalations::Column::ReasonKind.eq("tolerance"))
        .filter(escalations::Column::Status.is_in(["open", "acknowledged"]))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        if let Some(d) = e.tolerance_dimension {
            *escalated.entry(d).or_insert(0) += 1;
        }
    }
    let mut needs_escalation = false;
    let mut dimensions = Vec::new();
    let mut statuses = Vec::new();
    for dim in Dimension::ALL {
        let own = rows
            .get(&(plan.pid, dim.token().to_string()))
            .map(|r| r.pid);
        let up: Vec<(Uuid, Option<Uuid>)> = ancestors
            .iter()
            .map(|a| (*a, rows.get(&(*a, dim.token().to_string())).map(|r| r.pid)))
            .collect();
        let r = resolve(plan.pid, own, &up);
        let row = r.value.and_then(|pid| rows.values().find(|x| x.pid == pid));
        let Some(row) = row else {
            statuses.push(Status::NotSet);
            dimensions.push(json!({
                "dimension": dim.token(), "unit": dim.unit(), "basis": dim.basis(),
                "status": Status::NotSet, "limit": null, "source": Source::None,
                "reason": "no limit is set for this plan or any plan above it",
            }));
            continue;
        };
        let measure = match dim {
            Dimension::Time => measure_time(ctx, plan).await?,
            Dimension::Cost => measure_cost(ctx, plan.pid).await?,
            Dimension::Risk => measure_risk(ctx, plan.pid).await?,
            Dimension::Resources => measure_resources(allocs, plan.pid, today),
        };
        let (status, used) = rules::classify(
            measure.value,
            row.limit_value,
            i64::from(row.amber_from_percent),
        );
        statuses.push(status);
        let open_escalations = escalated.get(dim.token()).copied().unwrap_or(0);
        let escalation_needed = status == Status::Red && open_escalations == 0;
        needs_escalation |= escalation_needed;
        let from = r.from_plan.filter(|p| *p != plan.pid);
        let from_readable = match from {
            Some(p) => super::plan_access::readable_plans(ctx, caller, &[p])
                .await
                .contains(&p),
            None => true,
        };
        dimensions.push(json!({
            "dimension": dim.token(), "unit": dim.unit(), "basis": dim.basis(),
            "status": status, "limit": row.limit_value, "amber_from_percent": row.amber_from_percent,
            "measured": measure.value, "usage_percent": used, "reason": measure.reason,
            "source": r.source, "from_plan_pid": if from_readable { from } else { None },
            "note": row.note,
            "open_escalations": open_escalations, "escalation_needed": escalation_needed,
        }));
    }
    Ok(json!({
        "plan_pid": plan.pid, "plan_name": plan.name, "overall": rules::overall(&statuses),
        "escalation_needed": needs_escalation,
        "as_of": chrono::Utc::now(), "dimensions": dimensions,
        "how_to_read": "Green: below amber_from_percent of the limit. Amber: from there up to the limit. Red: beyond it. \
    Unknown: a limit is set but could not be measured (the reason says why); it is never shown as green. not_set: no limit here or above.",
    }))
}

/// `GET /api/plans/{pid}/tolerances`.
#[debug_handler]
async fn get_plan(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let allocs = readable_allocations(&ctx, &caller).await?;
    format::json(picture(&ctx, &caller, &plan, &allocs).await?)
}

/// `PUT /api/plans/{pid}/tolerances/{dimension}` body.
#[derive(Debug, Deserialize)]
struct SetBody {
    limit: i64,
    #[serde(default)]
    amber_from_percent: Option<i64>,
    #[serde(default)]
    note: Option<String>,
}

fn dimension_of(raw: &str) -> Result<Dimension> {
    Dimension::parse(raw).ok_or_else(|| {
        unprocessable(&format!(
            "dimension must be one of time, cost, risk, resources, not {raw:?}"
        ))
    })
}

/// `PUT /api/plans/{pid}/tolerances/{dimension}`.
#[debug_handler]
async fn set_one(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, dimension)): Path<(String, String)>,
    Json(b): Json<SetBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let dim = dimension_of(&dimension)?;
    let amber = b
        .amber_from_percent
        .unwrap_or(rules::DEFAULT_AMBER_FROM_PERCENT);
    let note = b
        .note
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(str::to_string);
    let problems = rules::problems(dim, b.limit, amber, note.as_deref());
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = plan_tolerances::Entity::find()
        .filter(plan_tolerances::Column::PlanPid.eq(plan.pid))
        .filter(plan_tolerances::Column::Dimension.eq(dim.token()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let amber = i32::try_from(amber).unwrap_or(75);
    if let Some(row) = existing {
        let mut a: plan_tolerances::ActiveModel = row.into();
        a.limit_value = ActiveValue::set(b.limit);
        a.amber_from_percent = ActiveValue::set(amber);
        a.note = ActiveValue::set(note);
        a.set_by = ActiveValue::set(caller.actor().map(str::to_string));
        a.update(&ctx.db).await.map_err(db_err)?;
    } else {
        plan_tolerances::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            dimension: ActiveValue::set(dim.token().to_string()),
            limit_value: ActiveValue::set(b.limit),
            amber_from_percent: ActiveValue::set(amber),
            note: ActiveValue::set(note),
            set_by: ActiveValue::set(caller.actor().map(str::to_string)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?;
    }
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "tolerance_set",
        caller.actor(),
        Some(json!({ "dimension": dim.token(), "limit": b.limit, "amber_from_percent": amber })),
    )
    .await
    .ok();
    let allocs = readable_allocations(&ctx, &caller).await?;
    format::json(picture(&ctx, &caller, &plan, &allocs).await?)
}

/// `DELETE /api/plans/{pid}/tolerances/{dimension}` and its POST twin.
#[debug_handler]
async fn remove_one(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, dimension)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let dim = dimension_of(&dimension)?;
    let row = plan_tolerances::Entity::find()
        .filter(plan_tolerances::Column::PlanPid.eq(plan.pid))
        .filter(plan_tolerances::Column::Dimension.eq(dim.token()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    row.delete(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "tolerance_removed",
        caller.actor(),
        Some(json!({ "dimension": dim.token() })),
    )
    .await
    .ok();
    let allocs = readable_allocations(&ctx, &caller).await?;
    format::json(picture(&ctx, &caller, &plan, &allocs).await?)
}

/// `GET /api/tolerances` query.
#[derive(Debug, Deserialize)]
struct BoardQuery {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    dimension: Option<String>,
}

/// `GET /api/tolerances?status=&dimension=`: the readable plans that have any tolerance, worst first.
#[debug_handler]
async fn board(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<BoardQuery>,
) -> Result<Response> {
    let want_status = match q.status.as_deref() {
        None => None,
        Some(s) => Some(
            ["green", "amber", "red", "unknown", "not_set"]
                .iter()
                .find(|x| **x == s)
                .copied()
                .ok_or_else(|| {
                    unprocessable("status must be green, amber, red, unknown or not_set")
                })?,
        ),
    };
    let want_dimension = q.dimension.as_deref().map(dimension_of).transpose()?;
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .order_by_asc(plans::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let pids: Vec<Uuid> = all.iter().map(|p| p.pid).collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &pids).await;
    let visible: Vec<&plans::Model> = all.iter().filter(|p| readable.contains(&p.pid)).collect();
    let truncated = visible.len() as u64 > BOARD_CAP;
    let allocs = readable_allocations(&ctx, &caller).await?;
    let mut out: Vec<Value> = Vec::new();
    for plan in visible
        .into_iter()
        .take(usize::try_from(BOARD_CAP).unwrap_or(usize::MAX))
    {
        let p = picture(&ctx, &caller, plan, &allocs).await?;
        let overall: Status =
            serde_json::from_value(p["overall"].clone()).unwrap_or(Status::NotSet);
        if overall == Status::NotSet {
            continue;
        }
        if let Some(want) = want_status
            && p["overall"] != want
        {
            continue;
        }
        if let Some(d) = want_dimension
            && !p["dimensions"].as_array().is_some_and(|ds| {
                ds.iter()
                    .any(|x| x["dimension"] == d.token() && x["status"] != "not_set")
            })
        {
            continue;
        }
        out.push(p);
    }
    let rank = |v: &Value| match v["overall"].as_str() {
        Some("red") => 0,
        Some("amber") => 1,
        Some("unknown") => 2,
        _ => 3,
    };
    out.sort_by_key(rank);
    format::json(json!({
        "plans": out, "truncated": truncated, "measured_at_most": BOARD_CAP,
        "note": "Plans you can read that have a tolerance (their own or inherited), worst first. Measuring each plan reads its schedule, budget, risks and bookings, so at most 200 plans are measured per request.",
    }))
}

/// `POST /api/plans/{pid}/tolerances/escalate`: raise one escalation, at the first level of the governance path, for
/// every red dimension that has no live escalation. Nothing happens on its own: a person asks for it, and asking
/// twice raises nothing the second time.
#[debug_handler]
async fn escalate(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let allocs = readable_allocations(&ctx, &caller).await?;
    let p = picture(&ctx, &caller, &plan, &allocs).await?;
    let mut raised = Vec::new();
    for d in p["dimensions"].as_array().into_iter().flatten() {
        if d["escalation_needed"] != true {
            continue;
        }
        let token = d["dimension"].as_str().unwrap_or_default();
        let unit = d["unit"].as_str().unwrap_or_default();
        let row = super::escalations::insert(
            &ctx,
            &caller,
            plan.pid,
            super::escalations::New {
                title: format!("{token} tolerance exceeded"),
                description: Some(format!(
                    "Measured {} {unit} against a limit of {} {unit} ({}% of the limit).",
                    d["measured"], d["limit"], d["usage_percent"]
                )),
                reason_kind: "tolerance".to_string(),
                tolerance_dimension: Some(token.to_string()),
                level: None,
                escalated_to: None,
                owner_ref: None,
                respond_by: None,
            },
        )
        .await?;
        raised.push(super::escalations::view(
            &row,
            chrono::Utc::now().date_naive(),
        ));
    }
    format::json(
        json!({ "raised": raised, "tolerances": picture(&ctx, &caller, &plan, &allocs).await? }),
    )
}

/// The tolerance routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/tolerances", get(board))
        .add("/plans/{pid}/tolerances", get(get_plan))
        .add("/plans/{pid}/tolerances/escalate", post(escalate))
        .add("/plans/{pid}/tolerances/{dimension}", put(set_one))
        .add("/plans/{pid}/tolerances/{dimension}", delete(remove_one))
        .add(
            "/plans/{pid}/tolerances/{dimension}/delete",
            post(remove_one),
        )
}

//! The portfolio ledger (EX-12). Rules in [`crate::ledger`].
//!
//! * `POST`/`GET /api/funding-allocations`: money a funding source allocated, on a date (append-only);
//! * `PUT /api/plans/{pid}/budget-lines/{line}/funding-allocation`: which allocation a line draws on;
//! * `GET /api/funding-allocations/{pid}/trace`: **what did this allocation buy?** in one answer: the budget
//!   lines that draw on it, their gates, the milestones met on those plans, the benefits validated on them,
//!   per currency, and the money **not traced** to any line, with its amount;
//! * `PUT`/`GET /api/cost-codes`: the map from the finance system's cost codes to budget lines;
//! * `POST /api/ledger/reconcile`: a CSV ledger export set beside the register. It writes nothing.
//!
//! Only plans the caller may read are traced: a line of a plan they cannot read is nowhere in the answer, so
//! to them the money on it shows as not traced.

use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::ledger::{self as rules, Line};
use crate::models::_entities::{
    benefit_validations, benefits, budget_lines, cost_code_maps as codes,
    funding_allocations as allocations, funding_sources, milestones, plans,
};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    let t = e.to_string();
    if t.contains("ccm_once") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "conflict",
                "that cost code is already mapped: map it to one line only",
            ),
        );
    }
    Error::string(&t)
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/funding-allocations", post(create))
        .add("/funding-allocations", get(list))
        .add("/funding-allocations/{pid}/trace", get(trace))
        .add(
            "/plans/{pid}/budget-lines/{line}/funding-allocation",
            put(attach),
        )
        .add("/cost-codes", put(map_code))
        .add("/cost-codes", get(list_codes))
        .add("/ledger/reconcile", post(reconcile))
}

async fn visible_plans(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<HashSet<Uuid>> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    Ok(super::plan_access::readable_plans(ctx, caller, &all).await)
}

// ---- allocations ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct NewAllocation {
    funding_source_pid: Uuid,
    amount_minor: i64,
    currency: String,
    allocated_on: chrono::NaiveDate,
    #[serde(default)]
    reference: Option<String>,
}

fn view(a: &allocations::Model, source: &str) -> Value {
    json!({ "pid": a.pid, "funding_source_pid": a.funding_source_pid, "funding_source": source, "amount_minor": a.amount_minor,
        "currency": a.currency, "allocated_on": a.allocated_on, "reference": a.reference })
}

/// `POST /api/funding-allocations`
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewAllocation>,
) -> Result<Response> {
    let src = funding_sources::Entity::find()
        .filter(funding_sources::Column::Pid.eq(b.funding_source_pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or_else(|| unprocessable("funding_source_pid is not a funding source"))?;
    let mut problems = Vec::new();
    if b.amount_minor <= 0 {
        problems.push("amount_minor must be above 0");
    }
    if !crate::governance::valid_currency(&b.currency) {
        problems.push("currency must be an ISO 4217 code");
    }
    if b.allocated_on > Utc::now().date_naive() {
        problems.push("allocated_on is not in the future");
    }
    if b.reference
        .as_deref()
        .is_some_and(|r| r.chars().count() > 120)
    {
        problems.push("reference is at most 120 characters");
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let row = allocations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        funding_source_pid: ActiveValue::set(src.pid),
        amount_minor: ActiveValue::set(b.amount_minor),
        currency: ActiveValue::set(b.currency.clone()),
        allocated_on: ActiveValue::set(b.allocated_on),
        reference: ActiveValue::set(
            b.reference
                .map(|r| r.trim().to_string())
                .filter(|r| !r.is_empty()),
        ),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "funding_allocation_recorded",
        caller.actor(),
        Some(json!({ "amount_minor": row.amount_minor, "currency": row.currency })),
    )
    .await;
    format::json(view(&row, &src.name))
}

/// The lines drawing on each allocation that the caller may read.
async fn lines_by_allocation(
    ctx: &AppContext,
    visible: &HashSet<Uuid>,
) -> Result<HashMap<Uuid, Vec<budget_lines::Model>>> {
    let mut out: HashMap<Uuid, Vec<budget_lines::Model>> = HashMap::new();
    for l in budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .filter(budget_lines::Column::FundingAllocationPid.is_not_null())
        .order_by_asc(budget_lines::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    {
        if visible.contains(&l.plan_pid)
            && let Some(a) = l.funding_allocation_pid
        {
            out.entry(a).or_default().push(l);
        }
    }
    Ok(out)
}

/// `GET /api/funding-allocations` — each allocation, how much of it budget lines you may read draw on, and
/// the money not traced to any line.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let visible = visible_plans(&ctx, &caller).await?;
    let by = lines_by_allocation(&ctx, &visible).await?;
    let names: HashMap<Uuid, String> = funding_sources::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|s| (s.pid, s.name))
        .collect();
    let rows = allocations::Entity::find()
        .order_by_desc(allocations::Column::AllocatedOn)
        .order_by_desc(allocations::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(json!({ "allocations": rows.iter().map(|a| {
        let planned: i64 = by.get(&a.pid).map_or(0, |ls| ls.iter().map(|l| l.planned_minor).sum());
        let mut v = view(a, names.get(&a.funding_source_pid).map_or("", String::as_str));
        v["planned_on_lines_minor"] = json!(planned);
        v["untraced_minor"] = json!(rules::untraced(a.amount_minor, planned));
        v["lines"] = json!(by.get(&a.pid).map_or(0, Vec::len));
        v
    }).collect::<Vec<_>>() }))
}

#[derive(Debug, Deserialize)]
struct AttachBody {
    /// The allocation the line draws on; `null` detaches it.
    #[serde(default)]
    funding_allocation_pid: Option<Uuid>,
}

/// `PUT /api/plans/{pid}/budget-lines/{line}/funding-allocation`
#[debug_handler]
async fn attach(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, line)): Path<(String, Uuid)>,
    Json(b): Json<AttachBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = budget_lines::Entity::find()
        .filter(budget_lines::Column::Pid.eq(line))
        .filter(budget_lines::Column::PlanPid.eq(plan.pid))
        .filter(budget_lines::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    if let Some(a) = b.funding_allocation_pid {
        let alloc = allocations::Entity::find()
            .filter(allocations::Column::Pid.eq(a))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .ok_or_else(|| unprocessable("funding_allocation_pid is not an allocation"))?;
        if row.funding_source_pid != Some(alloc.funding_source_pid) {
            return Err(unprocessable(
                "the line's funding source must be the allocation's source: set the line's funding source first",
            ));
        }
        if row.currency != alloc.currency {
            return Err(unprocessable(
                "the line and the allocation are in different currencies",
            ));
        }
    }
    let mut a: budget_lines::ActiveModel = row.into();
    a.funding_allocation_pid = ActiveValue::set(b.funding_allocation_pid);
    a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "budget_line_allocation_set",
        caller.actor(),
        Some(json!({ "line": line, "allocation": b.funding_allocation_pid })),
    )
    .await;
    format::json(json!({ "line": line, "funding_allocation_pid": b.funding_allocation_pid }))
}

/// `GET /api/funding-allocations/{pid}/trace` — what the allocation bought.
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn trace(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let a = allocations::Entity::find()
        .filter(allocations::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let source = funding_sources::Entity::find()
        .filter(funding_sources::Column::Pid.eq(a.funding_source_pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .map(|s| s.name)
        .unwrap_or_default();
    let visible = visible_plans(&ctx, &caller).await?;
    let lines = lines_by_allocation(&ctx, &visible)
        .await?
        .remove(&a.pid)
        .unwrap_or_default();
    let plan_ids: Vec<Uuid> = lines
        .iter()
        .map(|l| l.plan_pid)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let names: HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(plan_ids.clone()))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    // Milestones met on those plans.
    let ms: Vec<milestones::Model> = if plan_ids.is_empty() {
        Vec::new()
    } else {
        milestones::Entity::find()
            .filter(milestones::Column::PlanPid.is_in(plan_ids.clone()))
            .filter(milestones::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
    };
    let derived =
        super::milestones::derive(&ctx, &ms, Utc::now().date_naive(), 14, 5, false).await?;
    let milestone_rows: Vec<Value> = derived.json.iter().map(|(_, _, v)| json!({
        "plan": names.get(&Uuid::parse_str(v["plan_pid"].as_str().unwrap_or("")).unwrap_or_default()), "name": v["name"], "due": v["due"],
        "met": v["achieved"], "met_on": v["achieved_on"],
    })).collect();
    let met = milestone_rows.iter().filter(|m| m["met"] == true).count();
    // Benefits validated on those plans, per currency.
    let bens: Vec<benefits::Model> = if plan_ids.is_empty() {
        Vec::new()
    } else {
        benefits::Entity::find()
            .filter(benefits::Column::PlanPid.is_in(plan_ids.clone()))
            .filter(benefits::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
    };
    let bids: Vec<Uuid> = bens.iter().map(|b| b.pid).collect();
    let vals = if bids.is_empty() {
        Vec::new()
    } else {
        benefit_validations::Entity::find()
            .filter(benefit_validations::Column::BenefitPid.is_in(bids))
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
    };
    let mut validated: BTreeMap<String, i64> = BTreeMap::new();
    let mut planned_b: BTreeMap<String, i64> = BTreeMap::new();
    // A validation states the amount validated to date, so the latest one per benefit counts.
    let mut latest: HashMap<Uuid, &benefit_validations::Model> = HashMap::new();
    for v in &vals {
        if latest.get(&v.benefit_pid).is_none_or(|x| x.id < v.id) {
            latest.insert(v.benefit_pid, v);
        }
    }
    for v in latest.values() {
        *validated.entry(v.currency.clone()).or_default() += v.amount_minor;
    }
    for b in &bens {
        if let (Some(c), Some(t)) = (&b.currency, b.target_minor) {
            *planned_b.entry(c.clone()).or_default() += t;
        }
    }
    let planned: i64 = lines.iter().map(|l| l.planned_minor).sum();
    let actual: i64 = lines.iter().map(|l| l.actual_minor).sum();
    let gated = lines.iter().filter(|l| l.gate.is_some()).count();
    let released = lines.iter().filter(|l| l.released_at.is_some()).count();
    let untraced = rules::untraced(a.amount_minor, planned);
    format::json(json!({
        "allocation": view(&a, &source),
        "currency": a.currency,
        "lines": lines.iter().map(|l| json!({ "line_pid": l.pid, "plan_pid": l.plan_pid, "plan": names.get(&l.plan_pid), "description": l.description,
            "category": l.category, "planned_minor": l.planned_minor, "actual_minor": l.actual_minor, "gate": l.gate, "released": l.released_at.is_some() })).collect::<Vec<_>>(),
        "money": {
            "allocated_minor": a.amount_minor, "planned_on_lines_minor": planned, "spent_minor": actual,
            "untraced_minor": untraced,
            "untraced_reason": if lines.is_empty() { Some("no budget line you may read draws on this allocation") } else if untraced > 0 { Some("allocated but not yet planned on any budget line") } else if untraced < 0 { Some("lines plan more than was allocated") } else { None },
        },
        "gates": { "lines_with_a_gate": gated, "released": released },
        "milestones": { "met": met, "total": milestone_rows.len(), "items": milestone_rows },
        "benefits": {
            "validated_by_currency": validated.iter().map(|(c, v)| json!({ "currency": c, "validated_minor": v })).collect::<Vec<_>>(),
            "planned_by_currency": planned_b.iter().map(|(c, v)| json!({ "currency": c, "planned_minor": v })).collect::<Vec<_>>(),
            "benefits": bens.len(),
        },
        "note": "What the allocation bought, in one answer: the budget lines that draw on it, their gates, the milestones met and the benefits validated on those plans. Money allocated but not on any line is untraced and is shown with its amount. Plans you may not read are not in this answer, so to you the money on their lines shows as untraced.",
    }))
}

// ---- cost codes -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CodeBody {
    cost_code: String,
    budget_line_pid: Uuid,
}

/// `PUT /api/cost-codes` — map a finance cost code to a budget line (one code, one line).
#[debug_handler]
async fn map_code(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<CodeBody>,
) -> Result<Response> {
    let code = b.cost_code.trim().to_string();
    if !(1..=60).contains(&code.chars().count()) {
        return Err(unprocessable("cost_code is 1 to 60 characters"));
    }
    let line = budget_lines::Entity::find()
        .filter(budget_lines::Column::Pid.eq(b.budget_line_pid))
        .filter(budget_lines::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or_else(|| unprocessable("budget_line_pid is not a budget line"))?;
    // The line sits on a plan: the caller must be able to read it, or the answer is the same as for no line.
    if !visible_plans(&ctx, &caller).await?.contains(&line.plan_pid) {
        return Err(unprocessable("budget_line_pid is not a budget line"));
    }
    let existing = codes::Entity::find()
        .filter(sea_orm::sea_query::Expr::cust_with_values(
            "lower(btrim(cost_code)) = lower($1)",
            [code.clone()],
        ))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let row = if let Some(e) = existing {
        let mut a: codes::ActiveModel = e.into();
        a.budget_line_pid = ActiveValue::set(line.pid);
        a.update(&ctx.db).await.map_err(|e| db_err(&e))?
    } else {
        codes::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            cost_code: ActiveValue::set(code),
            budget_line_pid: ActiveValue::set(line.pid),
            created_by: ActiveValue::set(caller.actor().map(str::to_string)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    };
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "cost_code_mapped",
        caller.actor(),
        Some(json!({ "line": line.pid })),
    )
    .await;
    format::json(json!({ "cost_code": row.cost_code, "budget_line_pid": row.budget_line_pid }))
}

/// `GET /api/cost-codes` — the mapped codes whose line is on a plan you may read.
#[debug_handler]
async fn list_codes(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let visible = visible_plans(&ctx, &caller).await?;
    let lines: HashMap<Uuid, budget_lines::Model> = budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|l| (l.pid, l))
        .collect();
    let rows = codes::Entity::find()
        .order_by_asc(codes::Column::CostCode)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(
        json!({ "cost_codes": rows.iter().filter(|c| lines.get(&c.budget_line_pid).is_some_and(|l| visible.contains(&l.plan_pid)))
        .map(|c| json!({ "cost_code": c.cost_code, "budget_line_pid": c.budget_line_pid })).collect::<Vec<_>>() }),
    )
}

/// `POST /api/ledger/reconcile` — body: a CSV export (`cost_code,amount_minor,currency`). Writes nothing.
#[debug_handler]
async fn reconcile(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    body: String,
) -> Result<Response> {
    if body.len() > 1_000_000 {
        return Err(unprocessable("the file is over 1 MB: split it"));
    }
    let rows = rules::parse(&body).map_err(|p| unprocessable(&p.join("; ")))?;
    let visible = visible_plans(&ctx, &caller).await?;
    let lines: HashMap<Uuid, budget_lines::Model> = budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|l| (l.pid, l))
        .collect();
    let map: HashMap<String, Line> = codes::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter_map(|c| {
            let l = lines.get(&c.budget_line_pid)?;
            visible.contains(&l.plan_pid).then(|| {
                (
                    c.cost_code.trim().to_lowercase(),
                    Line {
                        line_pid: l.pid,
                        currency: l.currency.clone(),
                        actual_minor: l.actual_minor,
                    },
                )
            })
        })
        .collect();
    let report = rules::reconcile(&rows, &map);
    format::json(json!({
        "rows": rows.len(), "report": report,
        "note": "Each row's cost code is looked up in the cost-code map; the rows for a code are summed and set beside what the register holds as spent on the line. A code that is not mapped (or maps to a line you may not read) is listed as unmatched with its amount. A row in another currency than its line is a mismatch and is not summed in. Nothing is written.",
    }))
}

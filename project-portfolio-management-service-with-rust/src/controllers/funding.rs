//! Funding sources and financial-year money (DP-6). Rules in [`crate::financial_year`].
//!
//! `capex` is capital and `opex` is revenue (the existing budget-line category). The pack groups each
//! programme's budget lines by funding source, class and currency for the financial year the pack's
//! cut-off falls in: budget, spend to date and variance, with the existing whole-life forecast beside
//! them per currency. A line with no funding source is counted and listed, never hidden.

use super::refuse;
use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::financial_year::{self as rules, Line, Row};
use crate::models::_entities::{budget_lines, funding_sources, plans};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    refuse(
        StatusCode::UNPROCESSABLE_ENTITY,
        "unprocessable_entity",
        message,
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/funding-sources", get(list_sources))
        .add("/funding-sources", post(create_source))
        .add("/funding-sources/{pid}", put(update_source))
        .add(
            "/plans/{pid}/budget-lines/{line}/funding-source",
            put(set_line_source),
        )
        .add("/portfolio-money", get(portfolio_money))
}

#[derive(Debug, Deserialize)]
struct SourceBody {
    name: String,
}

/// `POST /api/funding-sources` — add to the owner-supplied list.
#[debug_handler]
async fn create_source(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<SourceBody>,
) -> Result<Response> {
    let name = b.name.trim().to_string();
    if name.chars().count() < 2 || name.chars().count() > 120 {
        return Err(unprocessable("name must be 2 to 120 characters"));
    }
    let dup = funding_sources::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .any(|s| s.name.to_lowercase() == name.to_lowercase());
    if dup {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "a funding source with that name exists",
        ));
    }
    let row = funding_sources::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(name),
        active: ActiveValue::set(true),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "funding_source_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({ "pid": row.pid, "name": row.name, "active": row.active }))
}

/// `GET /api/funding-sources`.
#[debug_handler]
async fn list_sources(State(ctx): State<AppContext>) -> Result<Response> {
    let mut rows = funding_sources::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    rows.sort_by_key(|r| r.name.to_lowercase());
    format::json(serde_json::json!({
        "funding_sources": rows.iter().map(|r| serde_json::json!({
            "pid": r.pid, "name": r.name, "active": r.active })).collect::<Vec<_>>(),
    }))
}

#[derive(Debug, Deserialize)]
struct ActiveBody {
    active: bool,
}

/// `PUT /api/funding-sources/{pid}` — retire or restore a source. Lines keep a retired source.
#[debug_handler]
async fn update_source(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<Uuid>,
    Json(b): Json<ActiveBody>,
) -> Result<Response> {
    let row = funding_sources::Entity::find()
        .filter(funding_sources::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let mut a: funding_sources::ActiveModel = row.into();
    a.active = ActiveValue::set(b.active);
    let row = a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "funding_source_updated",
        caller.actor(),
        Some(serde_json::json!({ "active": b.active })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({ "pid": row.pid, "name": row.name, "active": row.active }))
}

#[derive(Debug, Deserialize)]
struct LineSourceBody {
    #[serde(default)]
    funding_source_pid: Option<Uuid>,
}

/// Whether a funding source exists and is active.
///
/// # Errors
///
/// A 422 when it does not.
pub(crate) async fn check_source(ctx: &AppContext, pid: Uuid) -> Result<()> {
    let ok = funding_sources::Entity::find()
        .filter(funding_sources::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .is_some_and(|s| s.active);
    if ok {
        Ok(())
    } else {
        Err(unprocessable(
            "funding_source_pid is not an active funding source",
        ))
    }
}

/// `PUT /api/plans/{pid}/budget-lines/{line}/funding-source` — set or clear a line's funding source.
#[debug_handler]
async fn set_line_source(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, line)): Path<(String, Uuid)>,
    Json(b): Json<LineSourceBody>,
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
    if let Some(s) = b.funding_source_pid {
        check_source(&ctx, s).await?;
    }
    let mut a: budget_lines::ActiveModel = row.into();
    a.funding_source_pid = ActiveValue::set(b.funding_source_pid);
    a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "budget_line_funding_source_set",
        caller.actor(),
        Some(serde_json::json!({ "line": line, "funding_source_pid": b.funding_source_pid })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({ "line": line, "funding_source_pid": b.funding_source_pid }))
}

#[derive(Debug, Deserialize)]
struct MoneyQuery {
    /// The year the financial year starts in, e.g. 2026 for 2026/27 (April start).
    #[serde(default)]
    fy: Option<i32>,
    #[serde(default)]
    combine_classes: Option<bool>,
}

/// `GET /api/portfolio-money[?fy=][&combine_classes=true]` — programme money by funding source for the
/// plans you can read.
#[debug_handler]
async fn portfolio_money(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<MoneyQuery>,
) -> Result<Response> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let sm = rules::start_month();
    let day = match q.fy {
        Some(y) if (2000..=2100).contains(&y) => {
            NaiveDate::from_ymd_opt(y, sm, 1).ok_or_else(|| unprocessable("fy is not a year"))?
        }
        Some(_) => return Err(unprocessable("fy must be a year between 2000 and 2100")),
        None => Utc::now().date_naive(),
    };
    let body = money_section(&ctx, &readable, day, q.combine_classes.unwrap_or(false)).await?;
    format::json(body)
}

fn ancestors_programme(
    by_pid: &HashMap<Uuid, &plans::Model>,
    mut at: Uuid,
    include: &HashSet<Uuid>,
) -> Option<Uuid> {
    for _ in 0..64 {
        let p = by_pid.get(&at)?;
        if p.kind.as_deref() == Some("Program") && include.contains(&p.pid) {
            return Some(p.pid);
        }
        at = p.parent_pid?;
    }
    None
}

fn row_json(r: &Row, names: &HashMap<Uuid, String>) -> serde_json::Value {
    serde_json::json!({
        "funding_source_pid": r.source,
        "funding_source": r.source.map_or("no funding source".to_string(),
            |s| names.get(&s).cloned().unwrap_or_else(|| "unknown funding source".to_string())),
        "class": r.class, "currency": r.currency,
        "budget_minor": r.budget_minor, "spend_minor": r.spend_minor,
        "variance_minor": r.variance_minor(), "lines": r.lines,
    })
}

/// The money section: programmes by funding source for the financial year containing `day`. Lines on
/// plans outside `include` (private or marked) are not read.
///
/// # Errors
///
/// When a database read fails.
#[allow(clippy::too_many_lines)]
pub(crate) async fn money_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    day: NaiveDate,
    combine_classes: bool,
) -> Result<serde_json::Value> {
    let sm = rules::start_month();
    let year = rules::year_of(day, sm);
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let by_pid: HashMap<Uuid, &plans::Model> = all.iter().map(|p| (p.pid, p)).collect();
    let names: HashMap<Uuid, String> = funding_sources::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|s| (s.pid, s.name))
        .collect();
    let rows = budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut lines: Vec<Line> = Vec::new();
    let mut unsourced: Vec<serde_json::Value> = Vec::new();
    for b in rows.iter().filter(|b| include.contains(&b.plan_pid)) {
        let programme = ancestors_programme(&by_pid, b.plan_pid, include).unwrap_or(Uuid::nil());
        if b.funding_source_pid.is_none() && unsourced.len() < 100 {
            unsourced.push(serde_json::json!({
                "line": b.pid, "plan": by_pid.get(&b.plan_pid).map(|p| p.name.clone()),
                "category": b.category, "description": b.description, "currency": b.currency,
                "planned_minor": b.planned_minor,
            }));
        }
        lines.push(Line {
            programme,
            source: b.funding_source_pid,
            class: b.category.clone(),
            currency: b.currency.clone(),
            planned_minor: b.planned_minor,
            actual_minor: b.actual_minor,
            period_start: b.period_start,
            period_end: b.period_end,
        });
    }
    let (grouped, quality) = rules::group(&lines, &year, combine_classes);
    let mut by_programme: BTreeMap<Uuid, Vec<&Row>> = BTreeMap::new();
    for r in &grouped {
        by_programme.entry(r.programme).or_default().push(r);
    }
    let mut programmes = Vec::new();
    let mut ids: Vec<&plans::Model> = all
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    ids.sort_by(|a, b| a.name.cmp(&b.name).then(a.pid.cmp(&b.pid)));
    for p in ids {
        // The existing whole-life forecast, rolled up over the programme and what is beneath it that
        // may be read, one row per currency.
        let mut children: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for c in all.iter().filter(|c| include.contains(&c.pid)) {
            if let Some(par) = c.parent_pid {
                children.entry(par).or_default().push(c.pid);
            }
        }
        let mut tree = vec![p.pid];
        let mut i = 0;
        while i < tree.len() && tree.len() < 2000 {
            if let Some(cs) = children.get(&tree[i]) {
                for c in cs {
                    if !tree.contains(c) {
                        tree.push(*c);
                    }
                }
            }
            i += 1;
        }
        let mut fs = Vec::new();
        for n in &tree {
            fs.push(super::financials::forecast_of(ctx, *n).await?);
        }
        let roll = crate::financials::rollup_forecast(&fs);
        programmes.push(serde_json::json!({
            "plan_pid": p.pid, "name": p.name,
            "by_funding_source": by_programme.get(&p.pid).map(|v| v.iter().map(|r| row_json(r, &names)).collect::<Vec<_>>()).unwrap_or_default(),
            "whole_life_forecast": {
                "rows": roll.rows, "plans_without_a_currency_signal": roll.no_currency_signal,
                "note": "The existing forecast (actual to date plus estimate to complete) over the whole life of the programme, not the financial year; one row per currency.",
            },
        }));
    }
    let loose: Vec<serde_json::Value> = by_programme
        .get(&Uuid::nil())
        .map(|v| v.iter().map(|r| row_json(r, &names)).collect())
        .unwrap_or_default();
    let _ = Datelike::year(&day);
    Ok(serde_json::json!({
        "financial_year": { "label": year.label, "start": year.start, "end": year.end, "starts_in_month": sm },
        "classes_combined": combine_classes,
        "programmes": programmes,
        "not_under_a_programme": loose,
        "data_quality": {
            "lines_with_no_funding_source": quality.no_source_lines,
            "listed_lines_with_no_funding_source": unsourced,
            "lines_with_no_dates": quality.undated_lines,
            "lines_in_other_financial_years": quality.other_year_lines,
            "lines_running_past_the_year_end": quality.spanning_lines,
        },
        "note": "Capital (capex) and revenue (opex) are shown apart and are never summed unless classes_combined is true; currencies are never summed. A line belongs to the financial year its period start falls in (its period end when it has no start); a line with no dates is outside every year and counted above. Budget is planned_minor and spend is actual_minor; variance is budget less spend. A line with no funding source is under \"no funding source\".",
    }))
}

/// The pack's money section for the financial year containing the cut-off.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    cut_off: DateTime<Utc>,
) -> Result<serde_json::Value> {
    money_section(ctx, include, cut_off.date_naive(), false).await
}

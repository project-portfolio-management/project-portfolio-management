//! Funding sources and financial-year money (DP-6). Rules in [`crate::financial_year`].
//!
//! `capex` is capital and `opex` is revenue (the existing budget-line category). The pack groups each
//! programme's budget lines by funding source, class and currency for the financial year the pack's
//! cut-off falls in: budget, spend to date and variance, with the existing whole-life forecast beside
//! them per currency. A line with no funding source is counted and listed, never hidden.

use super::refuse;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::prelude::*;
use axum::http::StatusCode;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::financial_year::{self as rules, Line, Row};
use crate::models::_entities::{budget_lines, funding_sources, plans};
use crate::models::audit_logs::Model as AuditModel;

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

/// Request body for adding a funding source.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SourceBody {
    /// Display name of the funding source; unique names and length are checked by the handler.
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
    let dup = all_sources(&ctx)
        .await?
        .into_iter()
        .any(|s| s.name.to_lowercase() == name.to_lowercase());
    if dup {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "a funding source with that name exists",
        ));
    }
    // The row and its audit row in one transaction (before SQ-1 batch 3 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    let row: funding_sources::Model = sqlx::query_as(
        "INSERT INTO funding_sources (pid, name, active, created_by)
         VALUES ($1, $2, TRUE, $3)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&name)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "funding_source_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid, "name": row.name, "active": row.active }))
}

/// `GET /api/funding-sources`.
#[debug_handler]
async fn list_sources(State(ctx): State<AppContext>) -> Result<Response> {
    let mut rows = all_sources(&ctx).await?;
    rows.sort_by_key(|r| r.name.to_lowercase());
    format::json(serde_json::json!({
        "funding_sources": rows.iter().map(|r| serde_json::json!({
            "pid": r.pid, "name": r.name, "active": r.active })).collect::<Vec<_>>(),
    }))
}

/// Request body for retiring or restoring a funding source.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ActiveBody {
    /// False retires the source (it stops being selectable); true restores it.
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
    let row = find_source(&ctx, pid).await?.ok_or(Error::NotFound)?;
    // The update and its audit row in one transaction (before SQ-1 batch 3 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    let row: funding_sources::Model =
        sqlx::query_as("UPDATE funding_sources SET active = $1 WHERE id = $2 RETURNING *")
            .bind(b.active)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "funding_source_updated",
        caller.actor(),
        Some(serde_json::json!({ "active": b.active })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid, "name": row.name, "active": row.active }))
}

/// Request body for setting a budget line's funding source.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LineSourceBody {
    /// Funding source to assign; absent clears the line's source, which then reports under "no funding source".
    #[serde(default)]
    funding_source_pid: Option<Uuid>,
}

/// Every funding source, in no particular order.
async fn all_sources(ctx: &AppContext) -> Result<Vec<funding_sources::Model>> {
    Ok(sqlx::query_as("SELECT * FROM funding_sources")
        .fetch_all(crate::db::pool(ctx))
        .await?)
}

/// One funding source by pid, if there is one.
async fn find_source(ctx: &AppContext, pid: Uuid) -> Result<Option<funding_sources::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM funding_sources WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// Whether a funding source exists and is active.
///
/// # Errors
///
/// A 422 when it does not.
pub(crate) async fn check_source(ctx: &AppContext, pid: Uuid) -> Result<()> {
    let ok = find_source(ctx, pid).await?.is_some_and(|s| s.active);
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
    let row: budget_lines::Model = sqlx::query_as(
        "SELECT * FROM budget_lines WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(line)
    .bind(plan.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    if let Some(s) = b.funding_source_pid {
        check_source(&ctx, s).await?;
    }
    // The update and its audit row in one transaction (before SQ-1 batch 3 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE budget_lines SET funding_source_pid = $1 WHERE id = $2 RETURNING id")
        .bind(b.funding_source_pid)
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "budget_line_funding_source_set",
        caller.actor(),
        Some(serde_json::json!({ "line": line, "funding_source_pid": b.funding_source_pid })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "line": line, "funding_source_pid": b.funding_source_pid }))
}

/// Query parameters for the portfolio money view.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MoneyQuery {
    /// The year the financial year starts in, e.g. 2026 for 2026/27 (April start).
    #[serde(default)]
    fy: Option<i32>,
    /// When true, capital and revenue classes are summed together; the default keeps them apart. Currencies are never summed.
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
    let all: Vec<Uuid> = live_plans(&ctx).await?.into_iter().map(|p| p.pid).collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let sm = rules::start_month();
    let day = match q.fy {
        Some(y) if (2000..=2100).contains(&y) => {
            NaiveDate::from_ymd_opt(y, sm, 1).ok_or_else(|| unprocessable("fy is not a year"))?
        }
        Some(_) => return Err(unprocessable("fy must be a year between 2000 and 2100")),
        None => crate::tz::deployment_today(),
    };
    let body = money_section(&ctx, &readable, day, q.combine_classes.unwrap_or(false)).await?;
    format::json(body)
}

/// Every live (not deleted) plan, in no particular order.
async fn live_plans(ctx: &AppContext) -> Result<Vec<plans::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plans WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// Walk up the plan parents (at most 64 steps, guarding against cycles) to the nearest ancestor that is a Program included in `include`; None when there is none.
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

/// JSON for one money row: source name (with fallbacks for none or unknown), class, currency, budget, spend and variance in minor currency units.
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
pub(crate) async fn money_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    day: NaiveDate,
    combine_classes: bool,
) -> Result<serde_json::Value> {
    let sm = rules::start_month();
    let year = rules::year_of(day, sm);
    let all = live_plans(ctx).await?;
    let by_pid: HashMap<Uuid, &plans::Model> = all.iter().map(|p| (p.pid, p)).collect();
    let names: HashMap<Uuid, String> = all_sources(ctx)
        .await?
        .into_iter()
        .map(|s| (s.pid, s.name))
        .collect();
    let rows: Vec<budget_lines::Model> =
        sqlx::query_as("SELECT * FROM budget_lines WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?;
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

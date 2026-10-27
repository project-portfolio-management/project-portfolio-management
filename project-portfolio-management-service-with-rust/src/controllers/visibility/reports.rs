//! Saved reports and running them.

use super::super::unprocessable;
use axum::http::{StatusCode, header};
use sea_orm::QueryOrder;
use serde::Deserialize;
use std::fmt::Write as _;
use uuid::Uuid;

use super::super::plans::parse_kind_label;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{budget_lines, plans, report_definitions};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::validation::MAX_TEXT_LEN;
use crate::visibility as rules;
use loco_rs::prelude::*;

/// `POST /api/reports` body (PPM-9).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReportPayload {
    name: String,
    collection: String,
    #[serde(default)]
    filters: serde_json::Value,
    fields: Vec<String>,
    /// Optional one-field grouping (T-28e); see [`GROUP_FIELDS`].
    #[serde(default)]
    group_by: Option<String>,
}

/// Columns a report may project.
const REPORT_FIELDS: &[&str] = &[
    "pid",
    "kind",
    "name",
    "stage",
    "status",
    "start_date",
    "target_date",
    "created_at",
];

/// Columns a report may group by — the categorical ones; `pid`, `name`
/// and `created_at` are unique-ish and would make one group per plan.
const GROUP_FIELDS: &[&str] = &["kind", "stage", "status", "start_date", "target_date"];

#[debug_handler]
pub(super) async fn create_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ReportPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if payload.name.trim().is_empty() || payload.name.len() > MAX_TEXT_LEN {
        problems.push("name is required (and capped)".to_string());
    }
    // `collection` is legacy metadata now that there is one recursive
    // `plans` collection: blank means "all plans", and a non-blank value
    // is accepted only as an optional kind filter
    // (portfolio/project/product/program/practice/process/purpose/pathway/proposal).
    if !payload.collection.trim().is_empty() && parse_kind_label(&payload.collection).is_none() {
        problems.push(
            "collection must be blank (all plans) or a kind (portfolio/project/product/program/practice/process/purpose/pathway/proposal)"
                .to_string(),
        );
    }
    if payload.fields.is_empty() {
        problems.push("fields must name at least one column".to_string());
    }
    for field in &payload.fields {
        if !REPORT_FIELDS.contains(&field.as_str()) {
            problems.push(format!(
                "unknown field {field:?} (allowed: {REPORT_FIELDS:?})"
            ));
        }
    }
    if let Some(group) = payload
        .group_by
        .as_deref()
        .filter(|g| !GROUP_FIELDS.contains(g))
    {
        problems.push(format!(
            "unknown group_by {group:?} (allowed: {GROUP_FIELDS:?})"
        ));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = report_definitions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(payload.name.clone()),
        collection: ActiveValue::set(payload.collection.clone()),
        filters: ActiveValue::set(payload.filters.clone()),
        fields: ActiveValue::set(serde_json::json!(payload.fields)),
        group_by: ActiveValue::set(payload.group_by.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "report_saved", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/reports` — saved definitions.
#[debug_handler]
pub(super) async fn list_reports(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = report_definitions::Entity::find()
        .filter(report_definitions::Column::DeletedAt.is_null())
        .order_by_asc(report_definitions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    format::json(rows)
}

/// `DELETE /api/reports/{pid}` — soft-delete a definition.
#[debug_handler]
pub(super) async fn delete_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = super::super::parse_pid(&pid)?;
    let row = vis_models::find_report(&ctx.db, pid).await?;
    let row_pid = row.pid;
    let mut active: report_definitions::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row_pid, "report_deleted", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::empty_json()
}

/// One projected report cell.
fn report_cell(model: &plans::Model, field: &str) -> String {
    match field {
        "pid" => model.pid.to_string(),
        "kind" => model
            .kind
            .clone()
            .unwrap_or_else(|| "unspecified".to_string()),
        "name" => model.name.clone(),
        "stage" => model.stage.clone().unwrap_or_default(),
        "created_at" => model.created_at.to_rfc3339(),
        "status" | "start_date" | "target_date" => model
            .data
            .get(field)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        _ => String::new(),
    }
}

/// `GET /api/reports/{pid}/run?format=json|csv` — execute a saved
/// report synchronously (row cap 1000; scheduled/bulk-artifact runs
/// are the family bulk machinery, roadmap).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RunParams {
    #[serde(default)]
    format: Option<String>,
}

#[debug_handler]
pub(super) async fn run_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(params): Query<RunParams>,
) -> Result<Response> {
    let pid = super::super::parse_pid(&pid)?;
    let def = vis_models::find_report(&ctx.db, pid).await?;
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    execute_report(&ctx, &def, params.format.as_deref(), &scope).await
}

/// Run a saved report definition to a response (shared by the on-demand run and
/// the scheduled run, T-28e, so the two can never produce different output).
pub(crate) async fn execute_report(
    ctx: &AppContext,
    def: &report_definitions::Model,
    format: Option<&str>,
    scope: &super::super::plan_access::HiddenScope,
) -> Result<Response> {
    let params = RunParams {
        format: format.map(str::to_string),
    };
    // The report's legacy `collection` is now an optional kind filter over
    // the single recursive `plans` collection: blank (or unrecognized) ⇒
    // all plans; a kind label ⇒ that kind only (filtered in-memory below).
    let kind_filter = parse_kind_label(&def.collection);
    let fields: Vec<String> = serde_json::from_value(def.fields.clone()).unwrap_or_default();
    let stage = def.filters.get("stage").and_then(|v| v.as_str());
    let status = def.filters.get("status").and_then(|v| v.as_str());
    let name_like = def
        .filters
        .get("name_like")
        .and_then(|v| v.as_str())
        .map(str::to_lowercase);
    let rows = scope.keep(crate::models::plans::Model::list(&ctx.db, 1000).await?);
    let selected: Vec<&plans::Model> = rows
        .iter()
        .filter(|m| {
            kind_filter
                .is_none_or(|want| m.kind.as_deref().and_then(parse_kind_label) == Some(want))
        })
        .filter(|m| stage.is_none_or(|want| m.stage.as_deref() == Some(want)))
        .filter(|m| {
            status.is_none_or(|want| m.data.get("status").and_then(|v| v.as_str()) == Some(want))
        })
        .filter(|m| {
            name_like
                .as_deref()
                .is_none_or(|want| m.name.to_lowercase().contains(want))
        })
        .collect();
    if let Some(group_field) = def.group_by.as_deref() {
        return run_grouped(
            ctx,
            &def.name,
            group_field,
            &selected,
            params.format.as_deref(),
        )
        .await;
    }
    if params.format.as_deref() == Some("csv") {
        let mut csv = fields
            .iter()
            .map(|f| rules::csv_field(f))
            .collect::<Vec<_>>()
            .join(",");
        csv.push('\n');
        for model in &selected {
            let line = fields
                .iter()
                .map(|f| rules::csv_field(&report_cell(model, f)))
                .collect::<Vec<_>>()
                .join(",");
            csv.push_str(&line);
            csv.push('\n');
        }
        return Ok((
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/csv"),
            )],
            csv,
        )
            .into_response());
    }
    let objects: Vec<serde_json::Value> = selected
        .iter()
        .map(|model| {
            fields
                .iter()
                .map(|f| (f.clone(), serde_json::json!(report_cell(model, f))))
                .collect::<serde_json::Map<String, serde_json::Value>>()
                .into()
        })
        .collect();
    format::json(serde_json::json!({
        "name": def.name,
        "collection": def.collection,
        "rows": objects.len(),
        "data": objects,
    }))
}

/// A grouped run (T-28e): one row per group value with the plan count
/// and the group's budget lines summed **per currency** — currencies are
/// never merged, and a group with no lines reports an empty `money`.
async fn run_grouped(
    ctx: &AppContext,
    name: &str,
    group_field: &str,
    selected: &[&plans::Model],
    format: Option<&str>,
) -> Result<Response> {
    let pids: Vec<Uuid> = selected.iter().map(|m| m.pid).collect();
    let lines = budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .filter(budget_lines::Column::PlanPid.is_in(pids))
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let entries = selected
        .iter()
        .map(|m| {
            let value = match report_cell(m, group_field) {
                v if v.is_empty() => "unspecified".to_string(),
                v => v,
            };
            let money = lines
                .iter()
                .filter(|b| b.plan_pid == m.pid)
                .map(|b| crate::insights::MoneyLine {
                    currency: b.currency.clone(),
                    planned_minor: b.planned_minor,
                    actual_minor: b.actual_minor,
                })
                .collect();
            (value, money)
        })
        .collect();
    let groups = crate::insights::group_report(entries);
    if format == Some("csv") {
        let mut csv = format!(
            "{},plans,currency,planned_minor,actual_minor,remaining_minor,line_count\n",
            rules::csv_field(group_field)
        );
        for g in &groups {
            let head = format!("{},{}", rules::csv_field(&g.value), g.count);
            if g.money.is_empty() {
                let _ = writeln!(csv, "{head},,,,,");
            }
            for r in &g.money {
                let _ = writeln!(
                    csv,
                    "{head},{},{},{},{},{}",
                    rules::csv_field(&r.currency),
                    r.planned_minor,
                    r.actual_minor,
                    r.remaining_minor,
                    r.line_count
                );
            }
        }
        return Ok((
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/csv"),
            )],
            csv,
        )
            .into_response());
    }
    format::json(serde_json::json!({
        "name": name,
        "group_by": group_field,
        "rows": selected.len(),
        "groups": groups,
    }))
}

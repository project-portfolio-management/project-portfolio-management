//! Printable output (PL-10): the board pack and a plan summary as PDF.
//! The writer is [`crate::pdf`] — dependency-free and deterministic.
//!
//! **A PDF is never more permissive than its JSON.** The board pack goes
//! through the same oversight authorisation as `GET /api/board/pack`, built
//! from the same function; the plan summary is read-checked like the plan, and
//! is rendered from the **masked** record and counts only: no lead, owner
//! organisation, assignee or risk owner ever reaches the text layer.
//!
//! **Deterministic when stamped.** The PDF states its `as_of`; pass
//! `?as_of=<RFC 3339>` and two renders of unchanged data are byte-identical.
//! Without it, `as_of` is the moment of rendering.
//!
//! **Not signed.** Signing needs a named signer (a person or a service key) and
//! a key policy; that decision is open (see the repo `plan.md`).

use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::Value;

use super::discussion::resolve_subject;
use crate::artifacts::content_disposition;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{milestones, plans, risks, tasks};
use crate::pdf;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

/// `?as_of=` — the stamp the document carries.
fn stamp(raw: Option<&str>) -> Result<DateTime<Utc>> {
    match raw {
        None => Ok(Utc::now()),
        Some(s) => DateTime::parse_from_rfc3339(s)
            .map(|d| d.to_utc())
            .map_err(|_| unprocessable("as_of must be an RFC 3339 instant")),
    }
}

fn respond(filename: &str, bytes: Vec<u8>) -> axum::response::Response {
    (
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (header::CONTENT_DISPOSITION, content_disposition(filename)),
            (
                header::HeaderName::from_static("x-content-type-options"),
                "nosniff".to_string(),
            ),
            (header::CACHE_CONTROL, "private, no-store".to_string()),
        ],
        bytes,
    )
        .into_response()
}

/// Print a JSON value as indented `key: value` lines (arrays as `- item`),
/// keys in their stored (sorted) order, skipping `skip` at the top level.
fn flatten(v: &Value, depth: usize, skip: &[&str], out: &mut Vec<String>) {
    let pad = "  ".repeat(depth);
    match v {
        Value::Object(map) => {
            for (k, val) in map {
                if depth == 0 && skip.contains(&k.as_str()) {
                    continue;
                }
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        out.push(format!("{pad}{k}:"));
                        flatten(val, depth + 1, skip, out);
                    }
                    other => out.push(format!("{pad}{k}: {}", scalar(other))),
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                match item {
                    Value::Object(map) => out.push(format!(
                        "{pad}- {}",
                        map.iter()
                            .map(|(k, x)| format!("{k}={}", compact(x)))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                    other => out.push(format!("{pad}- {}", scalar(other))),
                }
            }
        }
        other => out.push(format!("{pad}{}", scalar(other))),
    }
}

fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "-".to_string(),
        other => other.to_string(),
    }
}

fn compact(v: &Value) -> String {
    match v {
        Value::Object(_) | Value::Array(_) => v.to_string(),
        other => scalar(other),
    }
}

/// `GET /api/board/pack.pdf` query.
#[derive(Debug, Deserialize)]
struct PackParams {
    #[serde(default)]
    from: Option<String>,
    #[serde(default)]
    to: Option<String>,
    #[serde(default)]
    as_of: Option<String>,
}

/// `GET /api/board/pack.pdf?from=&to=&as_of=` — the board pack as a document.
#[debug_handler]
async fn board_pack_pdf(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<PackParams>,
) -> Result<axum::response::Response> {
    let as_of = stamp(p.as_of.as_deref())?;
    let body = super::oversight::authorized_board_pack(&ctx, &caller, p.from, p.to).await?;
    let window = &body["window"];
    let mut lines = vec![
        "BOARD PACK".to_string(),
        format!(
            "Window: {} to {}",
            scalar(&window["from"]),
            scalar(&window["to"])
        ),
        format!("As of: {}", as_of.to_rfc3339()),
        "Masking: this pack carries no people references.".to_string(),
        String::new(),
    ];
    flatten(&body, 0, &["as_of", "window"], &mut lines);
    Ok(respond("board-pack.pdf", pdf::render("Board pack", &lines)))
}

/// `GET /api/plans/{pid}/summary.pdf` query.
#[derive(Debug, Deserialize)]
struct SummaryParams {
    #[serde(default)]
    as_of: Option<String>,
}

/// `GET /api/plans/{pid}/summary.pdf` — a one-plan summary, masked, as a
/// document.
#[debug_handler]
async fn plan_summary_pdf(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(p): Query<SummaryParams>,
) -> Result<axum::response::Response> {
    let as_of = stamp(p.as_of.as_deref())?;
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let row = plans::Entity::find()
        .filter(plans::Column::Pid.eq(subject.pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let masked = crate::privacy::mask_plan(&row.to_plan()?);
    let task_rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(row.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for t in &task_rows {
        *by_status.entry(t.status.clone()).or_default() += 1;
    }
    let ms = milestones::Entity::find()
        .filter(milestones::Column::PlanPid.eq(row.pid))
        .filter(milestones::Column::DeletedAt.is_null())
        .order_by_asc(milestones::Column::Due)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let open_risks = risks::Entity::find()
        .filter(risks::Column::PlanPid.eq(row.pid))
        .filter(risks::Column::DeletedAt.is_null())
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    let plan_json = serde_json::to_value(&masked).unwrap_or_default();
    let field = |k: &str| scalar(&plan_json[k]);
    let mut lines = vec![
        format!("PLAN SUMMARY: {}", masked.name),
        format!("As of: {}", as_of.to_rfc3339()),
        "Masking: masked profile (no lead, owner organisation, assignee or risk owner)."
            .to_string(),
        String::new(),
        format!("Kind: {}", field("kind")),
        format!("Status: {}", field("status")),
        format!("Phase: {}", field("phase")),
        format!("Stage: {}", row.stage.clone().unwrap_or_else(|| "-".into())),
        format!(
            "Start: {}   Target: {}",
            field("start_date"),
            field("target_date")
        ),
    ];
    if !masked.keywords.is_empty() {
        lines.push(format!("Keywords: {}", masked.keywords.join(", ")));
    }
    if !masked.goals.is_empty() {
        lines.push(String::new());
        lines.push("Goals".to_string());
        lines.extend(masked.goals.iter().map(|g| format!("- {}", g.title)));
    }
    lines.push(String::new());
    lines.push(format!("Tasks: {} total", task_rows.len()));
    lines.extend(by_status.iter().map(|(s, n)| format!("  {s}: {n}")));
    lines.push(format!("Risks recorded: {open_risks}"));
    if !ms.is_empty() {
        lines.push(String::new());
        lines.push("Milestones".to_string());
        lines.extend(ms.iter().map(|m| {
            format!(
                "- {} due {}{}",
                m.name,
                m.due,
                if m.done { " (done)" } else { "" }
            )
        }));
    }
    Ok(respond(
        "plan-summary.pdf",
        pdf::render("Plan summary", &lines),
    ))
}

/// The printable-output routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/board/pack.pdf", get(board_pack_pdf))
        .add("/plans/{pid}/summary.pdf", get(plan_summary_pdf))
}

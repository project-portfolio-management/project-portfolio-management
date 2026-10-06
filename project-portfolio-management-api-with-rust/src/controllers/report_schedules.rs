//! Scheduled report runs (T-28e): a saved report run on a cadence. Pure rules
//! in [`crate::report_schedules`].
//!
//! Each due run executes the report through the **same code** as the on-demand
//! run ([`super::visibility::execute_report`]), stores the output as an
//! expiring artifact (30 days by default,
//! `PROJECT_PORTFOLIO_MANAGEMENT_REPORT_TTL_DAYS`), and is **audited like an
//! export — even at zero rows**. The sweep is claim-based (the next date is
//! moved with a conditional update before the work), so a ticker and an
//! external cron can never run one schedule twice, and a missed run runs once.
//! A failure is recorded on the schedule and the date still advances, so a
//! broken report cannot spin.
//!
//! Only the owner can read their schedules and download the latest output.

use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use chrono::{NaiveDate, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect, sea_query::Expr};
use serde::Deserialize;
use uuid::Uuid;

use crate::artifacts::{self, Store};
use crate::auth::AuthUser;
use crate::models::_entities::{artifacts as artifact_rows, report_definitions, report_schedules};
use crate::models::audit_logs::Model as AuditModel;
use crate::report_schedules as rules;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn ttl() -> chrono::Duration {
    let days = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_REPORT_TTL_DAYS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .unwrap_or(30);
    chrono::Duration::days(days.max(1))
}

fn view(s: &report_schedules::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": s.pid, "report_pid": s.report_pid, "cadence": s.cadence, "format": s.format,
        "next_run_date": s.next_run_date.to_string(), "enabled": s.enabled,
        "last_run_at": s.last_run_at, "last_status": s.last_status,
        "last_rows": s.last_rows, "last_error": s.last_error,
        "has_output": s.last_artifact_pid.is_some(),
    })
}

/// `POST /api/reports/{pid}/schedule` body.
#[derive(Debug, Deserialize)]
struct ScheduleBody {
    cadence: String,
    #[serde(default)]
    format: Option<String>,
    /// First run date (UTC, `YYYY-MM-DD`); default today.
    #[serde(default)]
    start: Option<String>,
}

/// `POST /api/reports/{pid}/schedule` — run this saved report on a cadence.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ScheduleBody>,
) -> Result<Response> {
    let report = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    report_definitions::Entity::find()
        .filter(report_definitions::Column::Pid.eq(report))
        .filter(report_definitions::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let format = b.format.as_deref().unwrap_or("csv");
    if !rules::CADENCES.contains(&b.cadence.as_str()) {
        return Err(unprocessable(&format!(
            "cadence must be one of {:?}",
            rules::CADENCES
        )));
    }
    if !rules::FORMATS.contains(&format) {
        return Err(unprocessable(&format!(
            "format must be one of {:?}",
            rules::FORMATS
        )));
    }
    let today = Utc::now().date_naive();
    let start = match b.start.as_deref() {
        None => today,
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| unprocessable("start must be a date, YYYY-MM-DD"))?,
    };
    let held = report_schedules::Entity::find()
        .filter(report_schedules::Column::Owner.eq(claims.sub.as_str()))
        .filter(report_schedules::Column::DeletedAt.is_null())
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    if held >= rules::MAX_PER_PERSON {
        return Err(unprocessable(&format!(
            "at most {} schedules per person",
            rules::MAX_PER_PERSON
        )));
    }
    let row = report_schedules::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        report_pid: ActiveValue::set(report),
        owner: ActiveValue::set(claims.sub.clone()),
        cadence: ActiveValue::set(b.cadence),
        format: ActiveValue::set(format.to_string()),
        anchor_date: ActiveValue::set(start),
        next_run_date: ActiveValue::set(start),
        enabled: ActiveValue::set(true),
        last_run_at: ActiveValue::set(None),
        last_status: ActiveValue::set(None),
        last_rows: ActiveValue::set(None),
        last_artifact_pid: ActiveValue::set(None),
        last_error: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    format::json(view(&row))
}

/// `GET /api/report-schedules` — the caller's own schedules.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    let rows = report_schedules::Entity::find()
        .filter(report_schedules::Column::Owner.eq(claims.sub.as_str()))
        .filter(report_schedules::Column::DeletedAt.is_null())
        .order_by_asc(report_schedules::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(view).collect::<Vec<_>>())
}

async fn own(ctx: &AppContext, owner: &str, pid: &str) -> Result<report_schedules::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    report_schedules::Entity::find()
        .filter(report_schedules::Column::Pid.eq(pid))
        .filter(report_schedules::Column::Owner.eq(owner))
        .filter(report_schedules::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `POST /api/report-schedules/{pid}/delete` (and `DELETE`) — stop a schedule.
/// Another person's is `404`, identically.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = own(&ctx, &claims.sub, &pid).await?;
    let mut a: report_schedules::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(Utc::now().into()));
    a.enabled = ActiveValue::set(false);
    a.update(&ctx.db).await.map_err(db_err)?;
    format::empty_json()
}

/// `GET /api/report-schedules/{pid}/latest` — the most recent output (owner
/// only). `404` when nothing has run yet or the artifact has expired.
#[debug_handler]
async fn latest(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = own(&ctx, &claims.sub, &pid).await?;
    let artifact = row.last_artifact_pid.ok_or(Error::NotFound)?;
    let (meta, bytes) = super::bulk::read_artifact(&ctx, artifact).await?;
    if meta.expires_at.is_some_and(|e| e < Utc::now()) {
        return Err(Error::NotFound);
    }
    Ok((
        [
            (header::CONTENT_TYPE, meta.content_type.clone()),
            (
                header::CONTENT_DISPOSITION,
                artifacts::content_disposition(&meta.filename),
            ),
            (
                header::HeaderName::from_static("x-content-type-options"),
                "nosniff".to_string(),
            ),
            (header::CACHE_CONTROL, "private, no-store".to_string()),
        ],
        bytes,
    )
        .into_response())
}

/// What one sweep did.
#[derive(Debug, Default, serde::Serialize)]
pub struct Swept {
    /// Runs that produced an artifact.
    pub ran: usize,
    /// Runs that failed (recorded on the schedule).
    pub failed: usize,
    /// Due schedules another sweeper had already claimed.
    pub already_claimed: usize,
}

/// Run every due schedule (at most `limit`). Claim-based.
///
/// # Errors
///
/// Propagates database errors from finding the due schedules.
pub async fn run_due(ctx: &AppContext, limit: u64) -> Result<Swept> {
    let today = Utc::now().date_naive();
    let due = report_schedules::Entity::find()
        .filter(report_schedules::Column::Enabled.eq(true))
        .filter(report_schedules::Column::DeletedAt.is_null())
        .filter(report_schedules::Column::NextRunDate.lte(today))
        .order_by_asc(report_schedules::Column::Id)
        .limit(limit)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = Swept::default();
    for s in due {
        let next = rules::after_run(&s.cadence, s.anchor_date, today)
            .unwrap_or(today + chrono::Duration::days(1));
        // The claim: only one sweeper moves the date off the value it read.
        let claimed = report_schedules::Entity::update_many()
            .col_expr(report_schedules::Column::NextRunDate, Expr::value(next))
            .filter(report_schedules::Column::Pid.eq(s.pid))
            .filter(report_schedules::Column::NextRunDate.eq(s.next_run_date))
            .exec(&ctx.db)
            .await
            .map_err(db_err)?;
        if claimed.rows_affected != 1 {
            out.already_claimed += 1;
            continue;
        }
        match run_one(ctx, &s).await {
            Ok(()) => out.ran += 1,
            Err(why) => {
                out.failed += 1;
                let mut a: report_schedules::ActiveModel = s.clone().into();
                a.last_run_at = ActiveValue::set(Some(Utc::now().into()));
                a.last_status = ActiveValue::set(Some("failed".to_string()));
                a.last_error = ActiveValue::set(Some(why.clone()));
                let _ = a.update(&ctx.db).await;
                AuditModel::record(
                    &ctx.db,
                    s.pid,
                    "report_run_failed",
                    Some(&s.owner),
                    Some(serde_json::json!({ "report_pid": s.report_pid, "error": why })),
                )
                .await
                .ok();
            }
        }
    }
    Ok(out)
}

async fn run_one(ctx: &AppContext, s: &report_schedules::Model) -> std::result::Result<(), String> {
    let def = report_definitions::Entity::find()
        .filter(report_definitions::Column::Pid.eq(s.report_pid))
        .filter(report_definitions::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "the report no longer exists".to_string())?;
    // A scheduled run has no signed-in reader, so it sees only what anyone may: the
    // private plans are out of it (a recipient who is a member opens the report themselves).
    let scope = super::plan_access::HiddenScope::of(ctx, &crate::auth::MaybeAuthUser(None))
        .await
        .map_err(|e| e.to_string())?;
    let resp = super::visibility::execute_report(ctx, &def, Some(&s.format), &scope)
        .await
        .map_err(|e| e.to_string())?;
    let bytes = axum::body::to_bytes(resp.into_body(), 64 << 20)
        .await
        .map_err(|e| e.to_string())?
        .to_vec();
    let body = String::from_utf8_lossy(&bytes).into_owned();
    let rows = rules::rows_in(&s.format, &body);
    let (ctype, ext) = if s.format == "csv" {
        ("text/csv", "csv")
    } else {
        ("application/json", "json")
    };
    let filename =
        artifacts::sanitize_filename(&format!("{}-{}.{ext}", def.name, Utc::now().date_naive()));
    let sha = artifacts::sha256_hex(&bytes);
    let size = i64::try_from(bytes.len()).unwrap_or(i64::MAX);
    let key = tokio::task::spawn_blocking(move || Store::from_env().put(&bytes))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| format!("could not store the file: {e}"))?;
    let artifact = artifact_rows::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        kind: ActiveValue::set("report_run".to_string()),
        filename: ActiveValue::set(filename),
        content_type: ActiveValue::set(ctype.to_string()),
        size_bytes: ActiveValue::set(size),
        sha256: ActiveValue::set(sha),
        storage_key: ActiveValue::set(Some(key)),
        created_by: ActiveValue::set(Some(s.owner.clone())),
        expires_at: ActiveValue::set(Some((Utc::now() + ttl()).into())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| e.to_string())?;
    let mut a: report_schedules::ActiveModel = s.clone().into();
    a.last_run_at = ActiveValue::set(Some(Utc::now().into()));
    a.last_status = ActiveValue::set(Some("ok".to_string()));
    a.last_rows = ActiveValue::set(Some(rows));
    a.last_artifact_pid = ActiveValue::set(Some(artifact.pid));
    a.last_error = ActiveValue::set(None);
    a.update(&ctx.db).await.map_err(|e| e.to_string())?;
    // Audited like an export: every run, even a zero-row one.
    AuditModel::record(
        &ctx.db,
        s.pid,
        "report_run_scheduled",
        Some(&s.owner),
        Some(
            serde_json::json!({ "report_pid": s.report_pid, "format": s.format, "rows": rows,
                                  "artifact_pid": artifact.pid }),
        ),
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// `POST /api/report-schedules/sweep` — run what is due now. Admin only (the
/// ticker does it too; this is for an external cron and for tests).
#[debug_handler]
async fn sweep(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    if !claims
        .attrs
        .get("access")
        .is_some_and(|v| v.iter().any(|a| a == "admin"))
    {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new("forbidden", "sweeping needs the admin attribute"),
        ));
    }
    format::json(run_due(&ctx, 200).await?)
}

/// The scheduled-report routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/reports/{pid}/schedule", post(create))
        .add("/report-schedules", get(list))
        .add("/report-schedules/sweep", post(sweep))
        .add("/report-schedules/{pid}", delete(remove))
        .add("/report-schedules/{pid}/delete", post(remove))
        .add("/report-schedules/{pid}/latest", get(latest))
}

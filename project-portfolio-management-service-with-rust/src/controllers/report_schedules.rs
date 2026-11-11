//! Scheduled report runs (T-28e): a saved report run on a cadence. Pure rules
//! in [`crate::report_schedules`].
//!
//! Each due run executes the report through the **same code** as the on-demand
//! run (`super::visibility::execute_report`), stores the output as an
//! expiring artifact (30 calendar days by default,
//! `PROJECT_PORTFOLIO_MANAGEMENT_REPORT_TTL_DAYS`), and is **audited like an
//! export — even at zero rows**. The sweep is claim-based (the next date is
//! moved with a conditional update before the work), so a ticker and an
//! external cron can never run one schedule twice, and a missed run runs once.
//! A failure is recorded on the schedule and the date still advances, so a
//! broken report cannot spin.
//!
//! Only the owner can read their schedules and download the latest output.

use super::unprocessable;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::artifacts::{self, Store};
use crate::auth::AuthUser;
use crate::models::_entities::{report_definitions, report_schedules};
use crate::models::audit_logs::Model as AuditModel;
use crate::report_schedules as rules;

/// How long a scheduled report file is kept: `PROJECT_PORTFOLIO_MANAGEMENT_REPORT_TTL_DAYS` days, default 30, never less than 1.
fn ttl() -> chrono::Duration {
    let days = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_REPORT_TTL_DAYS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .unwrap_or(30);
    chrono::Duration::days(days.max(1))
}

/// JSON view of a report schedule, including the outcome of its last run.
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ScheduleBody {
    /// How often the report runs; validated by the `report_schedules` rules.
    cadence: String,
    /// Output format of the run (`json` or `csv`); absent means `csv`.
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
    let report = super::parse_pid(&pid)?;
    find_report(&ctx, report).await?.ok_or(Error::NotFound)?;
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
    let today = crate::tz::deployment_today();
    let start = match b.start.as_deref() {
        None => today,
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| unprocessable("start must be a date, YYYY-MM-DD"))?,
    };
    let held: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM report_schedules WHERE owner = $1 AND deleted_at IS NULL",
    )
    .bind(claims.sub.as_str())
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    if u64::try_from(held).unwrap_or(0) >= rules::MAX_PER_PERSON {
        return Err(unprocessable(&format!(
            "at most {} schedules per person",
            rules::MAX_PER_PERSON
        )));
    }
    let row: report_schedules::Model = sqlx::query_as(
        "INSERT INTO report_schedules
             (pid, report_pid, owner, cadence, format, anchor_date, next_run_date, enabled, last_run_at, last_status,
              last_rows, last_artifact_pid, last_error, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $6, true, NULL, NULL, NULL, NULL, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(report)
    .bind(&claims.sub)
    .bind(&b.cadence)
    .bind(format)
    .bind(start)
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    format::json(view(&row))
}

/// `GET /api/report-schedules` — the caller's own schedules.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    let rows: Vec<report_schedules::Model> = sqlx::query_as(
        "SELECT * FROM report_schedules WHERE owner = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(claims.sub.as_str())
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(rows.iter().map(view).collect::<Vec<_>>())
}

/// Load the caller's own non-deleted schedule; another owner's schedule is `NotFound`, indistinguishable from a missing one.
async fn own(ctx: &AppContext, owner: &str, pid: &str) -> Result<report_schedules::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as(
        "SELECT * FROM report_schedules WHERE pid = $1 AND owner = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .bind(owner)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)
}

/// A live (not soft-deleted) saved report.
async fn find_report(
    ctx: &AppContext,
    pid: Uuid,
) -> std::result::Result<Option<report_definitions::Model>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM report_definitions WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await
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
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE report_schedules SET deleted_at = $1, enabled = false WHERE id = $2 RETURNING id",
    )
    .bind(now())
    .bind(row.id)
    .fetch_one(crate::db::pool(&ctx))
    .await?;
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
    let today = crate::tz::deployment_today();
    let due: Vec<report_schedules::Model> = sqlx::query_as(
        "SELECT * FROM report_schedules
         WHERE enabled = true AND deleted_at IS NULL AND next_run_date <= $1
         ORDER BY id
         LIMIT $2",
    )
    .bind(today)
    .bind(i64::try_from(limit).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let mut out = Swept::default();
    for s in due {
        let next = rules::after_run(&s.cadence, s.anchor_date, today)
            .unwrap_or(today + chrono::Duration::days(1));
        // The claim: only one sweeper moves the date off the value it read.
        let claimed = sqlx::query(
            "UPDATE report_schedules SET next_run_date = $1 WHERE pid = $2 AND next_run_date = $3",
        )
        .bind(next)
        .bind(s.pid)
        .bind(s.next_run_date)
        .execute(crate::db::pool(ctx))
        .await?;
        if claimed.rows_affected() != 1 {
            out.already_claimed += 1;
            continue;
        }
        match run_one(ctx, &s).await {
            Ok(()) => out.ran += 1,
            Err(why) => {
                out.failed += 1;
                let _ = sqlx::query_scalar::<_, i64>(
                    "UPDATE report_schedules
                     SET last_run_at = $1, last_status = 'failed', last_error = $2
                     WHERE id = $3
                     RETURNING id",
                )
                .bind(now())
                .bind(&why)
                .bind(s.id)
                .fetch_one(crate::db::pool(ctx))
                .await;
                AuditModel::record_or_log_on(
                    crate::db::pool(ctx),
                    s.pid,
                    "report_run_failed",
                    Some(&s.owner),
                    Some(serde_json::json!({ "report_pid": s.report_pid, "error": why })),
                )
                .await;
            }
        }
    }
    Ok(out)
}

/// Run one schedule: execute its report as an anonymous reader (so private plans are excluded), store the output as an expiring artifact, and record the outcome on the schedule. Returns the error text on failure.
async fn run_one(ctx: &AppContext, s: &report_schedules::Model) -> std::result::Result<(), String> {
    let def = find_report(ctx, s.report_pid)
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
    let filename = artifacts::sanitize_filename(&format!(
        "{}-{}.{ext}",
        def.name,
        crate::tz::deployment_today()
    ));
    let sha = artifacts::sha256_hex(&bytes);
    let size = i64::try_from(bytes.len()).unwrap_or(i64::MAX);
    let key = tokio::task::spawn_blocking(move || Store::from_env().put(&bytes))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| format!("could not store the file: {e}"))?;
    // The artifact row, the schedule's outcome and the audit row are one change (the stored bytes are not: a refused
    // change leaves an unreferenced blob, which nothing reads).
    let mut tx = crate::db::begin(ctx).await.map_err(|e| e.to_string())?;
    let artifact_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO artifacts
             (pid, kind, filename, content_type, size_bytes, sha256, storage_key, created_by, expires_at)
         VALUES ($1, 'report_run', $2, $3, $4, $5, $6, $7, $8)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(filename)
    .bind(ctype)
    .bind(size)
    .bind(sha)
    .bind(key)
    .bind(&s.owner)
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(Utc::now() + ttl()))
    .fetch_one(tx.conn())
    .await
    .map_err(|e| e.to_string())?;
    sqlx::query_scalar::<_, i64>(
        "UPDATE report_schedules
         SET last_run_at = $1, last_status = 'ok', last_rows = $2, last_artifact_pid = $3, last_error = NULL
         WHERE id = $4
         RETURNING id",
    )
    .bind(now())
    .bind(rows)
    .bind(artifact_pid)
    .bind(s.id)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| e.to_string())?;
    // Audited like an export: every run, even a zero-row one.
    AuditModel::record_on(
        tx.conn(),
        s.pid,
        "report_run_scheduled",
        Some(&s.owner),
        Some(
            serde_json::json!({ "report_pid": s.report_pid, "format": s.format, "rows": rows,
                                  "artifact_pid": artifact_pid }),
        ),
    )
    .await
    .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Now, as the `timestamptz` the schedule's columns hold.
fn now() -> chrono::DateTime<chrono::FixedOffset> {
    Utc::now().into()
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

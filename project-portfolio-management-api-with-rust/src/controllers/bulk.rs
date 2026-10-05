//! Bulk import / export of plans (T-8). Design: `spec/bulk-import-export.md`;
//! pure codecs and rules in [`crate::bulk`].
//!
//! A `POST` returns **202 with a job id** after storing the source (import) or
//! the authorised plan list (export) and queueing a [`BulkWorker`]; the work
//! runs there, reading and writing artifacts ([`crate::artifacts`]).
//!
//! **Authorisation happens when the job is requested**, with the caller's real
//! identity, because a worker has no claims: an export stores the list of
//! plans the caller may read (private plans and record-level policy applied);
//! `full` masking and soft-deleted rows need the *destructive* right. An
//! import is a destructive named POST at the guard.
//!
//! **Imports go through the normal create / update path** (`create_and_emit`,
//! `update_and_emit`), so events and audit are exactly as for a single request.
//! Per-row **record-level stage policy is not applied to imports** — the
//! background worker has no claims — which is why the whole import is a
//! destructive action. One bad row never aborts the load; the counters always
//! reconcile.

use axum::body::Bytes;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use loco_rs::bgworker::BackgroundWorker;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use project_portfolio_management_matcher::{MatchConfig, MatchingEngine};
use sea_orm::sea_query::{Expr, Func};
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::plan_access::readable_plans;
use crate::artifacts::{self, Store};
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::bulk::{self, Counters, Decoded, Key, RowError};
use crate::models::_entities::{artifacts as artifact_rows, bulk_jobs, plans, review_queue_items};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::plans::Model as PlanModel;
use crate::streaming;
use crate::workers::bulk::{BulkWorker, BulkWorkerArgs};

const ENTITY: &str = "plans";
/// Default import size cap: 50 MiB.
const DEFAULT_IMPORT_MAX: usize = 50 * 1024 * 1024;
/// A `running` job not touched for this long is considered abandoned.
const LEASE_SECS: i64 = 600;
/// Plans one export may carry.
const MAX_EXPORT_ROWS: u64 = 100_000;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn forbidden(message: &str) -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new("forbidden", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn import_max() -> usize {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_BULK_MAX_BYTES")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(DEFAULT_IMPORT_MAX)
}

fn ttl() -> chrono::Duration {
    let hours = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_BULK_TTL_HOURS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .unwrap_or(24);
    chrono::Duration::hours(hours.max(1))
}

fn is_admin(caller: &MaybeAuthUser) -> bool {
    caller.claims().is_some_and(|c| {
        c.attrs
            .get("access")
            .is_some_and(|v| v.iter().any(|a| a == "admin"))
    })
}

/// Whether the caller holds the destructive right (always true when
/// enforcement is off).
fn destructive_ok(caller: &MaybeAuthUser) -> bool {
    crate::auth::authorize_record(
        caller,
        authentication_verifier::Action::Destructive,
        &std::collections::BTreeMap::new(),
    )
    .is_ok()
}

// ───────────────────────────── job helpers ─────────────────────────────

async fn store_artifact(
    ctx: &AppContext,
    kind: &str,
    filename: &str,
    content_type: &str,
    bytes: Vec<u8>,
    actor: Option<&str>,
) -> Result<artifact_rows::Model> {
    let sha = artifacts::sha256_hex(&bytes);
    let size = i64::try_from(bytes.len()).unwrap_or(i64::MAX);
    let key = tokio::task::spawn_blocking(move || Store::from_env().put(&bytes))
        .await
        .map_err(|e| Error::string(&e.to_string()))?
        .map_err(|e| Error::string(&format!("could not store the file: {e}")))?;
    artifact_rows::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        kind: ActiveValue::set(kind.to_string()),
        filename: ActiveValue::set(filename.to_string()),
        content_type: ActiveValue::set(content_type.to_string()),
        size_bytes: ActiveValue::set(size),
        sha256: ActiveValue::set(sha),
        storage_key: ActiveValue::set(Some(key)),
        created_by: ActiveValue::set(actor.map(str::to_string)),
        expires_at: ActiveValue::set(Some((chrono::Utc::now() + ttl()).into())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)
}

pub(crate) async fn read_artifact(
    ctx: &AppContext,
    pid: Uuid,
) -> Result<(artifact_rows::Model, Vec<u8>)> {
    let row = artifact_rows::Entity::find()
        .filter(artifact_rows::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let key = row.storage_key.clone().ok_or(Error::NotFound)?;
    let bytes = tokio::task::spawn_blocking(move || Store::from_env().get(&key))
        .await
        .map_err(|e| Error::string(&e.to_string()))?
        .map_err(|_| Error::NotFound)?;
    if artifacts::sha256_hex(&bytes) != row.sha256 {
        return Err(Error::string(
            "a stored artifact failed its integrity check",
        ));
    }
    Ok((row, bytes))
}

async fn find_job(ctx: &AppContext, pid: &str, kind: Option<&str>) -> Result<bulk_jobs::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    let job = bulk_jobs::Entity::find()
        .filter(bulk_jobs::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if kind.is_some_and(|k| job.kind != k) {
        return Err(Error::NotFound);
    }
    Ok(job)
}

/// A job is visible to its requester (and to an admin). Anyone else gets the
/// same `404` as an unknown id.
fn visible(job: &bulk_jobs::Model, caller: &MaybeAuthUser) -> Result<()> {
    if job.requested_by.as_deref() == caller.actor() || is_admin(caller) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

fn job_view(
    job: &bulk_jobs::Model,
    artifact_expiry: Option<chrono::DateTime<chrono::Utc>>,
) -> serde_json::Value {
    let id = job.pid;
    let counters = Counters {
        total: job.rows_total,
        created: job.rows_created,
        updated: job.rows_updated,
        unchanged: job.rows_unchanged,
        queued_for_review: job.rows_queued_for_review,
        failed: job.rows_failed,
    };
    serde_json::json!({
        "job_id": id,
        "kind": job.kind,
        "status": job.status,
        "format": job.format,
        "counts": counters,
        "reconciles": counters.reconciles(),
        "rows_omitted": job.rows_omitted,
        "errors_url": (job.rows_failed > 0 && job.error_artifact.is_some())
            .then(|| format!("/api/plans/bulk-jobs/{id}/errors")),
        "review_url": (job.rows_queued_for_review > 0)
            .then(|| format!("/api/plans/review-queue?job={id}")),
        "download_url": (job.kind == "export" && job.result_artifact.is_some()
            && matches!(job.status.as_str(), "completed" | "completed_with_errors"))
            .then(|| format!("/api/plans/export/{id}/download")),
        "expires_at": artifact_expiry,
        "failure_reason": job.failure_reason,
        "progress": job.progress,
        "cancel_requested": job.cancel_requested_at.is_some(),
        "cancelled": job.status == "cancelled",
        "note": (job.status == "cancelled").then(|| format!(
            "cancelled: {} row(s) were processed and kept, {} were not reached",
            job.rows_total, job.rows_omitted)),
        "started_at": job.started_at,
        "finished_at": job.finished_at,
    })
}

async fn job_response(ctx: &AppContext, job: &bulk_jobs::Model) -> Result<Response> {
    let expiry = match job.result_artifact.or(job.source_artifact) {
        Some(a) => artifact_rows::Entity::find()
            .filter(artifact_rows::Column::Pid.eq(a))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .and_then(|r| r.expires_at.map(|e| e.to_utc())),
        None => None,
    };
    format::json(job_view(job, expiry))
}

fn accepted(job: &bulk_jobs::Model) -> Response {
    (
        StatusCode::ACCEPTED,
        axum::Json(serde_json::json!({ "job_id": job.pid, "status": job.status })),
    )
        .into_response()
}

async fn queue_job(ctx: &AppContext, job: &bulk_jobs::Model) -> Result<()> {
    BulkWorker::perform_later(ctx, BulkWorkerArgs { job_pid: job.pid })
        .await
        .map(|_| ())
}

// ───────────────────────────── import ─────────────────────────────

/// `POST /api/plans/import` query.
#[derive(Debug, Deserialize)]
struct ImportParams {
    /// `jsonl` (default) or `csv`.
    #[serde(default)]
    format: Option<String>,
    /// What a keyless row that looks like a duplicate does: `review`
    /// (default; held for a human) or `create` (no duplicate detection).
    #[serde(default)]
    on_keyless: Option<String>,
}

/// `POST /api/plans/import` — queue an import of the request body.
#[debug_handler]
async fn import(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Query(p): Query<ImportParams>,
    body: Bytes,
) -> Result<Response> {
    let format = p.format.as_deref().unwrap_or("jsonl");
    if !["jsonl", "csv"].contains(&format) {
        return Err(unprocessable("format must be jsonl or csv"));
    }
    let on_keyless = p.on_keyless.as_deref().unwrap_or("review");
    if !["review", "create"].contains(&on_keyless) {
        return Err(unprocessable("on_keyless must be review or create"));
    }
    if body.is_empty() {
        return Err(unprocessable("the import body is empty"));
    }
    if body.len() > import_max() {
        return Err(unprocessable(&format!(
            "the import is over the {} byte limit",
            import_max()
        )));
    }
    let key = headers
        .get("idempotency-key")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|k| !k.is_empty())
        .map(|k| k.chars().take(200).collect::<String>());
    if let Some(k) = &key
        && let Some(existing) = bulk_jobs::Entity::find()
            .filter(bulk_jobs::Column::Entity.eq(ENTITY))
            .filter(bulk_jobs::Column::Kind.eq("import"))
            .filter(bulk_jobs::Column::IdempotencyKey.eq(k.as_str()))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
    {
        // A retried request returns the original job.
        visible(&existing, &caller)?;
        return Ok((
            StatusCode::OK,
            axum::Json(serde_json::json!({ "job_id": existing.pid, "status": existing.status, "replayed": true })),
        )
            .into_response());
    }
    let ctype = if format == "csv" {
        "text/csv"
    } else {
        "application/x-ndjson"
    };
    let source = store_artifact(
        &ctx,
        "bulk_source",
        "import-source",
        ctype,
        body.to_vec(),
        caller.actor(),
    )
    .await?;
    let job = new_job(
        &ctx,
        "import",
        format,
        key,
        caller.actor(),
        serde_json::json!({ "on_keyless": on_keyless }),
        Some(source.pid),
    )
    .await?;
    AuditModel::record(
        &ctx.db,
        job.pid,
        "bulk_import_requested",
        caller.actor(),
        None,
    )
    .await
    .ok();
    queue_job(&ctx, &job).await?;
    // Re-read: in the foreground profile the job has already run.
    let fresh = find_job(&ctx, &job.pid.to_string(), None)
        .await
        .unwrap_or(job);
    Ok(accepted(&fresh))
}

async fn new_job(
    ctx: &AppContext,
    kind: &str,
    format: &str,
    key: Option<String>,
    actor: Option<&str>,
    params: serde_json::Value,
    source: Option<Uuid>,
) -> Result<bulk_jobs::Model> {
    bulk_jobs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        entity: ActiveValue::set(ENTITY.to_string()),
        kind: ActiveValue::set(kind.to_string()),
        status: ActiveValue::set("queued".to_string()),
        format: ActiveValue::set(format.to_string()),
        idempotency_key: ActiveValue::set(key),
        requested_by: ActiveValue::set(actor.map(str::to_string)),
        params: ActiveValue::set(params),
        rows_total: ActiveValue::set(0),
        rows_created: ActiveValue::set(0),
        rows_updated: ActiveValue::set(0),
        rows_unchanged: ActiveValue::set(0),
        rows_queued_for_review: ActiveValue::set(0),
        rows_failed: ActiveValue::set(0),
        rows_omitted: ActiveValue::set(0),
        source_artifact: ActiveValue::set(source),
        result_artifact: ActiveValue::set(None),
        error_artifact: ActiveValue::set(None),
        started_at: ActiveValue::set(None),
        finished_at: ActiveValue::set(None),
        failure_reason: ActiveValue::set(None),
        progress: ActiveValue::set(0),
        cancel_requested_at: ActiveValue::set(None),
        cancelled_by: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)
}

// ───────────────────────────── export ─────────────────────────────

/// `POST /api/plans/export` body.
#[derive(Debug, Deserialize)]
struct ExportBody {
    #[serde(default)]
    format: Option<String>,
    /// `masked` (default) or `full`.
    #[serde(default)]
    masking_profile: Option<String>,
    #[serde(default)]
    include_soft_deleted: bool,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    name_like: Option<String>,
}

/// `POST /api/plans/export` — queue an export of the plans the caller may read.
#[debug_handler]
async fn export(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<ExportBody>,
) -> Result<Response> {
    let format = b.format.as_deref().unwrap_or("jsonl");
    if !["jsonl", "csv"].contains(&format) {
        return Err(unprocessable("format must be jsonl or csv"));
    }
    let profile = b.masking_profile.as_deref().unwrap_or("masked");
    if !["masked", "full"].contains(&profile) {
        return Err(unprocessable("masking_profile must be masked or full"));
    }
    // People references are personal data: `full` and soft-deleted rows are
    // gated behind the destructive right.
    if (profile == "full" || b.include_soft_deleted) && !destructive_ok(&caller) {
        return Err(forbidden(
            "a full (unmasked) or soft-deleted export needs the destructive right",
        ));
    }
    let mut q = plans::Entity::find();
    if !b.include_soft_deleted {
        q = q.filter(plans::Column::DeletedAt.is_null());
    }
    if let Some(kind) = b.kind.as_deref().filter(|k| !k.trim().is_empty()) {
        q = q.filter(plans::Column::Kind.eq(kind));
    }
    if let Some(like) = b.name_like.as_deref().filter(|n| !n.trim().is_empty()) {
        // Typed, case-insensitive substring match: no hand-written SQL.
        // (Scoped import: the trait's `max` would shadow `Ord::max` elsewhere.)
        use sea_orm::ExprTrait;
        let pattern = format!("%{}%", like.trim().to_lowercase());
        q = q.filter(Expr::expr(Func::lower(Expr::col(plans::Column::Name))).like(pattern));
    }
    let candidates = q
        .order_by_asc(plans::Column::Id)
        .limit(MAX_EXPORT_ROWS + 1)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if candidates.len() as u64 > MAX_EXPORT_ROWS {
        return Err(unprocessable(&format!(
            "an export carries at most {MAX_EXPORT_ROWS} plans; narrow the filter"
        )));
    }
    let live: Vec<Uuid> = candidates
        .iter()
        .filter(|p| p.deleted_at.is_none())
        .map(|p| p.pid)
        .collect();
    let readable = readable_plans(&ctx, &caller, &live).await;
    // A soft-deleted plan is only reachable with the destructive right, which
    // was required above; a live one must be readable by this caller.
    let pids: Vec<Uuid> = candidates
        .iter()
        .filter(|p| p.deleted_at.is_some() || readable.contains(&p.pid))
        .map(|p| p.pid)
        .collect();
    let omitted = candidates.len() - pids.len();
    let job = new_job(
        &ctx,
        "export",
        format,
        None,
        caller.actor(),
        serde_json::json!({
            "masking_profile": profile,
            "include_soft_deleted": b.include_soft_deleted,
            "filter": { "kind": b.kind, "name_like": b.name_like },
            "pids": pids,
            "omitted": omitted,
        }),
        None,
    )
    .await?;
    queue_job(&ctx, &job).await?;
    let fresh = find_job(&ctx, &job.pid.to_string(), None)
        .await
        .unwrap_or(job);
    Ok(accepted(&fresh))
}

// ───────────────────────────── cancel ─────────────────────────────

/// `POST /api/plans/bulk-jobs/{id}/cancel` — ask a job to stop.
///
/// Cooperative: a **queued** job is cancelled at once (it never runs); a
/// **running** one is told to stop and ends `cancelled` after the row it is on,
/// keeping what it finished and counting what it did not reach. Cancelling a job
/// that has already finished (or already been cancelled) is a no-op, not an error.
/// Only the requester or an admin may cancel (anyone else gets the `404` an unknown
/// id would). Audited, once.
#[debug_handler]
async fn cancel_job(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(id): Path<String>,
) -> Result<Response> {
    let job = find_job(&ctx, &id, None).await?;
    visible(&job, &caller)?;
    let now = chrono::Utc::now();
    let who = caller.actor().map(str::to_string);
    // Queued: cancel it before it starts. The claim in `run_job` needs `queued`, so
    // the two cannot both win.
    let queued = bulk_jobs::Entity::update_many()
        .col_expr(bulk_jobs::Column::Status, Expr::value("cancelled"))
        .col_expr(bulk_jobs::Column::CancelledBy, Expr::value(who.clone()))
        .col_expr(bulk_jobs::Column::CancelRequestedAt, Expr::value(now))
        .col_expr(bulk_jobs::Column::FinishedAt, Expr::value(now))
        .col_expr(bulk_jobs::Column::UpdatedAt, Expr::value(now))
        .filter(bulk_jobs::Column::Pid.eq(job.pid))
        .filter(bulk_jobs::Column::Status.eq("queued"))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    let (changed, kind) = if queued.rows_affected == 1 {
        (true, "bulk_job_cancelled")
    } else {
        let running = bulk_jobs::Entity::update_many()
            .col_expr(bulk_jobs::Column::CancelRequestedAt, Expr::value(now))
            .col_expr(bulk_jobs::Column::CancelledBy, Expr::value(who.clone()))
            .col_expr(bulk_jobs::Column::UpdatedAt, Expr::value(now))
            .filter(bulk_jobs::Column::Pid.eq(job.pid))
            .filter(bulk_jobs::Column::Status.eq("running"))
            .filter(bulk_jobs::Column::CancelRequestedAt.is_null())
            .exec(&ctx.db)
            .await
            .map_err(db_err)?;
        (running.rows_affected == 1, "bulk_job_cancel_requested")
    };
    if changed {
        AuditModel::record(
            &ctx.db,
            job.pid,
            kind,
            caller.actor(),
            Some(serde_json::json!({ "kind": job.kind })),
        )
        .await
        .ok();
    }
    let now_job = find_job(&ctx, &id, None).await?;
    format::json(serde_json::json!({
        "job_id": now_job.pid,
        "status": now_job.status,
        "cancel_requested": now_job.cancel_requested_at.is_some(),
        "changed": changed,
        "note": if changed { "asked to stop: a running job finishes the row it is on, keeps what it processed, and ends cancelled" }
                else { "nothing to cancel: the job has already finished, or was already asked to stop" },
    }))
}

// ───────────────────────────── status / download ─────────────────────────────

/// `GET /api/plans/import/{id}`.
#[debug_handler]
async fn import_status(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(id): Path<String>,
) -> Result<Response> {
    let job = find_job(&ctx, &id, Some("import")).await?;
    visible(&job, &caller)?;
    job_response(&ctx, &job).await
}

/// `GET /api/plans/export/{id}`.
#[debug_handler]
async fn export_status(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(id): Path<String>,
) -> Result<Response> {
    let job = find_job(&ctx, &id, Some("export")).await?;
    visible(&job, &caller)?;
    job_response(&ctx, &job).await
}

/// `GET /api/plans/export/{id}/download` — the export file.
#[debug_handler]
async fn export_download(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(id): Path<String>,
) -> Result<axum::response::Response> {
    let job = find_job(&ctx, &id, Some("export")).await?;
    visible(&job, &caller)?;
    let artifact = job.result_artifact.ok_or(Error::NotFound)?;
    let (row, bytes) = read_artifact(&ctx, artifact).await?;
    Ok((
        [
            (header::CONTENT_TYPE, row.content_type.clone()),
            (
                header::CONTENT_DISPOSITION,
                artifacts::content_disposition(&row.filename),
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

/// `GET /api/plans/bulk-jobs` query.
#[derive(Debug, Deserialize)]
struct JobListParams {
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    status: Option<String>,
}

/// `GET /api/plans/bulk-jobs` — the caller's jobs (an admin sees all).
#[debug_handler]
async fn list_jobs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<JobListParams>,
) -> Result<Response> {
    let mut q = bulk_jobs::Entity::find().filter(bulk_jobs::Column::Entity.eq(ENTITY));
    if let Some(k) = p.kind {
        q = q.filter(bulk_jobs::Column::Kind.eq(k));
    }
    if let Some(s) = p.status {
        q = q.filter(bulk_jobs::Column::Status.eq(s));
    }
    let rows = q
        .order_by_desc(bulk_jobs::Column::Id)
        .limit(200)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mine: Vec<serde_json::Value> = rows
        .iter()
        .filter(|j| visible(j, &caller).is_ok())
        .map(|j| job_view(j, None))
        .collect();
    format::json(mine)
}

/// `GET /api/plans/bulk-jobs/{id}/errors` query.
#[derive(Debug, Deserialize)]
struct ErrorsParams {
    #[serde(default)]
    format: Option<String>,
}

/// `GET /api/plans/bulk-jobs/{id}/errors?format=csv|jsonl` — the per-row error
/// report: `row_number, source_line, field, code, message`.
#[debug_handler]
async fn job_errors(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(id): Path<String>,
    Query(p): Query<ErrorsParams>,
) -> Result<axum::response::Response> {
    let job = find_job(&ctx, &id, None).await?;
    visible(&job, &caller)?;
    let artifact = job.error_artifact.ok_or(Error::NotFound)?;
    let (_, bytes) = read_artifact(&ctx, artifact).await?;
    let text = String::from_utf8_lossy(&bytes).to_string();
    let errors: Vec<RowError> = text
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .map(|v| RowError {
            row_number: usize::try_from(v["row_number"].as_u64().unwrap_or(0)).unwrap_or(0),
            source_line: usize::try_from(v["source_line"].as_u64().unwrap_or(0)).unwrap_or(0),
            field: v["field"].as_str().unwrap_or_default().to_string(),
            code: code_of(v["code"].as_str().unwrap_or_default()),
            message: v["message"].as_str().unwrap_or_default().to_string(),
        })
        .collect();
    if p.format.as_deref() == Some("jsonl") {
        return Ok((
            [(header::CONTENT_TYPE, "application/x-ndjson".to_string())],
            bulk::errors_jsonl(&errors),
        )
            .into_response());
    }
    Ok((
        [(header::CONTENT_TYPE, "text/csv".to_string())],
        bulk::errors_csv(&errors),
    )
        .into_response())
}

/// Map a stored error code back to its `'static` form.
fn code_of(s: &str) -> &'static str {
    const CODES: &[&str] = &[
        "decode",
        "validation",
        "cycle",
        "unknown_pid",
        "search_unavailable",
        "unmapped_field",
        "conflict",
        "internal",
    ];
    CODES
        .iter()
        .find(|c| **c == s)
        .copied()
        .unwrap_or("internal")
}

// ───────────────────────────── review queue ─────────────────────────────

/// `GET /api/plans/review-queue` query.
#[derive(Debug, Deserialize)]
struct QueueParams {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    job: Option<Uuid>,
}

/// `GET /api/plans/review-queue` — rows held for a decision.
#[debug_handler]
async fn list_queue(
    State(ctx): State<AppContext>,
    Query(p): Query<QueueParams>,
) -> Result<Response> {
    let mut q = review_queue_items::Entity::find();
    q = q.filter(review_queue_items::Column::Status.eq(p.status.as_deref().unwrap_or("pending")));
    if let Some(job) = p.job {
        q = q.filter(review_queue_items::Column::JobPid.eq(job));
    }
    format::json(
        q.order_by_asc(review_queue_items::Column::Id)
            .limit(500)
            .all(&ctx.db)
            .await
            .map_err(db_err)?,
    )
}

async fn find_item(ctx: &AppContext, pid: &str) -> Result<review_queue_items::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    review_queue_items::Entity::find()
        .filter(review_queue_items::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `POST /api/plans/review-queue/{pid}/confirm` — it is a new plan after all:
/// create it. (Merging into a match is not implemented; reject and use
/// `POST /api/plans/merge`.)
#[debug_handler]
async fn confirm_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    if item.status != "pending" {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", format!("the item is already {}", item.status)),
        ));
    }
    let plan: project_portfolio_management_matcher::Plan =
        serde_json::from_value(item.candidate.clone())
            .map_err(|_| unprocessable("the held candidate is no longer a valid plan"))?;
    super::plans::validate(&plan)?;
    let model = streaming::create_and_emit(&ctx.db, &plan, caller.actor()).await?;
    let mut a: review_queue_items::ActiveModel = item.into();
    a.status = ActiveValue::set("confirmed".to_string());
    a.resolved_by = ActiveValue::set(caller.actor().map(str::to_string));
    a.resolved_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    a.resolution_pid = ActiveValue::set(Some(model.pid));
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    format::json(row)
}

/// `POST /api/plans/review-queue/{pid}/reject` — it is a duplicate: discard
/// the candidate (nothing was created).
#[debug_handler]
async fn reject_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    if item.status != "pending" {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", format!("the item is already {}", item.status)),
        ));
    }
    let mut a: review_queue_items::ActiveModel = item.into();
    a.status = ActiveValue::set("rejected".to_string());
    a.resolved_by = ActiveValue::set(caller.actor().map(str::to_string));
    a.resolved_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    format::json(a.update(&ctx.db).await.map_err(db_err)?)
}

// ───────────────────────────── sweep ─────────────────────────────

/// `POST /api/plans/bulk-jobs/sweep` — delete expired artifact files (the rows
/// stay as tombstones) and re-queue `running` jobs abandoned past the lease.
/// Admin only.
#[debug_handler]
async fn sweep(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    if !claims
        .attrs
        .get("access")
        .is_some_and(|v| v.iter().any(|a| a == "admin"))
    {
        return Err(forbidden("the sweep needs the admin attribute"));
    }
    format::json(run_sweep(&ctx).await?)
}

/// The sweep itself, shared with any scheduler.
///
/// # Errors
///
/// Propagates database errors.
pub async fn run_sweep(ctx: &AppContext) -> Result<serde_json::Value> {
    let now = chrono::Utc::now();
    let expired = artifact_rows::Entity::find()
        .filter(artifact_rows::Column::ExpiresAt.lt(now))
        .filter(artifact_rows::Column::StorageKey.is_not_null())
        .limit(1000)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut deleted = 0usize;
    for a in expired {
        if let Some(key) = a.storage_key.clone() {
            let _ = tokio::task::spawn_blocking(move || Store::from_env().remove(&key)).await;
        }
        let mut m: artifact_rows::ActiveModel = a.into();
        m.storage_key = ActiveValue::set(None);
        m.update(&ctx.db).await.map_err(db_err)?;
        deleted += 1;
    }
    let stale = bulk_jobs::Entity::find()
        .filter(bulk_jobs::Column::Status.eq("running"))
        .filter(bulk_jobs::Column::UpdatedAt.lt(now - chrono::Duration::seconds(LEASE_SECS)))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut requeued = 0usize;
    for j in stale {
        let mut m: bulk_jobs::ActiveModel = j.clone().into();
        m.status = ActiveValue::set("queued".to_string());
        m.updated_at = ActiveValue::set(now.into());
        m.update(&ctx.db).await.map_err(db_err)?;
        queue_job(ctx, &j).await?;
        requeued += 1;
    }
    Ok(serde_json::json!({ "artifacts_deleted": deleted, "jobs_requeued": requeued }))
}

// ───────────────────────────── the worker's work ─────────────────────────────

/// Run one queued job to its terminal state. Idempotent: a job that is not
/// `queued` (already claimed, or finished) is left alone.
///
/// # Errors
///
/// Propagates database errors; a job-level failure is recorded on the job
/// (`failed` + `failure_reason`) rather than returned.
pub async fn run_job(ctx: &AppContext, job_pid: Uuid) -> Result<()> {
    let now = chrono::Utc::now();
    let claimed = bulk_jobs::Entity::update_many()
        .col_expr(bulk_jobs::Column::Status, Expr::value("running"))
        .col_expr(bulk_jobs::Column::StartedAt, Expr::value(now))
        .col_expr(bulk_jobs::Column::UpdatedAt, Expr::value(now))
        .filter(bulk_jobs::Column::Pid.eq(job_pid))
        .filter(bulk_jobs::Column::Status.eq("queued"))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    if claimed.rows_affected != 1 {
        return Ok(()); // someone else holds it, or it is done
    }
    let job = find_job(ctx, &job_pid.to_string(), None).await?;
    let outcome = if job.kind == "import" {
        run_import(ctx, &job).await
    } else {
        run_export(ctx, &job).await
    };
    if let Err(reason) = outcome {
        let mut m: bulk_jobs::ActiveModel = find_job(ctx, &job_pid.to_string(), None).await?.into();
        m.status = ActiveValue::set("failed".to_string());
        m.failure_reason = ActiveValue::set(Some(reason));
        m.finished_at = ActiveValue::set(Some(chrono::Utc::now().into()));
        m.updated_at = ActiveValue::set(chrono::Utc::now().into());
        m.update(&ctx.db).await.map_err(db_err)?;
    }
    Ok(())
}

fn row_error(d: &Decoded, field: &str, code: &'static str, message: String) -> RowError {
    RowError {
        row_number: d.row_number,
        source_line: d.source_line,
        field: field.to_string(),
        code,
        message,
    }
}

fn text_of(e: &Error) -> String {
    match e {
        Error::CustomError(_, d) => d
            .description
            .clone()
            .or_else(|| d.error.clone())
            .unwrap_or_default(),
        other => other.to_string(),
    }
}

async fn find_by_key(ctx: &AppContext, key: &Key) -> Result<Option<PlanModel>> {
    use sea_orm::{DbBackend, Statement};
    let row = match key {
        Key::Pid(p) => {
            plans::Entity::find()
                .filter(plans::Column::DeletedAt.is_null())
                .filter(plans::Column::Pid.eq(*p))
                .one(&ctx.db)
                .await
        }
        // Bound parameters in a plain statement: no SQL is built from the
        // imported values.
        Key::Identifier { scheme, value } => {
            let probe = serde_json::json!([{ "scheme": scheme, "value": value }]).to_string();
            plans::Entity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    "SELECT * FROM plans WHERE deleted_at IS NULL \
                         AND (data->'identifiers') @> $1::jsonb ORDER BY id LIMIT 1",
                    [probe.into()],
                ))
                .one(&ctx.db)
                .await
        }
        Key::OwnerCode { owner, code } => {
            plans::Entity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    "SELECT * FROM plans WHERE deleted_at IS NULL \
                         AND data->>'owner_org_id' = $1 AND data->>'code' = $2 \
                         ORDER BY id LIMIT 1",
                    [owner.clone().into(), code.clone().into()],
                ))
                .one(&ctx.db)
                .await
        }
    }
    .map_err(db_err)?;
    Ok(row)
}

enum RowOutcome {
    Created,
    Updated,
    Unchanged,
    Queued,
}

async fn process_row(
    ctx: &AppContext,
    d: &Decoded,
    pid: Option<&str>,
    plan: &project_portfolio_management_matcher::Plan,
    on_keyless: &str,
    actor: Option<&str>,
    job_pid: Uuid,
) -> std::result::Result<RowOutcome, Vec<RowError>> {
    if let Err(e) = super::plans::validate(plan) {
        return Err(vec![row_error(d, "", "validation", text_of(&e))]);
    }
    let keys = bulk::keys_of(pid, plan);
    // A pid that names nothing is an error, never a silent create.
    let mut existing: Option<PlanModel> = None;
    for key in &keys {
        match find_by_key(ctx, key).await {
            Ok(Some(m)) => {
                existing = Some(m);
                break;
            }
            Ok(None) => {}
            Err(e) => return Err(vec![row_error(d, "", "internal", text_of(&e))]),
        }
    }
    if existing.is_none() && matches!(keys.first(), Some(Key::Pid(_))) {
        return Err(vec![row_error(
            d,
            "pid",
            "unknown_pid",
            "no live plan has this pid".into(),
        )]);
    }
    if let Some(model) = existing {
        let current = model
            .to_plan()
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])?;
        if serde_json::to_value(&current).ok() == serde_json::to_value(plan).ok() {
            return Ok(RowOutcome::Unchanged);
        }
        if let Err(e) = super::plans::check_no_cycle(ctx, Some(model.pid), plan).await {
            return Err(vec![row_error(d, "parent_ref", "cycle", text_of(&e))]);
        }
        return streaming::update_and_emit(&ctx.db, model, plan, actor)
            .await
            .map(|_| RowOutcome::Updated)
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())]);
    }
    if let Err(e) = super::plans::check_no_cycle(ctx, None, plan).await {
        return Err(vec![row_error(d, "parent_ref", "cycle", text_of(&e))]);
    }
    // Keyless ⇒ the normal duplicate detection decides.
    if keys.is_empty() && on_keyless == "review" {
        let Some(index) = crate::search::engine() else {
            return Err(vec![row_error(
                d,
                "",
                "search_unavailable",
                "the search index is unavailable, so duplicates cannot be checked".into(),
            )]);
        };
        let candidates = index
            .candidates(plan, 50)
            .map_err(|e| vec![row_error(d, "", "search_unavailable", e.to_string())])?;
        let pids: Vec<Uuid> = candidates
            .iter()
            .filter_map(|s| Uuid::parse_str(s).ok())
            .collect();
        let rows = PlanModel::find_by_pids(&ctx.db, &pids)
            .await
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])?;
        let engine = MatchingEngine::new(MatchConfig::default());
        let mut hits = Vec::new();
        for r in &rows {
            if let Ok(c) = r.to_plan() {
                let m = engine.match_plans(plan, &c);
                if m.is_match {
                    hits.push(
                        serde_json::json!({ "pid": r.pid, "name": r.name, "score": m.score,
                                                  "confidence": format!("{:?}", m.confidence) }),
                    );
                }
            }
        }
        if !hits.is_empty() {
            review_queue_items::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                status: ActiveValue::set("pending".to_string()),
                provenance: ActiveValue::set("import".to_string()),
                candidate: ActiveValue::set(serde_json::to_value(plan).unwrap_or_default()),
                matches: ActiveValue::set(serde_json::json!(hits)),
                job_pid: ActiveValue::set(Some(job_pid)),
                source_line: ActiveValue::set(i32::try_from(d.source_line).unwrap_or(i32::MAX)),
                resolved_by: ActiveValue::set(None),
                resolved_at: ActiveValue::set(None),
                resolution_pid: ActiveValue::set(None),
                resolution_comment: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])?;
            return Ok(RowOutcome::Queued);
        }
    }
    streaming::create_and_emit(&ctx.db, plan, actor)
        .await
        .map(|_| RowOutcome::Created)
        .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])
}

async fn finish(
    ctx: &AppContext,
    job: &bulk_jobs::Model,
    c: Counters,
    omitted: i64,
    result: Option<Uuid>,
    errors: Option<Uuid>,
) -> std::result::Result<(), String> {
    let mut m: bulk_jobs::ActiveModel = job.clone().into();
    m.status = ActiveValue::set(c.status().to_string());
    m.progress = ActiveValue::set(100);
    m.rows_total = ActiveValue::set(c.total);
    m.rows_created = ActiveValue::set(c.created);
    m.rows_updated = ActiveValue::set(c.updated);
    m.rows_unchanged = ActiveValue::set(c.unchanged);
    m.rows_queued_for_review = ActiveValue::set(c.queued_for_review);
    m.rows_failed = ActiveValue::set(c.failed);
    m.rows_omitted = ActiveValue::set(omitted);
    m.result_artifact = ActiveValue::set(result);
    m.error_artifact = ActiveValue::set(errors);
    m.finished_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    m.updated_at = ActiveValue::set(chrono::Utc::now().into());
    m.update(&ctx.db)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Record how far a running job is. A trigger in the database makes the stored
/// value monotone even if two updates race or a caller passes a lower number.
async fn set_progress(ctx: &AppContext, job: Uuid, percent: i32) {
    let _ = bulk_jobs::Entity::update_many()
        .col_expr(
            bulk_jobs::Column::Progress,
            Expr::cust_with_values("GREATEST(progress, $1)", [percent]),
        )
        .filter(bulk_jobs::Column::Pid.eq(job))
        .filter(bulk_jobs::Column::Status.eq("running"))
        .exec(&ctx.db)
        .await;
}

/// Whether someone asked for this job to stop.
async fn cancel_requested(ctx: &AppContext, job: Uuid) -> bool {
    bulk_jobs::Entity::find()
        .filter(bulk_jobs::Column::Pid.eq(job))
        .one(&ctx.db)
        .await
        .ok()
        .flatten()
        .is_some_and(|j| j.cancel_requested_at.is_some())
}

/// End a job that was cancelled part-way: `cancelled`, with what was processed kept
/// and counted and what was not reached in `rows_omitted`.
async fn finish_cancelled(
    ctx: &AppContext,
    job: &bulk_jobs::Model,
    c: Counters,
    not_reached: i64,
    errors: Option<Uuid>,
) -> std::result::Result<(), String> {
    let mut m: bulk_jobs::ActiveModel = job.clone().into();
    m.status = ActiveValue::set(bulk::Counters::cancelled_status().to_string());
    m.rows_total = ActiveValue::set(c.total);
    m.rows_created = ActiveValue::set(c.created);
    m.rows_updated = ActiveValue::set(c.updated);
    m.rows_unchanged = ActiveValue::set(c.unchanged);
    m.rows_queued_for_review = ActiveValue::set(c.queued_for_review);
    m.rows_failed = ActiveValue::set(c.failed);
    m.rows_omitted = ActiveValue::set(not_reached);
    m.error_artifact = ActiveValue::set(errors);
    m.finished_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    m.updated_at = ActiveValue::set(chrono::Utc::now().into());
    m.update(&ctx.db)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())?;
    AuditModel::record(
        &ctx.db,
        job.pid,
        "bulk_job_cancelled",
        job.cancelled_by.as_deref().or(job.requested_by.as_deref()),
        Some(serde_json::json!({ "counts": c, "not_reached": not_reached })),
    )
    .await
    .ok();
    Ok(())
}

async fn run_import(ctx: &AppContext, job: &bulk_jobs::Model) -> std::result::Result<(), String> {
    let source = job.source_artifact.ok_or("the job has no source file")?;
    let (_, bytes) = read_artifact(ctx, source).await.map_err(|e| text_of(&e))?;
    let text = String::from_utf8(bytes).map_err(|_| "the source is not valid UTF-8".to_string())?;
    let decoded = if job.format == "csv" {
        bulk::decode_csv(&text)?
    } else {
        bulk::decode_jsonl(&text)
    };
    if decoded.len() > bulk::MAX_ROWS {
        return Err(format!("an import carries at most {} rows", bulk::MAX_ROWS));
    }
    let on_keyless = job.params["on_keyless"]
        .as_str()
        .unwrap_or("review")
        .to_string();
    let actor = job.requested_by.as_deref();
    let mut counters = Counters::default();
    let mut errors: Vec<RowError> = Vec::new();
    let total_rows = i64::try_from(decoded.len()).unwrap_or(i64::MAX);
    let mut cancelled = cancel_requested(ctx, job.pid).await;
    for (n, d) in decoded.iter().enumerate() {
        if cancelled {
            break;
        }
        counters.total += 1;
        match &d.result {
            Err(errs) => {
                counters.failed += 1;
                errors.extend(errs.iter().cloned());
            }
            Ok((pid, plan)) => {
                match process_row(ctx, d, pid.as_deref(), plan, &on_keyless, actor, job.pid).await {
                    Ok(RowOutcome::Created) => counters.created += 1,
                    Ok(RowOutcome::Updated) => counters.updated += 1,
                    Ok(RowOutcome::Unchanged) => counters.unchanged += 1,
                    Ok(RowOutcome::Queued) => counters.queued_for_review += 1,
                    Err(errs) => {
                        counters.failed += 1;
                        errors.extend(errs);
                    }
                }
            }
        }
        // Each row is complete before the next begins, so stopping here leaves no
        // half-written row. Look at the cancel flag every few rows, and say how far we are.
        if (n + 1) % bulk::CANCEL_CHECK_EVERY == 0 {
            set_progress(
                ctx,
                job.pid,
                bulk::progress_percent(counters.total, total_rows),
            )
            .await;
            cancelled = cancel_requested(ctx, job.pid).await;
        }
    }
    debug_assert!(counters.reconciles());
    let error_artifact = if errors.is_empty() {
        None
    } else {
        Some(
            store_artifact(
                ctx,
                "bulk_errors",
                "import-errors.jsonl",
                "application/x-ndjson",
                bulk::errors_jsonl(&errors).into_bytes(),
                actor,
            )
            .await
            .map_err(|e| text_of(&e))?
            .pid,
        )
    };
    if cancelled && counters.total < total_rows {
        return finish_cancelled(
            ctx,
            job,
            counters,
            total_rows - counters.total,
            error_artifact,
        )
        .await;
    }
    finish(ctx, job, counters, 0, None, error_artifact).await?;
    AuditModel::record(
        &ctx.db,
        job.pid,
        "bulk_import_finished",
        actor,
        Some(serde_json::json!({ "counts": counters })),
    )
    .await
    .ok();
    Ok(())
}

#[allow(clippy::too_many_lines)]
async fn run_export(ctx: &AppContext, job: &bulk_jobs::Model) -> std::result::Result<(), String> {
    let pids: Vec<Uuid> = job.params["pids"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().and_then(|s| Uuid::parse_str(s).ok()))
                .collect()
        })
        .unwrap_or_default();
    let masked = job.params["masking_profile"].as_str() != Some("full");
    let omitted = job.params["omitted"].as_i64().unwrap_or(0);
    let mut items: Vec<(Uuid, serde_json::Value)> = Vec::new();
    let chunks = i64::try_from(pids.chunks(500).count()).unwrap_or(1);
    for (n, chunk) in pids.chunks(500).enumerate() {
        if cancel_requested(ctx, job.pid).await {
            // A partial export would pass for a whole one, so a cancelled export
            // produces no file; it says so and reports how far it got.
            let c = Counters::default();
            let done = i64::try_from(items.len()).unwrap_or(i64::MAX);
            let c = Counters {
                total: done,
                unchanged: done,
                ..c
            };
            return finish_cancelled(
                ctx,
                job,
                c,
                i64::try_from(pids.len()).unwrap_or(0) - done,
                None,
            )
            .await;
        }
        set_progress(
            ctx,
            job.pid,
            bulk::progress_percent(i64::try_from(n).unwrap_or(0), chunks),
        )
        .await;
        let rows = plans::Entity::find()
            .filter(plans::Column::Pid.is_in(chunk.to_vec()))
            .order_by_asc(plans::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(|e| e.to_string())?;
        for r in rows {
            let plan = PlanModel::to_plan(&r).map_err(|e| e.to_string())?;
            let plan = if masked {
                crate::privacy::mask_plan(&plan)
            } else {
                plan
            };
            items.push((
                r.pid,
                serde_json::to_value(&plan).map_err(|e| e.to_string())?,
            ));
        }
    }
    items.sort_by_key(|(pid, _)| *pid);
    let (body, ctype, name) = if job.format == "csv" {
        (bulk::encode_csv(&items), "text/csv", "plans-export.csv")
    } else {
        (
            bulk::encode_jsonl(&items),
            "application/x-ndjson",
            "plans-export.jsonl",
        )
    };
    let artifact = store_artifact(
        ctx,
        "bulk_result",
        name,
        ctype,
        body.into_bytes(),
        job.requested_by.as_deref(),
    )
    .await
    .map_err(|e| text_of(&e))?;
    let counters = Counters {
        total: i64::try_from(items.len()).unwrap_or(i64::MAX),
        created: 0,
        updated: 0,
        unchanged: i64::try_from(items.len()).unwrap_or(i64::MAX),
        queued_for_review: 0,
        failed: 0,
    };
    finish(ctx, job, counters, omitted, Some(artifact.pid), None).await?;
    // Every export is audited — **even a zero-row one**.
    AuditModel::record(
        &ctx.db,
        job.pid,
        "bulk_export",
        job.requested_by.as_deref(),
        Some(serde_json::json!({
            "masking_profile": job.params["masking_profile"],
            "include_soft_deleted": job.params["include_soft_deleted"],
            "filter": job.params["filter"],
            "rows": items.len(),
            "omitted": omitted,
        })),
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// The bulk routes.
pub fn routes() -> Routes {
    let ceiling = import_max().saturating_add(64 * 1024);
    Routes::new()
        .prefix("/api")
        .add(
            "/plans/import",
            post(import).layer(DefaultBodyLimit::max(ceiling)),
        )
        .add("/plans/import/{id}", get(import_status))
        .add("/plans/export", post(export))
        .add("/plans/export/{id}", get(export_status))
        .add("/plans/export/{id}/download", get(export_download))
        .add("/plans/bulk-jobs", get(list_jobs))
        .add("/plans/bulk-jobs/sweep", post(sweep))
        .add("/plans/bulk-jobs/{id}/errors", get(job_errors))
        .add("/plans/bulk-jobs/{id}/cancel", post(cancel_job))
        .add("/plans/review-queue", get(list_queue))
        .add("/plans/review-queue/{pid}/confirm", post(confirm_item))
        .add("/plans/review-queue/{pid}/reject", post(reject_item))
}

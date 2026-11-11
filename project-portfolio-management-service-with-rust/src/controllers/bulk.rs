//! Bulk import / export of plans (T-8). Design: `spec/bulk-import-export.md`;
//! pure codecs and rules in [`crate::bulk`].
//!
//! A `POST` returns **202 with a job id** after storing the source (import) or
//! the authorized plan list (export) and queueing a [`BulkWorker`]; the work
//! runs there, reading and writing artifacts ([`crate::artifacts`]).
//!
//! **Authorization happens when the job is requested**, with the caller's real
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

use super::forbidden;
use super::unprocessable;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axa::middleware::body_limit;
use axa::worker::BackgroundWorker;
use axum::body::Bytes;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use project_portfolio_management_matcher::{MatchConfig, MatchingEngine};
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

/// Entity name recorded on every `bulk_jobs` row; only plans are bulk-loaded today.
const ENTITY: &str = "plans";
/// Default import size cap: 50 MiB.
const DEFAULT_IMPORT_MAX: usize = 50 * 1024 * 1024;
/// A `running` job not touched for this long is considered abandoned.
const LEASE_SECS: i64 = 600;
/// Plans one export may carry.
const MAX_EXPORT_ROWS: u64 = 100_000;

/// Now, as a time stamp with an offset, the type the time columns bind as.
fn stamp() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().into()
}

/// Maximum accepted import body in bytes: `PROJECT_PORTFOLIO_MANAGEMENT_BULK_MAX_BYTES` when it parses, otherwise the 50 MiB default.
fn import_max() -> usize {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_BULK_MAX_BYTES")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(DEFAULT_IMPORT_MAX)
}

/// How long a stored source/result artifact lives before it expires: `PROJECT_PORTFOLIO_MANAGEMENT_BULK_TTL_HOURS` (default 24), never less than one hour.
fn ttl() -> chrono::Duration {
    let hours = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_BULK_TTL_HOURS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .unwrap_or(24);
    chrono::Duration::hours(hours.max(1))
}

/// Whether the caller is an administrator under the plan-access rules (used to let admins see every job).
fn is_admin(caller: &MaybeAuthUser) -> bool {
    super::plan_access::is_admin(caller.claims())
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

/// Writes `bytes` to the artifact store and records an `artifacts` row (size, SHA-256, expiry from [`ttl`], creator). The hash is what [`read_artifact`] later verifies.
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
    // The columns `SeaORM`'s insert set, no more.
    Ok(sqlx::query_as(
        "INSERT INTO artifacts
             (pid, kind, filename, content_type, size_bytes, sha256, storage_key, created_by, expires_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(kind)
    .bind(filename)
    .bind(content_type)
    .bind(size)
    .bind(sha)
    .bind(key)
    .bind(actor)
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(
        chrono::Utc::now() + ttl(),
    ))
    .fetch_one(crate::db::pool(ctx))
    .await?)
}

/// Loads an artifact row and its bytes from the store, returning `NotFound` when the row, key or object is missing and an error when the stored hash no longer matches (integrity check).
pub(crate) async fn read_artifact(
    ctx: &AppContext,
    pid: Uuid,
) -> Result<(artifact_rows::Model, Vec<u8>)> {
    let row: artifact_rows::Model =
        sqlx::query_as("SELECT * FROM artifacts WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
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

/// Fetches a job by public id; when `kind` is given and differs (import vs export) the job is reported as `NotFound` rather than leaking its existence.
async fn find_job(ctx: &AppContext, pid: &str, kind: Option<&str>) -> Result<bulk_jobs::Model> {
    let pid = super::parse_pid(pid)?;
    let job: bulk_jobs::Model = sqlx::query_as("SELECT * FROM bulk_jobs WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
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

/// Builds the JSON representation of a job: status, progress, row counters and artifact references, with the artifact expiry when one is known.
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

/// Renders a job as an HTTP JSON response, looking up the expiry of its result (or else source) artifact for the view.
async fn job_response(ctx: &AppContext, job: &bulk_jobs::Model) -> Result<Response> {
    let expiry = match job.result_artifact.or(job.source_artifact) {
        // Only the expiry is read.
        Some(a) => sqlx::query_scalar::<_, Option<chrono::DateTime<chrono::FixedOffset>>>(
            "SELECT expires_at FROM artifacts WHERE pid = $1 LIMIT 1",
        )
        .bind(a)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .flatten()
        .map(|e| e.to_utc()),
        None => None,
    };
    format::json(job_view(job, expiry))
}

/// The `202 Accepted` response returned when a job is queued, carrying the job id and its status.
fn accepted(job: &bulk_jobs::Model) -> Response {
    (
        StatusCode::ACCEPTED,
        axum::Json(serde_json::json!({ "job_id": job.pid, "status": job.status })),
    )
        .into_response()
}

/// Hands the job to the background [`BulkWorker`]; the worker re-reads the job by pid, so only the pid travels.
async fn queue_job(ctx: &AppContext, job: &bulk_jobs::Model) -> Result<()> {
    BulkWorker::perform_later(ctx, BulkWorkerArgs { job_pid: job.pid })
        .await
        .map(|_| ())
}

// ───────────────────────────── import ─────────────────────────────

/// `POST /api/plans/import` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ImportParams {
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
        && let Some(existing) = sqlx::query_as::<_, bulk_jobs::Model>(
            "SELECT * FROM bulk_jobs WHERE entity = $1 AND kind = 'import' AND idempotency_key = $2 LIMIT 1",
        )
        .bind(ENTITY)
        .bind(k.as_str())
        .fetch_optional(crate::db::pool(&ctx))
        .await?
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
    // The job runs after this request, with no token to ask: what was hidden from the caller travels with it.
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .plan_pids();
    // The job row and the audit row of its request are one change.
    let mut tx = crate::db::begin(&ctx).await?;
    let job = new_job(
        &mut tx,
        "import",
        format,
        key,
        caller.actor(),
        serde_json::json!({ "on_keyless": on_keyless, "hidden_plans": hidden }),
        Some(source.pid),
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        job.pid,
        "bulk_import_requested",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    queue_job(&ctx, &job).await?;
    // Re-read: in the foreground profile the job has already run.
    let fresh = find_job(&ctx, &job.pid.to_string(), None)
        .await
        .unwrap_or(job);
    Ok(accepted(&fresh))
}

/// Inserts a new `queued` job row with all counters zero and no timing or failure data. On a [`crate::db::Session`],
/// so a caller may use a transaction.
async fn new_job(
    db: &mut impl crate::db::Session,
    kind: &str,
    format: &str,
    key: Option<String>,
    actor: Option<&str>,
    params: serde_json::Value,
    source: Option<Uuid>,
) -> Result<bulk_jobs::Model> {
    // The columns `SeaORM`'s insert set, no more.
    Ok(sqlx::query_as(
        "INSERT INTO bulk_jobs
             (pid, entity, kind, status, format, idempotency_key, requested_by, params, rows_total, rows_created,
              rows_updated, rows_unchanged, rows_queued_for_review, rows_failed, rows_omitted, source_artifact,
              result_artifact, error_artifact, started_at, finished_at, failure_reason, progress, cancel_requested_at,
              cancelled_by)
         VALUES ($1, $2, $3, 'queued', $4, $5, $6, $7, 0, 0, 0, 0, 0, 0, 0, $8, NULL, NULL, NULL, NULL, NULL, 0, NULL,
                 NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(ENTITY)
    .bind(kind)
    .bind(format)
    .bind(key)
    .bind(actor)
    .bind(params)
    .bind(source)
    .fetch_one(db.ex())
    .await?)
}

// ───────────────────────────── export ─────────────────────────────

/// `POST /api/plans/export` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ExportBody {
    #[serde(default)]
    /// Output format, `csv` or `jsonl`; absent means the endpoint default.
    format: Option<String>,
    /// `masked` (default) or `full`.
    #[serde(default)]
    masking_profile: Option<String>,
    #[serde(default)]
    /// Whether soft-deleted plans are included; needs the destructive right.
    include_soft_deleted: bool,
    #[serde(default)]
    /// Optional plan kind filter.
    kind: Option<String>,
    #[serde(default)]
    /// Optional case-insensitive name substring filter.
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
    // SQL from literals; every filter value is bound.
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM plans WHERE TRUE");
    if !b.include_soft_deleted {
        q.push(" AND deleted_at IS NULL");
    }
    if let Some(kind) = b.kind.as_deref().filter(|k| !k.trim().is_empty()) {
        q.push(" AND kind = ").push_bind(kind);
    }
    if let Some(like) = b.name_like.as_deref().filter(|n| !n.trim().is_empty()) {
        // A case-insensitive substring match; `%` and `_` in the filter keep their `LIKE` meaning, as before.
        let pattern = format!("%{}%", like.trim().to_lowercase());
        q.push(" AND LOWER(name) LIKE ").push_bind(pattern);
    }
    q.push(" ORDER BY id LIMIT ")
        .push_bind(i64::try_from(MAX_EXPORT_ROWS + 1).unwrap_or(i64::MAX));
    let candidates: Vec<plans::Model> = q.build_query_as().fetch_all(crate::db::pool(&ctx)).await?;
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
        &mut crate::db::pool(&ctx),
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
    let mut tx = crate::db::begin(&ctx).await?;
    let queued = sqlx::query(
        "UPDATE bulk_jobs
         SET status = 'cancelled', cancelled_by = $1, cancel_requested_at = $2, finished_at = $2, updated_at = $2
         WHERE pid = $3 AND status = 'queued'",
    )
    .bind(&who)
    .bind(now)
    .bind(job.pid)
    .execute(tx.conn())
    .await?;
    let (changed, kind) = if queued.rows_affected() == 1 {
        (true, "bulk_job_cancelled")
    } else {
        let running = sqlx::query(
            "UPDATE bulk_jobs
             SET cancel_requested_at = $1, cancelled_by = $2, updated_at = $1
             WHERE pid = $3 AND status = 'running' AND cancel_requested_at IS NULL",
        )
        .bind(now)
        .bind(&who)
        .bind(job.pid)
        .execute(tx.conn())
        .await?;
        (running.rows_affected() == 1, "bulk_job_cancel_requested")
    };
    if changed {
        AuditModel::record_on(
            tx.conn(),
            job.pid,
            kind,
            caller.actor(),
            Some(serde_json::json!({ "kind": job.kind })),
        )
        .await
        .map_err(super::audit_failed)?;
    }
    tx.commit().await?;
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct JobListParams {
    #[serde(default)]
    /// Filter by job kind (`import` or `export`); absent means both.
    kind: Option<String>,
    #[serde(default)]
    /// Filter by job status; absent means any status.
    status: Option<String>,
}

/// `GET /api/plans/bulk-jobs` — the caller's jobs (an admin sees all).
#[debug_handler]
async fn list_jobs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<JobListParams>,
) -> Result<Response> {
    let mut q =
        sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM bulk_jobs WHERE entity = ");
    q.push_bind(ENTITY);
    if let Some(k) = p.kind {
        q.push(" AND kind = ").push_bind(k);
    }
    if let Some(s) = p.status {
        q.push(" AND status = ").push_bind(s);
    }
    q.push(" ORDER BY id DESC LIMIT 200");
    let rows: Vec<bulk_jobs::Model> = q.build_query_as().fetch_all(crate::db::pool(&ctx)).await?;
    let mine: Vec<serde_json::Value> = rows
        .iter()
        .filter(|j| visible(j, &caller).is_ok())
        .map(|j| job_view(j, None))
        .collect();
    format::json(mine)
}

/// `GET /api/plans/bulk-jobs/{id}/errors` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ErrorsParams {
    #[serde(default)]
    /// Error report format, `csv` or `jsonl`; absent means the default (csv).
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct QueueParams {
    #[serde(default)]
    /// Filter by review-item status; absent means the default open items.
    status: Option<String>,
    #[serde(default)]
    /// Restrict to items produced by this job; absent means all jobs.
    job: Option<Uuid>,
}

/// `GET /api/plans/review-queue` — rows held for a decision.
#[debug_handler]
async fn list_queue(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<QueueParams>,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM review_queue_items WHERE status = ",
    );
    q.push_bind(p.status.as_deref().unwrap_or("pending"));
    if let Some(job) = p.job {
        q.push(" AND job_pid = ").push_bind(job);
    }
    q.push(" ORDER BY id LIMIT 500");
    let rows: Vec<review_queue_items::Model> =
        q.build_query_as().fetch_all(crate::db::pool(&ctx)).await?;
    format::json(
        rows.into_iter()
            .map(|row| without_hidden(row, &scope))
            .collect::<Vec<_>>(),
    )
}

/// A held row with the matches the reader may not see taken out: they name plans.
fn without_hidden(
    mut row: review_queue_items::Model,
    scope: &super::plan_access::HiddenScope,
) -> review_queue_items::Model {
    if let Some(matches) = row.matches.as_array_mut() {
        matches.retain(|m| {
            m["pid"]
                .as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .is_none_or(|pid| !scope.plan(&pid))
        });
    }
    let parent = row.candidate["parent_ref"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s.trim()).ok());
    if parent.is_some_and(|p| scope.plan(&p)) {
        row.candidate["parent_ref"] = serde_json::Value::Null;
    }
    row
}

/// Fetches a review-queue item by public id, `NotFound` when unknown.
async fn find_item(ctx: &AppContext, pid: &str) -> Result<review_queue_items::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM review_queue_items WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
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
    super::plans::require_visible_parent(&ctx, &caller, &plan, None).await?;
    let model = streaming::create_and_emit(&ctx, &plan, caller.actor()).await?;
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    let row: review_queue_items::Model = sqlx::query_as(
        "UPDATE review_queue_items
         SET status = 'confirmed', resolved_by = $1, resolved_at = $2, resolution_pid = $3
         WHERE id = $4
         RETURNING *",
    )
    .bind(caller.actor())
    .bind(stamp())
    .bind(model.pid)
    .bind(item.id)
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    format::json(without_hidden(row, &scope))
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
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    let row: review_queue_items::Model = sqlx::query_as(
        "UPDATE review_queue_items SET status = 'rejected', resolved_by = $1, resolved_at = $2 WHERE id = $3
         RETURNING *",
    )
    .bind(caller.actor())
    .bind(stamp())
    .bind(item.id)
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    format::json(without_hidden(row, &scope))
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
    // Unordered, as before.
    let expired: Vec<artifact_rows::Model> = sqlx::query_as(
        "SELECT * FROM artifacts WHERE expires_at < $1 AND storage_key IS NOT NULL LIMIT 1000",
    )
    .bind(now)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let mut deleted = 0usize;
    for a in expired {
        if let Some(key) = a.storage_key.clone() {
            let _ = tokio::task::spawn_blocking(move || Store::from_env().remove(&key)).await;
        }
        // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
        sqlx::query_scalar::<_, i64>(
            "UPDATE artifacts SET storage_key = NULL WHERE id = $1 RETURNING id",
        )
        .bind(a.id)
        .fetch_one(crate::db::pool(ctx))
        .await?;
        deleted += 1;
    }
    // Unordered, as before.
    let stale: Vec<bulk_jobs::Model> =
        sqlx::query_as("SELECT * FROM bulk_jobs WHERE status = 'running' AND updated_at < $1")
            .bind(now - chrono::Duration::seconds(LEASE_SECS))
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let mut requeued = 0usize;
    for j in stale {
        sqlx::query_scalar::<_, i64>(
            "UPDATE bulk_jobs SET status = 'queued', updated_at = $1 WHERE id = $2 RETURNING id",
        )
        .bind(chrono::DateTime::<chrono::FixedOffset>::from(now))
        .bind(j.id)
        .fetch_one(crate::db::pool(ctx))
        .await?;
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
    let claimed = sqlx::query(
        "UPDATE bulk_jobs SET status = 'running', started_at = $1, updated_at = $1
         WHERE pid = $2 AND status = 'queued'",
    )
    .bind(now)
    .bind(job_pid)
    .execute(crate::db::pool(ctx))
    .await?;
    if claimed.rows_affected() != 1 {
        return Ok(()); // someone else holds it, or it is done
    }
    let job = find_job(ctx, &job_pid.to_string(), None).await?;
    let outcome = if job.kind == "import" {
        run_import(ctx, &job).await
    } else {
        run_export(ctx, &job).await
    };
    if let Err(reason) = outcome {
        let failed = find_job(ctx, &job_pid.to_string(), None).await?;
        sqlx::query_scalar::<_, i64>(
            "UPDATE bulk_jobs SET status = 'failed', failure_reason = $1, finished_at = $2, updated_at = $3
             WHERE id = $4
             RETURNING id",
        )
        .bind(reason)
        .bind(stamp())
        .bind(stamp())
        .bind(failed.id)
        .fetch_one(crate::db::pool(ctx))
        .await?;
    }
    Ok(())
}

/// Builds a per-row error record tied to the decoded row's number and source line, with a stable machine `code` and a human message.
fn row_error(d: &Decoded, field: &str, code: &'static str, message: String) -> RowError {
    RowError {
        row_number: d.row_number,
        source_line: d.source_line,
        field: field.to_string(),
        code,
        message,
    }
}

/// Extracts a human-readable message from an error, preferring the custom error detail, then its code.
fn text_of(e: &Error) -> String {
    match e {
        Error::CustomError(_, d) => d
            .detail
            .clone()
            .or_else(|| d.code.clone())
            .unwrap_or_default(),
        other => other.to_string(),
    }
}

/// Finds the live (not soft-deleted) plan matching one import key: by pid, by an identifier scheme/value pair, or by owner organisation and code. The raw SQL binds every imported value as a parameter and orders by id so a duplicate key resolves deterministically.
async fn find_by_key(ctx: &AppContext, key: &Key) -> Result<Option<PlanModel>> {
    // Bound parameters in a plain statement: no SQL is built from the imported values.
    let row = match key {
        Key::Pid(p) => {
            sqlx::query_as("SELECT * FROM plans WHERE deleted_at IS NULL AND pid = $1 LIMIT 1")
                .bind(*p)
                .fetch_optional(crate::db::pool(ctx))
                .await?
        }
        Key::Identifier { scheme, value } => {
            let probe = serde_json::json!([{ "scheme": scheme, "value": value }]).to_string();
            sqlx::query_as(
                "SELECT * FROM plans WHERE deleted_at IS NULL \
                     AND (data->'identifiers') @> $1::jsonb ORDER BY id LIMIT 1",
            )
            .bind(probe)
            .fetch_optional(crate::db::pool(ctx))
            .await?
        }
        Key::OwnerCode { owner, code } => {
            sqlx::query_as(
                "SELECT * FROM plans WHERE deleted_at IS NULL \
                     AND data->>'owner_org_id' = $1 AND data->>'code' = $2 \
                     ORDER BY id LIMIT 1",
            )
            .bind(owner)
            .bind(code)
            .fetch_optional(crate::db::pool(ctx))
            .await?
        }
    };
    Ok(row)
}

/// What happened to one imported row; mirrors the job counters.
enum RowOutcome {
    /// A new plan was created.
    Created,
    /// An existing plan was changed.
    Updated,
    /// The row matched a plan and carried no change.
    Unchanged,
    /// The row was ambiguous or keyless and was held in the review queue instead of being written.
    Queued,
}

/// The field of a row that names no plan the job may touch: a pid that names nothing, or anything in `hidden`.
fn unknown_field(
    plan: &project_portfolio_management_matcher::Plan,
    existing: Option<&PlanModel>,
    keys: &[Key],
    hidden: &std::collections::HashSet<Uuid>,
) -> Option<&'static str> {
    let parent = plan
        .parent_ref
        .as_deref()
        .and_then(|s| Uuid::parse_str(s.trim()).ok());
    if parent.is_some_and(|p| hidden.contains(&p)) {
        return Some("parent_ref");
    }
    let hits_hidden = existing.is_some_and(|m| hidden.contains(&m.pid));
    (hits_hidden || (existing.is_none() && matches!(keys.first(), Some(Key::Pid(_)))))
        .then_some("pid")
}

/// The plans hidden from whoever asked for this import, as recorded with the job.
fn hidden_of(job: &bulk_jobs::Model) -> std::collections::HashSet<Uuid> {
    job.params["hidden_plans"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_str().and_then(|s| Uuid::parse_str(s).ok()))
        .collect()
}

/// What one import job was asked to do, for each of its rows.
struct ImportRun<'a> {
    /// What a keyless row that looks like a duplicate does.
    on_keyless: &'a str,
    /// Who asked for the import.
    actor: Option<&'a str>,
    /// The job.
    job_pid: Uuid,
    /// The plans hidden from whoever asked.
    hidden: std::collections::HashSet<Uuid>,
}

/// Validates, matches and applies one imported row through the normal create/update path, so events and audit match a single request. Returns the outcome or the row errors; a pid naming no live plan is an error, never a silent create. A row that reaches a plan in `hidden` gets the error of one that matched nothing.
async fn process_row(
    ctx: &AppContext,
    d: &Decoded,
    pid: Option<&str>,
    plan: &project_portfolio_management_matcher::Plan,
    run: &ImportRun<'_>,
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
    if let Some(field) = unknown_field(plan, existing.as_ref(), &keys, &run.hidden) {
        return Err(vec![row_error(
            d,
            field,
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
        return streaming::update_and_emit(ctx, model, plan, run.actor)
            .await
            .map(|_| RowOutcome::Updated)
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())]);
    }
    if let Err(e) = super::plans::check_no_cycle(ctx, None, plan).await {
        return Err(vec![row_error(d, "parent_ref", "cycle", text_of(&e))]);
    }
    // Keyless ⇒ the normal duplicate detection decides.
    if keys.is_empty() && run.on_keyless == "review" {
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
        let mut rows = PlanModel::find_by_pids(ctx, &pids)
            .await
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])?;
        // A title must not find a hidden plan here either: held or created, the answer is the one no match gives.
        rows.retain(|r| !run.hidden.contains(&r.pid));
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
            // The columns `SeaORM`'s insert set, no more.
            sqlx::query(
                "INSERT INTO review_queue_items
                     (pid, status, provenance, candidate, matches, job_pid, source_line, resolved_by, resolved_at,
                      resolution_pid, resolution_comment)
                 VALUES ($1, 'pending', 'import', $2, $3, $4, $5, NULL, NULL, NULL, NULL)",
            )
            .bind(Uuid::new_v4())
            .bind(serde_json::to_value(plan).unwrap_or_default())
            .bind(serde_json::json!(hits))
            .bind(run.job_pid)
            .bind(i32::try_from(d.source_line).unwrap_or(i32::MAX))
            .execute(crate::db::pool(ctx))
            .await
            .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])?;
            return Ok(RowOutcome::Queued);
        }
    }
    streaming::create_and_emit(ctx, plan, run.actor)
        .await
        .map(|_| RowOutcome::Created)
        .map_err(|e| vec![row_error(d, "", "internal", e.to_string())])
}

/// Records the terminal state of a job: status derived from the counters, progress 100, counters, omitted count, result/error artifacts and finish time.
async fn finish(
    ctx: &AppContext,
    job: &bulk_jobs::Model,
    c: Counters,
    omitted: i64,
    result: Option<Uuid>,
    errors: Option<Uuid>,
) -> std::result::Result<(), String> {
    // By id, as `SeaORM`'s update of the job was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE bulk_jobs
         SET status = $1, progress = 100, rows_total = $2, rows_created = $3, rows_updated = $4,
             rows_unchanged = $5, rows_queued_for_review = $6, rows_failed = $7, rows_omitted = $8,
             result_artifact = $9, error_artifact = $10, finished_at = $11, updated_at = $12
         WHERE id = $13
         RETURNING id",
    )
    .bind(c.status())
    .bind(c.total)
    .bind(c.created)
    .bind(c.updated)
    .bind(c.unchanged)
    .bind(c.queued_for_review)
    .bind(c.failed)
    .bind(omitted)
    .bind(result)
    .bind(errors)
    .bind(stamp())
    .bind(stamp())
    .bind(job.id)
    .fetch_one(crate::db::pool(ctx))
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Record how far a running job is. A trigger in the database makes the stored
/// value monotone even if two updates race or a caller passes a lower number.
async fn set_progress(ctx: &AppContext, job: Uuid, percent: i32) {
    let _ = sqlx::query(
        "UPDATE bulk_jobs SET progress = GREATEST(progress, $1) WHERE pid = $2 AND status = 'running'",
    )
    .bind(percent)
    .bind(job)
    .execute(crate::db::pool(ctx))
    .await;
}

/// Whether someone asked for this job to stop.
async fn cancel_requested(ctx: &AppContext, job: Uuid) -> bool {
    // A failed read is "no", as before.
    sqlx::query_scalar::<_, bool>(
        "SELECT cancel_requested_at IS NOT NULL FROM bulk_jobs WHERE pid = $1 LIMIT 1",
    )
    .bind(job)
    .fetch_optional(crate::db::pool(ctx))
    .await
    .ok()
    .flatten()
    .unwrap_or(false)
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
    // By id, as `SeaORM`'s update of the job was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE bulk_jobs
         SET status = $1, rows_total = $2, rows_created = $3, rows_updated = $4, rows_unchanged = $5,
             rows_queued_for_review = $6, rows_failed = $7, rows_omitted = $8, error_artifact = $9,
             finished_at = $10, updated_at = $11
         WHERE id = $12
         RETURNING id",
    )
    .bind(bulk::Counters::cancelled_status())
    .bind(c.total)
    .bind(c.created)
    .bind(c.updated)
    .bind(c.unchanged)
    .bind(c.queued_for_review)
    .bind(c.failed)
    .bind(not_reached)
    .bind(errors)
    .bind(stamp())
    .bind(stamp())
    .bind(job.id)
    .fetch_one(crate::db::pool(ctx))
    .await
    .map_err(|e| e.to_string())?;
    AuditModel::record_or_log_on(
        crate::db::pool(ctx),
        job.pid,
        "bulk_job_cancelled",
        job.cancelled_by.as_deref().or(job.requested_by.as_deref()),
        Some(serde_json::json!({ "counts": c, "not_reached": not_reached })),
    )
    .await;
    Ok(())
}

/// Worker body for an import: reads and decodes the source, enforces the row cap, processes each row while polling for cancellation, then stores the error report and finishes the job. Errors returned here fail the whole job.
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
    let run = ImportRun {
        on_keyless: &on_keyless,
        actor: job.requested_by.as_deref(),
        job_pid: job.pid,
        hidden: hidden_of(job),
    };
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
            Ok((pid, plan)) => match process_row(ctx, d, pid.as_deref(), plan, &run).await {
                Ok(RowOutcome::Created) => counters.created += 1,
                Ok(RowOutcome::Updated) => counters.updated += 1,
                Ok(RowOutcome::Unchanged) => counters.unchanged += 1,
                Ok(RowOutcome::Queued) => counters.queued_for_review += 1,
                Err(errs) => {
                    counters.failed += 1;
                    errors.extend(errs);
                }
            },
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
                run.actor,
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
    AuditModel::record_or_log_on(
        crate::db::pool(ctx),
        job.pid,
        "bulk_import_finished",
        run.actor,
        Some(serde_json::json!({ "counts": counters })),
    )
    .await;
    Ok(())
}

#[expect(clippy::too_many_lines)]
/// Worker body for an export: loads the plans authorized when the job was requested, encodes them in the job's format and masking profile, stores the result artifact and finishes the job.
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
        let rows: Vec<plans::Model> =
            sqlx::query_as("SELECT * FROM plans WHERE pid = ANY($1) ORDER BY id")
                .bind(chunk)
                .fetch_all(crate::db::pool(ctx))
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
    AuditModel::record_on(
        crate::db::pool(ctx),
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
        .add("/plans/import", body_limit(post(import), ceiling))
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

//! File attachments (LT-5) over the artifact store ([`crate::artifacts`]).
//!
//! An upload is the **raw request body** (`POST /api/attachments?subject_kind=
//! &subject_pid=&filename=` with its `Content-Type`), refused **before
//! anything is stored** when the type is not on the allow-list or the size is
//! over the cap. The stored bytes carry a SHA-256 that is re-checked on every
//! download. A download is re-authorized against the **subject's** record-level
//! policy (and private-plan membership), not just the attachment's own row,
//! and is served with `nosniff` and `Content-Disposition: attachment` so a
//! file can never be rendered as a page.

use super::forbidden;
use super::unprocessable;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axa::middleware::body_limit;
use axum::body::Bytes;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::artifacts::{self, Store};
use crate::auth::MaybeAuthUser;
use crate::content_check;
use crate::discussion as subject_rules;
use crate::models::_entities::{artifacts as artifact_rows, attachments};
use crate::models::audit_logs::Model as AuditModel;

/// Whether the caller holds the admin role; admins may act on attachments beyond their own.
fn is_admin(caller: &MaybeAuthUser) -> bool {
    super::plan_access::is_admin(caller.claims())
}

/// `POST /api/attachments` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct UploadParams {
    /// Kind of record the file is attached to; must be one of the allowed subject kinds.
    subject_kind: String,
    /// Pid of that record, as text; parsed and access-checked when the request is handled.
    subject_pid: String,
    /// Client-supplied name; sanitised before it is stored.
    filename: String,
}

/// `POST /api/attachments` — attach the request body to a subject.
#[debug_handler]
async fn upload(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Query(p): Query<UploadParams>,
    body: Bytes,
) -> Result<Response> {
    if !subject_rules::is_token(subject_rules::SUBJECT_KINDS, &p.subject_kind) {
        return Err(unprocessable(&format!(
            "unknown subject_kind (allowed: {:?})",
            subject_rules::SUBJECT_KINDS
        )));
    }
    let declared = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    // Everything that can be refused is refused before a byte is stored.
    let problems = artifacts::upload_problems(declared, body.len(), artifacts::max_bytes());
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    // The content must be what the declared type says, and no program is accepted (RG-8).
    let content = content_check::content_problems(&artifacts::media_type(declared), &body);
    if !content.is_empty() {
        return Err(unprocessable(&content.join("; ")));
    }
    let subject = resolve_subject(&ctx, &caller, &p.subject_kind, &p.subject_pid).await?;
    // An optional virus scanner has the last word, before anything is stored. A configured scanner
    // that cannot answer refuses the upload (fail closed).
    match content_check::scan(&body).await {
        content_check::Verdict::NotConfigured | content_check::Verdict::Clean => {}
        content_check::Verdict::Infected(name) => {
            AuditModel::record_or_log_on(
                crate::db::pool(&ctx),
                subject.pid,
                "attachment_refused_by_scan",
                caller.actor(),
                Some(serde_json::json!({ "subject_kind": p.subject_kind, "signature": name })),
            )
            .await;
            return Err(unprocessable("the file was refused by the malware scan"));
        }
        content_check::Verdict::Unavailable(why) => {
            tracing::error!(error = %why, "the attachment scanner did not answer; upload refused");
            return Err(Error::CustomError(
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorDetail::new(
                    "scan_unavailable",
                    "the malware scanner is not available; try again later",
                ),
            ));
        }
    }
    let filename = artifacts::sanitize_filename(&p.filename);
    let sha = artifacts::sha256_hex(&body);
    let size = i64::try_from(body.len()).unwrap_or(i64::MAX);
    let bytes = body.to_vec();
    let key = tokio::task::spawn_blocking(move || Store::from_env().put(&bytes))
        .await
        .map_err(|e| Error::string(&e.to_string()))?
        .map_err(|e| Error::string(&format!("could not store the file: {e}")))?;
    // The artifact, the attachment and the audit row are one change (the stored bytes are not: a refused
    // change leaves an unreferenced blob, which nothing reads).
    let mut tx = crate::db::begin(&ctx).await?;
    let (artifact_pid, content_type): (Uuid, String) = sqlx::query_as(
        "INSERT INTO artifacts
             (pid, kind, filename, content_type, size_bytes, sha256, storage_key, created_by, expires_at)
         VALUES ($1, 'attachment', $2, $3, $4, $5, $6, $7, NULL)
         RETURNING pid, content_type",
    )
    .bind(Uuid::new_v4())
    .bind(&filename)
    .bind(artifacts::media_type(declared))
    .bind(size)
    .bind(&sha)
    .bind(key)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO attachments (pid, subject_kind, subject_pid, artifact_pid, uploaded_by, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(&p.subject_kind)
    .bind(subject.pid)
    .bind(artifact_pid)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "attachment_added",
        caller.actor(),
        Some(
            serde_json::json!({ "subject_kind": p.subject_kind, "subject_pid": subject.pid,
                                  "filename": filename, "sha256": sha }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    index_text(&ctx, row_pid, declared, &body).await;
    format::json(serde_json::json!({
        "pid": row_pid, "filename": filename, "content_type": content_type,
        "size_bytes": size, "sha256": sha,
    }))
}

/// Extract and store an attachment's text (PL-8). Best-effort: whatever goes
/// wrong is recorded as `unindexed` with a reason and **never fails the
/// upload**. Reads untrusted bytes, so it runs off the async threads and is
/// bounded (see [`crate::extract`]).
async fn index_text(ctx: &AppContext, attachment_pid: Uuid, declared: &str, body: &Bytes) {
    let media = artifacts::media_type(declared);
    let bytes = body.to_vec();
    let result = tokio::task::spawn_blocking(move || crate::extract::extract(&media, &bytes))
        .await
        .unwrap_or(crate::extract::Extraction::Unindexed("extractor_failed"));
    let (status, reason, text) = match result {
        crate::extract::Extraction::Text(t) => ("indexed", None, t),
        crate::extract::Extraction::Unindexed(r) => {
            ("unindexed", Some(r.to_string()), String::new())
        }
    };
    // Postgres text cannot hold NUL.
    let text = text.replace('\0', "");
    let _ = sqlx::query(
        "INSERT INTO attachment_texts (pid, attachment_pid, status, reason, body) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(attachment_pid)
    .bind(status)
    .bind(reason)
    .bind(text)
    .execute(crate::db::pool(ctx))
    .await;
}

/// `GET /api/attachments/search` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SearchParams {
    /// Search text, 2 to 200 characters after trimming.
    q: String,
}

/// One search hit, before the subject's access check.
#[derive(sqlx::FromRow)]
struct Hit {
    /// The attachment.
    attachment_pid: Uuid,
    /// Its subject's kind.
    subject_kind: String,
    /// Its subject.
    subject_pid: Uuid,
    /// The file's name.
    filename: String,
    /// The matching words in context.
    snippet: String,
}

/// `GET /api/attachments/search?q=` — find attachments by their text (PL-8).
///
/// **Access follows the subject:** every candidate hit is passed through the
/// same record-level check as reading the attachment itself, and one the caller
/// may not read is dropped — not shown as a count, not as a title. Soft-deleted
/// attachments never match. The response also says how many attachments have no
/// extractable text (so "no result" is not mistaken for "not there").
#[debug_handler]
async fn search(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<SearchParams>,
) -> Result<Response> {
    let q = p.q.trim();
    if q.chars().count() < 2 || q.chars().count() > 200 {
        return Err(unprocessable("q must be 2 to 200 characters"));
    }
    let rows: Vec<Hit> = sqlx::query_as(
        "SELECT a.pid AS attachment_pid, a.subject_kind, a.subject_pid, f.filename, \
                    ts_headline('simple', t.body, plainto_tsquery('simple', $1), \
                                'MaxFragments=1,MaxWords=18,MinWords=5') AS snippet \
             FROM attachment_texts t \
             JOIN attachments a ON a.pid = t.attachment_pid AND a.deleted_at IS NULL \
             JOIN artifacts f ON f.pid = a.artifact_pid \
             WHERE t.status = 'indexed' \
               AND to_tsvector('simple', t.body) @@ plainto_tsquery('simple', $1) \
             ORDER BY ts_rank(to_tsvector('simple', t.body), plainto_tsquery('simple', $1)) DESC, a.id \
             LIMIT 200",
    )
    .bind(q)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut out = Vec::new();
    for r in rows {
        if resolve_subject(&ctx, &caller, &r.subject_kind, &r.subject_pid.to_string())
            .await
            .is_err()
        {
            continue;
        }
        out.push(serde_json::json!({
            "attachment_pid": r.attachment_pid,
            "subject_kind": r.subject_kind, "subject_pid": r.subject_pid,
            "filename": r.filename,
            "snippet": r.snippet,
        }));
        if out.len() >= 50 {
            break;
        }
    }
    let unindexed: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM attachment_texts WHERE status = 'unindexed'")
            .fetch_one(crate::db::pool(&ctx))
            .await?;
    format::json(serde_json::json!({ "results": out, "unindexed_attachments": unindexed }))
}

/// `GET /api/attachments` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SubjectParams {
    /// Kind of record whose files are listed.
    subject_kind: String,
    /// Pid of that record, as text.
    subject_pid: String,
}

/// `GET /api/attachments?subject_kind=&subject_pid=` — a subject's files.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<SubjectParams>,
) -> Result<Response> {
    let subject = resolve_subject(&ctx, &caller, &p.subject_kind, &p.subject_pid).await?;
    let rows: Vec<attachments::Model> = sqlx::query_as(
        "SELECT * FROM attachments
         WHERE subject_kind = $1 AND subject_pid = $2 AND deleted_at IS NULL
         ORDER BY id",
    )
    .bind(&p.subject_kind)
    .bind(subject.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut out = Vec::new();
    for r in rows {
        let Some(a) = find_artifact(&ctx, r.artifact_pid).await? else {
            continue;
        };
        out.push(serde_json::json!({
            "pid": r.pid, "filename": a.filename, "content_type": a.content_type,
            "size_bytes": a.size_bytes, "sha256": a.sha256,
            "uploaded_by": r.uploaded_by, "created_at": r.created_at,
        }));
    }
    format::json(out)
}

/// The artifact row behind an attachment, deleted or not (as before: an attachment's file is not hidden by its
/// artifact's `deleted_at`).
async fn find_artifact(ctx: &AppContext, pid: Uuid) -> Result<Option<artifact_rows::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM artifacts WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// Load a live (not soft-deleted) attachment by pid, or `NotFound`. This only locates the row; callers must still authorise against its subject.
async fn find_attachment(ctx: &AppContext, pid: &str) -> Result<attachments::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM attachments WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// `GET /api/attachments/{pid}/download` — the bytes, re-authorized against
/// the subject and re-verified against the stored SHA-256.
#[debug_handler]
async fn download(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<axum::response::Response> {
    let att = find_attachment(&ctx, &pid).await?;
    // Authorized on the SUBJECT, so losing access to the plan loses the file.
    resolve_subject(
        &ctx,
        &caller,
        &att.subject_kind,
        &att.subject_pid.to_string(),
    )
    .await?;
    let artifact = find_artifact(&ctx, att.artifact_pid)
        .await?
        .ok_or(Error::NotFound)?;
    let key = artifact.storage_key.clone().ok_or(Error::NotFound)?;
    let bytes = tokio::task::spawn_blocking(move || Store::from_env().get(&key))
        .await
        .map_err(|e| Error::string(&e.to_string()))?
        .map_err(|_| Error::NotFound)?;
    if artifacts::sha256_hex(&bytes) != artifact.sha256 {
        // The file on disk is not the file that was uploaded. Never serve it.
        tracing::error!(attachment = %att.pid, "stored attachment failed its SHA-256 check");
        return Err(Error::string("the stored file failed its integrity check"));
    }
    Ok((
        [
            (header::CONTENT_TYPE, artifact.content_type.clone()),
            (
                header::CONTENT_DISPOSITION,
                artifacts::content_disposition(&artifact.filename),
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

/// `POST /api/attachments/{pid}/delete` (and `DELETE`) — soft-delete. The
/// uploader or an admin may; the bytes stay until a sweep.
#[debug_handler]
async fn delete_one(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let att = find_attachment(&ctx, &pid).await?;
    resolve_subject(
        &ctx,
        &caller,
        &att.subject_kind,
        &att.subject_pid.to_string(),
    )
    .await?;
    if att.uploaded_by.as_deref() != caller.actor() && !is_admin(&caller) {
        return Err(forbidden(
            "only the uploader or an admin may remove an attachment",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error (`500`), as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE attachments SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(
        chrono::Utc::now(),
    ))
    .bind(att.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        att.pid,
        "attachment_removed",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// The attachment routes.
pub fn routes() -> Routes {
    // The service-wide body limit is 2 MiB; raise it to the configured cap (plus
    // a little) for the upload route only, with axa's `body_limit`, which binds the
    // body itself (Axum's `DefaultBodyLimit` raises only the extractors' limit, and
    // axa 0.11's stack would still refuse at 2 MiB). The handler still refuses anything
    // over the cap, so this layer is a transport ceiling, not the rule.
    let ceiling = artifacts::max_bytes().saturating_add(64 * 1024);
    Routes::new()
        .prefix("/api")
        .add("/attachments", body_limit(post(upload), ceiling))
        .add("/attachments", get(list))
        .add("/attachments/search", get(search))
        .add("/attachments/{pid}/download", get(download))
        .add("/attachments/{pid}", delete(delete_one))
        .add("/attachments/{pid}/delete", post(delete_one))
}

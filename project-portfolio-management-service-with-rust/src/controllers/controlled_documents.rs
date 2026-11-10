//! Controlled documents (PL-6). Rules in [`crate::documents`]; this is the wiring.
//!
//! A document's versions move `draft → in_review → approved → effective → superseded`.
//! Review is a PL-1 approval request on the version (`document_version` subject), so only
//! the named approver approves; a version becomes `effective` only if that approval
//! really was approved, and in the same step the version it replaces is `superseded`
//! naming its successor. Once a version has left draft its text never changes (an
//! edit is the next draft) — in the database itself as well as here. Every transition
//! is audited.

use super::unprocessable;
use super::{conflict, forbidden};
use crate::db::Tx;
use crate::prelude::*;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::documents::{self as rules, Span, Status};
use crate::models::_entities::{
    approval_requests, controlled_documents as docs, document_versions as versions,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;

/// True when the caller carries the `access: admin` attribute, which lets them act on documents they did not create.
fn is_admin(claims: &authentication_verifier::Claims) -> bool {
    crate::auth::has_attr(claims, "access", "admin")
}

/// Documents are read by signed-in people only, whether or not they belong to a plan.
fn require_signed_in(caller: &MaybeAuthUser) -> Result<()> {
    if caller.0.is_some() {
        Ok(())
    } else {
        Err(Error::Unauthorized("sign in to read documents".to_string()))
    }
}

/// A document the caller may read: live, and not in a private plan they are outside.
async fn find_doc(ctx: &AppContext, caller: &MaybeAuthUser, pid: &str) -> Result<docs::Model> {
    require_signed_in(caller)?;
    let pid = super::parse_pid(pid)?;
    let doc: docs::Model = sqlx::query_as(
        "SELECT * FROM controlled_documents WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)?;
    if let Some(plan) = doc.plan_pid {
        if super::guests::caller_is_guest(caller)
            && !super::guests::guest_covers_plan(ctx, caller, plan, "read").await
        {
            return Err(Error::NotFound);
        }
        if super::plan_access::hidden_plans(ctx, &[plan], caller)
            .await
            .contains(&plan)
        {
            return Err(Error::NotFound);
        }
    }
    Ok(doc)
}

/// The statement for all versions of a document, oldest first (ascending version number).
const VERSIONS_OF: &str =
    "SELECT * FROM document_versions WHERE document_pid = $1 ORDER BY version";

/// All versions of a document, oldest first (ascending version number).
async fn versions_of(ctx: &AppContext, doc: Uuid) -> Result<Vec<versions::Model>> {
    Ok(sqlx::query_as(VERSIONS_OF)
        .bind(doc)
        .fetch_all(crate::db::pool(ctx))
        .await?)
}

/// [`versions_of`] inside a transaction.
async fn versions_in(tx: &mut Tx, doc: Uuid) -> Result<Vec<versions::Model>> {
    Ok(sqlx::query_as(VERSIONS_OF)
        .bind(doc)
        .fetch_all(tx.conn())
        .await?)
}

/// JSON shape of one version. The text is included only when `with_body` is set, so list and summary views stay small.
fn version_view(v: &versions::Model, with_body: bool) -> serde_json::Value {
    let mut out = serde_json::json!({
        "pid": v.pid, "version": v.version, "status": v.status, "change_summary": v.change_summary,
        "approval_pid": v.approval_pid, "effective_from": v.effective_from,
        "superseded_on": v.superseded_on, "superseded_by": v.superseded_by, "created_by": v.created_by,
    });
    if with_body {
        out["body"] = serde_json::json!(v.body);
    }
    out
}

/// JSON shape of a document with its version summaries (no bodies) and `in_force_today`, the version in force on the deployment's current date, or null when none is.
async fn doc_view(ctx: &AppContext, doc: &docs::Model) -> Result<serde_json::Value> {
    let vs = versions_of(ctx, doc.pid).await?;
    let spans: Vec<Span> = vs
        .iter()
        .filter_map(|v| {
            v.effective_from.map(|f| Span {
                version: v.version,
                effective_from: f,
                superseded_on: v.superseded_on,
            })
        })
        .collect();
    Ok(serde_json::json!({
        "pid": doc.pid, "title": doc.title, "plan_pid": doc.plan_pid, "created_by": doc.created_by,
        "in_force_today": rules::in_force(&spans, crate::tz::deployment_today()),
        "versions": vs.iter().map(|v| version_view(v, false)).collect::<Vec<_>>(),
    }))
}

/// Request body for creating a document.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CreateBody {
    /// Document title; validated by the documents rules and stored trimmed.
    title: String,
    /// Plan the document belongs to, if any. Absent means a standalone document not tied to a plan.
    #[serde(default)]
    plan_pid: Option<Uuid>,
    /// Text of the first draft; absent is stored as an empty draft.
    #[serde(default)]
    body: Option<String>,
    /// Optional note describing the version, at most 500 characters.
    #[serde(default)]
    change_summary: Option<String>,
}

/// Insert one version row in `draft` status with no approval, effective date or supersession. `by` is the subject recorded as its creator.
async fn insert_version(
    tx: &mut Tx,
    doc: Uuid,
    version: i32,
    body: &str,
    summary: Option<&str>,
    by: &str,
) -> Result<versions::Model> {
    Ok(sqlx::query_as(
        "INSERT INTO document_versions
             (pid, document_pid, version, body, change_summary, status, approval_pid, effective_from,
              superseded_on, superseded_by, created_by)
         VALUES ($1, $2, $3, $4, $5, $6, NULL, NULL, NULL, NULL, $7)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(doc)
    .bind(version)
    .bind(body)
    .bind(summary)
    .bind(Status::Draft.token())
    .bind(by)
    .fetch_one(tx.conn())
    .await?)
}

/// `POST /api/documents` — create a document and its first draft.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<CreateBody>,
) -> Result<Response> {
    let mut problems = rules::problems(Some(&b.title), b.body.as_deref());
    if b.change_summary
        .as_deref()
        .is_some_and(|s| s.chars().count() > 500)
    {
        problems.push("change_summary is at most 500 characters".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let caller = MaybeAuthUser(Some(claims.clone()));
    if let Some(plan) = b.plan_pid {
        resolve_subject(&ctx, &caller, "plan", &plan.to_string()).await?;
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let doc: docs::Model = sqlx::query_as(
        "INSERT INTO controlled_documents (pid, title, plan_pid, created_by, deleted_at)
         VALUES ($1, $2, $3, $4, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.title.trim())
    .bind(b.plan_pid)
    .bind(&claims.sub)
    .fetch_one(tx.conn())
    .await?;
    let v = insert_version(
        &mut tx,
        doc.pid,
        1,
        b.body.as_deref().unwrap_or(""),
        b.change_summary.as_deref(),
        &claims.sub,
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        doc.pid,
        "document_created",
        Some(&claims.sub),
        Some(serde_json::json!({ "title": doc.title, "plan_pid": doc.plan_pid })),
    )
    .await
    .map_err(Error::Model)?;
    AuditModel::record_on(
        tx.conn(),
        v.pid,
        "document_version_created",
        Some(&claims.sub),
        Some(serde_json::json!({ "document_pid": doc.pid, "version": 1 })),
    )
    .await
    .map_err(Error::Model)?;
    tx.commit().await?;
    format::json(doc_view(&ctx, &doc).await?)
}

/// `GET /api/documents` — the documents the caller may read, with the version in force today.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    cursor: super::pagination::Cursor,
    caller: MaybeAuthUser,
) -> Result<Response> {
    require_signed_in(&caller)?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let rows = scope.keep(
        sqlx::query_as::<_, docs::Model>(
            "SELECT * FROM controlled_documents WHERE deleted_at IS NULL ORDER BY id",
        )
        .fetch_all(crate::db::pool(&ctx))
        .await?,
    );
    let guest = super::guests::caller_is_guest(&caller);
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|d| !guest || d.plan_pid.is_some())
        .collect();
    // Only the requested page is turned into views (each view reads its versions).
    let page = cursor.page(super::pagination::MAX_LIMIT)?;
    let (limit, offset) = (page.fetch(), page.offset());
    let mut out = Vec::new();
    for d in rows
        .iter()
        .skip(usize::try_from(offset).unwrap_or(usize::MAX))
        .take(usize::try_from(limit).unwrap_or(usize::MAX))
    {
        out.push(doc_view(&ctx, d).await?);
    }
    Ok(page.respond(out))
}

/// `GET /api/documents/{pid}` — a document and its versions (without their text).
#[debug_handler]
async fn get_doc(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let doc = find_doc(&ctx, &caller, &pid).await?;
    format::json(doc_view(&ctx, &doc).await?)
}

/// `GET /api/documents/{pid}/versions/{version}` — one version with its text.
#[debug_handler]
async fn get_version(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, version)): Path<(String, i32)>,
) -> Result<Response> {
    let doc = find_doc(&ctx, &caller, &pid).await?;
    let v = versions_of(&ctx, doc.pid)
        .await?
        .into_iter()
        .find(|v| v.version == version)
        .ok_or(Error::NotFound)?;
    format::json(version_view(&v, true))
}

/// Query parameters for the in-force lookup.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct InForceParams {
    /// Date to evaluate; absent means the deployment's current date.
    #[serde(default)]
    on: Option<NaiveDate>,
}

/// `GET /api/documents/{pid}/in-force?on=YYYY-MM-DD` — the version in force on a date
/// (default today), with its text; `in_force: null` when none was.
#[debug_handler]
async fn in_force(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<InForceParams>,
) -> Result<Response> {
    let doc = find_doc(&ctx, &caller, &pid).await?;
    let date = q.on.unwrap_or_else(crate::tz::deployment_today);
    let vs = versions_of(&ctx, doc.pid).await?;
    let spans: Vec<Span> = vs
        .iter()
        .filter_map(|v| {
            v.effective_from.map(|f| Span {
                version: v.version,
                effective_from: f,
                superseded_on: v.superseded_on,
            })
        })
        .collect();
    let hit = rules::in_force(&spans, date).and_then(|n| vs.iter().find(|v| v.version == n));
    format::json(
        serde_json::json!({ "document_pid": doc.pid, "on": date, "in_force": hit.map(|v| version_view(v, true)) }),
    )
}

/// Request body for editing a draft version.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DraftBody {
    /// New text for the draft; absent leaves the existing text unchanged (or, for a new version, copies the latest text).
    #[serde(default)]
    body: Option<String>,
    /// New change summary; absent leaves the existing summary unchanged.
    #[serde(default)]
    change_summary: Option<String>,
}

/// Fetch one numbered version of a document, or `NotFound` when that number does not exist.
async fn find_version(ctx: &AppContext, doc: Uuid, n: i32) -> Result<versions::Model> {
    versions_of(ctx, doc)
        .await?
        .into_iter()
        .find(|v| v.version == n)
        .ok_or(Error::NotFound)
}

/// Allow only the person who created the document, or an admin; everyone else gets Forbidden. Used for write actions on a document.
fn owner_only(doc: &docs::Model, claims: &authentication_verifier::Claims) -> Result<()> {
    if doc.created_by.as_deref() == Some(claims.sub.as_str()) || is_admin(claims) {
        Ok(())
    } else {
        Err(forbidden(
            "only the person who created this document, or an admin, may do this",
        ))
    }
}

/// `POST /api/documents/{pid}/versions` — start the next draft, copying the latest text
/// unless a body is given. Refused while another version is still in flight.
#[debug_handler]
async fn new_version(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(b): Json<DraftBody>,
) -> Result<Response> {
    let caller = MaybeAuthUser(Some(claims.clone()));
    let doc = find_doc(&ctx, &caller, &pid).await?;
    owner_only(&doc, &claims)?;
    let problems = rules::problems(None, b.body.as_deref());
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    // Serialize per document so two people cannot both open "the next draft".
    super::advisory_lock_on(&mut tx, "document", doc.pid).await?;
    let vs = versions_in(&mut tx, doc.pid).await?;
    if vs.iter().any(|v| {
        matches!(
            Status::parse(&v.status),
            Some(Status::Draft | Status::InReview | Status::Approved)
        )
    }) {
        return Err(conflict(
            "a version is still in flight: finish or withdraw it before starting the next draft",
        ));
    }
    let latest = vs.last().ok_or(Error::NotFound)?;
    let next = latest.version + 1;
    let body = b.body.unwrap_or_else(|| latest.body.clone());
    let v = insert_version(
        &mut tx,
        doc.pid,
        next,
        &body,
        b.change_summary.as_deref(),
        &claims.sub,
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        v.pid,
        "document_version_created",
        Some(&claims.sub),
        Some(serde_json::json!({ "document_pid": doc.pid, "version": next })),
    )
    .await
    .map_err(Error::Model)?;
    tx.commit().await?;
    format::json(version_view(&v, true))
}

/// `PUT /api/documents/{pid}/versions/{version}` — edit a **draft**. A version past draft
/// is immutable (409): its next draft is where the change goes.
#[debug_handler]
async fn edit_version(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path((pid, n)): Path<(String, i32)>,
    Json(b): Json<DraftBody>,
) -> Result<Response> {
    let caller = MaybeAuthUser(Some(claims.clone()));
    let doc = find_doc(&ctx, &caller, &pid).await?;
    owner_only(&doc, &claims)?;
    let v = find_version(&ctx, doc.pid, n).await?;
    if !Status::parse(&v.status).is_some_and(Status::is_editable) {
        return Err(conflict(&format!(
            "version {n} is {}: its text is final, so make the next draft",
            v.status
        )));
    }
    let problems = rules::problems(None, b.body.as_deref());
    if !problems.is_empty()
        || b.change_summary
            .as_deref()
            .is_some_and(|s| s.chars().count() > 500)
    {
        return Err(unprocessable(&format!(
            "{}change_summary is at most 500 characters",
            problems.join("; ")
        )));
    }
    let was = v.body.chars().count();
    let mut tx = crate::db::begin(&ctx).await?;
    // Only the columns given are named; with none, the row is read back unchanged.
    let row: versions::Model = if b.body.is_none() && b.change_summary.is_none() {
        sqlx::query_as("SELECT * FROM document_versions WHERE id = $1 LIMIT 1")
            .bind(v.id)
            .fetch_one(tx.conn())
            .await?
    } else {
        let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE document_versions SET ");
        let mut set = q.separated(", ");
        if let Some(body) = &b.body {
            set.push("body = ").push_bind_unseparated(body.clone());
        }
        if b.change_summary.is_some() {
            set.push("change_summary = ")
                .push_bind_unseparated(b.change_summary.clone());
        }
        q.push(" WHERE id = ").push_bind(v.id).push(" RETURNING *");
        q.build_query_as().fetch_one(tx.conn()).await?
    };
    AuditModel::record_on(tx.conn(), row.pid, "document_version_edited", Some(&claims.sub), Some(serde_json::json!({ "document_pid": doc.pid, "version": n, "chars_before": was, "chars_after": row.body.chars().count() })))
        .await
        .map_err(Error::Model)?;
    tx.commit().await?;
    format::json(version_view(&row, true))
}

/// Request body for submitting a draft for review.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SubmitBody {
    /// `person:` or `worker:` URN of the approver; never the submitter themselves.
    approver_ref: String,
    /// Optional message passed to the approver with the approval request.
    #[serde(default)]
    message: Option<String>,
}

/// `POST /api/documents/{pid}/versions/{version}/submit` — send a draft for review: an
/// approval request goes to the named approver, and the version is `in_review`.
#[debug_handler]
async fn submit(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path((pid, n)): Path<(String, i32)>,
    Json(b): Json<SubmitBody>,
) -> Result<Response> {
    let caller = MaybeAuthUser(Some(claims.clone()));
    let doc = find_doc(&ctx, &caller, &pid).await?;
    owner_only(&doc, &claims)?;
    if !super::governance::valid_ref(&b.approver_ref, &["person", "worker"]) {
        return Err(unprocessable("approver_ref must be a person:/worker: URN"));
    }
    if b.approver_ref.ends_with(&claims.sub) {
        return Err(unprocessable(
            "you cannot approve your own document: name someone else",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    // Serialize per document, then re-read: two submits cannot both pass the draft check.
    super::advisory_lock_on(&mut tx, "document", doc.pid).await?;
    let v = versions_in(&mut tx, doc.pid)
        .await?
        .into_iter()
        .find(|v| v.version == n)
        .ok_or(Error::NotFound)?;
    rules::check_move(
        Status::parse(&v.status).unwrap_or(Status::Superseded),
        Status::InReview,
    )
    .map_err(|m| conflict(&m))?;
    if v.body.trim().is_empty() {
        return Err(unprocessable("a version with no text cannot be reviewed"));
    }
    let approval: approval_requests::Model = sqlx::query_as(
        "INSERT INTO approval_requests
             (pid, subject_kind, subject_pid, requested_by, approver_ref, message, status,
              decision_comment, decided_by, decided_at)
         VALUES ($1, 'document_version', $2, $3, $4, $5, 'pending', NULL, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(v.pid)
    .bind(&claims.sub)
    .bind(&b.approver_ref)
    .bind(
        b.message
            .clone()
            .unwrap_or_else(|| format!("Approve {} version {n}", doc.title)),
    )
    .fetch_one(tx.conn())
    .await?;
    let row: versions::Model = sqlx::query_as(
        "UPDATE document_versions SET status = $1, approval_pid = $2 WHERE id = $3 RETURNING *",
    )
    .bind(Status::InReview.token())
    .bind(approval.pid)
    .bind(v.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "document_version_submitted", Some(&claims.sub), Some(serde_json::json!({ "document_pid": doc.pid, "version": n, "approval_pid": approval.pid })))
        .await
        .map_err(Error::Model)?;
    cap_models::notify_on(
        &mut tx,
        &b.approver_ref,
        "document_version",
        row.pid,
        "approval_requested",
        "Your approval has been requested",
    )
    .await?;
    tx.commit().await?;
    format::json(version_view(&row, false))
}

/// What an approval's outcome does to the version under review: approved → `approved`;
/// rejected or withdrawn → back to `draft` (edit and resubmit). Safe to call twice.
///
/// # Errors
///
/// A database failure; the approval's decision stands.
pub(crate) async fn on_approval(
    ctx: &AppContext,
    approval: &approval_requests::Model,
) -> Result<()> {
    if approval.subject_kind != "document_version" {
        return Ok(());
    }
    let mut tx = crate::db::begin(ctx).await?;
    let Some(v) = sqlx::query_as::<_, versions::Model>(
        "SELECT * FROM document_versions WHERE pid = $1 LIMIT 1 FOR UPDATE",
    )
    .bind(approval.subject_pid)
    .fetch_optional(tx.conn())
    .await?
    else {
        return Ok(());
    };
    // Only the approval the version is waiting on counts, and only while it is in review.
    if v.approval_pid != Some(approval.pid) || v.status != Status::InReview.token() {
        return Ok(());
    }
    let (to, event) = match approval.status.as_str() {
        "approved" => (Status::Approved, "document_version_approved"),
        "rejected" | "withdrawn" => (Status::Draft, "document_version_returned_to_draft"),
        _ => return Ok(()),
    };
    rules::check_move(Status::InReview, to).map_err(|m| conflict(&m))?;
    let (doc, n) = (v.document_pid, v.version);
    let row: versions::Model = if to == Status::Draft {
        sqlx::query_as(
            "UPDATE document_versions SET status = $1, approval_pid = NULL WHERE id = $2 RETURNING *",
        )
    } else {
        sqlx::query_as("UPDATE document_versions SET status = $1 WHERE id = $2 RETURNING *")
    }
    .bind(to.token())
    .bind(v.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, event, approval.decided_by.as_deref().or(Some(&approval.requested_by)), Some(serde_json::json!({ "document_pid": doc, "version": n, "approval_pid": approval.pid, "comment": approval.decision_comment })))
        .await
        .map_err(Error::Model)?;
    tx.commit().await?;
    Ok(())
}

/// `POST /api/documents/{pid}/versions/{version}/make-effective` — bring an approved
/// version into force **today**. Needs the approval that approved it to really be
/// approved; the version it replaces is superseded the same day, naming this one.
#[debug_handler]
async fn make_effective(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path((pid, n)): Path<(String, i32)>,
) -> Result<Response> {
    let caller = MaybeAuthUser(Some(claims.clone()));
    let doc = find_doc(&ctx, &caller, &pid).await?;
    owner_only(&doc, &claims)?;
    let mut tx = crate::db::begin(&ctx).await?;
    super::advisory_lock_on(&mut tx, "document", doc.pid).await?;
    let vs = versions_in(&mut tx, doc.pid).await?;
    let v = vs
        .iter()
        .find(|v| v.version == n)
        .ok_or(Error::NotFound)?
        .clone();
    rules::check_move(
        Status::parse(&v.status).unwrap_or(Status::Superseded),
        Status::Effective,
    )
    .map_err(|m| conflict(&m))?;
    // The approval must exist, be about this version, and have been approved: a status
    // flag is not an approval.
    let Some(approval_pid) = v.approval_pid else {
        return Err(unprocessable(
            "a document with no approval cannot become effective",
        ));
    };
    let approval: Option<approval_requests::Model> =
        sqlx::query_as("SELECT * FROM approval_requests WHERE pid = $1 LIMIT 1")
            .bind(approval_pid)
            .fetch_optional(tx.conn())
            .await?;
    let approved = approval.as_ref().is_some_and(|a| {
        a.status == "approved" && a.subject_kind == "document_version" && a.subject_pid == v.pid
    });
    if !approved {
        return Err(unprocessable(
            "a document with no approval cannot become effective: its approval request is not approved",
        ));
    }
    let current = vs.iter().find(|x| x.status == Status::Effective.token());
    let today = crate::tz::deployment_today();
    let from = rules::effective_date(today, current.and_then(|c| c.effective_from))
        .map_err(|m| conflict(&m))?;
    // The version it replaces is superseded first (one effective at a time), naming this one.
    if let Some(old) = current {
        rules::check_move(Status::Effective, Status::Superseded).map_err(|m| conflict(&m))?;
        let (old_pid, old_n) = (old.pid, old.version);
        sqlx::query(
            "UPDATE document_versions SET status = $1, superseded_on = $2, superseded_by = $3
             WHERE id = $4 RETURNING id",
        )
        .bind(Status::Superseded.token())
        .bind(from)
        .bind(v.pid)
        .bind(old.id)
        .fetch_one(tx.conn())
        .await?;
        AuditModel::record_on(tx.conn(), old_pid, "document_version_superseded", Some(&claims.sub), Some(serde_json::json!({ "document_pid": doc.pid, "version": old_n, "superseded_by": v.pid, "on": from })))
            .await
            .map_err(Error::Model)?;
    }
    let row: versions::Model = sqlx::query_as(
        "UPDATE document_versions SET status = $1, effective_from = $2 WHERE id = $3 RETURNING *",
    )
    .bind(Status::Effective.token())
    .bind(from)
    .bind(v.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "document_version_made_effective",
        Some(&claims.sub),
        Some(serde_json::json!({ "document_pid": doc.pid, "version": n, "effective_from": from })),
    )
    .await
    .map_err(Error::Model)?;
    tx.commit().await?;
    format::json(version_view(&row, false))
}

/// The document routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/documents", post(create))
        .add("/documents", get(list))
        .add("/documents/{pid}", get(get_doc))
        .add("/documents/{pid}/in-force", get(in_force))
        .add("/documents/{pid}/versions", post(new_version))
        .add("/documents/{pid}/versions/{version}", get(get_version))
        .add("/documents/{pid}/versions/{version}", put(edit_version))
        .add("/documents/{pid}/versions/{version}/submit", post(submit))
        .add(
            "/documents/{pid}/versions/{version}/make-effective",
            post(make_effective),
        )
}

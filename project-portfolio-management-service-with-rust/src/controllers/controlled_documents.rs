//! Controlled documents (PL-6). Rules in [`crate::documents`]; this is the wiring.
//!
//! A document's versions move `draft → in_review → approved → effective → superseded`.
//! Review is a PL-1 approval request on the version (`document_version` subject), so only
//! the named approver approves; a version becomes `effective` only if that approval
//! really was approved, and in the same step the version it replaces is `superseded`
//! naming its successor. Once a version has left draft its text never changes (an
//! edit is the next draft) — in the database itself as well as here. Every transition
//! is audited.

use super::{conflict, forbidden};
use super::{db_err, unprocessable};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, QueryOrder, QuerySelect, TransactionTrait};
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
    let doc = docs::Entity::find()
        .filter(docs::Column::Pid.eq(pid))
        .filter(docs::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if let Some(plan) = doc.plan_pid {
        if super::guests::caller_is_guest(caller)
            && !super::guests::guest_covers_plan(&ctx.db, caller, plan, "read").await
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

async fn versions_of<C: ConnectionTrait>(conn: &C, doc: Uuid) -> Result<Vec<versions::Model>> {
    versions::Entity::find()
        .filter(versions::Column::DocumentPid.eq(doc))
        .order_by_asc(versions::Column::Version)
        .all(conn)
        .await
        .map_err(db_err)
}

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

async fn doc_view(ctx: &AppContext, doc: &docs::Model) -> Result<serde_json::Value> {
    let vs = versions_of(&ctx.db, doc.pid).await?;
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
        "in_force_today": rules::in_force(&spans, crate::tz::utc_today()),
        "versions": vs.iter().map(|v| version_view(v, false)).collect::<Vec<_>>(),
    }))
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    title: String,
    #[serde(default)]
    plan_pid: Option<Uuid>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    change_summary: Option<String>,
}

async fn insert_version<C: ConnectionTrait>(
    conn: &C,
    doc: Uuid,
    version: i32,
    body: &str,
    summary: Option<&str>,
    by: &str,
) -> Result<versions::Model> {
    versions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        document_pid: ActiveValue::set(doc),
        version: ActiveValue::set(version),
        body: ActiveValue::set(body.to_string()),
        change_summary: ActiveValue::set(summary.map(str::to_string)),
        status: ActiveValue::set(Status::Draft.token().to_string()),
        approval_pid: ActiveValue::set(None),
        effective_from: ActiveValue::set(None),
        superseded_on: ActiveValue::set(None),
        superseded_by: ActiveValue::set(None),
        created_by: ActiveValue::set(Some(by.to_string())),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map_err(db_err)
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let doc = docs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        title: ActiveValue::set(b.title.trim().to_string()),
        plan_pid: ActiveValue::set(b.plan_pid),
        created_by: ActiveValue::set(Some(claims.sub.clone())),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    let v = insert_version(
        &txn,
        doc.pid,
        1,
        b.body.as_deref().unwrap_or(""),
        b.change_summary.as_deref(),
        &claims.sub,
    )
    .await?;
    AuditModel::record(
        &txn,
        doc.pid,
        "document_created",
        Some(&claims.sub),
        Some(serde_json::json!({ "title": doc.title, "plan_pid": doc.plan_pid })),
    )
    .await
    .map_err(Error::Model)?;
    AuditModel::record(
        &txn,
        v.pid,
        "document_version_created",
        Some(&claims.sub),
        Some(serde_json::json!({ "document_pid": doc.pid, "version": 1 })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    format::json(doc_view(&ctx, &doc).await?)
}

/// `GET /api/documents` — the documents the caller may read, with the version in force today.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<super::paging::PageQuery>,
) -> Result<Response> {
    require_signed_in(&caller)?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let rows = scope.keep(
        docs::Entity::find()
            .filter(docs::Column::DeletedAt.is_null())
            .order_by_asc(docs::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?,
    );
    let guest = super::guests::caller_is_guest(&caller);
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|d| !guest || d.plan_pid.is_some())
        .collect();
    // Only the requested page is turned into views (each view reads its versions).
    let page = super::pagination::Page {
        limit: q.limit,
        offset: q.offset,
    };
    page.check_offset()?;
    let (limit, offset) = page.resolve(super::pagination::MAX_LIMIT);
    let total = rows.len() as u64;
    let mut out = Vec::new();
    for d in rows
        .iter()
        .skip(usize::try_from(offset).unwrap_or(usize::MAX))
        .take(usize::try_from(limit).unwrap_or(usize::MAX))
    {
        out.push(doc_view(&ctx, d).await?);
    }
    Ok(super::pagination::with_page_headers(
        format::json(out)?,
        total,
        limit,
        offset,
    ))
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
    let v = versions_of(&ctx.db, doc.pid)
        .await?
        .into_iter()
        .find(|v| v.version == version)
        .ok_or(Error::NotFound)?;
    format::json(version_view(&v, true))
}

#[derive(Debug, Deserialize)]
struct InForceParams {
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
    let date = q.on.unwrap_or_else(crate::tz::utc_today);
    let vs = versions_of(&ctx.db, doc.pid).await?;
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

#[derive(Debug, Deserialize)]
struct DraftBody {
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    change_summary: Option<String>,
}

async fn find_version(ctx: &AppContext, doc: Uuid, n: i32) -> Result<versions::Model> {
    versions_of(&ctx.db, doc)
        .await?
        .into_iter()
        .find(|v| v.version == n)
        .ok_or(Error::NotFound)
}

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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    // Serialize per document so two people cannot both open "the next draft".
    super::advisory_lock(&txn, "document", doc.pid).await?;
    let vs = versions_of(&txn, doc.pid).await?;
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
        &txn,
        doc.pid,
        next,
        &body,
        b.change_summary.as_deref(),
        &claims.sub,
    )
    .await?;
    AuditModel::record(
        &txn,
        v.pid,
        "document_version_created",
        Some(&claims.sub),
        Some(serde_json::json!({ "document_pid": doc.pid, "version": next })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
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
    let mut a: versions::ActiveModel = v.into();
    if let Some(body) = &b.body {
        a.body = ActiveValue::set(body.clone());
    }
    if b.change_summary.is_some() {
        a.change_summary = ActiveValue::set(b.change_summary.clone());
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, row.pid, "document_version_edited", Some(&claims.sub), Some(serde_json::json!({ "document_pid": doc.pid, "version": n, "chars_before": was, "chars_after": row.body.chars().count() })))
        .await
        .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    format::json(version_view(&row, true))
}

#[derive(Debug, Deserialize)]
struct SubmitBody {
    approver_ref: String,
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    // Serialize per document, then re-read: two submits cannot both pass the draft check.
    super::advisory_lock(&txn, "document", doc.pid).await?;
    let v = versions_of(&txn, doc.pid)
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
    let approval = approval_requests::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        subject_kind: ActiveValue::set("document_version".to_string()),
        subject_pid: ActiveValue::set(v.pid),
        requested_by: ActiveValue::set(claims.sub.clone()),
        approver_ref: ActiveValue::set(b.approver_ref.clone()),
        message: ActiveValue::set(Some(
            b.message
                .clone()
                .unwrap_or_else(|| format!("Approve {} version {n}", doc.title)),
        )),
        status: ActiveValue::set("pending".to_string()),
        decision_comment: ActiveValue::set(None),
        decided_by: ActiveValue::set(None),
        decided_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    let mut a: versions::ActiveModel = v.into();
    a.status = ActiveValue::set(Status::InReview.token().to_string());
    a.approval_pid = ActiveValue::set(Some(approval.pid));
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, row.pid, "document_version_submitted", Some(&claims.sub), Some(serde_json::json!({ "document_pid": doc.pid, "version": n, "approval_pid": approval.pid })))
        .await
        .map_err(Error::Model)?;
    cap_models::notify(
        &txn,
        &b.approver_ref,
        "document_version",
        row.pid,
        "approval_requested",
        "Your approval has been requested",
    )
    .await?;
    txn.commit().await.map_err(db_err)?;
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let Some(v) = versions::Entity::find()
        .filter(versions::Column::Pid.eq(approval.subject_pid))
        .lock_exclusive()
        .one(&txn)
        .await
        .map_err(db_err)?
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
    let mut a: versions::ActiveModel = v.into();
    a.status = ActiveValue::set(to.token().to_string());
    if to == Status::Draft {
        a.approval_pid = ActiveValue::set(None);
    }
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, row.pid, event, approval.decided_by.as_deref().or(Some(&approval.requested_by)), Some(serde_json::json!({ "document_pid": doc, "version": n, "approval_pid": approval.pid, "comment": approval.decision_comment })))
        .await
        .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    super::advisory_lock(&txn, "document", doc.pid).await?;
    let vs = versions_of(&txn, doc.pid).await?;
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
    let approval = approval_requests::Entity::find()
        .filter(approval_requests::Column::Pid.eq(approval_pid))
        .one(&txn)
        .await
        .map_err(db_err)?;
    let approved = approval.as_ref().is_some_and(|a| {
        a.status == "approved" && a.subject_kind == "document_version" && a.subject_pid == v.pid
    });
    if !approved {
        return Err(unprocessable(
            "a document with no approval cannot become effective: its approval request is not approved",
        ));
    }
    let current = vs.iter().find(|x| x.status == Status::Effective.token());
    let today = crate::tz::utc_today();
    let from = rules::effective_date(today, current.and_then(|c| c.effective_from))
        .map_err(|m| conflict(&m))?;
    // The version it replaces is superseded first (one effective at a time), naming this one.
    if let Some(old) = current {
        rules::check_move(Status::Effective, Status::Superseded).map_err(|m| conflict(&m))?;
        let (old_pid, old_n) = (old.pid, old.version);
        let mut a: versions::ActiveModel = old.clone().into();
        a.status = ActiveValue::set(Status::Superseded.token().to_string());
        a.superseded_on = ActiveValue::set(Some(from));
        a.superseded_by = ActiveValue::set(Some(v.pid));
        a.update(&txn).await.map_err(db_err)?;
        AuditModel::record(&txn, old_pid, "document_version_superseded", Some(&claims.sub), Some(serde_json::json!({ "document_pid": doc.pid, "version": old_n, "superseded_by": v.pid, "on": from })))
            .await
            .map_err(Error::Model)?;
    }
    let mut a: versions::ActiveModel = v.clone().into();
    a.status = ActiveValue::set(Status::Effective.token().to_string());
    a.effective_from = ActiveValue::set(Some(from));
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "document_version_made_effective",
        Some(&claims.sub),
        Some(serde_json::json!({ "document_pid": doc.pid, "version": n, "effective_from": from })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
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

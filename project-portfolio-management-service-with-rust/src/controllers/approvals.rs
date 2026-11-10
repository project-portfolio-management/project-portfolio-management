//! Approval requests (PL-1): ask a named person to approve a plan, task,
//! risk, proposal, idea, canvas item, budget line or scenario, and record
//! their decision. Pure rules in [`crate::approvals`].
//!
//! Only the **named approver** may decide, a decision is final, a rejection
//! must say why, and only the requester may withdraw. The subject's owning
//! plan's record-level read check applies to every call. The approver is
//! told (`approval_requested`) and the requester is told of the outcome
//! (`approval_decided`), both honouring notification preferences.

use super::unprocessable;
use super::{conflict, forbidden};
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::governance::valid_ref;
use crate::approvals::{self as rules, Decision};
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::discussion as discussion_rules;
use crate::models::_entities::approval_requests;
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;

/// Loads an approval request by public id, `NotFound` when unknown.
async fn find(ctx: &AppContext, pid: &str) -> Result<approval_requests::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM approval_requests WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// `POST /api/approvals` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RequestPayload {
    /// Kind of the subject to be approved.
    subject_kind: String,
    /// Public id of the subject, as text; checked against a live subject by the handler.
    subject_pid: String,
    /// The one person asked: a `person:` / `worker:` URN.
    approver_ref: String,
    #[serde(default)]
    /// Optional note to the approver; absent means none.
    message: Option<String>,
}

/// `POST /api/approvals` — ask someone to approve a subject.
#[debug_handler]
async fn request_approval(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(p): Json<RequestPayload>,
) -> Result<Response> {
    if !discussion_rules::is_token(discussion_rules::SUBJECT_KINDS, &p.subject_kind) {
        return Err(unprocessable(&format!(
            "unknown subject_kind (allowed: {:?})",
            discussion_rules::SUBJECT_KINDS
        )));
    }
    if !valid_ref(&p.approver_ref, &["person", "worker"]) {
        return Err(unprocessable("approver_ref must be a person:/worker: URN"));
    }
    if p.message
        .as_deref()
        .is_some_and(|m| m.chars().count() > rules::MAX_TEXT)
    {
        return Err(unprocessable("message: too long"));
    }
    let caller = MaybeAuthUser(Some(claims.clone()));
    let subject = resolve_subject(&ctx, &caller, &p.subject_kind, &p.subject_pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: approval_requests::Model = sqlx::query_as(
        "INSERT INTO approval_requests
             (pid, subject_kind, subject_pid, requested_by, approver_ref, message, status,
              decision_comment, decided_by, decided_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'pending', NULL, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&p.subject_kind)
    .bind(subject.pid)
    .bind(&claims.sub)
    .bind(&p.approver_ref)
    .bind(&p.message)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "approval_requested",
        Some(&claims.sub),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    cap_models::notify_on(
        &mut crate::db::pool(&ctx),
        &p.approver_ref,
        &p.subject_kind,
        subject.pid,
        "approval_requested",
        "Your approval has been requested",
    )
    .await
    .ok();
    format::json(row)
}

/// `GET /api/approvals` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// `approver` (waiting on me) or `requester` (I asked); absent = both.
    #[serde(default)]
    mine: Option<String>,
    #[serde(default)]
    /// Filter by request status; absent means all statuses.
    status: Option<String>,
    #[serde(default)]
    /// Filter by subject kind, used together with `subject_pid`.
    subject_kind: Option<String>,
    #[serde(default)]
    /// Filter by subject pid, used together with `subject_kind`.
    subject_pid: Option<String>,
}

/// `GET /api/approvals` — requests involving the caller (as approver or
/// requester) or, narrowed by `subject_*`, attached to one subject the
/// caller can read.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(p): Query<ListParams>,
) -> Result<Response> {
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM approval_requests WHERE ");
    if let (Some(kind), Some(pid)) = (p.subject_kind.as_deref(), p.subject_pid.as_deref()) {
        let caller = MaybeAuthUser(Some(claims.clone()));
        let subject = resolve_subject(&ctx, &caller, kind, pid).await?;
        q.push("subject_kind = ")
            .push_bind(kind.to_string())
            .push(" AND subject_pid = ")
            .push_bind(subject.pid);
    } else {
        let me = Uuid::parse_str(&claims.sub).ok();
        let refs: Vec<String> = me
            .map(|u| vec![format!("person:{u}"), format!("worker:{u}")])
            .unwrap_or_default();
        match p.mine.as_deref() {
            Some("approver") => {
                q.push("approver_ref = ANY(").push_bind(refs).push(")");
            }
            Some("requester") => {
                q.push("requested_by = ").push_bind(claims.sub.clone());
            }
            None => {
                q.push("(approver_ref = ANY(")
                    .push_bind(refs)
                    .push(") OR requested_by = ")
                    .push_bind(claims.sub.clone())
                    .push(")");
            }
            Some(_) => return Err(unprocessable("mine must be approver or requester")),
        }
    }
    if let Some(status) = p.status {
        if !rules::STATUSES.contains(&status.as_str()) {
            return Err(unprocessable(&format!(
                "status must be one of {:?}",
                rules::STATUSES
            )));
        }
        q.push(" AND status = ").push_bind(status);
    }
    q.push(" ORDER BY id DESC LIMIT 500");
    let rows: Vec<approval_requests::Model> =
        q.build_query_as().fetch_all(crate::db::pool(&ctx)).await?;
    format::json(rows)
}

/// `POST /api/approvals/{pid}/decide` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DecidePayload {
    /// `approved` | `rejected`.
    decision: String,
    #[serde(default)]
    /// Optional comment recorded with the decision; absent means none.
    comment: Option<String>,
}

/// `POST /api/approvals/{pid}/decide` — the named approver decides.
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(p): Json<DecidePayload>,
) -> Result<Response> {
    let row = find(&ctx, &pid).await?;
    if !rules::is_approver(&row.approver_ref, Some(&claims.sub)) {
        return Err(forbidden("only the named approver may decide this request"));
    }
    let decision = Decision::parse(&p.decision)
        .ok_or_else(|| unprocessable("decision must be approved or rejected"))?;
    let status =
        rules::decide(&row.status, decision, p.comment.as_deref()).map_err(|r| match r {
            rules::Refusal::NotPending(_) => conflict(&r.message()),
            _ => unprocessable(&r.message()),
        })?;
    // The subject's owning plan must still be readable by the decider.
    let caller = MaybeAuthUser(Some(claims.clone()));
    resolve_subject(
        &ctx,
        &caller,
        &row.subject_kind,
        &row.subject_pid.to_string(),
    )
    .await?;
    let (request_pid, requester, subject_kind, subject_pid) = (
        row.pid,
        row.requested_by.clone(),
        row.subject_kind.clone(),
        row.subject_pid,
    );
    let mut tx = crate::db::begin(&ctx).await?;
    let updated = settle(
        &mut tx,
        request_pid,
        status,
        p.comment.as_ref().map(|c| c.trim().to_string()),
        Some(claims.sub.clone()),
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        request_pid,
        &format!("approval_{status}"),
        Some(&claims.sub),
        Some(serde_json::json!({
            "subject_kind": subject_kind,
            "subject_pid": subject_pid,
            "comment": p.comment,
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    // An approval that belongs to a process moves that process on (or ends it).
    super::processes::on_approval(&ctx, &updated).await?;
    super::controlled_documents::on_approval(&ctx, &updated).await?;
    if let Some(target) = discussion_rules::sub_as_recipient(&requester) {
        cap_models::notify_on(
            &mut crate::db::pool(&ctx),
            &target,
            &subject_kind,
            subject_pid,
            "approval_decided",
            &format!("Your approval request was {status}"),
        )
        .await
        .ok();
    }
    format::json(updated)
}

/// `POST /api/approvals/{pid}/withdraw` — the requester withdraws a
/// pending request. A decision is not touched.
#[debug_handler]
async fn withdraw(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find(&ctx, &pid).await?;
    if row.requested_by != claims.sub {
        return Err(forbidden("only the requester may withdraw this request"));
    }
    let status = rules::withdraw(&row.status).map_err(|r| conflict(&r.message()))?;
    let request_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    let updated = settle(&mut tx, request_pid, status, None, None).await?;
    AuditModel::record_on(
        tx.conn(),
        request_pid,
        "approval_withdrawn",
        Some(&claims.sub),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    super::processes::on_approval(&ctx, &updated).await?;
    super::controlled_documents::on_approval(&ctx, &updated).await?;
    format::json(updated)
}

/// Move a request out of `pending`, and only out of `pending`: the write itself carries
/// the condition, so of two racing decisions (or a decision and a withdrawal) exactly one
/// lands and the other is a 409 — never a last-write-wins that leaves what hangs off the
/// request (a process step, a document version) disagreeing with it.
async fn settle(
    tx: &mut crate::db::Tx,
    pid: Uuid,
    status: &str,
    comment: Option<String>,
    decided_by: Option<String>,
) -> Result<approval_requests::Model> {
    let done = if decided_by.is_some() {
        sqlx::query(
            "UPDATE approval_requests
             SET status = $1, decision_comment = $2, decided_by = $3, decided_at = $4
             WHERE pid = $5 AND status = 'pending'",
        )
        .bind(status)
        .bind(comment)
        .bind(decided_by)
        .bind(chrono::Utc::now().fixed_offset())
        .bind(pid)
        .execute(tx.conn())
        .await?
    } else {
        sqlx::query(
            "UPDATE approval_requests SET status = $1 WHERE pid = $2 AND status = 'pending'",
        )
        .bind(status)
        .bind(pid)
        .execute(tx.conn())
        .await?
    };
    if done.rows_affected() != 1 {
        return Err(conflict("this request was already decided or withdrawn"));
    }
    sqlx::query_as("SELECT * FROM approval_requests WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(tx.conn())
        .await?
        .ok_or(Error::NotFound)
}

/// The approval routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/approvals", post(request_approval))
        .add("/approvals", get(list))
        .add("/approvals/{pid}/decide", post(decide))
        .add("/approvals/{pid}/withdraw", post(withdraw))
}

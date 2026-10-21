//! Approval requests (PL-1): ask a named person to approve a plan, task,
//! risk, proposal, idea, canvas item, budget line or scenario, and record
//! their decision. Pure rules in [`crate::approvals`].
//!
//! Only the **named approver** may decide, a decision is final, a rejection
//! must say why, and only the requester may withdraw. The subject's owning
//! plan's record-level read check applies to every call. The approver is
//! told (`approval_requested`) and the requester is told of the outcome
//! (`approval_decided`), both honouring notification preferences.

use super::{conflict, forbidden};
use super::{db_err, unprocessable};
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
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

async fn find(ctx: &AppContext, pid: &str) -> Result<approval_requests::Model> {
    let pid = super::parse_pid(pid)?;
    approval_requests::Entity::find()
        .filter(approval_requests::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `POST /api/approvals` body.
#[derive(Debug, Deserialize)]
struct RequestPayload {
    subject_kind: String,
    subject_pid: String,
    /// The one person asked: a `person:` / `worker:` URN.
    approver_ref: String,
    #[serde(default)]
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
    let row = approval_requests::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        subject_kind: ActiveValue::set(p.subject_kind.clone()),
        subject_pid: ActiveValue::set(subject.pid),
        requested_by: ActiveValue::set(claims.sub.clone()),
        approver_ref: ActiveValue::set(p.approver_ref.clone()),
        message: ActiveValue::set(p.message.clone()),
        status: ActiveValue::set("pending".to_string()),
        decision_comment: ActiveValue::set(None),
        decided_by: ActiveValue::set(None),
        decided_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "approval_requested",
        Some(&claims.sub),
        None,
    )
    .await;
    cap_models::notify(
        &ctx.db,
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
#[derive(Debug, Deserialize)]
struct ListParams {
    /// `approver` (waiting on me) or `requester` (I asked); absent = both.
    #[serde(default)]
    mine: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    subject_kind: Option<String>,
    #[serde(default)]
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
    let mut q = approval_requests::Entity::find();
    if let (Some(kind), Some(pid)) = (p.subject_kind.as_deref(), p.subject_pid.as_deref()) {
        let caller = MaybeAuthUser(Some(claims.clone()));
        let subject = resolve_subject(&ctx, &caller, kind, pid).await?;
        q = q
            .filter(approval_requests::Column::SubjectKind.eq(kind))
            .filter(approval_requests::Column::SubjectPid.eq(subject.pid));
    } else {
        let me = Uuid::parse_str(&claims.sub).ok();
        let refs: Vec<String> = me
            .map(|u| vec![format!("person:{u}"), format!("worker:{u}")])
            .unwrap_or_default();
        let as_approver = approval_requests::Column::ApproverRef.is_in(refs);
        let as_requester = approval_requests::Column::RequestedBy.eq(claims.sub.as_str());
        q = match p.mine.as_deref() {
            Some("approver") => q.filter(as_approver),
            Some("requester") => q.filter(as_requester),
            None => q.filter(sea_orm::Condition::any().add(as_approver).add(as_requester)),
            Some(_) => return Err(unprocessable("mine must be approver or requester")),
        };
    }
    if let Some(status) = p.status {
        if !rules::STATUSES.contains(&status.as_str()) {
            return Err(unprocessable(&format!(
                "status must be one of {:?}",
                rules::STATUSES
            )));
        }
        q = q.filter(approval_requests::Column::Status.eq(status));
    }
    format::json(
        q.order_by_desc(approval_requests::Column::Id)
            .limit(500)
            .all(&ctx.db)
            .await
            .map_err(db_err)?,
    )
}

/// `POST /api/approvals/{pid}/decide` body.
#[derive(Debug, Deserialize)]
struct DecidePayload {
    /// `approved` | `rejected`.
    decision: String,
    #[serde(default)]
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
    let updated = settle(
        &ctx,
        request_pid,
        status,
        p.comment.as_ref().map(|c| c.trim().to_string()),
        Some(claims.sub.clone()),
    )
    .await?;
    AuditModel::record_or_log(
        &ctx.db,
        request_pid,
        &format!("approval_{status}"),
        Some(&claims.sub),
        Some(serde_json::json!({
            "subject_kind": subject_kind,
            "subject_pid": subject_pid,
            "comment": p.comment,
        })),
    )
    .await;
    // An approval that belongs to a process moves that process on (or ends it).
    super::processes::on_approval(&ctx, &updated).await?;
    super::controlled_documents::on_approval(&ctx, &updated).await?;
    if let Some(target) = discussion_rules::sub_as_recipient(&requester) {
        cap_models::notify(
            &ctx.db,
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
    let updated = settle(&ctx, request_pid, status, None, None).await?;
    AuditModel::record_or_log(
        &ctx.db,
        request_pid,
        "approval_withdrawn",
        Some(&claims.sub),
        None,
    )
    .await;
    super::processes::on_approval(&ctx, &updated).await?;
    super::controlled_documents::on_approval(&ctx, &updated).await?;
    format::json(updated)
}

/// Move a request out of `pending`, and only out of `pending`: the write itself carries
/// the condition, so of two racing decisions (or a decision and a withdrawal) exactly one
/// lands and the other is a 409 — never a last-write-wins that leaves what hangs off the
/// request (a process step, a document version) disagreeing with it.
async fn settle(
    ctx: &AppContext,
    pid: Uuid,
    status: &str,
    comment: Option<String>,
    decided_by: Option<String>,
) -> Result<approval_requests::Model> {
    let decided = decided_by.is_some();
    let mut q = approval_requests::Entity::update_many()
        .col_expr(
            approval_requests::Column::Status,
            sea_orm::sea_query::Expr::value(status.to_string()),
        )
        .filter(approval_requests::Column::Pid.eq(pid))
        .filter(approval_requests::Column::Status.eq("pending"));
    if decided {
        q = q
            .col_expr(
                approval_requests::Column::DecisionComment,
                sea_orm::sea_query::Expr::value(comment),
            )
            .col_expr(
                approval_requests::Column::DecidedBy,
                sea_orm::sea_query::Expr::value(decided_by),
            )
            .col_expr(
                approval_requests::Column::DecidedAt,
                sea_orm::sea_query::Expr::value(chrono::Utc::now().fixed_offset()),
            );
    }
    let done = q.exec(&ctx.db).await.map_err(db_err)?;
    if done.rows_affected != 1 {
        return Err(conflict("this request was already decided or withdrawn"));
    }
    approval_requests::Entity::find()
        .filter(approval_requests::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
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

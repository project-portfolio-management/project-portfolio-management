//! Comments and reactions (LT-4) on plans, tasks, risks, proposals,
//! ideas and canvas items. Pure rules in [`crate::discussion`]. The
//! subject is resolved to a live row first, and for plan-owned subjects
//! (plan, task, risk) the **owning plan's** record-level read check
//! applies — a comment thread never reveals a plan its reader could not
//! `GET`. Only a comment's author edits or deletes it; every edit keeps
//! the previous body; every mutation audits.

use std::collections::BTreeMap;

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::discussion as rules;
use crate::models::_entities::{
    budget_lines, canvas_items, comment_edits, comments, ideas, plans, proposals, reactions, risks,
    scenarios, tasks,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;

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

/// What the controller needs to know about a resolved subject.
pub(crate) struct Subject {
    pub(crate) pid: Uuid,
    /// The owning plan, for the record-level read check.
    pub(crate) plan_pid: Option<Uuid>,
    /// An `EntityRef` URN who owns / is assigned the subject, if any.
    pub(crate) owner_ref: Option<String>,
}

/// Resolve `(kind, pid)` to a live row, or `404`. Applies the owning
/// plan's record-level read check (`403` when the policy denies it).
#[allow(clippy::too_many_lines)] // one arm per subject kind
pub(crate) async fn resolve_subject(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    kind: &str,
    pid: &str,
) -> Result<Subject> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    let subject = match kind {
        "plan" => {
            let row = plans::Entity::find()
                .filter(plans::Column::Pid.eq(pid))
                .filter(plans::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: Some(row.pid),
                owner_ref: None,
            }
        }
        "task" => {
            let row = tasks::Entity::find()
                .filter(tasks::Column::Pid.eq(pid))
                .filter(tasks::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: Some(row.plan_pid),
                owner_ref: row.assignee_ref,
            }
        }
        "risk" => {
            let row = risks::Entity::find()
                .filter(risks::Column::Pid.eq(pid))
                .filter(risks::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: Some(row.plan_pid),
                owner_ref: row.owner_ref,
            }
        }
        "proposal" => {
            proposals::Entity::find()
                .filter(proposals::Column::Pid.eq(pid))
                .filter(proposals::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: None,
                owner_ref: None,
            }
        }
        "idea" => {
            ideas::Entity::find()
                .filter(ideas::Column::Pid.eq(pid))
                .filter(ideas::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: None,
                owner_ref: None,
            }
        }
        "canvas_item" => {
            let row = canvas_items::Entity::find()
                .filter(canvas_items::Column::Pid.eq(pid))
                .filter(canvas_items::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: None,
                owner_ref: row.owner_ref,
            }
        }
        "budget_line" => {
            let row = budget_lines::Entity::find()
                .filter(budget_lines::Column::Pid.eq(pid))
                .filter(budget_lines::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: Some(row.plan_pid),
                owner_ref: None,
            }
        }
        "scenario" => {
            scenarios::Entity::find()
                .filter(scenarios::Column::Pid.eq(pid))
                .filter(scenarios::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
                .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid: None,
                owner_ref: None,
            }
        }
        _ => return Err(unprocessable("unknown subject_kind")),
    };
    if let Some(plan_pid) = subject.plan_pid {
        // PL-7: a guest sees a subject only through a live grant on its plan.
        if super::guests::caller_is_guest(caller)
            && !super::guests::guest_covers_plan(&ctx.db, caller, plan_pid, "read").await
        {
            return Err(Error::NotFound);
        }
        // RM-8: a subject in a private plan is `404` to a non-member.
        if super::plan_access::hidden_plans(ctx, &[plan_pid], caller)
            .await
            .contains(&plan_pid)
        {
            return Err(Error::NotFound);
        }
        let stage = plans::Entity::find()
            .filter(plans::Column::Pid.eq(plan_pid))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .and_then(|p| p.stage);
        if crate::auth::authorize_record(
            caller,
            authentication_verifier::Action::Read,
            &crate::auth::plan_resource_attrs(stage.as_deref()),
        )
        .is_err()
        {
            return Err(forbidden("the owning plan is not readable by this caller"));
        }
    }
    Ok(subject)
}

/// `POST /api/comments` body.
#[derive(Debug, Deserialize)]
struct CommentPayload {
    subject_kind: String,
    subject_pid: String,
    body: String,
}

/// `POST /api/comments` — add a comment; notifies the subject's owner
/// and earlier commenters (never the author, each once).
#[debug_handler]
async fn create_comment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<CommentPayload>,
) -> Result<Response> {
    let problems = rules::comment_problems(&payload.subject_kind, &payload.body);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let subject =
        resolve_subject(&ctx, &caller, &payload.subject_kind, &payload.subject_pid).await?;
    if super::guests::caller_is_guest(&caller) {
        let Some(plan) = subject.plan_pid else {
            return Err(Error::NotFound);
        };
        if !super::guests::guest_covers_plan(&ctx.db, &caller, plan, "comment").await {
            return Err(forbidden("this guest grant does not allow commenting"));
        }
    }
    let prior: Vec<String> = comments::Entity::find()
        .filter(comments::Column::SubjectKind.eq(payload.subject_kind.as_str()))
        .filter(comments::Column::SubjectPid.eq(subject.pid))
        .filter(comments::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .filter_map(|c| c.author_sub)
        .collect();
    let row = comments::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        subject_kind: ActiveValue::set(payload.subject_kind.clone()),
        subject_pid: ActiveValue::set(subject.pid),
        author_sub: ActiveValue::set(caller.actor().map(str::to_string)),
        body: ActiveValue::set(payload.body.clone()),
        edited_at: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(&ctx.db, row.pid, "comment_added", caller.actor(), None)
        .await
        .ok();
    if let Some(plan_pid) = subject.plan_pid {
        crate::live::publish(
            plan_pid,
            "comment_added",
            serde_json::json!({ "subject_kind": payload.subject_kind, "subject_pid": subject.pid }),
        );
    }
    // Commenting follows the thread (unless the user opted out of auto-watch).
    if let Some(sub) = caller.actor() {
        super::watchers::ensure_watching(&ctx.db, &payload.subject_kind, subject.pid, sub).await;
    }
    let watching = super::watchers::watcher_subs(&ctx.db, &payload.subject_kind, subject.pid).await;
    for target in rules::notify_targets(
        caller.actor(),
        subject.owner_ref.as_deref(),
        &prior,
        &watching,
    ) {
        // Best-effort: a failed notification must never lose the comment.
        cap_models::notify(
            &ctx.db,
            &target,
            &payload.subject_kind,
            subject.pid,
            "comment",
            "New comment on a thread you follow",
        )
        .await
        .ok();
    }
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// Query naming a subject.
#[derive(Debug, Deserialize)]
struct SubjectParams {
    subject_kind: String,
    subject_pid: String,
}

/// `GET /api/comments?subject_kind=&subject_pid=` — the live thread,
/// oldest first.
#[debug_handler]
async fn list_comments(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<SubjectParams>,
) -> Result<Response> {
    let subject = resolve_subject(&ctx, &caller, &params.subject_kind, &params.subject_pid).await?;
    let rows = comments::Entity::find()
        .filter(comments::Column::SubjectKind.eq(params.subject_kind.as_str()))
        .filter(comments::Column::SubjectPid.eq(subject.pid))
        .filter(comments::Column::DeletedAt.is_null())
        .order_by_asc(comments::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows)
}

async fn find_comment(ctx: &AppContext, pid: &str) -> Result<comments::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    comments::Entity::find()
        .filter(comments::Column::Pid.eq(pid))
        .filter(comments::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// Only the author may change a comment.
fn require_author(comment: &comments::Model, caller: &MaybeAuthUser) -> Result<()> {
    if rules::may_change(comment.author_sub.as_deref(), caller.actor()) {
        Ok(())
    } else {
        Err(forbidden("only the author may change a comment"))
    }
}

/// `PUT /api/comments/{pid}` body.
#[derive(Debug, Deserialize)]
struct EditPayload {
    body: String,
}

/// `PUT /api/comments/{pid}` — the author edits; the previous body is
/// kept in `comment_edits` in the same transaction.
#[debug_handler]
async fn edit_comment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<EditPayload>,
) -> Result<Response> {
    let comment = find_comment(&ctx, &pid).await?;
    require_author(&comment, &caller)?;
    let problems = rules::comment_problems("plan", &payload.body);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    // Resolving the subject re-applies the owning plan's read check.
    resolve_subject(
        &ctx,
        &caller,
        &comment.subject_kind,
        &comment.subject_pid.to_string(),
    )
    .await?;
    let comment_pid = comment.pid;
    let previous = comment.body.clone();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    comment_edits::ActiveModel {
        comment_pid: ActiveValue::set(comment_pid),
        previous_body: ActiveValue::set(previous),
        edited_by: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    let mut active: comments::ActiveModel = comment.into();
    active.body = ActiveValue::set(payload.body.clone());
    active.edited_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    let row = active.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, comment_pid, "comment_edited", caller.actor(), None)
        .await
        .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    format::json(row)
}

/// `GET /api/comments/{pid}/history` — previous bodies, oldest first.
#[debug_handler]
async fn comment_history(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let comment = find_comment(&ctx, &pid).await?;
    resolve_subject(
        &ctx,
        &caller,
        &comment.subject_kind,
        &comment.subject_pid.to_string(),
    )
    .await?;
    let rows = comment_edits::Entity::find()
        .filter(comment_edits::Column::CommentPid.eq(comment.pid))
        .order_by_asc(comment_edits::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows)
}

/// `DELETE /api/comments/{pid}` — the author soft-deletes.
#[debug_handler]
async fn delete_comment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let comment = find_comment(&ctx, &pid).await?;
    require_author(&comment, &caller)?;
    let comment_pid = comment.pid;
    let mut active: comments::ActiveModel = comment.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        comment_pid,
        "comment_deleted",
        caller.actor(),
        None,
    )
    .await
    .ok();
    format::empty_json()
}

/// Who a reaction is attributed to: the token `sub`, or a fixed marker
/// when auth is off (one anonymous reactor per kind, never unbounded).
fn reactor(caller: &MaybeAuthUser) -> String {
    caller.actor().unwrap_or("anonymous").to_string()
}

/// `POST /api/reactions` body.
#[derive(Debug, Deserialize)]
struct ReactionPayload {
    subject_kind: String,
    subject_pid: String,
    kind: String,
}

/// `POST /api/reactions` — react. **Idempotent**: a repeat is `200` with
/// `created: false` and writes nothing.
#[debug_handler]
async fn add_reaction(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ReactionPayload>,
) -> Result<Response> {
    let problems = rules::reaction_problems(&payload.subject_kind, &payload.kind);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let subject =
        resolve_subject(&ctx, &caller, &payload.subject_kind, &payload.subject_pid).await?;
    let sub = reactor(&caller);
    let existing = reactions::Entity::find()
        .filter(reactions::Column::SubjectKind.eq(payload.subject_kind.as_str()))
        .filter(reactions::Column::SubjectPid.eq(subject.pid))
        .filter(reactions::Column::Sub.eq(sub.as_str()))
        .filter(reactions::Column::Kind.eq(payload.kind.as_str()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    if existing.is_some() {
        return format::json(serde_json::json!({ "created": false }));
    }
    let inserted = reactions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        subject_kind: ActiveValue::set(payload.subject_kind.clone()),
        subject_pid: ActiveValue::set(subject.pid),
        sub: ActiveValue::set(sub),
        kind: ActiveValue::set(payload.kind.clone()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await;
    // A concurrent identical insert loses to the unique index: that is
    // the same idempotent outcome, not an error.
    let created = inserted.is_ok();
    format::json(serde_json::json!({ "created": created }))
}

/// `DELETE /api/reactions?subject_kind=&subject_pid=&kind=` — remove
/// the caller's own reaction (idempotent).
#[derive(Debug, Deserialize)]
struct RemoveParams {
    subject_kind: String,
    subject_pid: String,
    kind: String,
}

#[debug_handler]
async fn remove_reaction(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<RemoveParams>,
) -> Result<Response> {
    let problems = rules::reaction_problems(&params.subject_kind, &params.kind);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let subject = resolve_subject(&ctx, &caller, &params.subject_kind, &params.subject_pid).await?;
    reactions::Entity::delete_many()
        .filter(reactions::Column::SubjectKind.eq(params.subject_kind.as_str()))
        .filter(reactions::Column::SubjectPid.eq(subject.pid))
        .filter(reactions::Column::Sub.eq(reactor(&caller)))
        .filter(reactions::Column::Kind.eq(params.kind.as_str()))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    format::empty_json()
}

/// `POST /api/reactions/remove` — the same as `DELETE /api/reactions`, for a
/// write-level user (the default policy reserves DELETE for admins).
#[debug_handler]
async fn remove_reaction_post(
    state: State<AppContext>,
    caller: MaybeAuthUser,
    Json(p): Json<ReactionPayload>,
) -> Result<Response> {
    remove_reaction(
        state,
        caller,
        Query(RemoveParams {
            subject_kind: p.subject_kind,
            subject_pid: p.subject_pid,
            kind: p.kind,
        }),
    )
    .await
}

/// `GET /api/reactions?subject_kind=&subject_pid=` — counts per kind
/// (every kind, zero included) and the caller's own.
#[debug_handler]
async fn list_reactions(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<SubjectParams>,
) -> Result<Response> {
    let subject = resolve_subject(&ctx, &caller, &params.subject_kind, &params.subject_pid).await?;
    let rows = reactions::Entity::find()
        .filter(reactions::Column::SubjectKind.eq(params.subject_kind.as_str()))
        .filter(reactions::Column::SubjectPid.eq(subject.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let counts: BTreeMap<&str, usize> =
        rules::reaction_counts(rows.iter().map(|r| r.kind.as_str()))
            .into_iter()
            .collect();
    let me = reactor(&caller);
    let mine: Vec<&str> = rows
        .iter()
        .filter(|r| r.sub == me)
        .map(|r| r.kind.as_str())
        .collect();
    format::json(serde_json::json!({ "counts": counts, "mine": mine }))
}

/// The comment and reaction routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/comments", post(create_comment))
        .add("/comments", get(list_comments))
        .add("/comments/{pid}", put(edit_comment))
        .add("/comments/{pid}", delete(delete_comment))
        // POST twins: the default policy reserves DELETE for admins, and an
        // author must always be able to remove their own words and reactions.
        .add("/comments/{pid}/delete", post(delete_comment))
        .add("/reactions/remove", post(remove_reaction_post))
        .add("/comments/{pid}/history", get(comment_history))
        .add("/reactions", post(add_reaction))
        .add("/reactions", get(list_reactions))
        .add("/reactions", delete(remove_reaction))
}

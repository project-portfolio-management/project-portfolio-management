//! Comments and reactions (LT-4) on plans, tasks, risks, proposals,
//! ideas and canvas items. Pure rules in [`crate::discussion`]. The
//! subject is resolved to a live row first, and for plan-owned subjects
//! (plan, task, risk) the **owning plan's** record-level read check
//! applies — a comment thread never reveals a plan its reader could not
//! `GET`. Only a comment's author edits or deletes it; every edit keeps
//! the previous body; every mutation audits.

use super::forbidden;
use super::unprocessable;
use std::collections::BTreeMap;

use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::discussion as rules;
use crate::models::_entities::{comment_edits, comments};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;

/// What the controller needs to know about a resolved subject.
pub(crate) struct Subject {
    /// Internal id of the subject row (never the numeric key).
    pub(crate) pid: Uuid,
    /// The owning plan, for the record-level read check.
    pub(crate) plan_pid: Option<Uuid>,
    /// An `EntityRef` URN who owns / is assigned the subject, if any.
    pub(crate) owner_ref: Option<String>,
}

/// Resolve `(kind, pid)` to a live row, or `404`. Applies the owning
/// plan's record-level read check (`403` when the policy denies it).
#[expect(clippy::too_many_lines)] // one arm per subject kind
pub(crate) async fn resolve_subject(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    kind: &str,
    pid: &str,
) -> Result<Subject> {
    let pid = super::parse_pid(pid)?;
    let db = crate::db::pool(ctx);
    // A live row of one table by pid, or `404`; the table is a literal.
    macro_rules! live {
        ($sql:literal) => {
            sqlx::query_as($sql)
                .bind(pid)
                .fetch_optional(db)
                .await?
                .ok_or(Error::NotFound)?
        };
    }
    let subject = match kind {
        "plan" => {
            let (plan_pid,): (Uuid,) =
                live!("SELECT pid FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1");
            Subject {
                pid,
                plan_pid: Some(plan_pid),
                owner_ref: None,
            }
        }
        "task" => {
            let (plan_pid, owner_ref): (Uuid, Option<String>) = live!(
                "SELECT plan_pid, assignee_ref FROM tasks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
            );
            Subject {
                pid,
                plan_pid: Some(plan_pid),
                owner_ref,
            }
        }
        "risk" => {
            let (plan_pid, owner_ref): (Uuid, Option<String>) = live!(
                "SELECT plan_pid, owner_ref FROM risks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
            );
            Subject {
                pid,
                plan_pid: Some(plan_pid),
                owner_ref,
            }
        }
        "proposal" => {
            let (promoted,): (Option<Uuid>,) = live!(
                "SELECT promoted_plan_pid FROM proposals WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
            );
            // One that became a plan is that plan's.
            Subject {
                pid,
                plan_pid: promoted,
                owner_ref: None,
            }
        }
        "idea" => {
            let (_,): (Uuid,) =
                live!("SELECT pid FROM ideas WHERE pid = $1 AND deleted_at IS NULL LIMIT 1");
            Subject {
                pid,
                plan_pid: None,
                owner_ref: None,
            }
        }
        "canvas_item" => {
            let (canvas_pid, owner_ref): (Uuid, Option<String>) = live!(
                "SELECT canvas_pid, owner_ref FROM canvas_items WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
            );
            // An item is its canvas's, and a canvas may be free-standing.
            let (plan_pid,): (Option<Uuid>,) =
                sqlx::query_as("SELECT plan_pid FROM canvases WHERE pid = $1 LIMIT 1")
                    .bind(canvas_pid)
                    .fetch_optional(db)
                    .await?
                    .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid,
                owner_ref,
            }
        }
        "budget_line" => {
            let (plan_pid,): (Uuid,) = live!(
                "SELECT plan_pid FROM budget_lines WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
            );
            Subject {
                pid,
                plan_pid: Some(plan_pid),
                owner_ref: None,
            }
        }
        "scenario" => {
            let (_,): (Uuid,) =
                live!("SELECT pid FROM scenarios WHERE pid = $1 AND deleted_at IS NULL LIMIT 1");
            Subject {
                pid,
                plan_pid: None,
                owner_ref: None,
            }
        }
        "document_version" => {
            // A document version is read through the plan its document belongs to, if any.
            let (document_pid,): (Uuid,) =
                live!("SELECT document_pid FROM document_versions WHERE pid = $1 LIMIT 1");
            let (plan_pid,): (Option<Uuid>,) = sqlx::query_as(
                "SELECT plan_pid FROM controlled_documents WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
            )
            .bind(document_pid)
            .fetch_optional(db)
            .await?
            .ok_or(Error::NotFound)?;
            Subject {
                pid,
                plan_pid,
                owner_ref: None,
            }
        }
        _ => return Err(unprocessable("unknown subject_kind")),
    };
    if let Some(plan_pid) = subject.plan_pid {
        // PL-7: a guest sees a subject only through a live grant on its plan.
        if super::guests::caller_is_guest(caller)
            && !super::guests::guest_covers_plan(ctx, caller, plan_pid, "read").await
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
        let stage: Option<String> = sqlx::query_scalar::<_, Option<String>>(
            "SELECT stage FROM plans WHERE pid = $1 LIMIT 1",
        )
        .bind(plan_pid)
        .fetch_optional(db)
        .await?
        .flatten();
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CommentPayload {
    /// Kind of subject commented on (for example a task or plan); validated by `resolve_subject`.
    subject_kind: String,
    /// Public id of the subject; the caller must be able to read it.
    subject_pid: String,
    /// Comment text; length and emptiness are checked by the discussion rules.
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
        if !super::guests::guest_covers_plan(&ctx, &caller, plan, "comment").await {
            return Err(forbidden("this guest grant does not allow commenting"));
        }
    }
    let prior: Vec<String> = sqlx::query_scalar::<_, Option<String>>(
        "SELECT author_sub FROM comments WHERE subject_kind = $1 AND subject_pid = $2 AND deleted_at IS NULL",
    )
    .bind(payload.subject_kind.as_str())
    .bind(subject.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .into_iter()
    .flatten()
    .collect();
    let mut tx = crate::db::begin(&ctx).await?;
    let (row_pid,): (Uuid,) = sqlx::query_as(
        "INSERT INTO comments (pid, subject_kind, subject_pid, author_sub, body, edited_at, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(payload.subject_kind.as_str())
    .bind(subject.pid)
    .bind(caller.actor())
    .bind(payload.body.as_str())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row_pid, "comment_added", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    if let Some(plan_pid) = subject.plan_pid {
        crate::live::publish(
            plan_pid,
            "comment_added",
            serde_json::json!({ "subject_kind": payload.subject_kind, "subject_pid": subject.pid }),
        );
    }
    // Commenting follows the thread (unless the user opted out of auto-watch).
    if let Some(sub) = caller.actor() {
        super::watchers::ensure_watching(&ctx, &payload.subject_kind, subject.pid, sub).await;
    }
    let watching = super::watchers::watcher_subs(&ctx, &payload.subject_kind, subject.pid).await;
    for target in rules::notify_targets(
        caller.actor(),
        subject.owner_ref.as_deref(),
        &prior,
        &watching,
    ) {
        // Best-effort: a failed notification must never lose the comment.
        cap_models::notify_on(
            &mut crate::db::pool(&ctx),
            &target,
            &payload.subject_kind,
            subject.pid,
            "comment",
            "New comment on a thread you follow",
        )
        .await
        .ok();
    }
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// Query naming a subject.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SubjectParams {
    /// Kind of subject whose thread is requested.
    subject_kind: String,
    /// Public id of that subject.
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
    let rows: Vec<comments::Model> = sqlx::query_as(
        "SELECT * FROM comments WHERE subject_kind = $1 AND subject_pid = $2 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(params.subject_kind.as_str())
    .bind(subject.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(rows)
}

/// Load a live (not soft-deleted) comment by public id, or 404.
async fn find_comment(ctx: &AppContext, pid: &str) -> Result<comments::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM comments WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct EditPayload {
    /// The replacement comment text; the previous body is kept in `comment_edits`.
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
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "INSERT INTO comment_edits (comment_pid, previous_body, edited_by) VALUES ($1, $2, $3)",
    )
    .bind(comment_pid)
    .bind(comment.body.as_str())
    .bind(caller.actor())
    .execute(tx.conn())
    .await?;
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    let row: comments::Model =
        sqlx::query_as("UPDATE comments SET body = $1, edited_at = $2 WHERE id = $3 RETURNING *")
            .bind(payload.body.as_str())
            .bind(chrono::DateTime::<chrono::FixedOffset>::from(
                chrono::Utc::now(),
            ))
            .bind(comment.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        comment_pid,
        "comment_edited",
        caller.actor(),
        None,
    )
    .await?;
    tx.commit().await?;
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
    let rows: Vec<comment_edits::Model> =
        sqlx::query_as("SELECT * FROM comment_edits WHERE comment_pid = $1 ORDER BY id")
            .bind(comment.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(rows)
}

/// `DELETE /api/comments/{pid}` — the author soft-deletes, while they can still read the subject.
#[debug_handler]
async fn delete_comment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let comment = find_comment(&ctx, &pid).await?;
    require_author(&comment, &caller)?;
    // As for an edit: an author who can no longer read the subject's plan cannot touch the comment either.
    resolve_subject(
        &ctx,
        &caller,
        &comment.subject_kind,
        &comment.subject_pid.to_string(),
    )
    .await?;
    let comment_pid = comment.pid;
    let deleted_at = chrono::DateTime::<chrono::FixedOffset>::from(chrono::Utc::now());
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    sqlx::query_scalar::<_, i64>("UPDATE comments SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(deleted_at)
        .bind(comment.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        comment_pid,
        "comment_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// Who a reaction is attributed to: the token `sub`, or a fixed marker
/// when auth is off (one anonymous reactor per kind, never unbounded).
fn reactor(caller: &MaybeAuthUser) -> String {
    caller.actor().unwrap_or("anonymous").to_string()
}

/// `POST /api/reactions` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReactionPayload {
    /// Kind of subject reacted to.
    subject_kind: String,
    /// Public id of the subject reacted to.
    subject_pid: String,
    /// Reaction kind (emoji token); checked by `rules::reaction_problems`.
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
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM reactions WHERE subject_kind = $1 AND subject_pid = $2 AND sub = $3 AND kind = $4 LIMIT 1",
    )
    .bind(payload.subject_kind.as_str())
    .bind(subject.pid)
    .bind(sub.as_str())
    .bind(payload.kind.as_str())
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    if existing.is_some() {
        return format::json(serde_json::json!({ "created": false }));
    }
    let inserted = sqlx::query(
        "INSERT INTO reactions (pid, subject_kind, subject_pid, sub, kind) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(payload.subject_kind.as_str())
    .bind(subject.pid)
    .bind(sub.as_str())
    .bind(payload.kind.as_str())
    .execute(crate::db::pool(&ctx))
    .await;
    // A concurrent identical insert loses to the unique index: that is
    // the same idempotent outcome, not an error.
    let created = inserted.is_ok();
    format::json(serde_json::json!({ "created": created }))
}

/// `DELETE /api/reactions?subject_kind=&subject_pid=&kind=` — remove
/// the caller's own reaction (idempotent).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RemoveParams {
    /// Kind of subject the reaction is on.
    subject_kind: String,
    /// Public id of that subject.
    subject_pid: String,
    /// Reaction kind to remove.
    kind: String,
}

/// `DELETE /api/reactions` handler. Removes only the caller's own reaction (matched on the reactor identity); removing one that does not exist is not an error.
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
    sqlx::query(
        "DELETE FROM reactions WHERE subject_kind = $1 AND subject_pid = $2 AND sub = $3 AND kind = $4",
    )
    .bind(params.subject_kind.as_str())
    .bind(subject.pid)
    .bind(reactor(&caller))
    .bind(params.kind.as_str())
    .execute(crate::db::pool(&ctx))
    .await?;
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
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT kind, sub FROM reactions WHERE subject_kind = $1 AND subject_pid = $2",
    )
    .bind(params.subject_kind.as_str())
    .bind(subject.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let counts: BTreeMap<&str, usize> =
        rules::reaction_counts(rows.iter().map(|(kind, _)| kind.as_str()))
            .into_iter()
            .collect();
    let me = reactor(&caller);
    let mine: Vec<&str> = rows
        .iter()
        .filter(|(_, sub)| *sub == me)
        .map(|(kind, _)| kind.as_str())
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

//! Watchers (RM-3): follow a plan, task, risk, proposal, idea or canvas
//! item and be told when it changes. A watcher joins a comment's
//! notification targets and a task move's; it grants **no access** — the
//! subject stays readable only under the reader's own record-level ABAC,
//! and the notification text is generic, so nothing about a subject leaks
//! through it.
//!
//! Following is idempotent and personal: the caller's own token `sub`, and a
//! user's watch list is never readable by another. A user may switch off
//! **auto-watch** (the `auto_watch` preference) so creating or being
//! assigned a thing does not subscribe them.

use super::unprocessable;
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::discussion as rules;
use crate::models::_entities::watchers;
use crate::models::capabilities as cap_models;

/// The `sub`s watching a subject.
pub(crate) async fn watcher_subs(ctx: &AppContext, kind: &str, pid: Uuid) -> Vec<String> {
    sqlx::query_scalar("SELECT sub FROM watchers WHERE subject_kind = $1 AND subject_pid = $2")
        .bind(kind)
        .bind(pid)
        .fetch_all(crate::db::pool(ctx))
        .await
        .unwrap_or_default()
}

/// Subscribe `sub` unless they switched `auto_watch` off. Best-effort and
/// idempotent: a failure here must never fail the action that triggered it.
pub(crate) async fn ensure_watching(ctx: &AppContext, kind: &str, pid: Uuid, sub: &str) {
    let opted_out = sqlx::query(
        "SELECT 1 FROM notification_prefs WHERE sub = $1 AND kind = $2 AND enabled = false LIMIT 1",
    )
    .bind(sub)
    .bind("auto_watch")
    .fetch_optional(crate::db::pool(ctx))
    .await
    .ok()
    .flatten()
    .is_some();
    if opted_out {
        return;
    }
    insert_if_absent(ctx, kind, pid, sub).await.ok();
}

/// Insert a watch row unless one exists. `true` when a row was added.
async fn insert_if_absent(
    ctx: &AppContext,
    kind: &str,
    pid: Uuid,
    sub: &str,
) -> std::result::Result<bool, sqlx::Error> {
    let exists = sqlx::query(
        "SELECT 1 FROM watchers WHERE subject_kind = $1 AND subject_pid = $2 AND sub = $3 LIMIT 1",
    )
    .bind(kind)
    .bind(pid)
    .bind(sub)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .is_some();
    if exists {
        return Ok(false);
    }
    // A concurrent identical insert loses to the unique index: the same
    // idempotent outcome, not an error. On the pool, so the failed insert
    // aborts no one's transaction.
    Ok(sqlx::query(
        "INSERT INTO watchers (pid, subject_kind, subject_pid, sub) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(Uuid::new_v4())
    .bind(kind)
    .bind(pid)
    .bind(sub)
    .fetch_one(crate::db::pool(ctx))
    .await
    .is_ok())
}

/// Tell every watcher of a subject that it changed — never the actor who
/// changed it, each exactly once. Best-effort: a failed notification never
/// fails the change. The text is generic by design.
pub(crate) async fn notify_watchers(
    ctx: &AppContext,
    kind: &str,
    pid: Uuid,
    actor: Option<&str>,
    message: &str,
) {
    let subs = watcher_subs(ctx, kind, pid).await;
    for target in rules::notify_targets(actor, None, &[], &subs) {
        cap_models::notify_on(
            &mut crate::db::pool(ctx),
            &target,
            kind,
            pid,
            "watch_update",
            message,
        )
        .await
        .ok();
    }
}

/// `POST/DELETE/GET /api/watchers` subject query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SubjectParams {
    /// Kind of record to watch; must be one of the allowed subject kinds.
    subject_kind: String,
    /// Pid of the record, as text.
    subject_pid: String,
}

/// `POST /api/watchers` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct WatchPayload {
    /// Kind of record to follow; must be one of the allowed subject kinds.
    subject_kind: String,
    /// Pid of the record, as text.
    subject_pid: String,
}

/// `POST /api/watchers` — follow a subject (idempotent).
#[debug_handler]
async fn watch(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(p): Json<WatchPayload>,
) -> Result<Response> {
    if !rules::is_token(rules::SUBJECT_KINDS, &p.subject_kind) {
        return Err(unprocessable(&format!(
            "unknown subject_kind (allowed: {:?})",
            rules::SUBJECT_KINDS
        )));
    }
    // Following requires being able to read the subject at all.
    let caller = MaybeAuthUser(Some(claims.clone()));
    let subject = resolve_subject(&ctx, &caller, &p.subject_kind, &p.subject_pid).await?;
    let created = insert_if_absent(&ctx, &p.subject_kind, subject.pid, &claims.sub).await?;
    format::json(serde_json::json!({ "watching": true, "created": created }))
}

/// `DELETE /api/watchers?subject_kind=&subject_pid=` — stop following
/// (idempotent).
#[debug_handler]
async fn unwatch(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(p): Query<SubjectParams>,
) -> Result<Response> {
    let pid = super::parse_pid(&p.subject_pid)?;
    sqlx::query("DELETE FROM watchers WHERE subject_kind = $1 AND subject_pid = $2 AND sub = $3")
        .bind(p.subject_kind.as_str())
        .bind(pid)
        .bind(claims.sub.as_str())
        .execute(crate::db::pool(&ctx))
        .await?;
    format::empty_json()
}

/// `POST /api/watchers/unwatch` — the same as `DELETE`, for a write-level
/// user (the default policy reserves DELETE for admins).
#[debug_handler]
async fn unwatch_post(
    state: State<AppContext>,
    claims: AuthUser,
    Json(p): Json<WatchPayload>,
) -> Result<Response> {
    unwatch(
        state,
        claims,
        Query(SubjectParams {
            subject_kind: p.subject_kind,
            subject_pid: p.subject_pid,
        }),
    )
    .await
}

/// `GET /api/watchers?subject_kind=&subject_pid=` — how many follow a
/// subject and whether the caller does. Other watchers are never named.
#[debug_handler]
async fn status(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(p): Query<SubjectParams>,
) -> Result<Response> {
    let caller = MaybeAuthUser(Some(claims.clone()));
    let subject = resolve_subject(&ctx, &caller, &p.subject_kind, &p.subject_pid).await?;
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM watchers WHERE subject_kind = $1 AND subject_pid = $2",
    )
    .bind(p.subject_kind.as_str())
    .bind(subject.pid)
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    let mine = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM watchers WHERE subject_kind = $1 AND subject_pid = $2 AND sub = $3",
    )
    .bind(p.subject_kind.as_str())
    .bind(subject.pid)
    .bind(claims.sub.as_str())
    .fetch_one(crate::db::pool(&ctx))
    .await?
        > 0;
    format::json(serde_json::json!({ "count": count, "mine": mine }))
}

/// `GET /api/me/watching` — what the caller follows, newest first.
#[debug_handler]
async fn my_watching(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
) -> Result<Response> {
    // A watch outlives access to the plan, and stays stored for if access comes back.
    let sub = claims.sub.clone();
    let caller = crate::auth::MaybeAuthUser(Some(claims));
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let owned = scope.owned_pids(&ctx).await?;
    // The hidden pids as one bound array, which no number of them can take past the bind-parameter limit.
    let owned: Vec<Uuid> = owned.into_iter().collect();
    let rows: Vec<watchers::Model> = sqlx::query_as(
        "SELECT * FROM watchers
         WHERE sub = $1 AND subject_pid <> ALL($2)
         ORDER BY id DESC
         LIMIT $3",
    )
    .bind(sub.as_str())
    .bind(&owned)
    .bind(500_i64)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let items: Vec<serde_json::Value> = rows
        .iter()
        .map(
            |w| serde_json::json!({ "subject_kind": w.subject_kind, "subject_pid": w.subject_pid }),
        )
        .collect();
    format::json(serde_json::json!({ "count": items.len(), "items": items }))
}

/// The watcher routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/watchers", post(watch))
        .add("/watchers", get(status))
        .add("/watchers", delete(unwatch))
        .add("/watchers/unwatch", post(unwatch_post))
        .add("/me/watching", get(my_watching))
}

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

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, PaginatorTrait, QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::discussion as rules;
use crate::models::_entities::{notification_prefs, watchers};
use crate::models::capabilities as cap_models;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

/// The `sub`s watching a subject.
pub(crate) async fn watcher_subs(db: &DatabaseConnection, kind: &str, pid: Uuid) -> Vec<String> {
    watchers::Entity::find()
        .filter(watchers::Column::SubjectKind.eq(kind))
        .filter(watchers::Column::SubjectPid.eq(pid))
        .all(db)
        .await
        .map(|rows| rows.into_iter().map(|w| w.sub).collect())
        .unwrap_or_default()
}

/// Subscribe `sub` unless they switched `auto_watch` off. Best-effort and
/// idempotent: a failure here must never fail the action that triggered it.
pub(crate) async fn ensure_watching(db: &DatabaseConnection, kind: &str, pid: Uuid, sub: &str) {
    let opted_out = notification_prefs::Entity::find()
        .filter(notification_prefs::Column::Sub.eq(sub))
        .filter(notification_prefs::Column::Kind.eq("auto_watch"))
        .filter(notification_prefs::Column::Enabled.eq(false))
        .one(db)
        .await
        .ok()
        .flatten()
        .is_some();
    if opted_out {
        return;
    }
    insert_if_absent(db, kind, pid, sub).await.ok();
}

/// Insert a watch row unless one exists. `true` when a row was added.
async fn insert_if_absent(
    db: &DatabaseConnection,
    kind: &str,
    pid: Uuid,
    sub: &str,
) -> std::result::Result<bool, sea_orm::DbErr> {
    let exists = watchers::Entity::find()
        .filter(watchers::Column::SubjectKind.eq(kind))
        .filter(watchers::Column::SubjectPid.eq(pid))
        .filter(watchers::Column::Sub.eq(sub))
        .one(db)
        .await?
        .is_some();
    if exists {
        return Ok(false);
    }
    // A concurrent identical insert loses to the unique index: the same
    // idempotent outcome, not an error.
    Ok(watchers::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        subject_kind: ActiveValue::set(kind.to_string()),
        subject_pid: ActiveValue::set(pid),
        sub: ActiveValue::set(sub.to_string()),
        ..Default::default()
    }
    .insert(db)
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
    let subs = watcher_subs(&ctx.db, kind, pid).await;
    for target in rules::notify_targets(actor, None, &[], &subs) {
        cap_models::notify(&ctx.db, &target, kind, pid, "watch_update", message)
            .await
            .ok();
    }
}

/// `POST/DELETE/GET /api/watchers` subject query.
#[derive(Debug, Deserialize)]
struct SubjectParams {
    subject_kind: String,
    subject_pid: String,
}

/// `POST /api/watchers` body.
#[derive(Debug, Deserialize)]
struct WatchPayload {
    subject_kind: String,
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
    let created = insert_if_absent(&ctx.db, &p.subject_kind, subject.pid, &claims.sub)
        .await
        .map_err(db_err)?;
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
    let pid = Uuid::parse_str(&p.subject_pid).map_err(|_| Error::NotFound)?;
    watchers::Entity::delete_many()
        .filter(watchers::Column::SubjectKind.eq(p.subject_kind.as_str()))
        .filter(watchers::Column::SubjectPid.eq(pid))
        .filter(watchers::Column::Sub.eq(claims.sub.as_str()))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    format::empty_json()
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
    let all = watchers::Entity::find()
        .filter(watchers::Column::SubjectKind.eq(p.subject_kind.as_str()))
        .filter(watchers::Column::SubjectPid.eq(subject.pid));
    let count = all.clone().count(&ctx.db).await.map_err(db_err)?;
    let mine = all
        .filter(watchers::Column::Sub.eq(claims.sub.as_str()))
        .count(&ctx.db)
        .await
        .map_err(db_err)?
        > 0;
    format::json(serde_json::json!({ "count": count, "mine": mine }))
}

/// `GET /api/me/watching` — what the caller follows, newest first.
#[debug_handler]
async fn my_watching(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
) -> Result<Response> {
    let rows = watchers::Entity::find()
        .filter(watchers::Column::Sub.eq(claims.sub.as_str()))
        .order_by_desc(watchers::Column::Id)
        .limit(500)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
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
        .add("/me/watching", get(my_watching))
}

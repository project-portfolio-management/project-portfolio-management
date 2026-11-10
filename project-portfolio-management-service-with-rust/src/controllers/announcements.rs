//! The announcement feed. Rules in [`crate::announcements`]; this is the wiring.
//!
//! Anyone may read the live feed. **Publishing, editing and retracting** need
//! admission to the `comms` area by the policy (`access=admin` or `comms=publish`,
//! see [`crate::auth::service_rules`]; or authentication off, as for every write);
//! otherwise `403`. Audited. `?include=all` (publishers only) also lists the
//! scheduled and expired ones, which is how a publisher manages the feed.

use super::unprocessable;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::announcements::{self as rules, Severity, Sortable, State as Window};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::announcements;
use crate::models::audit_logs::Model as AuditModel;

/// Whether the caller may publish, edit or retract announcements.
fn may_publish(caller: &MaybeAuthUser) -> bool {
    crate::auth::area_allows(caller, "comms", authentication_verifier::Action::Write)
}

/// The `403` returned to a caller without publish rights.
fn forbidden() -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new(
            "forbidden",
            "publishing announcements needs access=admin or comms=publish",
        ),
    )
}

/// Renders one announcement for the API, including its computed window state (scheduled, live or expired) relative to `now`.
fn view(a: &announcements::Model, now: DateTime<Utc>) -> serde_json::Value {
    serde_json::json!({
        "pid": a.pid, "title": a.title, "body": a.body, "severity": a.severity,
        "pinned": a.pinned, "publish_at": a.publish_at, "expires_at": a.expires_at,
        "author_ref": a.author_ref,
        "state": rules::state(a.publish_at.into(), a.expires_at.map(Into::into), now),
    })
}

/// Query for `GET /api/announcements`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// `all` (publishers only) also lists scheduled and expired announcements; absent lists the live feed.
    #[serde(default)]
    include: Option<String>,
    /// Maximum announcements returned; the response reports `total` and `truncated` when more exist.
    #[serde(default)]
    limit: Option<u64>,
}

/// `GET /api/announcements` — the live feed: pinned, then severity, then newest.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let all = q.include.as_deref() == Some("all");
    if all && !may_publish(&caller) {
        return Err(forbidden());
    }
    let now = Utc::now();
    let rows: Vec<announcements::Model> =
        sqlx::query_as("SELECT * FROM announcements WHERE deleted_at IS NULL ORDER BY id DESC")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut keep: Vec<&announcements::Model> = rows
        .iter()
        .filter(|a| {
            all || rules::state(a.publish_at.into(), a.expires_at.map(Into::into), now)
                == Window::Live
        })
        .collect();
    let mut keyed: Vec<(Sortable, &announcements::Model)> = keep
        .drain(..)
        .map(|a| {
            (
                Sortable {
                    id: a.id,
                    pinned: a.pinned,
                    severity: Severity::parse(&a.severity).unwrap_or(Severity::Info),
                    publish_at: a.publish_at.into(),
                },
                a,
            )
        })
        .collect();
    keyed.sort_by(|a, b| {
        let mut pair = [a.0, b.0];
        rules::order(&mut pair);
        if pair[0] == a.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let limit = usize::try_from(q.limit.unwrap_or(50).clamp(1, 200)).unwrap_or(50);
    let total = keyed.len();
    format::json(serde_json::json!({
        "announcements": keyed.iter().take(limit).map(|(_, a)| view(a, now)).collect::<Vec<_>>(),
        "total": total,
        "truncated": total > limit,
        "as_of": now,
    }))
}

/// Body for publishing (`POST /api/announcements`) and replacing (`PUT /api/announcements/{pid}`) an announcement.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Input {
    /// Headline; validated and trimmed by the rules.
    title: String,
    /// Announcement text, sent as `body` in JSON.
    #[serde(rename = "body")]
    text: String,
    /// Severity token; defaults to `info`.
    #[serde(default = "info")]
    severity: String,
    /// Pinned announcements sort ahead of the rest of the feed.
    #[serde(default)]
    pinned: bool,
    /// Default now.
    #[serde(default)]
    publish_at: Option<DateTime<Utc>>,
    /// When it stops showing; absent means it never expires.
    #[serde(default)]
    expires_at: Option<DateTime<Utc>>,
}

/// Serde default for `severity`: `info`.
fn info() -> String {
    "info".to_string()
}

/// `POST /api/announcements` — publish (or schedule) an announcement.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<Input>,
) -> Result<Response> {
    if !may_publish(&caller) {
        return Err(forbidden());
    }
    let publish_at = b.publish_at.unwrap_or_else(Utc::now);
    let problems = rules::problems(&b.title, &b.text, &b.severity, publish_at, b.expires_at);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: announcements::Model = sqlx::query_as(
        "INSERT INTO announcements
             (pid, title, body, severity, pinned, publish_at, expires_at, author_ref, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.title.trim())
    .bind(b.text.trim())
    .bind(&b.severity)
    .bind(b.pinned)
    .bind(publish_at.fixed_offset())
    .bind(b.expires_at.map(|e| e.fixed_offset()))
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "announcement_published", caller.actor(),
        Some(serde_json::json!({ "title": row.title, "severity": row.severity, "pinned": row.pinned }))).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row, Utc::now()))
}

/// Loads a live (not retracted) announcement by public id, `404` when absent.
async fn find(ctx: &AppContext, pid: &str) -> Result<announcements::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM announcements WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// `PUT /api/announcements/{pid}` — replace the fields of an announcement.
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Input>,
) -> Result<Response> {
    if !may_publish(&caller) {
        return Err(forbidden());
    }
    let row = find(&ctx, &pid).await?;
    let publish_at = b.publish_at.unwrap_or_else(|| row.publish_at.into());
    let problems = rules::problems(&b.title, &b.text, &b.severity, publish_at, b.expires_at);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let was =
        serde_json::json!({ "title": row.title, "severity": row.severity, "pinned": row.pinned });
    let mut tx = crate::db::begin(&ctx).await?;
    let row: announcements::Model = sqlx::query_as(
        "UPDATE announcements
         SET title = $1, body = $2, severity = $3, pinned = $4, publish_at = $5, expires_at = $6
         WHERE id = $7
         RETURNING *",
    )
    .bind(b.title.trim())
    .bind(b.text.trim())
    .bind(&b.severity)
    .bind(b.pinned)
    .bind(publish_at.fixed_offset())
    .bind(b.expires_at.map(|e| e.fixed_offset()))
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "announcement_edited",
        caller.actor(),
        Some(serde_json::json!({ "from": was,
            "to": { "title": row.title, "severity": row.severity, "pinned": row.pinned } })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row, Utc::now()))
}

/// `DELETE /api/announcements/{pid}` — retract (soft delete, audited).
#[debug_handler]
async fn retract(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    if !may_publish(&caller) {
        return Err(forbidden());
    }
    let row = find(&ctx, &pid).await?;
    let pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE announcements SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(Utc::now().fixed_offset())
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        pid,
        "announcement_retracted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// The announcement routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/announcements", get(list))
        .add("/announcements", post(create))
        .add("/announcements/{pid}", put(update))
        .add("/announcements/{pid}", delete(retract))
}

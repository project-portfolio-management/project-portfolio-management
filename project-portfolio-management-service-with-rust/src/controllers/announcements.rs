//! The announcement feed. Rules in [`crate::announcements`]; this is the wiring.
//!
//! Anyone may read the live feed. **Publishing, editing and retracting** need
//! admission to the `comms` area by the policy (`access=admin` or `comms=publish`,
//! see [`crate::auth::service_rules`]; or authentication off, as for every write);
//! otherwise `403`. Audited. `?include=all` (publishers only) also lists the
//! scheduled and expired ones, which is how a publisher manages the feed.

use super::{db_err, unprocessable};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
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

fn forbidden() -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new(
            "forbidden",
            "publishing announcements needs access=admin or comms=publish",
        ),
    )
}

fn view(a: &announcements::Model, now: DateTime<Utc>) -> serde_json::Value {
    serde_json::json!({
        "pid": a.pid, "title": a.title, "body": a.body, "severity": a.severity,
        "pinned": a.pinned, "publish_at": a.publish_at, "expires_at": a.expires_at,
        "author_ref": a.author_ref,
        "state": rules::state(a.publish_at.into(), a.expires_at.map(Into::into), now),
    })
}

#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    include: Option<String>,
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
    let rows = announcements::Entity::find()
        .filter(announcements::Column::DeletedAt.is_null())
        .order_by_desc(announcements::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
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

#[derive(Debug, Deserialize)]
struct Input {
    title: String,
    #[serde(rename = "body")]
    text: String,
    #[serde(default = "info")]
    severity: String,
    #[serde(default)]
    pinned: bool,
    /// Default now.
    #[serde(default)]
    publish_at: Option<DateTime<Utc>>,
    #[serde(default)]
    expires_at: Option<DateTime<Utc>>,
}

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
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = announcements::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        title: ActiveValue::set(b.title.trim().to_string()),
        body: ActiveValue::set(b.text.trim().to_string()),
        severity: ActiveValue::set(b.severity.clone()),
        pinned: ActiveValue::set(b.pinned),
        publish_at: ActiveValue::set(publish_at.into()),
        expires_at: ActiveValue::set(b.expires_at.map(Into::into)),
        author_ref: ActiveValue::set(caller.actor().map(str::to_string)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(&txn, row.pid, "announcement_published", caller.actor(),
        Some(serde_json::json!({ "title": row.title, "severity": row.severity, "pinned": row.pinned }))).await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(view(&row, Utc::now()))
}

async fn find(ctx: &AppContext, pid: &str) -> Result<announcements::Model> {
    let pid = super::parse_pid(pid)?;
    announcements::Entity::find()
        .filter(announcements::Column::Pid.eq(pid))
        .filter(announcements::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
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
    let mut a: announcements::ActiveModel = row.into();
    a.title = ActiveValue::set(b.title.trim().to_string());
    a.body = ActiveValue::set(b.text.trim().to_string());
    a.severity = ActiveValue::set(b.severity.clone());
    a.pinned = ActiveValue::set(b.pinned);
    a.publish_at = ActiveValue::set(publish_at.into());
    a.expires_at = ActiveValue::set(b.expires_at.map(Into::into));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "announcement_edited",
        caller.actor(),
        Some(serde_json::json!({ "from": was,
            "to": { "title": row.title, "severity": row.severity, "pinned": row.pinned } })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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
    let mut a: announcements::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, pid, "announcement_retracted", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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

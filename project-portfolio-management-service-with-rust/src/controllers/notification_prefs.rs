//! Notification preferences and the caller's inbox (PL-5). Pure rules in
//! [`crate::notification_prefs`]; the suppression itself happens where a
//! notification is written ([`crate::models::capabilities::notify`]), so
//! **no** call site can forget to honour it.
//!
//! Everything here is the caller's own: identity is the token `sub`, and
//! one user's preferences are never readable by another. A `sub` that is
//! not a UUID cannot be addressed as a `person:` reference, so its
//! preferences could never take effect — that is refused up front rather
//! than silently stored.

use super::{db_err, unprocessable};
use std::collections::BTreeSet;

use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::models::_entities::{notification_prefs, notifications};
use crate::notification_prefs as rules;
use crate::tz;

/// The caller's user id in the form notifications are addressed by, or a
/// `422` saying why preferences cannot apply to them.
fn addressable(sub: &str) -> Result<Uuid> {
    Uuid::parse_str(sub).map_err(|_| {
        unprocessable("sub_not_addressable: this sign-in identity cannot receive person-addressed notifications")
    })
}

/// `PUT /api/me/notification-prefs` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PrefPayload {
    kind: String,
    #[serde(default)]
    channel: Option<String>,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default)]
    digest: Option<String>,
}

const fn default_enabled() -> bool {
    true
}

/// `PUT /api/me/notification-prefs` — set (upsert) one choice.
#[debug_handler]
async fn put_pref(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<PrefPayload>,
) -> Result<Response> {
    let sub = addressable(&claims.sub)?.to_string();
    let channel = payload
        .channel
        .clone()
        .unwrap_or_else(|| rules::DEFAULT_CHANNEL.to_string());
    let digest = payload
        .digest
        .clone()
        .unwrap_or_else(|| "instant".to_string());
    let problems = rules::problems(&payload.kind, &channel, &digest);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = notification_prefs::Entity::find()
        .filter(notification_prefs::Column::Sub.eq(sub.as_str()))
        .filter(notification_prefs::Column::Kind.eq(payload.kind.as_str()))
        .filter(notification_prefs::Column::Channel.eq(channel.as_str()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let row = match existing {
        Some(row) => {
            let mut active: notification_prefs::ActiveModel = row.into();
            active.enabled = ActiveValue::set(payload.enabled);
            active.digest = ActiveValue::set(digest);
            active.update(&ctx.db).await.map_err(db_err)?
        }
        None => notification_prefs::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            sub: ActiveValue::set(sub),
            kind: ActiveValue::set(payload.kind.clone()),
            channel: ActiveValue::set(channel),
            enabled: ActiveValue::set(payload.enabled),
            digest: ActiveValue::set(digest),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?,
    };
    format::json(row)
}

/// `GET /api/me/notification-prefs` — the caller's own choices, plus what
/// can be chosen and what applies when nothing is.
#[debug_handler]
async fn list_prefs(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    let sub = addressable(&claims.sub)?.to_string();
    let rows = notification_prefs::Entity::find()
        .filter(notification_prefs::Column::Sub.eq(sub))
        .order_by_asc(notification_prefs::Column::Kind)
        .order_by_asc(notification_prefs::Column::Channel)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(serde_json::json!({
        "prefs": rows,
        "known_kinds": rules::KINDS,
        "channels": rules::CHANNELS,
        "digests": rules::DIGESTS,
        "default": { "enabled": true, "digest": "instant" },
    }))
}

/// `DELETE /api/me/notification-prefs?kind=&channel=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ResetParams {
    kind: String,
    #[serde(default)]
    channel: Option<String>,
}

/// `DELETE /api/me/notification-prefs` — back to the default (idempotent).
#[debug_handler]
async fn reset_pref(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(params): Query<ResetParams>,
) -> Result<Response> {
    let sub = addressable(&claims.sub)?.to_string();
    let channel = params
        .channel
        .unwrap_or_else(|| rules::DEFAULT_CHANNEL.to_string());
    let problems = rules::problems(&params.kind, &channel, "instant");
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    notification_prefs::Entity::delete_many()
        .filter(notification_prefs::Column::Sub.eq(sub))
        .filter(notification_prefs::Column::Kind.eq(params.kind))
        .filter(notification_prefs::Column::Channel.eq(channel))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    format::empty_json()
}

/// `POST /api/me/notification-prefs/reset` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ResetBody {
    kind: String,
    #[serde(default)]
    channel: Option<String>,
}

/// `POST /api/me/notification-prefs/reset` — the same as `DELETE`, for a
/// write-level user (the default policy reserves DELETE for admins).
#[debug_handler]
async fn reset_pref_post(
    state: State<AppContext>,
    claims: AuthUser,
    Json(b): Json<ResetBody>,
) -> Result<Response> {
    reset_pref(
        state,
        claims,
        Query(ResetParams {
            kind: b.kind,
            channel: b.channel,
        }),
    )
    .await
}

/// `GET /api/me/inbox?tz=&unread=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct InboxParams {
    /// IANA zone; a daily digest groups by the caller's local day.
    tz: String,
    #[serde(default)]
    unread: Option<bool>,
}

/// Rows read per request.
const INBOX_CAP: u64 = 500;

/// `GET /api/me/inbox` — the caller's notifications with their digest
/// choices applied. A kind set to `daily` collapses into one line per
/// (subject, local day) that **lists every notification** it stands for;
/// nothing is dropped. `unread_count` counts individual notifications.
#[debug_handler]
async fn inbox(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(params): Query<InboxParams>,
) -> Result<Response> {
    let zone = tz::parse_zone(&params.tz).map_err(|e| unprocessable(&e))?;
    let user = addressable(&claims.sub)?;
    let refs = vec![format!("person:{user}"), format!("worker:{user}")];
    let mut query = notifications::Entity::find()
        .filter(notifications::Column::RecipientRef.is_in(refs))
        .filter(notifications::Column::DeletedAt.is_null());
    if params.unread.unwrap_or(false) {
        query = query.filter(notifications::Column::ReadAt.is_null());
    }
    let rows = query
        .order_by_desc(notifications::Column::Id)
        .limit(INBOX_CAP)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let daily: BTreeSet<String> = notification_prefs::Entity::find()
        .filter(notification_prefs::Column::Sub.eq(user.to_string()))
        .filter(notification_prefs::Column::Channel.eq(rules::DEFAULT_CHANNEL))
        .filter(notification_prefs::Column::Digest.eq("daily"))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| p.kind)
        .collect();
    let unread_count = rows.iter().filter(|n| n.read_at.is_none()).count();
    let entries = rows
        .into_iter()
        .map(|n| rules::Entry {
            pid: n.pid,
            kind: n.kind,
            subject_kind: n.subject_kind,
            subject_pid: n.subject_pid,
            message: n.message,
            created_at: n.created_at.to_utc(),
            read: n.read_at.is_some(),
        })
        .collect();
    let items = rules::collapse(entries, &daily, zone);
    format::json(serde_json::json!({
        "tz": zone.name(),
        "items": items,
        "unread_count": unread_count,
    }))
}

/// The preference and inbox routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/me/notification-prefs", put(put_pref))
        .add("/me/notification-prefs", get(list_prefs))
        .add("/me/notification-prefs", delete(reset_pref))
        .add("/me/notification-prefs/reset", post(reset_pref_post))
        .add("/me/inbox", get(inbox))
}

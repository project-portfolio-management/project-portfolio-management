//! Notification preferences and the caller's inbox (PL-5). Pure rules in
//! [`crate::notification_prefs`]; the suppression itself happens where a
//! notification is written ([`crate::models::capabilities::notify_on`]), so
//! **no** call site can forget to honour it.
//!
//! Everything here is the caller's own: identity is the token `sub`, and
//! one user's preferences are never readable by another. A `sub` that is
//! not a UUID cannot be addressed as a `person:` reference, so its
//! preferences could never take effect — that is refused up front rather
//! than silently stored.

use super::unprocessable;
use std::collections::BTreeSet;

use crate::prelude::*;
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
    /// Notification kind the choice applies to; validated by the preference rules.
    kind: String,
    #[serde(default)]
    /// Channel the choice applies to (`in_app`, `webhook`, `chat` or `email`); absent means `in_app`, not every
    /// channel.
    channel: Option<String>,
    #[serde(default = "default_enabled")]
    /// Whether this kind is delivered; defaults to true when omitted.
    enabled: bool,
    #[serde(default)]
    /// Optional digest mode (for example `daily`); absent means deliver individually.
    digest: Option<String>,
}

/// Serde default for `enabled`: a preference that does not say otherwise leaves delivery on.
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
    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM notification_prefs WHERE sub = $1 AND kind = $2 AND channel = $3 LIMIT 1",
    )
    .bind(sub.as_str())
    .bind(payload.kind.as_str())
    .bind(channel.as_str())
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    let row: notification_prefs::Model =
        match existing {
            // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
            Some(id) => sqlx::query_as(
                "UPDATE notification_prefs SET enabled = $1, digest = $2 WHERE id = $3 RETURNING *",
            )
            .bind(payload.enabled)
            .bind(digest)
            .bind(id)
            .fetch_one(crate::db::pool(&ctx))
            .await?,
            None => {
                sqlx::query_as(
                    "INSERT INTO notification_prefs (pid, sub, kind, channel, enabled, digest)
                 VALUES ($1, $2, $3, $4, $5, $6)
                 RETURNING *",
                )
                .bind(Uuid::new_v4())
                .bind(sub)
                .bind(&payload.kind)
                .bind(channel)
                .bind(payload.enabled)
                .bind(digest)
                .fetch_one(crate::db::pool(&ctx))
                .await?
            }
        };
    format::json(row)
}

/// `GET /api/me/notification-prefs` — the caller's own choices, plus what
/// can be chosen and what applies when nothing is.
#[debug_handler]
async fn list_prefs(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    let sub = addressable(&claims.sub)?.to_string();
    let rows: Vec<notification_prefs::Model> =
        sqlx::query_as("SELECT * FROM notification_prefs WHERE sub = $1 ORDER BY kind, channel")
            .bind(sub)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
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
    /// Notification kind whose choice is reset.
    kind: String,
    #[serde(default)]
    /// Channel to reset; absent means `in_app`, not every channel.
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
    sqlx::query("DELETE FROM notification_prefs WHERE sub = $1 AND kind = $2 AND channel = $3")
        .bind(sub)
        .bind(params.kind)
        .bind(channel)
        .execute(crate::db::pool(&ctx))
        .await?;
    format::empty_json()
}

/// `POST /api/me/notification-prefs/reset` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ResetBody {
    /// Notification kind whose choice is reset.
    kind: String,
    #[serde(default)]
    /// Channel to reset; absent means `in_app`, not every channel.
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
    /// When true, only unread notifications; absent means all.
    unread: Option<bool>,
}

/// Rows read per request.
const INBOX_CAP: i64 = 500;

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
    let caller = crate::auth::MaybeAuthUser(Some(claims.clone()));
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM notifications WHERE recipient_ref = ANY(",
    );
    query.push_bind(refs).push(") AND deleted_at IS NULL");
    scope.notes_visible(&ctx, &mut query).await?;
    if params.unread.unwrap_or(false) {
        query.push(" AND read_at IS NULL");
    }
    query.push(" ORDER BY id DESC LIMIT ").push_bind(INBOX_CAP);
    let rows: Vec<notifications::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let daily: BTreeSet<String> = sqlx::query_scalar::<_, String>(
        "SELECT kind FROM notification_prefs WHERE sub = $1 AND channel = $2 AND digest = 'daily'",
    )
    .bind(user.to_string())
    .bind(rules::DEFAULT_CHANNEL)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .into_iter()
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

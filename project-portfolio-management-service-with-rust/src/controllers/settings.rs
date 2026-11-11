//! Deployment settings an administrator changes at run time. The first is the time zone that defines
//! "today" for portfolio-wide views and defaults ([`crate::tz`], doctrine 4). It is stored in
//! `deployment_settings`; each instance keeps it in memory and re-reads it every
//! `REFRESH_SECS` seconds, so a change made on one instance reaches the others without a restart.

use crate::prelude::*;
use axum::http::StatusCode;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use super::{refuse, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::models::audit_logs::Model as AuditModel;
use crate::tz;

/// The `deployment_settings` key holding the "today" zone.
const KEY: &str = "today_time_zone";

/// How often each instance re-reads the stored zone.
const REFRESH_SECS: u64 = 60;

/// The stored zone, who set it and when, or `None` if no administrator has set one.
async fn stored(
    ctx: &AppContext,
) -> std::result::Result<Option<(String, Option<String>, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT value, updated_by, updated_at::text AS updated_at FROM deployment_settings WHERE key = $1",
    )
    .bind(KEY)
    .fetch_optional(crate::db::pool(ctx))
    .await
}

/// Load the stored zone into memory: the stored one, or UTC when none is set. A stored value that
/// no longer parses (edited by hand) is logged and ignored, leaving the zone as it was.
///
/// # Errors
///
/// When the database cannot be read.
pub async fn refresh(ctx: &AppContext) -> std::result::Result<(), sqlx::Error> {
    match stored(ctx).await? {
        None => tz::set_today_zone(tz::Tz::UTC),
        Some((name, _, _)) => match tz::parse_zone(&name) {
            Ok(zone) => tz::set_today_zone(zone),
            Err(error) => {
                tracing::error!(%name, %error, "stored today_time_zone is not valid; keeping the current zone");
            }
        },
    }
    Ok(())
}

/// Load the zone now and keep it fresh. Called once at boot.
pub fn spawn(ctx: AppContext) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = refresh(&ctx).await {
                tracing::warn!(%error, "could not read the today time zone; keeping the current zone");
            }
            tokio::time::sleep(std::time::Duration::from_secs(REFRESH_SECS)).await;
        }
    });
}

/// Render the effective zone, whether it comes from an admin setting or the default, who last changed it and when, and the current instant and "today" in that zone.
async fn view(ctx: &AppContext) -> Result<Response> {
    let row = stored(ctx).await?;
    let zone = tz::today_zone();
    let now = Utc::now();
    format::json(serde_json::json!({
        "time_zone": zone.name(),
        "source": if row.is_some() { "admin" } else { "default" },
        "updated_by": row.as_ref().and_then(|r| r.1.clone()),
        "updated_at": row.as_ref().map(|r| r.2.clone()),
        "now_utc": now,
        "today": tz::deployment_today_at(now, zone),
    }))
}

/// `GET /api/settings/today-time-zone` — the zone that defines "today", where it came from, and the
/// date it gives right now.
#[debug_handler]
async fn get_zone(State(ctx): State<AppContext>) -> Result<Response> {
    view(&ctx).await
}

/// Request body for setting the zone that defines "today".
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ZoneBody {
    /// Zone name; parsed and rejected with a 422 if unknown.
    time_zone: String,
}

/// 403 unless the caller holds the administrator attribute; the zone changes what "today" means for every user.
fn require_admin(caller: &MaybeAuthUser) -> Result<()> {
    if super::plan_access::is_admin(caller.claims()) {
        Ok(())
    } else {
        Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "only an administrator can change the time zone that defines \"today\"",
        ))
    }
}

/// `PUT /api/settings/today-time-zone` — set the zone (an administrator). Audited with the before and
/// after.
#[debug_handler]
async fn put_zone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(body): Json<ZoneBody>,
) -> Result<Response> {
    require_admin(&caller)?;
    let zone = tz::parse_zone(&body.time_zone).map_err(|m| unprocessable(&m))?;
    let before = tz::today_zone();
    let actor = caller.actor().map(str::to_string);
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "INSERT INTO deployment_settings (key, value, updated_by, updated_at) VALUES ($1, $2, $3, now())
         ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_by = EXCLUDED.updated_by, updated_at = now()",
    )
    .bind(KEY)
    .bind(zone.name())
    .bind(actor.as_deref())
    .execute(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        Uuid::nil(),
        "today_time_zone_changed",
        actor.as_deref(),
        Some(serde_json::json!({ "before": before.name(), "after": zone.name() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    // Only once it is stored and audited does this instance use it.
    tz::set_today_zone(zone);
    view(&ctx).await
}

/// `DELETE /api/settings/today-time-zone` — go back to the default (UTC). An administrator; audited.
#[debug_handler]
async fn delete_zone(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    require_admin(&caller)?;
    let before = tz::today_zone();
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("DELETE FROM deployment_settings WHERE key = $1")
        .bind(KEY)
        .execute(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        Uuid::nil(),
        "today_time_zone_changed",
        caller.actor(),
        Some(serde_json::json!({ "before": before.name(), "after": tz::Tz::UTC.name() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    tz::set_today_zone(tz::Tz::UTC);
    view(&ctx).await
}

/// The settings routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/settings")
        .add("/today-time-zone", get(get_zone))
        .add("/today-time-zone", put(put_zone))
        .add("/today-time-zone", delete(delete_zone))
}

//! Deployment settings an administrator changes at run time. The first is the time zone that defines
//! "today" for portfolio-wide views and defaults ([`crate::tz`], doctrine 4). It is stored in
//! `deployment_settings`; each instance keeps it in memory and re-reads it every
//! [`REFRESH_SECS`] seconds, so a change made on one instance reaches the others without a restart.

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{DatabaseBackend, Statement};
use serde::Deserialize;
use uuid::Uuid;

use super::{db_err, refuse, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::models::audit_logs::Model as AuditModel;
use crate::tz;

/// The `deployment_settings` key holding the "today" zone.
const KEY: &str = "today_time_zone";

/// How often each instance re-reads the stored zone.
const REFRESH_SECS: u64 = 60;

/// The stored zone, who set it and when, or `None` if no administrator has set one.
async fn stored(
    db: &DatabaseConnection,
) -> std::result::Result<Option<(String, Option<String>, String)>, sea_orm::DbErr> {
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT value, updated_by, updated_at::text AS updated_at FROM deployment_settings WHERE key = $1",
            [KEY.into()],
        ))
        .await?;
    row.map(|r| {
        Ok((
            r.try_get::<String>("", "value")?,
            r.try_get::<Option<String>>("", "updated_by")?,
            r.try_get::<String>("", "updated_at")?,
        ))
    })
    .transpose()
}

/// Load the stored zone into memory: the stored one, or UTC when none is set. A stored value that
/// no longer parses (edited by hand) is logged and ignored, leaving the zone as it was.
///
/// # Errors
///
/// When the database cannot be read.
pub async fn refresh(db: &DatabaseConnection) -> std::result::Result<(), sea_orm::DbErr> {
    match stored(db).await? {
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
pub fn spawn(db: DatabaseConnection) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = refresh(&db).await {
                tracing::warn!(%error, "could not read the today time zone; keeping the current zone");
            }
            tokio::time::sleep(std::time::Duration::from_secs(REFRESH_SECS)).await;
        }
    });
}

async fn view(db: &DatabaseConnection) -> Result<Response> {
    let row = stored(db).await.map_err(db_err)?;
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
    view(&ctx.db).await
}

#[derive(Debug, Deserialize)]
struct ZoneBody {
    time_zone: String,
}

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
    ctx.db
        .execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "INSERT INTO deployment_settings (key, value, updated_by, updated_at) VALUES ($1, $2, $3, now())
             ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_by = EXCLUDED.updated_by, updated_at = now()",
            [KEY.into(), zone.name().into(), actor.clone().into()],
        ))
        .await
        .map_err(db_err)?;
    tz::set_today_zone(zone);
    AuditModel::record_or_log(
        &ctx.db,
        Uuid::nil(),
        "today_time_zone_changed",
        actor.as_deref(),
        Some(serde_json::json!({ "before": before.name(), "after": zone.name() })),
    )
    .await;
    view(&ctx.db).await
}

/// `DELETE /api/settings/today-time-zone` — go back to the default (UTC). An administrator; audited.
#[debug_handler]
async fn delete_zone(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    require_admin(&caller)?;
    let before = tz::today_zone();
    ctx.db
        .execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "DELETE FROM deployment_settings WHERE key = $1",
            [KEY.into()],
        ))
        .await
        .map_err(db_err)?;
    tz::set_today_zone(tz::Tz::UTC);
    AuditModel::record_or_log(
        &ctx.db,
        Uuid::nil(),
        "today_time_zone_changed",
        caller.actor(),
        Some(serde_json::json!({ "before": before.name(), "after": tz::Tz::UTC.name() })),
    )
    .await;
    view(&ctx.db).await
}

/// The settings routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/settings")
        .add("/today-time-zone", get(get_zone))
        .add("/today-time-zone", put(put_zone))
        .add("/today-time-zone", delete(delete_zone))
}

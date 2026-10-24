//! Notification delivery sinks (LT-6, RM-11): a person registers destinations
//! for **their own** notifications — a signed webhook, a chat webhook (Slack,
//! Mattermost, Discord, Telegram) or an email address. Pure rules and
//! renderers in [`crate::sinks`].
//!
//! **Never in the notification's path.** [`enqueue`] only writes a delivery
//! row; [`run_due`] (the ticker, an admin trigger, tests) sends it later with
//! bounded retries and growing backoff, so a slow or dead destination can
//! neither delay nor fail the notification. A failed delivery keeps its last
//! error and is given up on after [`crate::sinks::MAX_ATTEMPTS`].
//!
//! **Safe by construction.** A destination is validated on the way in (https to
//! a public host; never a private, link-local, metadata or internal address),
//! its name is resolved again at send time and refused if any answer is
//! private, redirects are not followed, and the secret is never returned in
//! full. A signed webhook carries the family's `X-Mxi-Signature` over the exact
//! body. A person's notification-channel preferences (`webhook` / `chat` /
//! `email`) are honoured per kind.

use super::{db_err, unprocessable};
use std::net::IpAddr;
use std::time::Duration;

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::models::_entities::{
    notification_prefs, notification_sinks, notifications, sink_deliveries,
};
use crate::notification_prefs::recipient_sub;
use crate::sinks::{self as rules, Note};

fn allow_loopback() -> bool {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_SINKS_ALLOW_LOOPBACK").is_ok()
}

/// Enqueue a delivery of `note` to each of its recipient's enabled sinks,
/// honouring their per-channel preferences. Best-effort and **never an
/// error**: the notification has already been written.
pub(crate) async fn enqueue<C: ConnectionTrait>(db: &C, note: &notifications::Model) {
    let Some(sub) = recipient_sub(&note.recipient_ref) else {
        return;
    };
    let Ok(sinks) = notification_sinks::Entity::find()
        .filter(notification_sinks::Column::Sub.eq(sub.as_str()))
        .filter(notification_sinks::Column::Enabled.eq(true))
        .filter(notification_sinks::Column::DeletedAt.is_null())
        .all(db)
        .await
    else {
        return;
    };
    for sink in sinks {
        let channel = rules::channel_of(&sink.kind);
        let opted_out = notification_prefs::Entity::find()
            .filter(notification_prefs::Column::Sub.eq(sub.as_str()))
            .filter(notification_prefs::Column::Kind.eq(note.kind.as_str()))
            .filter(notification_prefs::Column::Channel.eq(channel))
            .filter(notification_prefs::Column::Enabled.eq(false))
            .one(db)
            .await
            .ok()
            .flatten()
            .is_some();
        if opted_out {
            continue;
        }
        let _ = sink_deliveries::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            sink_pid: ActiveValue::set(sink.pid),
            notification_pid: ActiveValue::set(note.pid),
            status: ActiveValue::set("pending".to_string()),
            attempts: ActiveValue::set(0),
            next_attempt_at: ActiveValue::set(Some(Utc::now().into())),
            last_error: ActiveValue::set(None),
            delivered_at: ActiveValue::set(None),
            ..Default::default()
        }
        .insert(db)
        .await;
    }
}

enum Outcome {
    Sent,
    /// Try again later (carries the reason).
    Retry(String),
    /// Never going to work (carries the reason).
    Failed(String),
}

async fn send_http(sink: &notification_sinks::Model, note: &Note<'_>) -> Outcome {
    let loop_ok = allow_loopback();
    if !rules::destination_url_ok(&sink.destination, loop_ok) {
        return Outcome::Failed("the destination is no longer permitted".to_string());
    }
    // DNS, again, at send time: a name that now points inside is refused.
    if let Ok(url) = reqwest::Url::parse(&sink.destination)
        && let Some(host) = url.host_str()
        && host.trim_matches(['[', ']']).parse::<IpAddr>().is_err()
    {
        let port = url.port_or_known_default().unwrap_or(443);
        match tokio::net::lookup_host((host.to_string(), port)).await {
            Ok(addrs) => {
                let ips: Vec<IpAddr> = addrs.map(|a| a.ip()).collect();
                if !rules::resolved_ok(&ips, loop_ok) {
                    return Outcome::Failed(
                        "the destination resolves to a non-public address".to_string(),
                    );
                }
            }
            Err(e) => return Outcome::Retry(format!("dns: {e}")),
        }
    }
    let (body, ctype) = match rules::render(&sink.kind, note, &sink.settings) {
        Ok(b) => b,
        Err(e) => return Outcome::Failed(e),
    };
    let mut req = crate::webhooks::client()
        .post(&sink.destination)
        .timeout(Duration::from_secs(10))
        .header("content-type", ctype)
        .header(crate::webhooks::EVENT_ID_HEADER, note.pid.to_string());
    if sink.kind == "webhook"
        && let Some(sig) =
            crate::compliance::mac::tag(crate::compliance::mac::Domain::Webhook, body.as_bytes())
    {
        req = req.header(crate::webhooks::SIGNATURE_HEADER, sig);
    }
    match req.body(body).send().await {
        Ok(r) if r.status().is_success() => Outcome::Sent,
        Ok(r) if crate::webhooks::should_retry_status(r.status().as_u16()) => {
            Outcome::Retry(format!("HTTP {}", r.status()))
        }
        Ok(r) => Outcome::Failed(format!("HTTP {}", r.status())),
        Err(e) => Outcome::Retry(e.to_string()),
    }
}

async fn send_email(
    ctx: &AppContext,
    sink: &notification_sinks::Model,
    note: &Note<'_>,
    delivery: Uuid,
) -> Outcome {
    let (subject, text) = rules::render_email(note);
    // Through the sender in force, one row per recipient (email_deliveries); the sink keeps its own schedule.
    match super::email_deliveries::deliver(
        ctx,
        "sink",
        delivery,
        &sink.destination,
        &subject,
        &text,
    )
    .await
    {
        Ok(super::email_deliveries::Delivered::Sent) => Outcome::Sent,
        Ok(super::email_deliveries::Delivered::Retry(why)) => Outcome::Retry(why),
        Ok(super::email_deliveries::Delivered::Failed(why)) => Outcome::Failed(why),
        Err(e) => Outcome::Retry(e.to_string()),
    }
}

/// Deliveries attempted by one [`run_due`] pass, by outcome.
#[derive(Debug, Default, serde::Serialize)]
pub struct Dispatched {
    /// Delivered.
    pub sent: usize,
    /// Failed and will be retried.
    pub retrying: usize,
    /// Given up on (permanent failure or attempts exhausted).
    pub failed: usize,
    /// Skipped (the sink was removed or disabled, or the notification is gone).
    pub skipped: usize,
}

/// Send every delivery that is due (at most `limit`).
///
/// # Errors
///
/// Propagates database errors; a delivery's own failure is recorded on it.
pub async fn run_due(ctx: &AppContext, limit: u64) -> Result<Dispatched> {
    let now = Utc::now();
    let due = sink_deliveries::Entity::find()
        .filter(sink_deliveries::Column::Status.eq("pending"))
        .filter(sink_deliveries::Column::NextAttemptAt.lte(now))
        .order_by_asc(sink_deliveries::Column::Id)
        .limit(limit)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = Dispatched::default();
    for d in due {
        let sink = notification_sinks::Entity::find()
            .filter(notification_sinks::Column::Pid.eq(d.sink_pid))
            .filter(notification_sinks::Column::DeletedAt.is_null())
            .filter(notification_sinks::Column::Enabled.eq(true))
            .one(&ctx.db)
            .await
            .map_err(db_err)?;
        let note_row = notifications::Entity::find()
            .filter(notifications::Column::Pid.eq(d.notification_pid))
            .one(&ctx.db)
            .await
            .map_err(db_err)?;
        let (Some(sink), Some(n)) = (sink, note_row) else {
            mark(ctx, d, "skipped", None, None, None).await?;
            out.skipped += 1;
            continue;
        };
        let note = Note {
            pid: n.pid,
            kind: &n.kind,
            subject_kind: &n.subject_kind,
            subject_pid: n.subject_pid,
            message: &n.message,
            created_at: n.created_at.to_utc(),
        };
        let outcome = if sink.kind == "email" {
            send_email(ctx, &sink, &note, d.pid).await
        } else {
            send_http(&sink, &note).await
        };
        let attempts = d.attempts + 1;
        match outcome {
            Outcome::Sent => {
                mark(ctx, d, "sent", Some(attempts), None, None).await?;
                out.sent += 1;
            }
            Outcome::Retry(why) => {
                if let Some(when) = rules::next_attempt(Utc::now(), attempts) {
                    mark(ctx, d, "pending", Some(attempts), Some(why), Some(when)).await?;
                    out.retrying += 1;
                } else {
                    mark(ctx, d, "failed", Some(attempts), Some(why), None).await?;
                    out.failed += 1;
                }
            }
            Outcome::Failed(why) => {
                mark(ctx, d, "failed", Some(attempts), Some(why), None).await?;
                out.failed += 1;
            }
        }
    }
    Ok(out)
}

async fn mark(
    ctx: &AppContext,
    d: sink_deliveries::Model,
    status: &str,
    attempts: Option<i32>,
    error: Option<String>,
    next: Option<chrono::DateTime<Utc>>,
) -> Result<()> {
    let mut a: sink_deliveries::ActiveModel = d.into();
    a.status = ActiveValue::set(status.to_string());
    if let Some(n) = attempts {
        a.attempts = ActiveValue::set(n);
    }
    a.last_error = ActiveValue::set(error);
    a.next_attempt_at = ActiveValue::set(next.map(Into::into));
    if status == "sent" {
        a.delivered_at = ActiveValue::set(Some(Utc::now().into()));
    }
    a.updated_at = ActiveValue::set(Utc::now().into());
    a.update(&ctx.db).await.map_err(db_err)?;
    Ok(())
}

/// Start the dispatcher ticker: every few seconds it sends what is due.
/// Env-gated off with `PROJECT_PORTFOLIO_MANAGEMENT_SINKS_TICK_SECS=0`.
pub fn spawn(ctx: &AppContext) {
    let secs = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_SINKS_TICK_SECS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(5);
    if secs == 0 {
        return;
    }
    let ctx = ctx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(secs)).await;
            if let Err(e) = run_due(&ctx, 100).await {
                tracing::warn!(error = %e, "sink dispatch pass failed");
            }
            if let Err(e) = super::email_deliveries::run_due(&ctx, 100).await {
                tracing::warn!(error = %e, "email retry pass failed");
            }
        }
    });
}

// ───────────────────────────── endpoints ─────────────────────────────

/// `POST /api/me/notification-sinks` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SinkBody {
    kind: String,
    destination: String,
    #[serde(default)]
    settings: Option<serde_json::Value>,
}

fn view(s: &notification_sinks::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": s.pid, "kind": s.kind, "enabled": s.enabled,
        "channel": rules::channel_of(&s.kind),
        // The destination is a secret: only a masked form is ever returned.
        "destination": rules::mask_destination(&s.kind, &s.destination),
    })
}

fn user_id(sub: &str) -> Result<String> {
    Uuid::parse_str(sub).map(|u| u.to_string()).map_err(|_| {
        unprocessable(
            "sub_not_addressable: this identity cannot receive person-addressed notifications",
        )
    })
}

/// `POST /api/me/notification-sinks` — register a destination.
#[debug_handler]
async fn add_sink(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<SinkBody>,
) -> Result<Response> {
    let sub = user_id(&claims.sub)?;
    let settings = b.settings.clone().unwrap_or_else(|| serde_json::json!({}));
    let problems = rules::problems(&b.kind, &b.destination, &settings, allow_loopback());
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let held = notification_sinks::Entity::find()
        .filter(notification_sinks::Column::Sub.eq(sub.as_str()))
        .filter(notification_sinks::Column::DeletedAt.is_null())
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    if held >= rules::MAX_PER_PERSON as u64 {
        return Err(unprocessable(&format!(
            "at most {} sinks per person",
            rules::MAX_PER_PERSON
        )));
    }
    let row = notification_sinks::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        sub: ActiveValue::set(sub),
        kind: ActiveValue::set(b.kind.clone()),
        destination: ActiveValue::set(b.destination.trim().to_string()),
        settings: ActiveValue::set(settings),
        enabled: ActiveValue::set(true),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    format::json(view(&row))
}

/// `GET /api/me/notification-sinks` — the caller's own sinks, masked.
#[debug_handler]
async fn list_sinks(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    let sub = user_id(&claims.sub)?;
    let rows = notification_sinks::Entity::find()
        .filter(notification_sinks::Column::Sub.eq(sub))
        .filter(notification_sinks::Column::DeletedAt.is_null())
        .order_by_asc(notification_sinks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(view).collect::<Vec<_>>())
}

async fn own_sink(ctx: &AppContext, sub: &str, pid: &str) -> Result<notification_sinks::Model> {
    let pid = super::parse_pid(pid)?;
    notification_sinks::Entity::find()
        .filter(notification_sinks::Column::Pid.eq(pid))
        .filter(notification_sinks::Column::Sub.eq(sub))
        .filter(notification_sinks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `POST /api/me/notification-sinks/{pid}/delete` (and `DELETE`) — remove one
/// of the caller's own sinks. Another person's is `404`, identically.
#[debug_handler]
async fn delete_sink(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let sub = user_id(&claims.sub)?;
    let row = own_sink(&ctx, &sub, &pid).await?;
    let mut a: notification_sinks::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    format::empty_json()
}

/// `GET /api/me/notification-sinks/{pid}/deliveries` — the recent deliveries of
/// one of the caller's own sinks, with attempts and the last error.
#[debug_handler]
async fn sink_log(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let sub = user_id(&claims.sub)?;
    let sink = own_sink(&ctx, &sub, &pid).await?;
    let rows = sink_deliveries::Entity::find()
        .filter(sink_deliveries::Column::SinkPid.eq(sink.pid))
        .order_by_desc(sink_deliveries::Column::Id)
        .limit(50)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(
        rows.iter()
            .map(|d| {
                serde_json::json!({ "notification_pid": d.notification_pid, "status": d.status,
                    "attempts": d.attempts, "last_error": d.last_error,
                    "next_attempt_at": d.next_attempt_at, "delivered_at": d.delivered_at })
            })
            .collect::<Vec<_>>(),
    )
}

/// `POST /api/notification-sinks/dispatch` — send what is due now. Admin only
/// (the ticker does this on its own; this is for operators and tests).
#[debug_handler]
async fn dispatch(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    if !claims
        .attrs
        .get("access")
        .is_some_and(|v| v.iter().any(|a| a == "admin"))
    {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new("forbidden", "dispatching needs the admin attribute"),
        ));
    }
    format::json(run_due(&ctx, 500).await?)
}

/// The sink routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/me/notification-sinks", post(add_sink))
        .add("/me/notification-sinks", get(list_sinks))
        .add("/me/notification-sinks/{pid}", delete(delete_sink))
        .add("/me/notification-sinks/{pid}/delete", post(delete_sink))
        .add("/me/notification-sinks/{pid}/deliveries", get(sink_log))
        .add("/notification-sinks/dispatch", post(dispatch))
}

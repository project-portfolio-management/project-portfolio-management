//! Feeds (LT-7 iCalendar, RM-6 Atom): read one person's own work and inbox
//! in a calendar app or feed reader. Pure builders in [`crate::feeds`].
//!
//! A calendar app cannot send a bearer header, so a feed authenticates with a
//! **per-user, revocable token** in the query string (`/api/feeds/` is
//! therefore exempt from the blanket guard). A token is a secret of one
//! person: it reads **their own** items only — the tasks assigned to them,
//! the milestones and sprints of the plans those tasks sit in, and their own
//! notifications. Only its SHA-256 is stored; revoking takes effect on the
//! next fetch.
//!
//! The feed scope is deliberately *"your own work"* rather than *"everything
//! you may read"*: a feed fetch carries no claims, so record-level ABAC
//! cannot be evaluated for it, and widening it would leak. Nothing a feed
//! returns is anything the token's owner is not already assigned to.

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::feeds::{self as rules, Entry, Event};
use crate::link_token;
use crate::models::_entities::{
    feed_tokens, milestones, notifications, planned_slots, sprints, tasks,
};

/// 401 returned for any bad feed token (unknown, revoked or expired) with one message, so callers cannot tell which case applied.
fn unauthorized() -> Error {
    Error::CustomError(
        StatusCode::UNAUTHORIZED,
        ErrorDetail::new("unauthorized", "unknown, revoked or expired feed token"),
    )
}

/// `POST /api/me/feed-token?label=` query.
#[derive(Debug, Deserialize, Default, schemars::JsonSchema)]
pub(crate) struct IssueParams {
    /// Optional human label stored with the token so its owner can recognise it.
    #[serde(default)]
    label: Option<String>,
}

/// `POST /api/me/feed-token` — mint a feed token. **Shown once**: only its
/// hash is kept, so it cannot be shown again.
#[debug_handler]
async fn issue(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(params): Query<IssueParams>,
) -> Result<Response> {
    let label = params
        .label
        .map(|l| l.chars().take(100).collect::<String>());
    let token = rules::new_token();
    // One live token per person: issuing a new one revokes the earlier ones, so a leaked old URL
    // stops working on rotation.
    let mut tx = crate::db::begin(&ctx).await?;
    revoke_live(&mut tx, &claims.sub).await?;
    sqlx::query(
        "INSERT INTO feed_tokens (pid, sub, token_hash, label, revoked_at, last_used_at)
         VALUES ($1, $2, $3, $4, NULL, NULL)",
    )
    .bind(Uuid::new_v4())
    .bind(&claims.sub)
    .bind(rules::hash_token(&token))
    .bind(label)
    .execute(tx.conn())
    .await?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "token": token,
        "calendar_url": format!("/api/feeds/calendar.ics?token={token}"),
        "atom_url": format!("/api/feeds/me.atom?token={token}"),
        "note": "This token is shown once; only its hash is stored. It expires after 365 days and replaces any earlier token. Revoke with DELETE /api/me/feed-token.",
    }))
}

/// Revoke every live token of `sub`.
async fn revoke_live(db: &mut impl crate::db::Session, sub: &str) -> Result<()> {
    sqlx::query("UPDATE feed_tokens SET revoked_at = $1 WHERE sub = $2 AND revoked_at IS NULL")
        .bind(chrono::Utc::now())
        .bind(sub)
        .execute(db.ex())
        .await?;
    Ok(())
}

/// `DELETE /api/me/feed-token` — revoke **all** of the caller's tokens.
#[debug_handler]
async fn revoke(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    revoke_live(&mut crate::db::pool(&ctx), &claims.sub).await?;
    format::empty_json()
}

/// `?token=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TokenParams {
    /// The feed token presented in the query string; only its hash is compared with the stored one.
    token: String,
}

/// Resolve a presented token to its owner's `sub`, or `401`.
async fn owner_of(ctx: &AppContext, token: &str) -> Result<String> {
    if !rules::token_shaped(token) {
        return Err(unauthorized());
    }
    let row: feed_tokens::Model = sqlx::query_as(
        "SELECT * FROM feed_tokens WHERE token_hash = $1 AND revoked_at IS NULL LIMIT 1",
    )
    .bind(rules::hash_token(token))
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or_else(unauthorized)?;
    let link = link_token::Link {
        issued_at: row.created_at.with_timezone(&chrono::Utc),
        lifetime_days: link_token::FEED_TOKEN_LIFETIME_DAYS,
        revoked_at: None,
        redeemed_at: None,
        single_use: false,
    };
    link_token::check(&link, chrono::Utc::now()).map_err(|_| unauthorized())?;
    // Best-effort bookkeeping; never fails the fetch.
    sqlx::query("UPDATE feed_tokens SET last_used_at = $1 WHERE id = $2")
        .bind(chrono::Utc::now())
        .bind(row.id)
        .execute(crate::db::pool(ctx))
        .await
        .ok();
    Ok(row.sub)
}

/// The person and worker references a subject id can appear as in assignments; empty when the subject is not a UUID, which yields an empty feed rather than someone else's tasks.
fn refs_of(sub: &str) -> Vec<String> {
    Uuid::parse_str(sub)
        .map(|u| vec![format!("person:{u}"), format!("worker:{u}")])
        .unwrap_or_default()
}

/// The plans among `plans` hidden from a feed's owner. A feed URL carries no claims, so the owner is judged with none: not an
/// administrator and cleared for no handling mark, so a marked plan stays out of a calendar that may be shared.
async fn hidden_from_owner(
    ctx: &AppContext,
    sub: &str,
    plans: &[Uuid],
) -> std::collections::HashSet<Uuid> {
    super::plan_access::hidden_plans(ctx, plans, &feed_owner(sub)).await
}

/// A feed's owner as a caller with no claims beyond their identity.
fn feed_owner(sub: &str) -> crate::auth::MaybeAuthUser {
    crate::auth::MaybeAuthUser(
        serde_json::from_value(serde_json::json!({
            "sub": sub, "email": "", "name": "", "iss": "", "aud": "", "exp": 0, "iat": 0, "sid": "", "attrs": {},
        }))
        .ok(),
    )
}

/// `GET /api/feeds/calendar.ics?token=` — the owner's due tasks, plus the
/// milestones and sprint dates of the plans those tasks sit in, as all-day
/// events with stable UIDs.
#[debug_handler]
#[expect(clippy::too_many_lines)] // one linear pipeline: tasks, plans, milestones, sprints, slots
async fn calendar(
    State(ctx): State<AppContext>,
    Query(p): Query<TokenParams>,
) -> Result<axum::response::Response> {
    let sub = owner_of(&ctx, &p.token).await?;
    let refs = refs_of(&sub);
    let mine = if refs.is_empty() {
        Vec::new()
    } else {
        sqlx::query_as::<_, tasks::Model>(
            "SELECT * FROM tasks
             WHERE assignee_ref = ANY($1) AND deleted_at IS NULL AND done_at IS NULL
             ORDER BY id
             LIMIT 2000",
        )
        .bind(refs)
        .fetch_all(crate::db::pool(&ctx))
        .await?
    };
    let mut plan_pids: Vec<Uuid> = mine.iter().map(|t| t.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let hidden = hidden_from_owner(&ctx, &sub, &plan_pids).await;
    plan_pids.retain(|p| !hidden.contains(p));
    let mine: Vec<_> = mine
        .into_iter()
        .filter(|t| !hidden.contains(&t.plan_pid))
        .collect();
    let plan_names: std::collections::HashMap<Uuid, String> = if plan_pids.is_empty() {
        std::collections::HashMap::new()
    } else {
        sqlx::query_as::<_, (Uuid, String)>(
            "SELECT pid, name FROM plans WHERE pid = ANY($1) AND deleted_at IS NULL",
        )
        .bind(&plan_pids)
        .fetch_all(crate::db::pool(&ctx))
        .await?
        .into_iter()
        .collect()
    };
    let live: Vec<Uuid> = plan_names.keys().copied().collect();
    let mut events: Vec<Event> = Vec::new();
    for t in &mine {
        let (Some(due), Some(plan)) = (t.due_date, plan_names.get(&t.plan_pid)) else {
            continue;
        };
        events.push(Event {
            uid: format!("task-{}@ppm", t.pid),
            date: due,
            summary: t.title.clone(),
            description: Some(format!("Task in {plan}")),
            span: None,
        });
    }
    if !live.is_empty() {
        let found: Vec<milestones::Model> = sqlx::query_as(
            "SELECT * FROM milestones
             WHERE plan_pid = ANY($1) AND deleted_at IS NULL AND done = false
             ORDER BY id",
        )
        .bind(&live)
        .fetch_all(crate::db::pool(&ctx))
        .await?;
        for m in found {
            events.push(Event {
                uid: format!("milestone-{}@ppm", m.pid),
                date: m.due,
                summary: format!("Milestone: {}", m.name),
                description: plan_names
                    .get(&m.plan_pid)
                    .map(|p| format!("Milestone of {p}")),
                span: None,
            });
        }
        let found: Vec<sprints::Model> = sqlx::query_as(
            "SELECT * FROM sprints WHERE plan_pid = ANY($1) AND deleted_at IS NULL ORDER BY id",
        )
        .bind(&live)
        .fetch_all(crate::db::pool(&ctx))
        .await?;
        for s in found {
            events.push(Event {
                uid: format!("sprint-start-{}@ppm", s.pid),
                date: s.starts_on,
                summary: format!("Sprint starts: {}", s.name),
                description: None,
                span: None,
            });
            events.push(Event {
                uid: format!("sprint-end-{}@ppm", s.pid),
                date: s.ends_on,
                summary: format!("Sprint ends: {}", s.name),
                description: None,
                span: None,
            });
        }
    }
    // PL-4: the owner's own planned slots, as timed events (UTC). Slots on a
    // task that is gone are skipped; recent and future ones only.
    let cutoff = chrono::Utc::now() - chrono::Duration::days(7);
    let slots: Vec<planned_slots::Model> = sqlx::query_as(
        "SELECT * FROM planned_slots
         WHERE sub = $1 AND deleted_at IS NULL AND ends_at > $2
         ORDER BY starts_at
         LIMIT 500",
    )
    .bind(sub.as_str())
    .bind(cutoff)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    for s in slots {
        let Some(title) = sqlx::query_scalar::<_, String>(
            "SELECT title FROM tasks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(s.task_pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await?
        else {
            continue;
        };
        let (a, b) = (s.starts_at.to_utc(), s.ends_at.to_utc());
        events.push(Event {
            uid: format!("slot-{}@ppm", s.pid),
            date: a.date_naive(),
            summary: format!("Focus: {title}"),
            description: None,
            span: Some((a, b)),
        });
    }
    events.sort_by(|a, b| a.date.cmp(&b.date).then(a.uid.cmp(&b.uid)));
    let body = rules::ical("My work", chrono::Utc::now(), &events);
    Ok((
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/calendar; charset=utf-8"),
        )],
        body,
    )
        .into_response())
}

/// `GET /api/feeds/me.atom?token=` — the owner's notifications (newest 100)
/// as an Atom feed. `If-None-Match` returns `304` when nothing changed.
#[debug_handler]
async fn atom(
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    Query(p): Query<TokenParams>,
) -> Result<axum::response::Response> {
    let sub = owner_of(&ctx, &p.token).await?;
    let refs = refs_of(&sub);
    let scope = super::plan_access::HiddenScope::of(&ctx, &feed_owner(&sub)).await?;
    let rows = if refs.is_empty() {
        Vec::new()
    } else {
        let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
            "SELECT * FROM notifications WHERE recipient_ref = ANY(",
        );
        query.push_bind(refs).push(") AND deleted_at IS NULL");
        scope.notes_visible(&ctx, &mut query).await?;
        query.push(" ORDER BY id DESC LIMIT 100");
        query
            .build_query_as::<notifications::Model>()
            .fetch_all(crate::db::pool(&ctx))
            .await?
    };
    let entries: Vec<Entry> = rows
        .iter()
        .map(|n| Entry {
            id: n.pid,
            title: n.message.clone(),
            updated: n.created_at.to_utc(),
            summary: format!("{} · {}", n.kind, n.subject_kind),
        })
        .collect();
    // A stable feed id per owner, derived from the sub, so readers keep one subscription.
    let digest = rules::hash_token(&format!("ppm:feed:{sub}"));
    let feed_id = Uuid::parse_str(&digest[..32]).unwrap_or_else(|_| Uuid::nil());
    let body = rules::atom(feed_id, "Notifications", &entries);
    let etag = format!("\"{}\"", &rules::hash_token(&body)[..32]);
    if headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == etag)
    {
        return Ok((StatusCode::NOT_MODIFIED, [(header::ETAG, etag)]).into_response());
    }
    Ok((
        [
            (
                header::CONTENT_TYPE,
                "application/atom+xml; charset=utf-8".to_string(),
            ),
            (header::ETAG, etag),
        ],
        body,
    )
        .into_response())
}

/// The feed routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/me/feed-token", post(issue))
        .add("/me/feed-token", delete(revoke))
        // A write-level user (no delete right under the default policy) must
        // still be able to revoke their own feed: the POST twin.
        .add("/me/feed-token/revoke", post(revoke))
        .add("/feeds/calendar.ics", get(calendar))
        .add("/feeds/me.atom", get(atom))
}

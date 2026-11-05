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

use super::db_err;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::IntoResponse;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::feeds::{self as rules, Entry, Event};
use crate::link_token;
use crate::models::_entities::{
    feed_tokens, milestones, notifications, planned_slots, plans, sprints, tasks,
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    revoke_live(&txn, &claims.sub).await?;
    feed_tokens::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        sub: ActiveValue::set(claims.sub.clone()),
        token_hash: ActiveValue::set(rules::hash_token(&token)),
        label: ActiveValue::set(label),
        revoked_at: ActiveValue::set(None),
        last_used_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    txn.commit().await.map_err(db_err)?;
    format::json(serde_json::json!({
        "token": token,
        "calendar_url": format!("/api/feeds/calendar.ics?token={token}"),
        "atom_url": format!("/api/feeds/me.atom?token={token}"),
        "note": "This token is shown once; only its hash is stored. It expires after 365 days and replaces any earlier token. Revoke with DELETE /api/me/feed-token.",
    }))
}

/// Revoke every live token of `sub`.
async fn revoke_live(db: &impl ConnectionTrait, sub: &str) -> Result<()> {
    feed_tokens::Entity::update_many()
        .col_expr(
            feed_tokens::Column::RevokedAt,
            sea_orm::sea_query::Expr::value(chrono::Utc::now()),
        )
        .filter(feed_tokens::Column::Sub.eq(sub))
        .filter(feed_tokens::Column::RevokedAt.is_null())
        .exec(db)
        .await
        .map_err(db_err)?;
    Ok(())
}

/// `DELETE /api/me/feed-token` — revoke **all** of the caller's tokens.
#[debug_handler]
async fn revoke(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    revoke_live(&ctx.db, &claims.sub).await?;
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
    let row = feed_tokens::Entity::find()
        .filter(feed_tokens::Column::TokenHash.eq(rules::hash_token(token)))
        .filter(feed_tokens::Column::RevokedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
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
    let sub = row.sub.clone();
    let mut active: feed_tokens::ActiveModel = row.into();
    active.last_used_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&ctx.db).await.ok();
    Ok(sub)
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
        tasks::Entity::find()
            .filter(tasks::Column::AssigneeRef.is_in(refs))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::DoneAt.is_null())
            .order_by_asc(tasks::Column::Id)
            .limit(2_000)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
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
        plans::Entity::find()
            .filter(plans::Column::Pid.is_in(plan_pids.clone()))
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|p| (p.pid, p.name))
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
        for m in milestones::Entity::find()
            .filter(milestones::Column::PlanPid.is_in(live.clone()))
            .filter(milestones::Column::DeletedAt.is_null())
            .filter(milestones::Column::Done.eq(false))
            .order_by_asc(milestones::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
        {
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
        for s in sprints::Entity::find()
            .filter(sprints::Column::PlanPid.is_in(live))
            .filter(sprints::Column::DeletedAt.is_null())
            .order_by_asc(sprints::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
        {
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
    for s in planned_slots::Entity::find()
        .filter(planned_slots::Column::Sub.eq(sub.as_str()))
        .filter(planned_slots::Column::DeletedAt.is_null())
        .filter(planned_slots::Column::EndsAt.gt(cutoff))
        .order_by_asc(planned_slots::Column::StartsAt)
        .limit(500)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        let Some(t) = tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(s.task_pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
        else {
            continue;
        };
        let (a, b) = (s.starts_at.to_utc(), s.ends_at.to_utc());
        events.push(Event {
            uid: format!("slot-{}@ppm", s.pid),
            date: a.date_naive(),
            summary: format!("Focus: {}", t.title),
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
        notifications::Entity::find()
            .filter(notifications::Column::RecipientRef.is_in(refs))
            .filter(notifications::Column::DeletedAt.is_null())
            .filter(scope.notes_visible(&ctx).await?)
            .order_by_desc(notifications::Column::Id)
            .limit(100)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
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

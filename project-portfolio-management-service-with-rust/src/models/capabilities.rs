//! Model glue for the collaboration / automation capability tables:
//! the finders and the small set of
//! writes shared by more than one controller (the notification insert
//! is written by the automation engine *and* by the scheduled-action
//! sweep, so it lives here rather than being duplicated).

use crate::prelude::*;
use uuid::Uuid;

use super::_entities::{automation_runs, automations, notifications, reviews, scheduled_actions};

/// Find the live automation by public id, on the pool.
///
/// # Errors
///
/// [`Error::NotFound`] when absent or soft-deleted.
pub async fn find_automation(ctx: &AppContext, pid: Uuid) -> Result<automations::Model> {
    sqlx::query_as("SELECT * FROM automations WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Find the live scheduled action by public id, on the pool.
///
/// # Errors
///
/// [`Error::NotFound`] when absent or soft-deleted.
pub async fn find_scheduled_action(
    ctx: &AppContext,
    pid: Uuid,
) -> Result<scheduled_actions::Model> {
    sqlx::query_as("SELECT * FROM scheduled_actions WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Find the live review by public id, on the pool.
///
/// # Errors
///
/// [`Error::NotFound`] when absent or soft-deleted.
pub async fn find_review(ctx: &AppContext, pid: Uuid) -> Result<reviews::Model> {
    sqlx::query_as("SELECT * FROM reviews WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Find the live notification by public id, on the pool.
///
/// # Errors
///
/// [`Error::NotFound`] when absent or soft-deleted.
pub async fn find_notification(ctx: &AppContext, pid: Uuid) -> Result<notifications::Model> {
    sqlx::query_as("SELECT * FROM notifications WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Every live review invitation on one subject, oldest first, on the pool.
///
/// # Errors
///
/// Propagates database errors.
pub async fn reviews_for_subject(
    ctx: &AppContext,
    subject_kind: &str,
    subject_pid: Uuid,
) -> Result<Vec<reviews::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM reviews
         WHERE subject_kind = $1 AND subject_pid = $2 AND deleted_at IS NULL
         ORDER BY id",
    )
    .bind(subject_kind)
    .bind(subject_pid)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Every enabled automation whose trigger kind matches, cheapest-first
/// filter before the pure engine decides what actually fires.
///
/// # Errors
///
/// Propagates database errors.
pub async fn enabled_automations(
    ctx: &AppContext,
    trigger_kind: &str,
) -> Result<Vec<automations::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM automations
         WHERE trigger_kind = $1 AND enabled = true AND deleted_at IS NULL
         ORDER BY id",
    )
    .bind(trigger_kind)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Write one in-app notification, unless the recipient has switched that
/// kind off (PL-5). In-app only — this service has no email or push
/// transport, and does not pretend to. On `sqlx`: in a transaction ([`crate::db::Tx`]) or on the pool
/// ([`crate::db::pool`]), so a handler writes its notification beside its other writes.
///
/// Returns `Ok(None)` when the recipient opted out: **suppression is a
/// result, not an error**, so a caller can report it ("skipped") rather
/// than log a failure. A recipient with no preference row — including every
/// `organization:` reference — gets today's behaviour.
///
/// # Errors
///
/// Propagates database errors.
pub async fn notify_on(
    db: &mut impl crate::db::Session,
    recipient_ref: &str,
    subject_kind: &str,
    subject_pid: Uuid,
    kind: &str,
    message: &str,
) -> Result<Option<notifications::Model>> {
    if let Some(sub) = crate::notification_prefs::recipient_sub(recipient_ref) {
        let opted_out = sqlx::query(
            "SELECT 1 FROM notification_prefs
             WHERE sub = $1 AND kind = $2 AND channel = $3 AND enabled = false
             LIMIT 1",
        )
        .bind(sub)
        .bind(kind)
        .bind(crate::notification_prefs::DEFAULT_CHANNEL)
        .fetch_optional(db.ex())
        .await?
        .is_some();
        if opted_out {
            return Ok(None);
        }
    }
    let row: notifications::Model = sqlx::query_as(
        "INSERT INTO notifications
             (pid, recipient_ref, subject_kind, subject_pid, kind, message, read_at, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(recipient_ref)
    .bind(subject_kind)
    .bind(subject_pid)
    .bind(kind)
    .bind(message)
    .fetch_one(db.ex())
    .await?;
    crate::controllers::sinks::enqueue_on(db, &row).await;
    Ok(Some(row))
}

/// Record what one **action** of one automation firing did.
/// `action_index` (0-based) names which element of the rule's `actions`
/// array this row reports on (FR-32: "each action's outcome logged
/// separately") — a firing of an N-action rule calls this N times, one
/// row per action, never one row overwritten by the next action's
/// outcome.
///
/// # Errors
///
/// Propagates database errors.
pub async fn record_run_on(
    db: &mut impl crate::db::Session,
    automation_pid: Uuid,
    subject_kind: &str,
    subject_pid: Uuid,
    action_index: i32,
    outcome: &str,
    detail: serde_json::Value,
) -> Result<automation_runs::Model> {
    Ok(sqlx::query_as(
        "INSERT INTO automation_runs
             (pid, automation_pid, subject_kind, subject_pid, action_index, outcome, detail)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(automation_pid)
    .bind(subject_kind)
    .bind(subject_pid)
    .bind(action_index)
    .bind(outcome)
    .bind(detail)
    .fetch_one(db.ex())
    .await?)
}

/// One deadline to hold until it comes due.
#[derive(Debug, Clone)]
pub struct NewScheduledAction {
    /// What the action is about (`idea` / `proposal` / `plan` / `task`).
    pub subject_kind: String,
    /// The subject's pid.
    pub subject_pid: Uuid,
    /// One of `crate::automation::SCHEDULED_ACTION_KINDS`.
    pub action_kind: String,
    /// Action-specific detail (recipient, message, …).
    pub payload: serde_json::Value,
    /// When it comes due.
    pub due_at: chrono::DateTime<chrono::FixedOffset>,
    /// The automation that scheduled it, when it was not a person.
    pub source_automation_pid: Option<Uuid>,
    /// The actor who configured it.
    pub created_by: Option<String>,
}

/// Enqueue a scheduled action ("set and forget"): held until `due_at`,
/// then fired exactly once by the sweep.
///
/// # Errors
///
/// Propagates database errors.
pub async fn schedule_on(
    db: &mut impl crate::db::Session,
    new: NewScheduledAction,
) -> Result<scheduled_actions::Model> {
    let NewScheduledAction {
        subject_kind,
        subject_pid,
        action_kind,
        payload,
        due_at,
        source_automation_pid,
        created_by,
    } = new;
    Ok(sqlx::query_as(
        "INSERT INTO scheduled_actions
             (pid, subject_kind, subject_pid, action_kind, payload, due_at, status,
              source_automation_pid, created_by, fired_at, outcome, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'pending', $7, $8, NULL, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(subject_kind)
    .bind(subject_pid)
    .bind(action_kind)
    .bind(payload)
    .bind(due_at)
    .bind(source_automation_pid)
    .bind(created_by)
    .fetch_one(db.ex())
    .await?)
}

/// The pending scheduled actions due at or before `now`, oldest
/// deadline first, capped so one sweep can never run unbounded.
///
/// # Errors
///
/// Propagates database errors.
pub async fn due_actions(
    ctx: &AppContext,
    now: chrono::DateTime<chrono::FixedOffset>,
    cap: u64,
) -> Result<Vec<scheduled_actions::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM scheduled_actions
         WHERE status = 'pending' AND due_at <= $1 AND deleted_at IS NULL
         ORDER BY due_at
         LIMIT $2",
    )
    .bind(now)
    .bind(i64::try_from(cap).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

//! Milestones: create, list, reschedule, complete.

use super::super::unprocessable;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::milestones;
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::prelude::*;
use crate::validation::MAX_TEXT_LEN;

/// `POST /api/plans/{pid}/milestones` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MilestonePayload {
    /// Milestone name; required and capped at `MAX_TEXT_LEN`.
    name: String,
    /// Due date.
    due: chrono::NaiveDate,
    /// Optional kind (`milestone` / `demo` / `release` / `checkpoint`);
    /// absent reads as `milestone`.
    #[serde(default)]
    kind: Option<String>,
}

/// `POST /api/plans/{pid}/milestones`. Adds a not-yet-done milestone to a live plan after validating name and kind, and audits `milestone_added` in the same transaction.
#[debug_handler]
pub(super) async fn create_milestone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<MilestonePayload>,
) -> Result<Response> {
    if payload.name.trim().is_empty() || payload.name.len() > MAX_TEXT_LEN {
        return Err(unprocessable("name is required (and capped)"));
    }
    if let Some(kind) = payload.kind.as_deref()
        && !crate::engineering::MILESTONE_KINDS.contains(&kind)
    {
        return Err(unprocessable(&format!(
            "kind must be one of {:?}",
            crate::engineering::MILESTONE_KINDS
        )));
    }
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let row_pid = Uuid::new_v4();
    let mut tx = crate::db::begin(&ctx).await?;
    // The columns `SeaORM`'s insert set, no more (`completion_mode`, `done_at` and `done_reason` are the table's defaults).
    sqlx::query(
        "INSERT INTO milestones (pid, plan_pid, name, due, kind, done, deleted_at)
         VALUES ($1, $2, $3, $4, $5, FALSE, NULL)",
    )
    .bind(row_pid)
    .bind(item.pid)
    .bind(&payload.name)
    .bind(payload.due)
    .bind(&payload.kind)
    .execute(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row_pid, "milestone_added", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/plans/{pid}/milestones` — due-date order, with
/// overdue flags.
#[debug_handler]
pub(super) async fn list_milestones(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let today = crate::tz::deployment_today();
    let rows = vis_models::milestones_for(&ctx, item.pid).await?;
    let views: Vec<_> = rows
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "pid": m.pid.to_string(), "name": m.name, "due": m.due,
                "kind": m.kind.as_deref().unwrap_or("milestone"),
                "done": m.done, "overdue": !m.done && m.due < today,
            })
        })
        .collect();
    format::json(views)
}

/// `PUT /api/plans/{pid}/milestones/{m_pid}` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MilestoneReschedulePayload {
    /// New due date; the `milestone_due_changed` trigger fires only when it differs from the current one.
    due: chrono::NaiveDate,
    /// Why it moved; kept in the slip history (optional, up to 500 characters).
    #[serde(default)]
    reason: Option<String>,
}

/// `PUT /api/plans/{pid}/milestones/{m_pid}` — reschedule a milestone's
/// `due` date.
///
/// Added for T-28g: the `milestone_due_changed` trigger needs a real
/// write path, and none existed before this — a milestone could be
/// created and completed, never moved. Fires the trigger only when
/// `due` actually changes, mirroring `plans::update`'s
/// `plan_timeframe_changed` firing.
#[debug_handler]
pub(super) async fn reschedule_milestone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    Json(payload): Json<MilestoneReschedulePayload>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let m_pid = super::super::parse_pid(&m_pid)?;
    let milestone = vis_models::find_milestone(&ctx, m_pid).await?;
    if milestone.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    if payload
        .reason
        .as_deref()
        .is_some_and(|r| r.chars().count() > 500)
    {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    let old_due = milestone.due;
    let row_pid = milestone.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    // The one column `SeaORM`'s update set, by id; no row back is an error, as before.
    let row: milestones::Model =
        sqlx::query_as("UPDATE milestones SET due = $1 WHERE id = $2 RETURNING *")
            .bind(payload.due)
            .bind(milestone.id)
            .fetch_one(tx.conn())
            .await?;
    // The slip history: one append-only row per actual move.
    if old_due != payload.due {
        sqlx::query(
            "INSERT INTO milestone_slips (pid, milestone_pid, from_due, to_due, reason, actor)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(Uuid::new_v4())
        .bind(row_pid)
        .bind(old_due)
        .bind(payload.due)
        .bind(
            payload
                .reason
                .as_deref()
                .map(str::trim)
                .filter(|r| !r.is_empty()),
        )
        .bind(caller.actor())
        .execute(tx.conn())
        .await?;
    }
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "milestone_rescheduled",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::super::audit_failed)?;
    tx.commit().await?;

    if old_due != payload.due {
        super::super::automation::fire(
            &ctx,
            &crate::automation::TriggerFact {
                kind: "milestone_due_changed".to_string(),
                plan_pid: item.pid,
                from_status: Some(old_due.to_string()),
                to_status: Some(payload.due.to_string()),
            },
            "milestone",
            row_pid,
            caller.actor(),
        )
        .await;
    }

    format::json(serde_json::json!({
        "pid": row.pid.to_string(), "name": row.name, "due": row.due,
    }))
}

/// `POST /api/plans/{pid}/milestones/{m_pid}/complete` body (optional).
#[derive(Debug, Deserialize, Default, schemars::JsonSchema)]
pub(crate) struct MilestoneCompletePayload {
    /// Why. **Required** when the milestone is linked to tasks that are not all
    /// done: completing it by hand over open work is allowed, but not silently.
    #[serde(default)]
    reason: Option<String>,
}

/// `POST /api/plans/{pid}/milestones/{m_pid}/complete` — mark a milestone done.
/// The body is optional; a linked milestone with open tasks needs a `reason`.
#[debug_handler]
pub(super) async fn complete_milestone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    body: axum::body::Bytes,
) -> Result<Response> {
    let payload: MilestoneCompletePayload = if body.iter().all(u8::is_ascii_whitespace) {
        MilestoneCompletePayload::default()
    } else {
        serde_json::from_slice(&body)
            .map_err(|_| unprocessable("body must be JSON like {\"reason\": \"…\"}"))?
    };
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let m_pid = super::super::parse_pid(&m_pid)?;
    let milestone = vis_models::find_milestone(&ctx, m_pid).await?;
    if milestone.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    let reason = payload
        .reason
        .as_deref()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(str::to_string);
    if reason.as_deref().is_some_and(|r| r.chars().count() > 500) {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    // A linked milestone with open tasks may be completed by hand, but only
    // with a reason that says so.
    let progress = super::super::milestones::linked_progress(&ctx, &milestone).await?;
    let open = progress.iter().filter(|c| !**c).count();
    if milestone.completion_mode == "linked" && open > 0 && reason.is_none() {
        return Err(unprocessable(&format!(
            "{open} of {} linked tasks are not done; give a reason to complete the milestone anyway",
            progress.len()
        )));
    }
    let row_pid = milestone.pid;
    // Stamp the completion instant for the delivery-flow metrics
    // (idempotent completes keep the first stamp).
    let done_at: chrono::DateTime<chrono::FixedOffset> = milestone
        .done_at
        .unwrap_or_else(|| chrono::Utc::now().into());
    let mut tx = crate::db::begin(&ctx).await?;
    // The columns `SeaORM`'s update set (`done_reason` only when a reason is given), by id; no row back is an error.
    let row: milestones::Model = if let Some(reason) = reason.as_deref() {
        sqlx::query_as(
            "UPDATE milestones SET done = TRUE, done_at = $1, done_reason = $2 WHERE id = $3 RETURNING *",
        )
        .bind(done_at)
        .bind(reason)
        .bind(milestone.id)
        .fetch_one(tx.conn())
        .await?
    } else {
        sqlx::query_as("UPDATE milestones SET done = TRUE, done_at = $1 WHERE id = $2 RETURNING *")
            .bind(done_at)
            .bind(milestone.id)
            .fetch_one(tx.conn())
            .await?
    };
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "milestone_completed",
        caller.actor(),
        reason
            .as_ref()
            .map(|r| serde_json::json!({ "reason": r, "open_linked_tasks": open })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

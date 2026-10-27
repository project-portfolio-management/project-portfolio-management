//! Milestones: create, list, reschedule, complete.

use super::super::unprocessable;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::milestones;
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::validation::MAX_TEXT_LEN;
use loco_rs::prelude::*;

/// `POST /api/plans/{pid}/milestones` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MilestonePayload {
    name: String,
    due: chrono::NaiveDate,
    /// Optional kind (`milestone` / `demo` / `release` / `checkpoint`);
    /// absent reads as `milestone`.
    #[serde(default)]
    kind: Option<String>,
}

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
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = milestones::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(item.pid),
        name: ActiveValue::set(payload.name.clone()),
        due: ActiveValue::set(payload.due),
        kind: ActiveValue::set(payload.kind.clone()),
        done: ActiveValue::set(false),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "milestone_added", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
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
    let rows = vis_models::milestones_for(&ctx.db, item.pid).await?;
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
    let milestone = vis_models::find_milestone(&ctx.db, m_pid).await?;
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
    let mut active: milestones::ActiveModel = milestone.into();
    active.due = ActiveValue::set(payload.due);
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    // The slip history: one append-only row per actual move.
    if old_due != payload.due {
        crate::models::_entities::milestone_slips::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            milestone_pid: ActiveValue::set(row_pid),
            from_due: ActiveValue::set(old_due),
            to_due: ActiveValue::set(payload.due),
            reason: ActiveValue::set(
                payload
                    .reason
                    .as_deref()
                    .map(str::trim)
                    .filter(|r| !r.is_empty())
                    .map(str::to_string),
            ),
            actor: ActiveValue::set(caller.actor().map(str::to_string)),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    }
    AuditModel::record(&txn, row_pid, "milestone_rescheduled", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;

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
    let milestone = vis_models::find_milestone(&ctx.db, m_pid).await?;
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
    let first_stamp = milestone.done_at;
    let mut active: milestones::ActiveModel = milestone.into();
    active.done = ActiveValue::set(true);
    // Stamp the completion instant for the delivery-flow metrics
    // (idempotent completes keep the first stamp).
    active.done_at = ActiveValue::set(Some(
        first_stamp.unwrap_or_else(|| chrono::Utc::now().into()),
    ));
    if reason.is_some() {
        active.done_reason = ActiveValue::set(reason.clone());
    }
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(
        &txn,
        row_pid,
        "milestone_completed",
        caller.actor(),
        reason
            .as_ref()
            .map(|r| serde_json::json!({ "reason": r, "open_linked_tasks": open })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::json(row)
}

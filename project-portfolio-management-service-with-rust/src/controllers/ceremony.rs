//! **Sprint ceremonies** and the commitment snapshot (entity spec
//! §9.2b / FR-29).
//!
//! The retrospective already existed as `sprint_notes`. This adds
//! planning, daily and review as records, and the snapshot that makes a
//! mid-sprint scope change legible.
//!
//! # Why the commitment is written once
//!
//! Planning declares a set of tasks. If that set could be rewritten, a
//! sprint that grew by half would look like a sprint that was always
//! that size — the goalpost would move silently. So `commit` refuses a
//! second call, and the sprint's *current* task set is reported beside
//! the committed one so the delta is visible rather than inferred.
//!
//! # Sprint metrics are not flow metrics
//!
//! Velocity and burndown here are sprint-scoped and count-based. The
//! Flow Framework metrics (§1.6) are item-scoped and time-based.
//! Neither is derived from the other, and this module computes none of
//! the latter.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{ceremonies, sprint_notes, sprints};
use crate::models::audit_logs::Model as AuditModel;

/// The four ceremonies.
pub const CEREMONY_KINDS: &[&str] = &["planning", "daily", "review", "retrospective"];

/// Load a non-deleted sprint by pid string; `NotFound` when absent.
async fn find_sprint(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    raw: &str,
) -> Result<sprints::Model> {
    let pid = super::parse_pid(raw)?;
    let sprint: sprints::Model =
        sqlx::query_as("SELECT * FROM sprints WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    super::plan_access::require_readable(ctx, caller, sprint.plan_pid).await?;
    Ok(sprint)
}

/// `POST /api/sprints/{pid}/ceremonies` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CeremonyPayload {
    /// Ceremony kind; one of `CEREMONY_KINDS`.
    kind: String,
    /// Reference to the person who facilitated (a reference, not free text).
    #[serde(default)]
    facilitator_ref: Option<String>,
    /// Optional notes on what was decided.
    #[serde(default)]
    summary: Option<String>,
}

/// `POST /api/sprints/{pid}/ceremonies` — hold a ceremony.
#[debug_handler]
async fn create_ceremony(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<CeremonyPayload>,
) -> Result<Response> {
    let sprint = find_sprint(&ctx, &caller, &pid).await?;
    if !CEREMONY_KINDS.contains(&payload.kind.as_str()) {
        return Err(unprocessable(&format!(
            "kind must be one of {CEREMONY_KINDS:?}"
        )));
    }
    // A second planning or review is a re-plan, which is a new sprint —
    // and the schema refuses it too.
    if ["planning", "review"].contains(&payload.kind.as_str()) {
        // Only whether one exists is read.
        let existing: Option<i32> = sqlx::query_scalar(
            "SELECT id FROM ceremonies WHERE sprint_pid = $1 AND kind = $2 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(sprint.pid)
        .bind(payload.kind.as_str())
        .fetch_optional(crate::db::pool(&ctx))
        .await?;
        if existing.is_some() {
            return Err(unprocessable(&format!(
                "this sprint already held its {}: a second one is a re-plan, \
                 which is a new sprint",
                payload.kind
            )));
        }
    }

    let row_pid = Uuid::new_v4();
    let mut tx = crate::db::begin(&ctx).await?;
    // The columns the insert set before SQ-1, no more (`held_at` is the table's default, now).
    sqlx::query(
        "INSERT INTO ceremonies (pid, sprint_pid, kind, facilitator_ref, summary, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL)",
    )
    .bind(row_pid)
    .bind(sprint.pid)
    .bind(&payload.kind)
    .bind(&payload.facilitator_ref)
    .bind(&payload.summary)
    .execute(tx.conn())
    .await?;

    AuditModel::record_on(
        tx.conn(),
        sprint.plan_pid,
        "ceremony_held",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/sprints/{pid}/ceremonies` — with the retrospective notes
/// already stored against the sprint.
#[debug_handler]
async fn list_ceremonies(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let sprint = find_sprint(&ctx, &caller, &pid).await?;
    let held: Vec<ceremonies::Model> = sqlx::query_as(
        "SELECT * FROM ceremonies WHERE sprint_pid = $1 AND deleted_at IS NULL ORDER BY held_at",
    )
    .bind(sprint.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    // In the database's order, as before (no `ORDER BY`).
    let notes: Vec<sprint_notes::Model> =
        sqlx::query_as("SELECT * FROM sprint_notes WHERE sprint_pid = $1 AND deleted_at IS NULL")
            .bind(sprint.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(serde_json::json!({
        "ceremonies": held,
        // Every kind reported, even at zero — a sprint that never held a
        // retrospective is a finding, not a missing row.
        "held": CEREMONY_KINDS.iter().map(|kind| serde_json::json!({
            "kind": kind,
            "count": held.iter().filter(|c| c.kind == *kind).count(),
        })).collect::<Vec<_>>(),
        "retrospective_notes": notes,
    }))
}

/// `POST /api/sprints/{pid}/commit` — snapshot the committed task set.
///
/// Refuses a second call: a rewritable commitment is not a commitment,
/// and a sprint that grew by half would otherwise look like one that was
/// always that size.
#[debug_handler]
async fn commit_sprint(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let sprint = find_sprint(&ctx, &caller, &pid).await?;
    // Only whether one exists is read.
    let existing: Option<i32> =
        sqlx::query_scalar("SELECT id FROM sprint_commitments WHERE sprint_pid = $1 LIMIT 1")
            .bind(sprint.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?;
    if existing.is_some() {
        return Err(unprocessable(
            "this sprint is already committed: a rewritable commitment would let \
             mid-sprint scope look like scope committed at the outset",
        ));
    }

    // Only the pids are read, in the database's order, as before (no `ORDER BY`).
    let current: Vec<Uuid> =
        sqlx::query_scalar("SELECT pid FROM tasks WHERE sprint_pid = $1 AND deleted_at IS NULL")
            .bind(sprint.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut tx = crate::db::begin(&ctx).await?;
    for task_pid in &current {
        sqlx::query(
            "INSERT INTO sprint_commitments (pid, sprint_pid, task_pid) VALUES ($1, $2, $3)",
        )
        .bind(Uuid::new_v4())
        .bind(sprint.pid)
        .bind(task_pid)
        .execute(tx.conn())
        .await?;
    }

    AuditModel::record_on(
        tx.conn(),
        sprint.plan_pid,
        "sprint_committed",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "committed": current.len() }))
}

/// `GET /api/sprints/{pid}/commitment` — the committed set beside the
/// current one, so a scope change reads **as a change**.
#[debug_handler]
async fn commitment(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let sprint = find_sprint(&ctx, &caller, &pid).await?;
    // Only the task pids are read.
    let committed: Vec<Uuid> =
        sqlx::query_scalar("SELECT task_pid FROM sprint_commitments WHERE sprint_pid = $1")
            .bind(sprint.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let current: Vec<Uuid> =
        sqlx::query_scalar("SELECT pid FROM tasks WHERE sprint_pid = $1 AND deleted_at IS NULL")
            .bind(sprint.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;

    let committed_pids: std::collections::BTreeSet<Uuid> = committed.iter().copied().collect();
    let current_pids: std::collections::BTreeSet<Uuid> = current.into_iter().collect();

    let body = serde_json::json!({
        "sprint_pid": sprint.pid.to_string(),
        "committed": committed_pids.len(),
        "current": current_pids.len(),
        // Named, not just counted: "what was added" is the question.
        "added_after_commitment": current_pids.difference(&committed_pids)
            .map(ToString::to_string).collect::<Vec<_>>(),
        "removed_after_commitment": committed_pids.difference(&current_pids)
            .map(ToString::to_string).collect::<Vec<_>>(),
        "was_committed": !committed.is_empty(),
        "note": "Sprint velocity and burndown are sprint-scoped and count-based; \
                 the Flow Framework metrics are item-scoped and time-based. \
                 Neither is derived from the other.",
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// The ceremony routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/sprints/{pid}/ceremonies", post(create_ceremony))
        .add("/sprints/{pid}/ceremonies", get(list_ceremonies))
        .add("/sprints/{pid}/commit", post(commit_sprint))
        .add("/sprints/{pid}/commitment", get(commitment))
}

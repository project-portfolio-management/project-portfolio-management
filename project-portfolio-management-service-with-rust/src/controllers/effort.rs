//! **Recorded effort** and **utilization** — the surface over
//! [`crate::effort`] (entity spec §9.2c / FR-28, FR-35).
//!
//! # What this deliberately does not serve
//!
//! Per-person **utilization** is here, by the owner decision of
//! 2026-08-25. Per-person **cycle time, throughput and flow
//! efficiency** are not, and there is no endpoint from which they could
//! be derived by arithmetic — that refusal
//! (`agents/share/time-based-analysis.md` §7.1) was narrowed, not
//! repealed.
//!
//! # Why the roll-ups are not sorted by utilization
//!
//! Obligation 4: it is never the sole ranking key. The per-person view
//! returns in a stable `actor_ref` order and ships each figure beside
//! its numerator, its denominator, and the deduction that produced the
//! denominator — so a reader has to look at the working, not just the
//! ratio.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use super::find_live_plan as find_plan;
use crate::auth::MaybeAuthUser;
use crate::effort as rules;
use crate::models::_entities::{non_working_periods, time_entries, working_time_configs};
use crate::models::audit_logs::Model as AuditModel;

/// Rows scanned per read.
const MAX_ROWS: i64 = 5000;

/// Default window for a utilization read.
const DEFAULT_WINDOW_DAYS: i64 = 28;

/// `POST /api/plans/{pid}/time-entries` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct EntryPayload {
    /// `person:` or `worker:` URN of who spent the effort (a reference, not a name).
    actor_ref: String,
    /// Calendar date the effort was spent on; an approved timesheet week rejects new entries.
    spent_on: chrono::NaiveDate,
    /// Minutes spent, 1 to 1440 (one day).
    minutes: i64,
    /// Task the effort belongs to; absent means plan-level effort not tied to a task.
    #[serde(default)]
    task_pid: Option<Uuid>,
    /// Effort category token; unknown or absent values are normalized by the effort rules.
    #[serde(default)]
    category: Option<String>,
    /// Whether the time is billable; defaults to false.
    #[serde(default)]
    billable: bool,
    /// Free-text note on the entry; absent means none.
    #[serde(default)]
    note: Option<String>,
}

/// `POST /api/plans/{pid}/time-entries` — record effort.
#[debug_handler]
async fn create_entry(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<EntryPayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    if !payload.actor_ref.starts_with("person:") && !payload.actor_ref.starts_with("worker:") {
        return Err(unprocessable("actor_ref must be a person: or worker: URN"));
    }
    // A day has 1440 minutes. More than that against one date is a
    // typo or a fabrication, and either way it is not effort.
    if payload.minutes <= 0 || payload.minutes > 1_440 {
        return Err(unprocessable(
            "minutes must be between 1 and 1440 — a single day cannot hold more",
        ));
    }

    // LT-11: an approved week is locked against new entries.
    if super::timesheets::week_is_locked(&ctx, &payload.actor_ref, payload.spent_on).await? {
        return Err(Error::CustomError(
            axum::http::StatusCode::CONFLICT,
            axa::error::ErrorDetail::new(
                "conflict",
                "that person's week is approved and locked; it cannot take new entries",
            ),
        ));
    }
    // Time counts on the task it names, so that task must be this plan's.
    if let Some(task) = payload.task_pid {
        let known = sqlx::query(
            "SELECT 1 FROM tasks WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(task)
        .bind(plan.pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await?
        .is_some();
        if !known {
            return Err(unprocessable("task_pid is not a task of this plan"));
        }
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO time_entries
             (pid, plan_pid, task_pid, actor_ref, spent_on, minutes, category, billable, note, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(payload.task_pid)
    .bind(&payload.actor_ref)
    .bind(payload.spent_on)
    .bind(payload.minutes)
    .bind(rules::EffortCategory::parse(payload.category.as_deref()).token())
    .bind(payload.billable)
    .bind(&payload.note)
    .fetch_one(tx.conn())
    .await?;

    AuditModel::record_on(tx.conn(), plan.pid, "effort_recorded", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/plans/{pid}/time-entries` — newest first.
#[debug_handler]
async fn list_entries(Path(pid): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let rows: Vec<time_entries::Model> = sqlx::query_as(
        "SELECT * FROM time_entries WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY spent_on DESC LIMIT $2",
    )
    .bind(plan.pid)
    .bind(MAX_ROWS)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(serde_json::json!(rows))
}

/// Reduce time-entry rows to the facts the effort rules roll up (actor, minutes, category, billable).
fn facts_of(rows: &[time_entries::Model]) -> Vec<rules::EffortFact> {
    rows.iter()
        .map(|row| rules::EffortFact {
            actor_ref: row.actor_ref.clone(),
            minutes: row.minutes,
            category: rules::EffortCategory::parse(Some(&row.category)),
            billable: row.billable,
        })
        .collect()
}

/// `GET /api/plans/{pid}/effort` — roll-ups per plan, per task and per
/// assignee, every one labelled **asserted**.
#[debug_handler]
async fn effort_rollup(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let rows: Vec<time_entries::Model> = sqlx::query_as(
        "SELECT * FROM time_entries WHERE plan_pid = $1 AND deleted_at IS NULL LIMIT $2",
    )
    .bind(plan.pid)
    .bind(MAX_ROWS)
    .fetch_all(crate::db::pool(&ctx))
    .await?;

    let mut by_actor: std::collections::BTreeMap<String, Vec<time_entries::Model>> =
        std::collections::BTreeMap::new();
    let mut by_task: std::collections::BTreeMap<String, Vec<time_entries::Model>> =
        std::collections::BTreeMap::new();
    for row in &rows {
        by_actor
            .entry(row.actor_ref.clone())
            .or_default()
            .push(row.clone());
        if let Some(task_pid) = row.task_pid {
            by_task
                .entry(task_pid.to_string())
                .or_default()
                .push(row.clone());
        }
    }

    let body = serde_json::json!({
        "plan_pid": plan.pid.to_string(),
        "plan": rules::totals(&facts_of(&rows)),
        // Stable key order, and deliberately not sorted by size:
        // obligation 4, never the sole ranking key.
        "by_assignee": by_actor.iter().map(|(actor, entries)| serde_json::json!({
            "actor_ref": actor,
            "totals": rules::totals(&facts_of(entries)),
        })).collect::<Vec<_>>(),
        "by_task": by_task.iter().map(|(task, entries)| serde_json::json!({
            "task_pid": task,
            "totals": rules::totals(&facts_of(entries)),
        })).collect::<Vec<_>>(),
        "as_of": chrono::Utc::now(),
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// `POST /api/working-time` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct WorkingTimePayload {
    /// Scope the declaration applies to; absent is the organization-wide default read by the utilization report.
    #[serde(default)]
    scope_ref: Option<String>,
    /// Working minutes in a day, 1 to 1440.
    minutes_per_day: i32,
    /// Working days in a week, 1 to 7.
    working_days_per_week: i32,
}

/// `POST /api/working-time` — declare the capacity basis.
#[debug_handler]
async fn set_working_time(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<WorkingTimePayload>,
) -> Result<Response> {
    if payload.minutes_per_day <= 0 || payload.minutes_per_day > 1_440 {
        return Err(unprocessable("minutes_per_day must be between 1 and 1440"));
    }
    if !(1..=7).contains(&payload.working_days_per_week) {
        return Err(unprocessable(
            "working_days_per_week must be between 1 and 7",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO working_time_configs (pid, scope_ref, minutes_per_day, working_days_per_week, deleted_at)
         VALUES ($1, $2, $3, $4, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(&payload.scope_ref)
    .bind(payload.minutes_per_day)
    .bind(payload.working_days_per_week)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "working_time_declared",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `POST /api/non-working` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NonWorkingPayload {
    /// `person:` reference of the person who is away.
    person_ref: String,
    /// First day of the period (inclusive).
    starts_on: chrono::NaiveDate,
    /// Last day of the period (inclusive); not before `starts_on`.
    ends_on: chrono::NaiveDate,
    /// One of `leave`, `holiday`, `study_leave` or `non_project_duty`.
    kind: String,
    /// Free-text note; absent means none.
    #[serde(default)]
    note: Option<String>,
}

/// `POST /api/non-working` — record leave or other non-working time.
///
/// This is what stops somebody on leave reporting 0% utilization, which
/// would read as measured idleness.
#[debug_handler]
async fn create_non_working(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<NonWorkingPayload>,
) -> Result<Response> {
    if !["leave", "holiday", "study_leave", "non_project_duty"].contains(&payload.kind.as_str()) {
        return Err(unprocessable(
            "kind must be leave, holiday, study_leave or non_project_duty",
        ));
    }
    if payload.ends_on < payload.starts_on {
        return Err(unprocessable("ends_on must not precede starts_on"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO non_working_periods (pid, person_ref, starts_on, ends_on, kind, note, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(&payload.person_ref)
    .bind(payload.starts_on)
    .bind(payload.ends_on)
    .bind(&payload.kind)
    .bind(&payload.note)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "non_working_recorded",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/capacity/utilization` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct UtilizationParams {
    /// Grouping: `plan`, `team` or `person`; absent means `team`.
    #[serde(default)]
    by: Option<String>,
    /// Window length in days, counted back from today; values outside 1 to 366 fall back to the 28-day default.
    #[serde(default)]
    window_days: Option<i64>,
    /// Restrict the entries to one plan; absent includes all visible plans.
    #[serde(default)]
    plan_pid: Option<Uuid>,
}

/// Minutes of declared capacity in a window, from the config in force.
///
/// Integer arithmetic throughout, matching the rest of this crate: a
/// capacity denominator is the sort of figure people reconcile against
/// a payroll system, and a float would introduce a discrepancy nobody
/// could explain. `window_days × working_days ÷ 7 × minutes_per_day`,
/// with the division last so the truncation is a whole minute rather
/// than a fraction of a working week.
fn declared_minutes(config: Option<&working_time_configs::Model>, window_days: i64) -> i64 {
    let Some(config) = config else { return 0 };
    window_days
        .checked_mul(i64::from(config.working_days_per_week))
        .and_then(|d| d.checked_mul(i64::from(config.minutes_per_day)))
        .map_or(0, |total| total / 7)
}

/// `GET /api/capacity/utilization?by=plan|team|person` — effort against
/// declared capacity.
///
/// **No per-person cycle time, throughput or flow efficiency is served
/// from here or derivable from what it returns.**
#[debug_handler]
async fn utilization(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<UtilizationParams>,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let by = params.by.unwrap_or_else(|| "team".to_string());
    if !["plan", "team", "person"].contains(&by.as_str()) {
        return Err(unprocessable("by must be plan, team or person"));
    }
    let window_days = params
        .window_days
        .filter(|d| *d > 0 && *d <= 366)
        .unwrap_or(DEFAULT_WINDOW_DAYS);
    let since = crate::tz::deployment_today() - chrono::Duration::days(window_days);

    let entries = scope.keep(
        sqlx::query_as::<_, time_entries::Model>(
            "SELECT * FROM time_entries
             WHERE deleted_at IS NULL AND spent_on >= $1 AND ($2::uuid IS NULL OR plan_pid = $2)
             LIMIT $3",
        )
        .bind(since)
        .bind(params.plan_pid)
        .bind(MAX_ROWS)
        .fetch_all(crate::db::pool(&ctx))
        .await?,
    );

    let config: Option<working_time_configs::Model> = sqlx::query_as(
        "SELECT * FROM working_time_configs WHERE scope_ref IS NULL AND deleted_at IS NULL LIMIT 1",
    )
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    let declared = declared_minutes(config.as_ref(), window_days);

    let absences: Vec<non_working_periods::Model> = sqlx::query_as(
        "SELECT * FROM non_working_periods WHERE deleted_at IS NULL AND ends_on >= $1 LIMIT $2",
    )
    .bind(since)
    .bind(MAX_ROWS)
    .fetch_all(crate::db::pool(&ctx))
    .await?;

    let mut per_person: std::collections::BTreeMap<String, i64> = std::collections::BTreeMap::new();
    for entry in &entries {
        *per_person.entry(entry.actor_ref.clone()).or_default() += entry.minutes;
    }
    // Somebody who was entirely on leave has no effort rows, so they
    // would be invisible. Include them, precisely so the answer is
    // "on leave" rather than a silent absence.
    for absence in &absences {
        per_person.entry(absence.person_ref.clone()).or_insert(0);
    }

    let minutes_per_day = config.as_ref().map_or(0, |c| i64::from(c.minutes_per_day));
    let facts: Vec<rules::CapacityFact> = per_person
        .iter()
        .map(|(actor, minutes)| {
            let non_working: i64 = absences
                .iter()
                .filter(|a| &a.person_ref == actor)
                .map(|a| {
                    let days = (a.ends_on - a.starts_on).num_days() + 1;
                    days.max(0).saturating_mul(minutes_per_day)
                })
                .sum();
            rules::CapacityFact {
                actor_ref: actor.clone(),
                declared_minutes: declared,
                non_working_minutes: non_working.min(declared),
                effort_minutes: *minutes,
            }
        })
        .collect();

    // WW-1.6: both thresholds are the deployment's; the answer says which were used.
    let floor = rules::floor_minutes();
    let min_people = rules::min_group_people();
    let body = match by.as_str() {
        "person" => serde_json::json!({
            "by": "person",
            "window_days": window_days,
            // Stable order by actor, **not** sorted by utilization.
            "utilization": facts.iter().map(|f| rules::utilization(f, floor)).collect::<Vec<_>>(),
            "suppression": { "floor_minutes": floor, "min_group_people": min_people },
            "note": "Utilization near or above 100% is a warning about the queue, \
                     not an achievement. No per-person cycle time, throughput or \
                     flow efficiency is served here or derivable from it.",
        }),
        _ => serde_json::json!({
            "by": by,
            "window_days": window_days,
            "utilization": rules::group_utilization(&facts, floor, min_people),
            "people_counted": facts.len(),
            "suppression": { "floor_minutes": floor, "min_group_people": min_people },
        }),
    };
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// The effort and utilization routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/time-entries", post(create_entry))
        .add("/plans/{pid}/time-entries", get(list_entries))
        .add("/plans/{pid}/effort", get(effort_rollup))
        .add("/working-time", post(set_working_time))
        .add("/non-working", post(create_non_working))
        .add("/capacity/utilization", get(utilization))
}

//! Personal planner (PL-4): block time for a task, and see a day's slots in
//! the caller's own zone. Pure rules in [`crate::planner`].
//!
//! A slot belongs to the person who made it and is private to them. It never
//! changes the task's due date. Overlapping slots are allowed and **reported
//! as clashes**. A slot on a task the caller can no longer read is omitted
//! (and counted), never shown masked.

use super::unprocessable;
use crate::prelude::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::plan_access::readable_plans;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::{planned_slots, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::planner::{self as rules, Span};
use crate::tz;

/// Live slots one person may hold.
const MAX_SLOTS: u64 = 2_000;

/// `POST /api/me/planned-slots` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SlotBody {
    /// Task the time is blocked for.
    task_pid: Uuid,
    /// Start of the slot (UTC).
    starts_at: DateTime<Utc>,
    /// End of the slot (UTC); validated by `slot_problems`.
    ends_at: DateTime<Utc>,
}

/// `POST /api/me/planned-slots` — block time for a task.
#[debug_handler]
async fn create_slot(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<SlotBody>,
) -> Result<Response> {
    let problems = rules::slot_problems(b.starts_at, b.ends_at);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let caller = MaybeAuthUser(Some(claims.clone()));
    resolve_subject(&ctx, &caller, "task", &b.task_pid.to_string()).await?;
    let held: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM planned_slots WHERE sub = $1 AND deleted_at IS NULL",
    )
    .bind(claims.sub.as_str())
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    if u64::try_from(held).unwrap_or(0) >= MAX_SLOTS {
        return Err(unprocessable(&format!(
            "at most {MAX_SLOTS} planned slots per person"
        )));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: planned_slots::Model = sqlx::query_as(
        "INSERT INTO planned_slots (pid, sub, task_pid, starts_at, ends_at, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&claims.sub)
    .bind(b.task_pid)
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(b.starts_at))
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(b.ends_at))
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "slot_planned", Some(&claims.sub), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/me/planned-slots/{pid}/delete` (and `DELETE`) — remove one of
/// the caller's own slots. Another person's slot is `404`, identically.
#[debug_handler]
async fn delete_slot(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let row: planned_slots::Model = sqlx::query_as(
        "SELECT * FROM planned_slots WHERE pid = $1 AND sub = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .bind(claims.sub.as_str())
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await?;
    // An update that finds no row is an error, as it was.
    sqlx::query("UPDATE planned_slots SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::DateTime::<chrono::FixedOffset>::from(
            chrono::Utc::now(),
        ))
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(tx.conn(), pid, "slot_removed", Some(&claims.sub), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `GET /api/me/plan` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PlanParams {
    /// Local calendar day to show.
    day: NaiveDate,
    /// IANA time-zone name the day is read in.
    tz: String,
}

/// `GET /api/me/plan?day=&tz=` — the caller's slots for a local calendar day,
/// rendered in their zone, with clashes and the tasks due that day.
#[debug_handler]
async fn my_plan(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(p): Query<PlanParams>,
) -> Result<Response> {
    let zone = tz::parse_zone(&p.tz).map_err(|e| unprocessable(&e))?;
    let (from, to) = rules::day_window(p.day, zone);
    let slots: Vec<planned_slots::Model> = sqlx::query_as(
        "SELECT * FROM planned_slots
         WHERE sub = $1 AND deleted_at IS NULL AND starts_at < $2 AND ends_at > $3
         ORDER BY starts_at
         LIMIT 500",
    )
    .bind(claims.sub.as_str())
    .bind(to)
    .bind(from)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let task_pids: Vec<Uuid> = slots.iter().map(|s| s.task_pid).collect();
    let task_rows: Vec<tasks::Model> = if task_pids.is_empty() {
        Vec::new()
    } else {
        sqlx::query_as("SELECT * FROM tasks WHERE pid = ANY($1) AND deleted_at IS NULL")
            .bind(task_pids)
            .fetch_all(crate::db::pool(&ctx))
            .await?
    };
    let caller = MaybeAuthUser(Some(claims.clone()));
    let mut plan_pids: Vec<Uuid> = task_rows.iter().map(|t| t.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = readable_plans(&ctx, &caller, &plan_pids).await;
    // Deleted plans too, as before: a task's plan is named whatever became of it.
    let plan_names: std::collections::HashMap<Uuid, String> =
        sqlx::query_as::<_, (Uuid, String)>("SELECT pid, name FROM plans WHERE pid = ANY($1)")
            .bind(plan_pids)
            .fetch_all(crate::db::pool(&ctx))
            .await?
            .into_iter()
            .collect();
    let mut shown = Vec::new();
    let mut spans = Vec::new();
    let mut omitted = 0usize;
    for s in &slots {
        let Some(t) = task_rows.iter().find(|t| t.pid == s.task_pid) else {
            omitted += 1;
            continue;
        };
        if !readable.contains(&t.plan_pid) {
            omitted += 1;
            continue;
        }
        let (a, b) = (s.starts_at.to_utc(), s.ends_at.to_utc());
        spans.push(Span {
            id: s.pid,
            start: a,
            end: b,
        });
        shown.push(serde_json::json!({
            "pid": s.pid,
            "task": { "pid": t.pid, "title": t.title, "status": t.status,
                      "plan_pid": t.plan_pid, "plan_name": plan_names.get(&t.plan_pid) },
            "starts_at": a,
            "ends_at": b,
            "starts_local": a.with_timezone(&zone).to_rfc3339(),
            "ends_local": b.with_timezone(&zone).to_rfc3339(),
        }));
    }
    // Tasks due that day assigned to the caller: what the day still owes.
    let refs: Vec<String> = Uuid::parse_str(&claims.sub)
        .map(|u| vec![format!("person:{u}"), format!("worker:{u}")])
        .unwrap_or_default();
    let due: Vec<serde_json::Value> = if refs.is_empty() {
        Vec::new()
    } else {
        let rows: Vec<tasks::Model> = sqlx::query_as(
            "SELECT * FROM tasks
             WHERE assignee_ref = ANY($1) AND due_date = $2 AND deleted_at IS NULL AND done_at IS NULL",
        )
        .bind(refs)
        .bind(p.day)
        .fetch_all(crate::db::pool(&ctx))
        .await?;
        let pids: Vec<Uuid> = rows.iter().map(|t| t.plan_pid).collect();
        let ok = readable_plans(&ctx, &caller, &pids).await;
        rows.into_iter()
            .filter(|t| ok.contains(&t.plan_pid))
            .map(|t| serde_json::json!({ "pid": t.pid, "title": t.title, "plan_pid": t.plan_pid }))
            .collect()
    };
    format::json(serde_json::json!({
        "day": p.day,
        "tz": zone.name(),
        "window": { "from": from, "to": to, "hours": (to - from).num_hours() },
        "slots": shown,
        "clashes": rules::clashes(&spans),
        "due_on_day": due,
        "omitted_unreadable": omitted,
    }))
}

/// The planner routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/me/planned-slots", post(create_slot))
        .add("/me/planned-slots/{pid}", delete(delete_slot))
        .add("/me/planned-slots/{pid}/delete", post(delete_slot))
        .add("/me/plan", get(my_plan))
}

//! Personal planner (PL-4): block time for a task, and see a day's slots in
//! the caller's own zone. Pure rules in [`crate::planner`].
//!
//! A slot belongs to the person who made it and is private to them. It never
//! changes the task's due date. Overlapping slots are allowed and **reported
//! as clashes**. A slot on a task the caller can no longer read is omitted
//! (and counted), never shown masked.

use super::{db_err, unprocessable};
use chrono::{DateTime, NaiveDate, Utc};
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::plan_access::readable_plans;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::{planned_slots, plans, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::planner::{self as rules, Span};
use crate::tz;

/// Live slots one person may hold.
const MAX_SLOTS: u64 = 2_000;

/// `POST /api/me/planned-slots` body.
#[derive(Debug, Deserialize)]
struct SlotBody {
    task_pid: Uuid,
    starts_at: DateTime<Utc>,
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
    let held = planned_slots::Entity::find()
        .filter(planned_slots::Column::Sub.eq(claims.sub.as_str()))
        .filter(planned_slots::Column::DeletedAt.is_null())
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    if held >= MAX_SLOTS {
        return Err(unprocessable(&format!(
            "at most {MAX_SLOTS} planned slots per person"
        )));
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = planned_slots::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        sub: ActiveValue::set(claims.sub.clone()),
        task_pid: ActiveValue::set(b.task_pid),
        starts_at: ActiveValue::set(b.starts_at.into()),
        ends_at: ActiveValue::set(b.ends_at.into()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(&txn, row.pid, "slot_planned", Some(&claims.sub), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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
    let row = planned_slots::Entity::find()
        .filter(planned_slots::Column::Pid.eq(pid))
        .filter(planned_slots::Column::Sub.eq(claims.sub.as_str()))
        .filter(planned_slots::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut active: planned_slots::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    active.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, pid, "slot_removed", Some(&claims.sub), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::empty_json()
}

/// `GET /api/me/plan` query.
#[derive(Debug, Deserialize)]
struct PlanParams {
    day: NaiveDate,
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
    let slots = planned_slots::Entity::find()
        .filter(planned_slots::Column::Sub.eq(claims.sub.as_str()))
        .filter(planned_slots::Column::DeletedAt.is_null())
        .filter(planned_slots::Column::StartsAt.lt(to))
        .filter(planned_slots::Column::EndsAt.gt(from))
        .order_by_asc(planned_slots::Column::StartsAt)
        .limit(500)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let task_pids: Vec<Uuid> = slots.iter().map(|s| s.task_pid).collect();
    let task_rows = if task_pids.is_empty() {
        Vec::new()
    } else {
        tasks::Entity::find()
            .filter(tasks::Column::Pid.is_in(task_pids))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
    };
    let caller = MaybeAuthUser(Some(claims.clone()));
    let mut plan_pids: Vec<Uuid> = task_rows.iter().map(|t| t.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = readable_plans(&ctx, &caller, &plan_pids).await;
    let plan_names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(plan_pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
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
        let rows = tasks::Entity::find()
            .filter(tasks::Column::AssigneeRef.is_in(refs))
            .filter(tasks::Column::DueDate.eq(p.day))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::DoneAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
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

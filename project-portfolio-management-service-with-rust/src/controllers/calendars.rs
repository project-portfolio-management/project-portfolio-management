//! Working calendars (SC-2): one per plan, one per person. Pure rules in
//! [`crate::calendars`]; the engine in [`crate::cpm`].
//!
//! A plan with no calendar works all seven days (what every plan did before
//! calendars existed, so nothing already stored changes). A person's own
//! calendar is theirs to set; their **recorded absences** (`non_working_periods`)
//! count as holidays automatically. A task is scheduled on its plan's calendar,
//! and an assignee's narrows it: their work happens only on days both work.

use super::db_err;
use std::collections::HashMap;

use chrono::{NaiveDate, Utc};
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::calendars as rules;
use crate::cpm::WorkCalendar;
use crate::models::_entities::{non_working_periods, work_calendars};
use crate::models::audit_logs::Model as AuditModel;

fn holidays_of(row: &work_calendars::Model) -> Vec<NaiveDate> {
    serde_json::from_value::<Vec<String>>(row.holidays.clone())
        .unwrap_or_default()
        .iter()
        .filter_map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .collect()
}

async fn find<C: ConnectionTrait>(
    db: &C,
    scope: &str,
    subject: &str,
) -> Result<Option<work_calendars::Model>> {
    work_calendars::Entity::find()
        .filter(work_calendars::Column::Scope.eq(scope))
        .filter(work_calendars::Column::Subject.eq(subject))
        .filter(work_calendars::Column::DeletedAt.is_null())
        .one(db)
        .await
        .map_err(db_err)
}

/// A plan's calendar and where it came from (`plan` or `default`).
pub(crate) async fn plan_calendar(
    ctx: &AppContext,
    plan_pid: Uuid,
) -> Result<(WorkCalendar, &'static str)> {
    match find(&ctx.db, "plan", &plan_pid.to_string()).await? {
        Some(row) => Ok((
            rules::build(&row.weekdays, &holidays_of(&row))
                .unwrap_or_else(|_| WorkCalendar::seven_day()),
            "plan",
        )),
        None => Ok((WorkCalendar::seven_day(), "default")),
    }
}

/// For each person ref in `refs` that has a calendar or recorded absences, the
/// calendar their work runs on: the plan's, intersected with theirs. A person
/// with nothing recorded, or whose calendar shares no business day with the
/// plan's, is **absent from the map** (their tasks use the plan calendar; the
/// second return lists the latter so it can be reported, not hidden).
pub(crate) async fn person_calendars(
    ctx: &AppContext,
    plan_cal: &WorkCalendar,
    refs: &[String],
) -> Result<(HashMap<String, WorkCalendar>, Vec<String>)> {
    let mut out = HashMap::new();
    let mut conflicts = Vec::new();
    for r in refs {
        let row = find(&ctx.db, "person", r).await?;
        let absences: Vec<(NaiveDate, NaiveDate)> = non_working_periods::Entity::find()
            .filter(non_working_periods::Column::PersonRef.eq(r.as_str()))
            .filter(non_working_periods::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|p| (p.starts_on, p.ends_on))
            .collect();
        if row.is_none() && absences.is_empty() {
            continue;
        }
        let mut holidays = rules::expand_absences(&absences);
        let weekdays = row
            .as_ref()
            .map_or_else(|| "1111111".to_string(), |c| c.weekdays.clone());
        if let Some(c) = &row {
            holidays.extend(holidays_of(c));
        }
        match rules::build(&weekdays, &holidays).and_then(|own| plan_cal.intersect(&own)) {
            Ok(cal) => {
                out.insert(r.clone(), cal);
            }
            Err(_) => conflicts.push(r.clone()),
        }
    }
    Ok((out, conflicts))
}

fn view(row: Option<&work_calendars::Model>, default_note: &str) -> serde_json::Value {
    row.map_or_else(
        || serde_json::json!({ "source": "default", "weekdays": "1111111", "holidays": [], "note": default_note }),
        |c| serde_json::json!({ "source": c.scope, "weekdays": c.weekdays, "holidays": holidays_of(c) }),
    )
}

/// `PUT` body for both calendars.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CalendarBody {
    weekdays: String,
    #[serde(default)]
    holidays: Vec<NaiveDate>,
}

/// Set a plan's calendar (used by the MS Project import as well as the route).
pub(crate) async fn set_plan_calendar(
    ctx: &AppContext,
    plan_pid: Uuid,
    weekdays: &str,
    holidays: &[NaiveDate],
    actor: Option<&str>,
) -> Result<()> {
    let body = CalendarBody {
        weekdays: weekdays.to_string(),
        holidays: holidays.to_vec(),
    };
    upsert(ctx, "plan", &plan_pid.to_string(), &body, actor)
        .await
        .map(|_| ())
}

/// A plan's own calendar as stored (weekday mask, holidays), or `None` when it has none.
pub(crate) async fn plan_calendar_spec(
    ctx: &AppContext,
    plan_pid: Uuid,
) -> Result<Option<(String, Vec<NaiveDate>)>> {
    Ok(find(&ctx.db, "plan", &plan_pid.to_string())
        .await?
        .map(|row| (row.weekdays.clone(), holidays_of(&row))))
}

/// Whether a plan already has its own calendar.
pub(crate) async fn plan_has_calendar(ctx: &AppContext, plan_pid: Uuid) -> Result<bool> {
    Ok(find(&ctx.db, "plan", &plan_pid.to_string())
        .await?
        .is_some())
}

async fn upsert(
    ctx: &AppContext,
    scope: &str,
    subject: &str,
    b: &CalendarBody,
    actor: Option<&str>,
) -> Result<work_calendars::Model> {
    let problems = rules::problems(&b.weekdays, &b.holidays);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut holidays = b.holidays.clone();
    holidays.sort_unstable();
    let json = serde_json::json!(holidays.iter().map(ToString::to_string).collect::<Vec<_>>());
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = match find(&txn, scope, subject).await? {
        Some(existing) => {
            let mut a: work_calendars::ActiveModel = existing.into();
            a.weekdays = ActiveValue::set(b.weekdays.clone());
            a.holidays = ActiveValue::set(json);
            a.update(&txn).await.map_err(db_err)?
        }
        None => work_calendars::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            scope: ActiveValue::set(scope.to_string()),
            subject: ActiveValue::set(subject.to_string()),
            weekdays: ActiveValue::set(b.weekdays.clone()),
            holidays: ActiveValue::set(json),
            deleted_at: ActiveValue::set(None),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?,
    };
    AuditModel::record(
        &txn,
        row.pid,
        "calendar_set",
        actor,
        Some(
            serde_json::json!({ "scope": scope, "subject": subject, "weekdays": b.weekdays,
                                  "holidays": holidays.len(), "at": Utc::now() }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    Ok(row)
}

/// `GET /api/plans/{pid}/calendar` — the plan's calendar (all seven days when
/// none was set).
#[debug_handler]
async fn get_plan(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = find(&ctx.db, "plan", &plan.pid.to_string()).await?;
    format::json(view(row.as_ref(), "no calendar set: every day works"))
}

/// `PUT /api/plans/{pid}/calendar` — set the plan's working weekdays and holidays.
#[debug_handler]
async fn put_plan(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<CalendarBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = upsert(&ctx, "plan", &plan.pid.to_string(), &b, caller.actor()).await?;
    format::json(view(Some(&row), ""))
}

fn person_subject(sub: &str) -> Result<String> {
    Uuid::parse_str(sub)
        .map(|u| format!("person:{u}"))
        .map_err(|_| unprocessable("sub_not_addressable: this identity has no person reference"))
}

/// `GET /api/me/calendar` — the caller's own calendar.
#[debug_handler]
async fn get_me(State(ctx): State<AppContext>, AuthUser(claims): AuthUser) -> Result<Response> {
    let row = find(&ctx.db, "person", &person_subject(&claims.sub)?).await?;
    format::json(view(
        row.as_ref(),
        "no personal calendar set: the plan's calendar applies",
    ))
}

/// `PUT /api/me/calendar` — set the caller's own working weekdays and holidays.
/// Recorded absences already count automatically; this is for the weekly pattern.
#[debug_handler]
async fn put_me(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<CalendarBody>,
) -> Result<Response> {
    let subject = person_subject(&claims.sub)?;
    let row = upsert(&ctx, "person", &subject, &b, Some(&claims.sub)).await?;
    format::json(view(Some(&row), ""))
}

/// The calendar routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/calendar", get(get_plan))
        .add("/plans/{pid}/calendar", put(put_plan))
        .add("/me/calendar", get(get_me))
        .add("/me/calendar", put(put_me))
}

//! Timesheets (LT-11) and time reports (RM-10) over the recorded
//! `time_entries`. Pure rules in [`crate::timesheets`].
//!
//! - **Week grid** — one person's week by day and plan, in whole minutes,
//!   with billable and non-billable **always carried apart**.
//! - **Submit / approve / reject** — a person submits their **own** week;
//!   someone else with the approver right decides (never on their own week);
//!   a rejection says why; **approved is final** and locks the week against
//!   new entries (the lock lives in the time-entry create path).
//! - **Time report** — grouped by person / plan / category / billable /
//!   month, JSON or CSV, over plans the caller can read.
//!
//! An approver is a caller holding `access=admin` or the `timesheet_approver`
//! role (the token's `role` attribute — a vocabulary the deployment declares).

use super::{conflict, forbidden};
use super::{db_err, unprocessable};
use std::fmt::Write as _;

use axum::http::{StatusCode, header};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::plan_access::readable_plans;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::{time_entries, timesheet_weeks};
use crate::models::audit_logs::Model as AuditModel;
use crate::notification_prefs::recipient_sub;
use crate::timesheets::{self as rules, Entry, GroupBy};

/// Rows a report or week reads at most.
const ROW_CAP: u64 = 20_000;

fn is_approver(claims: &authentication_verifier::Claims) -> bool {
    let has = |key: &str, val: &str| {
        claims
            .attrs
            .get(key)
            .is_some_and(|v| v.iter().any(|a| a == val))
    };
    has("access", "admin") || has("role", "timesheet_approver")
}

/// The person a request is about: an explicit `person:`/`worker:` URN (with a
/// UUID id), or the caller's own `person:<sub>`.
fn person_of(raw: Option<&str>, sub: &str) -> Result<String> {
    match raw {
        Some(p) if recipient_sub(p).is_some() => Ok(p.to_string()),
        Some(_) => Err(unprocessable(
            "person must be a person:/worker: URN with a user id",
        )),
        None => Uuid::parse_str(sub)
            .map(|u| format!("person:{u}"))
            .map_err(|_| {
                unprocessable("sub_not_addressable: this identity has no person reference")
            }),
    }
}

fn is_self(person: &str, sub: &str) -> bool {
    recipient_sub(person).is_some_and(|id| id.eq_ignore_ascii_case(sub))
}

fn week_of(raw: &str) -> Result<NaiveDate> {
    rules::week_start(raw).ok_or_else(|| unprocessable("week must be an ISO week such as 2026-W40"))
}

async fn week_entries(
    ctx: &AppContext,
    person: &str,
    monday: NaiveDate,
) -> Result<Vec<time_entries::Model>> {
    time_entries::Entity::find()
        .filter(time_entries::Column::ActorRef.eq(person))
        .filter(time_entries::Column::DeletedAt.is_null())
        .filter(time_entries::Column::SpentOn.gte(monday))
        .filter(time_entries::Column::SpentOn.lte(monday + chrono::Duration::days(6)))
        .order_by_asc(time_entries::Column::SpentOn)
        .limit(ROW_CAP)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

fn fact(e: &time_entries::Model) -> Entry {
    Entry {
        plan_pid: e.plan_pid,
        actor_ref: e.actor_ref.clone(),
        date: e.spent_on,
        minutes: e.minutes,
        category: e.category.clone(),
        billable: e.billable,
    }
}

async fn week_row<C: ConnectionTrait>(
    db: &C,
    person: &str,
    monday: NaiveDate,
) -> Result<Option<timesheet_weeks::Model>> {
    timesheet_weeks::Entity::find()
        .filter(timesheet_weeks::Column::PersonRef.eq(person))
        .filter(timesheet_weeks::Column::WeekStart.eq(monday))
        .one(db)
        .await
        .map_err(db_err)
}

/// Whether `person`'s week containing `date` is approved (and so locked).
pub(crate) async fn week_is_locked(
    ctx: &AppContext,
    person: &str,
    date: NaiveDate,
) -> Result<bool> {
    Ok(week_row(&ctx.db, person, rules::monday_of(date))
        .await?
        .is_some_and(|r| r.status == "approved"))
}

/// `GET /api/timesheets` query.
#[derive(Debug, Deserialize)]
struct GridParams {
    #[serde(default)]
    person: Option<String>,
    week: String,
}

/// `GET /api/timesheets?person=&week=` — the week grid and its state.
#[debug_handler]
async fn get_week(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(p): Query<GridParams>,
) -> Result<Response> {
    let person = person_of(p.person.as_deref(), &claims.sub)?;
    if !is_self(&person, &claims.sub) && !is_approver(&claims) {
        return Err(forbidden(
            "only the person or an approver may read this week",
        ));
    }
    let monday = week_of(&p.week)?;
    let rows = week_entries(&ctx, &person, monday).await?;
    // Plans the *reader* may not see are left out of the grid (and counted).
    let caller = MaybeAuthUser(Some(claims.clone()));
    let mut plan_pids: Vec<Uuid> = rows.iter().map(|e| e.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = readable_plans(&ctx, &caller, &plan_pids).await;
    let facts: Vec<Entry> = rows
        .iter()
        .filter(|e| readable.contains(&e.plan_pid))
        .map(fact)
        .collect();
    let omitted = rows.len() - facts.len();
    let state = week_row(&ctx.db, &person, monday).await?;
    format::json(serde_json::json!({
        "person": person,
        "week": p.week,
        "week_start": monday,
        "status": state.as_ref().map_or("open", |s| s.status.as_str()),
        "decided_by": state.as_ref().and_then(|s| s.decided_by.clone()),
        "decision_comment": state.as_ref().and_then(|s| s.decision_comment.clone()),
        "grid": rules::grid(&facts, monday),
        "omitted_unreadable": omitted,
    }))
}

/// `POST /api/timesheets/submit` body.
#[derive(Debug, Deserialize)]
struct SubmitBody {
    #[serde(default)]
    person: Option<String>,
    week: String,
}

async fn set_state<C: ConnectionTrait>(
    db: &C,
    person: &str,
    monday: NaiveDate,
    status: &str,
    submitted: bool,
    decider: Option<(&str, Option<&str>)>,
) -> Result<timesheet_weeks::Model> {
    let existing = week_row(db, person, monday).await?;
    let now: sea_orm::prelude::DateTimeWithTimeZone = chrono::Utc::now().into();
    match existing {
        Some(row) => {
            let mut a: timesheet_weeks::ActiveModel = row.into();
            a.status = ActiveValue::set(status.to_string());
            if submitted {
                a.submitted_at = ActiveValue::set(Some(now));
                a.decided_by = ActiveValue::set(None);
                a.decided_at = ActiveValue::set(None);
                a.decision_comment = ActiveValue::set(None);
            }
            if let Some((by, comment)) = decider {
                a.decided_by = ActiveValue::set(Some(by.to_string()));
                a.decided_at = ActiveValue::set(Some(now));
                a.decision_comment = ActiveValue::set(comment.map(|c| c.trim().to_string()));
            }
            a.update(db).await.map_err(db_err)
        }
        None => timesheet_weeks::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            person_ref: ActiveValue::set(person.to_string()),
            week_start: ActiveValue::set(monday),
            status: ActiveValue::set(status.to_string()),
            submitted_at: ActiveValue::set(submitted.then_some(now)),
            decided_by: ActiveValue::set(decider.map(|(b, _)| b.to_string())),
            decided_at: ActiveValue::set(decider.map(|_| now)),
            decision_comment: ActiveValue::set(decider.and_then(|(_, c)| c.map(str::to_string))),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(db_err),
    }
}

/// `POST /api/timesheets/submit` — a person submits their **own** week.
#[debug_handler]
async fn submit(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<SubmitBody>,
) -> Result<Response> {
    let person = person_of(b.person.as_deref(), &claims.sub)?;
    if !is_self(&person, &claims.sub) {
        return Err(forbidden(
            "a week can only be submitted by the person it belongs to",
        ));
    }
    let monday = week_of(&b.week)?;
    let entries = week_entries(&ctx, &person, monday).await?;
    let current = week_row(&ctx.db, &person, monday).await?;
    let next =
        rules::submit(current.as_ref().map(|r| r.status.as_str()), entries.len()).map_err(|r| {
            match r {
                rules::Refusal::Empty => unprocessable(&r.message()),
                _ => conflict(&r.message()),
            }
        })?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = set_state(&txn, &person, monday, next, true, None).await?;
    AuditModel::record(
        &txn,
        row.pid,
        "timesheet_submitted",
        Some(&claims.sub),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(row)
}

/// `POST /api/timesheets/decide` body.
#[derive(Debug, Deserialize)]
struct DecideBody {
    person: String,
    week: String,
    /// `approved` | `rejected`.
    decision: String,
    #[serde(default)]
    comment: Option<String>,
}

/// `POST /api/timesheets/decide` — an approver decides a submitted week.
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<DecideBody>,
) -> Result<Response> {
    if !is_approver(&claims) {
        return Err(forbidden("deciding a timesheet needs the approver right"));
    }
    let person = person_of(Some(&b.person), &claims.sub)?;
    if is_self(&person, &claims.sub) {
        return Err(forbidden("nobody decides their own timesheet"));
    }
    let approve = match b.decision.as_str() {
        "approved" => true,
        "rejected" => false,
        _ => return Err(unprocessable("decision must be approved or rejected")),
    };
    let monday = week_of(&b.week)?;
    let current = week_row(&ctx.db, &person, monday).await?;
    let next = rules::decide(
        current.as_ref().map(|r| r.status.as_str()),
        approve,
        b.comment.as_deref(),
    )
    .map_err(|r| match r {
        rules::Refusal::CommentRequired => unprocessable(&r.message()),
        _ => conflict(&r.message()),
    })?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = set_state(
        &txn,
        &person,
        monday,
        next,
        false,
        Some((&claims.sub, b.comment.as_deref())),
    )
    .await?;
    AuditModel::record(
        &txn,
        row.pid,
        &format!("timesheet_{next}"),
        Some(&claims.sub),
        Some(serde_json::json!({ "person": person, "week_start": monday, "comment": b.comment })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(row)
}

/// `GET /api/time-reports` query.
#[derive(Debug, Deserialize)]
struct ReportParams {
    group_by: String,
    #[serde(default)]
    from: Option<NaiveDate>,
    #[serde(default)]
    to: Option<NaiveDate>,
    #[serde(default)]
    plan: Option<Uuid>,
    #[serde(default)]
    person: Option<String>,
    #[serde(default)]
    format: Option<String>,
}

/// `GET /api/time-reports?group_by=&from=&to=&plan=&person=&format=` — time
/// grouped by `person` / `plan` / `category` / `billable` / `month`. Dates are
/// calendar dates; default window is the last 90 calendar days. Plans the caller cannot
/// read are omitted and counted.
#[debug_handler]
async fn time_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<ReportParams>,
) -> Result<Response> {
    let by = GroupBy::parse(&p.group_by).ok_or_else(|| {
        unprocessable("group_by must be person, plan, category, billable or month")
    })?;
    let to = p.to.unwrap_or_else(crate::tz::deployment_today);
    let from = p.from.unwrap_or_else(|| to - chrono::Duration::days(89));
    if from > to {
        return Err(unprocessable("from must not be after to"));
    }
    let mut q = time_entries::Entity::find()
        .filter(time_entries::Column::DeletedAt.is_null())
        .filter(time_entries::Column::SpentOn.gte(from))
        .filter(time_entries::Column::SpentOn.lte(to));
    if let Some(plan) = p.plan {
        q = q.filter(time_entries::Column::PlanPid.eq(plan));
    }
    if let Some(person) = p.person.as_deref() {
        q = q.filter(time_entries::Column::ActorRef.eq(person));
    }
    let rows = q
        .order_by_asc(time_entries::Column::Id)
        .limit(ROW_CAP + 1)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    // Entries of a private plan the caller is not in do not exist for them: not shown, not counted.
    let rows = scope.keep(rows);
    let truncated = rows.len() as u64 > ROW_CAP;
    let mut plan_pids: Vec<Uuid> = rows.iter().map(|e| e.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = readable_plans(&ctx, &caller, &plan_pids).await;
    let facts: Vec<Entry> = rows
        .iter()
        .take(usize::try_from(ROW_CAP).unwrap_or(usize::MAX))
        .filter(|e| readable.contains(&e.plan_pid))
        .map(fact)
        .collect();
    let omitted = rows
        .len()
        .min(usize::try_from(ROW_CAP).unwrap_or(usize::MAX))
        - facts.len();
    let groups = rules::report(&facts, by);
    if p.format.as_deref() == Some("csv") {
        let mut csv = String::from("group,entries,billable_minutes,non_billable_minutes,minutes\n");
        for g in &groups {
            let _ = writeln!(
                csv,
                "{},{},{},{},{}",
                crate::visibility::csv_field(&g.key),
                g.entries,
                g.split.billable,
                g.split.non_billable,
                g.minutes
            );
        }
        return Ok((
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/csv"),
            )],
            csv,
        )
            .into_response());
    }
    let split: rules::Split = groups.iter().fold(rules::Split::default(), |mut s, g| {
        s.billable += g.split.billable;
        s.non_billable += g.split.non_billable;
        s
    });
    format::json(serde_json::json!({
        "group_by": p.group_by,
        "from": from,
        "to": to,
        "groups": groups,
        "split": split,
        "total_minutes": (!groups.is_empty()).then(|| split.total()),
        "reason": groups.is_empty().then_some("no_entries"),
        "omitted_unreadable": omitted,
        "truncated": truncated,
    }))
}

/// The timesheet and time-report routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/timesheets", get(get_week))
        .add("/timesheets/submit", post(submit))
        .add("/timesheets/decide", post(decide))
        .add("/time-reports", get(time_report))
}

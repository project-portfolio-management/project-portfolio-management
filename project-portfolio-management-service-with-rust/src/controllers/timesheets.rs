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

use super::unprocessable;
use super::{conflict, forbidden};
use std::fmt::Write as _;

use crate::prelude::*;
use axum::http::{StatusCode, header};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::plan_access::readable_plans;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::db::Session;
use crate::models::_entities::{time_entries, timesheet_weeks};
use crate::models::audit_logs::Model as AuditModel;
use crate::notification_prefs::recipient_sub;
use crate::timesheets::{self as rules, Entry, GroupBy};

/// Rows a report or week reads at most.
const ROW_CAP: i64 = 20_000;

/// Whether the token may decide timesheets: `access=admin` or the `timesheet_approver` value of the `role` attribute.
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

/// Whether the person reference names the caller (compared on the user id, case-insensitively).
fn is_self(person: &str, sub: &str) -> bool {
    recipient_sub(person).is_some_and(|id| id.eq_ignore_ascii_case(sub))
}

/// Parses an ISO week string (such as `2026-W40`) to its Monday, or a `422` when it is not a valid week.
fn week_of(raw: &str) -> Result<NaiveDate> {
    rules::week_start(raw).ok_or_else(|| unprocessable("week must be an ISO week such as 2026-W40"))
}

/// One person's non-deleted time entries for the Monday-to-Sunday week starting at `monday`, oldest first, capped at `ROW_CAP`.
async fn week_entries(
    ctx: &AppContext,
    person: &str,
    monday: NaiveDate,
) -> Result<Vec<time_entries::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM time_entries
         WHERE actor_ref = $1 AND deleted_at IS NULL AND spent_on >= $2 AND spent_on <= $3
         ORDER BY spent_on
         LIMIT $4",
    )
    .bind(person)
    .bind(monday)
    .bind(monday + chrono::Duration::days(6))
    .bind(ROW_CAP)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Projects a stored time entry onto the plain fact the pure grid rules aggregate. Minutes stay whole minutes and `billable` is carried through unchanged.
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

/// Finds the stored submit/approve state row for a person's week, or `None` when the week was never submitted (an open week has no row).
async fn week_row(
    db: &mut impl Session,
    person: &str,
    monday: NaiveDate,
) -> Result<Option<timesheet_weeks::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM timesheet_weeks WHERE person_ref = $1 AND week_start = $2 LIMIT 1",
    )
    .bind(person)
    .bind(monday)
    .fetch_optional(db.ex())
    .await?)
}

/// Whether `person`'s week containing `date` is approved (and so locked).
pub(crate) async fn week_is_locked(
    ctx: &AppContext,
    person: &str,
    date: NaiveDate,
) -> Result<bool> {
    Ok(
        week_row(&mut crate::db::pool(ctx), person, rules::monday_of(date))
            .await?
            .is_some_and(|r| r.status == "approved"),
    )
}

/// `GET /api/timesheets` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct GridParams {
    /// Optional person URN (`person:` or `worker:`); absent means the caller's own week.
    #[serde(default)]
    person: Option<String>,
    /// ISO week such as `2026-W40`.
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
    // `omitted_unreadable` counts what the policy withholds, never a plan the caller may not know is there.
    let rows = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .keep(rows);
    let facts: Vec<Entry> = rows
        .iter()
        .filter(|e| readable.contains(&e.plan_pid))
        .map(fact)
        .collect();
    let omitted = rows.len() - facts.len();
    let state = week_row(&mut crate::db::pool(&ctx), &person, monday).await?;
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SubmitBody {
    /// Optional person URN; must name the caller, since only the owner may submit.
    #[serde(default)]
    person: Option<String>,
    /// ISO week such as `2026-W40`.
    week: String,
}

/// Creates or updates the state row for a week. `submitted` stamps the submission time and clears any earlier decision; `decider` (who, optional comment) stamps a decision. Callers run it inside the transaction that also writes the audit row.
async fn set_state(
    db: &mut impl Session,
    person: &str,
    monday: NaiveDate,
    status: &str,
    submitted: bool,
    decider: Option<(&str, Option<&str>)>,
) -> Result<timesheet_weeks::Model> {
    // Upsert: a week has no row until its first submission or decision.
    let existing = week_row(db, person, monday).await?;
    let now = chrono::Utc::now().fixed_offset();
    match existing {
        Some(row) => {
            // An update names only the columns it changes, as before SQ-1: the status; the submission stamp
            // and a cleared decision when `submitted`; the decision when `decider` (it wins over the clearing).
            let all = "UPDATE timesheet_weeks
                 SET status = $1, submitted_at = $2, decided_by = $3, decided_at = $4, decision_comment = $5
                 WHERE id = $6
                 RETURNING *";
            let query = match (submitted, decider) {
                (true, Some((by, comment))) => sqlx::query_as(all)
                    .bind(status)
                    .bind(now)
                    .bind(by)
                    .bind(now)
                    .bind(comment.map(|c| c.trim().to_string())),
                (false, Some((by, comment))) => sqlx::query_as(
                    "UPDATE timesheet_weeks
                     SET status = $1, decided_by = $2, decided_at = $3, decision_comment = $4
                     WHERE id = $5
                     RETURNING *",
                )
                .bind(status)
                .bind(by)
                .bind(now)
                .bind(comment.map(|c| c.trim().to_string())),
                (true, None) => sqlx::query_as(all)
                    .bind(status)
                    .bind(now)
                    .bind(None::<String>)
                    .bind(None::<chrono::DateTime<chrono::FixedOffset>>)
                    .bind(None::<String>),
                (false, None) => {
                    sqlx::query_as("UPDATE timesheet_weeks SET status = $1 WHERE id = $2 RETURNING *")
                        .bind(status)
                }
            };
            Ok(query.bind(row.id).fetch_one(db.ex()).await?)
        }
        None => Ok(sqlx::query_as(
            "INSERT INTO timesheet_weeks
                 (pid, person_ref, week_start, status, submitted_at, decided_by, decided_at, decision_comment)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(person)
        .bind(monday)
        .bind(status)
        .bind(submitted.then_some(now))
        .bind(decider.map(|(b, _)| b.to_string()))
        .bind(decider.map(|_| now))
        .bind(decider.and_then(|(_, c)| c.map(str::to_string)))
        .fetch_one(db.ex())
        .await?),
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
    // Separation of duties: submit is owner-only here, and `decide` refuses the owner.
    if !is_self(&person, &claims.sub) {
        return Err(forbidden(
            "a week can only be submitted by the person it belongs to",
        ));
    }
    let monday = week_of(&b.week)?;
    let entries = week_entries(&ctx, &person, monday).await?;
    let current = week_row(&mut crate::db::pool(&ctx), &person, monday).await?;
    let next =
        rules::submit(current.as_ref().map(|r| r.status.as_str()), entries.len()).map_err(|r| {
            match r {
                rules::Refusal::Empty => unprocessable(&r.message()),
                _ => conflict(&r.message()),
            }
        })?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row = set_state(&mut tx, &person, monday, next, true, None).await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "timesheet_submitted",
        Some(&claims.sub),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/timesheets/decide` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DecideBody {
    /// Person URN whose week is being decided; must not be the decider.
    person: String,
    /// ISO week such as `2026-W40`.
    week: String,
    /// `approved` | `rejected`.
    decision: String,
    /// Why, required when rejecting; optional for an approval.
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
    let current = week_row(&mut crate::db::pool(&ctx), &person, monday).await?;
    let next = rules::decide(
        current.as_ref().map(|r| r.status.as_str()),
        approve,
        b.comment.as_deref(),
    )
    .map_err(|r| match r {
        rules::Refusal::CommentRequired => unprocessable(&r.message()),
        _ => conflict(&r.message()),
    })?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row = set_state(
        &mut tx,
        &person,
        monday,
        next,
        false,
        Some((&claims.sub, b.comment.as_deref())),
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        &format!("timesheet_{next}"),
        Some(&claims.sub),
        Some(serde_json::json!({ "person": person, "week_start": monday, "comment": b.comment })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/time-reports` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReportParams {
    /// Grouping dimension: `person`, `plan`, `category`, `billable` or `month`.
    group_by: String,
    /// Inclusive start date; absent means 90 calendar days back from the end.
    #[serde(default)]
    from: Option<NaiveDate>,
    /// Inclusive end date; absent means today.
    #[serde(default)]
    to: Option<NaiveDate>,
    /// Restrict the report to one plan, by public id; absent covers every readable plan.
    #[serde(default)]
    plan: Option<Uuid>,
    /// Restrict the report to one person URN; absent covers everyone.
    #[serde(default)]
    person: Option<String>,
    /// Output format; `csv` returns a CSV body, otherwise JSON.
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
    let rows: Vec<time_entries::Model> = sqlx::query_as(
        "SELECT * FROM time_entries
         WHERE deleted_at IS NULL AND spent_on >= $1 AND spent_on <= $2
           AND ($3::uuid IS NULL OR plan_pid = $3)
           AND ($4::text IS NULL OR actor_ref = $4)
         ORDER BY id
         LIMIT $5",
    )
    .bind(from)
    .bind(to)
    .bind(p.plan)
    .bind(p.person.as_deref())
    .bind(ROW_CAP + 1)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    // Entries of a private plan the caller is not in do not exist for them: not shown, not counted.
    let rows = scope.keep(rows);
    let truncated = i64::try_from(rows.len()).unwrap_or(i64::MAX) > ROW_CAP;
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

//! The monthly rhythm of the director pack (DP-12): **a schedule drafts, the portfolio office reviews, a
//! person issues, and subscribers are told with a link.**
//!
//! * The schedule only ever drafts. It has no way to issue: the issue route needs a person's token
//!   (`access=admin` or `directorpack=issuer`), and a pack the schedule drafted needs a review first.
//! * It drafts the month that just ended, with the cut-off at that month's last day, so what it drafts
//!   covers its whole period. A draft made earlier (by hand, with an earlier cut-off) is marked **stale**.
//! * The sweep is claim-based, like the report schedules: the next date moves with a conditional update
//!   before the work, so a ticker and a cron cannot draft twice, and a missed run drafts once.
//! * A notice carries the period, the audience, the version and a link. It never carries the body.

use super::refuse;
use crate::prelude::*;
use axum::http::StatusCode;
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::director_packs::{
    any_of_period, caller_audiences, caller_power, find, insert_draft, need,
};
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::director_pack::{self as rules, Power};
use crate::models::_entities::{
    director_pack_schedules as schedules, director_pack_subscriptions as subs,
    director_packs as packs,
};

/// One schedule by pid, if there is one.
async fn find_schedule(ctx: &AppContext, pid: Uuid) -> Result<Option<schedules::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM director_pack_schedules WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/director-pack-schedules", get(list_schedules))
        .add("/director-pack-schedules", post(create_schedule))
        .add("/director-pack-schedules/sweep", post(sweep))
        .add("/director-pack-schedules/{pid}", put(update_schedule))
        .add("/director-packs/{pid}/review", post(review))
        .add("/director-pack-subscriptions", get(list_subscriptions))
        .add("/director-pack-subscriptions", put(set_subscription))
}

/// JSON view of a director-pack schedule row, including its next run date and the outcome of its last run.
fn schedule_json(s: &schedules::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": s.pid, "audience": s.audience, "day_of_month": s.day_of_month,
        "next_run_date": s.next_run_date, "enabled": s.enabled,
        "last_run_on": s.last_run_on, "last_result": s.last_result,
    })
}

/// Request body creating a monthly director-pack schedule.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewSchedule {
    /// Audience the drafted pack is for; must be a known audience or `org:<pid>` of an active organization.
    audience: String,
    /// Day of the month the schedule runs on; see `first_run` for how the next date is chosen.
    day_of_month: i32,
}

/// The first run date strictly after `today`, on `day` of a month (the same rule the report schedules use).
fn first_run(day: i32, today: NaiveDate) -> Option<NaiveDate> {
    use chrono::Datelike;
    let anchor = NaiveDate::from_ymd_opt(today.year(), today.month(), u32::try_from(day).ok()?)?;
    crate::report_schedules::next_after("monthly", anchor, today)
        .filter(|d| *d > today)
        .map(|d| {
            // next_after is strictly after `today`; if this month's day is still ahead it is this month.
            if anchor > today { anchor } else { d }
        })
}

/// `POST /api/director-pack-schedules` — draft the pack for an audience on a day of each month.
#[debug_handler]
async fn create_schedule(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewSchedule>,
) -> Result<Response> {
    let actor = need(&caller, Power::Editor)?;
    if !super::organizations::audience_valid(&ctx, &b.audience).await? {
        return Err(unprocessable(&format!(
            "audience must be one of {:?}, or org:<pid> of an active organization",
            rules::AUDIENCES
        )));
    }
    if !(1..=28).contains(&b.day_of_month) {
        return Err(unprocessable(
            "day_of_month must be 1 to 28, so that every month has it",
        ));
    }
    let today = crate::tz::deployment_today();
    let next = first_run(b.day_of_month, today).ok_or_else(|| unprocessable("no such day"))?;
    // The row and its audit row in one transaction (before SQ-1 batch 4 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    let row: schedules::Model = sqlx::query_as(
        "INSERT INTO director_pack_schedules (pid, audience, day_of_month, next_run_date, enabled, created_by)
         VALUES ($1, $2, $3, $4, TRUE, $5)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&b.audience)
    .bind(b.day_of_month)
    .bind(next)
    .bind(&actor)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| {
        if e.to_string().contains("dps_one_per_audience") {
            refuse(
                StatusCode::CONFLICT,
                "conflict",
                "that audience already has a schedule; change it instead",
            )
        } else {
            e.into()
        }
    })?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "director_pack_schedule_created",
        Some(&actor),
        Some(serde_json::json!({ "audience": b.audience, "day_of_month": b.day_of_month })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(schedule_json(&row))
}

/// `GET /api/director-pack-schedules`.
#[debug_handler]
async fn list_schedules(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    need(&caller, Power::Editor)?;
    let rows: Vec<schedules::Model> =
        sqlx::query_as("SELECT * FROM director_pack_schedules ORDER BY audience")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(
        serde_json::json!({ "schedules": rows.iter().map(schedule_json).collect::<Vec<_>>() }),
    )
}

/// Request body changing a schedule; absent fields are left as they are.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdateSchedule {
    /// Switch the schedule on or off.
    #[serde(default)]
    enabled: Option<bool>,
    /// Move the schedule to a different day of the month.
    #[serde(default)]
    day_of_month: Option<i32>,
}

/// `PUT /api/director-pack-schedules/{pid}` — switch it off or on, or move its day.
#[debug_handler]
async fn update_schedule(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<Uuid>,
    Json(b): Json<UpdateSchedule>,
) -> Result<Response> {
    let actor = need(&caller, Power::Editor)?;
    let row = find_schedule(&ctx, pid).await?.ok_or(Error::NotFound)?;
    // Only the fields given are written; with none, the row is read back unchanged (as the empty update did before
    // SQ-1).
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE director_pack_schedules SET ");
    let mut set = q.separated(", ");
    let mut any = false;
    if let Some(e) = b.enabled {
        set.push("enabled = ").push_bind_unseparated(e);
        any = true;
    }
    if let Some(d) = b.day_of_month {
        if !(1..=28).contains(&d) {
            return Err(unprocessable(
                "day_of_month must be 1 to 28, so that every month has it",
            ));
        }
        let next = first_run(d, crate::tz::deployment_today())
            .ok_or_else(|| unprocessable("no such day"))?;
        set.push("day_of_month = ").push_bind_unseparated(d);
        set.push("next_run_date = ").push_bind_unseparated(next);
        any = true;
    }
    q.push(" WHERE id = ")
        .push_bind(row.id)
        .push(" RETURNING *");
    // The change and its audit row in one transaction (before SQ-1 batch 4 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    let row: schedules::Model = if any {
        q.build_query_as().fetch_one(tx.conn()).await?
    } else {
        sqlx::query_as("SELECT * FROM director_pack_schedules WHERE id = $1 LIMIT 1")
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?
    };
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "director_pack_schedule_updated",
        Some(&actor),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(schedule_json(&row))
}

/// What a sweep did.
#[derive(Debug, Default)]
pub struct Swept {
    /// The drafts made.
    pub drafted: Vec<Uuid>,
    /// Schedules that were due but had nothing to draft (the pack for that month already exists).
    pub skipped: usize,
}

/// Draft, for every due schedule, the pack for the month that just ended. **Never issues.**
///
/// # Errors
///
/// When a database statement fails.
pub async fn run_due(ctx: &AppContext, today: NaiveDate) -> Result<Swept> {
    let due: Vec<schedules::Model> = sqlx::query_as(
        "SELECT * FROM director_pack_schedules WHERE enabled = TRUE AND next_run_date <= $1 ORDER BY id",
    )
    .bind(today)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let mut out = Swept::default();
    for s in due {
        // Claim first: only the sweep that moves the date does the work.
        let next = first_run(s.day_of_month, today).unwrap_or(today + chrono::Duration::days(28));
        let claimed = sqlx::query(
            "UPDATE director_pack_schedules SET next_run_date = $1, last_run_on = $2
             WHERE pid = $3 AND next_run_date = $4",
        )
        .bind(next)
        .bind(today)
        .bind(s.pid)
        .bind(s.next_run_date)
        .execute(crate::db::pool(ctx))
        .await?;
        if claimed.rows_affected() == 0 {
            continue;
        }
        let period = rules::previous_period(today);
        let result = match draft_scheduled(ctx, &period, &s.audience).await {
            Ok(Some(p)) => {
                out.drafted.push(p);
                format!("drafted {period}")
            }
            Ok(None) => {
                out.skipped += 1;
                format!("skipped {period}: a pack for it already exists")
            }
            Err(e) => format!("failed {period}: {e}"),
        };
        sqlx::query("UPDATE director_pack_schedules SET last_result = $1 WHERE pid = $2")
            .bind(result)
            .bind(s.pid)
            .execute(crate::db::pool(ctx))
            .await?;
    }
    Ok(out)
}

/// Draft the pack for a period and audience unless one already exists (returns None then). The cut-off is the last second of the period so the draft covers it all; the origin is marked scheduled so it needs a review before issue.
async fn draft_scheduled(ctx: &AppContext, period: &str, audience: &str) -> Result<Option<Uuid>> {
    let exists = any_of_period(ctx, period, audience).await?.is_some();
    if exists {
        return Ok(None);
    }
    let end = rules::period_end(period).ok_or_else(|| Error::string("bad period"))?;
    let cut_off = end
        .and_hms_opt(23, 59, 59)
        .map(|t| t.and_utc())
        .ok_or_else(|| Error::string("bad cut-off"))?;
    let row = insert_draft(
        ctx,
        "schedule",
        period,
        audience,
        1,
        None,
        cut_off,
        None,
        "director_pack_drafted_by_schedule",
    )
    .await?;
    sqlx::query(
        "UPDATE director_packs SET origin = 'scheduled' WHERE pid = $1 AND status = 'draft'",
    )
    .bind(row.pid)
    .execute(crate::db::pool(ctx))
    .await?;
    Ok(Some(row.pid))
}

/// `POST /api/director-pack-schedules/sweep` — run what is due now (a deployment can drive this from cron
/// instead of the ticker). It drafts; it never issues.
#[debug_handler]
async fn sweep(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    need(&caller, Power::Editor)?;
    let r = run_due(&ctx, crate::tz::deployment_today()).await?;
    format::json(serde_json::json!({ "drafted": r.drafted, "skipped": r.skipped, "issued": 0 }))
}

/// `POST /api/director-packs/{pid}/review` — the portfolio office has looked over this draft. Needed before a
/// pack the schedule drafted can be issued. Drafts only; once.
#[debug_handler]
async fn review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let actor = need(&caller, Power::Editor)?;
    let row = find(&ctx, &pid).await?;
    if row.status != "draft" {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "only a draft is reviewed; an issued pack is frozen",
        ));
    }
    // The review and its audit row in one transaction (before SQ-1 batch 4 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    let done = sqlx::query(
        "UPDATE director_packs SET reviewed_by = $1, reviewed_at = $2
         WHERE pid = $3 AND status = 'draft' AND reviewed_at IS NULL",
    )
    .bind(&actor)
    .bind(Utc::now())
    .bind(row.pid)
    .execute(tx.conn())
    .await?;
    if done.rows_affected() == 0 {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this draft has already been reviewed",
        ));
    }
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "director_pack_reviewed",
        Some(&actor),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid, "reviewed": true }))
}

/// Request body subscribing or unsubscribing a person to an audience notice.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SubscribeBody {
    /// Audience the subscription applies to.
    audience: String,
    /// True to subscribe, false to unsubscribe.
    subscribed: bool,
    /// An editor may subscribe someone else (a partner reader cannot subscribe themselves).
    #[serde(default)]
    person: Option<String>,
}

/// `PUT /api/director-pack-subscriptions` — be told, with a link, when a pack for an audience is issued.
#[debug_handler]
async fn set_subscription(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<SubscribeBody>,
) -> Result<Response> {
    if !super::organizations::audience_valid(&ctx, &b.audience).await? {
        return Err(unprocessable(&format!(
            "audience must be one of {:?}, or org:<pid> of an active organization",
            rules::AUDIENCES
        )));
    }
    let power = caller_power(&caller);
    let who = if let Some(p) = &b.person {
        {
            if power < Some(Power::Editor) {
                return Err(refuse(
                    StatusCode::FORBIDDEN,
                    "forbidden",
                    "subscribing someone else needs directorpack=editor or issuer",
                ));
            }
            let p = p.trim();
            if p.is_empty() {
                return Err(unprocessable("person must not be empty"));
            }
            if p.contains(':') {
                p.to_string()
            } else {
                format!("person:{p}")
            }
        }
    } else {
        {
            let me = caller
                .actor()
                .ok_or_else(|| unprocessable("sign in to subscribe"))?;
            if power.is_none() && !caller_audiences(&caller).iter().any(|a| a == &b.audience) {
                return Err(refuse(
                    StatusCode::FORBIDDEN,
                    "forbidden",
                    "you can subscribe to the audiences whose packs you may read",
                ));
            }
            format!("person:{me}")
        }
    };
    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM director_pack_subscriptions
         WHERE audience = $1 AND lower(person_ref) = lower($2)
         LIMIT 1",
    )
    .bind(&b.audience)
    .bind(&who)
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    // The change and its audit row in one transaction (before SQ-1 batch 4 they were two statements on the pool).
    let mut tx = crate::db::begin(&ctx).await?;
    match (existing, b.subscribed) {
        (None, true) => {
            sqlx::query(
                "INSERT INTO director_pack_subscriptions (pid, audience, person_ref)
                 VALUES ($1, $2, $3)
                 RETURNING id",
            )
            .bind(Uuid::new_v4())
            .bind(&b.audience)
            .bind(&who)
            .fetch_one(tx.conn())
            .await?;
        }
        (Some(id), false) => {
            sqlx::query("DELETE FROM director_pack_subscriptions WHERE id = $1")
                .bind(id)
                .execute(tx.conn())
                .await?;
        }
        _ => {}
    }
    AuditModel::record_on(
        tx.conn(),
        Uuid::nil(),
        "director_pack_subscription_set",
        caller.actor(),
        Some(serde_json::json!({ "audience": b.audience, "subscribed": b.subscribed })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "audience": b.audience, "subscribed": b.subscribed }))
}

/// `GET /api/director-pack-subscriptions` — your own.
#[debug_handler]
async fn list_subscriptions(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let me = caller.actor().map(|a| format!("person:{a}"));
    let rows: Vec<subs::Model> =
        sqlx::query_as("SELECT * FROM director_pack_subscriptions ORDER BY audience")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mine: Vec<&str> = rows
        .iter()
        .filter(|r| {
            me.as_deref()
                .is_some_and(|m| m.eq_ignore_ascii_case(&r.person_ref))
        })
        .map(|r| r.audience.as_str())
        .collect();
    format::json(serde_json::json!({ "subscribed_to": mine }))
}

/// Tell each subscriber of the pack's audience that it has been issued: its period, audience and version
/// and a link. **Never the body.**
///
/// # Errors
///
/// When a database read fails; a notice that cannot be made for one person is skipped.
pub(crate) async fn notify_issued(ctx: &AppContext, pack: &packs::Model) -> Result<usize> {
    let rows: Vec<subs::Model> =
        sqlx::query_as("SELECT * FROM director_pack_subscriptions WHERE audience = $1")
            .bind(&pack.audience)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let message = format!(
        "The {} director pack for {} (version {}) has been issued. Open it: /director-packs/{}",
        pack.period, pack.audience, pack.version, pack.pid
    );
    let mut told = 0;
    for r in rows {
        if crate::models::capabilities::notify_on(
            &mut crate::db::pool(ctx),
            &r.person_ref,
            "director_pack",
            pack.pid,
            "director_pack_issued",
            &message,
        )
        .await
        .ok()
        .flatten()
        .is_some()
        {
            told += 1;
        }
    }
    Ok(told)
}

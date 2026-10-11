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

use axum::http::StatusCode;
use chrono::{NaiveDate, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, sea_query::Expr};
use serde::Deserialize;
use uuid::Uuid;

use super::director_packs::{caller_audiences, caller_power, find, insert_draft, need};
use crate::auth::MaybeAuthUser;
use crate::director_pack::{self as rules, Power};
use crate::models::_entities::{
    director_pack_schedules as schedules, director_pack_subscriptions as subs,
    director_packs as packs,
};
use crate::models::audit_logs::Model as AuditModel;

fn refuse(status: StatusCode, code: &str, message: &str) -> Error {
    Error::CustomError(status, ErrorDetail::new(code, message))
}

fn unprocessable(message: &str) -> Error {
    refuse(
        StatusCode::UNPROCESSABLE_ENTITY,
        "unprocessable_entity",
        message,
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

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

fn schedule_json(s: &schedules::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": s.pid, "audience": s.audience, "day_of_month": s.day_of_month,
        "next_run_date": s.next_run_date, "enabled": s.enabled,
        "last_run_on": s.last_run_on, "last_result": s.last_result,
    })
}

#[derive(Debug, Deserialize)]
struct NewSchedule {
    audience: String,
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
    let today = Utc::now().date_naive();
    let next = first_run(b.day_of_month, today).ok_or_else(|| unprocessable("no such day"))?;
    let row = schedules::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        audience: ActiveValue::set(b.audience.clone()),
        day_of_month: ActiveValue::set(b.day_of_month),
        next_run_date: ActiveValue::set(next),
        enabled: ActiveValue::set(true),
        created_by: ActiveValue::set(Some(actor.clone())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("dps_one_per_audience") {
            refuse(
                StatusCode::CONFLICT,
                "conflict",
                "that audience already has a schedule; change it instead",
            )
        } else {
            db_err(&e)
        }
    })?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "director_pack_schedule_created",
        Some(&actor),
        Some(serde_json::json!({ "audience": b.audience, "day_of_month": b.day_of_month })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(schedule_json(&row))
}

/// `GET /api/director-pack-schedules`.
#[debug_handler]
async fn list_schedules(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    need(&caller, Power::Editor)?;
    let rows = schedules::Entity::find()
        .order_by_asc(schedules::Column::Audience)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(
        serde_json::json!({ "schedules": rows.iter().map(schedule_json).collect::<Vec<_>>() }),
    )
}

#[derive(Debug, Deserialize)]
struct UpdateSchedule {
    #[serde(default)]
    enabled: Option<bool>,
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
    let row = schedules::Entity::find()
        .filter(schedules::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let mut a: schedules::ActiveModel = row.into();
    if let Some(e) = b.enabled {
        a.enabled = ActiveValue::set(e);
    }
    if let Some(d) = b.day_of_month {
        if !(1..=28).contains(&d) {
            return Err(unprocessable(
                "day_of_month must be 1 to 28, so that every month has it",
            ));
        }
        a.day_of_month = ActiveValue::set(d);
        a.next_run_date = ActiveValue::set(
            first_run(d, Utc::now().date_naive()).ok_or_else(|| unprocessable("no such day"))?,
        );
    }
    let row = a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "director_pack_schedule_updated",
        Some(&actor),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
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
    let due = schedules::Entity::find()
        .filter(schedules::Column::Enabled.eq(true))
        .filter(schedules::Column::NextRunDate.lte(today))
        .order_by_asc(schedules::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut out = Swept::default();
    for s in due {
        // Claim first: only the sweep that moves the date does the work.
        let next = first_run(s.day_of_month, today).unwrap_or(today + chrono::Duration::days(28));
        let claimed = schedules::Entity::update_many()
            .col_expr(schedules::Column::NextRunDate, Expr::value(next))
            .col_expr(schedules::Column::LastRunOn, Expr::value(today))
            .filter(schedules::Column::Pid.eq(s.pid))
            .filter(schedules::Column::NextRunDate.eq(s.next_run_date))
            .exec(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?;
        if claimed.rows_affected == 0 {
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
        schedules::Entity::update_many()
            .col_expr(schedules::Column::LastResult, Expr::value(result))
            .filter(schedules::Column::Pid.eq(s.pid))
            .exec(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?;
    }
    Ok(out)
}

async fn draft_scheduled(ctx: &AppContext, period: &str, audience: &str) -> Result<Option<Uuid>> {
    let exists = packs::Entity::find()
        .filter(packs::Column::Period.eq(period))
        .filter(packs::Column::Audience.eq(audience))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .is_some();
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
    packs::Entity::update_many()
        .col_expr(packs::Column::Origin, Expr::value("scheduled"))
        .filter(packs::Column::Pid.eq(row.pid))
        .filter(packs::Column::Status.eq("draft"))
        .exec(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    Ok(Some(row.pid))
}

/// `POST /api/director-pack-schedules/sweep` — run what is due now (a deployment can drive this from cron
/// instead of the ticker). It drafts; it never issues.
#[debug_handler]
async fn sweep(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    need(&caller, Power::Editor)?;
    let r = run_due(&ctx, Utc::now().date_naive()).await?;
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
    let done = packs::Entity::update_many()
        .col_expr(packs::Column::ReviewedBy, Expr::value(actor.clone()))
        .col_expr(packs::Column::ReviewedAt, Expr::value(Utc::now()))
        .filter(packs::Column::Pid.eq(row.pid))
        .filter(packs::Column::Status.eq("draft"))
        .filter(packs::Column::ReviewedAt.is_null())
        .exec(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    if done.rows_affected == 0 {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this draft has already been reviewed",
        ));
    }
    AuditModel::record(
        &ctx.db,
        row.pid,
        "director_pack_reviewed",
        Some(&actor),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({ "pid": row.pid, "reviewed": true }))
}

#[derive(Debug, Deserialize)]
struct SubscribeBody {
    audience: String,
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
    let existing = subs::Entity::find()
        .filter(subs::Column::Audience.eq(b.audience.as_str()))
        .filter(Expr::cust_with_values(
            "lower(person_ref) = lower($1)",
            [who.clone()],
        ))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    match (existing, b.subscribed) {
        (None, true) => {
            subs::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                audience: ActiveValue::set(b.audience.clone()),
                person_ref: ActiveValue::set(who.clone()),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?;
        }
        (Some(e), false) => {
            subs::Entity::delete_by_id(e.id)
                .exec(&ctx.db)
                .await
                .map_err(|e| db_err(&e))?;
        }
        _ => {}
    }
    AuditModel::record(
        &ctx.db,
        Uuid::nil(),
        "director_pack_subscription_set",
        caller.actor(),
        Some(serde_json::json!({ "audience": b.audience, "subscribed": b.subscribed })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({ "audience": b.audience, "subscribed": b.subscribed }))
}

/// `GET /api/director-pack-subscriptions` — your own.
#[debug_handler]
async fn list_subscriptions(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let me = caller.actor().map(|a| format!("person:{a}"));
    let rows = subs::Entity::find()
        .order_by_asc(subs::Column::Audience)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
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
    let rows = subs::Entity::find()
        .filter(subs::Column::Audience.eq(pack.audience.as_str()))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let message = format!(
        "The {} director pack for {} (version {}) has been issued. Open it: /director-packs/{}",
        pack.period, pack.audience, pack.version, pack.pid
    );
    let mut told = 0;
    for r in rows {
        if crate::models::capabilities::notify(
            &ctx.db,
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

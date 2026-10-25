//! The weekly portfolio-exceptions digest (NC-17). Rules in [`crate::exceptions_digest`].
//!
//! * `GET /api/exceptions/digest`: the digest as the caller would see it (the plans they can read);
//! * [`send_weekly`]: what the `exceptions_digest` task runs. It builds the digest as the system identity, so a
//!   private plan or a plan under a handling mark is never in it, and sends it as an `exceptions_digest` notification to each
//!   person in `PROJECT_PORTFOLIO_MANAGEMENT_DIGEST_RECIPIENTS` (comma-separated `person:` references). The
//!   notification goes through the recipient's own sinks and preferences like any other. A recipient who already has
//!   one from this week (Monday to Sunday of the deployment's "today") is skipped, so running the task twice sends once.

use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde_json::json;
use uuid::Uuid;

use super::db_err;
use crate::auth::MaybeAuthUser;
use crate::exceptions_digest::{self as rules, Digest, Item, Kind};
use crate::models::_entities::{delivery_exceptions, escalations, notifications, plans};

/// The environment variable naming who receives the weekly digest.
pub const RECIPIENTS_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_DIGEST_RECIPIENTS";

/// The notification kind.
const KIND: &str = "exceptions_digest";

/// The red and amber tolerance dimensions in `pictures`, and the names of the plans they are on.
fn tolerance_items(pictures: &[serde_json::Value]) -> (Vec<Item>, HashMap<Uuid, String>) {
    let mut items = Vec::new();
    let mut names: HashMap<Uuid, String> = HashMap::new();
    for p in pictures {
        let (Some(pid), Some(name)) = (
            p["plan_pid"].as_str().and_then(|s| Uuid::parse_str(s).ok()),
            p["plan_name"].as_str(),
        ) else {
            continue;
        };
        names.insert(pid, name.to_string());
        for d in p["dimensions"].as_array().into_iter().flatten() {
            let severity = match d["status"].as_str() {
                Some("red") => 0,
                Some("amber") => 2,
                _ => continue,
            };
            items.push(Item {
                kind: Kind::Tolerance,
                plan_pid: pid,
                plan_name: name.to_string(),
                summary: format!(
                    "{} tolerance {} ({}% of the limit)",
                    d["dimension"].as_str().unwrap_or("?"),
                    d["status"].as_str().unwrap_or("?"),
                    d["usage_percent"]
                ),
                severity,
            });
        }
    }
    (items, names)
}

/// Build the digest for `caller`: every red or amber tolerance, open threshold exception and live escalation on a
/// plan they can read.
///
/// # Errors
///
/// When a database statement fails.
pub(crate) async fn gather(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<Digest> {
    let today = crate::tz::deployment_today();
    let (pictures, _) = super::tolerances::pictures(ctx, caller).await?;
    let (mut items, mut names) = tolerance_items(&pictures);

    // Threshold exceptions and escalations: only on plans the caller can read.
    let thresholds = delivery_exceptions::Entity::find()
        .filter(delivery_exceptions::Column::Status.eq("open"))
        .filter(delivery_exceptions::Column::ThresholdKind.is_not_null())
        .order_by_asc(delivery_exceptions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let live = escalations::Entity::find()
        .filter(escalations::Column::DeletedAt.is_null())
        .filter(escalations::Column::Status.is_in(["open", "acknowledged"]))
        .order_by_asc(escalations::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut wanted: Vec<Uuid> = thresholds.iter().map(|t| t.plan_pid).collect();
    wanted.extend(live.iter().map(|e| e.plan_pid));
    wanted.sort();
    wanted.dedup();
    let readable = super::plan_access::readable_plans(ctx, caller, &wanted).await;
    let missing: Vec<Uuid> = wanted
        .iter()
        .copied()
        .filter(|p| readable.contains(p) && !names.contains_key(p))
        .collect();
    if !missing.is_empty() {
        for plan in plans::Entity::find()
            .filter(plans::Column::Pid.is_in(missing))
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
        {
            names.insert(plan.pid, plan.name);
        }
    }
    for t in thresholds {
        if let (true, Some(name)) = (readable.contains(&t.plan_pid), names.get(&t.plan_pid)) {
            items.push(Item {
                kind: Kind::Threshold,
                plan_pid: t.plan_pid,
                plan_name: name.clone(),
                summary: format!(
                    "{} threshold crossed; decide by {}",
                    t.threshold_kind.as_deref().unwrap_or("?"),
                    t.decide_by
                        .map_or_else(|| "(no date)".to_string(), |d| d.to_string())
                ),
                severity: 1,
            });
        }
    }
    for e in live {
        if let (true, Some(name)) = (readable.contains(&e.plan_pid), names.get(&e.plan_pid)) {
            let overdue = crate::escalations::is_overdue(&e.status, e.respond_by, today);
            items.push(Item {
                kind: Kind::Escalation,
                plan_pid: e.plan_pid,
                plan_name: name.clone(),
                summary: format!(
                    "escalation \"{}\" is {}{}",
                    e.title,
                    e.status,
                    if overdue { " and overdue" } else { "" }
                ),
                severity: if overdue { 0 } else { 2 },
            });
        }
    }
    Ok(rules::build(items, today))
}

/// `GET /api/exceptions/digest` — the weekly digest as the caller would see it: every red or amber tolerance, every open threshold exception and every live escalation on a plan they can read, worst first, with counts over all of them and the message a recipient would be sent.
#[debug_handler]
async fn get_digest(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let digest = gather(&ctx, &caller).await?;
    format::json(json!({ "digest": digest, "message": rules::message(&digest) }))
}

/// Monday of the week `today` falls in.
fn week_start(today: NaiveDate) -> NaiveDate {
    today - chrono::Duration::days(i64::from(today.weekday().num_days_from_monday()))
}

/// What a run of [`send_weekly`] did.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Sent {
    /// Notifications created.
    pub sent: usize,
    /// Recipients skipped because they already had this week's.
    pub already: usize,
    /// Recipients skipped because their entry is not a `person:` or `worker:` reference with a user id.
    pub unaddressable: usize,
}

/// Send this week's digest to each configured recipient who has not had it.
///
/// # Errors
///
/// When a database statement fails.
pub async fn send_weekly(ctx: &AppContext, recipients: &[String]) -> Result<Sent> {
    let mut out = Sent::default();
    if recipients.is_empty() {
        return Ok(out);
    }
    let digest = gather(ctx, &super::tolerances::system_caller()).await?;
    let message = rules::message(&digest);
    let since = week_start(digest.as_of)
        .and_hms_opt(0, 0, 0)
        .map(|d| d.and_utc());
    for who in recipients {
        if crate::notification_prefs::recipient_sub(who).is_none() {
            out.unaddressable += 1;
            continue;
        }
        let mut q = notifications::Entity::find()
            .filter(notifications::Column::RecipientRef.eq(who.as_str()))
            .filter(notifications::Column::Kind.eq(KIND));
        if let Some(since) = since {
            q = q.filter(notifications::Column::CreatedAt.gte(since));
        }
        if q.one(&ctx.db).await.map_err(db_err)?.is_some() {
            out.already += 1;
            continue;
        }
        if crate::models::capabilities::notify(
            &ctx.db,
            who,
            "portfolio",
            Uuid::nil(),
            KIND,
            &message,
        )
        .await?
        .is_some()
        {
            out.sent += 1;
        }
    }
    Ok(out)
}

/// The recipients named in [`RECIPIENTS_ENV`].
#[must_use]
pub fn recipients() -> Vec<String> {
    std::env::var(RECIPIENTS_ENV)
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/exceptions/digest", get(get_digest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_week_starts_on_monday() {
        let monday = NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
        assert_eq!(week_start(monday), monday);
        let sunday = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
        assert_eq!(week_start(sunday), monday);
    }
}

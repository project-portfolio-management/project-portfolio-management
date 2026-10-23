//! `GET /api/director-pack/issues-and-changes` (IR-6): issues and change requests for the monthly director
//! pack, over the plans the caller can read. Private plans, handling marks and break-glass are applied by
//! `readable_plans`, the same filter every other cross-plan summary uses. When the caller can read no plan
//! the answer says `available: false` with the reason rather than a page of zeros.

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use crate::auth::MaybeAuthUser;
use crate::director_summary::{self as rules, ChangeLite, IssueLite};
use crate::models::_entities::{change_requests, issues};

#[derive(Deserialize)]
struct Params {
    /// Calendar days the "in the period" counts look back; 1 to 366, default 30.
    #[serde(default)]
    window_days: Option<i64>,
}

async fn summary(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<Params>,
) -> Result<Response> {
    let window = p.window_days.unwrap_or(30);
    if !(1..=366).contains(&window) {
        return Err(Error::CustomError(
            StatusCode::UNPROCESSABLE_ENTITY,
            ErrorDetail::new("invalid", "window_days must be 1 to 366".to_string()),
        ));
    }
    let issue_rows = issues::Entity::find()
        .filter(issues::Column::DeletedAt.is_null())
        .order_by_asc(issues::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let change_rows = change_requests::Entity::find()
        .filter(change_requests::Column::DeletedAt.is_null())
        .order_by_asc(change_requests::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut plans: Vec<Uuid> = issue_rows
        .iter()
        .map(|i| i.plan_pid)
        .chain(change_rows.iter().map(|c| c.plan_pid))
        .collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    let today = crate::tz::utc_today();
    let issue_lite: Vec<IssueLite> = issue_rows
        .iter()
        .filter(|i| readable.contains(&i.plan_pid))
        .map(|i| IssueLite {
            plan_pid: i.plan_pid.to_string(),
            title: i.title.clone(),
            severity: i.severity.clone(),
            status: i.status.clone(),
            raised_on: i.raised_on,
            resolved_on: i.resolved_on,
            due_date: i.due_date,
        })
        .collect();
    let change_lite: Vec<ChangeLite> = change_rows
        .iter()
        .filter(|c| readable.contains(&c.plan_pid))
        .map(|c| ChangeLite {
            plan_pid: c.plan_pid.to_string(),
            title: c.title.clone(),
            status: c.status.clone(),
            requested_on: c.requested_on,
            decided_on: c.decided_on,
        })
        .collect();
    let available = !readable.is_empty();
    format::json(serde_json::json!({
        "available": available,
        "reason": (!available).then_some("no plan with issues or change requests is visible to you"),
        "window_days": window,
        "as_of": today,
        "issues": rules::issues(&issue_lite, today, window),
        "change_requests": rules::changes(&change_lite, today, window),
    }))
}

/// The route.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/director-pack/issues-and-changes", get(summary))
}

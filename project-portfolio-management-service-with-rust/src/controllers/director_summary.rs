//! `GET /api/director-pack/issues-and-changes` (IR-6): issues and change requests for the monthly director
//! pack, over the plans the caller can read. Private plans, handling marks and break-glass are applied by
//! `readable_plans`, the same filter every other cross-plan summary uses. When the caller can read no plan
//! the answer says `available: false` with the reason rather than a page of zeros.

use axa::error::ErrorDetail;
use axa::prelude::*;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::director_summary::{self as rules, ChangeLite, IssueLite};
use crate::models::_entities::{change_requests, issues};

/// Query for `GET /api/director-pack/issues-and-changes`.
#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct Params {
    /// Calendar days the "in the period" counts look back; 1 to 366, default 30.
    #[serde(default)]
    window_days: Option<i64>,
}

/// `GET /api/director-pack/issues-and-changes`. Counts issues and change requests only over plans the caller may read; `window_days` outside 1 to 366 is refused with 422. When nothing is readable the answer says so rather than reporting zeros.
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
    let pool = crate::db::pool(&ctx);
    let issue_rows: Vec<issues::Model> =
        sqlx::query_as("SELECT * FROM issues WHERE deleted_at IS NULL ORDER BY id")
            .fetch_all(pool)
            .await?;
    let change_rows: Vec<change_requests::Model> =
        sqlx::query_as("SELECT * FROM change_requests WHERE deleted_at IS NULL ORDER BY id")
            .fetch_all(pool)
            .await?;
    let mut plans: Vec<Uuid> = issue_rows
        .iter()
        .map(|i| i.plan_pid)
        .chain(change_rows.iter().map(|c| c.plan_pid))
        .collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    let today = crate::tz::deployment_today();
    let issue_lite: Vec<IssueLite> = issue_rows
        .iter()
        .filter(|i| readable.contains(&i.plan_pid))
        .map(|i| IssueLite {
            plan_pid: i.plan_pid.to_string(),
            title: i.title.clone(),
            severity: i.severity.clone(),
            status: i.status.clone(),
            done: crate::issues::is_done(&i.status_category),
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

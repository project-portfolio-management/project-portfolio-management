//! `GET /api/plans/{pid}/people-impact` (FB-4, entity spec §8.5a): the workforce service's (WPM's) people impact
//! for one plan, asked under the caller's own token and passed through unchanged, labelled `source: "wpm"`. The
//! rules are in [`crate::wpm_bridge`]; this is the wiring. The private-plan guard applies as to any
//! `/api/plans/{pid}/…` route.

use loco_rs::prelude::*;
use serde::Deserialize;

use super::unprocessable_entity as unprocessable;
use crate::wpm_bridge as bridge;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/people-impact", get(show))
}

#[derive(Debug, Deserialize)]
struct Query {
    /// How many months, from this one, WPM looks for over-committed pool-months in (1 to 36; WPM's default when
    /// absent).
    months: Option<u32>,
}

/// `GET /api/plans/{pid}/people-impact[?months=N]` — the workforce service's people impact for the plan, passed through with `source: "wpm"`, or `unknown` with the reason.
#[debug_handler]
async fn show(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: axum::http::HeaderMap,
    axum::extract::Query(q): axum::extract::Query<Query>,
) -> Result<Response> {
    if q.months
        .is_some_and(|m| m == 0 || m > bridge::PEOPLE_IMPACT_MAX_MONTHS)
    {
        return Err(unprocessable(&format!(
            "months is 1 to {}",
            bridge::PEOPLE_IMPACT_MAX_MONTHS
        )));
    }
    let plan = super::find_live_plan(&ctx, &pid).await?;
    let target = plan.to_plan().ok().and_then(|p| p.target_date);
    let bearer = bridge::bearer_of(&headers);
    format::json(
        bridge::people_impact(plan.pid, target.as_deref(), q.months, bearer.as_deref()).await,
    )
}

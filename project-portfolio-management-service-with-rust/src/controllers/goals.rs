//! Goals under a plan (FR-12). A goal lives in the plan's `data.goals[]`
//! (the matchable payload), so each write rewrites the plan through the
//! same path as `PUT /api/plans/{pid}`: one transaction-free update, the
//! same validation, audit row and event. A `Goal` has no identifier, so a
//! goal is addressed by its zero-based position; a delete shifts the ones
//! after it down.
//!
//! Because a position moves when another goal is removed, every answer carries
//! an `ETag` of the goal list, and a replace or delete that sends `If-Match` is
//! refused with `412` when the list has changed since (CF-2): the caller re-reads
//! instead of editing the wrong goal. Without `If-Match` the write goes ahead as
//! before.

use axum::http::{HeaderMap, StatusCode, header};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use project_portfolio_management_matcher::Goal;
use serde::Serialize;

use super::plans::validate;
use crate::auth::MaybeAuthUser;
use crate::metrics::Metrics;
use crate::models::plans::Model as PlanModel;
use crate::streaming;

/// A goal and its position.
#[derive(Debug, Serialize)]
struct Indexed {
    index: usize,
    #[serde(flatten)]
    goal: Goal,
}

fn indexed(goals: Vec<Goal>) -> Vec<Indexed> {
    goals
        .into_iter()
        .enumerate()
        .map(|(index, goal)| Indexed { index, goal })
        .collect()
}

async fn load(
    ctx: &AppContext,
    pid: &str,
) -> Result<(PlanModel, project_portfolio_management_matcher::Plan)> {
    let model = PlanModel::find_by_pid(&ctx.db, pid)
        .await
        .map_err(super::model_not_found)?;
    let plan = model.to_plan().map_err(Error::Model)?;
    Ok((model, plan))
}

/// The goal list's entity tag: a digest of the goals as stored, so it changes
/// whenever a goal is added, edited, moved or removed.
fn goals_etag(goals: &[Goal]) -> String {
    let hex = crate::artifacts::sha256_hex(&serde_json::to_vec(goals).unwrap_or_default());
    format!("W/\"{}\"", &hex[..16])
}

/// The goals as JSON with their `ETag`.
fn with_etag(goals: Vec<Goal>) -> Result<Response> {
    let tag = goals_etag(&goals);
    let mut res = format::json(indexed(goals))?;
    if let Ok(v) = header::HeaderValue::from_str(&tag) {
        res.headers_mut().insert(header::ETAG, v);
    }
    Ok(res)
}

/// Refuse with `412` when the caller sent `If-Match` for a goal list that has changed.
fn check_if_match(headers: &HeaderMap, goals: &[Goal]) -> Result<()> {
    let Some(sent) = headers.get(header::IF_MATCH).and_then(|v| v.to_str().ok()) else {
        return Ok(());
    };
    let current = goals_etag(goals);
    let matches = sent.split(',').map(str::trim).any(|t| {
        t == "*" || t == current || t.trim_start_matches("W/") == current.trim_start_matches("W/")
    });
    if matches {
        Ok(())
    } else {
        Err(Error::CustomError(
            StatusCode::PRECONDITION_FAILED,
            ErrorDetail::new(
                "precondition_failed",
                "the plan's goals changed since you read them; read them again, then retry",
            ),
        ))
    }
}

/// Write the changed plan back and answer with the goals.
async fn save(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    model: PlanModel,
    plan: project_portfolio_management_matcher::Plan,
) -> Result<Response> {
    validate(&plan)?;
    crate::auth::authorize_record(
        caller,
        authentication_verifier::Action::Write,
        &crate::auth::plan_resource_attrs(model.stage.as_deref()),
    )
    .map_err(super::record_rejection)?;
    let updated = streaming::update_and_emit(&ctx.db, model, &plan, caller.actor()).await?;
    Metrics::global().plan_updated_total.inc();
    with_etag(updated.to_plan().map_err(Error::Model)?.goals)
}

fn no_such_goal() -> Error {
    Error::CustomError(
        StatusCode::NOT_FOUND,
        ErrorDetail::new("not_found", "the plan has no goal at that position"),
    )
}

/// `GET /api/plans/{pid}/goals`.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let (_, plan) = load(&ctx, &pid).await?;
    with_etag(plan.goals)
}

/// `POST /api/plans/{pid}/goals`: append a goal.
#[debug_handler]
async fn add(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(goal): Json<Goal>,
) -> Result<Response> {
    let (model, mut plan) = load(&ctx, &pid).await?;
    plan.goals.push(goal);
    save(&ctx, &caller, model, plan).await
}

/// `PUT /api/plans/{pid}/goals/{index}`: replace the goal at a position.
#[debug_handler]
async fn replace(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Path((pid, index)): Path<(String, usize)>,
    Json(goal): Json<Goal>,
) -> Result<Response> {
    let (model, mut plan) = load(&ctx, &pid).await?;
    check_if_match(&headers, &plan.goals)?;
    *plan.goals.get_mut(index).ok_or_else(no_such_goal)? = goal;
    save(&ctx, &caller, model, plan).await
}

/// `DELETE /api/plans/{pid}/goals/{index}` and its POST twin.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Path((pid, index)): Path<(String, usize)>,
) -> Result<Response> {
    let (model, mut plan) = load(&ctx, &pid).await?;
    check_if_match(&headers, &plan.goals)?;
    if index >= plan.goals.len() {
        return Err(no_such_goal());
    }
    plan.goals.remove(index);
    save(&ctx, &caller, model, plan).await
}

/// The goal routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/goals", get(list))
        .add("/plans/{pid}/goals", post(add))
        .add("/plans/{pid}/goals/{index}", put(replace))
        .add("/plans/{pid}/goals/{index}", delete(remove))
        .add("/plans/{pid}/goals/{index}/delete", post(remove))
}

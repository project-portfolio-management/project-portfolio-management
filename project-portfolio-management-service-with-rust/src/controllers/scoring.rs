//! WSJF and RICE (NC-6): the estimated inputs of two prioritization models, and each model's score with its
//! derivation. Rules in [`crate::scoring_models`]; the ranking by any model, the Smart Score included, is
//! `GET /api/prioritization?model=` in [`super::prioritization`].
//!
//! * `PUT /api/plans/{pid}/scoring-inputs`: set the inputs (all optional; the body replaces the earlier ones,
//!   audited with the before and after);
//! * `GET /api/plans/{pid}/scoring-inputs`: what is set;
//! * `GET /api/plans/{pid}/score?model=wsjf|rice`: the score, its formula, every component explained, and the
//!   inputs it is still waiting for (a score with any input missing is `null`, never a guess).

use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{Value, json};

use super::unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::scoring_inputs;
use crate::models::audit_logs::Model as AuditModel;
use crate::scoring_models::{self as rules, ModelInputs};

/// The inputs of a stored row.
pub(crate) fn inputs_of(row: &scoring_inputs::Model) -> ModelInputs {
    ModelInputs {
        business_value: row.business_value,
        time_criticality: row.time_criticality,
        risk_reduction: row.risk_reduction,
        job_size: row.job_size,
        reach: row.reach,
        impact: row.impact,
        confidence_percent: row.confidence_percent,
        effort: row.effort,
    }
}

/// A model's score by name, or `None` for a name that is not one of these two.
pub(crate) fn score_by_name(model: &str, inputs: &ModelInputs) -> Option<rules::ModelScore> {
    match model {
        "wsjf" => Some(rules::wsjf(inputs)),
        "rice" => Some(rules::rice(inputs)),
        _ => None,
    }
}

/// JSON view of a plan's scoring inputs. `set` is false when none were saved, in which case the inputs are all absent rather than zero.
fn view(plan: Uuid, row: Option<&scoring_inputs::Model>) -> Value {
    let inputs = row.map(inputs_of).unwrap_or_default();
    json!({
        "plan_pid": plan,
        "set": row.is_some(),
        "inputs": inputs,
        "set_by": row.and_then(|r| r.set_by.clone()),
        "updated_at": row.map(|r| r.updated_at),
    })
}

/// `PUT /api/plans/{pid}/scoring-inputs`.
#[debug_handler]
async fn put_inputs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(body): Json<ModelInputs>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    rules::validate(&body).map_err(|p| unprocessable(&p.join("; ")))?;
    let mut tx = crate::db::begin(&ctx).await?;
    let existing: Option<scoring_inputs::Model> =
        sqlx::query_as("SELECT * FROM scoring_inputs WHERE plan_pid = $1 LIMIT 1")
            .bind(plan.pid)
            .fetch_optional(tx.conn())
            .await?;
    let before = existing.as_ref().map(inputs_of);
    let who = caller.actor();
    // Every input and who set them, as `SeaORM`'s update and insert did; the update by id, where no row back is
    // an error, as before.
    let row: scoring_inputs::Model = if let Some(e) = existing {
        sqlx::query_as(
            "UPDATE scoring_inputs SET business_value = $1, time_criticality = $2, risk_reduction = $3,
                 job_size = $4, reach = $5, impact = $6, confidence_percent = $7, effort = $8, set_by = $9
             WHERE id = $10 RETURNING *",
        )
        .bind(body.business_value)
        .bind(body.time_criticality)
        .bind(body.risk_reduction)
        .bind(body.job_size)
        .bind(body.reach)
        .bind(body.impact)
        .bind(body.confidence_percent)
        .bind(body.effort)
        .bind(who)
        .bind(e.id)
        .fetch_one(tx.conn())
        .await?
    } else {
        sqlx::query_as(
            "INSERT INTO scoring_inputs (plan_pid, business_value, time_criticality, risk_reduction, job_size,
                 reach, impact, confidence_percent, effort, set_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
        )
        .bind(plan.pid)
        .bind(body.business_value)
        .bind(body.time_criticality)
        .bind(body.risk_reduction)
        .bind(body.job_size)
        .bind(body.reach)
        .bind(body.impact)
        .bind(body.confidence_percent)
        .bind(body.effort)
        .bind(who)
        .fetch_one(tx.conn())
        .await?
    };
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "scoring_inputs_set",
        caller.actor(),
        Some(json!({ "before": before, "after": body })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(plan.pid, Some(&row)))
}

/// `GET /api/plans/{pid}/scoring-inputs`.
#[debug_handler]
async fn get_inputs(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row: Option<scoring_inputs::Model> =
        sqlx::query_as("SELECT * FROM scoring_inputs WHERE plan_pid = $1 LIMIT 1")
            .bind(plan.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?;
    format::json(view(plan.pid, row.as_ref()))
}

/// `GET /api/plans/{pid}/score?model=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ScoreParams {
    /// `wsjf` or `rice`.
    model: String,
}

/// `GET /api/plans/{pid}/score?model=wsjf|rice`.
#[debug_handler]
async fn plan_score(
    State(ctx): State<AppContext>,
    _headers: HeaderMap,
    Path(pid): Path<String>,
    Query(q): Query<ScoreParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row: Option<scoring_inputs::Model> =
        sqlx::query_as("SELECT * FROM scoring_inputs WHERE plan_pid = $1 LIMIT 1")
            .bind(plan.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?;
    let inputs = row.as_ref().map(inputs_of).unwrap_or_default();
    let Some(score) = score_by_name(&q.model, &inputs) else {
        return Err(unprocessable(
            "model is wsjf or rice (the Smart Score is GET /api/plans/{pid}/smart-score)",
        ));
    };
    format::json(json!({
        "pid": plan.pid,
        "name": plan.name,
        "inputs_set": row.is_some(),
        "score": score,
    }))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/scoring-inputs", put(put_inputs))
        .add("/plans/{pid}/scoring-inputs", get(get_inputs))
        .add("/plans/{pid}/score", get(plan_score))
}

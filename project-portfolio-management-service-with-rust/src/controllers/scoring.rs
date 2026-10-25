//! WSJF and RICE (NC-6): the estimated inputs of two prioritization models, and each model's score with its
//! derivation. Rules in [`crate::scoring_models`]; the ranking by any model, the Smart Score included, is
//! `GET /api/prioritization?model=` in [`super::prioritization`].
//!
//! * `PUT /api/plans/{pid}/scoring-inputs`: set the inputs (all optional; the body replaces the earlier ones,
//!   audited with the before and after);
//! * `GET /api/plans/{pid}/scoring-inputs`: what is set;
//! * `GET /api/plans/{pid}/score?model=wsjf|rice`: the score, its formula, every component explained, and the
//!   inputs it is still waiting for (a score with any input missing is `null`, never a guess).

use axum::http::HeaderMap;
use loco_rs::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{db_err, unprocessable};
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
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let existing = scoring_inputs::Entity::find()
        .filter(scoring_inputs::Column::PlanPid.eq(plan.pid))
        .one(&txn)
        .await
        .map_err(db_err)?;
    let before = existing.as_ref().map(inputs_of);
    let who = caller.actor().map(str::to_string);
    let row = if let Some(e) = existing {
        let mut active: scoring_inputs::ActiveModel = e.into();
        apply(&mut active, &body, who);
        active.update(&txn).await.map_err(db_err)?
    } else {
        let mut active = scoring_inputs::ActiveModel {
            plan_pid: ActiveValue::set(plan.pid),
            ..Default::default()
        };
        apply(&mut active, &body, who);
        active.insert(&txn).await.map_err(db_err)?
    };
    AuditModel::record(
        &txn,
        plan.pid,
        "scoring_inputs_set",
        caller.actor(),
        Some(json!({ "before": before, "after": body })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(view(plan.pid, Some(&row)))
}

fn apply(row: &mut scoring_inputs::ActiveModel, inputs: &ModelInputs, who: Option<String>) {
    row.business_value = ActiveValue::set(inputs.business_value);
    row.time_criticality = ActiveValue::set(inputs.time_criticality);
    row.risk_reduction = ActiveValue::set(inputs.risk_reduction);
    row.job_size = ActiveValue::set(inputs.job_size);
    row.reach = ActiveValue::set(inputs.reach);
    row.impact = ActiveValue::set(inputs.impact);
    row.confidence_percent = ActiveValue::set(inputs.confidence_percent);
    row.effort = ActiveValue::set(inputs.effort);
    row.set_by = ActiveValue::set(who);
}

/// `GET /api/plans/{pid}/scoring-inputs`.
#[debug_handler]
async fn get_inputs(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = scoring_inputs::Entity::find()
        .filter(scoring_inputs::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
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
    let row = scoring_inputs::Entity::find()
        .filter(scoring_inputs::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
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

//! The lessons-learned register (NC-8). A plan records what it learned at a phase gate or at closure; at intake
//! the matcher compares a proposal with the plans that have lessons and shows theirs, with the match evidence.
//!
//! * `POST /api/plans/{pid}/lessons`: record a lesson (audited; append-only);
//! * `GET /api/plans/{pid}/lessons`: the plan's lessons, oldest first;
//! * `POST /api/plans/{pid}/lessons/{lpid}/withdraw`: withdraw one (kept in the audit trail, no longer shown);
//! * `GET /api/proposals/{pid}/lessons`: lessons from similar plans, best match first.

use loco_rs::prelude::*;
use project_portfolio_management_matcher::{MatchConfig, MatchingEngine, Plan};
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::plans::parse_kind_label;
use super::{db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::governance as rules;
use crate::models::_entities::lessons;
use crate::models::audit_logs::Model as AuditModel;
use crate::models::governance as gov;
use crate::models::plans::Model as PlanModel;
use crate::validation::MAX_TEXT_LEN;

/// What a lesson is about.
const CATEGORIES: &[&str] = &["went_well", "went_badly", "recommendation"];
/// The occasion that is not a gate.
const CLOSURE: &str = "closure";
/// The most plans an intake comparison considers (those with lessons, most recently written first).
const MAX_COMPARED: u64 = 500;

/// `POST /api/plans/{pid}/lessons` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LessonPayload {
    /// A gate (`g0_concept` … `g5_benefits`) or `closure`.
    occasion: String,
    /// `went_well`, `went_badly` or `recommendation`.
    category: String,
    /// The lesson in a sentence.
    summary: String,
    /// More, when a sentence is not enough.
    #[serde(default)]
    detail: Option<String>,
}

fn problems(p: &LessonPayload) -> Vec<String> {
    let mut out = Vec::new();
    if p.occasion != CLOSURE && !rules::is_token(rules::GATES, &p.occasion) {
        out.push(format!(
            "occasion is closure or one of {}",
            rules::GATES.join(", ")
        ));
    }
    if !CATEGORIES.contains(&p.category.as_str()) {
        out.push(format!("category is one of {}", CATEGORIES.join(", ")));
    }
    if p.summary.trim().is_empty() || p.summary.chars().count() > 300 {
        out.push("summary is required (up to 300 characters)".to_string());
    }
    if p.detail.as_deref().is_some_and(|d| d.len() > MAX_TEXT_LEN) {
        out.push("detail is too long".to_string());
    }
    out
}

/// `POST /api/plans/{pid}/lessons`.
#[debug_handler]
async fn record(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(body): Json<LessonPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let found = problems(&body);
    if !found.is_empty() {
        return Err(unprocessable(&found.join("; ")));
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = lessons::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        occasion: ActiveValue::set(body.occasion.clone()),
        category: ActiveValue::set(body.category.clone()),
        summary: ActiveValue::set(body.summary.trim().to_string()),
        detail: ActiveValue::set(body.detail.clone()),
        recorded_by: ActiveValue::set(caller.actor().map(str::to_string)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan.pid,
        "lesson_recorded",
        caller.actor(),
        Some(json!({ "lesson": row.pid, "occasion": row.occasion, "category": row.category })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(row)
}

/// `GET /api/plans/{pid}/lessons`.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = lessons::Entity::find()
        .filter(lessons::Column::PlanPid.eq(plan.pid))
        .filter(lessons::Column::DeletedAt.is_null())
        .order_by_asc(lessons::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows)
}

/// `POST /api/plans/{pid}/lessons/{lpid}/withdraw`.
#[debug_handler]
async fn withdraw(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, lpid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let lesson_pid = gov::parse_pid(&lpid)?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = lessons::Entity::find()
        .filter(lessons::Column::Pid.eq(lesson_pid))
        .filter(lessons::Column::PlanPid.eq(plan.pid))
        .filter(lessons::Column::DeletedAt.is_null())
        .one(&txn)
        .await
        .map_err(db_err)?
        .ok_or_else(|| Error::NotFound)?;
    let mut active: lessons::ActiveModel = row.clone().into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan.pid,
        "lesson_withdrawn",
        caller.actor(),
        Some(json!({ "lesson": row.pid, "summary": row.summary })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({ "withdrawn": row.pid }))
}

/// `GET /api/proposals/{pid}/lessons` — the proposal's title is matched, by the same engine as the duplicate
/// check, against the plans that have lessons; each probable match is returned with its score, confidence and
/// per-component breakdown (the evidence) and that plan's lessons. Plans the caller may not read are not
/// compared. Only plans with lessons are compared, whether or not they are closed.
#[debug_handler]
async fn for_proposal(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let proposal = gov::find_proposal(&ctx.db, gov::parse_pid(&pid)?).await?;
    let fence = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let all = lessons::Entity::find()
        .filter(lessons::Column::DeletedAt.is_null())
        .order_by_desc(lessons::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut by_plan: Vec<(Uuid, Vec<lessons::Model>)> = Vec::new();
    for l in all {
        if fence.plan(&l.plan_pid) {
            continue;
        }
        match by_plan.iter_mut().find(|(p, _)| *p == l.plan_pid) {
            Some((_, v)) => v.push(l),
            None => by_plan.push((l.plan_pid, vec![l])),
        }
    }
    by_plan.truncate(usize::try_from(MAX_COMPARED).unwrap_or(usize::MAX));

    let engine = MatchingEngine::new(MatchConfig::default());
    let query = Plan {
        kind: parse_kind_label(&proposal.kind_target),
        name: proposal.title.clone(),
        ..Plan::default()
    };
    let mut hits = Vec::new();
    for (plan_pid, mut rows) in by_plan {
        let Ok(plan) = PlanModel::find_by_pid(&ctx.db, &plan_pid.to_string()).await else {
            continue;
        };
        let result = engine.match_plans(&query, &plan.to_plan()?);
        if result.is_match {
            rows.sort_by_key(|l| l.id);
            hits.push((result.score, plan, result, rows));
        }
    }
    hits.sort_by(|a, b| b.0.total_cmp(&a.0));
    format::json(
        hits.into_iter()
            .map(|(score, plan, result, rows)| {
                json!({
                    "plan_pid": plan.pid,
                    "plan_name": plan.name,
                    "match": {
                        "score": score,
                        "confidence": result.confidence,
                        "breakdown": result.breakdown,
                    },
                    "lessons": rows,
                })
            })
            .collect::<Vec<_>>(),
    )
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/lessons", post(record))
        .add("/plans/{pid}/lessons", get(list))
        .add("/plans/{pid}/lessons/{lpid}/withdraw", post(withdraw))
        .add("/proposals/{pid}/lessons", get(for_proposal))
}

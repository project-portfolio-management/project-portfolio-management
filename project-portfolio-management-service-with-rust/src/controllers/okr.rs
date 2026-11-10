//! The **OKR engine** surface over [`crate::okr`] (entity spec §9.2b /
//! FR-27).
//!
//! Key results hang off an **objective** (which has a `pid`, a `period`
//! and weighted plan alignment), not off a plan's `goals[]` — those are
//! bare structs in the JSONB payload with no identifier, so a key result
//! bound to one would be orphaned by any reordering.
//!
//! **Every score is derived on read.** There is no stored progress
//! column, so recording a check-in corrects every figure that rests on
//! it rather than leaving a stale one behind.
//!
//! Three refusals worth knowing, because each is a number that would
//! otherwise look measured and be wrong:
//!
//! - An objective with no measurable key result is `unmeasured` and
//!   sorts **last**, never `0`.
//! - `start_value` cannot be updated: progress from a moving baseline
//!   is not progress.
//! - Confidence is recorded and **never** blended into a score.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    key_result_check_ins, key_results, objective_links, objectives, plans,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::okr as rules;

/// Parse a wire metric token; `None` means unsupported and is a `422` for
/// the caller.
fn parse_metric(raw: &str) -> Option<rules::Metric> {
    match raw {
        "number" => Some(rules::Metric::Number),
        "percent" => Some(rules::Metric::Percent),
        "currency" => Some(rules::Metric::Currency),
        "boolean" => Some(rules::Metric::Boolean),
        _ => None,
    }
}

/// Parse a wire direction token (`increase`, `decrease`, `maintain`);
/// `None` means unsupported.
fn parse_direction(raw: &str) -> Option<rules::Direction> {
    match raw {
        "increase" => Some(rules::Direction::Increase),
        "decrease" => Some(rules::Direction::Decrease),
        "maintain" => Some(rules::Direction::Maintain),
        _ => None,
    }
}

/// Turn a stored row into the pure layer's facts.
fn fact_of(row: &key_results::Model) -> Option<rules::KeyResultFact> {
    Some(rules::KeyResultFact {
        metric: parse_metric(&row.metric)?,
        direction: parse_direction(&row.direction)?,
        start_value: row.start_value,
        target_value: row.target_value,
        current_value: row.current_value,
        tolerance: row.tolerance,
        currency: row.currency.clone(),
    })
}

/// Load one live (not soft-deleted) objective by public id.
async fn find_objective(ctx: &AppContext, raw: &str) -> Result<objectives::Model> {
    let pid = super::parse_pid(raw)?;
    sqlx::query_as("SELECT * FROM objectives WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Load one live (not soft-deleted) key result by public id.
async fn find_key_result(ctx: &AppContext, raw: &str) -> Result<key_results::Model> {
    let pid = super::parse_pid(raw)?;
    sqlx::query_as("SELECT * FROM key_results WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Live key results for one objective.
async fn key_results_of(ctx: &AppContext, objective_pid: Uuid) -> Result<Vec<key_results::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM key_results WHERE objective_pid = $1 AND deleted_at IS NULL")
            .bind(objective_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// `POST /api/objectives/{pid}/key-results` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct KeyResultPayload {
    /// Key result title; required and capped.
    title: String,
    /// Wire metric kind: `number`, `percent`, `currency` or `boolean`.
    metric: String,
    /// Wire direction of travel: `increase`, `decrease` or `maintain`.
    direction: String,
    /// Baseline the progress is measured from; fixed at creation and never
    /// updated, because progress from a moving baseline is not progress.
    start_value: i64,
    /// Value that counts as done, in the metric's integer unit.
    target_value: i64,
    #[serde(default)]
    /// Band around the target; a `maintain` key result without it is
    /// unmeasurable rather than assumed exact.
    tolerance: Option<i64>,
    #[serde(default)]
    /// Display unit; informational only.
    unit: Option<String>,
    #[serde(default)]
    /// Currency code, required when the metric is `currency`.
    currency: Option<String>,
    #[serde(default)]
    /// Reference (not a copy) to the person who owns the key result.
    owner_ref: Option<String>,
}

/// `POST /api/objectives/{pid}/key-results` — declare a key result.
///
/// `current_value` starts **at the baseline**, not at zero: a key result
/// running from 100 defects down to 0 is 0% done at 100, and seeding it
/// at zero would report it complete on the day it was created.
#[debug_handler]
async fn create_key_result(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<KeyResultPayload>,
) -> Result<Response> {
    let objective = find_objective(&ctx, &pid).await?;

    let Some(metric) = parse_metric(&payload.metric) else {
        return Err(unprocessable(
            "metric must be number, percent, currency or boolean",
        ));
    };
    let Some(direction) = parse_direction(&payload.direction) else {
        return Err(unprocessable(
            "direction must be increase, decrease or maintain",
        ));
    };
    if payload.title.trim().is_empty() || payload.title.len() > crate::validation::MAX_TEXT_LEN {
        return Err(unprocessable("title is required (and capped)"));
    }
    if direction == rules::Direction::Maintain && payload.tolerance.is_none() {
        return Err(unprocessable(
            "a `maintain` key result needs a `tolerance`: a band is what the \
             direction means, and without one it is unmeasurable",
        ));
    }
    if payload.tolerance.is_some_and(|t| t < 0) {
        return Err(unprocessable("tolerance must not be negative"));
    }
    if metric == rules::Metric::Currency && payload.currency.is_none() {
        return Err(unprocessable(
            "a currency-valued key result must name its currency: this service \
             never converts between them",
        ));
    }
    // A key result with nowhere to travel cannot report progress. Say so
    // at write, rather than letting it read `unmeasured` for a quarter.
    if direction != rules::Direction::Maintain && payload.start_value == payload.target_value {
        return Err(unprocessable(
            "start_value equals target_value, so there is no distance to travel \
             and progress can never be computed",
        ));
    }

    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO key_results
             (pid, objective_pid, title, metric, direction, start_value, target_value, current_value,
              tolerance, unit, currency, owner_ref, due_date, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NULL, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(objective.pid)
    .bind(payload.title.trim())
    .bind(&payload.metric)
    .bind(&payload.direction)
    .bind(payload.start_value)
    .bind(payload.target_value)
    .bind(payload.start_value)
    .bind(payload.tolerance)
    .bind(&payload.unit)
    .bind(&payload.currency)
    .bind(&payload.owner_ref)
    .fetch_one(tx.conn())
    .await?;

    AuditModel::record_on(
        tx.conn(),
        objective.pid,
        "key_result_declared",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/objectives/{pid}/key-results` — with each one's derived
/// progress and, where absent, the reason.
#[debug_handler]
async fn list_key_results(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let objective = find_objective(&ctx, &pid).await?;
    let rows = key_results_of(&ctx, objective.pid).await?;
    let view: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            let (progress, reason) =
                fact_of(row).map_or((None, None), |fact| match rules::progress(&fact) {
                    Ok(value) => (Some(value), None),
                    Err(why) => (None, Some(why)),
                });
            serde_json::json!({
                "key_result": row,
                "progress_basis_points": progress,
                "unmeasurable": reason,
            })
        })
        .collect();
    format::json(serde_json::json!(view))
}

/// `POST /api/key-results/{pid}/check-ins` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CheckInPayload {
    /// The new measured value in the key result's unit.
    value: i64,
    #[serde(default)]
    /// Self-reported confidence, recorded beside the value and never blended
    /// into a score; absent means none was given.
    confidence: Option<i16>,
    #[serde(default)]
    /// Free-text context for the reading.
    note: Option<String>,
}

/// `POST /api/key-results/{pid}/check-ins` — record an observation.
///
/// Advances `current_value`; **never** `start_value`. Confidence is
/// stored and never blended into any score.
#[debug_handler]
async fn create_check_in(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<CheckInPayload>,
) -> Result<Response> {
    let key_result = find_key_result(&ctx, &pid).await?;
    if payload.confidence.is_some_and(|c| !(0..=100).contains(&c)) {
        return Err(unprocessable("confidence must be between 0 and 100"));
    }

    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO key_result_check_ins (pid, key_result_pid, value, confidence, note, actor)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(key_result.pid)
    .bind(payload.value)
    .bind(payload.confidence)
    .bind(&payload.note)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;

    // The observation moves the current value. The baseline is left
    // alone — deliberately, and there is no path here that touches it.
    let objective_pid = key_result.objective_pid;
    let updated: key_results::Model =
        sqlx::query_as("UPDATE key_results SET current_value = $1 WHERE id = $2 RETURNING *")
            .bind(payload.value)
            .bind(key_result.id)
            .fetch_one(tx.conn())
            .await?;

    AuditModel::record_on(
        tx.conn(),
        objective_pid,
        "key_result_checked_in",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;

    let progress = fact_of(&updated).and_then(|fact| rules::progress(&fact).ok());
    format::json(serde_json::json!({
        "pid": row_pid.to_string(),
        "current_value": updated.current_value,
        "progress_basis_points": progress,
        // Echoed so a reader can see it was recorded, and see that it
        // did not move the score.
        "confidence": payload.confidence,
    }))
}

/// `GET /api/key-results/{pid}/check-ins` — newest first.
#[debug_handler]
async fn list_check_ins(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let key_result = find_key_result(&ctx, &pid).await?;
    let rows: Vec<key_result_check_ins::Model> = sqlx::query_as(
        "SELECT * FROM key_result_check_ins WHERE key_result_pid = $1 ORDER BY observed_at DESC",
    )
    .bind(key_result.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(serde_json::json!(rows))
}

/// `GET /api/plans/{pid}/okr` — the plan's alignment-weighted score and
/// the objectives behind it.
///
/// An objective with no measurable key result reports `unmeasured` and
/// is excluded from the weighted mean — it must neither drag the plan
/// down nor silently lift it.
#[debug_handler]
async fn plan_okr(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> Result<Response> {
    let plan_pid = super::parse_pid(&pid)?;
    let plan: plans::Model =
        sqlx::query_as("SELECT * FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;

    let links: Vec<objective_links::Model> =
        sqlx::query_as("SELECT * FROM objective_links WHERE plan_pid = $1")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;

    // A deleted objective no longer counts toward the plan.
    let live: std::collections::HashSet<Uuid> =
        sqlx::query_as::<_, objectives::Model>("SELECT * FROM objectives WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(&ctx))
            .await?
            .into_iter()
            .map(|o| o.pid)
            .collect();
    let links: Vec<_> = links
        .into_iter()
        .filter(|l| live.contains(&l.objective_pid))
        .collect();
    let mut aligned = Vec::with_capacity(links.len());
    let mut detail = Vec::with_capacity(links.len());
    for link in &links {
        let rows = key_results_of(&ctx, link.objective_pid).await?;
        let facts: Vec<rules::KeyResultFact> = rows.iter().filter_map(fact_of).collect();
        let score = rules::objective_score(&facts);
        aligned.push(rules::AlignedObjective {
            score,
            weight: i64::from(link.weight),
        });
        detail.push(serde_json::json!({
            "objective_pid": link.objective_pid.to_string(),
            "weight": link.weight,
            "score_basis_points": score,
            "measured": score.is_some(),
            "key_results": rows.len(),
            "single_currency": rules::same_currency(&facts),
        }));
    }

    let score = rules::plan_score(&aligned);
    let body = serde_json::json!({
        "plan_pid": plan.pid.to_string(),
        "score_basis_points": score,
        // `measured: false` is not a zero — a plan whose objectives are
        // all unmeasured sorts last, it does not score bottom.
        "measured": score.is_some(),
        "objectives": detail,
        "as_of": chrono::Utc::now(),
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

// ───────────────────────── portfolio view, edit and delete ─────────────────────────

/// `GET /api/okrs?period=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct OkrParams {
    /// Only objectives of this period (an exact match).
    #[serde(default)]
    period: Option<String>,
}

/// `GET /api/okrs` — every objective with its key results, each one's derived
/// progress (or why it has none) and latest check-in, and the objective's score.
///
/// Objectives are ordered **least progressed first** (where attention is needed),
/// and an objective with no measurable key result is `unmeasured` and sorts
/// **last** — never as a zero. Confidence is shown beside each key result and is
/// never blended into a score.
#[debug_handler]
async fn list_okrs(
    State(ctx): State<AppContext>,
    axa::extract::Query(q): axa::extract::Query<OkrParams>,
) -> Result<Response> {
    let objs: Vec<objectives::Model> = sqlx::query_as(
        "SELECT * FROM objectives WHERE deleted_at IS NULL AND ($1::text IS NULL OR period = $1)
         ORDER BY id",
    )
    .bind(q.period.as_deref())
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut rows: Vec<(Option<i64>, serde_json::Value)> = Vec::with_capacity(objs.len());
    for o in &objs {
        let krs = key_results_of(&ctx, o.pid).await?;
        let pids: Vec<Uuid> = krs.iter().map(|k| k.pid).collect();
        let check_ins = if pids.is_empty() {
            Vec::new()
        } else {
            sqlx::query_as::<_, key_result_check_ins::Model>(
                "SELECT * FROM key_result_check_ins WHERE key_result_pid = ANY($1)
                 ORDER BY observed_at DESC, id DESC",
            )
            .bind(pids)
            .fetch_all(crate::db::pool(&ctx))
            .await?
        };
        let facts: Vec<rules::KeyResultFact> = krs.iter().filter_map(fact_of).collect();
        let score = rules::objective_score(&facts);
        let view: Vec<serde_json::Value> = krs
            .iter()
            .map(|row| {
                let (progress, reason) =
                    fact_of(row).map_or((None, None), |fact| match rules::progress(&fact) {
                        Ok(value) => (Some(value), None),
                        Err(why) => (None, Some(why)),
                    });
                let latest = check_ins.iter().find(|c| c.key_result_pid == row.pid);
                serde_json::json!({
                    "key_result": row,
                    "progress_basis_points": progress,
                    "unmeasurable": reason,
                    "latest_check_in": latest.map(|c| serde_json::json!({
                        "observed_at": c.observed_at, "value": c.value,
                        "confidence": c.confidence, "note": c.note })),
                })
            })
            .collect();
        rows.push((
            score,
            serde_json::json!({
                "objective": o,
                "score_basis_points": score,
                "measured": score.is_some(),
                "key_results": view,
                "single_currency": rules::same_currency(&facts),
            }),
        ));
    }
    // Measured, least progressed first; unmeasured last. Stable on creation order.
    rows.sort_by_key(|(score, _)| (score.is_none(), score.unwrap_or(0)));
    format::json(serde_json::json!({
        "period": q.period,
        "objectives": rows.into_iter().map(|(_, v)| v).collect::<Vec<_>>(),
        "as_of": chrono::Utc::now(),
        "note": "Scores are derived on read. An objective with no measurable key result is unmeasured and sorts last, never as zero. Confidence is recorded beside each key result and never blended into a score.",
    }))
}

/// `PUT /api/objectives/{pid}` body: replaces title, description and period.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ObjectiveUpdate {
    /// New objective title.
    title: String,
    #[serde(default)]
    /// New description; absent clears or leaves it unset.
    description: Option<String>,
    #[serde(default)]
    /// Planning period label (for example a quarter) the objective belongs to.
    period: Option<String>,
}

/// `PUT /api/objectives/{pid}` — edit an objective's title, description, period.
#[debug_handler]
async fn update_objective(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<ObjectiveUpdate>,
) -> Result<Response> {
    let objective = find_objective(&ctx, &pid).await?;
    if b.title.trim().is_empty() || b.title.len() > crate::validation::MAX_TEXT_LEN {
        return Err(unprocessable("title is required (and capped)"));
    }
    let was = serde_json::json!({ "title": objective.title, "description": objective.description,
                                   "period": objective.period });
    let mut tx = crate::db::begin(&ctx).await?;
    let row: objectives::Model = sqlx::query_as(
        "UPDATE objectives SET title = $1, description = $2, period = $3 WHERE id = $4 RETURNING *",
    )
    .bind(b.title.trim())
    .bind(&b.description)
    .bind(&b.period)
    .bind(objective.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "objective_updated",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": { "title": row.title,
            "description": row.description, "period": row.period } })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `DELETE /api/objectives/{pid}` — soft-delete an objective. Its key results go
/// with it (they are read through it), and plans stop counting it.
#[debug_handler]
async fn delete_objective(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let objective = find_objective(&ctx, &pid).await?;
    let row_pid = objective.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE objectives SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::Utc::now().fixed_offset())
        .bind(objective.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "objective_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `PUT /api/key-results/{pid}` body. The baseline (`start_value`), `metric`,
/// `direction` and `currency` are **not** editable — progress measured from a
/// moving baseline, or a changed unit, is not progress — and sending one is
/// refused.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct KeyResultUpdate {
    /// New key result title.
    title: String,
    /// New target value, in the metric's unit.
    target_value: i64,
    #[serde(default)]
    /// New tolerance; absent means none.
    tolerance: Option<i64>,
    #[serde(default)]
    /// New display unit.
    unit: Option<String>,
    #[serde(default)]
    /// Reference to the (possibly new) owner.
    owner_ref: Option<String>,
    #[serde(default)]
    /// Optional due date for the key result.
    due_date: Option<chrono::NaiveDate>,
}

/// `PUT /api/key-results/{pid}` — edit a key result's title, target, tolerance,
/// unit, owner and due date.
#[debug_handler]
async fn update_key_result(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<KeyResultUpdate>,
) -> Result<Response> {
    let kr = find_key_result(&ctx, &pid).await?;
    if b.title.trim().is_empty() || b.title.len() > crate::validation::MAX_TEXT_LEN {
        return Err(unprocessable("title is required (and capped)"));
    }
    if b.tolerance.is_some_and(|t| t < 0) {
        return Err(unprocessable("tolerance must not be negative"));
    }
    let maintain = parse_direction(&kr.direction) == Some(rules::Direction::Maintain);
    if maintain && b.tolerance.is_none() {
        return Err(unprocessable("a `maintain` key result needs a `tolerance`"));
    }
    if !maintain && b.target_value == kr.start_value {
        return Err(unprocessable(
            "target_value equals the baseline, so there would be no distance to travel",
        ));
    }
    let was = serde_json::json!({ "title": kr.title, "target_value": kr.target_value,
        "tolerance": kr.tolerance, "unit": kr.unit, "owner_ref": kr.owner_ref, "due_date": kr.due_date });
    let mut tx = crate::db::begin(&ctx).await?;
    let row: key_results::Model = sqlx::query_as(
        "UPDATE key_results
         SET title = $1, target_value = $2, tolerance = $3, unit = $4, owner_ref = $5, due_date = $6
         WHERE id = $7
         RETURNING *",
    )
    .bind(b.title.trim())
    .bind(b.target_value)
    .bind(b.tolerance)
    .bind(&b.unit)
    .bind(&b.owner_ref)
    .bind(b.due_date)
    .bind(kr.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "key_result_updated",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": { "title": row.title,
            "target_value": row.target_value, "tolerance": row.tolerance, "unit": row.unit,
            "owner_ref": row.owner_ref, "due_date": row.due_date } })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `DELETE /api/key-results/{pid}` — soft-delete a key result. Scores that rest
/// on it are recomputed without it.
#[debug_handler]
async fn delete_key_result(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let kr = find_key_result(&ctx, &pid).await?;
    let row_pid = kr.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE key_results SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::Utc::now().fixed_offset())
        .bind(kr.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "key_result_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// The OKR routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/objectives/{pid}/key-results", post(create_key_result))
        .add("/objectives/{pid}/key-results", get(list_key_results))
        .add("/key-results/{pid}/check-ins", post(create_check_in))
        .add("/key-results/{pid}/check-ins", get(list_check_ins))
        .add("/plans/{pid}/okr", get(plan_okr))
        .add("/okrs", get(list_okrs))
        .add("/objectives/{pid}", put(update_objective))
        .add("/objectives/{pid}", delete(delete_objective))
        .add("/key-results/{pid}", put(update_key_result))
        .add("/key-results/{pid}", delete(delete_key_result))
}

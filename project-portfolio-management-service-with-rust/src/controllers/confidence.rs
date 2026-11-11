//! Delivery confidence (DP-2), the exceptions it opens (DP-3) and strategic aims with the portfolio
//! map (DP-4). Rules in [`crate::delivery_confidence`].
//!
//! A stated confidence is **append-only** and sits beside the derived RAG; it is never blended with it.
//! Setting `amber_red` or `red` opens an exception and starts a clock; a worsening while one is open
//! is recorded without restarting it; only a recorded decision closes it. The map and the exception
//! lists are built here once and used both by the live routes (for the plans the caller may read) and by
//! the director pack (for plans that are neither private nor marked).

use super::refuse;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::prelude::*;
use axum::http::StatusCode;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::{MaybeAuthUser, require_auth};
use crate::delivery_confidence::{self as rules, Assessment, Parts};
use crate::models::_entities::{
    budget_lines, delivery_confidence_assessments as assessments,
    delivery_exceptions as exceptions, objective_links, objectives, plans, risks,
};
use crate::models::audit_logs::Model as AuditModel;

/// Whether the caller carries administrator access, as decided by the shared plan-access rule.
fn is_admin(caller: &MaybeAuthUser) -> bool {
    super::plan_access::is_admin(caller.claims())
}

/// Whether the caller holds `confidence=independent` (or is an administrator): only they may record an
/// independent assessment. With login off everyone is trusted, as for every other write.
fn may_assess_independently(caller: &MaybeAuthUser) -> bool {
    if !require_auth() {
        return true;
    }
    is_admin(caller)
        || caller
            .claims()
            .and_then(|c| c.attrs.get("confidence"))
            .is_some_and(|v| v.iter().any(|x| x == "independent"))
}

// ---------------------------------------------------------------------------------------------------
// DP-2: delivery confidence
// ---------------------------------------------------------------------------------------------------

/// Renders one stated assessment for the API; `assessor` is the recorded actor string, not a resolved person.
fn assessment_json(a: &assessments::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": a.pid, "plan_pid": a.plan_pid, "value": a.value, "kind": a.kind,
        "rationale": a.rationale, "assessor": a.assessor, "assessed_on": a.assessed_on,
    })
}

/// `PUT /api/plans/{pid}/delivery-confidence` body. The stated value is validated by `rules::assessment_problems`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AssessBody {
    /// Stated confidence value (for example `green`, `amber_red`, `red`); the allowed tokens live in [`crate::delivery_confidence`].
    value: String,
    /// Either `self` or `independent`; only `independent` needs `confidence=independent` or admin access.
    kind: String,
    /// Why the assessor holds this view; required so the value is never a bare colour.
    rationale: String,
    /// Defaults to today; not in the future.
    #[serde(default)]
    assessed_on: Option<NaiveDate>,
}

/// Renders one exception with its completeness and breach state. A closed exception is judged at its own closing time and an open one against `now`; `window` is the allowed hours to complete it.
fn exception_json(e: &exceptions::Model, now: DateTime<Utc>, window: i64) -> serde_json::Value {
    let parts = Parts {
        cause: e.cause.as_deref(),
        decision_needed: e.decision_needed.as_deref(),
        owner_ref: e.owner_ref.as_deref(),
        decide_by: e.decide_by,
    };
    let opened: DateTime<Utc> = e.opened_at.into();
    let completed: Option<DateTime<Utc>> = e.completed_at.map(Into::into);
    let closed: Option<DateTime<Utc>> = e.closed_at.map(Into::into);
    // A closed exception is judged at its completion; an open one by the clock now.
    let breach = rules::breached(opened, completed, closed.unwrap_or(now), window);
    serde_json::json!({
        "pid": e.pid, "plan_pid": e.plan_pid, "status": e.status, "opened_at": e.opened_at,
        "opened_value": e.opened_value, "worst_value": e.worst_value, "worsened_count": e.worsened_count,
        "cause": e.cause, "decision_needed": e.decision_needed, "owner_ref": e.owner_ref,
        "owner_role": e.owner_role, "decide_by": e.decide_by, "completed_at": e.completed_at,
        "complete": rules::missing(&parts).is_empty(), "missing": rules::missing(&parts),
        "breached": breach, "window_hours": window,
        "hours_open": (closed.unwrap_or(now) - opened).num_hours(),
        "closed_at": e.closed_at, "decision": e.decision, "decided_by": e.decided_by,
    })
}

/// `PUT /api/plans/{pid}/delivery-confidence` — append a stated assessment (it never replaces one).
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn set_confidence(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<AssessBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let problems = rules::assessment_problems(&b.value, &b.kind, &b.rationale);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let today = crate::tz::deployment_today();
    let on = b.assessed_on.unwrap_or(today);
    if on > today {
        return Err(unprocessable("assessed_on must not be in the future"));
    }
    if b.kind == "independent" && !may_assess_independently(&caller) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "an independent assessment needs access=admin or confidence=independent",
        ));
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let mut tx = crate::db::begin(&ctx).await?;
    let previous: Option<assessments::Model> = sqlx::query_as(
        "SELECT * FROM delivery_confidence_assessments WHERE plan_pid = $1 ORDER BY id DESC LIMIT 1",
    )
    .bind(plan.pid)
    .fetch_optional(tx.conn())
    .await?;
    let row: assessments::Model = sqlx::query_as(
        "INSERT INTO delivery_confidence_assessments
             (pid, plan_pid, value, kind, rationale, assessor, assessed_on)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(&b.value)
    .bind(&b.kind)
    .bind(b.rationale.trim())
    .bind(&actor)
    .bind(on)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "delivery_confidence_set",
        Some(&actor),
        Some(serde_json::json!({
            "plan_pid": plan.pid, "value": b.value, "kind": b.kind,
            "previous": previous.as_ref().map(|p| p.value.clone()),
        })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    // DP-3: reaching amber-red or red opens an exception; a worsening while one is open is recorded and
    // does not restart its clock; improving never closes it (only a decision does).
    let mut exception = None;
    if rules::opens_exception(&b.value) {
        let open: Option<exceptions::Model> = sqlx::query_as(
            "SELECT * FROM delivery_exceptions WHERE plan_pid = $1 AND status = 'open' LIMIT 1",
        )
        .bind(plan.pid)
        .fetch_optional(tx.conn())
        .await?;
        let now = Utc::now();
        exception = Some(if let Some(open) = open {
            if rules::worsens(&open.worst_value, &b.value) {
                let updated: exceptions::Model = sqlx::query_as(
                    "UPDATE delivery_exceptions SET worst_value = $1, worsened_count = $2
                     WHERE id = $3
                     RETURNING *",
                )
                .bind(&b.value)
                .bind(open.worsened_count + 1)
                .bind(open.id)
                .fetch_one(tx.conn())
                .await?;
                AuditModel::record_on(
                    tx.conn(),
                    updated.pid,
                    "delivery_exception_worsened",
                    Some(&actor),
                    Some(serde_json::json!({ "from": open.worst_value, "to": b.value })),
                )
                .await
                .map_err(|e| Error::string(&e.to_string()))?;
                updated
            } else {
                open
            }
        } else {
            let opened: exceptions::Model = sqlx::query_as(
                "INSERT INTO delivery_exceptions
                     (pid, plan_pid, status, opened_at, opened_by, opened_value, worst_value, worsened_count)
                 VALUES ($1, $2, 'open', $3, $4, $5, $5, 0)
                 RETURNING *",
            )
            .bind(Uuid::new_v4())
            .bind(plan.pid)
            .bind(now)
            .bind(&actor)
            .bind(&b.value)
            .fetch_one(tx.conn())
            .await?;
            AuditModel::record_on(
                tx.conn(),
                opened.pid,
                "delivery_exception_opened",
                Some(&actor),
                Some(serde_json::json!({ "plan_pid": plan.pid, "value": b.value })),
            )
            .await
            .map_err(|e| Error::string(&e.to_string()))?;
            opened
        });
    }
    tx.commit().await?;
    format::json(serde_json::json!({
        "assessment": assessment_json(&row),
        "exception": exception.map(|e| exception_json(&e, Utc::now(), rules::window_hours())),
    }))
}

/// `GET /api/plans/{pid}/delivery-confidence` — the whole history, oldest first, and what the map shows.
#[debug_handler]
async fn get_confidence(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows: Vec<assessments::Model> = sqlx::query_as(
        "SELECT * FROM delivery_confidence_assessments WHERE plan_pid = $1 ORDER BY id",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let history: Vec<Assessment> = rows.iter().map(lite).collect();
    let period = Utc::now().format("%Y-%m").to_string();
    let shown = rules::shown(&history).map(|s| {
        serde_json::json!({
            "value": s.value, "kind": s.kind, "assessed_on": s.assessed_on,
            "stale": rules::is_stale(s.assessed_on, &period),
            "marked_self": s.kind == "self",
        })
    });
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "shown": shown.clone().unwrap_or_else(|| serde_json::json!({ "value": "not_assessed" })),
        "history": rows.iter().map(assessment_json).collect::<Vec<_>>(),
        "note": "Stated by a person and kept beside the derived RAG, never blended with it. No assessment is not_assessed, not green. The history is append-only.",
    }))
}

/// Projects a stored assessment row onto the small value the pure rules operate on.
fn lite(a: &assessments::Model) -> Assessment {
    Assessment {
        value: a.value.clone(),
        kind: a.kind.clone(),
        assessed_on: a.assessed_on,
    }
}

// ---------------------------------------------------------------------------------------------------
// DP-3: exceptions
// ---------------------------------------------------------------------------------------------------

/// Query for `GET /api/exceptions`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ExceptionQuery {
    /// Optional filter, `open` or `closed`; anything else is a `422`.
    #[serde(default)]
    status: Option<String>,
    /// Optional filter on the computed breach flag; absent returns both.
    #[serde(default)]
    breached: Option<bool>,
}

/// `GET /api/exceptions[?status&breached]` — across the plans you can read.
#[debug_handler]
async fn list_exceptions(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ExceptionQuery>,
) -> Result<Response> {
    if q.status
        .as_deref()
        .is_some_and(|s| !matches!(s, "open" | "closed"))
    {
        return Err(unprocessable("status must be open or closed"));
    }
    let rows: Vec<exceptions::Model> =
        sqlx::query_as("SELECT * FROM delivery_exceptions ORDER BY opened_at DESC LIMIT 1000")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut plans: Vec<Uuid> = rows.iter().map(|r| r.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    let (now, window) = (Utc::now(), rules::window_hours());
    let shown: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| readable.contains(&r.plan_pid))
        .filter(|r| q.status.as_ref().is_none_or(|s| &r.status == s))
        .map(|r| exception_json(r, now, window))
        .filter(|v| q.breached.is_none_or(|b| v["breached"] == b))
        .collect();
    format::json(serde_json::json!({ "exceptions": shown, "window_hours": window }))
}

/// Loads an exception by public id, returning `404` both when it does not exist and when its plan is hidden from the caller, so existence is not leaked.
pub(crate) async fn find_exception(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<exceptions::Model> {
    let pid = super::parse_pid(pid)?;
    let row = exception_by_pid(ctx, pid).await?.ok_or(Error::NotFound)?;
    // The same answer as an exception that does not exist when its plan is hidden from the caller.
    if !super::plan_access::readable_plans(ctx, caller, &[row.plan_pid])
        .await
        .contains(&row.plan_pid)
    {
        return Err(Error::NotFound);
    }
    Ok(row)
}

/// One exception by pid, if there is one.
async fn exception_by_pid(ctx: &AppContext, pid: Uuid) -> Result<Option<exceptions::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM delivery_exceptions WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// `PUT /api/exceptions/{pid}` body; every field is optional and an omitted one keeps its stored value.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CompleteBody {
    /// What caused the exception; absent keeps the stored cause.
    #[serde(default)]
    cause: Option<String>,
    /// The decision someone must make to close the exception.
    #[serde(default)]
    decision_needed: Option<String>,
    /// Owner as a person reference. Packs show only the role, never this reference.
    #[serde(default)]
    owner_ref: Option<String>,
    /// The owner's role, which is what a pack shows instead of the person.
    #[serde(default)]
    owner_role: Option<String>,
    /// Date by which the decision is needed.
    #[serde(default)]
    decide_by: Option<NaiveDate>,
}

/// `PUT /api/exceptions/{pid}` — fill in the cause, the decision needed, the owner and the decide-by date.
/// Fields left out are kept; the exception is complete when all four are there.
#[debug_handler]
async fn complete_exception(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<CompleteBody>,
) -> Result<Response> {
    let row = find_exception(&ctx, &caller, &pid).await?;
    if row.status != "open" {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this exception is closed",
        ));
    }
    // Blank or whitespace-only text counts as absent, so it cannot mark a field complete.
    let clean = |s: &Option<String>| {
        s.as_ref()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    };
    // Only the fields given are written; with none given the row is read back unchanged, as `SeaORM`'s empty
    // update did.
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE delivery_exceptions SET ");
    let mut set = q.separated(", ");
    let mut any = false;
    for (column, value) in [
        ("cause = ", clean(&b.cause)),
        ("decision_needed = ", clean(&b.decision_needed)),
        ("owner_ref = ", clean(&b.owner_ref)),
        ("owner_role = ", clean(&b.owner_role)),
    ] {
        if let Some(v) = value {
            set.push(column).push_bind_unseparated(v);
            any = true;
        }
    }
    if let Some(d) = b.decide_by {
        set.push("decide_by = ").push_bind_unseparated(d);
        any = true;
    }
    q.push(" WHERE id = ")
        .push_bind(row.id)
        .push(" RETURNING *");
    let mut tx = crate::db::begin(&ctx).await?;
    let merged: exceptions::Model = if any {
        q.build_query_as().fetch_one(tx.conn()).await?
    } else {
        sqlx::query_as("SELECT * FROM delivery_exceptions WHERE id = $1 LIMIT 1")
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?
    };
    let parts = Parts {
        cause: merged.cause.as_deref(),
        decision_needed: merged.decision_needed.as_deref(),
        owner_ref: merged.owner_ref.as_deref(),
        decide_by: merged.decide_by,
    };
    let done = rules::missing(&parts).is_empty();
    let saved = if done && merged.completed_at.is_none() {
        sqlx::query_as("UPDATE delivery_exceptions SET completed_at = $1 WHERE id = $2 RETURNING *")
            .bind(Utc::now())
            .bind(merged.id)
            .fetch_one(tx.conn())
            .await?
    } else {
        merged
    };
    AuditModel::record_on(
        tx.conn(),
        saved.pid,
        "delivery_exception_updated",
        caller.actor(),
        Some(serde_json::json!({ "complete": done })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(exception_json(&saved, Utc::now(), rules::window_hours()))
}

/// `POST /api/exceptions/{pid}/decision` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DecisionBody {
    /// The recorded decision text; recording it is what closes the exception.
    decision: String,
}

/// `POST /api/exceptions/{pid}/decision` — record the decision, which closes it.
#[debug_handler]
async fn decide_exception(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<DecisionBody>,
) -> Result<Response> {
    let row = find_exception(&ctx, &caller, &pid).await?;
    if row.threshold_kind.is_some() {
        return Err(unprocessable(
            "this exception was opened by a threshold: it needs a pause, reset, stop or recorded override (POST /api/exceptions/{pid}/threshold-decision)",
        ));
    }
    if b.decision.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a decision of at least 10 characters is required",
        ));
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let mut tx = crate::db::begin(&ctx).await?;
    let done = sqlx::query(
        "UPDATE delivery_exceptions SET status = 'closed', closed_at = $1, decision = $2, decided_by = $3
         WHERE pid = $4 AND status = 'open'",
    )
    .bind(Utc::now())
    .bind(b.decision.trim())
    .bind(&actor)
    .bind(row.pid)
    .execute(tx.conn())
    .await?;
    if done.rows_affected() == 0 {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this exception is already closed",
        ));
    }
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "delivery_exception_closed",
        Some(&actor),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    let now = exception_by_pid(&ctx, row.pid)
        .await?
        .ok_or(Error::NotFound)?;
    format::json(exception_json(&now, Utc::now(), rules::window_hours()))
}

// ---------------------------------------------------------------------------------------------------
// DP-4: strategic aims and the portfolio map
// ---------------------------------------------------------------------------------------------------

/// `PUT /api/objectives/{pid}/strategic-aim` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AimBody {
    /// True marks the objective as a strategic aim, false unmarks it.
    strategic: bool,
}

/// `PUT /api/objectives/{pid}/strategic-aim` — mark or unmark an objective as a strategic aim.
#[debug_handler]
async fn mark_aim(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<AimBody>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let objective: objectives::Model =
        sqlx::query_as("SELECT * FROM objectives WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await?;
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM strategic_aims WHERE objective_pid = $1 LIMIT 1")
            .bind(objective.pid)
            .fetch_optional(tx.conn())
            .await?;
    match (b.strategic, existing) {
        (true, None) => {
            sqlx::query(
                "INSERT INTO strategic_aims (pid, objective_pid, marked_by) VALUES ($1, $2, $3) RETURNING id",
            )
            .bind(Uuid::new_v4())
            .bind(objective.pid)
            .bind(caller.actor())
            .fetch_one(tx.conn())
            .await?;
        }
        (false, Some(id)) => {
            // Unmarking also clears the primary flag of every programme that pointed at it.
            sqlx::query("UPDATE objective_links SET is_primary = FALSE WHERE objective_pid = $1")
                .bind(objective.pid)
                .execute(tx.conn())
                .await?;
            sqlx::query("DELETE FROM strategic_aims WHERE id = $1")
                .bind(id)
                .execute(tx.conn())
                .await?;
        }
        _ => {}
    }
    AuditModel::record_on(
        tx.conn(),
        objective.pid,
        if b.strategic {
            "strategic_aim_marked"
        } else {
            "strategic_aim_unmarked"
        },
        caller.actor(),
        None,
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "objective_pid": objective.pid, "strategic": b.strategic }))
}

/// `PUT /api/plans/{pid}/primary-aim` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PrimaryBody {
    /// Public id of the strategic aim that becomes the programme's one primary aim.
    objective_pid: Uuid,
}

/// `PUT /api/plans/{pid}/primary-aim` — the programme's one primary strategic aim. It links the
/// programme to the aim if it is not already, and makes any earlier primary an ordinary link.
#[debug_handler]
async fn set_primary(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<PrimaryBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let is_aim = sqlx::query("SELECT 1 FROM strategic_aims WHERE objective_pid = $1 LIMIT 1")
        .bind(b.objective_pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await?
        .is_some();
    if !is_aim {
        return Err(unprocessable(
            "that objective is not marked as a strategic aim",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE objective_links SET is_primary = FALSE WHERE plan_pid = $1")
        .bind(plan.pid)
        .execute(tx.conn())
        .await?;
    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM objective_links WHERE plan_pid = $1 AND objective_pid = $2 LIMIT 1",
    )
    .bind(plan.pid)
    .bind(b.objective_pid)
    .fetch_optional(tx.conn())
    .await?;
    if let Some(id) = existing {
        sqlx::query("UPDATE objective_links SET is_primary = TRUE WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_one(tx.conn())
            .await?;
    } else {
        sqlx::query(
            "INSERT INTO objective_links (pid, objective_pid, plan_pid, weight, is_primary)
             VALUES ($1, $2, $3, 3, TRUE)
             RETURNING id",
        )
        .bind(Uuid::new_v4())
        .bind(b.objective_pid)
        .bind(plan.pid)
        .fetch_one(tx.conn())
        .await?;
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "primary_aim_set",
        caller.actor(),
        Some(serde_json::json!({ "objective_pid": b.objective_pid })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({ "plan_pid": plan.pid, "primary_aim": b.objective_pid }))
}

/// One count out of a previous pack's map: the confidence value shown for a programme.
fn previous_value(body: Option<&str>, plan: Uuid) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(body?).ok()?;
    let groups = v.get("portfolio_map")?.get("groups")?.as_array()?;
    for g in groups {
        for p in g.get("programmes")?.as_array()? {
            if p.get("plan_pid")?.as_str()? == plan.to_string() {
                return p
                    .get("confidence")?
                    .get("value")?
                    .as_str()
                    .map(str::to_string);
            }
        }
    }
    None
}

/// Build the portfolio map for the programmes among `include`, as of `today` for the pack `period`.
/// `previous_body` is the previous issued pack, for the trend.
///
/// # Errors
///
/// When a database read fails.
#[expect(clippy::too_many_lines)]
pub(crate) async fn build_map(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    period: &str,
    previous_body: Option<&str>,
) -> Result<serde_json::Value> {
    let today = crate::tz::deployment_today();
    let all_plans: Vec<plans::Model> =
        sqlx::query_as("SELECT * FROM plans WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let programmes: Vec<&plans::Model> = all_plans
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    let risk_rows: Vec<risks::Model> = sqlx::query_as("SELECT * FROM risks")
        .fetch_all(crate::db::pool(ctx))
        .await?;
    let budget_rows: Vec<budget_lines::Model> = sqlx::query_as("SELECT * FROM budget_lines")
        .fetch_all(crate::db::pool(ctx))
        .await?;
    let deps = crate::models::visibility::all_dependencies(ctx).await?;
    let scoped: Vec<plans::Model> = all_plans
        .iter()
        .filter(|p| include.contains(&p.pid))
        .cloned()
        .collect();
    let rag = super::insights::rag_by_pid(&scoped, &risk_rows, &budget_rows, &deps, today);
    let aims: HashMap<Uuid, String> = {
        let marked: HashSet<Uuid> =
            sqlx::query_scalar::<_, Uuid>("SELECT objective_pid FROM strategic_aims")
                .fetch_all(crate::db::pool(ctx))
                .await?
                .into_iter()
                .collect();
        sqlx::query_as::<_, objectives::Model>("SELECT * FROM objectives WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .filter(|o| marked.contains(&o.pid))
            .map(|o| (o.pid, o.title))
            .collect()
    };
    let links: Vec<objective_links::Model> = sqlx::query_as("SELECT * FROM objective_links")
        .fetch_all(crate::db::pool(ctx))
        .await?;
    let statuses = super::programme_gates::status_map(ctx, today).await?;
    let (profiles, org_names) = super::programme_profile::all(ctx, include).await?;
    let history: Vec<assessments::Model> =
        sqlx::query_as("SELECT * FROM delivery_confidence_assessments ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let mut by_plan: HashMap<Uuid, Vec<Assessment>> = HashMap::new();
    for h in &history {
        by_plan.entry(h.plan_pid).or_default().push(lite(h));
    }
    // group key: Some(aim pid) or None for "no strategic aim".
    let mut groups: BTreeMap<Option<Uuid>, Vec<serde_json::Value>> = BTreeMap::new();
    for p in &programmes {
        let mine: Vec<&objective_links::Model> = links
            .iter()
            .filter(|l| l.plan_pid == p.pid && aims.contains_key(&l.objective_pid))
            .collect();
        let primary = mine.iter().find(|l| l.is_primary).map(|l| l.objective_pid);
        let others: Vec<String> = mine
            .iter()
            .filter(|l| Some(l.objective_pid) != primary)
            .filter_map(|l| aims.get(&l.objective_pid).cloned())
            .collect();
        let sh = rules::shown(by_plan.get(&p.pid).map_or(&[][..], Vec::as_slice));
        let derived = rag.get(&p.pid).copied();
        let value = sh.as_ref().map(|s| s.value.clone());
        let disagreement = match (&value, derived.and_then(rules::rank_of_rag)) {
            (Some(v), Some(d)) if rules::disagree(v, d) => Some("derived"),
            _ => None,
        };
        let self_gap = {
            let h = by_plan.get(&p.pid).map_or(&[][..], Vec::as_slice);
            let ind = h.iter().rev().find(|a| a.kind == "independent");
            let own = h.iter().rev().find(|a| a.kind == "self");
            match (ind, own) {
                (Some(i), Some(s)) => {
                    rules::rank(&s.value).is_some_and(|r| rules::disagree(&i.value, r))
                }
                _ => false,
            }
        };
        let confidence = match &sh {
            Some(s) => serde_json::json!({
                "value": s.value, "kind": s.kind, "marked_self": s.kind == "self",
                "assessed_on": s.assessed_on, "stale": rules::is_stale(s.assessed_on, period),
            }),
            None => serde_json::json!({ "value": "not_assessed" }),
        };
        let prev = previous_value(previous_body, p.pid);
        let trend = if previous_body.is_none() {
            "no_earlier_pack"
        } else {
            rules::trend(prev.as_deref(), value.as_deref())
        };
        groups.entry(primary).or_default().push(serde_json::json!({
            "plan_pid": p.pid, "name": p.name, "stage": p.stage, "phase": p.phase,
            "programme_status": statuses.get(&p.pid).copied().unwrap_or("active"),
            "delivery": super::programme_profile::view(profiles.get(&p.pid), &org_names),
            "confidence": confidence, "derived_rag": derived,
            "trend": trend,
            "disagreement": { "with_derived": disagreement.is_some(), "independent_vs_self": self_gap },
            "also_serves": others,
            "reason_no_primary": (primary.is_none() && !mine.is_empty())
                .then_some("linked to strategic aims but none is marked primary"),
        }));
    }
    let mut out = Vec::new();
    let mut titled: Vec<(&Uuid, &String)> = aims.iter().collect();
    titled.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
    for (pid, title) in titled {
        let progs = groups.remove(&Some(*pid)).unwrap_or_default();
        out.push(serde_json::json!({ "aim_pid": pid, "aim": title, "programmes": progs }));
    }
    let none = groups.remove(&None).unwrap_or_default();
    out.push(serde_json::json!({ "aim_pid": null, "aim": "no strategic aim", "programmes": none }));
    let total: usize = out
        .iter()
        .map(|g| g["programmes"].as_array().map_or(0, Vec::len))
        .sum();
    Ok(serde_json::json!({
        "period": period,
        "programmes_total": total,
        "groups": out,
        "note": "Programmes are plans of kind Program. A programme appears once, under its primary strategic aim; other aims it serves are named. Delivery confidence is the stated value (latest independent, else latest self, marked), beside the derived RAG, never blended. Not assessed is not green. Trend compares with the previous issued pack.",
    }))
}

/// `GET /api/portfolio-map[?period=YYYY-MM]` — the map for the programmes you can read.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MapQuery {
    /// Optional `YYYY-MM` period; absent means the current month (UTC).
    #[serde(default)]
    period: Option<String>,
}

/// `GET /api/portfolio-map` handler: the map of programmes the caller may read, grouped under their primary strategic aim. Read-only.
#[debug_handler]
async fn portfolio_map(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<MapQuery>,
) -> Result<Response> {
    let period = q
        .period
        .unwrap_or_else(|| Utc::now().format("%Y-%m").to_string());
    if !crate::director_pack::valid_period(&period) {
        return Err(unprocessable("period must be YYYY-MM"));
    }
    let all: Vec<Uuid> = sqlx::query_scalar("SELECT pid FROM plans WHERE deleted_at IS NULL")
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let map = build_map(&ctx, &readable, &period, None).await?;
    format::json(map)
}

/// The exceptions section of a pack, over the plans in `include`.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn exceptions_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    period: &str,
    now: DateTime<Utc>,
) -> Result<serde_json::Value> {
    let names: HashMap<Uuid, String> =
        sqlx::query_as::<_, (Uuid, String)>("SELECT pid, name FROM plans")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .collect();
    let rows: Vec<exceptions::Model> = sqlx::query_as::<_, exceptions::Model>(
        "SELECT * FROM delivery_exceptions ORDER BY opened_at",
    )
    .fetch_all(crate::db::pool(ctx))
    .await?
    .into_iter()
    .filter(|r| include.contains(&r.plan_pid))
    .collect();
    let window = rules::window_hours();
    let start = rules::period_start(period);
    let item = |r: &exceptions::Model| {
        let j = exception_json(r, now, window);
        // The pack names the owner by role and never by person.
        serde_json::json!({
            "plan_pid": r.plan_pid, "plan": names.get(&r.plan_pid), "opened_at": r.opened_at,
            "worst_value": r.worst_value, "worsened_count": r.worsened_count,
            "cause": r.cause, "decision_needed": r.decision_needed,
            "owner_role": r.owner_role, "decide_by": r.decide_by,
            "complete": j["complete"], "breached": j["breached"], "hours_open": j["hours_open"],
            "decision": r.decision, "closed_at": r.closed_at,
        })
    };
    let open: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| r.status == "open")
        .map(item)
        .collect();
    let breached: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| exception_json(r, now, window)["breached"] == true)
        .map(item)
        .collect();
    let closed: Vec<serde_json::Value> = rows
        .iter()
        .filter(|r| {
            r.status == "closed"
                && r.closed_at.is_some_and(|c| {
                    let d = DateTime::<Utc>::from(c).date_naive();
                    start.is_some_and(|s| d >= s && d.format("%Y-%m").to_string() == period)
                })
        })
        .map(item)
        .collect();
    Ok(serde_json::json!({
        "window_hours": window,
        "open": open, "breached": breached, "closed_in_period": closed,
        "note": "An exception opens when delivery confidence reaches amber-red or red. It is complete with a cause, the decision needed, a named owner and a decide-by date, and closes only when a decision is recorded. Breached means it was not complete within the window counted from when it opened. Owners are shown by role, never by person.",
    }))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/delivery-confidence", put(set_confidence))
        .add("/plans/{pid}/delivery-confidence", get(get_confidence))
        .add("/exceptions", get(list_exceptions))
        .add("/exceptions/{pid}", put(complete_exception))
        .add("/exceptions/{pid}/decision", post(decide_exception))
        .add("/objectives/{pid}/strategic-aim", put(mark_aim))
        .add("/plans/{pid}/primary-aim", put(set_primary))
        .add("/portfolio-map", get(portfolio_map))
}

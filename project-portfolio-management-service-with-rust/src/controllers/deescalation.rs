//! The outcome scorecard and the de-escalation tracker (EX-10). Rules in [`crate::deescalation`].
//!
//! * scorecards: a named set of KPIs read by quarter (`/api/scorecards`), with an optional starter set of
//!   measures that has **no targets** and is not applied unless asked for (`/api/scorecard-template`);
//! * the tracker (`/api/deescalation`): the escalation level as the sponsor set it, the criteria for coming
//!   down with the evidence per quarter, and the **consecutive quarters met**. The count is offered to the
//!   person who decides; it moves no level. A quarter with a missing reading breaks the run and says why.

use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::deescalation::{self as rules, Input, Quarter, QuarterState};
use crate::kpi::{self, Direction, Status};
use crate::models::_entities::{
    criterion_evidence as evidence, deescalation_criteria as crits, escalation_levels as levels,
    kpi_readings, kpis, scorecards,
};
use crate::models::audit_logs::Model as AuditModel;

/// Map a database error to an API error; unique-constraint violations on scorecard name or the single de-escalation scorecard become 409 conflicts, anything else a generic error.
fn db_err(e: &sqlx::Error) -> Error {
    let t = e.to_string();
    if t.contains("sc_name") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "a scorecard with that name exists"),
        );
    }
    if t.contains("sc_one_deescalation") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "conflict",
                "another scorecard is already the one the de-escalation tracker reads",
            ),
        );
    }
    Error::string(&t)
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/scorecards", post(create))
        .add("/scorecards", get(list))
        .add("/scorecard-template", get(template))
        .add("/scorecards/{pid}", get(show))
        .add("/scorecards/{pid}/kpis", put(set_kpis))
        .add("/scorecards/{pid}/from-template", post(from_template))
        .add("/deescalation", get(tracker))
        .add("/deescalation/level", put(set_level))
        .add("/deescalation/criteria", post(add_criterion))
        .add("/deescalation/criteria/{pid}/evidence", put(set_evidence))
}

/// True when the caller holds the admin access attribute (or auth is open).
fn is_admin(caller: &MaybeAuthUser) -> bool {
    caller.holds_or_open(|c| crate::auth::has_attr(c, "access", "admin"))
}

/// True when the caller holds the given role attribute (or auth is open).
fn has_role(caller: &MaybeAuthUser, role: &str) -> bool {
    caller.holds_or_open(|c| crate::auth::has_attr(c, "role", role))
}

/// Pids of non-deleted plans the caller may read; scorecards only count KPIs from these.
async fn readable(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<HashSet<Uuid>> {
    let all: Vec<Uuid> = sqlx::query_scalar("SELECT pid FROM plans WHERE deleted_at IS NULL")
        .fetch_all(crate::db::pool(ctx))
        .await
        .map_err(|e| db_err(&e))?;
    Ok(super::plan_access::readable_plans(ctx, caller, &all).await)
}

// ---- scorecards -----------------------------------------------------------------------------

/// Request body for creating a scorecard.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewScorecard {
    /// Scorecard name; must be unique.
    name: String,
    /// Make this the scorecard the de-escalation tracker reads.
    #[serde(default)]
    for_deescalation: bool,
}

/// `POST /api/scorecards`
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewScorecard>,
) -> Result<Response> {
    if !(3..=120).contains(&b.name.trim().chars().count()) {
        return Err(unprocessable("name is 3 to 120 characters"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: scorecards::Model = sqlx::query_as(
        "INSERT INTO scorecards (pid, name, for_deescalation, active)
         VALUES ($1, $2, $3, TRUE)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.name.trim())
    .bind(b.for_deescalation)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "scorecard_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(
        json!({ "pid": row.pid, "name": row.name, "for_deescalation": row.for_deescalation }),
    )
}

/// `GET /api/scorecards`
#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let rows: Vec<scorecards::Model> = sqlx::query_as("SELECT * FROM scorecards ORDER BY name")
        .fetch_all(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?;
    format::json(
        json!({ "scorecards": rows.iter().map(|r| json!({ "pid": r.pid, "name": r.name, "for_deescalation": r.for_deescalation, "active": r.active })).collect::<Vec<_>>() }),
    )
}

/// Request body replacing the KPI membership of a scorecard (at most 60, duplicates collapsed).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SetKpis {
    /// Pids of the KPIs that replace the scorecard membership.
    kpi_pids: Vec<Uuid>,
}

/// Load a scorecard by pid string; a malformed pid is rejected by `parse_pid` and an absent row is `NotFound`.
async fn find_scorecard(ctx: &AppContext, pid: &str) -> Result<scorecards::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM scorecards WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)
}

/// `PUT /api/scorecards/{pid}/kpis` — the KPIs on the scorecard (replaces the set).
#[debug_handler]
async fn set_kpis(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<SetKpis>,
) -> Result<Response> {
    let sc = find_scorecard(&ctx, &pid).await?;
    let wanted: Vec<Uuid> = b
        .kpi_pids
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if wanted.len() > 60 {
        return Err(unprocessable("a scorecard holds at most 60 KPIs"));
    }
    let found: Vec<Uuid> =
        sqlx::query_scalar("SELECT pid FROM kpis WHERE pid = ANY($1) AND deleted_at IS NULL")
            .bind(&wanted)
            .fetch_all(crate::db::pool(&ctx))
            .await
            .map_err(|e| db_err(&e))?;
    // A hidden KPI gets the answer of one that does not exist, and stays on the card: the caller cannot see to remove it.
    let hidden: Vec<Uuid> = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .owned_pids(&ctx)
        .await?
        .into_iter()
        .collect();
    if found.len() != wanted.len() || wanted.iter().any(|k| hidden.contains(k)) {
        return Err(unprocessable("every kpi_pid must be a KPI"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    // The hidden KPIs as one bound array, which no number of them can take past the bind-parameter limit.
    sqlx::query("DELETE FROM scorecard_kpis WHERE scorecard_pid = $1 AND kpi_pid <> ALL($2)")
        .bind(sc.pid)
        .bind(&hidden)
        .execute(tx.conn())
        .await
        .map_err(|e| db_err(&e))?;
    for k in &wanted {
        insert_member(&mut tx, sc.pid, *k).await?;
    }
    AuditModel::record_on(
        tx.conn(),
        sc.pid,
        "scorecard_kpis_set",
        caller.actor(),
        Some(json!({ "count": wanted.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "scorecard_pid": sc.pid, "kpi_pids": wanted }))
}

/// Put one KPI on a scorecard, in the caller's transaction.
async fn insert_member(tx: &mut crate::db::Tx, scorecard: Uuid, kpi: Uuid) -> Result<()> {
    sqlx::query(
        "INSERT INTO scorecard_kpis (pid, scorecard_pid, kpi_pid) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(Uuid::new_v4())
    .bind(scorecard)
    .bind(kpi)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    Ok(())
}

/// `GET /api/scorecard-template` — a starter set of measures with no targets; nothing is applied.
#[debug_handler]
async fn template() -> Result<Response> {
    format::json(json!({
        "measures": rules::TEMPLATE.iter().map(|(key, name, dir, unit)| json!({ "key": key, "name": name, "direction": dir, "unit": unit })).collect::<Vec<_>>(),
        "note": "A starter set, offered and not applied: it has no targets, and a deployment chooses its own measures. POST /api/scorecards/{pid}/from-template creates the ones you pick, with the targets you set.",
    }))
}

/// One template measure chosen for creation, with the owner-set thresholds.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Pick {
    /// Key of a measure in the built-in scorecard template.
    key: String,
    /// Target the KPI must reach to be green, in the measure unit.
    target_value: i64,
    /// Threshold at which the KPI is red; must be consistent with the measure direction.
    red_value: i64,
}

/// Request body for creating KPIs from the scorecard template.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct FromTemplate {
    /// Template measures chosen, each with its own thresholds.
    measures: Vec<Pick>,
}

/// `POST /api/scorecards/{pid}/from-template` — create the picked measures as KPIs (with your targets) and add them.
#[debug_handler]
async fn from_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<FromTemplate>,
) -> Result<Response> {
    let sc = find_scorecard(&ctx, &pid).await?;
    if b.measures.is_empty() || b.measures.len() > rules::TEMPLATE.len() {
        return Err(unprocessable("pick at least one measure from the template"));
    }
    let mut problems = Vec::new();
    let mut chosen = Vec::new();
    for p in &b.measures {
        let Some((_, name, dir, unit)) = rules::TEMPLATE.iter().find(|(k, ..)| *k == p.key) else {
            problems.push(format!("{} is not in the template", p.key));
            continue;
        };
        let d = Direction::parse(dir).unwrap_or(Direction::HigherBetter);
        if let Some(why) = kpi::threshold_problem(d, p.target_value, p.red_value) {
            problems.push(format!("{}: {why}", p.key));
        }
        chosen.push((*name, *dir, *unit, p));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut made = Vec::new();
    let mut tx = crate::db::begin(&ctx).await?;
    for (name, dir, unit, p) in chosen {
        let k: Uuid = sqlx::query_scalar(
            "INSERT INTO kpis (pid, name, unit, direction, target_value, red_value, cadence)
             VALUES ($1, $2, $3, $4, $5, $6, 'quarterly')
             RETURNING pid",
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(unit)
        .bind(dir)
        .bind(p.target_value)
        .bind(p.red_value)
        .fetch_one(tx.conn())
        .await
        .map_err(|e| db_err(&e))?;
        insert_member(&mut tx, sc.pid, k).await?;
        made.push(k);
    }
    AuditModel::record_on(
        tx.conn(),
        sc.pid,
        "scorecard_from_template",
        caller.actor(),
        Some(json!({ "created": made.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "scorecard_pid": sc.pid, "kpi_pids": made }))
}

/// One KPI of a scorecard by quarter. A KPI is *met* in a quarter when the latest reading dated in it is at or
/// better than the target; with no reading it is missing (`met` null), not unmet.
async fn kpi_rows(
    ctx: &AppContext,
    sc: &scorecards::Model,
    quarters: &[Quarter],
    visible: &HashSet<Uuid>,
) -> Result<(Vec<Value>, usize, Vec<Vec<Input>>)> {
    let ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT kpi_pid FROM scorecard_kpis WHERE scorecard_pid = $1 ORDER BY id",
    )
    .bind(sc.pid)
    .fetch_all(crate::db::pool(ctx))
    .await
    .map_err(|e| db_err(&e))?;
    let all: Vec<kpis::Model> =
        sqlx::query_as("SELECT * FROM kpis WHERE pid = ANY($1) AND deleted_at IS NULL")
            .bind(&ids)
            .fetch_all(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?;
    let by: HashMap<Uuid, kpis::Model> = all.into_iter().map(|k| (k.pid, k)).collect();
    let mut hidden = 0;
    let mut rows = Vec::new();
    let mut inputs: Vec<Vec<Input>> = vec![Vec::new(); quarters.len()];
    for id in ids {
        let Some(k) = by.get(&id) else { continue };
        if k.plan_pid.is_some_and(|p| !visible.contains(&p)) {
            hidden += 1;
            continue;
        }
        let dir = Direction::parse(&k.direction).unwrap_or(Direction::HigherBetter);
        let readings: Vec<kpi_readings::Model> = sqlx::query_as(
            "SELECT * FROM kpi_readings WHERE kpi_pid = $1 ORDER BY observed_on, id",
        )
        .bind(k.pid)
        .fetch_all(crate::db::pool(ctx))
        .await
        .map_err(|e| db_err(&e))?;
        let mut per = Vec::new();
        for (i, q) in quarters.iter().enumerate() {
            let latest = readings
                .iter()
                .rfind(|r| r.observed_on >= q.start() && r.observed_on <= q.end());
            let met = latest
                .map(|r| kpi::status(dir, k.target_value, k.red_value, r.value) == Status::Green);
            inputs[i].push(Input {
                name: k.name.clone(),
                met,
            });
            per.push(json!({ "quarter": q.label(), "value": latest.map(|r| r.value), "observed_on": latest.map(|r| r.observed_on), "met": met }));
        }
        rows.push(json!({ "kpi_pid": k.pid, "name": k.name, "unit": k.unit, "direction": k.direction, "target_value": k.target_value, "quarters": per }));
    }
    Ok((rows, hidden, inputs))
}

/// Query string for the scorecard view.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Quarters {
    /// Number of finished quarters to show; absent means 4.
    #[serde(default)]
    quarters: Option<usize>,
}

/// `GET /api/scorecards/{pid}[?quarters=4]` — each KPI by finished quarter.
#[debug_handler]
async fn show(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<Quarters>,
) -> Result<Response> {
    let sc = find_scorecard(&ctx, &pid).await?;
    let qs = rules::window(crate::tz::deployment_today(), q.quarters.unwrap_or(4));
    let visible = readable(&ctx, &caller).await?;
    let (rows, hidden, inputs) = kpi_rows(&ctx, &sc, &qs, &visible).await?;
    let states: Vec<Value> = qs
        .iter()
        .zip(&inputs)
        .map(|(q, i)| json!({ "quarter": q.label(), "state": rules::quarter_state(i) }))
        .collect();
    format::json(json!({
        "scorecard": { "pid": sc.pid, "name": sc.name, "for_deescalation": sc.for_deescalation },
        "quarters": qs.iter().map(|q| q.label()).collect::<Vec<_>>(),
        "kpis": rows, "kpis_you_may_not_read": hidden, "by_quarter": states,
        "note": "Only finished quarters are read. A KPI is met in a quarter when the latest reading dated in it is at or better than its target; a quarter with no reading is missing, which is not the same as not met.",
    }))
}

// ---- the tracker ----------------------------------------------------------------------------

/// Request body recording the sponsor-set escalation level.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LevelBody {
    /// Escalation level label set by the sponsor, 1 to 60 characters after trimming.
    level: String,
    /// Optional reason for the level.
    #[serde(default)]
    note: Option<String>,
}

/// `PUT /api/deescalation/level` — record the escalation level the sponsor has set. It is never computed.
#[debug_handler]
async fn set_level(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<LevelBody>,
) -> Result<Response> {
    if !is_admin(&caller) && !has_role(&caller, "sponsor") {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new(
                "forbidden",
                "the escalation level is recorded by the sponsor (role=sponsor) or an administrator",
            ),
        ));
    }
    if !(1..=60).contains(&b.level.trim().chars().count()) {
        return Err(unprocessable("level is 1 to 60 characters"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: levels::Model = sqlx::query_as(
        "INSERT INTO escalation_levels (pid, level, note, set_by, set_on)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.level.trim())
    .bind(
        b.note
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty()),
    )
    .bind(caller.actor())
    .bind(crate::tz::deployment_today())
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "escalation_level_set",
        caller.actor(),
        Some(json!({ "level": row.level })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "level": row.level, "set_on": row.set_on }))
}

/// Request body adding a de-escalation criterion.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CriterionBody {
    /// Criterion text, 5 to 500 characters after trimming.
    criterion: String,
}

/// `POST /api/deescalation/criteria`
#[debug_handler]
async fn add_criterion(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<CriterionBody>,
) -> Result<Response> {
    if !(5..=500).contains(&b.criterion.trim().chars().count()) {
        return Err(unprocessable("a criterion is 5 to 500 characters"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: crits::Model = sqlx::query_as(
        "INSERT INTO deescalation_criteria (pid, criterion, active)
         VALUES ($1, $2, TRUE)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.criterion.trim())
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "deescalation_criterion_added",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "pid": row.pid, "criterion": row.criterion }))
}

/// Request body recording evidence for one criterion in one finished quarter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct EvidenceBody {
    /// `YYYY-Qn`, a finished quarter.
    quarter: String,
    /// Whether the criterion was met in that quarter.
    met: bool,
    /// Optional link to supporting evidence.
    #[serde(default)]
    link: Option<String>,
    /// Required justification, at least 5 characters after trimming.
    note: String,
}

/// `PUT /api/deescalation/criteria/{pid}/evidence` — record whether the criterion was met in a finished quarter.
#[debug_handler]
async fn set_evidence(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<EvidenceBody>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let c: crits::Model =
        sqlx::query_as("SELECT * FROM deescalation_criteria WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await
            .map_err(|e| db_err(&e))?
            .ok_or(Error::NotFound)?;
    let Some(q) = Quarter::parse(&b.quarter) else {
        return Err(unprocessable("quarter must be YYYY-Qn"));
    };
    if q > rules::last_finished(crate::tz::deployment_today()) {
        return Err(unprocessable(
            "a quarter that has not finished cannot be recorded",
        ));
    }
    if b.note.trim().chars().count() < 5 {
        return Err(unprocessable("a note of at least 5 characters is required"));
    }
    if let Some(l) = &b.link
        && (l.len() > 500
            || !(l.starts_with("https://") || l.starts_with("http://"))
            || l.contains(char::is_whitespace))
    {
        return Err(unprocessable(
            "link must be an http or https address with no spaces, up to 500 characters",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "INSERT INTO criterion_evidence
             (pid, criterion_pid, quarter, met, link, note, recorded_by, recorded_on)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id",
    )
    .bind(Uuid::new_v4())
    .bind(c.pid)
    .bind(q.label())
    .bind(b.met)
    .bind(b.link.as_deref())
    .bind(b.note.trim())
    .bind(caller.actor())
    .bind(crate::tz::deployment_today())
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_on(
        tx.conn(),
        c.pid,
        "deescalation_evidence_recorded",
        caller.actor(),
        Some(json!({ "quarter": q.label(), "met": b.met })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "criterion_pid": c.pid, "quarter": q.label(), "met": b.met }))
}

/// The tracker as of `today`, over the plans in `visible`.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn tracker_view(
    ctx: &AppContext,
    visible: &HashSet<Uuid>,
    today: chrono::NaiveDate,
    with_detail: bool,
) -> Result<Value> {
    let (need, unusable) = rules::required_quarters();
    let qs = rules::window(today, need.max(4));
    let level: Option<levels::Model> =
        sqlx::query_as("SELECT * FROM escalation_levels ORDER BY id DESC LIMIT 1")
            .fetch_optional(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?;
    let history: Vec<levels::Model> =
        sqlx::query_as("SELECT * FROM escalation_levels ORDER BY id DESC")
            .fetch_all(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?;
    let criteria: Vec<crits::Model> =
        sqlx::query_as("SELECT * FROM deescalation_criteria WHERE active = TRUE ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?;
    let evid: Vec<evidence::Model> = sqlx::query_as("SELECT * FROM criterion_evidence ORDER BY id")
        .fetch_all(crate::db::pool(ctx))
        .await
        .map_err(|e| db_err(&e))?;
    let mut latest: HashMap<(Uuid, String), &evidence::Model> = HashMap::new();
    for e in &evid {
        latest.insert((e.criterion_pid, e.quarter.clone()), e);
    }
    let sc: Option<scorecards::Model> =
        sqlx::query_as("SELECT * FROM scorecards WHERE for_deescalation = TRUE LIMIT 1")
            .fetch_optional(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?;
    let (kpi_rows_v, hidden, mut inputs) = match &sc {
        Some(s) => kpi_rows(ctx, s, &qs, visible).await?,
        None => (Vec::new(), 0, vec![Vec::new(); qs.len()]),
    };
    for (i, q) in qs.iter().enumerate() {
        for c in &criteria {
            inputs[i].push(Input {
                name: c.criterion.clone(),
                met: latest.get(&(c.pid, q.label())).map(|e| e.met),
            });
        }
    }
    let states: Vec<QuarterState> = inputs.iter().map(|i| rules::quarter_state(i)).collect();
    let run = rules::consecutive_met(&states);
    let mut out = json!({
        "level": level.as_ref().map(|l| json!({ "level": l.level, "set_on": l.set_on, "note": l.note })),
        "level_note": "The level is recorded as the sponsor set it. It is never computed here, and nothing below changes it.",
        "scorecard": sc.as_ref().map(|s| json!({ "pid": s.pid, "name": s.name })),
        "required_quarters": need, "required_quarters_setting_unusable": unusable,
        "consecutive_quarters_met": run,
        "rule_satisfied": run >= need,
        "by_quarter": qs.iter().zip(&states).map(|(q, s)| json!({ "quarter": q.label(), "state": s })).collect::<Vec<_>>(),
        "criteria_count": criteria.len(), "kpis_you_may_not_read": hidden,
    });
    if with_detail {
        out["kpis"] = json!(kpi_rows_v);
        out["criteria"] = json!(criteria.iter().map(|c| json!({
            "pid": c.pid, "criterion": c.criterion,
            "quarters": qs.iter().map(|q| { let e = latest.get(&(c.pid, q.label())); json!({ "quarter": q.label(), "met": e.map(|x| x.met), "link": e.and_then(|x| x.link.clone()), "note": e.map(|x| x.note.clone()) }) }).collect::<Vec<_>>(),
        })).collect::<Vec<_>>());
        out["level_history"] = json!(
            history
                .iter()
                .map(|l| json!({ "level": l.level, "set_on": l.set_on }))
                .collect::<Vec<_>>()
        );
    }
    Ok(out)
}

/// `GET /api/deescalation`
#[debug_handler]
async fn tracker(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let visible = readable(&ctx, &caller).await?;
    format::json(tracker_view(&ctx, &visible, crate::tz::deployment_today(), true).await?)
}

/// The pack's section: the scorecard of the last finished quarter and the tracker without evidence detail.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext, include: &HashSet<Uuid>) -> Result<Value> {
    let mut v = tracker_view(ctx, include, crate::tz::deployment_today(), true).await?;
    // The pack carries the level, the run and the KPI figures; the evidence notes and links stay in the tracker.
    if let Some(c) = v.get_mut("criteria").and_then(Value::as_array_mut) {
        for x in c {
            if let Some(qs) = x.get_mut("quarters").and_then(Value::as_array_mut) {
                for q in qs {
                    q["note"] = Value::Null;
                    q["link"] = Value::Null;
                }
            }
        }
    }
    Ok(v)
}

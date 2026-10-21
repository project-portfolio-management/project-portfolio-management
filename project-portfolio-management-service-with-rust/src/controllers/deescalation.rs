//! The outcome scorecard and the de-escalation tracker (EX-10). Rules in [`crate::deescalation`].
//!
//! * scorecards: a named set of KPIs read by quarter (`/api/scorecards`), with an optional starter set of
//!   measures that has **no targets** and is not applied unless asked for (`/api/scorecard-template`);
//! * the tracker (`/api/deescalation`): the escalation level as the sponsor set it, the criteria for coming
//!   down with the evidence per quarter, and the **consecutive quarters met**. The count is offered to the
//!   person who decides; it moves no level. A quarter with a missing reading breaks the run and says why.

use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::deescalation::{self as rules, Input, Quarter, QuarterState};
use crate::kpi::{self, Direction, Status};
use crate::models::_entities::{
    criterion_evidence as evidence, deescalation_criteria as crits, escalation_levels as levels,
    kpi_readings, kpis, plans, scorecard_kpis as members, scorecards,
};
use crate::models::audit_logs::Model as AuditModel;

fn db_err(e: &sea_orm::DbErr) -> Error {
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

fn is_admin(caller: &MaybeAuthUser) -> bool {
    caller.holds_or_open(|c| crate::auth::has_attr(c, "access", "admin"))
}

fn has_role(caller: &MaybeAuthUser, role: &str) -> bool {
    caller.holds_or_open(|c| crate::auth::has_attr(c, "role", role))
}

async fn readable(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<HashSet<Uuid>> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    Ok(super::plan_access::readable_plans(ctx, caller, &all).await)
}

// ---- scorecards -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct NewScorecard {
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
    let row = scorecards::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(b.name.trim().to_string()),
        for_deescalation: ActiveValue::set(b.for_deescalation),
        active: ActiveValue::set(true),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(&ctx.db, row.pid, "scorecard_created", caller.actor(), None).await;
    format::json(
        json!({ "pid": row.pid, "name": row.name, "for_deescalation": row.for_deescalation }),
    )
}

/// `GET /api/scorecards`
#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = scorecards::Entity::find()
        .order_by_asc(scorecards::Column::Name)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(
        json!({ "scorecards": rows.iter().map(|r| json!({ "pid": r.pid, "name": r.name, "for_deescalation": r.for_deescalation, "active": r.active })).collect::<Vec<_>>() }),
    )
}

#[derive(Debug, Deserialize)]
struct SetKpis {
    kpi_pids: Vec<Uuid>,
}

async fn find_scorecard(ctx: &AppContext, pid: &str) -> Result<scorecards::Model> {
    let pid = super::parse_pid(pid)?;
    scorecards::Entity::find()
        .filter(scorecards::Column::Pid.eq(pid))
        .one(&ctx.db)
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
    let found = kpis::Entity::find()
        .filter(kpis::Column::Pid.is_in(wanted.clone()))
        .filter(kpis::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    if found.len() != wanted.len() {
        return Err(unprocessable("every kpi_pid must be a KPI"));
    }
    members::Entity::delete_many()
        .filter(members::Column::ScorecardPid.eq(sc.pid))
        .exec(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    for k in &wanted {
        members::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            scorecard_pid: ActiveValue::set(sc.pid),
            kpi_pid: ActiveValue::set(*k),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    }
    AuditModel::record_or_log(
        &ctx.db,
        sc.pid,
        "scorecard_kpis_set",
        caller.actor(),
        Some(json!({ "count": wanted.len() })),
    )
    .await;
    format::json(json!({ "scorecard_pid": sc.pid, "kpi_pids": wanted }))
}

/// `GET /api/scorecard-template` — a starter set of measures with no targets; nothing is applied.
#[debug_handler]
async fn template() -> Result<Response> {
    format::json(json!({
        "measures": rules::TEMPLATE.iter().map(|(key, name, dir, unit)| json!({ "key": key, "name": name, "direction": dir, "unit": unit })).collect::<Vec<_>>(),
        "note": "A starter set, offered and not applied: it has no targets, and a deployment chooses its own measures. POST /api/scorecards/{pid}/from-template creates the ones you pick, with the targets you set.",
    }))
}

#[derive(Debug, Deserialize)]
struct Pick {
    key: String,
    target_value: i64,
    red_value: i64,
}

#[derive(Debug, Deserialize)]
struct FromTemplate {
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
    for (name, dir, unit, p) in chosen {
        let k = kpis::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            name: ActiveValue::set(name.to_string()),
            unit: ActiveValue::set(Some(unit.to_string())),
            direction: ActiveValue::set(dir.to_string()),
            target_value: ActiveValue::set(p.target_value),
            red_value: ActiveValue::set(p.red_value),
            cadence: ActiveValue::set(Some("quarterly".to_string())),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
        members::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            scorecard_pid: ActiveValue::set(sc.pid),
            kpi_pid: ActiveValue::set(k.pid),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
        made.push(k.pid);
    }
    AuditModel::record_or_log(
        &ctx.db,
        sc.pid,
        "scorecard_from_template",
        caller.actor(),
        Some(json!({ "created": made.len() })),
    )
    .await;
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
    let ids: Vec<Uuid> = members::Entity::find()
        .filter(members::Column::ScorecardPid.eq(sc.pid))
        .order_by_asc(members::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|m| m.kpi_pid)
        .collect();
    let all = kpis::Entity::find()
        .filter(kpis::Column::Pid.is_in(ids.clone()))
        .filter(kpis::Column::DeletedAt.is_null())
        .all(&ctx.db)
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
        let readings = kpi_readings::Entity::find()
            .filter(kpi_readings::Column::KpiPid.eq(k.pid))
            .order_by_asc(kpi_readings::Column::ObservedOn)
            .order_by_asc(kpi_readings::Column::Id)
            .all(&ctx.db)
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

#[derive(Debug, Deserialize)]
struct Quarters {
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
    let qs = rules::window(Utc::now().date_naive(), q.quarters.unwrap_or(4));
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

#[derive(Debug, Deserialize)]
struct LevelBody {
    level: String,
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
    let row = levels::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        level: ActiveValue::set(b.level.trim().to_string()),
        note: ActiveValue::set(
            b.note
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty()),
        ),
        set_by: ActiveValue::set(caller.actor().map(str::to_string)),
        set_on: ActiveValue::set(Utc::now().date_naive()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "escalation_level_set",
        caller.actor(),
        Some(json!({ "level": row.level })),
    )
    .await;
    format::json(json!({ "level": row.level, "set_on": row.set_on }))
}

#[derive(Debug, Deserialize)]
struct CriterionBody {
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
    let row = crits::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        criterion: ActiveValue::set(b.criterion.trim().to_string()),
        active: ActiveValue::set(true),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "deescalation_criterion_added",
        caller.actor(),
        None,
    )
    .await;
    format::json(json!({ "pid": row.pid, "criterion": row.criterion }))
}

#[derive(Debug, Deserialize)]
struct EvidenceBody {
    /// `YYYY-Qn`, a finished quarter.
    quarter: String,
    met: bool,
    #[serde(default)]
    link: Option<String>,
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
    let c = crits::Entity::find()
        .filter(crits::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    let Some(q) = Quarter::parse(&b.quarter) else {
        return Err(unprocessable("quarter must be YYYY-Qn"));
    };
    if q > rules::last_finished(Utc::now().date_naive()) {
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
    evidence::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        criterion_pid: ActiveValue::set(c.pid),
        quarter: ActiveValue::set(q.label()),
        met: ActiveValue::set(b.met),
        link: ActiveValue::set(b.link.clone()),
        note: ActiveValue::set(b.note.trim().to_string()),
        recorded_by: ActiveValue::set(caller.actor().map(str::to_string)),
        recorded_on: ActiveValue::set(Utc::now().date_naive()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_or_log(
        &ctx.db,
        c.pid,
        "deescalation_evidence_recorded",
        caller.actor(),
        Some(json!({ "quarter": q.label(), "met": b.met })),
    )
    .await;
    format::json(json!({ "criterion_pid": c.pid, "quarter": q.label(), "met": b.met }))
}

/// The tracker as of `today`, over the plans in `visible`.
///
/// # Errors
///
/// When a database read fails.
#[allow(clippy::too_many_lines)]
pub(crate) async fn tracker_view(
    ctx: &AppContext,
    visible: &HashSet<Uuid>,
    today: chrono::NaiveDate,
    with_detail: bool,
) -> Result<Value> {
    let (need, unusable) = rules::required_quarters();
    let qs = rules::window(today, need.max(4));
    let level = levels::Entity::find()
        .order_by_desc(levels::Column::Id)
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let history = levels::Entity::find()
        .order_by_desc(levels::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let criteria = crits::Entity::find()
        .filter(crits::Column::Active.eq(true))
        .order_by_asc(crits::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let evid = evidence::Entity::find()
        .order_by_asc(evidence::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut latest: HashMap<(Uuid, String), &evidence::Model> = HashMap::new();
    for e in &evid {
        latest.insert((e.criterion_pid, e.quarter.clone()), e);
    }
    let sc = scorecards::Entity::find()
        .filter(scorecards::Column::ForDeescalation.eq(true))
        .one(&ctx.db)
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
    format::json(tracker_view(&ctx, &visible, Utc::now().date_naive(), true).await?)
}

/// The pack's section: the scorecard of the last finished quarter and the tracker without evidence detail.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext, include: &HashSet<Uuid>) -> Result<Value> {
    let mut v = tracker_view(ctx, include, Utc::now().date_naive(), true).await?;
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

//! Validated benefits, per year and non-financial (DP-9). Rules in [`crate::benefit_validation`].
//!
//! What was *realized* is a claim; it is *validated* only by a recorded validation from someone who is
//! not a member of the programme. A member is anyone named on the programme or anything beneath it: its
//! lead, an allocation, a RACI assignment, a task assignee or a private-plan member.

use super::refuse;
use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{MaybeAuthUser, require_auth};
use crate::benefit_validation::{self as rules, Financial};
use crate::models::_entities::{
    allocations, attachments, benefit_validations as validations, benefits, kpi_readings, kpis,
    plan_access_settings, plans, raci_assignments, tasks,
};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    refuse(
        StatusCode::UNPROCESSABLE_ENTITY,
        "unprocessable_entity",
        message,
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add(
            "/plans/{pid}/benefits/{benefit}/classification",
            put(classify),
        )
        .add(
            "/plans/{pid}/benefits/{benefit}/validations",
            post(validate),
        )
        .add("/plans/{pid}/benefits/{benefit}/validations", get(history))
        .add("/portfolio-benefits", get(live))
}

async fn find_benefit(ctx: &AppContext, plan: Uuid, pid: Uuid) -> Result<benefits::Model> {
    benefits::Entity::find()
        .filter(benefits::Column::Pid.eq(pid))
        .filter(benefits::Column::PlanPid.eq(plan))
        .filter(benefits::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)
}

#[derive(Debug, Deserialize)]
struct ClassifyBody {
    #[serde(default)]
    recurrence: Option<String>,
    #[serde(default)]
    annualized_minor: Option<i64>,
    #[serde(default)]
    classification: Option<String>,
    #[serde(default)]
    kpi_pid: Option<Uuid>,
}

/// `PUT …/benefits/{benefit}/classification` — say whether a benefit recurs, its yearly amount, its
/// classification and, for a non-financial one, the KPI that measures it. Fields left out are cleared.
#[debug_handler]
async fn classify(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, benefit)): Path<(String, Uuid)>,
    Json(b): Json<ClassifyBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = find_benefit(&ctx, plan.pid, benefit).await?;
    let mut problems = Vec::new();
    if let Some(r) = &b.recurrence
        && !rules::RECURRENCES.contains(&r.as_str())
    {
        problems.push(format!(
            "recurrence must be one of {:?}",
            rules::RECURRENCES
        ));
    }
    if let Some(c) = &b.classification
        && !rules::CLASSIFICATIONS.contains(&c.as_str())
    {
        problems.push(format!(
            "classification must be one of {:?}",
            rules::CLASSIFICATIONS
        ));
    }
    if b.annualized_minor.is_some() && b.recurrence.as_deref() != Some("recurring") {
        problems.push("annualized_minor belongs to a recurring benefit".to_string());
    }
    if b.annualized_minor.is_some_and(|a| a < 0) {
        problems.push("annualized_minor must not be negative".to_string());
    }
    if b.annualized_minor.is_some() && row.currency.is_none() {
        problems
            .push("a yearly amount needs a financial benefit (one with a currency)".to_string());
    }
    if let Some(k) = b.kpi_pid {
        let exists = kpis::Entity::find()
            .filter(kpis::Column::Pid.eq(k))
            .filter(kpis::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .is_some();
        if !exists {
            problems.push("kpi_pid is not a KPI".to_string());
        }
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut a: benefits::ActiveModel = row.into();
    a.recurrence = ActiveValue::set(b.recurrence.clone());
    a.annualized_minor = ActiveValue::set(b.annualized_minor);
    a.classification = ActiveValue::set(b.classification.clone());
    a.kpi_pid = ActiveValue::set(b.kpi_pid);
    let row = a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "benefit_classified",
        caller.actor(),
        Some(serde_json::json!({ "benefit": row.pid, "recurrence": b.recurrence, "classification": b.classification })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({
        "benefit": row.pid, "recurrence": row.recurrence, "annualized_minor": row.annualized_minor,
        "classification": row.classification, "kpi_pid": row.kpi_pid,
    }))
}

/// Every person reference on the programme (or the plan, if it is under none) and everything beneath it.
async fn members_of_scope(ctx: &AppContext, plan: &plans::Model) -> Result<Vec<String>> {
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let by_pid: HashMap<Uuid, &plans::Model> = all.iter().map(|p| (p.pid, p)).collect();
    // The nearest programme above the plan, else the plan itself.
    let mut root = plan.pid;
    let mut at = Some(plan.pid);
    for _ in 0..64 {
        let Some(p) = at.and_then(|a| by_pid.get(&a)) else {
            break;
        };
        if p.kind.as_deref() == Some("Program") {
            root = p.pid;
            break;
        }
        at = p.parent_pid;
    }
    let mut children: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for p in &all {
        if let Some(par) = p.parent_pid {
            children.entry(par).or_default().push(p.pid);
        }
    }
    let mut scope: HashSet<Uuid> = HashSet::from([root]);
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        for c in children.get(&n).into_iter().flatten() {
            if scope.insert(*c) {
                stack.push(*c);
            }
        }
    }
    let ids: Vec<Uuid> = scope.iter().copied().collect();
    let mut refs: Vec<String> = Vec::new();
    for p in all.iter().filter(|p| scope.contains(&p.pid)) {
        if let Some(l) = p.data.get("lead_ref").and_then(|v| v.as_str()) {
            refs.push(l.to_string());
        }
    }
    refs.extend(
        allocations::Entity::find()
            .filter(allocations::Column::DeletedAt.is_null())
            .filter(allocations::Column::PlanPid.is_in(ids.clone()))
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .map(|a| a.person_ref),
    );
    refs.extend(
        raci_assignments::Entity::find()
            .filter(raci_assignments::Column::DeletedAt.is_null())
            .filter(raci_assignments::Column::PlanPid.is_in(ids.clone()))
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .map(|a| a.person_ref),
    );
    refs.extend(
        tasks::Entity::find()
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::PlanPid.is_in(ids.clone()))
            .filter(tasks::Column::AssigneeRef.is_not_null())
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .filter_map(|t| t.assignee_ref),
    );
    for s in plan_access_settings::Entity::find()
        .filter(plan_access_settings::Column::PlanPid.is_in(ids))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    {
        if let Some(m) = s.members.as_array() {
            refs.extend(m.iter().filter_map(|v| v.as_str().map(str::to_string)));
        }
    }
    Ok(refs)
}

#[derive(Debug, Deserialize)]
struct ValidateBody {
    amount_minor: i64,
    #[serde(default)]
    evidence_attachment_pid: Option<Uuid>,
    #[serde(default)]
    note: Option<String>,
}

/// `POST …/benefits/{benefit}/validations` — an independent person states the amount of the benefit
/// validated to date. Refused (403) from a member of the programme. Append-only; the latest counts.
#[debug_handler]
async fn validate(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, benefit)): Path<(String, Uuid)>,
    Json(b): Json<ValidateBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = find_benefit(&ctx, plan.pid, benefit).await?;
    let Some(currency) = row.currency.clone() else {
        return Err(unprocessable(
            "a non-financial benefit is measured by its KPI, not validated as an amount",
        ));
    };
    if b.amount_minor < 0 {
        return Err(unprocessable("amount_minor must not be negative"));
    }
    if let Some(e) = b.evidence_attachment_pid {
        let ok = attachments::Entity::find()
            .filter(attachments::Column::Pid.eq(e))
            .filter(attachments::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .is_some_and(|a| a.subject_pid == row.pid || a.subject_pid == plan.pid);
        if !ok {
            return Err(unprocessable(
                "evidence_attachment_pid is not an attachment of this benefit or its plan",
            ));
        }
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    if require_auth() {
        let refs = members_of_scope(&ctx, &plan).await?;
        if rules::is_member(&actor, &refs) {
            return Err(refuse(
                StatusCode::FORBIDDEN,
                "forbidden",
                "a validator may not be a member of the programme",
            ));
        }
    }
    let v = validations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        benefit_pid: ActiveValue::set(row.pid),
        plan_pid: ActiveValue::set(plan.pid),
        amount_minor: ActiveValue::set(b.amount_minor),
        currency: ActiveValue::set(currency),
        validator: ActiveValue::set(Some(actor)),
        evidence_attachment_pid: ActiveValue::set(b.evidence_attachment_pid),
        validated_on: ActiveValue::set(Utc::now().date_naive()),
        note: ActiveValue::set(
            b.note
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty()),
        ),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "benefit_validated",
        caller.actor(),
        Some(serde_json::json!({ "benefit": row.pid, "amount_minor": v.amount_minor })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(serde_json::json!({
        "validation": v.pid, "benefit": row.pid, "amount_minor": v.amount_minor, "currency": v.currency,
        "validated_on": v.validated_on,
    }))
}

/// `GET …/benefits/{benefit}/validations` — the history, oldest first, and where the benefit stands.
#[debug_handler]
async fn history(
    State(ctx): State<AppContext>,
    Path((pid, benefit)): Path<(String, Uuid)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = find_benefit(&ctx, plan.pid, benefit).await?;
    let rows = validations::Entity::find()
        .filter(validations::Column::BenefitPid.eq(row.pid))
        .order_by_asc(validations::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let latest = rows.last().map(|v| v.amount_minor);
    format::json(serde_json::json!({
        "benefit": row.pid, "claimed_minor": row.realized_minor, "validated_minor": latest,
        "standing": rules::standing(row.realized_minor, latest).word(),
        "history": rows.iter().map(|v| serde_json::json!({
            "amount_minor": v.amount_minor, "currency": v.currency, "validator": v.validator,
            "validated_on": v.validated_on, "evidence_attachment_pid": v.evidence_attachment_pid,
            "note": v.note })).collect::<Vec<_>>(),
        "note": "Realized is a claim. It is validated only by a recorded validation from someone outside the programme; each validation states the amount validated to date and the latest counts.",
    }))
}

/// The nearest programme at or above a plan, if any in `include`.
fn programme_of(
    by_pid: &HashMap<Uuid, &plans::Model>,
    mut at: Uuid,
    include: &HashSet<Uuid>,
) -> Option<Uuid> {
    for _ in 0..64 {
        let p = by_pid.get(&at)?;
        if p.kind.as_deref() == Some("Program") && include.contains(&p.pid) {
            return Some(p.pid);
        }
        at = p.parent_pid?;
    }
    None
}

fn row_json(r: &rules::Row) -> serde_json::Value {
    serde_json::json!({
        "currency": r.currency, "recurrence": r.recurrence, "classification": r.classification,
        "planned_minor": r.planned_minor, "claimed_minor": r.claimed_minor,
        "validated_minor": r.validated_minor, "unvalidated_claim_minor": r.unvalidated_claim_minor,
        "benefits": r.benefits, "with_no_planned_amount": r.with_no_planned_amount,
        "claimed_only": r.claimed_only,
    })
}

/// The pack's benefits section over the plans in `include`: planned against validated per programme and
/// in total, per currency, recurrence and classification; non-financial benefits by KPI status.
///
/// # Errors
///
/// When a database read fails.
#[allow(clippy::too_many_lines)]
pub(crate) async fn benefits_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
) -> Result<serde_json::Value> {
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let by_pid: HashMap<Uuid, &plans::Model> = all.iter().map(|p| (p.pid, p)).collect();
    let rows = benefits::Entity::find()
        .filter(benefits::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let latest: HashMap<Uuid, i64> = {
        let mut m = HashMap::new();
        for v in validations::Entity::find()
            .order_by_asc(validations::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
        {
            m.insert(v.benefit_pid, v.amount_minor);
        }
        m
    };
    let kpi_rows = kpis::Entity::find()
        .filter(kpis::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let readings = kpi_readings::Entity::find()
        .order_by_asc(kpi_readings::Column::ObservedOn)
        .order_by_asc(kpi_readings::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut last_reading: HashMap<Uuid, i64> = HashMap::new();
    for r in &readings {
        last_reading.insert(r.kpi_pid, r.value);
    }
    let mut financial: Vec<Financial> = Vec::new();
    let mut non_fin: BTreeMap<Uuid, Vec<serde_json::Value>> = BTreeMap::new();
    let mut loose_non_fin: Vec<serde_json::Value> = Vec::new();
    for b in rows.iter().filter(|b| include.contains(&b.plan_pid)) {
        let programme = programme_of(&by_pid, b.plan_pid, include);
        if let Some(currency) = &b.currency {
            let planned = if b.recurrence.as_deref() == Some("recurring") {
                b.annualized_minor
            } else {
                b.target_minor
            };
            financial.push(Financial {
                programme: programme.unwrap_or(Uuid::nil()),
                currency: currency.clone(),
                recurrence: b.recurrence.clone(),
                classification: b.classification.clone(),
                planned_minor: planned,
                claimed_minor: b.realized_minor,
                validated_minor: latest.get(&b.pid).copied(),
            });
        } else {
            let status = match b.kpi_pid.and_then(|k| kpi_rows.iter().find(|x| x.pid == k)) {
                None => {
                    serde_json::json!({ "state": "not measured", "reason": "no KPI is linked to this benefit" })
                }
                Some(k) => {
                    let dir = crate::kpi::Direction::parse(&k.direction);
                    match dir {
                        Some(d) => {
                            let st = crate::kpi::status_of(
                                d,
                                k.target_value,
                                k.red_value,
                                last_reading.get(&k.pid).copied(),
                            );
                            serde_json::json!({ "state": st, "kpi": k.name, "latest": last_reading.get(&k.pid) })
                        }
                        None => {
                            serde_json::json!({ "state": "not measured", "reason": "the KPI has no usable direction" })
                        }
                    }
                }
            };
            let item = serde_json::json!({
                "plan": by_pid.get(&b.plan_pid).map(|p| p.name.clone()), "title": b.title,
                "classification": b.classification, "kpi": status,
            });
            match programme {
                Some(p) => non_fin.entry(p).or_default().push(item),
                None => loose_non_fin.push(item),
            }
        }
    }
    let grouped = rules::group(&financial);
    let mut progs: Vec<&plans::Model> = all
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    progs.sort_by(|a, b| a.name.cmp(&b.name).then(a.pid.cmp(&b.pid)));
    let programmes: Vec<serde_json::Value> = progs
        .iter()
        .map(|p| {
            serde_json::json!({
                "plan_pid": p.pid, "name": p.name,
                "financial": grouped.iter().filter(|r| r.programme == p.pid).map(row_json).collect::<Vec<_>>(),
                "non_financial": non_fin.get(&p.pid).cloned().unwrap_or_default(),
            })
        })
        .collect();
    // Totals across programmes: still one row per currency, recurrence and classification.
    let totals = {
        let mut t: Vec<Financial> = financial.clone();
        for f in &mut t {
            f.programme = Uuid::nil();
        }
        rules::group(&t).iter().map(row_json).collect::<Vec<_>>()
    };
    Ok(serde_json::json!({
        "programmes": programmes,
        "not_under_a_programme": {
            "financial": grouped.iter().filter(|r| r.programme == Uuid::nil()).map(row_json).collect::<Vec<_>>(),
            "non_financial": loose_non_fin,
        },
        "totals": totals,
        "note": "Planned is the yearly amount for a recurring benefit and the target for a one-off one; the two are never added together, nor are classifications or currencies. Claimed is what was realized and recorded; validated is the latest amount an independent person (not a member of the programme) recorded. Realized but not validated is claimed, never validated. For a recurring benefit the realized and validated amounts are read as yearly figures, as entered. A non-financial benefit reports its KPI's status, or not measured when it has no KPI.",
    }))
}

/// `GET /api/portfolio-benefits` — validated benefits for the programmes you can read.
#[debug_handler]
async fn live(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    format::json(benefits_section(&ctx, &readable).await?)
}

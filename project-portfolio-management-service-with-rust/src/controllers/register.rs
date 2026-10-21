//! Register completeness (EX-1): `GET /api/register-completeness` and the check behind "no register, no
//! money". The rules are [`crate::register_completeness`]; this reads what is on record.
//!
//! Programmes are plans of kind Program; their plans are the programme and everything under it (by parent).
//! Only plans the caller may read are listed.

use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    budget_lines, capacity_commitments, delivery_confidence_assessments as assessments,
    milestone_signoff_requirements, milestones, plan_dependencies, plans, programme_profiles,
    risks, schedule_baselines,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::register_completeness::{self as rules, Assessment, Facts};

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

/// What is gathered per programme.
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
struct Acc {
    any_line: bool,
    unfunded: bool,
    risk: bool,
    dep: bool,
    partner: bool,
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/register-completeness", get(list))
}

/// Each programme in `include` with its assessment, ordered by name.
///
/// # Errors
///
/// When a database read fails.
#[allow(clippy::too_many_lines)]
pub(crate) async fn assess_programmes(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
) -> Result<Vec<(plans::Model, Assessment)>> {
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut programmes: Vec<&plans::Model> = all
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    programmes.sort_by(|a, b| a.name.cmp(&b.name).then(a.pid.cmp(&b.pid)));
    // The programme each plan belongs to (itself, or its nearest ancestor of kind Program).
    let parent: HashMap<Uuid, Option<Uuid>> = all.iter().map(|p| (p.pid, p.parent_pid)).collect();
    let kind: HashMap<Uuid, bool> = all
        .iter()
        .map(|p| (p.pid, p.kind.as_deref() == Some("Program")))
        .collect();
    let programme_of = |start: Uuid| -> Option<Uuid> {
        let mut cur = Some(start);
        for _ in 0..64 {
            let c = cur?;
            if kind.get(&c).copied().unwrap_or(false) {
                return Some(c);
            }
            cur = parent.get(&c).copied().flatten();
        }
        None
    };

    let lines = budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let risk_rows = risks::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let assessed: HashSet<Uuid> = assessments::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|a| a.plan_pid)
        .collect();
    let baselined: HashSet<Uuid> = schedule_baselines::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|b| b.plan_pid)
        .collect();
    let deps = plan_dependencies::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let commitments: Vec<(Uuid, bool)> = capacity_commitments::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|c| (c.plan_pid, c.organization_pid.is_some()))
        .collect();
    let required_ms: HashSet<Uuid> = milestone_signoff_requirements::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|r| r.milestone_pid)
        .collect();
    let ms_plan: Vec<Uuid> = milestones::Entity::find()
        .filter(milestones::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|m| required_ms.contains(&m.pid))
        .map(|m| m.plan_pid)
        .collect();
    let models: HashMap<Uuid, Option<String>> = programme_profiles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| (p.plan_pid, p.delivery_model))
        .collect();

    let mut acc: HashMap<Uuid, Acc> = HashMap::new();
    let mut tally = |plan: Uuid, f: &mut dyn FnMut(&mut Acc)| {
        if let Some(p) = programme_of(plan) {
            f(acc.entry(p).or_default());
        }
    };
    for l in &lines {
        tally(l.plan_pid, &mut |a| {
            a.any_line = true;
            a.unfunded |= l.funding_source_pid.is_none();
        });
    }
    for r in &risk_rows {
        tally(r.plan_pid, &mut |a| a.risk = true);
    }
    for d in &deps {
        tally(d.predecessor_pid, &mut |a| a.dep = true);
        tally(d.successor_pid, &mut |a| a.dep = true);
    }
    for (plan, named) in &commitments {
        if *named {
            tally(*plan, &mut |a| a.partner = true);
        }
    }
    for plan in &ms_plan {
        tally(*plan, &mut |a| a.partner = true);
    }
    let in_programme =
        |prog: Uuid, with: &HashSet<Uuid>| with.iter().any(|p| programme_of(*p) == Some(prog));

    Ok(programmes
        .into_iter()
        .map(|p| {
            let a = acc.remove(&p.pid).unwrap_or_default();
            let model = models.get(&p.pid).cloned().flatten();
            let facts = Facts {
                sro: p
                    .data
                    .get("lead_ref")
                    .and_then(Value::as_str)
                    .is_some_and(|s| !s.trim().is_empty()),
                // A programme delivered with partners needs them on record; one that is not stated, or is
                // a central product, is not asked.
                partners_required: matches!(
                    model.as_deref(),
                    Some("central_platform_local_deployment" | "local_programme_central_standards")
                ),
                has_partners: a.partner,
                stage: p.stage.is_some(),
                baseline: in_programme(p.pid, &baselined),
                target_date: p
                    .data
                    .get("target_date")
                    .and_then(Value::as_str)
                    .is_some_and(|s| !s.trim().is_empty()),
                budget: a.any_line,
                all_funded: a.any_line.then_some(!a.unfunded),
                confidence: in_programme(p.pid, &assessed),
                risks: a.risk,
                dependencies: a.dep,
            };
            (p.clone(), rules::assess(&facts))
        })
        .collect())
}

fn row(p: &plans::Model, a: &Assessment) -> Value {
    json!({
        "plan_pid": p.pid, "name": p.name, "complete": a.complete(),
        "score_pct": f64::from(a.score_bp) / 100.0,
        "present": a.present, "applicable": a.applicable,
        "missing": a.missing, "not_applicable": a.not_applicable,
    })
}

/// The pack's data-quality entry: every incomplete programme, each missing field named.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext, include: &HashSet<Uuid>) -> Result<Value> {
    let all = assess_programmes(ctx, include).await?;
    let incomplete: Vec<Value> = all
        .iter()
        .filter(|(_, a)| !a.complete())
        .map(|(p, a)| row(p, a))
        .collect();
    Ok(json!({
        "programmes": all.len(), "complete": all.len() - incomplete.len(),
        "incomplete": incomplete, "enforced": rules::enforced(),
    }))
}

/// `GET /api/register-completeness` — every programme you can read, with what is missing.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let rows = assess_programmes(&ctx, &readable).await?;
    format::json(json!({
        "enforced": rules::enforced(),
        "programmes": rows.iter().map(|(p, a)| row(p, a)).collect::<Vec<_>>(),
        "fields": rules::FIELDS,
        "note": "The score is the share of applicable fields that are on record, and each missing field is named. Partners are asked of a programme only where its delivery model has partners; a funding source only where there is a budget line. The programme's lead stands for the senior responsible owner (there is no separate role). With PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED on, releasing a budget line or booking a firm allocation on an incomplete programme is refused, naming the fields; off (the default), the action succeeds and the warning is audited.",
    }))
}

/// The "no register, no money" check for an action on `plan` (a plan in, or being, a programme). With the
/// switch on, an incomplete programme refuses the action (422) naming the fields; with it off, the action
/// goes ahead and the warning is audited. A plan under no programme is not checked.
///
/// # Errors
///
/// 422 when enforced and the programme is incomplete; a database error otherwise.
pub(crate) async fn guard(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    action: &str,
) -> Result<()> {
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let by: HashMap<Uuid, &plans::Model> = all.iter().map(|p| (p.pid, p)).collect();
    let mut cur = Some(plan);
    let mut prog = None;
    for _ in 0..64 {
        let Some(c) = cur else { break };
        let Some(p) = by.get(&c) else { break };
        if p.kind.as_deref() == Some("Program") {
            prog = Some(c);
            break;
        }
        cur = p.parent_pid;
    }
    let Some(prog) = prog else { return Ok(()) };
    let rows = assess_programmes(ctx, &HashSet::from([prog])).await?;
    let Some((p, a)) = rows.first() else {
        return Ok(());
    };
    if a.complete() {
        return Ok(());
    }
    if rules::enforced() {
        return Err(Error::CustomError(
            StatusCode::UNPROCESSABLE_ENTITY,
            ErrorDetail::new(
                "register_incomplete",
                format!(
                    "{action} is refused: the register of programme {} is incomplete; missing: {}",
                    p.name,
                    a.missing.join(", ")
                ),
            ),
        ));
    }
    AuditModel::record_or_log(
        &ctx.db,
        p.pid,
        "register_incomplete_warning",
        caller.actor(),
        Some(json!({ "action": action, "missing": a.missing })),
    )
    .await;
    Ok(())
}

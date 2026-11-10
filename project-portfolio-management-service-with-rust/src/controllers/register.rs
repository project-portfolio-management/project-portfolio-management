//! Register completeness (EX-1): `GET /api/register-completeness` and the check behind "no register, no
//! money". The rules are [`crate::register_completeness`]; this reads what is on record.
//!
//! Programmes are plans of kind Program; their plans are the programme and everything under it (by parent).
//! Only plans the caller may read are listed.

use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::plans;
use crate::models::audit_logs::Model as AuditModel;
use crate::register_completeness::{self as rules, Assessment, Facts};

/// What is gathered per programme.
#[derive(Default)]
#[expect(clippy::struct_excessive_bools)]
struct Acc {
    /// True when the programme (or a plan under it) has at least one budget line.
    any_line: bool,
    /// True when any of those budget lines has no funding source.
    unfunded: bool,
    /// True when at least one risk is registered under the programme.
    risk: bool,
    /// True when a dependency touches the programme, as predecessor or successor.
    dep: bool,
    /// True when a named partner commitment or a required partner milestone exists.
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
#[expect(clippy::too_many_lines)]
pub(crate) async fn assess_programmes(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
) -> Result<Vec<(plans::Model, Assessment)>> {
    let all = live_plans(ctx).await?;
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

    let db = crate::db::pool(ctx);
    let lines: Vec<Line> = sqlx::query_as(
        "SELECT plan_pid, funding_source_pid FROM budget_lines WHERE deleted_at IS NULL",
    )
    .fetch_all(db)
    .await?;
    let risk_rows: Vec<Uuid> = sqlx::query_scalar("SELECT plan_pid FROM risks")
        .fetch_all(db)
        .await?;
    let assessed: HashSet<Uuid> =
        sqlx::query_scalar::<_, Uuid>("SELECT plan_pid FROM delivery_confidence_assessments")
            .fetch_all(db)
            .await?
            .into_iter()
            .collect();
    let baselined: HashSet<Uuid> =
        sqlx::query_scalar::<_, Uuid>("SELECT plan_pid FROM schedule_baselines")
            .fetch_all(db)
            .await?
            .into_iter()
            .collect();
    let deps: Vec<(Uuid, Uuid)> =
        sqlx::query_as("SELECT predecessor_pid, successor_pid FROM plan_dependencies")
            .fetch_all(db)
            .await?;
    let commitments: Vec<(Uuid, bool)> =
        sqlx::query_as("SELECT plan_pid, organization_pid IS NOT NULL FROM capacity_commitments")
            .fetch_all(db)
            .await?;
    let required_ms: HashSet<Uuid> =
        sqlx::query_scalar::<_, Uuid>("SELECT milestone_pid FROM milestone_signoff_requirements")
            .fetch_all(db)
            .await?
            .into_iter()
            .collect();
    let ms_plan: Vec<Uuid> = sqlx::query_as::<_, (Uuid, Uuid)>(
        "SELECT pid, plan_pid FROM milestones WHERE deleted_at IS NULL",
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .filter(|(pid, _)| required_ms.contains(pid))
    .map(|(_, plan)| plan)
    .collect();
    let models: HashMap<Uuid, Option<String>> =
        sqlx::query_as("SELECT plan_pid, delivery_model FROM programme_profiles")
            .fetch_all(db)
            .await?
            .into_iter()
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
        tally(*r, &mut |a| a.risk = true);
    }
    for (predecessor, successor) in &deps {
        tally(*predecessor, &mut |a| a.dep = true);
        tally(*successor, &mut |a| a.dep = true);
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

/// A live budget line: its plan and its funding source, if any.
#[derive(sqlx::FromRow)]
struct Line {
    /// The plan the line is on.
    plan_pid: Uuid,
    /// Its funding source; none is unfunded.
    funding_source_pid: Option<Uuid>,
}

/// Every plan that is not deleted, in no particular order.
async fn live_plans(ctx: &AppContext) -> Result<Vec<plans::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plans WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// One programme's completeness as JSON: the score as a percentage (stored in basis points, so divided by 100) plus the present, applicable, missing and not-applicable fields.
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
    let all: Vec<Uuid> = sqlx::query_scalar("SELECT pid FROM plans WHERE deleted_at IS NULL")
        .fetch_all(crate::db::pool(&ctx))
        .await?;
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
    let all = live_plans(ctx).await?;
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
    AuditModel::record_or_log_on(
        crate::db::pool(ctx),
        p.pid,
        "register_incomplete_warning",
        caller.actor(),
        Some(json!({ "action": action, "missing": a.missing })),
    )
    .await;
    Ok(())
}

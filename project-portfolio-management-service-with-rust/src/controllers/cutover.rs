//! Go / no-go for safety-critical cutovers (EX-6). Rules in [`crate::cutover`].
//!
//! A decision starts as a **draft** with its stop criteria, is **published**, and only then **opened**: from
//! the moment it is published its criteria cannot be added to, edited or removed (the database refuses), so a
//! criterion cannot be moved to suit the answer; a changed criterion is a **new version**. While open, each
//! criterion is marked met or not (with evidence) and the configured roles co-sign. Then it is decided:
//! **go** needs every criterion met and every co-signature; **no-go** and **delay for safety** are decisions
//! in their own right and the pack shows them as such, not as slippage.
//!
//! Marking and deciding are by anyone who may write to the plan; a co-signature needs the signer's token to
//! carry the role in its `cutover_role` attribute.

#![expect(clippy::many_single_char_names)]

use super::conflict;
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::db_err;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cutover as rules;
use crate::models::_entities::{
    cutover_cosignatures as cosigs, cutover_criteria as criteria, cutover_decisions as decisions,
    plans,
};
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/cutover-decisions", post(create))
        .add("/plans/{pid}/cutover-decisions", get(list))
        .add(
            "/plans/{pid}/cutover-decisions/{did}/criteria",
            put(replace_criteria),
        )
        .add(
            "/plans/{pid}/cutover-decisions/{did}/publish",
            post(publish),
        )
        .add("/plans/{pid}/cutover-decisions/{did}/open", post(open))
        .add(
            "/plans/{pid}/cutover-decisions/{did}/criteria/{n}/mark",
            put(mark),
        )
        .add("/plans/{pid}/cutover-decisions/{did}/cosign", post(cosign))
        .add("/plans/{pid}/cutover-decisions/{did}/decide", post(decide))
        .add(
            "/plans/{pid}/cutover-decisions/{did}/new-version",
            post(new_version),
        )
}

async fn load(ctx: &AppContext, pid: &str, did: &str) -> Result<(plans::Model, decisions::Model)> {
    let plan = super::governance::find_item(ctx, pid).await?;
    let did = super::parse_pid(did)?;
    let d = decisions::Entity::find()
        .filter(decisions::Column::Pid.eq(did))
        .filter(decisions::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    Ok((plan, d))
}

async fn parts(
    ctx: &AppContext,
    d: &decisions::Model,
) -> Result<(Vec<criteria::Model>, Vec<cosigs::Model>)> {
    let c = criteria::Entity::find()
        .filter(criteria::Column::DecisionPid.eq(d.pid))
        .order_by_asc(criteria::Column::Position)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let s = cosigs::Entity::find()
        .filter(cosigs::Column::DecisionPid.eq(d.pid))
        .order_by_asc(cosigs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    Ok((c, s))
}

fn view(d: &decisions::Model, c: &[criteria::Model], s: &[cosigs::Model]) -> Value {
    let required = rules::required_roles();
    let signed: Vec<&str> = s.iter().map(|x| x.role.as_str()).collect();
    json!({
        "pid": d.pid, "plan_pid": d.plan_pid, "title": d.title, "version": d.version,
        "supersedes_pid": d.supersedes_pid, "status": d.status,
        "published_on": d.published_on, "opened_on": d.opened_on,
        "outcome": d.outcome, "decided_on": d.decided_on, "decision_reason": d.decision_reason,
        "criteria": c.iter().map(|x| json!({ "n": x.position, "criterion": x.criterion, "met": x.met, "evidence": x.evidence, "marked_on": x.marked_on })).collect::<Vec<_>>(),
        "cosignatures": { "required": required, "signed": s.iter().map(|x| json!({ "role": x.role, "signed_on": x.signed_on })).collect::<Vec<_>>(),
            "missing": required.iter().filter(|r| !signed.contains(&r.as_str())).collect::<Vec<_>>() },
        "go_would_be_refused_because": if d.status == "open" { rules::go_problems(&c.iter().map(|x| x.met).collect::<Vec<_>>(), &required, &signed.iter().map(|x| (*x).to_string()).collect::<Vec<_>>()) } else { vec![] },
    })
}

fn clean(list: &[String]) -> Result<Vec<String>> {
    let items: Vec<String> = list.iter().map(|x| x.trim().to_string()).collect();
    if !(1..=30).contains(&items.len()) {
        return Err(unprocessable("give 1 to 30 stop criteria"));
    }
    if items
        .iter()
        .any(|x| !(5..=500).contains(&x.chars().count()))
    {
        return Err(unprocessable("each stop criterion is 5 to 500 characters"));
    }
    Ok(items)
}

async fn put_criteria(ctx: &AppContext, decision: Uuid, items: &[String]) -> Result<()> {
    for (i, text) in items.iter().enumerate() {
        criteria::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            decision_pid: ActiveValue::set(decision),
            position: ActiveValue::set(i32::try_from(i + 1).unwrap_or(i32::MAX)),
            criterion: ActiveValue::set(text.clone()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?;
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct New {
    title: String,
    criteria: Vec<String>,
}

/// `POST /api/plans/{pid}/cutover-decisions` — a draft, version 1.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<New>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if !(3..=200).contains(&b.title.trim().chars().count()) {
        return Err(unprocessable("title is 3 to 200 characters"));
    }
    let items = clean(&b.criteria)?;
    let d = decisions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        title: ActiveValue::set(b.title.trim().to_string()),
        version: ActiveValue::set(1),
        status: ActiveValue::set("draft".to_string()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("cd_version") {
            conflict("a decision with that title exists for this plan: make a new version of it")
        } else {
            db_err(e)
        }
    })?;
    put_criteria(&ctx, d.pid, &items).await?;
    AuditModel::record_or_log(
        &ctx.db,
        d.pid,
        "cutover_decision_drafted",
        caller.actor(),
        None,
    )
    .await;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

/// `GET /api/plans/{pid}/cutover-decisions`
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = decisions::Entity::find()
        .filter(decisions::Column::PlanPid.eq(plan.pid))
        .order_by_desc(decisions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = Vec::new();
    for d in &rows {
        let (c, s) = parts(&ctx, d).await?;
        out.push(view(d, &c, &s));
    }
    format::json(json!({ "decisions": out, "required_cosigners": rules::required_roles() }))
}

#[derive(Debug, Deserialize)]
struct Criteria {
    criteria: Vec<String>,
}

/// `PUT …/criteria` — replace the criteria of a draft.
#[debug_handler]
async fn replace_criteria(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
    Json(b): Json<Criteria>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "draft" {
        return Err(conflict(
            "the criteria of a published decision cannot be edited: make a new version",
        ));
    }
    let items = clean(&b.criteria)?;
    criteria::Entity::delete_many()
        .filter(criteria::Column::DecisionPid.eq(d.pid))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    put_criteria(&ctx, d.pid, &items).await?;
    AuditModel::record_or_log(
        &ctx.db,
        d.pid,
        "cutover_criteria_edited",
        caller.actor(),
        Some(json!({ "count": items.len() })),
    )
    .await;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

async fn advance(
    ctx: &AppContext,
    d: decisions::Model,
    from: &str,
    to: &str,
    action: &str,
    caller: &MaybeAuthUser,
) -> Result<Response> {
    if d.status != from {
        return Err(conflict(&format!(
            "this decision is {}, not {from}",
            d.status
        )));
    }
    let today = crate::tz::utc_today();
    let mut a: decisions::ActiveModel = d.into();
    a.status = ActiveValue::set(to.to_string());
    if to == "published" {
        a.published_on = ActiveValue::set(Some(today));
    } else {
        a.opened_on = ActiveValue::set(Some(today));
    }
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(&ctx.db, row.pid, action, caller.actor(), None).await;
    let (c, s) = parts(ctx, &row).await?;
    format::json(view(&row, &c, &s))
}

/// `POST …/publish` — freeze the criteria.
#[debug_handler]
async fn publish(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    advance(
        &ctx,
        d,
        "draft",
        "published",
        "cutover_criteria_published",
        &caller,
    )
    .await
}

/// `POST …/open` — open the decision (the criteria were published first, and stay as they are).
#[debug_handler]
async fn open(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    advance(
        &ctx,
        d,
        "published",
        "open",
        "cutover_decision_opened",
        &caller,
    )
    .await
}

#[derive(Debug, Deserialize)]
struct Mark {
    met: bool,
    evidence: String,
}

/// `PUT …/criteria/{n}/mark` — mark a criterion met or not, with evidence.
#[debug_handler]
async fn mark(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did, n)): Path<(String, String, i32)>,
    Json(b): Json<Mark>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "open" {
        return Err(conflict("criteria are marked while the decision is open"));
    }
    if b.evidence.trim().chars().count() < 5 {
        return Err(unprocessable(
            "evidence of at least 5 characters is required",
        ));
    }
    let c = criteria::Entity::find()
        .filter(criteria::Column::DecisionPid.eq(d.pid))
        .filter(criteria::Column::Position.eq(n))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut a: criteria::ActiveModel = c.into();
    a.met = ActiveValue::set(Some(b.met));
    a.evidence = ActiveValue::set(Some(b.evidence.trim().to_string()));
    a.marked_by = ActiveValue::set(caller.actor().map(str::to_string));
    a.marked_on = ActiveValue::set(Some(crate::tz::utc_today()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        d.pid,
        "cutover_criterion_marked",
        caller.actor(),
        Some(json!({ "n": n, "met": b.met })),
    )
    .await;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

#[derive(Debug, Deserialize)]
struct Cosign {
    role: String,
}

/// `POST …/cosign` — co-sign for a role; the caller's token must carry it in `cutover_role`.
#[debug_handler]
async fn cosign(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
    Json(b): Json<Cosign>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "open" {
        return Err(conflict(
            "co-signatures are given while the decision is open",
        ));
    }
    let role = b.role.trim().to_lowercase();
    if !rules::required_roles().contains(&role) {
        return Err(unprocessable(&format!(
            "role must be one of {:?}",
            rules::required_roles()
        )));
    }
    let holds = caller.holds_or_open(|c| {
        c.attrs
            .get("cutover_role")
            .is_some_and(|v| v.iter().any(|r| r.eq_ignore_ascii_case(&role)))
    });
    if !holds {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new(
                "forbidden",
                "only a person holding that role (the cutover_role attribute) co-signs for it",
            ),
        ));
    }
    cosigs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        decision_pid: ActiveValue::set(d.pid),
        role: ActiveValue::set(role.clone()),
        signed_by: ActiveValue::set(caller.actor().map(str::to_string)),
        signed_on: ActiveValue::set(crate::tz::utc_today()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("ccs_once") {
            conflict("that role has already co-signed")
        } else {
            db_err(e)
        }
    })?;
    AuditModel::record_or_log(
        &ctx.db,
        d.pid,
        "cutover_cosigned",
        caller.actor(),
        Some(json!({ "role": role })),
    )
    .await;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

#[derive(Debug, Deserialize)]
struct Decide {
    outcome: String,
    reason: String,
}

/// `POST …/decide` — go, no-go or delay for safety.
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
    Json(b): Json<Decide>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "open" {
        return Err(conflict("only an open decision is decided"));
    }
    if !rules::OUTCOMES.contains(&b.outcome.as_str()) {
        return Err(unprocessable(&format!(
            "outcome must be one of {:?}",
            rules::OUTCOMES
        )));
    }
    if b.reason.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a reason of at least 10 characters is required",
        ));
    }
    let (c, s) = parts(&ctx, &d).await?;
    if b.outcome == "go" {
        let signed: Vec<String> = s.iter().map(|x| x.role.clone()).collect();
        let problems = rules::go_problems(
            &c.iter().map(|x| x.met).collect::<Vec<_>>(),
            &rules::required_roles(),
            &signed,
        );
        if !problems.is_empty() {
            return Err(unprocessable(&format!(
                "a go is refused: {}",
                problems.join("; ")
            )));
        }
    }
    let mut a: decisions::ActiveModel = d.into();
    a.status = ActiveValue::set("decided".to_string());
    a.outcome = ActiveValue::set(Some(b.outcome.clone()));
    a.decided_on = ActiveValue::set(Some(crate::tz::utc_today()));
    a.decided_by = ActiveValue::set(caller.actor().map(str::to_string));
    a.decision_reason = ActiveValue::set(Some(b.reason.trim().to_string()));
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "cutover_decided",
        caller.actor(),
        Some(json!({ "outcome": b.outcome })),
    )
    .await;
    format::json(view(&row, &c, &s))
}

/// `POST …/new-version` — a new draft with the same criteria, to change them. A published or open decision is
/// superseded; a decided one stays as decided (the new version supersedes it by reference).
#[debug_handler]
async fn new_version(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
) -> Result<Response> {
    let (plan, d) = load(&ctx, &pid, &did).await?;
    if d.status == "draft" || d.status == "superseded" {
        return Err(conflict(
            "only a published, open or decided decision has a new version",
        ));
    }
    if d.outcome.as_deref() == Some("go") {
        return Err(conflict("a decision that went ahead is final"));
    }
    let latest = decisions::Entity::find()
        .filter(decisions::Column::PlanPid.eq(plan.pid))
        .filter(sea_orm::sea_query::Expr::cust_with_values(
            "lower(title) = lower($1)",
            [d.title.clone()],
        ))
        .order_by_desc(decisions::Column::Version)
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if latest.pid != d.pid {
        return Err(conflict("there is already a newer version"));
    }
    let (old_c, _) = parts(&ctx, &d).await?;
    if d.status != "decided" {
        let mut a: decisions::ActiveModel = d.clone().into();
        a.status = ActiveValue::set("superseded".to_string());
        a.update(&ctx.db).await.map_err(db_err)?;
    }
    let nd = decisions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        title: ActiveValue::set(d.title.clone()),
        version: ActiveValue::set(d.version + 1),
        supersedes_pid: ActiveValue::set(Some(d.pid)),
        status: ActiveValue::set("draft".to_string()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    put_criteria(
        &ctx,
        nd.pid,
        &old_c
            .iter()
            .map(|x| x.criterion.clone())
            .collect::<Vec<_>>(),
    )
    .await?;
    AuditModel::record_or_log(
        &ctx.db,
        nd.pid,
        "cutover_decision_new_version",
        caller.actor(),
        Some(json!({ "supersedes": d.pid })),
    )
    .await;
    let (c, s) = parts(&ctx, &nd).await?;
    format::json(view(&nd, &c, &s))
}

/// The pack's section: the decisions made in the period, each as what it is. A no-go or a delay for safety is
/// a safety decision and is counted apart; it is not slippage.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(
    ctx: &AppContext,
    include: &std::collections::HashSet<Uuid>,
    period: &str,
) -> Result<Value> {
    let names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .filter(|p| include.contains(&p.pid))
        .map(|p| (p.pid, p.name))
        .collect();
    let rows: Vec<decisions::Model> = decisions::Entity::find()
        .filter(decisions::Column::Status.eq("decided"))
        .order_by_asc(decisions::Column::DecidedOn)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .filter(|d| {
            names.contains_key(&d.plan_pid)
                && d.decided_on
                    .is_some_and(|x| x.format("%Y-%m").to_string() == period)
        })
        .collect();
    let count = |o: &str| {
        rows.iter()
            .filter(|d| d.outcome.as_deref() == Some(o))
            .count()
    };
    Ok(json!({
        "decisions": rows.iter().map(|d| json!({ "plan": names[&d.plan_pid], "title": d.title, "version": d.version, "outcome": d.outcome, "decided_on": d.decided_on })).collect::<Vec<_>>(),
        "counts": { "go": count("go"), "no_go": count("no_go"), "delay_for_safety": count("delay_for_safety") },
        "note": "A cutover decided in the period. A no-go or a delay for safety is a decision taken on the stop criteria, and is not counted as slippage.",
    }))
}

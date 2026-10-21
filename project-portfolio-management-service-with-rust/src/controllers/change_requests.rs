//! Change requests (IR-4, IR-5). Rules and the state machine in [`crate::change_requests`].
//!
//! Plan-scoped (so private plans, handling marks and break-glass apply as for everything else under
//! `/api/plans/{pid}`). The requester drafts and submits; **one named approver, who is not the requester,
//! decides** and must give a reason; the approver is the only person who can review, approve or reject;
//! an approved change is then marked implemented. A schedule or cost impact is whatever the requester
//! stated, and, if they attached what-if edits, a **forecast** from the scheduler (labelled as one). A
//! baseline taken because of a change request is taken by a person who chooses to, never automatically.
//! When login is off (a development profile) the identity checks are skipped, as for every write.

use super::refuse;
use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use super::governance::find_item;
use super::unprocessable_entity as unprocessable;
use crate::auth::{MaybeAuthUser, require_auth};
use crate::change_requests::{self as rules, Action};
use crate::models::_entities::{change_request_risks, change_requests, issues, risks};
use crate::models::audit_logs::Model as AuditModel;
use crate::sandbox::{self, Edit};

fn actor_of(caller: &MaybeAuthUser) -> String {
    caller
        .actor()
        .map_or_else(|| "anonymous".to_string(), str::to_string)
}

/// Whether the identity checks apply (login is on). Off: every caller may do every step, as for any write.
fn checks_apply() -> bool {
    require_auth()
}

async fn find_cr(ctx: &AppContext, plan: Uuid, pid: &str) -> Result<change_requests::Model> {
    let pid = super::parse_pid(pid)?;
    change_requests::Entity::find()
        .filter(change_requests::Column::Pid.eq(pid))
        .filter(change_requests::Column::PlanPid.eq(plan))
        .filter(change_requests::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

async fn links_of(ctx: &AppContext, cr: Uuid) -> Result<(Vec<Uuid>, Vec<Uuid>)> {
    let issue_pids = issues::Entity::find()
        .filter(issues::Column::ChangeRequestPid.eq(cr))
        .filter(issues::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|i| i.pid)
        .collect();
    let risk_pids = change_request_risks::Entity::find()
        .filter(change_request_risks::Column::ChangeRequestPid.eq(cr))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| r.risk_pid)
        .collect();
    Ok((issue_pids, risk_pids))
}

fn view(c: &change_requests::Model, links: Option<&(Vec<Uuid>, Vec<Uuid>)>) -> serde_json::Value {
    let today = Utc::now().date_naive();
    serde_json::json!({
        "pid": c.pid, "plan_pid": c.plan_pid, "title": c.title, "description": c.description,
        "reason": c.reason, "kind": c.kind,
        "impact": {
            "schedule_business_days": c.impact_schedule_days,
            "cost_minor": c.impact_cost_minor, "cost_currency": c.cost_currency,
            "summary": c.impact_summary, "has_forecast_edits": c.edits.is_some(),
        },
        "edits": c.edits,
        "status": c.status, "requested_by": c.requested_by, "requested_on": c.requested_on,
        "age_calendar_days": rules::age_calendar_days(c.requested_on, today),
        "approver_ref": c.approver_ref, "decided_by": c.decided_by, "decided_on": c.decided_on,
        "decision_reason": c.decision_reason, "implemented_on": c.implemented_on,
        "baseline_pid": c.baseline_pid,
        "issue_pids": links.map(|l| l.0.clone()), "risk_pids": links.map(|l| l.1.clone()),
    })
}

#[derive(Debug, Deserialize)]
struct Input {
    title: String,
    description: String,
    reason: String,
    #[serde(default = "default_kind")]
    kind: String,
    #[serde(default)]
    impact_schedule_days: Option<i64>,
    #[serde(default)]
    impact_cost_minor: Option<i64>,
    #[serde(default)]
    cost_currency: Option<String>,
    #[serde(default)]
    impact_summary: Option<String>,
    /// What-if edits (the sandbox's edit list) whose forecast is shown as the impact.
    #[serde(default)]
    edits: Option<Vec<Edit>>,
}

fn default_kind() -> String {
    "other".to_string()
}

fn validate(b: &Input) -> Result<()> {
    let mut problems = rules::problems(&rules::Draft {
        title: &b.title,
        description: &b.description,
        reason: &b.reason,
        kind: &b.kind,
        impact_schedule_days: b.impact_schedule_days,
        impact_cost_minor: b.impact_cost_minor,
        cost_currency: b.cost_currency.as_deref(),
        impact_summary: b.impact_summary.as_deref(),
    });
    if let Some(edits) = &b.edits {
        if edits.is_empty() {
            problems.push("edits, when given, cannot be empty".to_string());
        }
        problems.extend(sandbox::check(edits));
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(unprocessable(&problems.join("; ")))
    }
}

fn edits_json(b: &Input) -> Option<serde_json::Value> {
    b.edits.as_ref().and_then(|e| serde_json::to_value(e).ok())
}

/// `POST /api/plans/{pid}/change-requests` — draft one.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Input>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    validate(&b)?;
    let row = change_requests::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        title: ActiveValue::set(b.title.trim().to_string()),
        description: ActiveValue::set(b.description.trim().to_string()),
        reason: ActiveValue::set(b.reason.trim().to_string()),
        kind: ActiveValue::set(b.kind.clone()),
        impact_schedule_days: ActiveValue::set(
            b.impact_schedule_days.and_then(|n| i32::try_from(n).ok()),
        ),
        impact_cost_minor: ActiveValue::set(b.impact_cost_minor),
        cost_currency: ActiveValue::set(b.cost_currency.clone()),
        impact_summary: ActiveValue::set(
            b.impact_summary
                .as_ref()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
        ),
        edits: ActiveValue::set(edits_json(&b)),
        status: ActiveValue::set("draft".to_string()),
        requested_by: ActiveValue::set(actor_of(&caller)),
        requested_on: ActiveValue::set(Utc::now().date_naive()),
        approver_ref: ActiveValue::set(None),
        decided_by: ActiveValue::set(None),
        decided_on: ActiveValue::set(None),
        decision_reason: ActiveValue::set(None),
        implemented_on: ActiveValue::set(None),
        baseline_pid: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "change_request_drafted",
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan.pid, "title": row.title, "kind": row.kind })),
    )
    .await;
    format::json(view(&row, None))
}

/// `PUT /api/plans/{pid}/change-requests/{cr}` — change a draft (its requester only).
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
    Json(b): Json<Input>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    if !rules::is_editable(&row.status) {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "only a draft can be edited; withdraw this one and draft another",
        ));
    }
    if checks_apply() && !rules::is_person(&row.requested_by, caller.actor()) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "only the person who drafted it can edit it",
        ));
    }
    validate(&b)?;
    let mut a: change_requests::ActiveModel = row.into();
    a.title = ActiveValue::set(b.title.trim().to_string());
    a.description = ActiveValue::set(b.description.trim().to_string());
    a.reason = ActiveValue::set(b.reason.trim().to_string());
    a.kind = ActiveValue::set(b.kind.clone());
    a.impact_schedule_days =
        ActiveValue::set(b.impact_schedule_days.and_then(|n| i32::try_from(n).ok()));
    a.impact_cost_minor = ActiveValue::set(b.impact_cost_minor);
    a.cost_currency = ActiveValue::set(b.cost_currency.clone());
    a.impact_summary = ActiveValue::set(
        b.impact_summary
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
    );
    a.edits = ActiveValue::set(edits_json(&b));
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "change_request_edited",
        caller.actor(),
        None,
    )
    .await;
    let links = links_of(&ctx, row.pid).await?;
    format::json(view(&row, Some(&links)))
}

#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    open: Option<bool>,
}

/// `GET /api/plans/{pid}/change-requests`.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    if q.status
        .as_deref()
        .is_some_and(|s| !rules::STATUSES.contains(&s))
    {
        return Err(unprocessable(&format!(
            "status must be one of {:?}",
            rules::STATUSES
        )));
    }
    let mut find = change_requests::Entity::find()
        .filter(change_requests::Column::PlanPid.eq(plan.pid))
        .filter(change_requests::Column::DeletedAt.is_null());
    if let Some(s) = &q.status {
        find = find.filter(change_requests::Column::Status.eq(s.as_str()));
    }
    if q.open == Some(true) {
        find = find.filter(change_requests::Column::Status.is_in([
            "draft",
            "submitted",
            "under_review",
            "approved",
        ]));
    }
    let rows = find
        .order_by_desc(change_requests::Column::Id)
        .limit(1000)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "change_requests": rows.iter().map(|c| view(c, None)).collect::<Vec<_>>(),
        "total": rows.len(),
    }))
}

/// `GET /api/plans/{pid}/change-requests/{cr}`.
#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    Path((pid, cr)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    let links = links_of(&ctx, row.pid).await?;
    format::json(view(&row, Some(&links)))
}

/// Move a change request by `action`, after the caller's permission has been checked by the caller.
async fn step(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    row: change_requests::Model,
    action: Action,
    edit: impl FnOnce(&mut change_requests::ActiveModel),
) -> Result<Response> {
    let Some(to) = rules::next(&row.status, action) else {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            &format!("a change request that is {} cannot do that", row.status),
        ));
    };
    let from = row.status.clone();
    let mut a: change_requests::ActiveModel = row.into();
    a.status = ActiveValue::set(to.to_string());
    edit(&mut a);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        &format!("change_request_{to}"),
        caller.actor(),
        Some(serde_json::json!({ "from": from, "to": to, "decision_reason": row.decision_reason })),
    )
    .await;
    let links = links_of(ctx, row.pid).await?;
    format::json(view(&row, Some(&links)))
}

#[derive(Debug, Deserialize)]
struct SubmitBody {
    approver_ref: String,
}

/// `POST …/{cr}/submit` — send a draft for decision, naming the approver.
#[debug_handler]
async fn submit(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
    Json(b): Json<SubmitBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    if checks_apply() && !rules::is_person(&row.requested_by, caller.actor()) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "only the person who drafted it can submit it",
        ));
    }
    if let Some(p) = rules::approver_problem(&b.approver_ref, &row.requested_by) {
        return Err(unprocessable(&p));
    }
    let approver = b.approver_ref.trim().to_string();
    step(&ctx, &caller, row, Action::Submit, |a| {
        a.approver_ref = ActiveValue::set(Some(approver));
    })
    .await
}

fn must_be_approver(row: &change_requests::Model, caller: &MaybeAuthUser) -> Result<()> {
    let named = row.approver_ref.as_deref().unwrap_or_default();
    if checks_apply() && !rules::is_person(named, caller.actor()) {
        // Not even an administrator decides for the named person: the approval is theirs.
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "only the named approver can do that",
        ));
    }
    Ok(())
}

/// `POST …/{cr}/review` — the approver starts reviewing.
#[debug_handler]
async fn review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    must_be_approver(&row, &caller)?;
    step(&ctx, &caller, row, Action::Review, |_| {}).await
}

#[derive(Debug, Deserialize)]
struct DecisionBody {
    /// `approve` or `reject`.
    decision: String,
    reason: String,
}

/// `POST …/{cr}/decision` — approve or reject, with a reason.
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
    Json(b): Json<DecisionBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    must_be_approver(&row, &caller)?;
    let action = match b.decision.as_str() {
        "approve" => Action::Approve,
        "reject" => Action::Reject,
        _ => return Err(unprocessable("decision must be approve or reject")),
    };
    if let Some(p) = rules::decision_problem(&b.reason) {
        return Err(unprocessable(&p));
    }
    let (who, on, reason) = (
        actor_of(&caller),
        Utc::now().date_naive(),
        b.reason.trim().to_string(),
    );
    step(&ctx, &caller, row, action, |a| {
        a.decided_by = ActiveValue::set(Some(who));
        a.decided_on = ActiveValue::set(Some(on));
        a.decision_reason = ActiveValue::set(Some(reason));
    })
    .await
}

/// `POST …/{cr}/implement` — the approved change has been carried out.
#[debug_handler]
async fn implement(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    let on = Utc::now().date_naive();
    step(&ctx, &caller, row, Action::Implement, |a| {
        a.implemented_on = ActiveValue::set(Some(on));
    })
    .await
}

/// `POST …/{cr}/withdraw` — the requester (or an administrator) withdraws it.
#[debug_handler]
async fn withdraw(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    if checks_apply()
        && !rules::is_person(&row.requested_by, caller.actor())
        && !super::plan_access::is_admin(caller.claims())
    {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "only the requester or an administrator can withdraw it",
        ));
    }
    step(&ctx, &caller, row, Action::Withdraw, |_| {}).await
}

#[derive(Debug, Deserialize)]
struct LinksBody {
    #[serde(default)]
    issue_pids: Vec<Uuid>,
    #[serde(default)]
    risk_pids: Vec<Uuid>,
}

/// `PUT …/{cr}/links` — replace the issues and risks this change request answers.
#[debug_handler]
async fn put_links(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
    Json(b): Json<LinksBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    if !rules::is_open(&row.status) {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "a closed change request cannot be relinked",
        ));
    }
    if b.issue_pids.len() > 200 || b.risk_pids.len() > 200 {
        return Err(unprocessable("at most 200 issues and 200 risks"));
    }
    let tx = ctx.db.begin().await.map_err(db_err)?;
    // Every issue and risk must belong to this plan and exist.
    for i in &b.issue_pids {
        let found = issues::Entity::find()
            .filter(issues::Column::Pid.eq(*i))
            .filter(issues::Column::PlanPid.eq(plan.pid))
            .filter(issues::Column::DeletedAt.is_null())
            .one(&tx)
            .await
            .map_err(db_err)?
            .ok_or_else(|| unprocessable(&format!("issue {i} is not an issue of this plan")))?;
        if found.change_request_pid.is_some_and(|c| c != row.pid) {
            return Err(refuse(
                StatusCode::CONFLICT,
                "conflict",
                &format!("issue {i} already belongs to another change request"),
            ));
        }
    }
    let known_risks = risks::Entity::find()
        .filter(risks::Column::PlanPid.eq(plan.pid))
        .filter(risks::Column::DeletedAt.is_null())
        .filter(risks::Column::Pid.is_in(b.risk_pids.clone()))
        .count(&tx)
        .await
        .map_err(db_err)?;
    let distinct_risks: std::collections::HashSet<_> = b.risk_pids.iter().collect();
    if known_risks != distinct_risks.len() as u64 {
        return Err(unprocessable("every risk must be a risk of this plan"));
    }
    // Issues: clear the ones no longer listed, set the ones listed.
    issues::Entity::update_many()
        .col_expr(
            issues::Column::ChangeRequestPid,
            sea_orm::sea_query::Expr::value(Option::<Uuid>::None),
        )
        .filter(issues::Column::ChangeRequestPid.eq(row.pid))
        .filter(issues::Column::Pid.is_not_in(b.issue_pids.clone()))
        .exec(&tx)
        .await
        .map_err(db_err)?;
    if !b.issue_pids.is_empty() {
        issues::Entity::update_many()
            .col_expr(
                issues::Column::ChangeRequestPid,
                sea_orm::sea_query::Expr::value(row.pid),
            )
            .filter(issues::Column::Pid.is_in(b.issue_pids.clone()))
            .exec(&tx)
            .await
            .map_err(db_err)?;
    }
    change_request_risks::Entity::delete_many()
        .filter(change_request_risks::Column::ChangeRequestPid.eq(row.pid))
        .exec(&tx)
        .await
        .map_err(db_err)?;
    for r in distinct_risks {
        change_request_risks::ActiveModel {
            change_request_pid: ActiveValue::set(row.pid),
            risk_pid: ActiveValue::set(*r),
            ..Default::default()
        }
        .insert(&tx)
        .await
        .map_err(db_err)?;
    }
    tx.commit().await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "change_request_relinked",
        caller.actor(),
        Some(serde_json::json!({ "issues": b.issue_pids.len(), "risks": b.risk_pids.len() })),
    )
    .await;
    let links = links_of(&ctx, row.pid).await?;
    format::json(view(&row, Some(&links)))
}

/// `GET …/{cr}/preview` — the forecast of the attached what-if edits (IR-5).
#[debug_handler]
async fn preview(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    let Some(raw) = row.edits.clone() else {
        return Err(unprocessable(
            "this change request carries no what-if edits, so there is no forecast; its stated impact is on the request",
        ));
    };
    let edits: Vec<Edit> = serde_json::from_value(raw)
        .map_err(|e| unprocessable(&format!("the stored edits cannot be read: {e}")))?;
    let forecast = sandbox_preview(&ctx, &caller, plan.pid, &edits).await?;
    format::json(serde_json::json!({
        "change_request_pid": row.pid,
        "label": "forecast",
        "note": "A forecast from the current schedule with the attached edits applied; nothing was changed. The impact stated on the request is the requester's own figure.",
        "stated_impact": { "schedule_business_days": row.impact_schedule_days, "cost_minor": row.impact_cost_minor, "cost_currency": row.cost_currency },
        "forecast": forecast,
    }))
}

async fn sandbox_preview(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    edits: &[Edit],
) -> Result<serde_json::Value> {
    super::sandbox::preview_json(ctx, caller, plan, edits).await
}

/// `POST …/{cr}/baseline` — take a schedule baseline because of this approved change (IR-5).
#[debug_handler]
async fn baseline(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, cr)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = find_cr(&ctx, plan.pid, &cr).await?;
    if !matches!(row.status.as_str(), "approved" | "implemented") {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "only an approved change request can be the reason for a baseline",
        ));
    }
    if row.baseline_pid.is_some() {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "a baseline has already been taken for this change request",
        ));
    }
    let reason: String = format!("Change request {}: {}", row.pid, row.title)
        .chars()
        .take(500)
        .collect();
    let taken = super::schedule_baselines::take(&ctx, &caller, plan.pid, reason).await?;
    let mut a: change_requests::ActiveModel = row.into();
    a.baseline_pid = ActiveValue::set(Some(taken.pid));
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "change_request_baselined",
        caller.actor(),
        Some(serde_json::json!({ "baseline_pid": taken.pid, "version": taken.version })),
    )
    .await;
    format::json(serde_json::json!({
        "change_request_pid": row.pid, "baseline_pid": taken.pid, "baseline_version": taken.version,
    }))
}

#[derive(Debug, Deserialize)]
struct PortfolioParams {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    open: Option<bool>,
}

/// `GET /api/change-requests` — across the plans the caller can read (IR-6 reads this).
#[debug_handler]
async fn portfolio(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<PortfolioParams>,
) -> Result<Response> {
    if q.status
        .as_deref()
        .is_some_and(|s| !rules::STATUSES.contains(&s))
    {
        return Err(unprocessable(&format!(
            "status must be one of {:?}",
            rules::STATUSES
        )));
    }
    let mut rows = change_requests::Entity::find()
        .filter(change_requests::Column::DeletedAt.is_null())
        .order_by_asc(change_requests::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut plans: Vec<Uuid> = rows.iter().map(|r| r.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    rows.retain(|r| readable.contains(&r.plan_pid));
    let today = Utc::now().date_naive();
    let mut by_status: std::collections::BTreeMap<&str, usize> =
        rules::STATUSES.iter().map(|s| (*s, 0)).collect();
    for r in &rows {
        if let Some(c) = by_status.get_mut(r.status.as_str()) {
            *c += 1;
        }
    }
    let awaiting = rows
        .iter()
        .filter(|r| matches!(r.status.as_str(), "submitted" | "under_review"))
        .count();
    let oldest_open = rows
        .iter()
        .filter(|r| rules::is_open(&r.status))
        .max_by_key(|r| rules::age_calendar_days(r.requested_on, today))
        .map(|r| view(r, None));
    rows.retain(|r| {
        q.status.as_ref().is_none_or(|s| &r.status == s)
            && (q.open != Some(true) || rules::is_open(&r.status))
    });
    rows.reverse();
    rows.truncate(500);
    format::json(serde_json::json!({
        "change_requests": rows.iter().map(|c| view(c, None)).collect::<Vec<_>>(),
        "summary": { "by_status": by_status, "awaiting_decision": awaiting, "oldest_open": oldest_open },
        "as_of": today,
        "note": "Counts cover every change request in plans you can read, whatever the filters. Newest first, at most 500.",
    }))
}

/// The change-request routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/change-requests", get(portfolio))
        .add("/plans/{pid}/change-requests", get(list))
        .add("/plans/{pid}/change-requests", post(create))
        .add("/plans/{pid}/change-requests/{cr}", get(get_one))
        .add("/plans/{pid}/change-requests/{cr}", put(update))
        .add("/plans/{pid}/change-requests/{cr}/submit", post(submit))
        .add("/plans/{pid}/change-requests/{cr}/review", post(review))
        .add("/plans/{pid}/change-requests/{cr}/decision", post(decide))
        .add(
            "/plans/{pid}/change-requests/{cr}/implement",
            post(implement),
        )
        .add("/plans/{pid}/change-requests/{cr}/withdraw", post(withdraw))
        .add("/plans/{pid}/change-requests/{cr}/links", put(put_links))
        .add("/plans/{pid}/change-requests/{cr}/preview", get(preview))
        .add("/plans/{pid}/change-requests/{cr}/baseline", post(baseline))
}

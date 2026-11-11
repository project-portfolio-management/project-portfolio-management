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
use crate::prelude::*;
use axum::http::StatusCode;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use super::unprocessable_entity as unprocessable;
use crate::auth::{MaybeAuthUser, require_auth};
use crate::change_requests::{self as rules, Action};
use crate::models::_entities::change_requests;
use crate::models::audit_logs::Model as AuditModel;
use crate::sandbox::{self, Edit};

/// The caller's actor string for audit and `requested_by`; `anonymous` when login is off.
fn actor_of(caller: &MaybeAuthUser) -> String {
    caller
        .actor()
        .map_or_else(|| "anonymous".to_string(), str::to_string)
}

/// Whether the identity checks apply (login is on). Off: every caller may do every step, as for any write.
fn checks_apply() -> bool {
    require_auth()
}

/// Loads a live (not soft-deleted) change request by pid, scoped to its plan, so a pid from another plan is `NotFound`.
async fn find_cr(ctx: &AppContext, plan: Uuid, pid: &str) -> Result<change_requests::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as(
        "SELECT * FROM change_requests WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .bind(plan)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)
}

/// Returns the pids of the live issues and the risks linked to a change request, as `(issues, risks)`.
async fn links_of(ctx: &AppContext, cr: Uuid) -> Result<(Vec<Uuid>, Vec<Uuid>)> {
    // Only the pids are read, in the database's order, as before (no `ORDER BY`).
    let issue_pids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT pid FROM issues WHERE change_request_pid = $1 AND deleted_at IS NULL",
    )
    .bind(cr)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let risk_pids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT risk_pid FROM change_request_risks WHERE change_request_pid = $1",
    )
    .bind(cr)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    Ok((issue_pids, risk_pids))
}

/// JSON view of a change request: the impact block (schedule in business days, cost in minor units with its currency), decision fields, computed age in calendar days, and linked issue/risk pids when `links` is supplied (null otherwise, meaning not loaded).
fn view(c: &change_requests::Model, links: Option<&(Vec<Uuid>, Vec<Uuid>)>) -> serde_json::Value {
    let today = crate::tz::deployment_today();
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `POST /api/plans/{pid}/change-requests`: a draft change request with optional impact figures.
pub(crate) struct Input {
    /// Short title of the change.
    title: String,
    /// What is being changed.
    description: String,
    /// Why the change is needed.
    reason: String,
    #[serde(default = "default_kind")]
    /// Change kind; defaults to `other` and is checked by the domain rules.
    kind: String,
    #[serde(default)]
    /// Estimated schedule impact in business days; null when not assessed, not zero.
    impact_schedule_days: Option<i64>,
    #[serde(default)]
    /// Estimated cost impact in minor currency units; null when not assessed, not zero.
    impact_cost_minor: Option<i64>,
    #[serde(default)]
    /// Currency of `impact_cost_minor`; expected whenever a cost is given.
    cost_currency: Option<String>,
    #[serde(default)]
    /// Free-text summary of the impact.
    impact_summary: Option<String>,
    /// What-if edits (the sandbox's edit list) whose forecast is shown as the impact.
    #[serde(default)]
    edits: Option<Vec<Edit>>,
}

/// Serde default for the change kind.
fn default_kind() -> String {
    "other".to_string()
}

/// Validates the draft against the change-request rules and reports every problem as one unprocessable response.
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

/// Serialises the optional what-if edits for storage; null when none were supplied.
fn edits_json(b: &Input) -> Option<serde_json::Value> {
    b.edits.as_ref().and_then(|e| serde_json::to_value(e).ok())
}

/// The trimmed impact summary; null when none or blank.
fn summary_of(b: &Input) -> Option<String> {
    b.impact_summary
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
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
    let mut tx = crate::db::begin(&ctx).await?;
    // The columns `SeaORM`'s insert set, no more.
    let row: change_requests::Model = sqlx::query_as(
        "INSERT INTO change_requests
             (pid, plan_pid, title, description, reason, kind, impact_schedule_days, impact_cost_minor, cost_currency,
              impact_summary, edits, status, requested_by, requested_on, approver_ref, decided_by, decided_on,
              decision_reason, implemented_on, baseline_pid, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'draft', $12, $13, NULL, NULL, NULL, NULL, NULL, NULL,
                 NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(b.title.trim())
    .bind(b.description.trim())
    .bind(b.reason.trim())
    .bind(&b.kind)
    .bind(b.impact_schedule_days.and_then(|n| i32::try_from(n).ok()))
    .bind(b.impact_cost_minor)
    .bind(&b.cost_currency)
    .bind(summary_of(&b))
    .bind(edits_json(&b))
    .bind(actor_of(&caller))
    .bind(crate::tz::deployment_today())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "change_request_drafted",
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan.pid, "title": row.title, "kind": row.kind })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, the nine columns `SeaORM`'s update named; no row back is an error, as its was.
    let row: change_requests::Model = sqlx::query_as(
        "UPDATE change_requests
         SET title = $1, description = $2, reason = $3, kind = $4, impact_schedule_days = $5, impact_cost_minor = $6,
             cost_currency = $7, impact_summary = $8, edits = $9
         WHERE id = $10
         RETURNING *",
    )
    .bind(b.title.trim())
    .bind(b.description.trim())
    .bind(b.reason.trim())
    .bind(&b.kind)
    .bind(b.impact_schedule_days.and_then(|n| i32::try_from(n).ok()))
    .bind(b.impact_cost_minor)
    .bind(&b.cost_currency)
    .bind(summary_of(&b))
    .bind(edits_json(&b))
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "change_request_edited",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let links = links_of(&ctx, row.pid).await?;
    format::json(view(&row, Some(&links)))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Query parameters for listing a plan's change requests.
pub(crate) struct ListParams {
    #[serde(default)]
    /// Restrict to one status; absent means all.
    status: Option<String>,
    #[serde(default)]
    /// When true, only requests still open (not decided or implemented).
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
    let mut find = sqlx::QueryBuilder::new(
        "SELECT * FROM change_requests WHERE deleted_at IS NULL AND plan_pid = ",
    );
    find.push_bind(plan.pid);
    if let Some(s) = &q.status {
        find.push(" AND status = ").push_bind(s.as_str());
    }
    if q.open == Some(true) {
        find.push(" AND status IN ('draft', 'submitted', 'under_review', 'approved')");
    }
    find.push(" ORDER BY id DESC LIMIT 1000");
    let rows: Vec<change_requests::Model> = find
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
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

/// What a step writes besides the status: each field `Some` is set, the rest are left alone.
#[derive(Default)]
struct StepFields {
    /// The approver named on submit.
    approver_ref: Option<String>,
    /// Who decided, on what day, and why.
    decided: Option<(String, NaiveDate, String)>,
    /// The day the change was implemented.
    implemented_on: Option<NaiveDate>,
}

/// Move a change request by `action`, after the caller's permission has been checked by the caller.
async fn step(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    row: change_requests::Model,
    action: Action,
    fields: StepFields,
) -> Result<Response> {
    let Some(to) = rules::next(&row.status, action) else {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            &format!("a change request that is {} cannot do that", row.status),
        ));
    };
    let from = row.status.clone();
    // By id, the status and the step's own columns, as `SeaORM`'s update named; no row back is an error, as its was.
    let mut update = sqlx::QueryBuilder::new("UPDATE change_requests SET status = ");
    update.push_bind(to);
    if let Some(approver) = fields.approver_ref {
        update.push(", approver_ref = ").push_bind(approver);
    }
    if let Some((who, on, reason)) = fields.decided {
        update.push(", decided_by = ").push_bind(who);
        update.push(", decided_on = ").push_bind(on);
        update.push(", decision_reason = ").push_bind(reason);
    }
    if let Some(on) = fields.implemented_on {
        update.push(", implemented_on = ").push_bind(on);
    }
    update
        .push(" WHERE id = ")
        .push_bind(row.id)
        .push(" RETURNING *");
    let mut tx = crate::db::begin(ctx).await?;
    let row: change_requests::Model = update.build_query_as().fetch_one(tx.conn()).await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        &format!("change_request_{to}"),
        caller.actor(),
        Some(serde_json::json!({ "from": from, "to": to, "decision_reason": row.decision_reason })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let links = links_of(ctx, row.pid).await?;
    format::json(view(&row, Some(&links)))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of the submit step.
pub(crate) struct SubmitBody {
    /// Person reference of the approver the draft is sent to; only that person may later decide.
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
    let fields = StepFields {
        approver_ref: Some(approver),
        ..StepFields::default()
    };
    step(&ctx, &caller, row, Action::Submit, fields).await
}

/// Refuses (403) unless the caller is the named approver, when identity checks apply. Administrators are deliberately not exempt: the approval belongs to the named person.
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
    step(&ctx, &caller, row, Action::Review, StepFields::default()).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of the decision step.
pub(crate) struct DecisionBody {
    /// `approve` or `reject`.
    decision: String,
    /// Why the decision was taken; recorded with the decision.
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
        crate::tz::deployment_today(),
        b.reason.trim().to_string(),
    );
    let fields = StepFields {
        decided: Some((who, on, reason)),
        ..StepFields::default()
    };
    step(&ctx, &caller, row, action, fields).await
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
    let fields = StepFields {
        implemented_on: Some(crate::tz::deployment_today()),
        ..StepFields::default()
    };
    step(&ctx, &caller, row, Action::Implement, fields).await
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
    step(&ctx, &caller, row, Action::Withdraw, StepFields::default()).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `PUT .../links`: the full replacement sets.
pub(crate) struct LinksBody {
    #[serde(default)]
    /// Issues the change request answers; the set replaces the current links.
    issue_pids: Vec<Uuid>,
    #[serde(default)]
    /// Risks the change request answers; the set replaces the current links.
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
    let mut tx = crate::db::begin(&ctx).await?;
    // Every issue and risk must belong to this plan and exist. Only the issue's link is read.
    for i in &b.issue_pids {
        let found: Option<Uuid> = sqlx::query_scalar::<_, Option<Uuid>>(
            "SELECT change_request_pid FROM issues WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(*i)
        .bind(plan.pid)
        .fetch_optional(tx.conn())
        .await?
        .ok_or_else(|| unprocessable(&format!("issue {i} is not an issue of this plan")))?;
        if found.is_some_and(|c| c != row.pid) {
            return Err(refuse(
                StatusCode::CONFLICT,
                "conflict",
                &format!("issue {i} already belongs to another change request"),
            ));
        }
    }
    let known_risks: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM risks WHERE plan_pid = $1 AND deleted_at IS NULL AND pid = ANY($2)",
    )
    .bind(plan.pid)
    .bind(&b.risk_pids)
    .fetch_one(tx.conn())
    .await?;
    let distinct_risks: std::collections::HashSet<_> = b.risk_pids.iter().collect();
    if u64::try_from(known_risks).ok() != Some(distinct_risks.len() as u64) {
        return Err(unprocessable("every risk must be a risk of this plan"));
    }
    // Issues: clear the ones no longer listed, set the ones listed.
    // `<> ALL` of an empty list is true, as `SeaORM`'s `NOT IN ()` was (it writes that as always true).
    sqlx::query(
        "UPDATE issues SET change_request_pid = NULL WHERE change_request_pid = $1 AND pid <> ALL($2)",
    )
    .bind(row.pid)
    .bind(&b.issue_pids)
    .execute(tx.conn())
    .await?;
    if !b.issue_pids.is_empty() {
        sqlx::query("UPDATE issues SET change_request_pid = $1 WHERE pid = ANY($2)")
            .bind(row.pid)
            .bind(&b.issue_pids)
            .execute(tx.conn())
            .await?;
    }
    sqlx::query("DELETE FROM change_request_risks WHERE change_request_pid = $1")
        .bind(row.pid)
        .execute(tx.conn())
        .await?;
    for r in distinct_risks {
        sqlx::query(
            "INSERT INTO change_request_risks (change_request_pid, risk_pid) VALUES ($1, $2)",
        )
        .bind(row.pid)
        .bind(*r)
        .execute(tx.conn())
        .await?;
    }
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "change_request_relinked",
        caller.actor(),
        Some(serde_json::json!({ "issues": b.issue_pids.len(), "risks": b.risk_pids.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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

/// Forecasts the plan with the given what-if edits via the sandbox, as the impact shown on a change request.
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
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, the one column `SeaORM`'s update named; no row back is an error, as its was. Only the pid is used.
    let row_pid: Uuid = sqlx::query_scalar(
        "UPDATE change_requests SET baseline_pid = $1 WHERE id = $2 RETURNING pid",
    )
    .bind(taken.pid)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "change_request_baselined",
        caller.actor(),
        Some(serde_json::json!({ "baseline_pid": taken.pid, "version": taken.version })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "change_request_pid": row_pid, "baseline_pid": taken.pid, "baseline_version": taken.version,
    }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Query parameters for the cross-plan change-request list.
pub(crate) struct PortfolioParams {
    #[serde(default)]
    /// Restrict to one status; absent means all.
    status: Option<String>,
    #[serde(default)]
    /// When true, only requests still open.
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
    let mut rows: Vec<change_requests::Model> =
        sqlx::query_as("SELECT * FROM change_requests WHERE deleted_at IS NULL ORDER BY id")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut plans: Vec<Uuid> = rows.iter().map(|r| r.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    rows.retain(|r| readable.contains(&r.plan_pid));
    let today = crate::tz::deployment_today();
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

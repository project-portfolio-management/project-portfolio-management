//! Workflow automation and the set-and-forget scheduler.
//!
//! - **Automations** — rules an operator configures once (`POST
//!   /api/automations`). When a task crosses the Kanban board (or a
//!   review verdict lands) `fire` evaluates them with the pure engine
//!   in [`crate::automation`] and applies the matching actions.
//! - **Scheduled actions** — the deadline queue. An action configured
//!   once is held until `due_at`, then fired **exactly once** by the
//!   sweep (`POST /api/scheduled-actions/sweep`, or the env-gated
//!   in-process ticker).
//! - **Runs** — every firing writes an `automation_runs` row, applied
//!   or not, so an automatic change is always traceable to the rule
//!   that made it.
//!
//! Three deliberate limits:
//!
//! - **An automation never breaks the operator's action.** If a rule
//!   fails, the failure is recorded as a `failed` run and the move that
//!   triggered it still stands. Automation is an assistant, not a gate.
//! - **No cascades.** Actions are applied without re-entering the
//!   engine, so one board move fires one generation of rules.
//! - **Exactly once.** The sweep claims each due row with a conditional
//!   update and only acts when it won the claim, so two concurrent
//!   sweeps cannot double-fire a deadline.

use super::unprocessable;
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::plan_access::push_none_of;
use crate::auth::MaybeAuthUser;
use crate::automation as rules;
use crate::engineering::TASK_STATUSES;
use crate::models::_entities::{
    automation_runs, automations, milestones, plan_dependencies, plans, scheduled_actions, tasks,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;
use crate::validation::MAX_TEXT_LEN;

/// Default page size for every list endpoint here — the cap each one
/// applied before paging (agents/restful.md) landed; a `limit` above
/// [`super::pagination::MAX_LIMIT`] is a `422`.
pub const LIST_CAP: u64 = 200;

/// Highest number of due actions one sweep will fire. A bounded sweep
/// keeps a backlog from turning into an unbounded write burst; the
/// response reports whether the cap was hit.
pub const SWEEP_CAP: u64 = 500;

/// `POST /api/automations` body.
///
/// `actions` is an **ordered** list (FR-32) — array order is the
/// declared firing order, applied in full every time the rule matches.
/// Each element is `{"kind": …, "value": …}`, the same shape
/// `action_kind`/`action_value` used singly before this rule type
/// existed.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AutomationPayload {
    /// Scope the rule to one plan's board; absent = every plan.
    #[serde(default)]
    plan_pid: Option<Uuid>,
    /// Rule name; required, trimmed, capped at `MAX_TEXT_LEN` characters.
    name: String,
    /// Event that fires the rule; validated against the known trigger kinds at write time.
    trigger_kind: String,
    /// For a status-change trigger, the status the task leaves; absent matches any.
    #[serde(default)]
    from_status: Option<String>,
    /// For a status-change trigger, the status the task enters; absent matches any.
    #[serde(default)]
    to_status: Option<String>,
    /// Ordered actions, each `{"kind": ..., "value": ...}`; all are applied in array order whenever the rule matches.
    actions: Vec<serde_json::Value>,
}

/// A rule on one plan needs that plan to be readable (`404`). A rule on **every** plan needs an administrator (`403`): it
/// also runs on private plans its author cannot see. With login off there is no caller to refuse.
async fn require_rule_scope(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Option<Uuid>,
) -> Result<()> {
    match plan {
        Some(_) => super::plan_access::require_readable_if_any(ctx, caller, plan).await,
        None if caller.holds_or_open(|c| super::plan_access::is_admin(Some(c))) => Ok(()),
        None => Err(super::forbidden(
            "only an administrator can manage a rule on every plan",
        )),
    }
}

/// `POST /api/automations` — configure one rule. The trigger and every
/// declared action are validated here, at write time, so a malformed
/// rule can never fail silently later when nobody is watching.
#[debug_handler]
async fn create_automation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<AutomationPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if payload.name.trim().is_empty() || payload.name.len() > MAX_TEXT_LEN {
        problems.push(format!(
            "name is required and capped at {MAX_TEXT_LEN} characters"
        ));
    }
    if let Err(e) = rules::validate_trigger(
        &payload.trigger_kind,
        payload.from_status.as_deref(),
        payload.to_status.as_deref(),
        TASK_STATUSES,
    ) {
        problems.push(e);
    }
    if let Err(e) = rules::validate_actions(&payload.actions, TASK_STATUSES) {
        problems.push(e);
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    // A new rule needs a live plan; `require_rule_scope` lets an old rule be switched off after its plan has gone.
    if let Some(plan) = payload.plan_pid {
        super::plan_access::require_readable(&ctx, &caller, plan).await?;
    }
    require_rule_scope(&ctx, &caller, payload.plan_pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: automations::Model = sqlx::query_as(
        "INSERT INTO automations
             (pid, plan_pid, name, trigger_kind, from_status, to_status, actions, enabled, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, true, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(payload.plan_pid)
    .bind(payload.name.trim())
    .bind(payload.trigger_kind.as_str())
    .bind(payload.from_status.as_deref())
    .bind(payload.to_status.as_deref())
    .bind(serde_json::Value::Array(payload.actions.clone()))
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "automation_created",
        caller.actor(),
        Some(serde_json::json!({
            "trigger_kind": row.trigger_kind,
            "action_kinds": row.actions.as_array().map(|a| {
                a.iter().filter_map(|v| v.get("kind").and_then(serde_json::Value::as_str)).collect::<Vec<_>>()
            }),
            "plan_pid": row.plan_pid.map(|p| p.to_string()),
        })),
    ).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/automations` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AutomationParams {
    /// Only rules scoped to this plan; absent applies no plan filter.
    #[serde(default)]
    plan: Option<Uuid>,
    /// Only rules with this trigger kind; absent applies no trigger filter.
    #[serde(default)]
    trigger: Option<String>,
    /// `true` ⇒ only enabled rules.
    #[serde(default)]
    enabled: Option<bool>,
}

/// `GET /api/automations?plan=&trigger=&enabled=&limit=&cursor=` — the
/// configured rules, oldest first. Paged by cursor (agents/restful.md):
/// `Link: rel="next"` while more rows follow, and no total.
///
/// # Errors
///
/// `422` for a bad `limit` or `cursor`, and `400` for a cursor past [`super::pagination::MAX_OFFSET`]; a DB
/// error when the query fails.
#[debug_handler]
async fn list_automations(
    State(ctx): State<AppContext>,
    cursor: super::pagination::Cursor,
    caller: MaybeAuthUser,
    Query(params): Query<AutomationParams>,
) -> Result<Response> {
    let page = cursor.page(LIST_CAP)?;
    let (limit, offset) = (page.fetch(), page.offset());
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM automations WHERE deleted_at IS NULL",
    );
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .plan_pids();
    if !hidden.is_empty() {
        query
            .push(" AND (plan_pid IS NULL OR plan_pid <> ALL(")
            .push_bind(hidden)
            .push("))");
    }
    if let Some(plan) = params.plan {
        query.push(" AND plan_pid = ").push_bind(plan);
    }
    if let Some(trigger) = params.trigger {
        query.push(" AND trigger_kind = ").push_bind(trigger);
    }
    if let Some(enabled) = params.enabled {
        query.push(" AND enabled = ").push_bind(enabled);
    }
    push_page(&mut query, "id", limit, offset);
    let rows: Vec<automations::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    Ok(page.respond(rows))
}

/// Flip one rule's `enabled` flag.
async fn set_enabled(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
    enabled: bool,
) -> Result<Response> {
    let row = cap_models::find_automation(ctx, crate::models::governance::parse_pid(pid)?).await?;
    require_rule_scope(ctx, caller, row.plan_pid).await?;
    let row_pid = row.pid;
    let mut tx = crate::db::begin(ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as `SeaORM`'s was.
    let row: automations::Model =
        sqlx::query_as("UPDATE automations SET enabled = $1 WHERE id = $2 RETURNING *")
            .bind(enabled)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
    let action = if enabled {
        "automation_enabled"
    } else {
        "automation_disabled"
    };
    AuditModel::record_on(tx.conn(), row_pid, action, caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/automations/{pid}/enable`.
#[debug_handler]
async fn enable_automation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    set_enabled(&ctx, &caller, &pid, true).await
}

/// `POST /api/automations/{pid}/disable` — the off switch. Disabling
/// keeps the rule and its run history; deleting is a separate call.
#[debug_handler]
async fn disable_automation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    set_enabled(&ctx, &caller, &pid, false).await
}

/// `DELETE /api/automations/{pid}` — soft delete.
#[debug_handler]
async fn delete_automation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row =
        cap_models::find_automation(&ctx, crate::models::governance::parse_pid(&pid)?).await?;
    require_rule_scope(&ctx, &caller, row.plan_pid).await?;
    let row_pid = row.pid;
    let deleted_at: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as `SeaORM`'s was.
    sqlx::query_scalar::<_, i32>(
        "UPDATE automations SET enabled = false, deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(deleted_at)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "automation_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// Order a list by `order` (a literal at every call, never input) and take one page of it.
fn push_page(
    query: &mut sqlx::QueryBuilder<sqlx::Postgres>,
    order: &'static str,
    limit: u64,
    offset: u64,
) {
    query
        .push(" ORDER BY ")
        .push(order)
        .push(" LIMIT ")
        .push_bind(i64::try_from(limit).unwrap_or(i64::MAX))
        .push(" OFFSET ")
        .push_bind(i64::try_from(offset).unwrap_or(i64::MAX));
}

/// Everything that belongs to a plan hidden from `caller`.
async fn hidden_pids(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<Vec<Uuid>> {
    let scope = super::plan_access::HiddenScope::of(ctx, caller).await?;
    Ok(scope.owned_pids(ctx).await?.into_iter().collect())
}

/// `GET /api/automations/runs` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RunParams {
    /// Only runs of this rule (its pid).
    #[serde(default)]
    automation: Option<Uuid>,
    /// Only runs against this subject (its pid).
    #[serde(default)]
    subject: Option<Uuid>,
    /// Only runs with this outcome, such as `applied`, `skipped` or `failed`.
    #[serde(default)]
    outcome: Option<String>,
}

/// `GET /api/automations/runs?automation=&subject=&outcome=&limit=&cursor=`
/// — what the automations actually did, newest first. Paged by cursor (agents/restful.md):
/// `Link: rel="next"` while more rows follow, and no total.
///
/// # Errors
///
/// `422` for a bad `limit` or `cursor`, and `400` for a cursor past [`super::pagination::MAX_OFFSET`]; a DB
/// error when the query fails.
#[debug_handler]
async fn list_runs(
    State(ctx): State<AppContext>,
    cursor: super::pagination::Cursor,
    caller: MaybeAuthUser,
    Query(params): Query<RunParams>,
) -> Result<Response> {
    let page = cursor.page(LIST_CAP)?;
    let (limit, offset) = (page.fetch(), page.offset());
    let mut query =
        sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM automation_runs WHERE true");
    // A rule on every plan runs on hidden plans too, so the subject counts as well as the rule.
    let owned = hidden_pids(&ctx, &caller).await?;
    if !owned.is_empty() {
        query.push(" AND ");
        push_none_of(&mut query, "automation_pid", owned.iter().copied());
        query.push(" AND ");
        push_none_of(&mut query, "subject_pid", owned);
    }
    if let Some(automation) = params.automation {
        query.push(" AND automation_pid = ").push_bind(automation);
    }
    if let Some(subject) = params.subject {
        query.push(" AND subject_pid = ").push_bind(subject);
    }
    if let Some(outcome) = params.outcome {
        query.push(" AND outcome = ").push_bind(outcome);
    }
    push_page(&mut query, "id DESC", limit, offset);
    let rows: Vec<automation_runs::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    Ok(page.respond(rows))
}

/// Read a non-blank string field from an action's stored value.
fn action_str(value: &serde_json::Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(std::string::ToString::to_string)
}

/// `assign` — put the named person on the task.
async fn act_assign(
    ctx: &AppContext,
    rule_name: &str,
    action_value: &serde_json::Value,
    subject_kind: &str,
    subject_pid: Uuid,
) -> (&'static str, serde_json::Value) {
    let Some(assignee) = action_str(action_value, "assignee_ref") else {
        return ("failed", serde_json::json!({ "reason": "no assignee_ref" }));
    };
    if subject_kind != "task" {
        return (
            "skipped",
            serde_json::json!({ "reason": "assign only applies to a task" }),
        );
    }
    let Ok(task) = find_live_task(ctx, subject_pid).await else {
        return (
            "skipped",
            serde_json::json!({ "reason": "task no longer live" }),
        );
    };
    let previous = task.assignee_ref.clone();
    // `fetch_one`: an update that finds no row is an error, as `SeaORM`'s was.
    let updated = sqlx::query_scalar::<_, i32>(
        "UPDATE tasks SET assignee_ref = $1 WHERE id = $2 RETURNING id",
    )
    .bind(&assignee)
    .bind(task.id)
    .fetch_one(crate::db::pool(ctx))
    .await;
    match updated {
        Ok(_) => {
            cap_models::notify_on(
                &mut crate::db::pool(ctx),
                &assignee,
                "task",
                subject_pid,
                "task_assigned",
                &format!("Assigned to you by automation `{rule_name}`"),
            )
            .await
            .ok();
            (
                "applied",
                serde_json::json!({ "from": previous, "to": assignee }),
            )
        }
        Err(e) => ("failed", serde_json::json!({ "reason": e.to_string() })),
    }
}

/// `set_task_status` — move the task itself.
///
/// Applied **without** re-entering the engine: automations do not
/// cascade.
async fn act_set_task_status(
    ctx: &AppContext,
    action_value: &serde_json::Value,
    subject_kind: &str,
    subject_pid: Uuid,
) -> (&'static str, serde_json::Value) {
    let Some(status) = action_str(action_value, "status") else {
        return ("failed", serde_json::json!({ "reason": "no status" }));
    };
    if subject_kind != "task" {
        return (
            "skipped",
            serde_json::json!({ "reason": "set_task_status only applies to a task" }),
        );
    }
    let Ok(task) = find_live_task(ctx, subject_pid).await else {
        return (
            "skipped",
            serde_json::json!({ "reason": "task no longer live" }),
        );
    };
    if task.status == status {
        return (
            "skipped",
            serde_json::json!({ "reason": "task already in that status" }),
        );
    }
    let from = task.status.clone();
    let first_done = task.done_at.is_none() && status == "done";
    let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    // `done_at` only on the first move to done, as before; `fetch_one`: an update that finds no row is an error.
    let sql = if first_done {
        "UPDATE tasks SET status = $1, status_changed_at = $2, done_at = $2 WHERE id = $3 RETURNING id"
    } else {
        "UPDATE tasks SET status = $1, status_changed_at = $2 WHERE id = $3 RETURNING id"
    };
    let updated = sqlx::query_scalar::<_, i32>(sql)
        .bind(&status)
        .bind(now)
        .bind(task.id)
        .fetch_one(crate::db::pool(ctx))
        .await;
    match updated {
        Ok(_) => ("applied", serde_json::json!({ "from": from, "to": status })),
        Err(e) => ("failed", serde_json::json!({ "reason": e.to_string() })),
    }
}

/// `notify` — write one in-app notification.
async fn act_notify(
    ctx: &AppContext,
    rule_name: &str,
    action_value: &serde_json::Value,
    subject_kind: &str,
    subject_pid: Uuid,
) -> (&'static str, serde_json::Value) {
    let Some(recipient) = action_str(action_value, "recipient_ref") else {
        return (
            "failed",
            serde_json::json!({ "reason": "no recipient_ref" }),
        );
    };
    let message = action_str(action_value, "message")
        .unwrap_or_else(|| format!("Automation `{rule_name}` fired"));
    match cap_models::notify_on(
        &mut crate::db::pool(ctx),
        &recipient,
        subject_kind,
        subject_pid,
        "automation",
        &message,
    )
    .await
    {
        Ok(Some(_)) => ("applied", serde_json::json!({ "recipient_ref": recipient })),
        // The recipient switched this kind off (PL-5): not a failure.
        Ok(None) => (
            "skipped",
            serde_json::json!({ "recipient_ref": recipient, "reason": "recipient_opted_out" }),
        ),
        Err(e) => ("failed", serde_json::json!({ "reason": e.to_string() })),
    }
}

/// `schedule_action` — hand a deadline to the set-and-forget queue.
async fn act_schedule(
    ctx: &AppContext,
    rule_pid: Uuid,
    action_value: &serde_json::Value,
    subject_kind: &str,
    subject_pid: Uuid,
    actor: Option<&str>,
) -> (&'static str, serde_json::Value) {
    let Some(action_kind) = action_str(action_value, "action_kind") else {
        return ("failed", serde_json::json!({ "reason": "no action_kind" }));
    };
    let Some(days) = action_value
        .get("in_days")
        .and_then(serde_json::Value::as_i64)
    else {
        return ("failed", serde_json::json!({ "reason": "no in_days" }));
    };
    let due_at = chrono::Utc::now() + chrono::Duration::days(days);
    match cap_models::schedule_on(
        &mut crate::db::pool(ctx),
        cap_models::NewScheduledAction {
            subject_kind: subject_kind.to_string(),
            subject_pid,
            action_kind,
            payload: action_value.clone(),
            due_at: due_at.into(),
            source_automation_pid: Some(rule_pid),
            created_by: actor.map(std::string::ToString::to_string),
        },
    )
    .await
    {
        Ok(row) => (
            "applied",
            serde_json::json!({
                "scheduled_action_pid": row.pid.to_string(),
                "due_at": row.due_at.to_rfc3339(),
            }),
        ),
        Err(e) => ("failed", serde_json::json!({ "reason": e.to_string() })),
    }
}

/// The shared context every action of one matched rule's firing needs
/// — bundled so `apply_action` takes a handful of arguments rather
/// than one per fact about the firing.
struct FiringContext<'a> {
    /// The matched rule being applied.
    rule: &'a automations::Model,
    /// The event that matched the rule, including the plan it happened in.
    fact: &'a rules::TriggerFact,
    /// Kind of the thing the rule acts on (for example `task`).
    subject_kind: &'a str,
    /// Pid of the thing the rule acts on.
    subject_pid: Uuid,
    /// Who caused the event, when known; carried into follow-up actions for the audit trail.
    actor: Option<&'a str>,
}

/// Apply one action of a matched rule and return the run outcome and its detail (`applied`, `skipped` or `failed`, with a reason on failure). Dispatches on the action kind; an unknown kind fails rather than being ignored.
/// Called once per element of the rule's `actions` array, each independently
/// logged (FR-32).
async fn apply_action(
    ctx: &AppContext,
    firing: &FiringContext<'_>,
    action: &rules::ParsedAction,
    action_index: i32,
) -> (&'static str, serde_json::Value) {
    let (rule, fact, subject_kind, subject_pid, actor) = (
        firing.rule,
        firing.fact,
        firing.subject_kind,
        firing.subject_pid,
        firing.actor,
    );
    match action.kind.as_str() {
        "assign" => act_assign(ctx, &rule.name, &action.value, subject_kind, subject_pid).await,
        "set_task_status" => {
            act_set_task_status(ctx, &action.value, subject_kind, subject_pid).await
        }
        "notify" => act_notify(ctx, &rule.name, &action.value, subject_kind, subject_pid).await,
        "schedule_action" => {
            act_schedule(
                ctx,
                rule.pid,
                &action.value,
                subject_kind,
                subject_pid,
                actor,
            )
            .await
        }
        "add_label" => {
            let Some(label) = action_str(&action.value, "label") else {
                return ("failed", serde_json::json!({ "reason": "no label" }));
            };
            match add_plan_label(ctx, fact.plan_pid, &label).await {
                Ok(true) => ("applied", serde_json::json!({ "label": label })),
                Ok(false) => (
                    "skipped",
                    serde_json::json!({ "reason": "plan already carries that label" }),
                ),
                Err(e) => ("failed", serde_json::json!({ "reason": e.to_string() })),
            }
        }
        "propose_reschedule" => act_propose_reschedule(ctx, &action.value, fact).await,
        "shift_dependents" => act_shift_dependents(ctx, rule.pid, action_index, fact, actor).await,
        other => (
            "failed",
            serde_json::json!({ "reason": format!("unknown action_kind `{other}`") }),
        ),
    }
}

/// One live task by pid.
async fn find_live_task(ctx: &AppContext, pid: Uuid) -> Result<tasks::Model> {
    sqlx::query_as("SELECT * FROM tasks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Append a label to a plan's payload tags, case-insensitively
/// de-duplicated (the tag convention in the entity spec §5.4).
/// `Ok(false)` when the plan already carries it.
async fn add_plan_label(ctx: &AppContext, plan_pid: Uuid, label: &str) -> Result<bool> {
    let model = find_live_plan(ctx, plan_pid)
        .await?
        .ok_or(Error::NotFound)?;
    let mut plan = model.to_plan().map_err(Error::Model)?;
    if plan
        .tags
        .iter()
        .any(|t| t.trim().eq_ignore_ascii_case(label))
    {
        return Ok(false);
    }
    plan.tags.push(label.to_string());
    model
        .update_data_on(&mut crate::db::pool(ctx), &plan)
        .await?;
    Ok(true)
}

/// One live plan by pid, `None` when there is none.
async fn find_live_plan(ctx: &AppContext, pid: Uuid) -> Result<Option<plans::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// Every direct finish-start successor of `plan_pid`, with its own
/// current dates — one query for the edges, then one per successor for
/// its dates (bounded by how many dependencies a plan realistically
/// declares; no pagination cap exists on `plan_dependencies` writes
/// today, so this is not yet N+1-hardened the way `tba::load_board` is).
async fn load_successors(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<rules::SuccessorFact>> {
    let edges: Vec<plan_dependencies::Model> =
        sqlx::query_as("SELECT * FROM plan_dependencies WHERE predecessor_pid = $1")
            .bind(plan_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let mut out = Vec::with_capacity(edges.len());
    for edge in edges {
        let Some(successor) = find_live_plan(ctx, edge.successor_pid).await? else {
            // The successor was deleted since the edge was recorded;
            // skip it rather than propose a shift for a plan that no
            // longer exists.
            continue;
        };
        let parsed = successor.to_plan().map_err(Error::Model)?;
        out.push(rules::SuccessorFact {
            edge_pid: edge.pid,
            successor_pid: edge.successor_pid,
            lag_days: edge.lag_days,
            current_start: parsed.start_date.and_then(|d| d.parse().ok()),
            current_end: parsed.target_date.and_then(|d| d.parse().ok()),
        });
    }
    Ok(out)
}

/// Parse the trigger fact's own old/new dates. `plan_timeframe_changed`
/// carries them in `from_status`/`to_status` (see
/// `crate::automation::TRIGGER_KINDS`'s own doc comment for why).
fn shift_dates(fact: &rules::TriggerFact) -> Option<(chrono::NaiveDate, chrono::NaiveDate)> {
    let old = fact.from_status.as_deref()?.parse().ok()?;
    let new = fact.to_status.as_deref()?.parse().ok()?;
    Some((old, new))
}

/// `propose_reschedule` — compute each direct successor's implied
/// shift and **notify**, moving nothing.
async fn act_propose_reschedule(
    ctx: &AppContext,
    action_value: &serde_json::Value,
    fact: &rules::TriggerFact,
) -> (&'static str, serde_json::Value) {
    if fact.kind != "plan_timeframe_changed" {
        return (
            "skipped",
            serde_json::json!({ "reason": "propose_reschedule only applies to plan_timeframe_changed" }),
        );
    }
    let Some(recipient) = action_str(action_value, "recipient_ref") else {
        return (
            "failed",
            serde_json::json!({ "reason": "no recipient_ref" }),
        );
    };
    let Some((old_end, new_end)) = shift_dates(fact) else {
        return (
            "skipped",
            serde_json::json!({ "reason": "no parseable target_date change" }),
        );
    };
    let successors = match load_successors(ctx, fact.plan_pid).await {
        Ok(s) => s,
        Err(e) => return ("failed", serde_json::json!({ "reason": e.to_string() })),
    };
    if successors.is_empty() {
        return (
            "skipped",
            serde_json::json!({ "reason": "no dependent plans" }),
        );
    }
    let shifts = rules::propose_shifts(old_end, new_end, &successors);
    let message = format!(
        "Automation proposes rescheduling {} dependent plan(s) following a {}-calendar-day shift to this plan's end date",
        shifts.len(),
        (new_end - old_end).num_days(),
    );
    match cap_models::notify_on(
        &mut crate::db::pool(ctx),
        &recipient,
        "plan",
        fact.plan_pid,
        "reschedule_proposed",
        &message,
    )
    .await
    {
        Ok(_) => (
            "applied",
            serde_json::json!({ "recipient_ref": recipient, "shifts": shifts }),
        ),
        Err(e) => ("failed", serde_json::json!({ "reason": e.to_string() })),
    }
}

/// `shift_dependents` — apply the same computation `propose_reschedule`
/// would, to each direct successor's own `start_date`/`target_date`.
///
/// **No cascade**: each successor is written directly (not through
/// `controllers::plans::update`), so applying a shift here never
/// re-fires `plan_timeframe_changed`. One `automation_runs` row is
/// logged per successor actually moved, in addition to the one the
/// caller ([`apply_rule_actions`]) already logs for this action as a
/// whole — the per-successor rows answer "what happened to this
/// specific plan", the per-action row answers "did this rule's action
/// fire at all".
async fn act_shift_dependents(
    ctx: &AppContext,
    rule_pid: Uuid,
    action_index: i32,
    fact: &rules::TriggerFact,
    actor: Option<&str>,
) -> (&'static str, serde_json::Value) {
    if fact.kind != "plan_timeframe_changed" {
        return (
            "skipped",
            serde_json::json!({ "reason": "shift_dependents only applies to plan_timeframe_changed" }),
        );
    }
    let Some((old_end, new_end)) = shift_dates(fact) else {
        return (
            "skipped",
            serde_json::json!({ "reason": "no parseable target_date change" }),
        );
    };
    let successors = match load_successors(ctx, fact.plan_pid).await {
        Ok(s) => s,
        Err(e) => return ("failed", serde_json::json!({ "reason": e.to_string() })),
    };
    if successors.is_empty() {
        return (
            "skipped",
            serde_json::json!({ "reason": "no dependent plans" }),
        );
    }
    let shifts = rules::propose_shifts(old_end, new_end, &successors);
    let mut moved = Vec::with_capacity(shifts.len());
    for shift in &shifts {
        let Ok(Some(model)) = find_live_plan(ctx, shift.successor_pid).await else {
            continue;
        };
        let Ok(mut plan) = model.to_plan() else {
            continue;
        };
        plan.start_date = shift.proposed_start.map(|d| d.to_string());
        plan.target_date = shift.proposed_end.map(|d| d.to_string());
        // The shift, its run record and its audit row are one change: a shift that cannot be audited is not
        // made (the rule never blocks the action that triggered it, so it is skipped, and logged).
        let Ok(mut tx) = crate::db::begin(ctx).await else {
            continue;
        };
        if model.update_data_on(&mut tx, &plan).await.is_err() {
            continue;
        }
        cap_models::record_run_on(
            &mut tx,
            rule_pid,
            "plan",
            shift.successor_pid,
            action_index,
            "applied",
            serde_json::json!({
                "edge_pid": shift.edge_pid.to_string(),
                "delta_days": shift.delta_days,
                "proposed_start": shift.proposed_start,
                "proposed_end": shift.proposed_end,
                "already_violated": shift.already_violated,
            }),
        )
        .await
        .ok();
        if let Err(error) = AuditModel::record_on(
            tx.conn(),
            shift.successor_pid,
            "automation_shifted_dependent",
            actor,
            Some(serde_json::json!({ "edge_pid": shift.edge_pid.to_string() })),
        )
        .await
        {
            let _ = super::audit_failed(error);
            continue;
        }
        if tx.commit().await.is_err() {
            continue;
        }
        moved.push(shift.successor_pid.to_string());
    }
    (
        "applied",
        serde_json::json!({ "moved": moved, "shifts": shifts }),
    )
}

/// Evaluate every enabled rule against one thing that just happened
/// and apply what matches.
///
/// Deliberately infallible from the caller's point of view: a rule
/// that fails is recorded as a `failed` run, never propagated, so an
/// automation can never undo or block the operator action that
/// triggered it.
pub(crate) async fn fire(
    ctx: &AppContext,
    fact: &rules::TriggerFact,
    subject_kind: &str,
    subject_pid: Uuid,
    actor: Option<&str>,
) {
    let candidates = match cap_models::enabled_automations(ctx, &fact.kind).await {
        Ok(rows) => rows,
        Err(err) => {
            tracing::warn!("automation lookup failed: {err}");
            return;
        }
    };
    for rule in candidates {
        let rule_fact = rules::RuleFact {
            enabled: rule.enabled,
            plan_pid: rule.plan_pid,
            trigger_kind: rule.trigger_kind.clone(),
            from_status: rule.from_status.clone(),
            to_status: rule.to_status.clone(),
        };
        if !rules::rule_matches(&rule_fact, fact) {
            continue;
        }
        apply_rule_actions(ctx, &rule, fact, subject_kind, subject_pid, actor).await;
    }
}

/// Parse, apply and log every action of one **already-matched** rule.
/// Shared by [`fire`] (one-shot triggers) and
/// [`sweep_milestone_due`] (the `milestone_due` sweep) — the two differ
/// only in *how* a rule is found to fire (a single write vs. a claimed
/// sweep row), not in what firing does once found.
async fn apply_rule_actions(
    ctx: &AppContext,
    rule: &automations::Model,
    fact: &rules::TriggerFact,
    subject_kind: &str,
    subject_pid: Uuid,
    actor: Option<&str>,
) {
    // Already validated at write time (`create_automation`); this can
    // only fail on data that predates that validation or was written
    // some other way, so it is logged and the rule is skipped entirely
    // — a partially-parsed action list has no safe partial-apply
    // reading — rather than firing on whatever parsed.
    let actions = match rule
        .actions
        .as_array()
        .map(std::vec::Vec::as_slice)
        .map(|actions| rules::validate_actions(actions, TASK_STATUSES))
    {
        Some(Ok(actions)) => actions,
        Some(Err(err)) => {
            tracing::warn!(
                "automation {} has an invalid actions array, not firing: {err}",
                rule.pid
            );
            return;
        }
        None => {
            tracing::warn!(
                "automation {} actions is not a JSON array, not firing",
                rule.pid
            );
            return;
        }
    };
    // Each action fires in declared order (array order) and logs its
    // own outcome (FR-32) — one rule with N actions writes N
    // `automation_runs` rows, distinguished by `action_index`, so a
    // partial failure (action 2 of 3) is visible rather than
    // overwriting or being swallowed by the next action's result.
    let firing = FiringContext {
        rule,
        fact,
        subject_kind,
        subject_pid,
        actor,
    };
    for (action_index, action) in actions.iter().enumerate() {
        let action_index = i32::try_from(action_index).unwrap_or(i32::MAX);
        let (outcome, detail) = apply_action(ctx, &firing, action, action_index).await;
        if let Err(err) = cap_models::record_run_on(
            &mut crate::db::pool(ctx),
            rule.pid,
            subject_kind,
            subject_pid,
            action_index,
            outcome,
            detail.clone(),
        )
        .await
        {
            tracing::warn!("automation run log failed: {err}");
        }
        AuditModel::record_or_log_on(
            crate::db::pool(ctx),
            subject_pid,
            "automation_fired",
            actor,
            Some(serde_json::json!({
                "automation_pid": rule.pid.to_string(),
                "name": rule.name,
                "action_index": action_index,
                "action_kind": action.kind,
                "outcome": outcome,
                "detail": detail,
            })),
        )
        .await;
    }
}

/// Sweep every live milestone whose `due` date has arrived (`due <=`
/// today) and is not yet `done`, firing every enabled `milestone_due`
/// rule that matches it — at most [`SWEEP_CAP`] milestones. Returns
/// `(fired, already_claimed, capped)`, where `fired` counts
/// **rule firings** (a milestone matched by two rules counts as two).
///
/// A due date is a **condition**, not a one-shot event, so — unlike
/// `fire` — each (rule, milestone) pair is **claimed** with an
/// `INSERT … ON CONFLICT DO NOTHING` into `automation_milestone_fires`
/// before it is applied: only the caller whose insert actually adds a
/// row proceeds, so two concurrent sweeps (or the ticker racing the
/// endpoint) cannot double-fire the same pair, and a milestone that
/// stays overdue across many sweeps fires exactly once per matching
/// rule, not once per sweep.
///
/// # Errors
///
/// Propagates a database error from the due-milestone query.
pub async fn sweep_milestone_due(ctx: &AppContext) -> Result<(usize, usize, bool)> {
    let today = crate::tz::deployment_today();
    let due: Vec<milestones::Model> = sqlx::query_as(
        "SELECT * FROM milestones
         WHERE due <= $1 AND done = false AND deleted_at IS NULL
         ORDER BY due
         LIMIT $2",
    )
    .bind(today)
    .bind(i64::try_from(SWEEP_CAP).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let capped = due.len() as u64 >= SWEEP_CAP;
    // A linked milestone whose tasks are all done is achieved, not overdue.
    let achieved = super::milestones::achieved_by_tasks(ctx, &due).await?;
    let due: Vec<_> = due
        .into_iter()
        .filter(|m| !achieved.contains(&m.pid))
        .collect();
    let candidates = cap_models::enabled_automations(ctx, "milestone_due").await?;
    let (mut fired, mut already_claimed) = (0, 0);
    for milestone in &due {
        let fact = rules::TriggerFact {
            kind: "milestone_due".to_string(),
            plan_pid: milestone.plan_pid,
            from_status: None,
            to_status: None,
        };
        for rule in &candidates {
            let rule_fact = rules::RuleFact {
                enabled: rule.enabled,
                plan_pid: rule.plan_pid,
                trigger_kind: rule.trigger_kind.clone(),
                from_status: rule.from_status.clone(),
                to_status: rule.to_status.clone(),
            };
            if !rules::rule_matches(&rule_fact, &fact) {
                continue;
            }
            // The claim: `DO NOTHING` suppresses the insert of a pair already claimed, and `RETURNING` then comes back
            // with no row, which is exactly "already claimed", not a failure.
            let claim = sqlx::query_scalar::<_, i32>(
                "INSERT INTO automation_milestone_fires (automation_pid, milestone_pid)
                 VALUES ($1, $2)
                 ON CONFLICT (automation_pid, milestone_pid) DO NOTHING
                 RETURNING id",
            )
            .bind(rule.pid)
            .bind(milestone.pid)
            .fetch_optional(crate::db::pool(ctx))
            .await;
            match claim {
                Ok(Some(_)) => {}
                Ok(None) => {
                    already_claimed += 1;
                    continue;
                }
                Err(err) => {
                    tracing::warn!("milestone-due claim failed: {err}");
                    continue;
                }
            }
            apply_rule_actions(ctx, rule, &fact, "milestone", milestone.pid, None).await;
            fired += 1;
        }
    }
    Ok((fired, already_claimed, capped))
}

/// `POST /api/scheduled-actions` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SchedulePayload {
    /// Kind of the subject: an idea, proposal, plan or task.
    subject_kind: String,
    /// Pid of the subject the action will act on when due.
    subject_pid: Uuid,
    /// What to do at the deadline; must be one of the scheduled action kinds.
    action_kind: String,
    /// Deadline in calendar days from now (1..=[`crate::automation::MAX_SCHEDULE_DAYS`]).
    in_days: i64,
    /// Person, worker or organization reference to notify; required for a `notify` action.
    #[serde(default)]
    recipient_ref: Option<String>,
    /// Optional note carried with the action, capped at `MAX_TEXT_LEN` characters.
    #[serde(default)]
    message: Option<String>,
}

/// `POST /api/scheduled-actions` — configure one deadline and forget
/// it.
#[debug_handler]
async fn create_scheduled_action(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<SchedulePayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if !rules::is_token(
        crate::collaboration::REVIEW_SUBJECT_KINDS,
        &payload.subject_kind,
    ) && payload.subject_kind != "task"
    {
        problems.push(
            "subject_kind must be one of [\"idea\", \"proposal\", \"plan\", \"task\"]".to_string(),
        );
    }
    if !rules::is_token(rules::SCHEDULED_ACTION_KINDS, &payload.action_kind) {
        problems.push(format!(
            "action_kind must be one of {:?}",
            rules::SCHEDULED_ACTION_KINDS
        ));
    }
    if payload.in_days < 1 || payload.in_days > rules::MAX_SCHEDULE_DAYS {
        problems.push(format!(
            "in_days must be between 1 and {}",
            rules::MAX_SCHEDULE_DAYS
        ));
    }
    if payload.action_kind == "notify"
        && !payload
            .recipient_ref
            .as_deref()
            .is_some_and(rules::person_like_ref)
    {
        problems.push(
            "a `notify` action needs a recipient_ref (person:/worker:/organization: URN)"
                .to_string(),
        );
    }
    if let Some(message) = payload.message.as_deref()
        && message.len() > MAX_TEXT_LEN
    {
        problems.push(format!("message exceeds {MAX_TEXT_LEN} characters"));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    // Its subject must exist and be readable: it would otherwise act on a plan the caller cannot see.
    super::discussion::resolve_subject(
        &ctx,
        &caller,
        &payload.subject_kind,
        &payload.subject_pid.to_string(),
    )
    .await?;
    let due_at = chrono::Utc::now() + chrono::Duration::days(payload.in_days);
    let mut tx = crate::db::begin(&ctx).await?;
    let row = cap_models::schedule_on(
        &mut tx,
        cap_models::NewScheduledAction {
            subject_kind: payload.subject_kind.clone(),
            subject_pid: payload.subject_pid,
            action_kind: payload.action_kind.clone(),
            payload: serde_json::json!({
                "recipient_ref": payload.recipient_ref,
                "message": payload.message,
            }),
            due_at: due_at.into(),
            source_automation_pid: None,
            created_by: caller.actor().map(std::string::ToString::to_string),
        },
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "scheduled_action_created",
        caller.actor(),
        Some(serde_json::json!({
            "action_kind": row.action_kind,
            "due_at": row.due_at.to_rfc3339(),
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/scheduled-actions` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ScheduledParams {
    /// Only rows in this status (for example `pending`); absent applies no status filter.
    #[serde(default)]
    status: Option<String>,
    /// Only rows for this subject (its pid).
    #[serde(default)]
    subject: Option<Uuid>,
    /// `true` ⇒ only rows already past their deadline.
    #[serde(default)]
    overdue: Option<bool>,
}

/// `GET /api/scheduled-actions?status=&subject=&overdue=&limit=&cursor=`
/// — the deadline queue, soonest first. Paged by cursor (agents/restful.md):
/// `Link: rel="next"` while more rows follow, and no total; the soonest-first order holds across pages, so page 2
/// picks up exactly where page 1 left off.
///
/// # Errors
///
/// `422` for a bad `limit` or `cursor`, and `400` for a cursor past [`super::pagination::MAX_OFFSET`]; a DB
/// error when the query fails.
#[debug_handler]
async fn list_scheduled_actions(
    State(ctx): State<AppContext>,
    cursor: super::pagination::Cursor,
    caller: MaybeAuthUser,
    Query(params): Query<ScheduledParams>,
) -> Result<Response> {
    let page = cursor.page(LIST_CAP)?;
    let (limit, offset) = (page.fetch(), page.offset());
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM scheduled_actions WHERE deleted_at IS NULL",
    );
    let owned = hidden_pids(&ctx, &caller).await?;
    if !owned.is_empty() {
        query.push(" AND ");
        push_none_of(&mut query, "subject_pid", owned);
    }
    if let Some(status) = params.status {
        query.push(" AND status = ").push_bind(status);
    }
    if let Some(subject) = params.subject {
        query.push(" AND subject_pid = ").push_bind(subject);
    }
    if params.overdue.unwrap_or(false) {
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        query
            .push(" AND status = 'pending' AND due_at <= ")
            .push_bind(now);
    }
    // A unique last key, so two actions due at the same instant keep their order between pages.
    push_page(&mut query, "due_at, id", limit, offset);
    let rows: Vec<scheduled_actions::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    Ok(page.respond(rows))
}

/// `DELETE /api/scheduled-actions/{pid}` — cancel a pending deadline.
/// An action that already fired cannot be cancelled after the fact.
#[debug_handler]
async fn cancel_scheduled_action(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = cap_models::find_scheduled_action(&ctx, crate::models::governance::parse_pid(&pid)?)
        .await?;
    if hidden_pids(&ctx, &caller).await?.contains(&row.subject_pid) {
        return Err(Error::NotFound);
    }
    if row.status != "pending" {
        return Err(unprocessable(&format!(
            "only a pending action can be cancelled; this one is `{}`",
            row.status
        )));
    }
    let row_pid = row.pid;
    let deleted_at: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as `SeaORM`'s was.
    sqlx::query_scalar::<_, i32>(
        "UPDATE scheduled_actions SET status = 'cancelled', deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(deleted_at)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "scheduled_action_cancelled",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// Fire everything due, at most [`SWEEP_CAP`] rows. Returns
/// `(fired, skipped, capped)`.
///
/// Each row is **claimed** with a conditional update
/// (`status = 'pending'` → `'fired'`) and only acted on when the claim
/// won, so two concurrent sweeps — or the ticker racing the endpoint —
/// cannot double-fire one deadline.
///
/// # Errors
///
/// Propagates database errors from the due-row query.
pub async fn sweep_due(ctx: &AppContext) -> Result<(usize, usize, bool)> {
    let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    let due = cap_models::due_actions(ctx, now, SWEEP_CAP).await?;
    let capped = due.len() as u64 >= SWEEP_CAP;
    let (mut fired, mut skipped) = (0, 0);
    for row in due {
        // Claim it: only the sweep that flips pending → fired acts.
        let claimed = sqlx::query(
            "UPDATE scheduled_actions SET status = 'fired', fired_at = $1
             WHERE pid = $2 AND status = 'pending'",
        )
        .bind(now)
        .bind(row.pid)
        .execute(crate::db::pool(ctx))
        .await?;
        if claimed.rows_affected() != 1 {
            skipped += 1;
            continue;
        }
        let outcome = perform_scheduled(ctx, &row).await;
        sqlx::query("UPDATE scheduled_actions SET outcome = $1 WHERE pid = $2")
            .bind(&outcome)
            .bind(row.pid)
            .execute(crate::db::pool(ctx))
            .await
            .ok();
        AuditModel::record_or_log_on(
            crate::db::pool(ctx),
            row.pid,
            "scheduled_action_fired",
            None,
            Some(serde_json::json!({
                "action_kind": row.action_kind,
                "outcome": outcome,
            })),
        )
        .await;
        fired += 1;
    }
    Ok((fired, skipped, capped))
}

/// Carry out one due action. Returns the recorded outcome.
async fn perform_scheduled(ctx: &AppContext, row: &scheduled_actions::Model) -> String {
    match row.action_kind.as_str() {
        "notify" => {
            let recipient = row
                .payload
                .get("recipient_ref")
                .and_then(serde_json::Value::as_str);
            let message = row
                .payload
                .get("message")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("A scheduled reminder came due");
            match recipient {
                Some(recipient) => match cap_models::notify_on(
                    &mut crate::db::pool(ctx),
                    recipient,
                    &row.subject_kind,
                    row.subject_pid,
                    "scheduled",
                    message,
                )
                .await
                {
                    Ok(Some(_)) => "notified".to_string(),
                    Ok(None) => "notified: recipient opted out".to_string(),
                    Err(e) => format!("failed: {e}"),
                },
                None => "failed: no recipient_ref".to_string(),
            }
        }
        "expire_review" => {
            // Expire only invitations still awaiting a verdict; a
            // submitted or declined review is left exactly as it is.
            let expired = sqlx::query(
                "UPDATE reviews SET status = 'expired'
                 WHERE subject_pid = $1 AND status = ANY($2) AND deleted_at IS NULL",
            )
            .bind(row.subject_pid)
            .bind(crate::collaboration::LIVE_REVIEW_STATUSES.to_vec())
            .execute(crate::db::pool(ctx))
            .await;
            match expired {
                Ok(result) => format!("expired {} review(s)", result.rows_affected()),
                Err(e) => format!("failed: {e}"),
            }
        }
        other => format!("failed: unknown action_kind `{other}`"),
    }
}

/// `POST /api/scheduled-actions/sweep` — fire every action now due.
/// Safe to call repeatedly and from more than one caller: claiming is
/// atomic, so an action fires exactly once.
#[debug_handler]
async fn sweep(State(ctx): State<AppContext>) -> Result<Response> {
    let (fired, skipped, capped) = sweep_due(&ctx).await?;
    format::json(serde_json::json!({
        "fired": fired,
        "skipped_already_claimed": skipped,
        "capped": capped,
        "cap": SWEEP_CAP,
        "as_of": chrono::Utc::now().to_rfc3339(),
    }))
}

/// `POST /api/automations/milestones/sweep` — fire every enabled
/// `milestone_due` rule matching a milestone whose `due` date has
/// arrived. Safe to call repeatedly and from more than one caller: the
/// claim (`automation_milestone_fires`) is atomic, so each
/// (rule, milestone) pair fires exactly once, ever — not once per
/// sweep while it stays overdue.
#[debug_handler]
async fn sweep_milestones(State(ctx): State<AppContext>) -> Result<Response> {
    let (fired, already_claimed, capped) = sweep_milestone_due(&ctx).await?;
    format::json(serde_json::json!({
        "fired": fired,
        "already_claimed": already_claimed,
        "capped": capped,
        "cap": SWEEP_CAP,
        "as_of": chrono::Utc::now().to_rfc3339(),
    }))
}

/// The automation routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/automations", post(create_automation))
        .add("/automations", get(list_automations))
        .add("/automations/runs", get(list_runs))
        .add("/automations/{pid}/enable", post(enable_automation))
        .add("/automations/{pid}/disable", post(disable_automation))
        .add("/automations/{pid}", delete(delete_automation))
        .add("/scheduled-actions", post(create_scheduled_action))
        .add("/scheduled-actions", get(list_scheduled_actions))
        .add("/scheduled-actions/sweep", post(sweep))
        .add("/automations/milestones/sweep", post(sweep_milestones))
        .add("/scheduled-actions/{pid}", delete(cancel_scheduled_action))
}

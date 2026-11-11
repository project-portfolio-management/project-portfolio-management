//! **Custom workflows** — the configuration surface over
//! [`crate::workflow`] (entity spec §9.2b / FR-26).
//!
//! A workflow declares the state vocabulary for a plan's tasks or
//! issues. **Every state must declare a category** (`todo` / `active` /
//! `waiting` / `done`), because the board, the burndown, the timeline
//! and every time-based-analysis figure compute from what a state
//! *means*. A state without one is refused here, and the schema refuses
//! it too — a rule this load-bearing does not live in a handler alone.
//!
//! **Resolution order**, used by the task path: the plan's own workflow
//! if it has one, else the deployment default, else the built-in
//! vocabulary. A plan with nothing configured therefore behaves exactly
//! as it did before this feature existed.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{workflow_states, workflow_transitions, workflows};
use crate::models::audit_logs::Model as AuditModel;
use crate::workflow as rules;

/// Rebuild the pure definition from stored rows.
async fn definition_of(ctx: &AppContext, workflow_pid: Uuid) -> Result<rules::WorkflowDef> {
    let states: Vec<workflow_states::Model> =
        sqlx::query_as("SELECT * FROM workflow_states WHERE workflow_pid = $1 ORDER BY position")
            .bind(workflow_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let transitions: Vec<workflow_transitions::Model> =
        sqlx::query_as("SELECT * FROM workflow_transitions WHERE workflow_pid = $1")
            .bind(workflow_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?;

    Ok(rules::WorkflowDef {
        states: states
            .iter()
            .filter_map(|row| {
                // A stored row whose category no longer parses is
                // dropped rather than defaulted. The schema CHECK makes
                // this unreachable today; if it ever happens, losing the
                // state is visible (moves to it are refused) where
                // guessing a category would be silent.
                rules::Category::parse(&row.category).map(|category| rules::StateDef {
                    key: row.state_key.clone(),
                    label: row.label.clone(),
                    labels: serde_json::from_value(row.labels.clone()).unwrap_or_default(),
                    category,
                    wip_limit: row.wip_limit.and_then(|w| u32::try_from(w).ok()),
                    is_initial: row.is_initial,
                    is_terminal: row.is_terminal,
                    field_rules: serde_json::from_value(row.field_rules.clone())
                        .unwrap_or_default(),
                })
            })
            .collect(),
        transitions: transitions
            .iter()
            .map(|row| rules::TransitionDef {
                from: row.from_key.clone(),
                to: row.to_key.clone(),
                roles: serde_json::from_value(row.roles.clone()).unwrap_or_default(),
            })
            .collect(),
    })
}

/// The task workflow a plan **owns**, if it has one (never the default or the
/// built-in) — what a plan template captures.
///
/// # Errors
///
/// Propagates a database error.
pub(crate) async fn own_definition(
    ctx: &AppContext,
    plan_pid: Uuid,
) -> Result<Option<rules::WorkflowDef>> {
    let own: Option<Uuid> = sqlx::query_scalar(
        "SELECT pid FROM workflows
         WHERE plan_pid = $1 AND applies_to = 'task' AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(plan_pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    match own {
        Some(w) => Ok(Some(definition_of(ctx, w).await?)),
        None => Ok(None),
    }
}

/// Register `def` as the task workflow owned by `plan_pid` (a plan template
/// instantiation giving the new plan the vocabulary its source had). This is the
/// service's own path: it copies a workflow onto a new plan, gates included, and
/// nobody can change one through it, so it asks for no administrator.
///
/// # Errors
///
/// Propagates a database error.
pub async fn register_own(
    ctx: &AppContext,
    plan_pid: Uuid,
    name: &str,
    def: &rules::WorkflowDef,
) -> Result<()> {
    let payload = WorkflowPayload {
        name: name.to_string(),
        applies_to: "task".to_string(),
        plan_pid: Some(plan_pid),
        issue_type: None,
        is_default: false,
        states: Vec::new(),
        transitions: Vec::new(),
    };
    persist(&mut crate::db::pool(ctx), &payload, def)
        .await
        .map(|_| ())
}

/// The workflow in force for a plan and resource kind.
///
/// The plan's own, else the deployment default, else the built-in.
/// **Never `None`**: there is always a vocabulary, which is what keeps
/// every existing board working with nothing configured.
///
/// # Errors
///
/// Propagates a database error while resolving the workflow.
pub async fn in_force(
    ctx: &AppContext,
    plan_pid: Uuid,
    applies_to: &str,
) -> Result<rules::WorkflowDef> {
    if let Some(workflow) = plan_own(ctx, plan_pid, applies_to).await? {
        return definition_of(ctx, workflow).await;
    }

    let fallback: Option<Uuid> = sqlx::query_scalar(
        "SELECT pid FROM workflows
         WHERE plan_pid IS NULL AND applies_to = $1 AND is_default = true
           AND issue_type IS NULL AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(applies_to)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    if let Some(workflow) = fallback {
        return definition_of(ctx, workflow).await;
    }

    Ok(if applies_to == "issue" {
        rules::built_in_issue()
    } else {
        rules::built_in_task()
    })
}

/// The live workflow a plan owns for `applies_to` that names no issue type, if any.
async fn plan_own(ctx: &AppContext, plan_pid: Uuid, applies_to: &str) -> Result<Option<Uuid>> {
    Ok(sqlx::query_scalar(
        "SELECT pid FROM workflows
         WHERE plan_pid = $1 AND applies_to = $2 AND issue_type IS NULL AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(plan_pid)
    .bind(applies_to)
    .fetch_optional(crate::db::pool(ctx))
    .await?)
}

/// The workflow an issue of type `kind` on a plan follows (OC-1): the plan's own for that type, else the plan's own,
/// else the deployment default for that type, else the deployment default, else the built-in vocabulary. A plan's own
/// workflow outranks a deployment one for a type, so the nearest scope always decides.
///
/// # Errors
///
/// Propagates a database error while resolving the workflow.
pub async fn in_force_issue(
    ctx: &AppContext,
    plan_pid: Uuid,
    kind: &str,
) -> Result<rules::WorkflowDef> {
    let typed_for_plan: Option<Uuid> = sqlx::query_scalar(
        "SELECT pid FROM workflows
         WHERE plan_pid = $1 AND applies_to = 'issue' AND issue_type = $2 AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(plan_pid)
    .bind(kind)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    if let Some(workflow) = typed_for_plan {
        return definition_of(ctx, workflow).await;
    }
    if let Some(workflow) = plan_own(ctx, plan_pid, "issue").await? {
        return definition_of(ctx, workflow).await;
    }
    let typed_default: Option<Uuid> = sqlx::query_scalar(
        "SELECT pid FROM workflows
         WHERE plan_pid IS NULL AND applies_to = 'issue' AND is_default = true
           AND issue_type = $1 AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(kind)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    if let Some(workflow) = typed_default {
        return definition_of(ctx, workflow).await;
    }
    in_force(ctx, plan_pid, "issue").await
}

/// `POST /api/workflows` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct WorkflowPayload {
    /// Workflow name shown in configuration lists.
    name: String,
    /// What the workflow governs: `task` or `issue`.
    applies_to: String,
    /// Plan the workflow is scoped to; absent means it is not tied to one plan (a deployment-wide definition).
    #[serde(default)]
    plan_pid: Option<Uuid>,
    /// For an issue workflow, the one issue type it governs (a key in `/api/issue-types`); absent governs every type
    /// without a workflow of its own. Not allowed on a task workflow.
    #[serde(default)]
    issue_type: Option<String>,
    /// Marks the deployment default, used by plans that have no workflow of their own.
    #[serde(default)]
    is_default: bool,
    /// Declared states; every one must carry a recognized category.
    states: Vec<StatePayload>,
    /// Allowed moves between states; empty means any move is permitted.
    #[serde(default)]
    transitions: Vec<TransitionPayload>,
}

/// One declared state of a workflow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct StatePayload {
    /// Stable machine key tasks store as their state.
    key: String,
    /// Display label.
    label: String,
    /// Issue workflows only: the label by language code, for languages that should not fall back to `label`.
    #[serde(default)]
    labels: std::collections::BTreeMap<String, String>,
    /// Meaning of the state (`todo`, `active`, `waiting` or `done`). Required and never defaulted, since boards and time analysis compute from it.
    category: String,
    /// Maximum items allowed in the state at once; absent means unlimited.
    #[serde(default)]
    wip_limit: Option<u32>,
    /// Marks the state new items start in.
    #[serde(default)]
    is_initial: bool,
    /// Marks a state work does not leave (typically done).
    #[serde(default)]
    is_terminal: bool,
    /// Issue workflows only: fields this state makes required or read-only, for everyone or for named roles.
    #[serde(default)]
    field_rules: Vec<rules::FieldRule>,
}

/// One allowed move between two states.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TransitionPayload {
    /// State key the move starts from.
    from: String,
    /// State key the move ends at.
    to: String,
    /// Roles allowed to make this move (RM-2); empty/absent means anyone.
    #[serde(default)]
    roles: Vec<String>,
}

/// Write the workflow and its states and transitions.
///
/// Split from the handler so the validation half stays readable; the
/// ordering matters — the parent row first, so a state can never
/// reference a workflow that does not exist.
async fn persist(
    db: &mut impl crate::db::Session,
    payload: &WorkflowPayload,
    def: &rules::WorkflowDef,
) -> Result<workflows::Model> {
    let workflow: workflows::Model = sqlx::query_as(
        "INSERT INTO workflows (pid, plan_pid, name, applies_to, issue_type, is_default, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(payload.plan_pid)
    .bind(payload.name.trim())
    .bind(&payload.applies_to)
    .bind(payload.issue_type.as_deref())
    // A plan-scoped workflow is never the deployment default:
    // "default" means the fallback when a plan has none.
    .bind(payload.is_default && payload.plan_pid.is_none())
    .fetch_one(db.ex())
    .await?;

    persist_definition(db, workflow.pid, def).await?;
    Ok(workflow)
}

/// Write a workflow's states and transitions.
async fn persist_definition(
    db: &mut impl crate::db::Session,
    workflow_pid: Uuid,
    def: &rules::WorkflowDef,
) -> Result<()> {
    for (position, state) in def.states.iter().enumerate() {
        sqlx::query(
            "INSERT INTO workflow_states
                 (pid, workflow_pid, state_key, label, labels, category, wip_limit, is_initial,
                  is_terminal, field_rules, position)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(Uuid::new_v4())
        .bind(workflow_pid)
        .bind(&state.key)
        .bind(&state.label)
        .bind(serde_json::json!(state.labels))
        .bind(state.category.token())
        .bind(state.wip_limit.and_then(|w| i32::try_from(w).ok()))
        .bind(state.is_initial)
        .bind(state.is_terminal)
        .bind(serde_json::json!(state.field_rules))
        .bind(i32::try_from(position).unwrap_or(0))
        .execute(db.ex())
        .await?;
    }
    for transition in &def.transitions {
        sqlx::query(
            "INSERT INTO workflow_transitions (pid, workflow_pid, from_key, to_key, roles)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(Uuid::new_v4())
        .bind(workflow_pid)
        .bind(&transition.from)
        .bind(&transition.to)
        .bind(serde_json::json!(transition.roles))
        .execute(db.ex())
        .await?;
    }
    Ok(())
}

/// `422` for a shape [`rules::validate`] accepts for tasks but an issue cannot take. The database requires a resolution
/// on `closed`, so `closed` must be terminal, and an issue that starts finished, or needing a resolution nobody has
/// written yet, could not be raised at all from a risk, a blocked task or a control action.
fn issue_shape(def: &rules::WorkflowDef) -> Result<()> {
    if def
        .states
        .iter()
        .any(|s| s.key == "closed" && !s.is_terminal)
    {
        return Err(unprocessable("the status `closed` must be terminal"));
    }
    let bad_start = rules::initial_state(def)
        .is_some_and(|s| s.is_terminal || s.category == rules::Category::Done);
    if bad_start {
        return Err(unprocessable(
            "new issues cannot start in a finished or terminal status",
        ));
    }
    Ok(())
}

/// `403` unless the caller holds the administrator attribute. A workflow decides who may move work, so changing one
/// cannot be open to anyone who can write.
fn require_workflow_admin(caller: &MaybeAuthUser) -> Result<()> {
    if super::plan_access::is_admin(caller.claims()) {
        Ok(())
    } else {
        Err(super::forbidden(
            "only an administrator can change a workflow",
        ))
    }
}

/// `409` if the scope already has its issue workflow. The database refuses it too; this says why in words.
async fn refuse_a_second_issue_workflow(ctx: &AppContext, payload: &WorkflowPayload) -> Result<()> {
    if payload.plan_pid.is_none() && !payload.is_default {
        return Ok(());
    }
    let mut find = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT 1 FROM workflows WHERE applies_to = 'issue' AND deleted_at IS NULL",
    );
    match payload.plan_pid {
        Some(plan) => {
            find.push(" AND plan_pid = ").push_bind(plan);
        }
        None => {
            find.push(" AND plan_pid IS NULL AND is_default = true");
        }
    }
    match &payload.issue_type {
        Some(key) => {
            find.push(" AND issue_type = ").push_bind(key.as_str());
        }
        None => {
            find.push(" AND issue_type IS NULL");
        }
    }
    find.push(" LIMIT 1");
    if find
        .build()
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .is_some()
    {
        return Err(super::conflict(
            "this scope already has an issue workflow; withdraw it first",
        ));
    }
    Ok(())
}

/// What an audit row says about a workflow. One plan can hold several, and a workflow decides who may move what.
fn audit_detail(workflow: &workflows::Model, def: &rules::WorkflowDef) -> serde_json::Value {
    serde_json::json!({
        "workflow_pid": workflow.pid, "name": workflow.name, "applies_to": workflow.applies_to,
        "issue_type": workflow.issue_type, "plan_pid": workflow.plan_pid,
        "is_default": workflow.is_default, "transitions": def.transitions,
    })
}

/// What a workflow may govern, and whether this caller may register it.
async fn check_scope(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    payload: &WorkflowPayload,
) -> Result<()> {
    if !["task", "issue"].contains(&payload.applies_to.as_str()) {
        return Err(unprocessable("applies_to must be task or issue"));
    }
    require_workflow_admin(caller)?;
    if payload.applies_to == "task" && payload.issue_type.is_some() {
        return Err(unprocessable(
            "issue_type applies to an issue workflow only",
        ));
    }
    if let Some(key) = &payload.issue_type {
        super::issue_configuration::require_usable_type(ctx, key).await?;
    }
    Ok(())
}

/// `404` unless the plan is live and the caller can read it, so a hidden plan looks the same as one that does not exist.
async fn require_readable_plan(ctx: &AppContext, caller: &MaybeAuthUser, plan: Uuid) -> Result<()> {
    let readable = super::plan_access::readable_plans(ctx, caller, &[plan]).await;
    if readable.contains(&plan) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// The live workflow a path names, if the caller can read its plan; `404` otherwise.
async fn require_workflow(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<workflows::Model> {
    let pid = super::parse_pid(pid)?;
    let workflow: workflows::Model =
        sqlx::query_as("SELECT * FROM workflows WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    if let Some(plan) = workflow.plan_pid {
        require_readable_plan(ctx, caller, plan).await?;
    }
    Ok(workflow)
}

/// Build and check a definition from a request, so registering and changing a workflow refuse the same things.
async fn definition_from(
    ctx: &AppContext,
    applies_to: &str,
    requested_states: &[StatePayload],
    requested_transitions: &[TransitionPayload],
) -> Result<rules::WorkflowDef> {
    // An unrecognized category is refused **by name**, so an operator
    // sees which state is wrong rather than that something is.
    let mut states = Vec::with_capacity(requested_states.len());
    for state in requested_states {
        let Some(category) = rules::Category::parse(&state.category) else {
            return Err(unprocessable(&format!(
                "state `{}` declares category `{}`, which is not one of todo, active, \
                 waiting, done. Every state must declare one: the board, the burndown \
                 and every flow figure are computed from what a state means, not its name.",
                state.key, state.category
            )));
        };
        states.push(rules::StateDef {
            key: state.key.trim().to_string(),
            label: state.label.trim().to_string(),
            labels: super::issue_configuration::usable_labels(&state.labels)?,
            category,
            wip_limit: state.wip_limit,
            is_initial: state.is_initial,
            is_terminal: state.is_terminal,
            field_rules: state
                .field_rules
                .iter()
                .map(|r| rules::FieldRule {
                    roles: r.roles.iter().map(|x| x.trim().to_string()).collect(),
                    ..r.clone()
                })
                .collect(),
        });
    }

    let def = rules::WorkflowDef {
        states,
        transitions: requested_transitions
            .iter()
            .map(|t| rules::TransitionDef {
                from: t.from.trim().to_string(),
                to: t.to.trim().to_string(),
                roles: t.roles.iter().map(|r| r.trim().to_string()).collect(),
            })
            .collect(),
    };
    if def
        .transitions
        .iter()
        .flat_map(|t| &t.roles)
        .chain(
            def.states
                .iter()
                .flat_map(|s| &s.field_rules)
                .flat_map(|r| &r.roles),
        )
        .any(|r| r.is_empty() || r.chars().count() > 64)
    {
        return Err(unprocessable(
            "roles must be non-blank names of at most 64 characters",
        ));
    }
    if applies_to == "issue" {
        issue_shape(&def)?;
    } else if def
        .states
        .iter()
        .any(|s| !s.field_rules.is_empty() || !s.labels.is_empty())
    {
        return Err(unprocessable(
            "field rules and translated labels apply to an issue workflow only",
        ));
    }
    let named: Vec<&str> = def
        .transitions
        .iter()
        .flat_map(|t| t.roles.iter().map(String::as_str))
        .chain(
            def.states
                .iter()
                .flat_map(|s| &s.field_rules)
                .flat_map(|r| r.roles.iter().map(String::as_str)),
        )
        .collect();
    super::issue_configuration::require_known_roles(ctx, &named).await?;
    if let Err(problems) = rules::validate(&def) {
        return Err(unprocessable(&format!(
            "workflow is not usable: {}",
            serde_json::to_string(&problems).unwrap_or_default()
        )));
    }
    Ok(def)
}

/// `POST /api/workflows` — register a workflow.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<WorkflowPayload>,
) -> Result<Response> {
    if let Some(plan_pid) = payload.plan_pid {
        require_readable_plan(&ctx, &caller, plan_pid).await?;
    }
    check_scope(&ctx, &caller, &payload).await?;
    if payload.name.trim().is_empty() || payload.name.len() > crate::validation::MAX_TEXT_LEN {
        return Err(unprocessable("name is required (and capped)"));
    }

    let def = definition_from(
        &ctx,
        &payload.applies_to,
        &payload.states,
        &payload.transitions,
    )
    .await?;

    // The workflow, its states and transitions, and the audit row are one change.
    if payload.applies_to == "issue" {
        refuse_a_second_issue_workflow(&ctx, &payload).await?;
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let workflow = persist(&mut tx, &payload, &def).await?;

    AuditModel::record_on(
        tx.conn(),
        payload.plan_pid.unwrap_or(workflow.pid),
        "workflow_registered",
        caller.actor(),
        Some(audit_detail(&workflow, &def)),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": workflow.pid.to_string() }))
}

/// `PUT /api/workflows/{pid}` body. What the workflow governs cannot change: withdraw it and register another.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ChangeWorkflow {
    /// Workflow name shown in configuration lists.
    name: String,
    /// The states it declares, replacing those it has.
    states: Vec<StatePayload>,
    /// Its allowed moves, replacing those it has; empty means any move is permitted.
    #[serde(default)]
    transitions: Vec<TransitionPayload>,
}

/// `PUT /api/workflows/{pid}` — change an issue workflow in place (an administrator): one change, audited with what it was.
/// An issue in a status that goes keeps it, as when a workflow is withdrawn. A task workflow is withdrawn and registered
/// again, which keeps the check on work left in a removed state.
#[debug_handler]
async fn change(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ChangeWorkflow>,
) -> Result<Response> {
    let workflow = require_workflow(&ctx, &caller, &pid).await?;
    if workflow.applies_to != "issue" {
        return Err(unprocessable(
            "only an issue workflow can be changed in place",
        ));
    }
    require_workflow_admin(&caller)?;
    let name = payload.name.trim().to_string();
    if name.is_empty() || name.len() > crate::validation::MAX_TEXT_LEN {
        return Err(unprocessable("name is required (and capped)"));
    }
    let def = definition_from(&ctx, "issue", &payload.states, &payload.transitions).await?;
    let was = audit_detail(&workflow, &definition_of(&ctx, workflow.pid).await?);
    let target = workflow.plan_pid.unwrap_or(workflow.pid);
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    let workflow: workflows::Model =
        sqlx::query_as("UPDATE workflows SET name = $1 WHERE id = $2 RETURNING *")
            .bind(&name)
            .bind(workflow.id)
            .fetch_one(tx.conn())
            .await?;
    sqlx::query("DELETE FROM workflow_states WHERE workflow_pid = $1")
        .bind(workflow.pid)
        .execute(tx.conn())
        .await?;
    sqlx::query("DELETE FROM workflow_transitions WHERE workflow_pid = $1")
        .bind(workflow.pid)
        .execute(tx.conn())
        .await?;
    persist_definition(&mut tx, workflow.pid, &def).await?;
    AuditModel::record_on(
        tx.conn(),
        target,
        "workflow_changed",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": audit_detail(&workflow, &def) })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": workflow.pid.to_string() }))
}

/// One state's words, as a list that has no workflow to hand needs them.
#[derive(Debug, Clone)]
pub(crate) struct StatusWords {
    /// The label.
    pub label: String,
    /// The label by language code.
    pub labels: serde_json::Value,
}

/// One live issue workflow's words for one status.
struct BookRow {
    /// The plan it belongs to; `None` is deployment-wide.
    plan: Option<Uuid>,
    /// The issue type it governs; `None` is every type without its own.
    issue_type: Option<String>,
    /// Whether it is a deployment default.
    is_default: bool,
    /// The status.
    status: String,
    /// What the status is called.
    words: StatusWords,
}

/// The live issue workflows' state words, read once for a list that spans plans and types.
pub(crate) struct StatusBook {
    /// One row per state.
    rows: Vec<BookRow>,
}

impl StatusBook {
    /// Reads the live issue workflows.
    pub(crate) async fn load(ctx: &AppContext) -> Result<Self> {
        let live: Vec<workflows::Model> = sqlx::query_as(
            "SELECT * FROM workflows WHERE applies_to = 'issue' AND deleted_at IS NULL",
        )
        .fetch_all(crate::db::pool(ctx))
        .await?;
        let live_pids: Vec<Uuid> = live.iter().map(|w| w.pid).collect();
        let states: Vec<workflow_states::Model> =
            sqlx::query_as("SELECT * FROM workflow_states WHERE workflow_pid = ANY($1)")
                .bind(&live_pids)
                .fetch_all(crate::db::pool(ctx))
                .await?;
        let rows = states
            .into_iter()
            .filter_map(|state| {
                let w = live.iter().find(|w| w.pid == state.workflow_pid)?;
                Some(BookRow {
                    plan: w.plan_pid,
                    issue_type: w.issue_type.clone(),
                    is_default: w.is_default,
                    status: state.state_key,
                    words: StatusWords {
                        label: state.label,
                        labels: state.labels,
                    },
                })
            })
            .collect();
        Ok(Self { rows })
    }

    /// The words for a status on a plan and type, nearest scope first, as [`in_force_issue`] orders them. `None` for a
    /// status no live workflow in those scopes declares.
    pub(crate) fn words(&self, plan: Uuid, kind: &str, status: &str) -> Option<&StatusWords> {
        let find = |wanted_plan: Option<Uuid>, typed: bool| {
            self.rows
                .iter()
                .find(|row| {
                    row.plan == wanted_plan
                        && row.status == status
                        && (wanted_plan.is_some() || row.is_default)
                        && if typed {
                            row.issue_type.as_deref() == Some(kind)
                        } else {
                            row.issue_type.is_none()
                        }
                })
                .map(|row| &row.words)
        };
        find(Some(plan), true)
            .or_else(|| find(Some(plan), false))
            .or_else(|| find(None, true))
            .or_else(|| find(None, false))
    }
}

/// `GET /api/workflows` — every registered workflow, leaving out those of a plan the caller cannot read: a plan's
/// workflow names the plan and how its work is typed.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let mut rows: Vec<workflows::Model> =
        sqlx::query_as("SELECT * FROM workflows WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut plan_pids: Vec<Uuid> = rows.iter().filter_map(|w| w.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plan_pids).await;
    rows.retain(|w| w.plan_pid.is_none_or(|p| readable.contains(&p)));
    format::json(serde_json::json!(rows))
}

/// `GET /api/workflows/{pid}` — one workflow with its states and moves, so a form can change it.
#[debug_handler]
async fn one(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let row = require_workflow(&ctx, &caller, &pid).await?;
    let def = definition_of(&ctx, row.pid).await?;
    let mut out = serde_json::json!(row);
    out["workflow"] = serde_json::json!(def);
    format::json(out)
}

/// `DELETE /api/workflows/{pid}` — withdraw a workflow.
///
/// Refused while any live task sits in a state only this workflow
/// declares: withdrawing it would leave that work in a state no
/// vocabulary explains, which is exactly the uncategorized-state
/// problem the category column exists to prevent.
#[debug_handler]
async fn remove(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let workflow = require_workflow(&ctx, &caller, &pid).await?;

    require_workflow_admin(&caller)?;
    // An issue keeps its status and category when its workflow goes, and can move on from there (`may_move_issue`).
    if let Some(plan_pid) = workflow.plan_pid.filter(|_| workflow.applies_to == "task") {
        let def = definition_of(&ctx, workflow.pid).await?;
        let built_in = if workflow.applies_to == "issue" {
            rules::built_in_issue()
        } else {
            rules::built_in_task()
        };
        let live: Vec<String> = sqlx::query_scalar(
            "SELECT status FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL",
        )
        .bind(plan_pid)
        .fetch_all(crate::db::pool(&ctx))
        .await?;
        let stranded: Vec<&str> = live
            .iter()
            .map(String::as_str)
            .filter(|status| {
                rules::category_of(&def, status).is_some()
                    && rules::category_of(&built_in, status).is_none()
            })
            .collect();
        if !stranded.is_empty() {
            return Err(unprocessable(&format!(
                "live work sits in {} state(s) only this workflow declares (e.g. `{}`); \
                 move that work first, or those items would be left in a state no \
                 vocabulary explains",
                stranded.len(),
                stranded.first().copied().unwrap_or_default()
            )));
        }
    }

    let detail = audit_detail(&workflow, &definition_of(&ctx, workflow.pid).await?);
    let plan_pid = workflow.plan_pid.unwrap_or(workflow.pid);
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    sqlx::query_scalar::<_, i32>("UPDATE workflows SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::DateTime::<chrono::FixedOffset>::from(
            chrono::Utc::now(),
        ))
        .bind(workflow.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        plan_pid,
        "workflow_withdrawn",
        caller.actor(),
        Some(detail),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `GET /api/plans/{pid}/workflow?applies_to=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct InForceParams {
    /// Which workflow type to report, `task` (default) or `issue`.
    #[serde(default)]
    applies_to: Option<String>,
}

/// `GET /api/plans/{pid}/workflow` — the vocabulary actually in force,
/// and where it came from.
#[debug_handler]
async fn plan_workflow(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    Query(params): Query<InForceParams>,
    headers: HeaderMap,
) -> Result<Response> {
    let plan_pid = super::parse_pid(&pid)?;
    let plan: Uuid =
        sqlx::query_scalar("SELECT pid FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;

    let applies_to = params.applies_to.unwrap_or_else(|| "task".to_string());
    if !["task", "issue"].contains(&applies_to.as_str()) {
        return Err(unprocessable("applies_to must be task or issue"));
    }
    let def = in_force(&ctx, plan, &applies_to).await?;

    let own = plan_own(&ctx, plan, &applies_to).await?;

    let body = serde_json::json!({
        "plan_pid": plan.to_string(),
        "applies_to": applies_to,
        // Naming the source matters: "why can I not move this card"
        // is answered by which vocabulary is in force, not by the
        // vocabulary alone.
        "source": if own.is_some() { "plan" } else { "built_in_or_default" },
        "constrained": !def.transitions.is_empty(),
        "workflow": def,
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// The workflow routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/workflows", post(create))
        .add("/workflows", get(list))
        .add("/workflows/{pid}", get(one))
        .add("/workflows/{pid}", put(change))
        .add("/workflows/{pid}", delete(remove))
        .add("/plans/{pid}/workflow", get(plan_workflow))
}

//! Plan templates (LT-2): save a plan tree as a reusable template and
//! instantiate a new tree from it. Pure model and rules in
//! [`crate::plan_templates`].
//!
//! **Saving** captures the plan and its descendants (breadth-first, bounded)
//! that the caller can read — a plan hidden from them is left out with its
//! subtree and counted. **Instantiating** creates every plan through
//! `streaming::create_and_emit`, so events and audit are written exactly as
//! for a single `POST /api/plans`. People and identity are never copied, dates
//! are re-derived from the start date given, and tasks come back at the
//! workflow's initial state with no history.
//!
//! Instantiation is **not one transaction** (each plan's outbox write is its
//! own); if it fails part-way, the plans already created are soft-deleted and
//! the failure is reported, so a half-built tree is never left behind.

use super::conflict;
use super::unprocessable;
use crate::prelude::*;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::plan_access::readable_plans;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{components, milestones, plan_templates, plans, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::plans::Model as PlanModel;
use crate::plan_templates::{self as rules, Snapshot, TplMilestone, TplNode, TplTask};
use crate::streaming;

/// A plan as captured, before offsets are computed.
struct Raw {
    /// The plan row itself.
    row: plans::Model,
    /// Index of the parent plan within the captured list; None for the root of the subtree.
    parent: Option<usize>,
    /// Live tasks of this plan.
    tasks: Vec<tasks::Model>,
    /// Live components of this plan.
    components: Vec<components::Model>,
    /// Live milestones of this plan.
    milestones: Vec<milestones::Model>,
}

#[expect(clippy::too_many_lines)] // one linear pipeline: walk, load, convert
/// Capture the readable subtree under `root` into a snapshot; returns it with
/// the number of plans left out because the caller cannot read them.
pub(super) async fn capture(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    root: plans::Model,
) -> Result<(Snapshot, usize)> {
    let mut raws: Vec<Raw> = vec![Raw {
        row: root,
        parent: None,
        tasks: vec![],
        components: vec![],
        milestones: vec![],
    }];
    let mut frontier: Vec<usize> = vec![0];
    let mut omitted = 0usize;
    for _depth in 0..rules::MAX_DEPTH {
        if frontier.is_empty() {
            break;
        }
        let parents: Vec<Uuid> = frontier.iter().map(|i| raws[*i].row.pid).collect();
        let children: Vec<plans::Model> = sqlx::query_as(
            "SELECT * FROM plans WHERE parent_pid = ANY($1) AND deleted_at IS NULL ORDER BY id",
        )
        .bind(parents)
        .fetch_all(crate::db::pool(ctx))
        .await?;
        let pids: Vec<Uuid> = children.iter().map(|c| c.pid).collect();
        let readable = readable_plans(ctx, caller, &pids).await;
        let mut next = Vec::new();
        for child in children {
            if !readable.contains(&child.pid) {
                omitted += 1;
                continue;
            }
            if raws.len() >= rules::MAX_PLANS {
                return Err(unprocessable(&format!(
                    "a template holds at most {} plans",
                    rules::MAX_PLANS
                )));
            }
            let parent = raws
                .iter()
                .position(|r| Some(r.row.pid) == child.parent_pid);
            raws.push(Raw {
                row: child,
                parent,
                tasks: vec![],
                components: vec![],
                milestones: vec![],
            });
            next.push(raws.len() - 1);
        }
        frontier = next;
    }
    let mut total_tasks = 0usize;
    for r in &mut raws {
        let pid = r.row.pid;
        r.tasks = sqlx::query_as(
            "SELECT * FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
        )
        .bind(pid)
        .fetch_all(crate::db::pool(ctx))
        .await?;
        total_tasks += r.tasks.len();
        r.components = sqlx::query_as(
            "SELECT * FROM components WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
        )
        .bind(pid)
        .fetch_all(crate::db::pool(ctx))
        .await?;
        r.milestones = sqlx::query_as(
            "SELECT * FROM milestones WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY due",
        )
        .bind(pid)
        .fetch_all(crate::db::pool(ctx))
        .await?;
    }
    if total_tasks > rules::MAX_TASKS {
        return Err(unprocessable(&format!(
            "a template holds at most {} tasks",
            rules::MAX_TASKS
        )));
    }

    // Dates → offsets from one base.
    let root_plan = raws[0].row.to_plan()?;
    let mut known: Vec<NaiveDate> = Vec::new();
    for r in &raws {
        known.extend(r.milestones.iter().map(|m| m.due));
        known.extend(r.tasks.iter().filter_map(|t| t.due_date));
        if let Ok(p) = r.row.to_plan() {
            known.extend(p.start_date.as_deref().and_then(rules::full_date));
            known.extend(p.target_date.as_deref().and_then(rules::full_date));
        }
    }
    let base = rules::choose_base(root_plan.start_date.as_deref(), &known);
    let off = |d: NaiveDate| base.map(|b| rules::offset(b, d));

    let mut nodes = Vec::new();
    for (key, r) in raws.iter().enumerate() {
        let plan = r.row.to_plan()?;
        let comp_name = |pid: Option<Uuid>| {
            pid.and_then(|c| r.components.iter().find(|x| x.pid == c))
                .map(|c| c.name.clone())
        };
        let task_key =
            |pid: Option<Uuid>| pid.and_then(|p| r.tasks.iter().position(|t| t.pid == p));
        nodes.push(TplNode {
            key,
            parent: r.parent,
            start_offset_days: plan
                .start_date
                .as_deref()
                .and_then(rules::full_date)
                .and_then(off),
            target_offset_days: plan
                .target_date
                .as_deref()
                .and_then(rules::full_date)
                .and_then(off),
            plan: rules::scrub(&plan),
            components: r.components.iter().map(|c| c.name.clone()).collect(),
            tasks: r
                .tasks
                .iter()
                .enumerate()
                .map(|(i, t)| TplTask {
                    key: i,
                    title: t.title.clone(),
                    description: t.description.clone(),
                    points: t.points,
                    flow_type: t.flow_type.clone(),
                    due_offset_days: t.due_date.and_then(off),
                    component: comp_name(t.component_pid),
                    parent: task_key(t.parent_task_pid),
                })
                .collect(),
            milestones: r
                .milestones
                .iter()
                .filter_map(|m| {
                    off(m.due).map(|o| TplMilestone {
                        name: m.name.clone(),
                        kind: m.kind.clone(),
                        offset_days: o,
                    })
                })
                .collect(),
            workflow: super::workflow::own_definition(ctx, r.row.pid).await?,
        });
    }
    let has_offsets = nodes.iter().any(|n| {
        n.start_offset_days.is_some()
            || n.target_offset_days.is_some()
            || !n.milestones.is_empty()
            || n.tasks.iter().any(|t| t.due_offset_days.is_some())
    });
    Ok((
        Snapshot {
            base_date: base,
            has_offsets,
            nodes,
        },
        omitted,
    ))
}

/// `POST /api/plans/{pid}/save-as-template` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SaveBody {
    /// Name for the new template.
    name: String,
    /// Optional description; absent means none.
    #[serde(default)]
    description: Option<String>,
}

/// `POST /api/plans/{pid}/save-as-template`.
#[debug_handler]
async fn save_as_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<SaveBody>,
) -> Result<Response> {
    let name = b.name.trim().to_string();
    if name.is_empty() || name.chars().count() > 120 {
        return Err(unprocessable("name is required (at most 120 characters)"));
    }
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let root = PlanModel::find_by_pid(&ctx, &subject.pid.to_string()).await?;
    // Only the names are compared (in Rust, as before, so "the same name" is Rust's `to_lowercase`, not the database's).
    let existing: Vec<String> =
        sqlx::query_scalar("SELECT name FROM plan_templates WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    if crate::starter_templates::is_starter_name(&name)
        || existing
            .iter()
            .any(|t| t.to_lowercase() == name.to_lowercase())
    {
        return Err(conflict("a template with this name already exists"));
    }
    let (snapshot, omitted) = capture(&ctx, &caller, root).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let (row_pid, row_name): (Uuid, String) = sqlx::query_as(
        "INSERT INTO plan_templates (pid, name, description, snapshot, created_by, deleted_at, source_plan_pid)
         VALUES ($1, $2, $3, $4, $5, NULL, $6)
         RETURNING pid, name",
    )
    .bind(Uuid::new_v4())
    .bind(name)
    .bind(&b.description)
    .bind(serde_json::json!(snapshot))
    .bind(caller.actor())
    .bind(subject.pid)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "plan_template_saved",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "pid": row_pid,
        "name": row_name,
        "plans": snapshot.nodes.len(),
        "tasks": snapshot.nodes.iter().map(|n| n.tasks.len()).sum::<usize>(),
        "milestones": snapshot.nodes.iter().map(|n| n.milestones.len()).sum::<usize>(),
        "needs_start_date": snapshot.has_offsets,
        "omitted_unreadable": omitted,
    }))
}

/// `GET /api/plan-templates` — name, description and size, not the snapshot. A template of a plan the caller cannot see is
/// left out.
#[debug_handler]
async fn list_templates(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let rows: Vec<plan_templates::Model> =
        sqlx::query_as("SELECT * FROM plan_templates WHERE deleted_at IS NULL ORDER BY id")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let rows = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .keep(rows);
    let items: Vec<serde_json::Value> = rows
        .iter()
        .map(|t| {
            let nodes = t.snapshot.get("nodes").and_then(|n| n.as_array()).map_or(0, Vec::len);
            serde_json::json!({
                "pid": t.pid, "name": t.name, "description": t.description,
                "starter": false,
                "plans": nodes,
                "needs_start_date": t.snapshot.get("has_offsets").and_then(serde_json::Value::as_bool).unwrap_or(false),
            })
        })
        .chain(crate::starter_templates::starters().into_iter().map(|s| {
            serde_json::json!({
                "pid": s.pid, "name": s.name, "description": s.description,
                "starter": true,
                "plans": s.snapshot.nodes.len(),
                "needs_start_date": s.snapshot.has_offsets,
            })
        }))
        .collect();
    format::json(items)
}

/// A starter (BR-13) as a template row: it is code, so it has no database row and is always version 1.
fn starter_row(s: &crate::starter_templates::Starter) -> plan_templates::Model {
    let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    plan_templates::Model {
        created_at: now,
        updated_at: now,
        id: 0,
        pid: s.pid,
        name: s.name.to_string(),
        description: Some(s.description.to_string()),
        snapshot: serde_json::to_value(&s.snapshot).unwrap_or_default(),
        version: 1,
        created_by: Some("system:starter".to_string()),
        deleted_at: None,
        source_plan_pid: None,
    }
}

/// `422` when the template is a starter: it is not edited, revised or deleted in place; save a copy instead.
pub(super) fn refuse_starter(row: &plan_templates::Model) -> Result<()> {
    if crate::starter_templates::find(row.pid).is_some() {
        return Err(unprocessable(
            "this is a starter template and is not changed in place; instantiate it and save the result under your own name",
        ));
    }
    Ok(())
}

/// Load a template by public id: a built-in starter is served from code, any other must exist and not be soft-deleted, else 404.
pub(super) async fn find_template(ctx: &AppContext, pid: &str) -> Result<plan_templates::Model> {
    let pid = super::parse_pid(pid)?;
    if let Some(s) = crate::starter_templates::find(pid) {
        return Ok(starter_row(&s));
    }
    sqlx::query_as("SELECT * FROM plan_templates WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// [`find_template`] for a caller: `404` for a template of a plan hidden from them. Hidden, not unreadable: a template
/// outlives a plan that is deleted, so a plan that has gone does not take its template with it.
pub(super) async fn find_visible_template(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<plan_templates::Model> {
    let row = find_template(ctx, pid).await?;
    if let Some(source) = row.source_plan_pid
        && super::plan_access::hidden_plans(ctx, &[source], caller)
            .await
            .contains(&source)
    {
        return Err(Error::NotFound);
    }
    Ok(row)
}

/// `GET /api/plan-templates/{pid}` — the full snapshot.
#[debug_handler]
async fn get_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    format::json(find_visible_template(&ctx, &caller, &pid).await?)
}

/// `POST /api/plan-templates/{pid}/delete` (and `DELETE`) — soft-delete.
#[debug_handler]
async fn delete_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_visible_template(&ctx, &caller, &pid).await?;
    refuse_starter(&row)?;
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE plan_templates SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(
        chrono::Utc::now(),
    ))
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "plan_template_deleted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `POST /api/plans/from-template/{tpl}` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct InstantiateBody {
    /// The new root plan's name (default: the template root's name).
    #[serde(default)]
    name: Option<String>,
    /// Required when the template captured any date.
    #[serde(default)]
    start_date: Option<NaiveDate>,
    /// Put the new root under this plan.
    #[serde(default)]
    parent_pid: Option<Uuid>,
}

/// `POST /api/plans/from-template/{tpl}`.
#[debug_handler]
async fn from_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(tpl): Path<String>,
    Json(b): Json<InstantiateBody>,
) -> Result<Response> {
    let row = find_visible_template(&ctx, &caller, &tpl).await?;
    let snap: Snapshot = serde_json::from_value(row.snapshot.clone())
        .map_err(|e| unprocessable(&format!("the template snapshot is unreadable: {e}")))?;
    if snap.has_offsets && b.start_date.is_none() {
        return Err(unprocessable(
            "start_date is required: this template carries dates as offsets",
        ));
    }
    if let Some(parent) = b.parent_pid {
        resolve_subject(&ctx, &caller, "plan", &parent.to_string())
            .await
            .map_err(|_| unprocessable("parent_pid names no plan this caller can see"))?;
    }
    let start = b.start_date;
    let date_of = |days: Option<i64>| match (start, days) {
        (Some(s), Some(d)) => Some(rules::shift(s, d)),
        _ => None,
    };

    let mut created: Vec<Uuid> = Vec::new();
    let result = build(
        &ctx,
        &caller,
        &snap,
        &b,
        &date_of,
        (row.pid, row.version),
        &mut created,
    )
    .await;
    // The plans are built one at a time, so they cannot share a transaction with the audit row; if the audit
    // row cannot be written the whole instantiation is undone the same way a failed build is.
    let result = match result {
        Ok(summary) => AuditModel::record_on(
            crate::db::pool(&ctx),
            row.pid,
            "plan_template_instantiated",
            caller.actor(),
            Some(serde_json::json!({ "root": created.first() })),
        )
        .await
        .map(|_| summary)
        .map_err(super::audit_failed),
        Err(e) => Err(e),
    };
    match result {
        Ok(summary) => format::json(summary),
        Err(e) => {
            // Roll back: soft-delete what was created, then report the failure.
            sqlx::query("DELETE FROM plan_template_links WHERE plan_pid = ANY($1)")
                .bind(&created)
                .execute(crate::db::pool(&ctx))
                .await
                .ok();
            for pid in created.iter().rev() {
                if let Ok(m) = PlanModel::find_by_pid(&ctx, &pid.to_string()).await {
                    streaming::delete_and_emit(&ctx, m, caller.actor())
                        .await
                        .ok();
                }
            }
            Err(e)
        }
    }
}

/// Create the plan tree described by a snapshot, shifting dates by the instantiate request. Every plan created is pushed onto `created` as it goes so the caller can undo a partial build. Returns the summary JSON.
#[expect(clippy::too_many_lines)] // one linear pipeline per plan: plan, workflow, components, tasks, milestones
async fn build(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    snap: &Snapshot,
    b: &InstantiateBody,
    date_of: &impl Fn(Option<i64>) -> Option<NaiveDate>,
    template: (Uuid, i32),
    created: &mut Vec<Uuid>,
) -> Result<serde_json::Value> {
    let mut pids: Vec<Uuid> = Vec::new();
    let (mut n_tasks, mut n_milestones) = (0usize, 0usize);
    for node in &snap.nodes {
        let mut plan = node.plan.clone();
        if node.parent.is_none()
            && let Some(name) = b.name.as_deref().map(str::trim).filter(|n| !n.is_empty())
        {
            plan.name = name.to_string();
        }
        plan.start_date = date_of(node.start_offset_days).map(|d| d.to_string());
        plan.target_date = date_of(node.target_offset_days).map(|d| d.to_string());
        plan.parent_ref = match node.parent {
            Some(p) => pids.get(p).map(ToString::to_string),
            None => b.parent_pid.map(|p| p.to_string()),
        };
        super::plans::validate(&plan)?;
        let model = streaming::create_and_emit(ctx, &plan, caller.actor()).await?;
        pids.push(model.pid);
        created.push(model.pid);
        let plan_pid = model.pid;

        if let Some(def) = &node.workflow {
            super::workflow::register_own(ctx, plan_pid, "From template", def).await?;
        }
        let workflow = super::workflow::in_force(ctx, plan_pid, "task").await?;
        let initial = workflow
            .states
            .iter()
            .find(|s| s.is_initial)
            .map_or("todo", |s| s.key.as_str())
            .to_string();

        let mut comp_pids: Vec<(String, Uuid)> = Vec::new();
        for name in &node.components {
            let c: Uuid = sqlx::query_scalar(
                "INSERT INTO components (pid, plan_pid, name, lead_ref, deleted_at)
                 VALUES ($1, $2, $3, NULL, NULL)
                 RETURNING pid",
            )
            .bind(Uuid::new_v4())
            .bind(plan_pid)
            .bind(name)
            .fetch_one(crate::db::pool(ctx))
            .await?;
            comp_pids.push((name.clone(), c));
        }
        let now = chrono::Utc::now();
        let mut task_pids: Vec<Uuid> = Vec::new();
        for t in &node.tasks {
            let tp = Uuid::new_v4();
            let at: chrono::DateTime<chrono::FixedOffset> = now.into();
            sqlx::query(
                "INSERT INTO tasks
                     (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points, due_date,
                      component_pid, parent_task_pid, status_changed_at, flow_type, done_at, deleted_at)
                 VALUES ($1, $2, NULL, $3, $4, $5, NULL, $6, $7, $8, NULL, $9, $10, $11, NULL)",
            )
            .bind(tp)
            .bind(plan_pid)
            .bind(&t.title)
            .bind(&t.description)
            .bind(&initial)
            .bind(t.points)
            .bind(date_of(t.due_offset_days))
            .bind(
                t.component
                    .as_ref()
                    .and_then(|n| comp_pids.iter().find(|(x, _)| x == n))
                    .map(|(_, p)| *p),
            )
            .bind(at)
            .bind(&t.flow_type)
            .bind(crate::workflow::is_done(&workflow, &initial).then_some(at))
            .execute(crate::db::pool(ctx))
            .await?;
            crate::tba::insert_transition_on(
                &mut crate::db::pool(ctx),
                crate::tba::NewTransition {
                    task_pid: tp,
                    plan_pid,
                    from_status: None,
                    to_status: &initial,
                    at: now,
                    actor_ref: caller.actor(),
                    assignee_ref: None,
                    blocked_reason: None,
                },
            )
            .await?;
            task_pids.push(tp);
            n_tasks += 1;
        }
        // Second pass: parents, now that every task has a pid.
        for t in &node.tasks {
            if let Some(parent_key) = t.parent
                && let (Some(child), Some(parent)) =
                    (task_pids.get(t.key), task_pids.get(parent_key))
            {
                sqlx::query("UPDATE tasks SET parent_task_pid = $1 WHERE pid = $2")
                    .bind(*parent)
                    .bind(*child)
                    .execute(crate::db::pool(ctx))
                    .await?;
            }
        }
        // Remember where the plan came from, for blueprint sync (NC-12).
        let task_map: Vec<serde_json::Value> = node
            .tasks
            .iter()
            .filter_map(|t| {
                task_pids
                    .get(t.key)
                    .map(|p| serde_json::json!({ "key": t.key, "task_pid": p }))
            })
            .collect();
        sqlx::query(
            "INSERT INTO plan_template_links (plan_pid, template_pid, node_key, template_version, baseline, task_map)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(plan_pid)
        .bind(template.0)
        .bind(i32::try_from(node.key).unwrap_or(0))
        .bind(template.1)
        .bind(serde_json::json!(node.tasks))
        .bind(serde_json::json!(task_map))
        .execute(crate::db::pool(ctx))
        .await?;
        for m in &node.milestones {
            let Some(due) = date_of(Some(m.offset_days)) else {
                continue;
            };
            sqlx::query(
                "INSERT INTO milestones (pid, plan_pid, name, due, done, done_at, kind, deleted_at)
                 VALUES ($1, $2, $3, $4, false, NULL, $5, NULL)",
            )
            .bind(Uuid::new_v4())
            .bind(plan_pid)
            .bind(&m.name)
            .bind(due)
            .bind(&m.kind)
            .execute(crate::db::pool(ctx))
            .await?;
            n_milestones += 1;
        }
    }
    Ok(serde_json::json!({
        "root_pid": pids.first(),
        "plans": pids,
        "tasks_created": n_tasks,
        "milestones_created": n_milestones,
    }))
}

/// The plan-template routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/save-as-template", post(save_as_template))
        .add("/plans/from-template/{tpl}", post(from_template))
        .add("/plan-templates", get(list_templates))
        .add("/plan-templates/{pid}", get(get_template))
        .add("/plan-templates/{pid}", delete(delete_template))
        .add("/plan-templates/{pid}/delete", post(delete_template))
}

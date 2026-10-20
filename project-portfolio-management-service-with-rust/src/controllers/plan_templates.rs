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
use super::{db_err, unprocessable};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
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
    row: plans::Model,
    parent: Option<usize>,
    tasks: Vec<tasks::Model>,
    components: Vec<components::Model>,
    milestones: Vec<milestones::Model>,
}

#[allow(clippy::too_many_lines)] // one linear pipeline: walk, load, convert
/// Capture the readable subtree under `root` into a snapshot; returns it with
/// the number of plans left out because the caller cannot read them.
async fn capture(
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
        let children = plans::Entity::find()
            .filter(plans::Column::ParentPid.is_in(parents))
            .filter(plans::Column::DeletedAt.is_null())
            .order_by_asc(plans::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
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
        r.tasks = tasks::Entity::find()
            .filter(tasks::Column::PlanPid.eq(pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .order_by_asc(tasks::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
        total_tasks += r.tasks.len();
        r.components = components::Entity::find()
            .filter(components::Column::PlanPid.eq(pid))
            .filter(components::Column::DeletedAt.is_null())
            .order_by_asc(components::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
        r.milestones = milestones::Entity::find()
            .filter(milestones::Column::PlanPid.eq(pid))
            .filter(milestones::Column::DeletedAt.is_null())
            .order_by_asc(milestones::Column::Due)
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
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
#[derive(Debug, Deserialize)]
struct SaveBody {
    name: String,
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
    let root = plans::Entity::find()
        .filter(plans::Column::Pid.eq(subject.pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let existing = plan_templates::Entity::find()
        .filter(plan_templates::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if existing
        .iter()
        .any(|t| t.name.to_lowercase() == name.to_lowercase())
    {
        return Err(conflict("a template with this name already exists"));
    }
    let (snapshot, omitted) = capture(&ctx, &caller, root).await?;
    let row = plan_templates::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(name),
        description: ActiveValue::set(b.description.clone()),
        snapshot: ActiveValue::set(serde_json::json!(snapshot)),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "plan_template_saved",
        caller.actor(),
        None,
    )
    .await
    .ok();
    format::json(serde_json::json!({
        "pid": row.pid,
        "name": row.name,
        "plans": snapshot.nodes.len(),
        "tasks": snapshot.nodes.iter().map(|n| n.tasks.len()).sum::<usize>(),
        "milestones": snapshot.nodes.iter().map(|n| n.milestones.len()).sum::<usize>(),
        "needs_start_date": snapshot.has_offsets,
        "omitted_unreadable": omitted,
    }))
}

/// `GET /api/plan-templates` — name, description and size, not the snapshot.
#[debug_handler]
async fn list_templates(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = plan_templates::Entity::find()
        .filter(plan_templates::Column::DeletedAt.is_null())
        .order_by_asc(plan_templates::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let items: Vec<serde_json::Value> = rows
        .iter()
        .map(|t| {
            let nodes = t.snapshot.get("nodes").and_then(|n| n.as_array()).map_or(0, Vec::len);
            serde_json::json!({
                "pid": t.pid, "name": t.name, "description": t.description,
                "plans": nodes,
                "needs_start_date": t.snapshot.get("has_offsets").and_then(serde_json::Value::as_bool).unwrap_or(false),
            })
        })
        .collect();
    format::json(items)
}

async fn find_template(ctx: &AppContext, pid: &str) -> Result<plan_templates::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    plan_templates::Entity::find()
        .filter(plan_templates::Column::Pid.eq(pid))
        .filter(plan_templates::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `GET /api/plan-templates/{pid}` — the full snapshot.
#[debug_handler]
async fn get_template(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    format::json(find_template(&ctx, &pid).await?)
}

/// `POST /api/plan-templates/{pid}/delete` (and `DELETE`) — soft-delete.
#[debug_handler]
async fn delete_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_template(&ctx, &pid).await?;
    let tpl_pid = row.pid;
    let mut active: plan_templates::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        tpl_pid,
        "plan_template_deleted",
        caller.actor(),
        None,
    )
    .await
    .ok();
    format::empty_json()
}

/// `POST /api/plans/from-template/{tpl}` body.
#[derive(Debug, Deserialize)]
struct InstantiateBody {
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
    let row = find_template(&ctx, &tpl).await?;
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
    let result = build(&ctx, &caller, &snap, &b, &date_of, &mut created).await;
    match result {
        Ok(summary) => {
            AuditModel::record(
                &ctx.db,
                row.pid,
                "plan_template_instantiated",
                caller.actor(),
                Some(serde_json::json!({ "root": created.first() })),
            )
            .await
            .ok();
            format::json(summary)
        }
        Err(e) => {
            // Roll back: soft-delete what was created, then report the failure.
            for pid in created.iter().rev() {
                if let Ok(m) = PlanModel::find_by_pid(&ctx.db, &pid.to_string()).await {
                    streaming::delete_and_emit(&ctx.db, m, caller.actor())
                        .await
                        .ok();
                }
            }
            Err(e)
        }
    }
}

#[allow(clippy::too_many_lines)] // one linear pipeline per plan: plan, workflow, components, tasks, milestones
async fn build(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    snap: &Snapshot,
    b: &InstantiateBody,
    date_of: &impl Fn(Option<i64>) -> Option<NaiveDate>,
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
        let model = streaming::create_and_emit(&ctx.db, &plan, caller.actor()).await?;
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
            let c = components::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan_pid),
                name: ActiveValue::set(name.clone()),
                lead_ref: ActiveValue::set(None),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
            comp_pids.push((name.clone(), c.pid));
        }
        let now = chrono::Utc::now();
        let mut task_pids: Vec<Uuid> = Vec::new();
        for t in &node.tasks {
            let tp = Uuid::new_v4();
            tasks::ActiveModel {
                pid: ActiveValue::set(tp),
                plan_pid: ActiveValue::set(plan_pid),
                sprint_pid: ActiveValue::set(None),
                title: ActiveValue::set(t.title.clone()),
                description: ActiveValue::set(t.description.clone()),
                status: ActiveValue::set(initial.clone()),
                assignee_ref: ActiveValue::set(None),
                points: ActiveValue::set(t.points),
                due_date: ActiveValue::set(date_of(t.due_offset_days)),
                component_pid: ActiveValue::set(
                    t.component
                        .as_ref()
                        .and_then(|n| comp_pids.iter().find(|(x, _)| x == n))
                        .map(|(_, p)| *p),
                ),
                parent_task_pid: ActiveValue::set(None),
                status_changed_at: ActiveValue::set(now.into()),
                flow_type: ActiveValue::set(t.flow_type.clone()),
                done_at: ActiveValue::set(
                    crate::workflow::is_done(&workflow, &initial).then(|| now.into()),
                ),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
            crate::tba::transition_row(
                tp,
                plan_pid,
                None,
                initial.clone(),
                now,
                caller.actor().map(ToString::to_string),
                None,
            )
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
            task_pids.push(tp);
            n_tasks += 1;
        }
        // Second pass: parents, now that every task has a pid.
        for t in &node.tasks {
            if let Some(parent_key) = t.parent
                && let (Some(child), Some(parent)) =
                    (task_pids.get(t.key), task_pids.get(parent_key))
            {
                tasks::Entity::update_many()
                    .col_expr(
                        tasks::Column::ParentTaskPid,
                        sea_orm::sea_query::Expr::value(*parent),
                    )
                    .filter(tasks::Column::Pid.eq(*child))
                    .exec(&ctx.db)
                    .await
                    .map_err(db_err)?;
            }
        }
        for m in &node.milestones {
            let Some(due) = date_of(Some(m.offset_days)) else {
                continue;
            };
            milestones::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan_pid),
                name: ActiveValue::set(m.name.clone()),
                due: ActiveValue::set(due),
                done: ActiveValue::set(false),
                done_at: ActiveValue::set(None),
                kind: ActiveValue::set(m.kind.clone()),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
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

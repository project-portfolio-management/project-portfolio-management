//! Blueprint sync (NC-12): show the plans made from a template what changed in it, and apply a change to one plan
//! with consent. Pure rules in [`crate::blueprint_sync`].
//!
//! * `POST /api/plan-templates/{pid}/revise`: replace the template's tasks with those of a plan tree (a new version);
//! * `GET /api/plan-templates/{pid}/plans`: the plans made from it, and how many changes each has waiting;
//! * `GET /api/plans/{pid}/template-diff`: what changed in the template since this plan was last brought up to date,
//!   each change marked when the plan's own people have edited or removed that task;
//! * `POST /api/plans/{pid}/template-sync`: accept changes by id. A change to a task the plan edited is refused
//!   unless it is also named in `overwrite_local`.

use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::plan_access::readable_plans;
use super::plan_templates::{capture, find_template};
use super::{db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::blueprint_sync::{self as rules, Kind, Local};
use crate::models::_entities::{plan_template_links, plan_templates, plans, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_templates::{Snapshot, TplTask};

/// Which of a plan's tasks came from which template task.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskLink {
    key: usize,
    task_pid: Uuid,
}

/// A link with its template and the template's tasks for this plan's node.
struct Loaded {
    link: plan_template_links::Model,
    template: plan_templates::Model,
    baseline: Vec<TplTask>,
    now: Vec<TplTask>,
    map: Vec<TaskLink>,
}

async fn load(ctx: &AppContext, plan_pid: Uuid) -> Result<Loaded> {
    let link = plan_template_links::Entity::find()
        .filter(plan_template_links::Column::PlanPid.eq(plan_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let template = find_template(ctx, &link.template_pid.to_string()).await?;
    let snapshot: Snapshot = serde_json::from_value(template.snapshot.clone())
        .map_err(|e| unprocessable(&format!("the template snapshot is unreadable: {e}")))?;
    let now = usize::try_from(link.node_key)
        .ok()
        .and_then(|k| snapshot.nodes.get(k))
        .map(|n| n.tasks.clone())
        .ok_or_else(|| unprocessable("the template no longer has this plan's place in its tree"))?;
    let baseline = serde_json::from_value(link.baseline.clone())
        .map_err(|e| unprocessable(&format!("the plan's baseline is unreadable: {e}")))?;
    let map = serde_json::from_value(link.task_map.clone())
        .map_err(|e| unprocessable(&format!("the plan's task map is unreadable: {e}")))?;
    Ok(Loaded {
        link,
        template,
        baseline,
        now,
        map,
    })
}

/// The plan's tasks the map names, still live.
async fn local_tasks(
    ctx: &AppContext,
    map: &[TaskLink],
) -> Result<std::collections::HashMap<Uuid, tasks::Model>> {
    let pids: Vec<Uuid> = map.iter().map(|m| m.task_pid).collect();
    let rows = tasks::Entity::find()
        .filter(tasks::Column::Pid.is_in(pids))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    Ok(rows.into_iter().map(|t| (t.pid, t)).collect())
}

fn local_of(
    map: &[TaskLink],
    rows: &std::collections::HashMap<Uuid, tasks::Model>,
    key: usize,
) -> Option<Local> {
    let pid = map.iter().find(|m| m.key == key)?.task_pid;
    rows.get(&pid).map(|t| Local {
        title: t.title.clone(),
        description: t.description.clone(),
        points: t.points,
    })
}

/// `POST /api/plan-templates/{pid}/revise` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReviseBody {
    /// The plan tree whose tasks the template should now hold.
    from_plan: Uuid,
}

/// `POST /api/plan-templates/{pid}/revise`: re-capture a plan tree as the template's next version. Plans made from
/// the template are not touched; each sees the difference through `template-diff`.
#[debug_handler]
async fn revise(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ReviseBody>,
) -> Result<Response> {
    let row = find_template(&ctx, &pid).await?;
    let old: Snapshot = serde_json::from_value(row.snapshot.clone())
        .map_err(|e| unprocessable(&format!("the template snapshot is unreadable: {e}")))?;
    let subject = resolve_subject(&ctx, &caller, "plan", &b.from_plan.to_string()).await?;
    let root = plans::Entity::find()
        .filter(plans::Column::Pid.eq(subject.pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let (captured, omitted) = capture(&ctx, &caller, root).await?;
    rules::same_shape(&old, &captured).map_err(|m| unprocessable(&m))?;
    let new = rules::rekey(&old, captured);
    let version = row.version + 1;
    let tpl_pid = row.pid;
    let mut active: plan_templates::ActiveModel = row.into();
    active.snapshot = ActiveValue::set(json!(new));
    active.version = ActiveValue::set(version);
    let txn = ctx.db.begin().await.map_err(db_err)?;
    active.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        tpl_pid,
        "plan_template_revised",
        caller.actor(),
        Some(json!({ "version": version, "omitted_unreadable": omitted })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({
        "pid": tpl_pid,
        "version": version,
        "omitted_unreadable": omitted,
    }))
}

/// `GET /api/plan-templates/{pid}/plans`: the plans made from this template that the caller can read, and how many
/// template changes each has waiting; a plan with none is up to date.
#[debug_handler]
async fn template_plans(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let template = find_template(&ctx, &pid).await?;
    let snapshot: Snapshot = serde_json::from_value(template.snapshot.clone())
        .map_err(|e| unprocessable(&format!("the template snapshot is unreadable: {e}")))?;
    let links = plan_template_links::Entity::find()
        .filter(plan_template_links::Column::TemplatePid.eq(template.pid))
        .order_by_asc(plan_template_links::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let pids: Vec<Uuid> = links.iter().map(|l| l.plan_pid).collect();
    let readable = readable_plans(&ctx, &caller, &pids).await;
    let names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let items: Vec<serde_json::Value> = links
        .iter()
        .filter(|l| readable.contains(&l.plan_pid))
        .map(|l| {
            let baseline: Vec<TplTask> =
                serde_json::from_value(l.baseline.clone()).unwrap_or_default();
            let now: Vec<TplTask> = usize::try_from(l.node_key)
                .ok()
                .and_then(|k| snapshot.nodes.get(k))
                .map(|n| n.tasks.clone())
                .unwrap_or_default();
            json!({
                "plan_pid": l.plan_pid,
                "name": names.get(&l.plan_pid),
                "synced_version": l.template_version,
                "template_version": template.version,
                "changes_waiting": rules::diff(&baseline, &now).len(),
            })
        })
        .collect();
    format::json(items)
}

/// `GET /api/plans/{pid}/template-diff`: the template's changes since this plan was last brought up to date, each
/// with whether the plan's own people have edited or removed that task. `404` for a plan not made from a template.
#[debug_handler]
async fn template_diff(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let l = load(&ctx, subject.pid).await?;
    let rows = local_tasks(&ctx, &l.map).await?;
    let changes: Vec<serde_json::Value> = rules::diff(&l.baseline, &l.now)
        .into_iter()
        .map(|c| {
            let local = local_of(&l.map, &rows, c.key);
            let edited = rules::locally_edited(&c, local.as_ref());
            json!({
                "id": c.id,
                "kind": c.kind,
                "title": c.title,
                "from": c.from,
                "to": c.to,
                "locally_edited": edited,
            })
        })
        .collect();
    format::json(json!({
        "template_pid": l.template.pid,
        "template_name": l.template.name,
        "synced_version": l.link.template_version,
        "template_version": l.template.version,
        "changes": changes,
    }))
}

/// `POST /api/plans/{pid}/template-sync` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SyncBody {
    /// The change ids to apply (`add:3`, `change:1`, `remove:2`); anything not named is left for later.
    accept: Vec<String>,
    /// Of those, the ones to apply even though the plan edited or removed the task. Without a name here, such a
    /// change is refused.
    #[serde(default)]
    overwrite_local: Vec<String>,
}

fn workflow_initial(def: &crate::workflow::WorkflowDef) -> String {
    def.states
        .iter()
        .find(|s| s.is_initial)
        .map_or("todo", |s| s.key.as_str())
        .to_string()
}

/// `POST /api/plans/{pid}/template-sync`: apply the named changes to this plan in one transaction. A change to a
/// task the plan edited is refused unless named in `overwrite_local`; refusals are listed, not errors.
#[debug_handler]
#[expect(clippy::too_many_lines)] // one linear pass: decide each change, apply it, move the baseline
async fn template_sync(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<SyncBody>,
) -> Result<Response> {
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let plan_pid = subject.pid;
    let l = load(&ctx, plan_pid).await?;
    let changes = rules::diff(&l.baseline, &l.now);
    let known: Vec<&str> = changes.iter().map(|c| c.id.as_str()).collect();
    let unknown: Vec<&String> = b
        .accept
        .iter()
        .chain(b.overwrite_local.iter())
        .filter(|id| !known.contains(&id.as_str()))
        .collect();
    if !unknown.is_empty() {
        return Err(unprocessable(&format!(
            "not a pending change of this plan: {}",
            unknown
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    if let Some(stray) = b.overwrite_local.iter().find(|id| !b.accept.contains(id)) {
        return Err(unprocessable(&format!(
            "{stray} is in overwrite_local but not in accept"
        )));
    }
    let rows = local_tasks(&ctx, &l.map).await?;
    let initial = {
        let def = super::workflow::in_force(&ctx, plan_pid, "task").await?;
        workflow_initial(&def)
    };

    let mut baseline = l.baseline.clone();
    let mut map = l.map.clone();
    let mut applied: Vec<String> = Vec::new();
    let mut overwritten: Vec<String> = Vec::new();
    let mut refused: Vec<serde_json::Value> = Vec::new();
    let now = chrono::Utc::now();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    for c in changes.iter().filter(|c| b.accept.contains(&c.id)) {
        let local = local_of(&l.map, &rows, c.key);
        let edited = rules::locally_edited(c, local.as_ref());
        if edited && !b.overwrite_local.contains(&c.id) {
            refused.push(json!({
                "id": c.id,
                "reason": "the plan has edited or removed this task; name the change in overwrite_local to apply it anyway",
            }));
            continue;
        }
        match (c.kind, &c.to) {
            (Kind::Add, Some(to)) => {
                let tp = Uuid::new_v4();
                insert_task(&txn, plan_pid, tp, to, &initial, now, caller.actor()).await?;
                map.push(TaskLink {
                    key: to.key,
                    task_pid: tp,
                });
                baseline.push(to.clone());
            }
            (Kind::Change, Some(to)) => {
                let Some(row) = local_row(&l.map, &rows, c.key) else {
                    refused.push(json!({
                        "id": c.id,
                        "reason": "the plan removed this task, so there is nothing to change",
                    }));
                    continue;
                };
                let mut active: tasks::ActiveModel = row.clone().into();
                active.title = ActiveValue::set(to.title.clone());
                active.description = ActiveValue::set(to.description.clone());
                active.points = ActiveValue::set(to.points);
                active.update(&txn).await.map_err(db_err)?;
                if let Some(slot) = baseline.iter_mut().find(|t| t.key == c.key) {
                    *slot = to.clone();
                }
            }
            (Kind::Remove, _) => {
                if let Some(row) = local_row(&l.map, &rows, c.key) {
                    let mut active: tasks::ActiveModel = row.clone().into();
                    active.deleted_at = ActiveValue::set(Some(now.into()));
                    active.update(&txn).await.map_err(db_err)?;
                }
                baseline.retain(|t| t.key != c.key);
                map.retain(|m| m.key != c.key);
            }
            _ => continue,
        }
        if edited {
            overwritten.push(c.id.clone());
        }
        applied.push(c.id.clone());
    }
    let remaining = rules::diff(&baseline, &l.now).len();
    let mut link: plan_template_links::ActiveModel = l.link.clone().into();
    link.baseline = ActiveValue::set(json!(baseline));
    link.task_map = ActiveValue::set(json!(map));
    if remaining == 0 {
        link.template_version = ActiveValue::set(l.template.version);
    }
    link.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan_pid,
        "plan_template_synced",
        caller.actor(),
        Some(json!({
            "template": l.template.pid,
            "applied": applied,
            "overwritten_local_edits": overwritten,
            "refused": refused.iter().filter_map(|r| r.get("id")).collect::<Vec<_>>(),
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({
        "applied": applied,
        "overwritten_local_edits": overwritten,
        "refused": refused,
        "remaining": remaining,
    }))
}

fn local_row<'a>(
    map: &[TaskLink],
    rows: &'a std::collections::HashMap<Uuid, tasks::Model>,
    key: usize,
) -> Option<&'a tasks::Model> {
    let pid = map.iter().find(|m| m.key == key)?.task_pid;
    rows.get(&pid)
}

/// One task at the workflow's initial state, with its first transition, inside the caller's transaction.
async fn insert_task(
    txn: &sea_orm::DatabaseTransaction,
    plan_pid: Uuid,
    pid: Uuid,
    t: &TplTask,
    initial: &str,
    now: chrono::DateTime<chrono::Utc>,
    actor: Option<&str>,
) -> Result<()> {
    tasks::ActiveModel {
        pid: ActiveValue::set(pid),
        plan_pid: ActiveValue::set(plan_pid),
        sprint_pid: ActiveValue::set(None),
        title: ActiveValue::set(t.title.clone()),
        description: ActiveValue::set(t.description.clone()),
        status: ActiveValue::set(initial.to_string()),
        assignee_ref: ActiveValue::set(None),
        points: ActiveValue::set(t.points),
        due_date: ActiveValue::set(None),
        component_pid: ActiveValue::set(None),
        parent_task_pid: ActiveValue::set(None),
        status_changed_at: ActiveValue::set(now.into()),
        flow_type: ActiveValue::set(t.flow_type.clone()),
        done_at: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(txn)
    .await
    .map_err(db_err)?;
    crate::tba::transition_row(
        pid,
        plan_pid,
        None,
        initial.to_string(),
        now,
        actor.map(ToString::to_string),
        None,
    )
    .insert(txn)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// The blueprint-sync routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plan-templates/{pid}/revise", post(revise))
        .add("/plan-templates/{pid}/plans", get(template_plans))
        .add("/plans/{pid}/template-diff", get(template_diff))
        .add("/plans/{pid}/template-sync", post(template_sync))
}

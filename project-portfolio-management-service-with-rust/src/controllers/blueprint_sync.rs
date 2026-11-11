//! Blueprint sync (NC-12): show the plans made from a template what changed in it, and apply a change to one plan
//! with consent. Pure rules in [`crate::blueprint_sync`].
//!
//! * `POST /api/plan-templates/{pid}/revise`: replace the template's tasks with those of a plan tree (a new version);
//! * `GET /api/plan-templates/{pid}/plans`: the plans made from it, and how many changes each has waiting;
//! * `GET /api/plans/{pid}/template-diff`: what changed in the template since this plan was last brought up to date,
//!   each change marked when the plan's own people have edited or removed that task;
//! * `POST /api/plans/{pid}/template-sync`: accept changes by id. A change to a task the plan edited is refused
//!   unless it is also named in `overwrite_local`.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::plan_access::readable_plans;
use super::plan_templates::{capture, find_template, find_visible_template};
use super::unprocessable;
use crate::auth::MaybeAuthUser;
use crate::blueprint_sync::{self as rules, Kind, Local};
use crate::models::_entities::{plan_template_links, plan_templates, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::plans::Model as PlanModel;
use crate::plan_templates::{Snapshot, TplTask};

/// Which of a plan's tasks came from which template task.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskLink {
    /// Index of the template task within the template node's task list.
    key: usize,
    /// The plan's own task created from that template task.
    task_pid: Uuid,
}

/// A link with its template and the template's tasks for this plan's node.
struct Loaded {
    /// The stored link between the plan and its template.
    link: plan_template_links::Model,
    /// The template row the plan was made from.
    template: plan_templates::Model,
    /// The template's tasks as of the last time this plan was brought up to date; diffs are taken against it.
    baseline: Vec<TplTask>,
    /// The template's tasks now, for this plan's node.
    now: Vec<TplTask>,
    /// Which plan task came from which template task.
    map: Vec<TaskLink>,
}

/// Load a plan's template link, the template, and the baseline, current and mapped tasks; `404` when the plan has no link, `422` when stored JSON or the plan's node in the template cannot be read.
async fn load(ctx: &AppContext, plan_pid: Uuid) -> Result<Loaded> {
    let link: plan_template_links::Model =
        sqlx::query_as("SELECT * FROM plan_template_links WHERE plan_pid = $1 LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
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
    let rows: Vec<tasks::Model> =
        sqlx::query_as("SELECT * FROM tasks WHERE pid = ANY($1) AND deleted_at IS NULL")
            .bind(pids)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    Ok(rows.into_iter().map(|t| (t.pid, t)).collect())
}

/// The plan's live task for a template task key, reduced to the fields a sync compares; `None` when the task was removed or never mapped, which marks a local removal.
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
    let row = find_visible_template(&ctx, &caller, &pid).await?;
    super::plan_templates::refuse_starter(&row)?;
    let old: Snapshot = serde_json::from_value(row.snapshot.clone())
        .map_err(|e| unprocessable(&format!("the template snapshot is unreadable: {e}")))?;
    let subject = resolve_subject(&ctx, &caller, "plan", &b.from_plan.to_string()).await?;
    let root = PlanModel::find_by_pid(&ctx, &subject.pid.to_string()).await?;
    let (captured, omitted) = capture(&ctx, &caller, root).await?;
    rules::same_shape(&old, &captured).map_err(|m| unprocessable(&m))?;
    let new = rules::rekey(&old, captured);
    let version = row.version + 1;
    let tpl_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, as the update of the row read above was before SQ-1; no row back is an error, as it was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE plan_templates SET source_plan_pid = $1, snapshot = $2, version = $3 WHERE id = $4 RETURNING id",
    )
    .bind(subject.pid)
    .bind(json!(new))
    .bind(version)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        tpl_pid,
        "plan_template_revised",
        caller.actor(),
        Some(json!({ "version": version, "omitted_unreadable": omitted })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    let template = find_visible_template(&ctx, &caller, &pid).await?;
    let snapshot: Snapshot = serde_json::from_value(template.snapshot.clone())
        .map_err(|e| unprocessable(&format!("the template snapshot is unreadable: {e}")))?;
    let links: Vec<plan_template_links::Model> =
        sqlx::query_as("SELECT * FROM plan_template_links WHERE template_pid = $1 ORDER BY id")
            .bind(template.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let pids: Vec<Uuid> = links.iter().map(|l| l.plan_pid).collect();
    let readable = readable_plans(&ctx, &caller, &pids).await;
    // Every plan named, soft-deleted ones too, as before; only the pid and the name are read.
    let names: std::collections::HashMap<Uuid, String> =
        sqlx::query_as::<_, (Uuid, String)>("SELECT pid, name FROM plans WHERE pid = ANY($1)")
            .bind(&pids)
            .fetch_all(crate::db::pool(&ctx))
            .await?
            .into_iter()
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

/// The key of the workflow's initial state, or `todo` when none is flagged initial.
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
    let mut tx = crate::db::begin(&ctx).await?;
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
                insert_task(&mut tx, plan_pid, tp, to, &initial, now, caller.actor()).await?;
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
                // By id, as the update of the row read above was before SQ-1; no row back is an error, as it was.
                sqlx::query_scalar::<_, i32>(
                    "UPDATE tasks SET title = $1, description = $2, points = $3 WHERE id = $4 RETURNING id",
                )
                .bind(&to.title)
                .bind(&to.description)
                .bind(to.points)
                .bind(row.id)
                .fetch_one(tx.conn())
                .await?;
                if let Some(slot) = baseline.iter_mut().find(|t| t.key == c.key) {
                    *slot = to.clone();
                }
            }
            (Kind::Remove, _) => {
                if let Some(row) = local_row(&l.map, &rows, c.key) {
                    sqlx::query_scalar::<_, i32>(
                        "UPDATE tasks SET deleted_at = $1 WHERE id = $2 RETURNING id",
                    )
                    .bind(chrono::DateTime::<chrono::FixedOffset>::from(now))
                    .bind(row.id)
                    .fetch_one(tx.conn())
                    .await?;
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
    // The version moves only when nothing is left waiting; the update names it only then, as before SQ-1.
    let link = if remaining == 0 {
        sqlx::query_scalar::<_, i64>(
            "UPDATE plan_template_links SET baseline = $1, task_map = $2, template_version = $3 WHERE id = $4
             RETURNING id",
        )
        .bind(json!(baseline))
        .bind(json!(map))
        .bind(l.template.version)
        .bind(l.link.id)
    } else {
        sqlx::query_scalar::<_, i64>(
            "UPDATE plan_template_links SET baseline = $1, task_map = $2 WHERE id = $3 RETURNING id",
        )
        .bind(json!(baseline))
        .bind(json!(map))
        .bind(l.link.id)
    };
    link.fetch_one(tx.conn()).await?;
    AuditModel::record_on(
        tx.conn(),
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
    tx.commit().await?;
    format::json(json!({
        "applied": applied,
        "overwritten_local_edits": overwritten,
        "refused": refused,
        "remaining": remaining,
    }))
}

/// The plan's live task row for a template task key, if it is still there.
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
    tx: &mut crate::db::Tx,
    plan_pid: Uuid,
    pid: Uuid,
    t: &TplTask,
    initial: &str,
    now: chrono::DateTime<chrono::Utc>,
    actor: Option<&str>,
) -> Result<()> {
    // The columns the insert set before SQ-1, no more.
    sqlx::query(
        "INSERT INTO tasks
             (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points, due_date, component_pid,
              parent_task_pid, status_changed_at, flow_type, done_at, deleted_at)
         VALUES ($1, $2, NULL, $3, $4, $5, NULL, $6, NULL, NULL, NULL, $7, $8, NULL, NULL)",
    )
    .bind(pid)
    .bind(plan_pid)
    .bind(&t.title)
    .bind(&t.description)
    .bind(initial)
    .bind(t.points)
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(now))
    .bind(&t.flow_type)
    .execute(tx.conn())
    .await?;
    crate::tba::insert_transition_on(
        tx,
        crate::tba::NewTransition {
            task_pid: pid,
            plan_pid,
            from_status: None,
            to_status: initial,
            at: now,
            actor_ref: actor,
            assignee_ref: None,
            blocked_reason: None,
        },
    )
    .await?;
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

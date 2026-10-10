//! Subtasks and the derived roll-up (RM-5). A task may name a parent in its
//! own plan; a parent's due date, estimate, progress and spent time are
//! **derived on read** from its descendants ([`crate::task_rollup`]) and never
//! stored. Parent links are validated here (same plan, no cycle, bounded
//! depth) and on the task create/update path.

use std::collections::BTreeMap;

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use uuid::Uuid;

use super::governance::find_item;
use crate::models::_entities::{tasks, time_entries};
use crate::task_rollup::{self as rules, Fact};

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

/// The plan's live tasks.
async fn plan_tasks(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<tasks::Model>> {
    tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan_pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// Validate a requested parent for a task: it must be a live task of the
/// **same plan**, not the task itself, not one of its own descendants, and
/// within the depth cap. `child` is `None` on create.
pub(crate) async fn check_parent(
    ctx: &AppContext,
    plan_pid: Uuid,
    child: Option<Uuid>,
    parent: Option<Uuid>,
) -> Result<()> {
    let Some(parent) = parent else { return Ok(()) };
    let all = plan_tasks(ctx, plan_pid).await?;
    if !all.iter().any(|t| t.pid == parent) {
        return Err(unprocessable(
            "parent_task_pid must name a live task of this plan",
        ));
    }
    let links: BTreeMap<Uuid, Uuid> = all
        .iter()
        .filter_map(|t| t.parent_task_pid.map(|p| (t.pid, p)))
        .collect();
    // On create the child has no pid yet: use a fresh one (no descendants).
    rules::parent_allowed(&links, child.unwrap_or_else(Uuid::new_v4), parent)
        .map_err(|m| unprocessable(&m))
}

/// Every live descendant of `root` within the plan, bounded.
fn descendants(all: &[tasks::Model], root: Uuid) -> Vec<&tasks::Model> {
    let mut out: Vec<&tasks::Model> = Vec::new();
    let mut frontier = vec![root];
    while let Some(n) = frontier.pop() {
        for t in all.iter().filter(|t| t.parent_task_pid == Some(n)) {
            if out.iter().any(|o| o.pid == t.pid) {
                continue;
            }
            out.push(t);
            frontier.push(t.pid);
            if out.len() >= 1_000 {
                return out;
            }
        }
    }
    out
}

/// `GET /api/plans/{pid}/tasks/{t_pid}/rollup` — the parent's derived view.
#[debug_handler]
async fn rollup(
    State(ctx): State<AppContext>,
    Path((pid, t_pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let t_pid = Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?;
    let all = plan_tasks(&ctx, plan.pid).await?;
    if !all.iter().any(|t| t.pid == t_pid) {
        return Err(Error::NotFound);
    }
    let kids = descendants(&all, t_pid);
    let mut pids: Vec<Uuid> = kids.iter().map(|t| t.pid).collect();
    pids.push(t_pid);
    let spent: Vec<(Option<Uuid>, i64)> = time_entries::Entity::find()
        .filter(time_entries::Column::TaskPid.is_in(pids))
        .filter(time_entries::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|e| (e.task_pid, e.minutes))
        .collect();
    let minutes_of = |p: Uuid| -> i64 {
        spent
            .iter()
            .filter(|(t, _)| *t == Some(p))
            .map(|(_, m)| *m)
            .sum()
    };
    let facts: Vec<Fact> = kids
        .iter()
        .map(|t| Fact {
            pid: t.pid,
            due: t.due_date,
            points: t.points,
            done: t.done_at.is_some(),
            spent_minutes: minutes_of(t.pid),
        })
        .collect();
    format::json(serde_json::json!({
        "task_pid": t_pid,
        "rollup": rules::rollup(&facts, minutes_of(t_pid)),
    }))
}

/// The subtask routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks/{t_pid}/rollup", get(rollup))
}

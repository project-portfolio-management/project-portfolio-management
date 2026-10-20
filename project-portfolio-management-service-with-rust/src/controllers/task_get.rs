//! `GET /api/plans/{pid}/tasks/{t_pid}` with `?include=` (RM-12b): one task, and,
//! when asked, the sub-resources that belong to it in the same response instead of
//! one request each. Without `include` the answer is the bare task, as the list
//! shows it.
//!
//! Each included set is the same live rows its own route would return (capped), and
//! the same gates apply: a sub-resource whose **module is off** for the plan is
//! omitted (reason `module_disabled`), and one that names **people** (time entries,
//! RACI rows, who moved the task) is omitted for a caller who may not manage people
//! (reason `masked`). An omitted name is *listed*, never silently missing; an unknown
//! name is a `422` that lists the allowed ones.

use super::db_err;
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;

use super::portfolio_resources::may_manage_people;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    raci_assignments, task_relations, task_transitions, tasks, time_entries,
};

/// Rows embedded per sub-resource.
const CAP: u64 = 200;

/// Includable names, the module each belongs to, and whether it names people.
pub const INCLUDABLE: &[(&str, &str, bool)] = &[
    ("children", "tasks", false),
    ("relations", "tasks", false),
    ("transitions", "tasks", true),
    ("raci", "tasks", true),
    ("time_entries", "time", true),
];

/// Parse `include=a,b` into known names; an unknown one lists what is allowed.
///
/// # Errors
///
/// A message naming each unknown name and the allowed ones.
pub fn parse(raw: &str) -> std::result::Result<Vec<&'static str>, String> {
    let mut out: Vec<&'static str> = Vec::new();
    let mut bad: Vec<String> = Vec::new();
    for part in raw.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        match INCLUDABLE.iter().find(|(n, _, _)| *n == part) {
            Some((n, _, _)) => {
                if !out.contains(n) {
                    out.push(n);
                }
            }
            None => bad.push(part.to_string()),
        }
    }
    if bad.is_empty() {
        Ok(out)
    } else {
        let allowed: Vec<&str> = INCLUDABLE.iter().map(|(n, _, _)| *n).collect();
        Err(format!("unknown include {bad:?} (allowed: {allowed:?})"))
    }
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    include: Option<String>,
}

/// `GET /api/plans/{pid}/tasks/{t_pid}[?include=children,relations,transitions,raci,time_entries]`.
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn get_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Query(q): Query<Params>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = super::engineering::find_task(&ctx, plan.pid, &t_pid).await?;
    let view = super::engineering::task_view(&task);
    let Some(raw) = q.include.as_deref().filter(|r| !r.trim().is_empty()) else {
        return format::json(view);
    };
    let names = parse(raw).map_err(|m| {
        Error::CustomError(
            StatusCode::UNPROCESSABLE_ENTITY,
            ErrorDetail::new("unprocessable", m),
        )
    })?;
    let people = may_manage_people(&caller);
    let (modules, _) = super::modules::modules_in_force(&ctx.db, plan.pid)
        .await
        .map_err(db_err)?;
    let mut included = serde_json::Map::new();
    let mut omitted = Vec::new();
    for name in names {
        let (_, module, names_people) = INCLUDABLE
            .iter()
            .find(|(n, _, _)| *n == name)
            .copied()
            .unwrap_or((name, "tasks", false));
        if names_people && !people {
            omitted.push(serde_json::json!({ "name": name, "reason": "masked" }));
            continue;
        }
        if !modules.iter().any(|m| m == module) {
            omitted.push(serde_json::json!({ "name": name, "reason": "module_disabled" }));
            continue;
        }
        let rows: serde_json::Value = match name {
            "children" => serde_json::to_value(
                tasks::Entity::find()
                    .filter(tasks::Column::ParentTaskPid.eq(task.pid))
                    .filter(tasks::Column::DeletedAt.is_null())
                    .order_by_asc(tasks::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
                    .iter()
                    .map(super::engineering::task_view)
                    .collect::<Vec<_>>(),
            ),
            "relations" => {
                let found = task_relations::Entity::find()
                    .filter(task_relations::Column::DeletedAt.is_null())
                    .filter(
                        sea_orm::Condition::any()
                            .add(task_relations::Column::FromTaskPid.eq(task.pid))
                            .add(task_relations::Column::ToTaskPid.eq(task.pid)),
                    )
                    .order_by_asc(task_relations::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?;
                // A relation to a task in a plan the caller cannot read is not shown, not even as a pid.
                let ends: Vec<Uuid> = found.iter().flat_map(|r| [r.from_task_pid, r.to_task_pid]).collect();
                let ok = super::cpm::readable_endpoints(&ctx, &caller, &ends).await?;
                serde_json::to_value(
                    found
                        .iter()
                        .filter(|r| ok.contains(&r.from_task_pid) && ok.contains(&r.to_task_pid))
                        .map(|r| serde_json::json!({ "pid": r.pid, "from": r.from_task_pid, "to": r.to_task_pid, "kind": r.kind }))
                        .collect::<Vec<_>>(),
                )
            }
            "transitions" => serde_json::to_value(
                task_transitions::Entity::find()
                    .filter(task_transitions::Column::TaskPid.eq(task.pid))
                    .order_by_asc(task_transitions::Column::At)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
                    .iter()
                    .map(|t| serde_json::json!({ "from": t.from_status, "to": t.to_status, "at": t.at, "actor_ref": t.actor_ref }))
                    .collect::<Vec<_>>(),
            ),
            "raci" => serde_json::to_value(
                raci_assignments::Entity::find()
                    .filter(raci_assignments::Column::TaskPid.eq(task.pid))
                    .filter(raci_assignments::Column::DeletedAt.is_null())
                    .order_by_asc(raci_assignments::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
                    .iter()
                    .map(|r| serde_json::json!({ "person_ref": r.person_ref, "role": r.role }))
                    .collect::<Vec<_>>(),
            ),
            _ => serde_json::to_value(
                time_entries::Entity::find()
                    .filter(time_entries::Column::TaskPid.eq(task.pid))
                    .filter(time_entries::Column::DeletedAt.is_null())
                    .order_by_asc(time_entries::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?,
            ),
        }
        .unwrap_or_default();
        included.insert(name.to_string(), rows);
    }
    format::json(serde_json::json!({ "task": view, "included": included, "omitted": omitted }))
}

/// The task read route.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks/{t_pid}", get(get_task))
}

//! The open tasks of the people in a role (the resource breakdown's drill-down):
//! `GET /api/resource-plan/role-tasks?role=<role>`.
//!
//! A role's people come from `resource_roles` (the same list `people_by_role` shows). Only a caller who may
//! manage people can ask, because the answer is a list of what named people are working on. A task in a plan
//! the caller cannot read (private, or past a stage gate they cannot read) is left out and counted in
//! `omitted_unreadable`, never shown masked. The `unclassified` row has no list of people and is refused.
//! Skills have no people at all, so there is no skill drill-down.

use super::db_err;
use std::collections::{BTreeMap, HashMap};

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::portfolio_resources::may_manage_people;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{plans, resource_roles, tasks};

/// Open tasks read per request; a role with more is told so.
const ROW_CAP: u64 = 500;

fn refuse(status: StatusCode, code: &str, message: &str) -> Error {
    Error::CustomError(status, ErrorDetail::new(code, message))
}

/// The plans of `rows` the caller can read, with their names: not hidden (private) and readable at the
/// plan's stage.
async fn readable_plans(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    rows: &[tasks::Model],
) -> Result<HashMap<Uuid, String>> {
    let mut plan_pids: Vec<Uuid> = rows.iter().map(|t| t.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let plan_rows = if plan_pids.is_empty() {
        Vec::new()
    } else {
        plans::Entity::find()
            .filter(plans::Column::Pid.is_in(plan_pids))
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
    };
    let hidden = super::plan_access::hidden_plans(
        ctx,
        &plan_rows.iter().map(|p| p.pid).collect::<Vec<_>>(),
        caller,
    )
    .await;
    Ok(plan_rows
        .into_iter()
        .filter(|p| !hidden.contains(&p.pid))
        .filter(|p| {
            crate::auth::authorize_record(
                caller,
                authentication_verifier::Action::Read,
                &crate::auth::plan_resource_attrs(p.stage.as_deref()),
            )
            .is_ok()
        })
        .map(|p| (p.pid, p.name))
        .collect())
}

/// `GET /api/resource-plan/role-tasks` query.
#[derive(Debug, Deserialize)]
struct Params {
    role: String,
}

/// `GET /api/resource-plan/role-tasks?role=`.
#[debug_handler]
async fn role_tasks(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<Params>,
) -> Result<Response> {
    if !may_manage_people(&caller) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "only a caller who may manage people can see what a role's people are working on",
        ));
    }
    if q.role == crate::portfolio_resources::UNCLASSIFIED {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable",
            "the unclassified row has no list of people; give a person a role first (PUT /api/resource-roles)",
        ));
    }
    let people: Vec<String> = resource_roles::Entity::find()
        .filter(resource_roles::Column::Role.eq(q.role.as_str()))
        .order_by_asc(resource_roles::Column::PersonRef)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| r.person_ref)
        .collect();
    let mut rows = if people.is_empty() {
        Vec::new()
    } else {
        tasks::Entity::find()
            .filter(tasks::Column::AssigneeRef.is_in(people.clone()))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::DoneAt.is_null())
            .order_by_asc(tasks::Column::Id)
            .limit(ROW_CAP + 1)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
    };
    let truncated = rows.len() as u64 > ROW_CAP;
    rows.truncate(usize::try_from(ROW_CAP).unwrap_or(usize::MAX));

    let readable = readable_plans(&ctx, &caller, &rows).await?;

    let mut by_person: BTreeMap<&str, Vec<serde_json::Value>> =
        people.iter().map(|p| (p.as_str(), Vec::new())).collect();
    let mut shown = 0_usize;
    for t in &rows {
        let (Some(plan_name), Some(who)) = (readable.get(&t.plan_pid), t.assignee_ref.as_deref())
        else {
            continue;
        };
        if let Some(list) = by_person.get_mut(who) {
            list.push(serde_json::json!({
                "pid": t.pid,
                "title": t.title,
                "status": t.status,
                "due_date": t.due_date,
                "plan_pid": t.plan_pid,
                "plan_name": plan_name,
            }));
            shown += 1;
        }
    }
    format::json(serde_json::json!({
        "role": q.role,
        "people": by_person
            .into_iter()
            .map(|(person_ref, tasks)| serde_json::json!({ "person_ref": person_ref, "tasks": tasks }))
            .collect::<Vec<_>>(),
        "tasks": shown,
        "omitted_unreadable": rows.len() - shown,
        "truncated": truncated,
        "note": "Open (not done) tasks assigned to the people whose primary role this is, in plans the caller can read; tasks in other plans are counted in omitted_unreadable and not shown. At most 500 tasks are read.",
    }))
}

/// The role drill-down route.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/resource-plan/role-tasks", get(role_tasks))
}

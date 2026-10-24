//! The RACI matrix: who is Responsible, Accountable, Consulted and Informed on
//! each task of a plan. Pure rules in [`crate::raci`].
//!
//! **A row is replaced as a whole** (`PUT …/tasks/{pid}/raci`): you send the people
//! and their letters and that becomes the row, atomically. Two Accountable people
//! on one task are **refused** (the database also enforces it, so two simultaneous
//! requests cannot both win); a row with no Accountable or no Responsible is
//! **allowed and reported**, because that is how a plan looks while it is being
//! set up. The old assignments are soft-deleted, so the history is kept.
//!
//! People named Responsible, Accountable or Informed are **auto-subscribed** to the
//! task (RM-3), unless they have switched `auto_watch` off. Removing them from the
//! row does **not** unsubscribe them (they may be watching for other reasons).

use super::db_err;
use std::collections::BTreeMap;

use axum::http::header;
use axum::response::IntoResponse;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{raci_assignments, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::raci::{self as rules, Entry};

async fn live_tasks(ctx: &AppContext, plan: Uuid) -> Result<Vec<tasks::Model>> {
    tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .order_by_asc(tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

async fn live_assignments(ctx: &AppContext, plan: Uuid) -> Result<Vec<raci_assignments::Model>> {
    raci_assignments::Entity::find()
        .filter(raci_assignments::Column::PlanPid.eq(plan))
        .filter(raci_assignments::Column::DeletedAt.is_null())
        .order_by_asc(raci_assignments::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

#[derive(Debug, Deserialize)]
struct MatrixParams {
    /// `csv` for a spreadsheet; JSON otherwise.
    #[serde(default)]
    format: Option<String>,
}

/// `GET /api/plans/{pid}/raci[?format=csv]` — the matrix: people across, tasks
/// down, letters in the cells, what is missing on each row, and each person's load.
#[debug_handler]
async fn matrix(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(p): Query<MatrixParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let tasks = live_tasks(&ctx, plan.pid).await?;
    let pairs: Vec<(Uuid, String)> = tasks.iter().map(|t| (t.pid, t.title.clone())).collect();
    let assigned: Vec<(Uuid, String, char)> = live_assignments(&ctx, plan.pid)
        .await?
        .into_iter()
        .filter_map(|a| Some((a.task_pid, a.person_ref, a.role.chars().next()?)))
        .collect();
    let m = rules::matrix(&pairs, &assigned);
    if p.format.as_deref() == Some("csv") {
        return Ok((
            [
                (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"raci.csv\"",
                ),
            ],
            rules::to_csv(&m),
        )
            .into_response());
    }
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "people": m.people,
        "rows": m.rows,
        "summary": m.summary,
        "workload": m.workload,
        "legend": { "R": "Responsible: does the work", "A": "Accountable: answers for it and signs it off (one per task)",
                    "C": "Consulted: asked before", "I": "Informed: told after" },
    }))
}

#[derive(Debug, Deserialize)]
struct Party {
    person_ref: String,
    /// Letters, e.g. `"AR"` (case-insensitive).
    roles: String,
}

#[derive(Debug, Deserialize)]
struct RowBody {
    assignments: Vec<Party>,
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/raci` — replace a task's RACI row.
#[expect(clippy::too_many_lines)]
#[debug_handler]
async fn set_row(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<RowBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let t_pid = super::parse_pid(&t_pid)?;
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(t_pid))
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut problems = Vec::new();
    let mut entries = Vec::new();
    for party in &b.assignments {
        if !super::governance::valid_ref(&party.person_ref, &["person", "worker"]) {
            problems.push(format!(
                "{:?} is not a person: or worker: URN",
                party.person_ref
            ));
        }
        match rules::parse_roles(&party.roles) {
            Some(roles) => entries.push(Entry {
                person_ref: party.person_ref.clone(),
                roles,
            }),
            None => problems.push(format!(
                "{:?} has a letter that is not R, A, C or I",
                party.roles
            )),
        }
    }
    if problems.is_empty() {
        problems.extend(rules::refusals(&entries));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let before: BTreeMap<String, String> = {
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        for a in raci_assignments::Entity::find()
            .filter(raci_assignments::Column::TaskPid.eq(task.pid))
            .filter(raci_assignments::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
        {
            m.entry(a.person_ref).or_default().push_str(&a.role);
        }
        m
    };
    let now = chrono::Utc::now();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    raci_assignments::Entity::update_many()
        .col_expr(
            raci_assignments::Column::DeletedAt,
            sea_orm::sea_query::Expr::value(now),
        )
        .filter(raci_assignments::Column::TaskPid.eq(task.pid))
        .filter(raci_assignments::Column::DeletedAt.is_null())
        .exec(&txn)
        .await
        .map_err(db_err)?;
    for e in &entries {
        for r in &e.roles {
            raci_assignments::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                task_pid: ActiveValue::set(task.pid),
                person_ref: ActiveValue::set(e.person_ref.clone()),
                role: ActiveValue::set(r.to_string()),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(|err| {
                if err.to_string().contains("raci_assignments_one_accountable") {
                    unprocessable("a task has one Accountable person; another request set one at the same moment")
                } else {
                    db_err(err)
                }
            })?;
        }
    }
    let after: BTreeMap<String, String> = entries
        .iter()
        .map(|e| (e.person_ref.clone(), e.roles.iter().collect()))
        .collect();
    AuditModel::record(
        &txn,
        task.pid,
        "raci_set",
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan.pid, "before": before, "after": after })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    // Responsible, Accountable and Informed people want to hear about the task.
    for e in &entries {
        if e.roles.iter().any(|r| matches!(r, 'R' | 'A' | 'I'))
            && let Some(sub) = e.person_ref.split_once(':').map(|(_, id)| id)
        {
            super::watchers::ensure_watching(&ctx.db, "task", task.pid, sub).await;
        }
    }
    crate::live::publish(
        plan.pid,
        "raci_updated",
        serde_json::json!({ "task_pid": task.pid }),
    );
    format::json(serde_json::json!({
        "task_pid": task.pid,
        "cells": after,
        "findings": rules::findings(&entries),
    }))
}

/// The RACI routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/raci", get(matrix))
        .add("/plans/{pid}/tasks/{t_pid}/raci", put(set_row))
}

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

use std::collections::BTreeMap;

use crate::prelude::*;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{raci_assignments, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::raci::{self as rules, Entry};

/// Live (not soft-deleted) tasks of a plan in creation order; these are the rows of the RACI matrix.
async fn live_tasks(ctx: &AppContext, plan: Uuid) -> Result<Vec<tasks::Model>> {
    Ok(
        sqlx::query_as(
            "SELECT * FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
        )
        .bind(plan)
        .fetch_all(crate::db::pool(ctx))
        .await?,
    )
}

/// Live RACI assignments of a plan in creation order; one row per person and role letter on a task.
async fn live_assignments(ctx: &AppContext, plan: Uuid) -> Result<Vec<raci_assignments::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM raci_assignments WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(plan)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Query parameters for the RACI matrix.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MatrixParams {
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

/// One person's roles on a task in a RACI row replacement.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Party {
    /// The person as a reference (URN), never a bare name.
    person_ref: String,
    /// Letters, e.g. `"AR"` (case-insensitive).
    roles: String,
}

/// Request body that replaces a task's whole RACI row.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RowBody {
    /// The complete new row; anything not listed is removed. An empty list clears the row.
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
    let task: tasks::Model = sqlx::query_as(
        "SELECT * FROM tasks WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(t_pid)
    .bind(plan.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
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
        let rows: Vec<raci_assignments::Model> = sqlx::query_as(
            "SELECT * FROM raci_assignments WHERE task_pid = $1 AND deleted_at IS NULL",
        )
        .bind(task.pid)
        .fetch_all(crate::db::pool(&ctx))
        .await?;
        for a in rows {
            m.entry(a.person_ref).or_default().push_str(&a.role);
        }
        m
    };
    let now = chrono::Utc::now();
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "UPDATE raci_assignments SET deleted_at = $1 WHERE task_pid = $2 AND deleted_at IS NULL",
    )
    .bind(now)
    .bind(task.pid)
    .execute(tx.conn())
    .await?;
    for e in &entries {
        for r in &e.roles {
            sqlx::query(
                "INSERT INTO raci_assignments (pid, plan_pid, task_pid, person_ref, role, deleted_at)
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(Uuid::new_v4())
            .bind(plan.pid)
            .bind(task.pid)
            .bind(&e.person_ref)
            .bind(r.to_string())
            .bind(None::<chrono::DateTime<chrono::Utc>>)
            .execute(tx.conn())
            .await
            .map_err(|err| {
                if err.to_string().contains("raci_assignments_one_accountable") {
                    unprocessable("a task has one Accountable person; another request set one at the same moment")
                } else {
                    Error::from(err)
                }
            })?;
        }
    }
    let after: BTreeMap<String, String> = entries
        .iter()
        .map(|e| (e.person_ref.clone(), e.roles.iter().collect()))
        .collect();
    AuditModel::record_on(
        tx.conn(),
        task.pid,
        "raci_set",
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan.pid, "before": before, "after": after })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    // Responsible, Accountable and Informed people want to hear about the task.
    for e in &entries {
        if e.roles.iter().any(|r| matches!(r, 'R' | 'A' | 'I'))
            && let Some(sub) = e.person_ref.split_once(':').map(|(_, id)| id)
        {
            super::watchers::ensure_watching(&ctx, "task", task.pid, sub).await;
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

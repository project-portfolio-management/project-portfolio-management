//! Jira and Asana CSV import into an **existing plan** (T-28n). The reader is
//! [`crate::tracker_import`].
//!
//! `POST /api/plans/{pid}/import/jira` and `…/import/asana` take the CSV export as the
//! body. `dry_run=true` (the default) reports without writing; `dry_run=false` applies it
//! in one transaction. **Idempotent:** an item's key is stored as `external_ref`
//! (`jira:PROJ-1`, `asana:<task id>`, unique per plan), so importing the file again updates
//! titles, deadlines, start dates and parents in place and never duplicates; a task's
//! *status* and description are left alone on re-import. **Nothing is silently dropped:**
//! every column, row, link, date and parent that did not come across is listed with the
//! reason. Assignees are names, not identities: they are kept in the description.

use super::db_err;
use std::collections::{HashMap, HashSet};

use axum::body::Bytes;
use axum::extract::DefaultBodyLimit;
use loco_rs::prelude::*;
use sea_orm::TransactionTrait;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{plan_dependencies, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::tracker_import::{self as reader, Bucket, Item, Source};

#[derive(Debug, Deserialize)]
struct ImportParams {
    /// `true` (default) reports without writing.
    #[serde(default)]
    dry_run: Option<bool>,
}

/// The workflow states an item's bucket maps to.
struct States {
    initial: String,
    active: Option<String>,
    done: Option<String>,
}

impl States {
    fn of(&self, b: Bucket) -> String {
        match b {
            Bucket::Done => self.done.clone().unwrap_or_else(|| self.initial.clone()),
            Bucket::Active => self.active.clone().unwrap_or_else(|| self.initial.clone()),
            Bucket::Initial => self.initial.clone(),
        }
    }
}

fn title_of(item: &Item) -> String {
    item.title
        .chars()
        .take(crate::validation::MAX_TEXT_LEN)
        .collect()
}

/// An item's due date, if the scheduler accepts it.
fn deadline_of(item: &Item) -> Option<chrono::NaiveDate> {
    item.due
        .filter(|d| crate::cpm::constraint_problems(None, None, Some(*d)).is_empty())
}

#[expect(clippy::too_many_lines)]
async fn run(
    ctx: AppContext,
    caller: MaybeAuthUser,
    pid: String,
    params: ImportParams,
    body: Bytes,
    source: Source,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let dry_run = params.dry_run.unwrap_or(true);
    let text = std::str::from_utf8(&body)
        .map_err(|_| unprocessable("the file is not UTF-8 text; export the CSV as UTF-8"))?;
    let parsed = reader::parse(source, text).map_err(|e| unprocessable(&e.to_string()))?;
    let mut unmapped: Vec<serde_json::Value> = parsed
        .unmapped
        .iter()
        .map(|u| serde_json::json!({ "what": u.what, "key": u.key, "reason": u.reason }))
        .collect();
    let mut note = |what: &str, key: Option<&str>, reason: String| {
        unmapped.push(serde_json::json!({ "what": what, "key": key, "reason": reason }));
    };

    let workflow = super::workflow::in_force(&ctx, plan.pid, "task").await?;
    let pick = |pred: &dyn Fn(&crate::workflow::StateDef) -> bool| {
        workflow
            .states
            .iter()
            .find(|s| pred(s))
            .map(|s| s.key.clone())
    };
    let mapping = States {
        initial: pick(&|s| s.is_initial).unwrap_or_else(|| "todo".to_string()),
        active: pick(&|s| s.category == crate::workflow::Category::Active),
        done: pick(&|s| s.category == crate::workflow::Category::Done),
    };
    if mapping.active.is_none() {
        note(
            "status",
            None,
            "the plan's workflow has no active state: tasks under way start in the initial state"
                .into(),
        );
    }
    if mapping.done.is_none() {
        note(
            "status",
            None,
            "the plan's workflow has no done state: finished tasks start in the initial state"
                .into(),
        );
    }
    for it in &parsed.items {
        if it.due.is_some() && deadline_of(it).is_none() {
            note(
                "date",
                Some(&it.key),
                "the due date is out of range; not imported".into(),
            );
        }
    }

    let prefix = source.prefix();
    let ext = |key: &str| format!("{prefix}:{key}");
    let existing: HashMap<String, tasks::Model> = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::ExternalRef.is_not_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .filter_map(|t| t.external_ref.clone().map(|e| (e, t)))
        .collect();
    let to_create = parsed
        .items
        .iter()
        .filter(|i| !existing.contains_key(&ext(&i.key)))
        .count();
    let to_update = parsed.items.len() - to_create;
    let links_in_file: usize = parsed.items.iter().map(|i| i.blocks.len()).sum();

    let (mut created, mut updated, mut links_created, mut links_existing) = (0, 0, 0, 0);
    if !dry_run {
        let plan_task_pids: Vec<Uuid> = tasks::Entity::find()
            .filter(tasks::Column::PlanPid.eq(plan.pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|t| t.pid)
            .collect();
        let mut edges: Vec<(Uuid, Uuid)> = super::cpm::deps_touching(&ctx, plan_task_pids)
            .await?
            .into_iter()
            .map(|d| (d.predecessor_pid, d.successor_pid))
            .collect();
        let have: HashSet<(Uuid, Uuid)> = edges.iter().copied().collect();
        let now = chrono::Utc::now();
        let txn = ctx.db.begin().await.map_err(db_err)?;
        let mut pid_of: HashMap<String, Uuid> = HashMap::new();
        for it in &parsed.items {
            let status = mapping.of(it.bucket);
            if let Some(row) = existing.get(&ext(&it.key)) {
                let mut a: tasks::ActiveModel = row.clone().into();
                a.title = ActiveValue::set(title_of(it));
                a.deadline = ActiveValue::set(deadline_of(it));
                if it.start.is_some() {
                    a.start_date = ActiveValue::set(it.start);
                }
                a.update(&txn).await.map_err(db_err)?;
                pid_of.insert(it.key.clone(), row.pid);
                updated += 1;
            } else {
                let row = tasks::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    plan_pid: ActiveValue::set(plan.pid),
                    title: ActiveValue::set(title_of(it)),
                    description: ActiveValue::set(it.description.clone()),
                    status: ActiveValue::set(status.clone()),
                    start_date: ActiveValue::set(it.start),
                    deadline: ActiveValue::set(deadline_of(it)),
                    external_ref: ActiveValue::set(Some(ext(&it.key))),
                    status_changed_at: ActiveValue::set(now.into()),
                    done_at: ActiveValue::set(
                        crate::workflow::is_done(&workflow, &status).then(|| now.into()),
                    ),
                    deleted_at: ActiveValue::set(None),
                    ..Default::default()
                }
                .insert(&txn)
                .await
                .map_err(db_err)?;
                crate::tba::transition_row(
                    row.pid,
                    plan.pid,
                    None,
                    status,
                    now,
                    caller.actor().map(ToString::to_string),
                    None,
                )
                .insert(&txn)
                .await
                .map_err(db_err)?;
                pid_of.insert(it.key.clone(), row.pid);
                created += 1;
            }
        }
        // Parents, now that every task has an id (a parent may come later in the file).
        for it in &parsed.items {
            let (Some(&me), Some(&parent)) = (
                pid_of.get(&it.key),
                it.parent.as_ref().and_then(|p| pid_of.get(p)),
            ) else {
                continue;
            };
            tasks::Entity::update_many()
                .col_expr(
                    tasks::Column::ParentTaskPid,
                    sea_orm::sea_query::Expr::value(parent),
                )
                .filter(tasks::Column::Pid.eq(me))
                .exec(&txn)
                .await
                .map_err(db_err)?;
        }
        for it in &parsed.items {
            for blocked in &it.blocks {
                let (Some(&pred), Some(&succ)) = (pid_of.get(&it.key), pid_of.get(blocked)) else {
                    continue;
                };
                if have.contains(&(pred, succ)) {
                    links_existing += 1;
                    continue;
                }
                if edges.contains(&(pred, succ)) {
                    continue;
                }
                if crate::visibility::would_create_cycle(&edges, pred, succ) {
                    note(
                        "link",
                        Some(&it.key),
                        format!("the link to {blocked} would close a cycle; skipped"),
                    );
                    continue;
                }
                plan_dependencies::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    predecessor_pid: ActiveValue::set(pred),
                    successor_pid: ActiveValue::set(succ),
                    lag_days: ActiveValue::set(0),
                    kind: ActiveValue::set("fs".to_string()),
                    ..Default::default()
                }
                .insert(&txn)
                .await
                .map_err(db_err)?;
                edges.push((pred, succ));
                links_created += 1;
            }
        }
        AuditModel::record(
            &txn,
            plan.pid,
            &format!("{prefix}_import"),
            caller.actor(),
            Some(
                serde_json::json!({ "tasks_created": created, "tasks_updated": updated,
                "links_created": links_created, "unmapped": unmapped.len() }),
            ),
        )
        .await
        .map_err(Error::Model)?;
        txn.commit().await.map_err(db_err)?;
    }
    format::json(serde_json::json!({
        "dry_run": dry_run,
        "source": prefix,
        "items_in_file": parsed.items.len(),
        "tasks_created": if dry_run { to_create } else { created },
        "tasks_updated": if dry_run { to_update } else { updated },
        "links_in_file": links_in_file,
        "links_created": links_created,
        "links_already_present": links_existing,
        "unmapped": unmapped,
        "note": "Due dates come across as deadlines; no durations are guessed, so imported tasks have no schedule until you give them one. A re-import updates titles, deadlines, start dates and parents, and never changes status or description. Assignees are names, not identities, and stay in the description.",
    }))
}

/// `POST /api/plans/{pid}/import/jira`.
#[debug_handler]
async fn import_jira(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(params): Query<ImportParams>,
    body: Bytes,
) -> Result<Response> {
    run(ctx, caller, pid, params, body, Source::Jira).await
}

/// `POST /api/plans/{pid}/import/asana`.
#[debug_handler]
async fn import_asana(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(params): Query<ImportParams>,
    body: Bytes,
) -> Result<Response> {
    run(ctx, caller, pid, params, body, Source::Asana).await
}

/// The tracker import routes (body limit raised to the reader's cap).
pub fn routes() -> Routes {
    let ceiling = reader::MAX_BYTES + 64 * 1024;
    Routes::new()
        .prefix("/api")
        .add(
            "/plans/{pid}/import/jira",
            post(import_jira).layer(DefaultBodyLimit::max(ceiling)),
        )
        .add(
            "/plans/{pid}/import/asana",
            post(import_asana).layer(DefaultBodyLimit::max(ceiling)),
        )
}

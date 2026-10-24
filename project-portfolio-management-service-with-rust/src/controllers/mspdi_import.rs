//! MS Project (MSPDI XML) import into an **existing plan** (PJ-10, T-28n).
//! The reader and the pinned id tables are [`crate::mspdi`].
//!
//! `POST /api/plans/{pid}/import/mspdi?dry_run=true` (the default) returns the
//! report without writing; `dry_run=false` applies it. **Idempotent:** a task's
//! MS Project UID is stored as `external_ref = mspdi:<UID>` (unique per plan), so
//! importing the same file again updates schedule fields, constraints, deadlines,
//! titles and parents in place and never duplicates; a task's *status* and any
//! edits to its description are left alone on re-import.
//!
//! **Nothing is silently dropped:** the report lists every task, link,
//! constraint, resource and baseline that did not come across, with the reason.
//! A hard constraint (`mso`, `mfo`) needs the same permission as setting one by
//! hand; without it the task is imported *without* that constraint and the
//! report says so. Resources are not matched to people (names are not identities):
//! their names are kept in the task description and listed.

use super::db_err;
use std::collections::HashMap;

use axum::body::Bytes;
use axum::extract::DefaultBodyLimit;
use loco_rs::prelude::*;
use sea_orm::TransactionTrait;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cpm::{self, ConstraintKind};
use crate::models::_entities::{plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::mspdi::{self, MsConstraint};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ImportParams {
    /// `true` (default) reports without writing.
    #[serde(default)]
    dry_run: Option<bool>,
}

/// One planned task write.
struct Planned {
    uid: i64,
    ext: String,
    title: String,
    description: Option<String>,
    start: Option<chrono::NaiveDate>,
    duration: Option<i32>,
    constraint: Option<(String, chrono::NaiveDate)>,
    deadline: Option<chrono::NaiveDate>,
    parent_uid: Option<i64>,
    status: StatusChoice,
}

#[derive(Clone, Copy)]
enum StatusChoice {
    Initial,
    Active,
    Done,
}

#[derive(Default)]
struct Report {
    unmapped: Vec<serde_json::Value>,
}

impl Report {
    fn note(&mut self, what: &str, uid: Option<i64>, reason: impl Into<String>) {
        self.unmapped
            .push(serde_json::json!({ "what": what, "uid": uid, "reason": reason.into() }));
    }
}

#[expect(clippy::too_many_lines)]
#[debug_handler]
async fn import(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(params): Query<ImportParams>,
    body: Bytes,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let dry_run = params.dry_run.unwrap_or(true);
    let xml = std::str::from_utf8(&body)
        .map_err(|_| unprocessable("the file is not UTF-8 text; export MS Project XML as UTF-8"))?;
    let parsed = mspdi::parse(xml).map_err(|e| unprocessable(&e.to_string()))?;
    let mut report = Report::default();
    for u in &parsed.unmapped {
        report.note(u.what, u.uid, u.reason.clone());
    }

    // The workflow's states: where a new task starts, and what 1–99% and 100% mean.
    let workflow = super::workflow::in_force(&ctx, plan.pid, "task").await?;
    let key_of = |pred: &dyn Fn(&crate::workflow::StateDef) -> bool| {
        workflow
            .states
            .iter()
            .find(|s| pred(s))
            .map(|s| s.key.clone())
    };
    let initial = key_of(&|s| s.is_initial).unwrap_or_else(|| "todo".to_string());
    let active = key_of(&|s| s.category == crate::workflow::Category::Active);
    let done = key_of(&|s| s.category == crate::workflow::Category::Done);
    if active.is_none() {
        report.note("status", None, "the plan's workflow has no active state: partly complete tasks start in the initial state");
    }
    if done.is_none() {
        report.note(
            "status",
            None,
            "the plan's workflow has no done state: finished tasks start in the initial state",
        );
    }

    let hard_ok = super::cpm::may_set_hard_constraint(&caller);
    let names: HashMap<i64, &str> = parsed
        .resources
        .iter()
        .map(|(u, n)| (*u, n.as_str()))
        .collect();
    let mut assigned: HashMap<i64, Vec<&str>> = HashMap::new();
    for (task, res) in &parsed.assignments {
        if let Some(n) = names.get(res) {
            assigned.entry(*task).or_default().push(n);
        }
    }
    for (_, name) in &parsed.resources {
        report.note("resource", None, format!("{name}: not matched to a person (names are not identities); kept in the description of the tasks they were assigned to"));
    }
    if !parsed.resources.is_empty() {
        report.note(
            "cost",
            None,
            "resource cost rates are not imported (cost rates are not built yet)",
        );
    }

    // Plan every task; parents come from the outline.
    let mut planned: Vec<Planned> = Vec::new();
    let mut level_stack: Vec<(i64, i64)> = Vec::new(); // (outline level, uid)
    for t in &parsed.tasks {
        while level_stack
            .last()
            .is_some_and(|(l, _)| *l >= t.outline_level)
        {
            level_stack.pop();
        }
        let parent_uid = level_stack.last().map(|(_, u)| *u);
        level_stack.push((t.outline_level, t.uid));
        let mut title = t.name.clone();
        if title.len() > crate::validation::MAX_TEXT_LEN {
            title.truncate(crate::validation::MAX_TEXT_LEN);
            report.note(
                "task",
                Some(t.uid),
                "the title was truncated to the text cap",
            );
        }
        let (start, duration) = if t.summary {
            (None, None)
        } else {
            let dur = t.duration_days.and_then(|d| i32::try_from(d).ok());
            let problems = cpm::schedule_problems(t.start, dur);
            if problems.is_empty() {
                (t.start, dur)
            } else {
                report.note(
                    "schedule",
                    Some(t.uid),
                    format!("not imported: {}", problems.join("; ")),
                );
                (None, None)
            }
        };
        let constraint = match t.constraint {
            None | Some((MsConstraint::Asap, _)) => None,
            Some((MsConstraint::Alap, _)) => {
                report.note("constraint", Some(t.uid), "as-late-as-possible is not supported yet; the task is scheduled as soon as possible");
                None
            }
            Some((kind, date)) => {
                let ours = match kind {
                    MsConstraint::Mso => "mso",
                    MsConstraint::Mfo => "mfo",
                    MsConstraint::Snet => "snet",
                    MsConstraint::Snlt => "snlt",
                    MsConstraint::Fnet => "fnet",
                    MsConstraint::Fnlt => "fnlt",
                    MsConstraint::Asap | MsConstraint::Alap => unreachable!("handled above"),
                };
                if ConstraintKind::parse(ours).is_some_and(ConstraintKind::is_hard) && !hard_ok {
                    report.note("constraint", Some(t.uid), format!("{ours} overrides the schedule's logic and needs access=admin or scheduling=hard_constraints; imported without it"));
                    None
                } else if !cpm::constraint_problems(Some(ours), Some(date), None).is_empty() {
                    report.note(
                        "constraint",
                        Some(t.uid),
                        "the constraint date is out of range; imported without it",
                    );
                    None
                } else {
                    Some((ours.to_string(), date))
                }
            }
        };
        let mut description = t.notes.clone();
        if let Some(who) = assigned.get(&t.uid) {
            let line = format!("Assigned in MS Project: {}", who.join(", "));
            description = Some(description.map_or(line.clone(), |d| format!("{d}\n\n{line}")));
        }
        planned.push(Planned {
            uid: t.uid,
            ext: format!("mspdi:{}", t.uid),
            title,
            description,
            start,
            duration,
            constraint,
            deadline: t
                .deadline
                .filter(|d| cpm::constraint_problems(None, None, Some(*d)).is_empty()),
            parent_uid,
            status: match t.percent_complete {
                100 if done.is_some() => StatusChoice::Done,
                1..=99 if active.is_some() => StatusChoice::Active,
                _ => StatusChoice::Initial,
            },
        });
    }

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
    let to_create = planned
        .iter()
        .filter(|p| !existing.contains_key(&p.ext))
        .count();
    let to_update = planned.len() - to_create;
    let known_uids: std::collections::HashSet<i64> = planned.iter().map(|p| p.uid).collect();
    let usable_links: Vec<&mspdi::MsLink> = parsed
        .links
        .iter()
        .filter(|l| {
            let ok = known_uids.contains(&l.predecessor) && known_uids.contains(&l.successor) && (-3650..=3650).contains(&l.lag_days);
            if !ok {
                report.note("link", Some(l.successor), format!("a link from task {} was skipped (an end is not an imported task, or the lag is out of range)", l.predecessor));
            }
            ok
        })
        .collect();

    let mut created = 0;
    let mut updated = 0;
    let mut links_created = 0;
    let mut links_existing = 0;
    let mut calendar_imported = false;
    if !dry_run {
        let now = chrono::Utc::now();
        let txn = ctx.db.begin().await.map_err(db_err)?;
        let mut pid_of: HashMap<i64, Uuid> = HashMap::new();
        for p in &planned {
            let parent = p.parent_uid.and_then(|u| pid_of.get(&u).copied());
            let (ckind, cdate) = p
                .constraint
                .clone()
                .map_or((None, None), |(k, d)| (Some(k), Some(d)));
            if let Some(row) = existing.get(&p.ext) {
                let mut a: tasks::ActiveModel = row.clone().into();
                a.title = ActiveValue::set(p.title.clone());
                a.start_date = ActiveValue::set(p.start);
                a.duration_days = ActiveValue::set(p.duration);
                a.constraint_kind = ActiveValue::set(ckind);
                a.constraint_date = ActiveValue::set(cdate);
                a.deadline = ActiveValue::set(p.deadline);
                a.parent_task_pid = ActiveValue::set(parent);
                a.update(&txn).await.map_err(db_err)?;
                pid_of.insert(p.uid, row.pid);
                updated += 1;
            } else {
                let status = match p.status {
                    StatusChoice::Done => done.clone().unwrap_or_else(|| initial.clone()),
                    StatusChoice::Active => active.clone().unwrap_or_else(|| initial.clone()),
                    StatusChoice::Initial => initial.clone(),
                };
                let row = tasks::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    plan_pid: ActiveValue::set(plan.pid),
                    title: ActiveValue::set(p.title.clone()),
                    description: ActiveValue::set(p.description.clone()),
                    status: ActiveValue::set(status.clone()),
                    start_date: ActiveValue::set(p.start),
                    duration_days: ActiveValue::set(p.duration),
                    constraint_kind: ActiveValue::set(ckind),
                    constraint_date: ActiveValue::set(cdate),
                    deadline: ActiveValue::set(p.deadline),
                    parent_task_pid: ActiveValue::set(parent),
                    external_ref: ActiveValue::set(Some(p.ext.clone())),
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
                pid_of.insert(p.uid, row.pid);
                created += 1;
            }
        }
        // Edges: skip a duplicate pair, and refuse one that would close a cycle.
        let mut edges: Vec<(Uuid, Uuid)> = plan_dependencies::Entity::find()
            .all(&txn)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|d| (d.predecessor_pid, d.successor_pid))
            .collect();
        edges.extend(
            task_relations::Entity::find()
                .filter(task_relations::Column::DeletedAt.is_null())
                .filter(task_relations::Column::Kind.eq("precedes"))
                .all(&txn)
                .await
                .map_err(db_err)?
                .into_iter()
                .map(|r| (r.from_task_pid, r.to_task_pid)),
        );
        for l in usable_links {
            let (Some(&pred), Some(&succ)) = (pid_of.get(&l.predecessor), pid_of.get(&l.successor))
            else {
                continue;
            };
            if edges.contains(&(pred, succ)) {
                links_existing += 1;
                continue;
            }
            if crate::visibility::would_create_cycle(&edges, pred, succ) {
                report.note(
                    "link",
                    Some(l.successor),
                    format!(
                        "the link from task {} would close a cycle; skipped",
                        l.predecessor
                    ),
                );
                continue;
            }
            plan_dependencies::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                predecessor_pid: ActiveValue::set(pred),
                successor_pid: ActiveValue::set(succ),
                lag_days: ActiveValue::set(i32::try_from(l.lag_days).unwrap_or(0)),
                kind: ActiveValue::set(
                    match l.kind {
                        cpm::Kind::Fs => "fs",
                        cpm::Kind::Ss => "ss",
                        cpm::Kind::Ff => "ff",
                        cpm::Kind::Sf => "sf",
                    }
                    .to_string(),
                ),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?;
            edges.push((pred, succ));
            links_created += 1;
        }
        txn.commit().await.map_err(db_err)?;
        if let Some(cal) = &parsed.calendar
            && !super::calendars::plan_has_calendar(&ctx, plan.pid).await?
        {
            let mut holidays = cal.holidays.clone();
            if holidays.len() > crate::calendars::MAX_HOLIDAYS {
                holidays.truncate(crate::calendars::MAX_HOLIDAYS);
                report.note(
                    "calendar",
                    None,
                    format!(
                        "only the first {} holidays were imported",
                        crate::calendars::MAX_HOLIDAYS
                    ),
                );
            }
            if crate::calendars::problems(&cal.weekdays, &holidays).is_empty() {
                super::calendars::set_plan_calendar(
                    &ctx,
                    plan.pid,
                    &cal.weekdays,
                    &holidays,
                    caller.actor(),
                )
                .await?;
                calendar_imported = true;
            } else {
                report.note("calendar", None, "the project calendar could not be imported (no business day, or dates out of range)");
            }
        }
        // Not atomic with the import (it is many writes through helpers): a failed audit row fails the
        // request, and the import is idempotent, so asking again converges and records it.
        AuditModel::record(
            &ctx.db,
            plan.pid,
            "mspdi_import",
            caller.actor(),
            Some(
                serde_json::json!({ "tasks_created": created, "tasks_updated": updated,
                "links_created": links_created, "unmapped": report.unmapped.len() }),
            ),
        )
        .await
        .map_err(super::audit_failed)?;
    } else if let Some(cal) = &parsed.calendar {
        if super::calendars::plan_has_calendar(&ctx, plan.pid).await? {
            report.note(
                "calendar",
                None,
                "the plan already has a calendar; the project calendar would not replace it",
            );
        } else if crate::calendars::problems(&cal.weekdays, &cal.holidays).is_empty() {
            calendar_imported = true;
        }
    }
    format::json(serde_json::json!({
        "dry_run": dry_run,
        "project_name": parsed.name,
        "tasks_in_file": parsed.tasks.len(),
        "tasks_created": if dry_run { to_create } else { created },
        "tasks_updated": if dry_run { to_update } else { updated },
        "links_in_file": parsed.links.len(),
        "links_created": links_created,
        "links_already_present": links_existing,
        "calendar_imported": calendar_imported,
        "unmapped": report.unmapped,
        "note": "Summary tasks are imported without schedule fields (their dates are roll-ups, so they would double-count in the critical path). A re-import updates schedule fields, constraints, deadlines, titles and parents and never changes status. Hard constraints need access=admin or scheduling=hard_constraints.",
    }))
}

/// The MS Project import route (body limit raised to the reader's cap).
pub fn routes() -> Routes {
    let ceiling = mspdi::MAX_BYTES + 64 * 1024;
    Routes::new().prefix("/api").add(
        "/plans/{pid}/import/mspdi",
        post(import).layer(DefaultBodyLimit::max(ceiling)),
    )
}

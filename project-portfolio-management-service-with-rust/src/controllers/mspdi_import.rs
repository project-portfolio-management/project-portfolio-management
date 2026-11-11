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

use std::collections::HashMap;

use crate::prelude::*;
use axa::middleware::body_limit;
use axum::body::Bytes;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cpm::{self, ConstraintKind};
use crate::models::_entities::tasks;
use crate::models::audit_logs::Model as AuditModel;
use crate::mspdi::{self, MsConstraint};

/// Query parameters for `POST /api/plans/{pid}/import/mspdi`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ImportParams {
    /// `true` (default) reports without writing.
    #[serde(default)]
    dry_run: Option<bool>,
}

/// One planned task write.
struct Planned {
    /// MS Project task UID, the key used to match tasks on re-import (stored as `mspdi:<uid>`).
    uid: i64,
    /// Stable external reference (`mspdi:<uid>`) stored on the task so a re-import updates instead of duplicating.
    ext: String,
    /// Task title, already truncated to the text cap.
    title: String,
    /// Task description: the file's notes plus a line naming the resources assigned in the source file; `None` when neither exists.
    description: Option<String>,
    /// Start date; `None` for summary tasks or when the schedule data was rejected (absent is not zero).
    start: Option<chrono::NaiveDate>,
    /// Duration in working days; `None` for summary tasks or rejected data.
    duration: Option<i32>,
    /// Optional scheduling constraint as (kind token, date); `None` when none was set or it could not be imported.
    constraint: Option<(String, chrono::NaiveDate)>,
    /// Optional hard deadline; `None` when absent or out of range.
    deadline: Option<chrono::NaiveDate>,
    /// UID of the enclosing summary task derived from the outline level; `None` for top-level tasks.
    parent_uid: Option<i64>,
    /// Workflow state the task should take when newly created.
    status: StatusChoice,
}

/// Which workflow state a newly created task starts in, derived from percent complete.
#[derive(Clone, Copy)]
enum StatusChoice {
    /// The workflow's initial state (0% complete, or no matching state available).
    Initial,
    /// The workflow's first active-category state (1 to 99% complete).
    Active,
    /// The workflow's done-category state (100% complete).
    Done,
}

/// Accumulates what the import could not map, so the response states gaps instead of silently dropping data.
#[derive(Default)]
struct Report {
    /// One entry per unmapped item: `what`, optional `uid`, and the `reason`.
    unmapped: Vec<serde_json::Value>,
}

impl Report {
    /// Records one unmapped item with its reason.
    fn note(&mut self, what: &str, uid: Option<i64>, reason: impl Into<String>) {
        self.unmapped
            .push(serde_json::json!({ "what": what, "uid": uid, "reason": reason.into() }));
    }
}

/// `POST /api/plans/{pid}/import/mspdi` — import an MS Project XML body into the plan. Dry run by default (`dry_run` absent means true); caller authorization and audit are handled in [`run`].
#[debug_handler]
async fn import(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(params): Query<ImportParams>,
    body: Bytes,
) -> Result<Response> {
    format::json(run(&ctx, &caller, &pid, params.dry_run.unwrap_or(true), &body).await?)
}

/// Import one MSPDI file into the plan `pid`; the report is the route's answer. Also called once per file by the
/// batch import ([`super::mspdi_batch`]).
#[expect(clippy::too_many_lines)]
pub(super) async fn run(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
    dry_run: bool,
    body: &[u8],
) -> Result<serde_json::Value> {
    let plan = super::governance::find_item(ctx, pid).await?;
    let xml = std::str::from_utf8(body)
        .map_err(|_| unprocessable("the file is not UTF-8 text; export MS Project XML as UTF-8"))?;
    let parsed = mspdi::parse(xml).map_err(|e| unprocessable(&e.to_string()))?;
    let mut report = Report::default();
    for u in &parsed.unmapped {
        report.note(u.what, u.uid, u.reason.clone());
    }

    // The workflow's states: where a new task starts, and what 1–99% and 100% mean.
    let workflow = super::workflow::in_force(ctx, plan.pid, "task").await?;
    let key_of = |pred: &dyn Fn(&crate::workflow::StateDef) -> bool| {
        workflow
            .states
            .iter()
            .find(|s| pred(s))
            .map(|s| s.key.clone())
    };
    let initial = key_of(&|s| s.is_initial).unwrap_or_else(|| "todo".to_string());
    // A file cannot place a task past a gate, so under restricted moves everything starts at the start.
    let restricted = !workflow.transitions.is_empty();
    let active =
        key_of(&|s| s.category == crate::workflow::Category::Active).filter(|_| !restricted);
    let done = key_of(&|s| s.category == crate::workflow::Category::Done).filter(|_| !restricted);
    if restricted {
        report.note(
            "status",
            None,
            "the plan's workflow restricts moves: every task starts in the initial state",
        );
    }
    if active.is_none() && !restricted {
        report.note("status", None, "the plan's workflow has no active state: partly complete tasks start in the initial state");
    }
    if done.is_none() && !restricted {
        report.note(
            "status",
            None,
            "the plan's workflow has no done state: finished tasks start in the initial state",
        );
    }

    let hard_ok = super::cpm::may_set_hard_constraint(caller);
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

    let existing: HashMap<String, tasks::Model> = sqlx::query_as::<_, tasks::Model>(
        "SELECT * FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL AND external_ref IS NOT NULL",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(ctx))
    .await?
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
        let at = chrono::DateTime::<chrono::FixedOffset>::from(now);
        let mut tx = crate::db::begin(ctx).await?;
        let mut pid_of: HashMap<i64, Uuid> = HashMap::new();
        for p in &planned {
            let parent = p.parent_uid.and_then(|u| pid_of.get(&u).copied());
            let (ckind, cdate) = p
                .constraint
                .clone()
                .map_or((None, None), |(k, d)| (Some(k), Some(d)));
            if let Some(row) = existing.get(&p.ext) {
                // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
                sqlx::query_scalar::<_, i32>(
                    "UPDATE tasks
                     SET title = $1, start_date = $2, duration_days = $3, constraint_kind = $4, constraint_date = $5,
                         deadline = $6, parent_task_pid = $7
                     WHERE id = $8
                     RETURNING id",
                )
                .bind(&p.title)
                .bind(p.start)
                .bind(p.duration)
                .bind(ckind)
                .bind(cdate)
                .bind(p.deadline)
                .bind(parent)
                .bind(row.id)
                .fetch_one(tx.conn())
                .await?;
                pid_of.insert(p.uid, row.pid);
                updated += 1;
            } else {
                let status = match p.status {
                    StatusChoice::Done => done.clone().unwrap_or_else(|| initial.clone()),
                    StatusChoice::Active => active.clone().unwrap_or_else(|| initial.clone()),
                    StatusChoice::Initial => initial.clone(),
                };
                let task_pid = Uuid::new_v4();
                // The columns `SeaORM`'s insert set, no more.
                sqlx::query(
                    "INSERT INTO tasks
                         (pid, plan_pid, title, description, status, start_date, duration_days, constraint_kind,
                          constraint_date, deadline, parent_task_pid, external_ref, status_changed_at, done_at,
                          deleted_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NULL)",
                )
                .bind(task_pid)
                .bind(plan.pid)
                .bind(&p.title)
                .bind(&p.description)
                .bind(&status)
                .bind(p.start)
                .bind(p.duration)
                .bind(ckind)
                .bind(cdate)
                .bind(p.deadline)
                .bind(parent)
                .bind(&p.ext)
                .bind(at)
                .bind(crate::workflow::is_done(&workflow, &status).then_some(at))
                .execute(tx.conn())
                .await?;
                crate::tba::insert_transition_on(
                    &mut tx,
                    crate::tba::NewTransition {
                        task_pid,
                        plan_pid: plan.pid,
                        from_status: None,
                        to_status: &status,
                        at: now,
                        actor_ref: caller.actor(),
                        assignee_ref: None,
                        blocked_reason: None,
                    },
                )
                .await?;
                pid_of.insert(p.uid, task_pid);
                created += 1;
            }
        }
        // Edges: skip a duplicate pair, and refuse one that would close a cycle.
        // Every dependency and live `precedes` relation (as before, not only this plan's); only the two ends are read.
        let mut edges: Vec<(Uuid, Uuid)> =
            sqlx::query_as("SELECT predecessor_pid, successor_pid FROM plan_dependencies")
                .fetch_all(tx.conn())
                .await?;
        edges.extend(
            sqlx::query_as::<_, (Uuid, Uuid)>(
                "SELECT from_task_pid, to_task_pid FROM task_relations WHERE deleted_at IS NULL AND kind = 'precedes'",
            )
            .fetch_all(tx.conn())
            .await?,
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
            sqlx::query(
                "INSERT INTO plan_dependencies (pid, predecessor_pid, successor_pid, lag_days, kind)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(Uuid::new_v4())
            .bind(pred)
            .bind(succ)
            .bind(i32::try_from(l.lag_days).unwrap_or(0))
            .bind(match l.kind {
                cpm::Kind::Fs => "fs",
                cpm::Kind::Ss => "ss",
                cpm::Kind::Ff => "ff",
                cpm::Kind::Sf => "sf",
            })
            .execute(tx.conn())
            .await?;
            edges.push((pred, succ));
            links_created += 1;
        }
        tx.commit().await?;
        if let Some(cal) = &parsed.calendar
            && !super::calendars::plan_has_calendar(ctx, plan.pid).await?
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
                    ctx,
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
        AuditModel::record_on(
            crate::db::pool(ctx),
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
        if super::calendars::plan_has_calendar(ctx, plan.pid).await? {
            report.note(
                "calendar",
                None,
                "the plan already has a calendar; the project calendar would not replace it",
            );
        } else if crate::calendars::problems(&cal.weekdays, &cal.holidays).is_empty() {
            calendar_imported = true;
        }
    }
    Ok(serde_json::json!({
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
        body_limit(post(import), ceiling),
    )
}

//! MS Project (MSPDI XML) export of one plan (PJ-10): `GET /api/plans/{pid}/export/mspdi`. The writer is
//! [`crate::mspdi::write`]; the importer is [`super::mspdi_import`], and what one writes the other reads.
//!
//! A task keeps the UID it was imported with (`external_ref = mspdi:<UID>`), so a file exported after an import
//! round-trips to the same UIDs; other tasks get fresh UIDs above the largest in use. Siblings follow
//! their stored order. Dependencies come from the plan's links (kind and lag) and from `precedes` task
//! relations (finish-to-start, no lag).

use super::db_err;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use axum::http::{StatusCode, header};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::MaybeAuthUser;
use crate::cpm;
use crate::models::_entities::{plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::mspdi::{self, MsCalendar, MsConstraint, MsLink, MsTask};

/// What the file does not carry; written into it as a comment and listed in the API description.
pub const NOT_EXPORTED: &str = "Exported from Project Portfolio Management. Not carried: assignees and resources, \
status other than done (percent complete is 0 or 100), sprints, components, effort and work contours, \
custom fields, comments, baselines, relations other than 'precedes', and elapsed-time lags (all lags are working days).";

fn constraint_of(kind: &str) -> Option<MsConstraint> {
    Some(match kind {
        "mso" => MsConstraint::Mso,
        "mfo" => MsConstraint::Mfo,
        "snet" => MsConstraint::Snet,
        "snlt" => MsConstraint::Snlt,
        "fnet" => MsConstraint::Fnet,
        "fnlt" => MsConstraint::Fnlt,
        _ => return None,
    })
}

fn link_kind(kind: &str) -> cpm::Kind {
    match kind {
        "ss" => cpm::Kind::Ss,
        "ff" => cpm::Kind::Ff,
        "sf" => cpm::Kind::Sf,
        _ => cpm::Kind::Fs,
    }
}

/// Tasks in outline order (parents before children, siblings by position then age). A task whose parent is
/// not in the list becomes top level; a cycle is broken rather than looped. Returns `(task, outline level)`.
fn outline(all: &[tasks::Model]) -> Vec<(&tasks::Model, i64)> {
    let known: BTreeSet<Uuid> = all.iter().map(|t| t.pid).collect();
    let mut kids: BTreeMap<Option<Uuid>, Vec<&tasks::Model>> = BTreeMap::new();
    for t in all {
        let parent = t
            .parent_task_pid
            .filter(|p| known.contains(p) && *p != t.pid);
        kids.entry(parent).or_default().push(t);
    }
    for list in kids.values_mut() {
        list.sort_by_key(|t| (t.sibling_position.unwrap_or(i32::MAX), t.id));
    }
    let mut out = Vec::with_capacity(all.len());
    let mut seen = BTreeSet::new();
    let mut stack: Vec<(&tasks::Model, i64)> = kids
        .get(&None)
        .into_iter()
        .flatten()
        .rev()
        .map(|t| (*t, 1))
        .collect();
    while let Some((t, level)) = stack.pop() {
        if !seen.insert(t.pid) {
            continue;
        }
        out.push((t, level));
        stack.extend(
            kids.get(&Some(t.pid))
                .into_iter()
                .flatten()
                .rev()
                .map(|c| (*c, level + 1)),
        );
    }
    // Anything left is in a cycle: list it at the top level rather than lose it.
    for t in all {
        if seen.insert(t.pid) {
            out.push((t, 1));
        }
    }
    out
}

/// A file name from a plan name: letters and digits, the rest dashes.
fn file_stem(name: &str) -> String {
    let stem: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let stem = stem.trim_matches('-');
    if stem.is_empty() {
        "plan".to_string()
    } else {
        stem.chars().take(60).collect()
    }
}

/// `GET /api/plans/{pid}/export/mspdi`: the plan's tasks, links and calendar as an MSPDI file.
#[debug_handler]
#[expect(clippy::too_many_lines)] // gather, order, number, link, write: one linear read
async fn export(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let all = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if all.len() > mspdi::MAX_TASKS {
        return Err(Error::CustomError(
            StatusCode::PAYLOAD_TOO_LARGE,
            ErrorDetail::new(
                "too_large",
                format!(
                    "the plan has {} tasks; an MSPDI file holds at most {} (the most the importer reads)",
                    all.len(),
                    mspdi::MAX_TASKS
                ),
            ),
        ));
    }
    let ordered = outline(&all);
    let has_children: BTreeSet<Uuid> = all.iter().filter_map(|t| t.parent_task_pid).collect();

    // UIDs: the imported one where present and unique, else fresh ones above the largest.
    let mut taken: BTreeSet<i64> = BTreeSet::new();
    let mut uid_of: HashMap<Uuid, i64> = HashMap::new();
    for (t, _) in &ordered {
        let imported = t
            .external_ref
            .as_deref()
            .and_then(|e| e.strip_prefix("mspdi:"))
            .and_then(|n| n.parse::<i64>().ok())
            .filter(|n| *n > 0);
        if let Some(n) = imported
            && taken.insert(n)
        {
            uid_of.insert(t.pid, n);
        }
    }
    let mut next = taken.iter().next_back().copied().unwrap_or(0);
    for (t, _) in &ordered {
        uid_of.entry(t.pid).or_insert_with(|| {
            next += 1;
            next
        });
    }

    let ms_tasks: Vec<MsTask> = ordered
        .iter()
        .map(|(t, level)| MsTask {
            uid: uid_of[&t.pid],
            name: t.title.clone(),
            outline_level: *level,
            start: t.start_date,
            duration_days: t.duration_days.map(i64::from),
            summary: has_children.contains(&t.pid),
            milestone: t.duration_days == Some(0),
            percent_complete: if t.done_at.is_some() { 100 } else { 0 },
            constraint: t
                .constraint_kind
                .as_deref()
                .and_then(constraint_of)
                .zip(t.constraint_date),
            deadline: t.deadline,
            notes: t.description.clone(),
        })
        .collect();

    let mut links: Vec<MsLink> = Vec::new();
    let mut pairs: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut push = |pred: Uuid, succ: Uuid, kind: cpm::Kind, lag: i64| {
        if let (Some(&p), Some(&s)) = (uid_of.get(&pred), uid_of.get(&succ))
            && p != s
            && pairs.insert((p, s))
        {
            links.push(MsLink {
                successor: s,
                predecessor: p,
                kind,
                lag_days: lag,
            });
        }
    };
    let pids: Vec<Uuid> = all.iter().map(|t| t.pid).collect();
    for d in plan_dependencies::Entity::find()
        .filter(plan_dependencies::Column::SuccessorPid.is_in(pids.clone()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        push(
            d.predecessor_pid,
            d.successor_pid,
            link_kind(&d.kind),
            i64::from(d.lag_days),
        );
    }
    for r in task_relations::Entity::find()
        .filter(task_relations::Column::DeletedAt.is_null())
        .filter(task_relations::Column::Kind.eq("precedes"))
        .filter(task_relations::Column::ToTaskPid.is_in(pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        push(r.from_task_pid, r.to_task_pid, cpm::Kind::Fs, 0);
    }

    let calendar = super::calendars::plan_calendar_spec(&ctx, plan.pid)
        .await?
        .map(|(weekdays, holidays)| MsCalendar { weekdays, holidays })
        // WW-1.4: a plan with no calendar is scheduled on every day (`plan_calendar`'s default), so that is the
        // calendar exported, not an assumed Monday to Friday.
        .or_else(|| {
            Some(MsCalendar {
                weekdays: "1111111".into(),
                holidays: Vec::new(),
            })
        });
    let xml = mspdi::write(
        &plan.name,
        NOT_EXPORTED,
        480,
        &ms_tasks,
        &links,
        calendar.as_ref(),
    );
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "mspdi_export",
        caller.actor(),
        Some(serde_json::json!({ "tasks": ms_tasks.len(), "links": links.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    Ok((
        [
            (
                header::CONTENT_TYPE,
                "application/xml; charset=utf-8".to_string(),
            ),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}.xml\"", file_stem(&plan.name)),
            ),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff".to_string()),
        ],
        xml,
    )
        .into_response())
}

/// The MS Project export route.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/export/mspdi", get(export))
}

//! The critical-path view of a plan's **task network** (SC-1, PJ-1, PJ-8).
//! The maths is [`crate::cpm`]; this is the wiring.
//!
//! A task is a node when it has a duration; its planned `start_date` is a lower
//! bound. Edges are the `plan_dependencies` whose two ends are tasks of the plan
//! (with a relationship kind and a lag or lead) plus the task `precedes`
//! relations (finish-to-start, no lag); a dependency wins when both name the
//! same pair. Nodes with no duration, or with no start and no scheduled
//! predecessor, are **listed as unscheduled with the reason** — never given a
//! float of zero.
//!
//! Changing a task's schedule goes through `PUT …/tasks/{pid}/schedule`, which
//! answers with **what moved** across the whole network (PJ-8), not just the
//! task that was edited.

use std::collections::HashMap;

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::cpm::{self, Edge, Kind, Node, Options, WorkCalendar};
use crate::models::_entities::{plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

/// A plan's task network, ready for [`cpm::analyze`].
pub(crate) struct Network {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub rows: Vec<tasks::Model>,
    /// The plan's calendar and where it came from (`plan` or `default`).
    pub plan_cal: WorkCalendar,
    pub cal_source: &'static str,
    /// Per-assignee calendars (the plan's intersected with theirs); a node's
    /// `own_calendar` indexes into it.
    pub own: Vec<WorkCalendar>,
    /// Assignees whose calendar shares no working day with the plan's.
    pub conflicts: Vec<String>,
}

impl Network {
    /// Run the engine on this network.
    pub(crate) fn analyze(&self, near: i64) -> std::result::Result<cpm::CpmResult, cpm::CpmError> {
        cpm::analyze_with(
            &self.nodes,
            &self.edges,
            &self.plan_cal,
            &self.own,
            Options {
                near_critical_days: near,
                ..Options::default()
            },
        )
    }
}

/// Load a plan's tasks and the edges among them.
pub(crate) async fn load_network(ctx: &AppContext, plan_pid: Uuid) -> Result<Network> {
    let rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan_pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .order_by_asc(tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let pids: std::collections::HashSet<Uuid> = rows.iter().map(|t| t.pid).collect();
    let (plan_cal, cal_source) = super::calendars::plan_calendar(ctx, plan_pid).await?;
    let mut refs: Vec<String> = rows.iter().filter_map(|t| t.assignee_ref.clone()).collect();
    refs.sort();
    refs.dedup();
    let (person, conflicts) = super::calendars::person_calendars(ctx, &plan_cal, &refs).await?;
    let mut own: Vec<WorkCalendar> = Vec::new();
    let mut index_of: HashMap<String, usize> = HashMap::new();
    for (r, cal) in person {
        index_of.insert(r, own.len());
        own.push(cal);
    }
    let nodes = rows
        .iter()
        .map(|t| Node {
            pid: t.pid,
            duration: t.duration_days.map(i64::from),
            planned_start: t.start_date,
            constraint: t
                .constraint_kind
                .as_deref()
                .and_then(cpm::ConstraintKind::parse)
                .zip(t.constraint_date)
                .map(|(kind, date)| cpm::Constraint { kind, date }),
            deadline: t.deadline,
            own_calendar: t
                .assignee_ref
                .as_ref()
                .and_then(|r| index_of.get(r).copied()),
        })
        .collect();
    let mut edges: Vec<Edge> = Vec::new();
    let mut seen: std::collections::HashSet<(Uuid, Uuid)> = std::collections::HashSet::new();
    for d in plan_dependencies::Entity::find()
        .order_by_asc(plan_dependencies::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        if pids.contains(&d.predecessor_pid) && pids.contains(&d.successor_pid) {
            seen.insert((d.predecessor_pid, d.successor_pid));
            edges.push(Edge {
                pid: d.pid,
                pred: d.predecessor_pid,
                succ: d.successor_pid,
                kind: Kind::parse(&d.kind).unwrap_or(Kind::Fs),
                lag: i64::from(d.lag_days),
            });
        }
    }
    for r in task_relations::Entity::find()
        .filter(task_relations::Column::DeletedAt.is_null())
        .filter(task_relations::Column::Kind.eq("precedes"))
        .order_by_asc(task_relations::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        if pids.contains(&r.from_task_pid)
            && pids.contains(&r.to_task_pid)
            && seen.insert((r.from_task_pid, r.to_task_pid))
        {
            edges.push(Edge {
                pid: r.pid,
                pred: r.from_task_pid,
                succ: r.to_task_pid,
                kind: Kind::Fs,
                lag: 0,
            });
        }
    }
    Ok(Network {
        nodes,
        edges,
        rows,
        plan_cal,
        cal_source,
        own,
        conflicts,
    })
}

fn cycle_error(on: &[Uuid]) -> Error {
    unprocessable(&format!(
        "the dependencies contain a cycle, so no schedule can be computed (on or behind it: {})",
        on.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

#[derive(Debug, Deserialize)]
struct CpmParams {
    /// Working days of float that count as near-critical (default 5).
    #[serde(default)]
    near: Option<i64>,
}

/// `GET /api/plans/{pid}/cpm` — float, critical and near-critical tasks, and
/// every critical chain.
#[debug_handler]
async fn plan_cpm(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(p): Query<CpmParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let net = load_network(&ctx, plan.pid).await?;
    let near = p.near.unwrap_or(5);
    if !(0..=cpm::MAX_DAYS).contains(&near) {
        return Err(unprocessable(&format!(
            "near must be 0 to {}",
            cpm::MAX_DAYS
        )));
    }
    let result = net
        .analyze(near)
        .map_err(|cpm::CpmError::Cycle(on)| cycle_error(&on))?;
    let by: HashMap<Uuid, &tasks::Model> = net.rows.iter().map(|t| (t.pid, t)).collect();
    let name = |pid: Uuid| by.get(&pid).map(|t| t.title.clone());
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "calendar": net.cal_source,
        "calendar_conflicts": net.conflicts,
        "project_finish": result.project_finish,
        "nodes": result.nodes.iter().map(|n| {
            let t = by.get(&n.pid);
            let mut v = serde_json::to_value(n).unwrap_or_default();
            v["title"] = serde_json::json!(name(n.pid));
            v["status"] = serde_json::json!(t.map(|t| t.status.clone()));
            v["duration_days"] = serde_json::json!(t.and_then(|t| t.duration_days));
            v["duration_optimistic_days"] = serde_json::json!(t.and_then(|t| t.duration_optimistic_days));
            v["duration_pessimistic_days"] = serde_json::json!(t.and_then(|t| t.duration_pessimistic_days));
            v["constraint_date"] = serde_json::json!(t.and_then(|t| t.constraint_date));
            v["deadline"] = serde_json::json!(t.and_then(|t| t.deadline));
            let own = net.nodes.iter().find(|x| x.pid == n.pid).and_then(|x| x.own_calendar);
            v["calendar"] = serde_json::json!(if own.is_some() { "plan+person" } else { "plan" });
            v
        }).collect::<Vec<_>>(),
        "unscheduled": result.unscheduled.iter().map(|u| serde_json::json!({
            "pid": u.pid, "title": name(u.pid), "reason": u.reason })).collect::<Vec<_>>(),
        "edges": net.edges.iter().map(|e| serde_json::json!({
            "pid": e.pid, "predecessor_pid": e.pred, "successor_pid": e.succ,
            "kind": e.kind, "lag_days": e.lag })).collect::<Vec<_>>(),
        "ignored_edges": result.ignored_edges,
        "critical_chains": result.chains,
        "chains_truncated": result.chains_truncated,
        "note": "Float is computed over the plan's tasks only, in working days of the plan calendar named above (a task whose assignee has their own calendar or recorded absences runs its work on the days both work). Negative float means a constraint or deadline cannot be met; a hard constraint (mso, mfo) overrides logic and a node it pins earlier than its predecessors allow is flagged hard_conflict. A task with no duration, or with no start and no scheduled predecessor, is unscheduled; completed tasks are included and carry their status.",
    }))
}

/// `PUT …/tasks/{pid}/schedule` body. An absent or `null` field **clears** it.
#[derive(Debug, Deserialize)]
struct ScheduleBody {
    #[serde(default)]
    start_date: Option<NaiveDate>,
    #[serde(default)]
    duration_days: Option<i32>,
    /// `snet`, `snlt`, `fnet`, `fnlt`, `mso` or `mfo`; with `constraint_date`.
    #[serde(default)]
    constraint_kind: Option<String>,
    #[serde(default)]
    constraint_date: Option<NaiveDate>,
    /// A soft deadline: late dates only (negative float), never early dates.
    #[serde(default)]
    deadline: Option<NaiveDate>,
    /// Required (and audited) when setting a hard constraint.
    #[serde(default)]
    reason: Option<String>,
}

/// Whether the caller may set a **hard** constraint (`mso` / `mfo`), which
/// overrides the schedule's logic. With authentication off there is no caller to
/// refuse (the deployment's posture for every write); with it on, the token needs
/// `access=admin` or `scheduling=hard_constraints`.
pub(crate) fn may_set_hard_constraint(caller: &MaybeAuthUser) -> bool {
    caller.claims().is_none_or(|c| {
        let has = |key: &str, want: &str| {
            c.attrs
                .get(key)
                .is_some_and(|v| v.iter().any(|a| a == want))
        };
        has("access", "admin") || has("scheduling", "hard_constraints")
    })
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/schedule` — set a task's planned start
/// and working duration; answers with the task and **what moved** (PJ-8).
#[debug_handler]
async fn set_schedule(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<ScheduleBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let t_pid = Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?;
    let mut problems = cpm::schedule_problems(b.start_date, b.duration_days);
    problems.extend(cpm::constraint_problems(
        b.constraint_kind.as_deref(),
        b.constraint_date,
        b.deadline,
    ));
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let kind = b.constraint_kind.as_deref().map(str::to_ascii_lowercase);
    let hard = kind
        .as_deref()
        .and_then(cpm::ConstraintKind::parse)
        .is_some_and(cpm::ConstraintKind::is_hard);
    if hard {
        if !may_set_hard_constraint(&caller) {
            return Err(Error::CustomError(
                StatusCode::FORBIDDEN,
                ErrorDetail::new(
                    "forbidden",
                    "a hard constraint (mso, mfo) overrides the schedule's logic and needs access=admin or scheduling=hard_constraints",
                ),
            ));
        }
        if b.reason.as_deref().is_none_or(|r| r.trim().is_empty()) {
            return Err(unprocessable("a hard constraint needs a reason"));
        }
    }
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(t_pid))
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let analyse = |net: &Network| net.analyze(Options::default().near_critical_days);
    let before = analyse(&load_network(&ctx, plan.pid).await?);
    let was = (
        task.start_date,
        task.duration_days,
        task.constraint_kind.clone(),
        task.constraint_date,
        task.deadline,
    );
    let mut a: tasks::ActiveModel = task.into();
    a.start_date = ActiveValue::set(b.start_date);
    a.duration_days = ActiveValue::set(b.duration_days);
    a.constraint_kind = ActiveValue::set(kind.clone());
    a.constraint_date = ActiveValue::set(b.constraint_date);
    a.deadline = ActiveValue::set(b.deadline);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    let after = analyse(&load_network(&ctx, plan.pid).await?);
    AuditModel::record(
        &ctx.db,
        row.pid,
        "task_schedule_updated",
        caller.actor(),
        Some(serde_json::json!({
            "from": { "start_date": was.0, "duration_days": was.1, "constraint_kind": was.2,
                      "constraint_date": was.3, "deadline": was.4 },
            "to": { "start_date": row.start_date, "duration_days": row.duration_days,
                    "constraint_kind": row.constraint_kind, "constraint_date": row.constraint_date,
                    "deadline": row.deadline },
            "reason": b.reason, "hard": hard,
        })),
    )
    .await
    .ok();
    let (moved, note) = match (&before, &after) {
        (Ok(b), Ok(a)) => (Some(cpm::diff(b, a)), None),
        _ => (
            None,
            Some("the network has a cycle, so what moved cannot be computed"),
        ),
    };
    format::json(serde_json::json!({
        "task": { "pid": row.pid, "start_date": row.start_date, "duration_days": row.duration_days,
                  "constraint_kind": row.constraint_kind, "constraint_date": row.constraint_date,
                  "deadline": row.deadline },
        "moved": moved,
        "note": note,
    }))
}

/// The CPM routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/cpm", get(plan_cpm))
        .add("/plans/{pid}/tasks/{t_pid}/schedule", put(set_schedule))
}

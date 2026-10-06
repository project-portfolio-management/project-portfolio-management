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

use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::ConnectionTrait;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::cpm::{self, Edge, Kind, Node, Options, WorkCalendar};
use crate::models::_entities::{plan_dependencies, plans, task_relations, tasks};
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
    /// Tasks of **other plans** that this plan's tasks depend on or are depended on by
    /// (PJ-13), keyed by task pid. Their nodes follow the plan's own in `nodes`.
    pub external: HashMap<Uuid, ExternalTask>,
    /// Dependencies to a plan the caller may not read: counted and placed, never named.
    pub restricted: Vec<Restricted>,
    /// More plans or tasks were linked than are followed.
    pub truncated: bool,
}

/// A task of another plan, as the plan's network sees it.
#[derive(Debug, Clone)]
pub(crate) struct ExternalTask {
    pub title: String,
    pub plan_pid: Uuid,
    pub plan_name: String,
}

/// A dependency whose other end the caller may not read.
#[derive(Debug, Clone)]
pub(crate) struct Restricted {
    pub edge_pid: Uuid,
    /// This plan's own task on the edge.
    pub task_pid: Uuid,
    /// Whether the hidden side is the `predecessor` or the `successor`.
    pub side: &'static str,
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

fn node_of(t: &tasks::Model, own_calendar: Option<usize>) -> Node {
    Node {
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
        own_calendar,
    }
}

/// The dependencies with at least one end among `ids`: the query is bounded by the plan(s)
/// asked about, never the whole table.
pub(crate) async fn deps_touching(
    ctx: &AppContext,
    ids: Vec<Uuid>,
) -> Result<Vec<plan_dependencies::Model>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    plan_dependencies::Entity::find()
        .filter(
            sea_orm::Condition::any()
                .add(plan_dependencies::Column::PredecessorPid.is_in(ids.clone()))
                .add(plan_dependencies::Column::SuccessorPid.is_in(ids)),
        )
        .order_by_asc(plan_dependencies::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// Load a plan's tasks and the edges among them.
pub(crate) async fn load_network(ctx: &AppContext, plan_pid: Uuid) -> Result<Network> {
    let rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan_pid))
        .filter(tasks::Column::DeletedAt.is_null())
        // A hammock is a derived span, not work: it is not in the network.
        .filter(tasks::Column::IsHammock.eq(false))
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
        .map(|t| {
            node_of(
                t,
                t.assignee_ref
                    .as_ref()
                    .and_then(|r| index_of.get(r).copied()),
            )
        })
        .collect();
    let mut edges: Vec<Edge> = Vec::new();
    let mut seen: std::collections::HashSet<(Uuid, Uuid)> = std::collections::HashSet::new();
    for d in deps_touching(ctx, pids.iter().copied().collect()).await? {
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
        .filter(task_relations::Column::FromTaskPid.is_in(pids.iter().copied().collect::<Vec<_>>()))
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
        external: HashMap::new(),
        restricted: Vec::new(),
        truncated: false,
    })
}

/// The most plans (besides its own) and tasks a cross-plan network follows.
const MAX_LINKED_PLANS: usize = 8;
const MAX_LINKED_TASKS: usize = 3000;

/// The plans among `pids` whose tasks the caller may see across a link: readable by
/// the plan rules, and, for a guest, covered by a live grant.
async fn linkable_plans(ctx: &AppContext, caller: &MaybeAuthUser, pids: &[Uuid]) -> HashSet<Uuid> {
    let mut ok = super::plan_access::readable_plans(ctx, caller, pids).await;
    if super::guests::caller_is_guest(caller) {
        let mut covered = HashSet::new();
        for p in &ok {
            if super::guests::guest_covers_plan(&ctx.db, caller, *p, "read").await {
                covered.insert(*p);
            }
        }
        ok = covered;
    }
    ok
}

/// A plan's network **with the tasks of other plans it is linked to** (PJ-13).
///
/// Dependencies may join tasks of different plans. Those the caller may read on
/// both ends are part of the network: the linked plans' tasks come in as nodes (on
/// their own plan's calendar intersected with this plan's, the same rule as an
/// assignee's calendar), so float is computed across the link. A link to a plan the
/// caller may **not** read is left out of the calculation and recorded only as a
/// [`Restricted`] edge: no name, owner or date of the hidden side is read into the
/// result, so nothing downstream can show it.
#[allow(clippy::too_many_lines)]
pub(crate) async fn load_network_cross(
    ctx: &AppContext,
    plan_pid: Uuid,
    caller: &MaybeAuthUser,
) -> Result<Network> {
    let mut net = load_network(ctx, plan_pid).await?;
    let own: HashSet<Uuid> = net.rows.iter().map(|t| t.pid).collect();
    // Discover the plans reachable through links, cap and all (readability comes after).
    let mut known_tasks: HashMap<Uuid, Uuid> = own.iter().map(|t| (*t, plan_pid)).collect();
    let mut plans_seen: Vec<Uuid> = vec![plan_pid];
    let mut truncated = false;
    for _ in 0..MAX_LINKED_PLANS {
        let deps = deps_touching(ctx, known_tasks.keys().copied().collect()).await?;
        let others: Vec<Uuid> = deps
            .iter()
            .filter(|d| {
                known_tasks.contains_key(&d.predecessor_pid)
                    != known_tasks.contains_key(&d.successor_pid)
            })
            .map(|d| {
                if known_tasks.contains_key(&d.predecessor_pid) {
                    d.successor_pid
                } else {
                    d.predecessor_pid
                }
            })
            .collect();
        if others.is_empty() {
            break;
        }
        let found: Vec<tasks::Model> = tasks::Entity::find()
            .filter(tasks::Column::Pid.is_in(others))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::IsHammock.eq(false))
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
        let fresh: Vec<Uuid> = found
            .iter()
            .map(|t| t.plan_pid)
            .filter(|p| !plans_seen.contains(p))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if fresh.is_empty() {
            for t in &found {
                known_tasks.insert(t.pid, t.plan_pid);
            }
            break;
        }
        for p in fresh {
            if plans_seen.len() > MAX_LINKED_PLANS {
                truncated = true;
                break;
            }
            plans_seen.push(p);
        }
        let all: Vec<tasks::Model> = tasks::Entity::find()
            .filter(tasks::Column::PlanPid.is_in(plans_seen.clone()))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::IsHammock.eq(false))
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
        for t in &all {
            known_tasks.insert(t.pid, t.plan_pid);
        }
        if known_tasks.len() > MAX_LINKED_TASKS {
            truncated = true;
            break;
        }
    }
    let deps = deps_touching(ctx, known_tasks.keys().copied().collect()).await?;
    let candidates: Vec<Uuid> = plans_seen
        .iter()
        .copied()
        .filter(|p| *p != plan_pid)
        .collect();
    let readable = linkable_plans(ctx, caller, &candidates).await;
    // Dependencies from this plan's tasks to a plan the caller may not read: place, never name.
    for d in &deps {
        let (ours, other, side) =
            if own.contains(&d.predecessor_pid) && !own.contains(&d.successor_pid) {
                (d.predecessor_pid, d.successor_pid, "successor")
            } else if own.contains(&d.successor_pid) && !own.contains(&d.predecessor_pid) {
                (d.successor_pid, d.predecessor_pid, "predecessor")
            } else {
                continue;
            };
        if let Some(other_plan) = known_tasks.get(&other)
            && candidates.contains(other_plan)
            && !readable.contains(other_plan)
        {
            net.restricted.push(Restricted {
                edge_pid: d.pid,
                task_pid: ours,
                side,
            });
        }
    }
    let ext_plans: Vec<Uuid> = candidates
        .iter()
        .copied()
        .filter(|p| readable.contains(p))
        .collect();
    if ext_plans.is_empty() {
        net.truncated = truncated;
        return Ok(net);
    }
    let names: HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(ext_plans.clone()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    for p in &ext_plans {
        let (plan_cal, _) = super::calendars::plan_calendar(ctx, *p).await?;
        let shared = net.plan_cal.intersect(&plan_cal);
        let idx = if let Ok(c) = shared {
            net.own.push(c);
            Some(net.own.len() - 1)
        } else {
            net.conflicts.push(format!("plan:{p}"));
            None
        };
        let rows = tasks::Entity::find()
            .filter(tasks::Column::PlanPid.eq(*p))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::IsHammock.eq(false))
            .order_by_asc(tasks::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?;
        for t in rows {
            net.nodes.push(node_of(&t, idx));
            net.external.insert(
                t.pid,
                ExternalTask {
                    title: t.title.clone(),
                    plan_pid: *p,
                    plan_name: names.get(p).cloned().unwrap_or_default(),
                },
            );
        }
    }
    // Edges among everything in the network that the plan's own loader did not already take.
    let in_net: HashSet<Uuid> = own
        .iter()
        .copied()
        .chain(net.external.keys().copied())
        .collect();
    let have: HashSet<Uuid> = net.edges.iter().map(|e| e.pid).collect();
    for d in &deps {
        if in_net.contains(&d.predecessor_pid)
            && in_net.contains(&d.successor_pid)
            && !have.contains(&d.pid)
        {
            net.edges.push(Edge {
                pid: d.pid,
                pred: d.predecessor_pid,
                succ: d.successor_pid,
                kind: Kind::parse(&d.kind).unwrap_or(Kind::Fs),
                lag: i64::from(d.lag_days),
            });
        }
    }
    net.truncated = truncated;
    Ok(net)
}

/// The nodes among `pids` (task or plan pids) the caller may read: a node is readable
/// when the plan it is, or belongs to, is readable (and, for a guest, granted).
pub(crate) async fn readable_endpoints(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pids: &[Uuid],
) -> Result<HashSet<Uuid>> {
    if pids.is_empty() {
        return Ok(HashSet::new());
    }
    let task_plan: HashMap<Uuid, Uuid> = tasks::Entity::find()
        .filter(tasks::Column::Pid.is_in(pids.to_vec()))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|t| (t.pid, t.plan_pid))
        .collect();
    let mut plan_ids: Vec<Uuid> = pids.to_vec();
    plan_ids.extend(task_plan.values().copied());
    plan_ids.sort();
    plan_ids.dedup();
    let ok = linkable_plans(ctx, caller, &plan_ids).await;
    Ok(pids
        .iter()
        .copied()
        .filter(|p| match task_plan.get(p) {
            Some(plan) => ok.contains(plan),
            None => ok.contains(p),
        })
        .collect())
}

/// The other plans whose tasks are linked to this plan's, for deciding whether it
/// can be deleted: `(readable plans with their names, how many the caller may not read)`.
pub(crate) async fn external_dependants(
    ctx: &AppContext,
    plan_pid: Uuid,
    caller: &MaybeAuthUser,
) -> Result<(Vec<(Uuid, String)>, usize)> {
    let own: HashSet<Uuid> = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan_pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|t| t.pid)
        .collect();
    if own.is_empty() {
        return Ok((Vec::new(), 0));
    }
    let others: Vec<Uuid> = deps_touching(ctx, own.iter().copied().collect())
        .await?
        .into_iter()
        .filter_map(|d| {
            match (
                own.contains(&d.predecessor_pid),
                own.contains(&d.successor_pid),
            ) {
                (true, false) => Some(d.successor_pid),
                (false, true) => Some(d.predecessor_pid),
                _ => None,
            }
        })
        .collect();
    let other_plans: HashSet<Uuid> = tasks::Entity::find()
        .filter(tasks::Column::Pid.is_in(others))
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::PlanPid.ne(plan_pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|t| t.plan_pid)
        .collect();
    let live: Vec<plans::Model> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(other_plans.into_iter().collect::<Vec<_>>()))
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let pids: Vec<Uuid> = live.iter().map(|p| p.pid).collect();
    let readable = linkable_plans(ctx, caller, &pids).await;
    let named: Vec<(Uuid, String)> = live
        .iter()
        .filter(|p| readable.contains(&p.pid))
        .map(|p| (p.pid, p.name.clone()))
        .collect();
    Ok((named, live.len() - readable.len().min(live.len())))
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
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(p): Query<CpmParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let net = load_network_cross(&ctx, plan.pid, &caller).await?;
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
    let name = |pid: Uuid| {
        by.get(&pid)
            .map(|t| t.title.clone())
            .or_else(|| net.external.get(&pid).map(|x| x.title.clone()))
    };
    // The plan's own finish, and the finish of everything it is linked to.
    let own_finish = result
        .nodes
        .iter()
        .filter(|n| by.contains_key(&n.pid))
        .map(|n| n.early_finish)
        .max();
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "calendar": net.cal_source,
        "calendar_conflicts": net.conflicts,
        "project_finish": if net.external.is_empty() { result.project_finish } else { own_finish },
        "network_finish": result.project_finish,
        "nodes": result.nodes.iter().map(|n| {
            let t = by.get(&n.pid);
            let mut v = serde_json::to_value(n).unwrap_or_default();
            v["title"] = serde_json::json!(name(n.pid));
            v["status"] = serde_json::json!(t.map(|t| t.status.clone()));
            v["duration_days"] = serde_json::json!(t.and_then(|t| t.duration_days));
            v["duration_optimistic_days"] = serde_json::json!(t.and_then(|t| t.duration_optimistic_days));
            v["duration_pessimistic_days"] = serde_json::json!(t.and_then(|t| t.duration_pessimistic_days));
            v["work_minutes"] = serde_json::json!(t.and_then(|t| t.work_minutes));
            v["units_percent"] = serde_json::json!(t.and_then(|t| t.units_percent));
            v["scheduling_rule"] = serde_json::json!(t.and_then(|t| t.scheduling_rule.clone()));
            v["contour"] = serde_json::json!(t.map(|t| t.contour.clone()));
            v["constraint_date"] = serde_json::json!(t.and_then(|t| t.constraint_date));
            v["deadline"] = serde_json::json!(t.and_then(|t| t.deadline));
            let own = net.nodes.iter().find(|x| x.pid == n.pid).and_then(|x| x.own_calendar);
            v["calendar"] = serde_json::json!(if own.is_some() { "plan+person" } else { "plan" });
            match net.external.get(&n.pid) {
                Some(x) => {
                    v["external"] = serde_json::json!(true);
                    v["plan_pid"] = serde_json::json!(x.plan_pid);
                    v["plan_name"] = serde_json::json!(x.plan_name);
                }
                None => v["external"] = serde_json::json!(false),
            }
            v
        }).collect::<Vec<_>>(),
        "unscheduled": result.unscheduled.iter().map(|u| serde_json::json!({
            "pid": u.pid, "title": name(u.pid), "reason": u.reason })).collect::<Vec<_>>(),
        "edges": net.edges.iter().map(|e| serde_json::json!({
            "pid": e.pid, "predecessor_pid": e.pred, "successor_pid": e.succ,
            "kind": e.kind, "lag_days": e.lag,
            "cross_plan": net.external.contains_key(&e.pred) != net.external.contains_key(&e.succ) })).collect::<Vec<_>>(),
        "restricted_edges": net.restricted.iter().map(|r| serde_json::json!({
            "pid": r.edge_pid, "task_pid": r.task_pid, "side": r.side,
            "label": "external dependency (restricted)" })).collect::<Vec<_>>(),
        "external_truncated": net.truncated,
        "hammocks": super::hammock::spans_for(&ctx, plan.pid, Some(&result)).await?,
        "ignored_edges": result.ignored_edges,
        "critical_chains": result.chains,
        "chains_truncated": result.chains_truncated,
        "note": "Float is computed over the plan's tasks and the tasks of other plans they are linked to (cross_plan edges, external nodes, each on its own plan's calendar intersected with this one); a link to a plan you cannot read is left out of the calculation and shown only as a restricted edge, with nothing about the other side. Float is computed over the plan's tasks only, in working days of the plan calendar named above (a task whose assignee has their own calendar or recorded absences runs its work on the days both work). Negative float means a constraint or deadline cannot be met; a hard constraint (mso, mfo) overrides logic and a node it pins earlier than its predecessors allow is flagged hard_conflict. A task with no duration, or with no start and no scheduled predecessor, is unscheduled; completed tasks are included and carry their status.",
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
#[allow(clippy::too_many_lines)]
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
    if task.is_hammock {
        return Err(unprocessable(
            "a hammock's span is derived from the tasks it links to: it has no schedule of its own to set",
        ));
    }
    let analyse = |net: &Network| net.analyze(Options::default().near_critical_days);
    let before = analyse(&load_network(&ctx, plan.pid).await?);
    // A task that carries an assignment (units and work) has its third quantity
    // recomputed, per its rule, when its duration changes (PJ-6).
    let assignment = super::assignments::recompute_for_duration(&task, b.duration_days)
        .map_err(|m| unprocessable(&m))?;
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
    if let Some(r) = &assignment {
        a.units_percent = ActiveValue::set(i32::try_from(r.triple.units_percent).ok());
        a.work_minutes = ActiveValue::set(Some(r.triple.work_minutes));
    }
    // The change and its audit row (with the reason) land together or not at all.
    let txn = ctx.db.begin().await.map_err(db_err)?;
    // The same lock a what-if commit holds: a plain edit cannot slip in between its freshness check and its writes.
    txn.execute_unprepared(&format!(
        "SELECT pg_advisory_xact_lock(hashtext('sandbox:{}'))",
        plan.pid
    ))
    .await
    .map_err(db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
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
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    let after = analyse(&load_network(&ctx, plan.pid).await?);
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
        // Present when the task carries work: what the rule recomputed.
        "assignment": assignment.map(|r| serde_json::json!({
            "derived": r.derived, "exact": r.exact,
            "units_percent": r.triple.units_percent, "work_minutes": r.triple.work_minutes,
            "duration_days": r.triple.duration_days })),
    }))
}

/// The CPM routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/cpm", get(plan_cpm))
        .add("/plans/{pid}/tasks/{t_pid}/schedule", put(set_schedule))
}

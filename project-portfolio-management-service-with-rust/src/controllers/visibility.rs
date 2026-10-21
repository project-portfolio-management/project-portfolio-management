//! PPM Phase-B visibility controllers (spec/15-roadmap PPM-6/7/8/9).
//!
//! - cross-item dependencies + the portfolio schedule view
//!   (violations + critical path)
//! - milestones, resource allocations + the capacity rollup
//! - saved reports, and the ETag-conditional at-a-glance dashboard
//!
//! Pure rules live in [`crate::visibility`]; nothing here feeds the
//! matcher (§8 partition rule).

use super::unprocessable;
use axum::http::{HeaderMap, StatusCode, header};
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use std::fmt::Write as _;
use uuid::Uuid;

use super::plans::parse_kind_label;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    allocations, budget_lines, milestones, plan_dependencies, plans, report_definitions, risks,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::validation::{MAX_ARRAY_LEN, MAX_ITEM_LEN, MAX_TEXT_LEN};
use crate::visibility as rules;
use crate::workers_client;

/// Find an active plan by pid across **any** collection (edges
/// may span kinds).
async fn find_any_item(ctx: &AppContext, pid: Uuid) -> Result<plans::Model> {
    plans::Entity::find()
        .filter(plans::Column::Pid.eq(pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .ok_or(Error::NotFound)
}

/// The schedule facts (parsed flexible dates) of one stored item.
pub(crate) fn schedule_item(model: &plans::Model) -> rules::ScheduleItem {
    let start = model.data.get("start_date").and_then(|v| v.as_str());
    let end = model.data.get("target_date").and_then(|v| v.as_str());
    rules::ScheduleItem {
        pid: model.pid,
        start: start.and_then(|s| rules::parse_flex_date(s, false)),
        end: end.and_then(|s| rules::parse_flex_date(s, true)),
    }
}

/// Whether the stored item is finished (Completed / Cancelled).
pub(crate) fn is_finished(model: &plans::Model) -> bool {
    matches!(
        model.data.get("status").and_then(|v| v.as_str()),
        Some("Completed" | "Cancelled")
    )
}

/// `POST /api/dependencies` body: a dependency between two plans, two tasks,
/// or one of each (SC-1): a relationship `kind` (`fs` default, `ss`, `ff`,
/// `sf`) and a lag in business days, **negative for a lead**.
#[derive(Debug, Deserialize)]
struct DependencyPayload {
    predecessor_pid: Uuid,
    successor_pid: Uuid,
    #[serde(default)]
    lag_days: i32,
    #[serde(default)]
    kind: Option<String>,
}

/// Whether `pid` is a live plan or a live task (a dependency endpoint).
async fn node_exists(ctx: &AppContext, pid: Uuid) -> Result<bool> {
    if plans::Entity::find()
        .filter(plans::Column::Pid.eq(pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .is_some()
    {
        return Ok(true);
    }
    Ok(crate::models::_entities::tasks::Entity::find()
        .filter(crate::models::_entities::tasks::Column::Pid.eq(pid))
        .filter(crate::models::_entities::tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .is_some())
}

/// `POST /api/dependencies` — add an edge (both items must exist,
/// any collection; self-edges, duplicates, and cycles refuse).
#[debug_handler]
async fn create_dependency(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<DependencyPayload>,
) -> Result<Response> {
    if !(-3650..=3650).contains(&payload.lag_days) {
        return Err(unprocessable(
            "lag_days must be -3650 to 3650 (negative is a lead)",
        ));
    }
    let kind = payload.kind.as_deref().unwrap_or("fs").to_ascii_lowercase();
    if crate::cpm::Kind::parse(&kind).is_none() {
        return Err(unprocessable("kind must be one of fs, ss, ff, sf"));
    }
    for end in [payload.predecessor_pid, payload.successor_pid] {
        if !node_exists(&ctx, end).await? {
            return Err(Error::NotFound);
        }
    }
    // A plan or task the caller cannot read does not exist for them: no probing for ids.
    let readable = super::cpm::readable_endpoints(
        &ctx,
        &caller,
        &[payload.predecessor_pid, payload.successor_pid],
    )
    .await?;
    if !readable.contains(&payload.predecessor_pid) || !readable.contains(&payload.successor_pid) {
        return Err(Error::NotFound);
    }
    // A hammock is a derived span, not work: it is linked through hammock links, never dependencies.
    if crate::models::_entities::tasks::Entity::find()
        .filter(crate::models::_entities::tasks::Column::IsHammock.eq(true))
        .filter(
            crate::models::_entities::tasks::Column::Pid
                .is_in([payload.predecessor_pid, payload.successor_pid]),
        )
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .is_some()
    {
        return Err(unprocessable(
            "a hammock is a derived span, not work: link it with hammock links, not a dependency",
        ));
    }
    let existing = vis_models::all_dependencies(&ctx.db).await?;
    if existing.iter().any(|e| {
        e.predecessor_pid == payload.predecessor_pid && e.successor_pid == payload.successor_pid
    }) {
        return Err(unprocessable("this dependency already exists"));
    }
    let mut edges: Vec<(Uuid, Uuid)> = existing
        .iter()
        .map(|e| (e.predecessor_pid, e.successor_pid))
        .collect();
    // Task `precedes` relations are schedule edges too: a cycle through them
    // is a cycle.
    edges.extend(
        crate::models::_entities::task_relations::Entity::find()
            .filter(crate::models::_entities::task_relations::Column::DeletedAt.is_null())
            .filter(crate::models::_entities::task_relations::Column::Kind.eq("precedes"))
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?
            .into_iter()
            .map(|r| (r.from_task_pid, r.to_task_pid)),
    );
    if rules::would_create_cycle(&edges, payload.predecessor_pid, payload.successor_pid) {
        return Err(unprocessable("this dependency would create a cycle"));
    }
    let row = plan_dependencies::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        predecessor_pid: ActiveValue::set(payload.predecessor_pid),
        successor_pid: ActiveValue::set(payload.successor_pid),
        lag_days: ActiveValue::set(payload.lag_days),
        kind: ActiveValue::set(kind),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row.pid, "dependency_added", caller.actor(), None).await;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/dependencies?item={pid}` — edges touching one item (or
/// all, capped 500).
#[derive(Debug, Deserialize)]
struct DependencyParams {
    #[serde(default)]
    item: Option<Uuid>,
}

#[debug_handler]
async fn list_dependencies(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<DependencyParams>,
) -> Result<Response> {
    // Filtering by an item the caller cannot read answers exactly as for an item with no
    // links: an empty list, so the filter cannot be used to probe for ids.
    if let Some(item) = params.item
        && !super::cpm::readable_endpoints(&ctx, &caller, &[item])
            .await?
            .contains(&item)
    {
        return format::json(Vec::<serde_json::Value>::new());
    }
    let rows = vis_models::all_dependencies(&ctx.db).await?;
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|e| {
            params
                .item
                .is_none_or(|pid| e.predecessor_pid == pid || e.successor_pid == pid)
        })
        .take(500)
        .collect();
    // An edge is shown only when the caller can read **both** ends; otherwise it is
    // an opaque "external dependency (restricted)" with nothing about the hidden side.
    let ends: Vec<Uuid> = rows
        .iter()
        .flat_map(|e| [e.predecessor_pid, e.successor_pid])
        .collect();
    let readable = super::cpm::readable_endpoints(&ctx, &caller, &ends).await?;
    let out: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|e| {
            if readable.contains(&e.predecessor_pid) && readable.contains(&e.successor_pid) {
                serde_json::to_value(&e).unwrap_or_default()
            } else {
                serde_json::json!({ "pid": e.pid, "restricted": true,
                    "label": "external dependency (restricted)" })
            }
        })
        .collect();
    format::json(out)
}

/// `DELETE /api/dependencies/{pid}`.
#[debug_handler]
async fn delete_dependency(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let row = vis_models::find_dependency(&ctx.db, pid).await?;
    // Removing a link needs to be able to read both ends; otherwise it is as if it were not there.
    let readable =
        super::cpm::readable_endpoints(&ctx, &caller, &[row.predecessor_pid, row.successor_pid])
            .await?;
    if !readable.contains(&row.predecessor_pid) || !readable.contains(&row.successor_pid) {
        return Err(Error::NotFound);
    }
    let row_pid = row.pid;
    plan_dependencies::Entity::delete_by_id(row.id)
        .exec(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row_pid, "dependency_removed", caller.actor(), None).await;
    format::empty_json()
}

/// `GET /api/plans/{pid}/schedule` — the roadmap view for one plan: its
/// children's timeframes, the dependency edges among them, finish-start
/// violations, and the critical path. Any plan may contain children, so
/// this is no longer restricted to a `Portfolio`-kind plan.
#[debug_handler]
async fn portfolio_schedule(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let parent_pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let portfolio = find_any_item(&ctx, parent_pid).await?;
    let members = plans::Entity::find()
        .filter(plans::Column::ParentPid.eq(parent_pid))
        .filter(plans::Column::DeletedAt.is_null())
        .order_by_asc(plans::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    // A child plan the caller may not see is not on their roadmap.
    let mut members = scope.keep(members);
    members.insert(0, portfolio);
    let facts: Vec<rules::ScheduleItem> = members.iter().map(schedule_item).collect();
    let member_pids: std::collections::HashSet<Uuid> = facts.iter().map(|f| f.pid).collect();
    let edges: Vec<rules::ScheduleEdge> = vis_models::all_dependencies(&ctx.db)
        .await?
        .into_iter()
        .filter(|e| {
            member_pids.contains(&e.predecessor_pid) && member_pids.contains(&e.successor_pid)
        })
        .map(|e| rules::ScheduleEdge {
            pid: e.pid,
            predecessor: e.predecessor_pid,
            successor: e.successor_pid,
            lag_days: e.lag_days,
            kind: crate::cpm::Kind::parse(&e.kind).unwrap_or(crate::cpm::Kind::Fs),
        })
        .collect();
    let violations = rules::violations(&facts, &edges);
    let critical = rules::critical_path(&facts, &edges);
    let (plan_cal, cal_source) = super::calendars::plan_calendar(&ctx, parent_pid).await?;
    let cpm_block = plan_level_cpm(&members, &facts, &edges, &plan_cal, cal_source);
    let items: Vec<_> = members
        .iter()
        .zip(&facts)
        .map(|(m, f)| {
            serde_json::json!({
                "pid": m.pid.to_string(), "kind": m.kind.as_deref().unwrap_or("unspecified"), "name": m.name,
                "stage": m.stage, "start": f.start, "end": f.end,
                "on_critical_path": critical.contains(&m.pid),
            })
        })
        .collect();
    format::json(serde_json::json!({
        "parent_pid": parent_pid.to_string(),
        "items": items,
        "edges": edges.iter().map(|e| serde_json::json!({
            "pid": e.pid.to_string(),
            "predecessor_pid": e.predecessor.to_string(),
            "successor_pid": e.successor.to_string(),
            "lag_days": e.lag_days,
            "kind": e.kind,
        })).collect::<Vec<_>>(),
        "cpm": cpm_block,
        "violations": violations,
        "critical_path": critical.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "undated": facts.iter().filter(|f| f.start.is_none() || f.end.is_none())
            .map(|f| f.pid.to_string()).collect::<Vec<_>>(),
    }))
}

/// The float-based critical path over a plan's **children** (the container is a
/// summary, not work, so it is not a node). Plan durations are the business days
/// between the parsed start and target (all seven days work until calendars,
/// SC-2); lag and kind come from the dependency. `critical_path` above stays the
/// longest dependent chain (now lag-aware); this block is the float-based answer
/// and the one to trust when they differ.
fn plan_level_cpm(
    members: &[plans::Model],
    facts: &[rules::ScheduleItem],
    edges: &[rules::ScheduleEdge],
    calendar: &crate::cpm::WorkCalendar,
    calendar_source: &str,
) -> serde_json::Value {
    use crate::cpm::{self, Edge, Node, Options};
    let container = members.first().map(|m| m.pid);
    let nodes: Vec<Node> = facts
        .iter()
        .filter(|f| Some(f.pid) != container)
        .map(|f| Node {
            pid: f.pid,
            duration: match (f.start, f.end) {
                (Some(s), Some(e)) if e >= s => Some((e - s).num_days() + 1),
                _ => None,
            },
            planned_start: f.start,
            ..Node::default()
        })
        .collect();
    let cpm_edges: Vec<Edge> = edges
        .iter()
        .map(|e| Edge {
            pid: e.pid,
            pred: e.predecessor,
            succ: e.successor,
            kind: e.kind,
            lag: i64::from(e.lag_days),
        })
        .collect();
    match cpm::analyze(&nodes, &cpm_edges, calendar, Options::default()) {
        Ok(r) => serde_json::json!({
            "calendar": calendar_source,
            "project_finish": r.project_finish,
            "nodes": r.nodes.iter().map(|n| serde_json::json!({
                "pid": n.pid, "total_float": n.total_float, "free_float": n.free_float,
                "critical": n.critical, "near_critical": n.near_critical,
                "early_start": n.early_start, "early_finish": n.early_finish,
                "pushed_days": n.pushed_days })).collect::<Vec<_>>(),
            "unscheduled": r.unscheduled,
            "critical_chains": r.chains,
        }),
        Err(cpm::CpmError::Cycle(on)) => serde_json::json!({ "error": "cycle", "on": on }),
    }
}

/// `POST /api/plans/{pid}/milestones` body.
#[derive(Debug, Deserialize)]
struct MilestonePayload {
    name: String,
    due: chrono::NaiveDate,
    /// Optional kind (`milestone` / `demo` / `release` / `checkpoint`);
    /// absent reads as `milestone`.
    #[serde(default)]
    kind: Option<String>,
}

#[debug_handler]
async fn create_milestone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<MilestonePayload>,
) -> Result<Response> {
    if payload.name.trim().is_empty() || payload.name.len() > MAX_TEXT_LEN {
        return Err(unprocessable("name is required (and capped)"));
    }
    if let Some(kind) = payload.kind.as_deref()
        && !crate::engineering::MILESTONE_KINDS.contains(&kind)
    {
        return Err(unprocessable(&format!(
            "kind must be one of {:?}",
            crate::engineering::MILESTONE_KINDS
        )));
    }
    let item = super::governance::find_item(&ctx, &pid).await?;
    let row = milestones::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(item.pid),
        name: ActiveValue::set(payload.name.clone()),
        due: ActiveValue::set(payload.due),
        kind: ActiveValue::set(payload.kind.clone()),
        done: ActiveValue::set(false),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row.pid, "milestone_added", caller.actor(), None).await;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/plans/{pid}/milestones` — due-date order, with
/// overdue flags.
#[debug_handler]
async fn list_milestones(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let today = chrono::Utc::now().date_naive();
    let rows = vis_models::milestones_for(&ctx.db, item.pid).await?;
    let views: Vec<_> = rows
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "pid": m.pid.to_string(), "name": m.name, "due": m.due,
                "kind": m.kind.as_deref().unwrap_or("milestone"),
                "done": m.done, "overdue": !m.done && m.due < today,
            })
        })
        .collect();
    format::json(views)
}

/// `PUT /api/plans/{pid}/milestones/{m_pid}` body.
#[derive(Debug, Deserialize)]
struct MilestoneReschedulePayload {
    due: chrono::NaiveDate,
    /// Why it moved; kept in the slip history (optional, up to 500 characters).
    #[serde(default)]
    reason: Option<String>,
}

/// `PUT /api/plans/{pid}/milestones/{m_pid}` — reschedule a milestone's
/// `due` date.
///
/// Added for T-28g: the `milestone_due_changed` trigger needs a real
/// write path, and none existed before this — a milestone could be
/// created and completed, never moved. Fires the trigger only when
/// `due` actually changes, mirroring `plans::update`'s
/// `plan_timeframe_changed` firing.
#[debug_handler]
async fn reschedule_milestone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    Json(payload): Json<MilestoneReschedulePayload>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let m_pid = Uuid::parse_str(&m_pid).map_err(|_| Error::NotFound)?;
    let milestone = vis_models::find_milestone(&ctx.db, m_pid).await?;
    if milestone.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    if payload
        .reason
        .as_deref()
        .is_some_and(|r| r.chars().count() > 500)
    {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    let old_due = milestone.due;
    let row_pid = milestone.pid;
    let mut active: milestones::ActiveModel = milestone.into();
    active.due = ActiveValue::set(payload.due);
    let row = active
        .update(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    // The slip history: one append-only row per actual move.
    if old_due != payload.due {
        crate::models::_entities::milestone_slips::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            milestone_pid: ActiveValue::set(row_pid),
            from_due: ActiveValue::set(old_due),
            to_due: ActiveValue::set(payload.due),
            reason: ActiveValue::set(
                payload
                    .reason
                    .as_deref()
                    .map(str::trim)
                    .filter(|r| !r.is_empty())
                    .map(str::to_string),
            ),
            actor: ActiveValue::set(caller.actor().map(str::to_string)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    }
    AuditModel::record_or_log(
        &ctx.db,
        row_pid,
        "milestone_rescheduled",
        caller.actor(),
        None,
    )
    .await;

    if old_due != payload.due {
        super::automation::fire(
            &ctx,
            &crate::automation::TriggerFact {
                kind: "milestone_due_changed".to_string(),
                plan_pid: item.pid,
                from_status: Some(old_due.to_string()),
                to_status: Some(payload.due.to_string()),
            },
            "milestone",
            row_pid,
            caller.actor(),
        )
        .await;
    }

    format::json(serde_json::json!({
        "pid": row.pid.to_string(), "name": row.name, "due": row.due,
    }))
}

/// `POST /api/plans/{pid}/milestones/{m_pid}/complete` body (optional).
#[derive(Debug, Deserialize, Default)]
struct MilestoneCompletePayload {
    /// Why. **Required** when the milestone is linked to tasks that are not all
    /// done: completing it by hand over open work is allowed, but not silently.
    #[serde(default)]
    reason: Option<String>,
}

/// `POST /api/plans/{pid}/milestones/{m_pid}/complete` — mark a milestone done.
/// The body is optional; a linked milestone with open tasks needs a `reason`.
#[debug_handler]
async fn complete_milestone(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    body: axum::body::Bytes,
) -> Result<Response> {
    let payload: MilestoneCompletePayload = if body.iter().all(u8::is_ascii_whitespace) {
        MilestoneCompletePayload::default()
    } else {
        serde_json::from_slice(&body)
            .map_err(|_| unprocessable("body must be JSON like {\"reason\": \"…\"}"))?
    };
    let item = super::governance::find_item(&ctx, &pid).await?;
    let m_pid = Uuid::parse_str(&m_pid).map_err(|_| Error::NotFound)?;
    let milestone = vis_models::find_milestone(&ctx.db, m_pid).await?;
    if milestone.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    let reason = payload
        .reason
        .as_deref()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(str::to_string);
    if reason.as_deref().is_some_and(|r| r.chars().count() > 500) {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    // A linked milestone with open tasks may be completed by hand, but only
    // with a reason that says so.
    let progress = super::milestones::linked_progress(&ctx, &milestone).await?;
    let open = progress.iter().filter(|c| !**c).count();
    if milestone.completion_mode == "linked" && open > 0 && reason.is_none() {
        return Err(unprocessable(&format!(
            "{open} of {} linked tasks are not done; give a reason to complete the milestone anyway",
            progress.len()
        )));
    }
    let row_pid = milestone.pid;
    let first_stamp = milestone.done_at;
    let mut active: milestones::ActiveModel = milestone.into();
    active.done = ActiveValue::set(true);
    // Stamp the completion instant for the delivery-flow metrics
    // (idempotent completes keep the first stamp).
    active.done_at = ActiveValue::set(Some(
        first_stamp.unwrap_or_else(|| chrono::Utc::now().into()),
    ));
    if reason.is_some() {
        active.done_reason = ActiveValue::set(reason.clone());
    }
    let row = active
        .update(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(
        &ctx.db,
        row_pid,
        "milestone_completed",
        caller.actor(),
        reason
            .as_ref()
            .map(|r| serde_json::json!({ "reason": r, "open_linked_tasks": open })),
    )
    .await;
    format::json(row)
}

/// `POST /api/plans/{pid}/allocations` body.
#[derive(Debug, Deserialize)]
struct AllocationPayload {
    /// `person:` / `worker:` `EntityRef` URN. Omit it with `placeholder: true`.
    #[serde(default)]
    person_ref: String,
    /// Role-only demand with no person yet (SC-7): needs a `role`, takes no
    /// `person_ref`, and is resolved to a named person later.
    #[serde(default)]
    placeholder: bool,
    #[serde(default)]
    role: Option<String>,
    /// Percent of the person's capacity (1–100).
    percent: i32,
    #[serde(default)]
    start_date: Option<chrono::NaiveDate>,
    #[serde(default)]
    end_date: Option<chrono::NaiveDate>,
    /// Short skill tags this allocation requires (T-28c). Only the
    /// requirement tags land here — never any resolved skill data.
    #[serde(default)]
    skills_required: Vec<String>,
    /// `committed` (default, firm) or `proposed` (hoped for; PJ-4). A proposal
    /// never counts toward committed utilization or over-allocation.
    #[serde(default)]
    booking: Option<String>,
    /// Asked of the resource manager (PJ-4).
    #[serde(default)]
    requested: bool,
}

#[debug_handler]
async fn create_allocation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<AllocationPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if payload.placeholder {
        if !payload.person_ref.is_empty() {
            problems.push("a placeholder takes no person_ref".to_string());
        }
        if payload.role.as_deref().is_none_or(|r| r.trim().is_empty()) {
            problems.push("a placeholder needs a role".to_string());
        }
    } else if !super::governance::valid_ref(&payload.person_ref, &["person", "worker"]) {
        problems.push("person_ref must be a person:/worker: URN".to_string());
    }
    if !(1..=100).contains(&payload.percent) {
        problems.push(format!("percent must be 1–100, got {}", payload.percent));
    }
    if let (Some(start), Some(end)) = (payload.start_date, payload.end_date)
        && end < start
    {
        problems.push("end_date is before start_date".to_string());
    }
    if payload
        .booking
        .as_deref()
        .is_some_and(|b| !["proposed", "committed"].contains(&b))
    {
        problems.push("booking must be proposed or committed".to_string());
    }
    if payload.skills_required.len() > MAX_ARRAY_LEN {
        problems.push(format!("skills_required: exceeds {MAX_ARRAY_LEN} entries"));
    }
    for (i, tag) in payload.skills_required.iter().enumerate() {
        if tag.trim().is_empty() {
            problems.push(format!("skills_required[{i}]: must not be blank"));
        } else if tag.chars().count() > MAX_ITEM_LEN {
            problems.push(format!(
                "skills_required[{i}]: exceeds {MAX_ITEM_LEN} characters"
            ));
        }
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let item = super::governance::find_item(&ctx, &pid).await?;
    if payload.booking.as_deref() != Some("proposed") {
        super::register::guard(&ctx, &caller, item.pid, "booking a firm allocation").await?;
    }
    let row = allocations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(item.pid),
        person_ref: ActiveValue::set(if payload.placeholder {
            format!(
                "{}{}",
                crate::portfolio_resources::PLACEHOLDER_PREFIX,
                Uuid::new_v4()
            )
        } else {
            payload.person_ref.clone()
        }),
        role: ActiveValue::set(payload.role.clone()),
        percent: ActiveValue::set(payload.percent),
        booking: ActiveValue::set(
            payload
                .booking
                .clone()
                .unwrap_or_else(|| "committed".to_string()),
        ),
        requested: ActiveValue::set(payload.requested),
        start_date: ActiveValue::set(payload.start_date),
        end_date: ActiveValue::set(payload.end_date),
        deleted_at: ActiveValue::set(None),
        skills_required: ActiveValue::set(serde_json::json!(payload.skills_required)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row.pid, "allocation_added", caller.actor(), None).await;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/plans/{pid}/skill-gap` — for every allocation on this
/// plan carrying `skills_required` tags, resolve the assigned
/// person's held skills live against the worker service (by
/// `EntityRef`, TTL-cached, T-28c) and report each tag's status.
#[debug_handler]
async fn skill_gap(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let rows = vis_models::allocations_for(&ctx.db, item.pid).await?;
    let base_url = std::env::var(workers_client::BASE_URL_ENV).ok();
    let mut findings = Vec::new();
    for row in &rows {
        let tags: Vec<String> =
            serde_json::from_value(row.skills_required.clone()).unwrap_or_default();
        // A placeholder has no person whose skills could be held yet.
        if tags.is_empty() || crate::portfolio_resources::is_placeholder(&row.person_ref) {
            continue;
        }
        let resolved = workers_client::resolve_skills(base_url.as_deref(), &row.person_ref).await;
        let (held, reason) = match &resolved {
            Ok(held) => (Some(held.as_slice()), None),
            Err(reason) => (None, Some(reason.as_str())),
        };
        findings.extend(rules::skill_gap(
            row.pid,
            &row.person_ref,
            &tags,
            held,
            reason,
        ));
    }
    format::json(serde_json::json!({ "plan_pid": item.pid.to_string(), "findings": findings }))
}

/// `GET /api/plans/{pid}/allocations`.
#[debug_handler]
async fn list_allocations(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    format::json(vis_models::allocations_for(&ctx.db, item.pid).await?)
}

/// `POST …/allocations/{a_pid}/commit` query.
#[derive(Debug, Deserialize, Default)]
struct CommitBody {
    /// Commit even though it would push the person over their capacity.
    #[serde(default)]
    confirm: bool,
}

/// `POST /api/plans/{pid}/allocations/{a_pid}/commit` — turn a proposed booking
/// into a committed one (PJ-4). Allowed to the named person, a resource manager
/// or an admin. If, with this booking firm, the person would be **over their
/// capacity on any day** (PJ-3 availability), the answer is `409` with the
/// numbers, and only `?confirm=true` commits it anyway (audited as an
/// over-commitment).
#[debug_handler]
async fn commit_allocation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, a_pid)): Path<(String, String)>,
    Query(body): Query<CommitBody>,
) -> Result<Response> {
    let confirm = body.confirm;
    let item = super::governance::find_item(&ctx, &pid).await?;
    let a_pid = Uuid::parse_str(&a_pid).map_err(|_| Error::NotFound)?;
    let allocation = vis_models::find_allocation(&ctx.db, a_pid).await?;
    if allocation.plan_pid != item.pid
        || !super::availability::may_manage_person(&caller, &allocation.person_ref)
    {
        return Err(Error::NotFound);
    }
    if allocation.booking == "committed" {
        return Err(Error::CustomError(
            axum::http::StatusCode::CONFLICT,
            loco_rs::controller::ErrorDetail::new("conflict", "this booking is already committed"),
        ));
    }
    // The person's committed load with this booking made firm, over its own span.
    let today = chrono::Utc::now().date_naive();
    let from = allocation.start_date.unwrap_or(today);
    let to = allocation.end_date.unwrap_or(from + chrono::Days::new(365));
    let mut allocs: Vec<crate::availability::Alloc> = vis_models::all_allocations(&ctx.db)
        .await?
        .iter()
        .filter(|a| a.person_ref == allocation.person_ref)
        .map(|a| crate::availability::Alloc {
            percent: a.percent,
            start: a.start_date,
            end: a.end_date,
        })
        .collect();
    allocs.push(crate::availability::Alloc {
        percent: allocation.percent,
        start: allocation.start_date,
        end: allocation.end_date,
    });
    let ranges = super::availability::ranges_for(&ctx, &allocation.person_ref).await?;
    let load = crate::availability::load(from, to, &allocs, &ranges);
    let over = load.days_over > 0;
    if over && !confirm {
        return Err(Error::CustomError(
            axum::http::StatusCode::CONFLICT,
            loco_rs::controller::ErrorDetail::new(
                "over_capacity",
                format!(
                    "committing this would put the person over capacity on {} calendar day(s) (peak {}% against a capacity of {}%); repeat the request with ?confirm=true to commit anyway. Over: {}",
                    load.days_over,
                    load.peak_demand,
                    load.min_capacity,
                    serde_json::to_string(&load.over).unwrap_or_default()
                ),
            ),
        ));
    }
    let row_pid = allocation.pid;
    let mut active: allocations::ActiveModel = allocation.into();
    active.booking = ActiveValue::set("committed".to_string());
    active
        .update(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(
        &ctx.db,
        row_pid,
        "allocation_committed",
        caller.actor(),
        Some(
            serde_json::json!({ "over_capacity": over, "days_over": load.days_over,
                                  "peak_percent": load.peak_demand }),
        ),
    )
    .await;
    format::json(
        serde_json::json!({ "pid": row_pid, "booking": "committed", "over_capacity": over }),
    )
}

/// `DELETE /api/plans/{pid}/allocations/{a_pid}` — soft-delete.
#[debug_handler]
async fn delete_allocation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, a_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let a_pid = Uuid::parse_str(&a_pid).map_err(|_| Error::NotFound)?;
    let allocation = vis_models::find_allocation(&ctx.db, a_pid).await?;
    if allocation.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    let row_pid = allocation.pid;
    let mut active: allocations::ActiveModel = allocation.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active
        .update(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row_pid, "allocation_removed", caller.actor(), None).await;
    format::empty_json()
}

/// `GET /api/capacity?from=&to=` — the per-person capacity rollup
/// over a window (default: today → today + 30 calendar days). Summed percent
/// over 100 ⇒ over-allocated (PPM-8).
#[derive(Debug, Deserialize)]
struct CapacityParams {
    #[serde(default)]
    from: Option<chrono::NaiveDate>,
    #[serde(default)]
    to: Option<chrono::NaiveDate>,
}

#[debug_handler]
async fn capacity(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<CapacityParams>,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let today = chrono::Utc::now().date_naive();
    let from = params.from.unwrap_or(today);
    let to = params.to.unwrap_or(from + chrono::Days::new(30));
    if to < from {
        return Err(unprocessable("to is before from"));
    }
    if (to - from).num_days() > crate::availability::MAX_WINDOW_DAYS {
        return Err(unprocessable("the window may span at most ten years"));
    }
    let rows = scope.keep(vis_models::all_allocations(&ctx.db).await?);
    // PJ-4: proposed bookings are reported as their own series, never merged in.
    let proposed_rows: Vec<_> = scope
        .keep(vis_models::all_allocations_with_proposed(&ctx.db).await?)
        .into_iter()
        .filter(|a| a.booking == "proposed")
        .collect();
    let all_ranges = super::availability::all_ranges(&ctx).await?;
    let mut people: Vec<&str> = rows
        .iter()
        .chain(proposed_rows.iter())
        .map(|a| a.person_ref.as_str())
        .collect();
    people.sort_unstable();
    people.dedup();
    let mut out = Vec::with_capacity(people.len());
    for person in people {
        let theirs: Vec<(i32, Option<chrono::NaiveDate>, Option<chrono::NaiveDate>)> = rows
            .iter()
            .filter(|a| a.person_ref == person)
            .map(|a| (a.percent, a.start_date, a.end_date))
            .collect();
        let total = rules::summed_percent(&theirs, from, to);
        // PJ-3: capacity is read per day from the person's availability ranges.
        // With none in the window the answer is exactly what it was before
        // (summed demand over 100); with some, a day is over when its demand
        // exceeds the capacity in force that day.
        let ranges = all_ranges.get(person).map_or(&[][..], Vec::as_slice);
        let in_window: Vec<_> = ranges
            .iter()
            .copied()
            .filter(|r| r.from <= to && r.to >= from)
            .collect();
        let allocs: Vec<crate::availability::Alloc> = theirs
            .iter()
            .map(|(percent, start, end)| crate::availability::Alloc {
                percent: *percent,
                start: *start,
                end: *end,
            })
            .collect();
        let load = crate::availability::load(from, to, &allocs, &in_window);
        let proposed: Vec<&allocations::Model> = proposed_rows
            .iter()
            .filter(|a| {
                a.person_ref == person
                    && a.start_date.is_none_or(|s| s <= to)
                    && a.end_date.is_none_or(|e| e >= from)
            })
            .collect();
        let would_exceed = !proposed.is_empty() && {
            let mut both = allocs.clone();
            both.extend(proposed.iter().map(|a| crate::availability::Alloc {
                percent: a.percent,
                start: a.start_date,
                end: a.end_date,
            }));
            crate::availability::load(from, to, &both, &in_window).days_over > 0
        };
        let over_allocated = if in_window.is_empty() {
            total > 100
        } else {
            load.days_over > 0
        };
        out.push(serde_json::json!({
            "person_ref": person,
            "allocated_percent": total,
            "over_allocated": over_allocated,
            "allocations": theirs.len(),
            "capacity": {
                "source": if in_window.is_empty() { "default" } else { "availability" },
                "min_percent": load.min_capacity,
            },
            "peak_percent": load.peak_demand,
            "over_capacity": load.over,
            // The proposed series: what is hoped for, kept apart from the firm
            // figures above. `would_exceed_capacity` asks whether committing all
            // of it would push this person over their capacity on any day.
            "proposed": {
                "percent": proposed.iter().map(|a| i64::from(a.percent)).sum::<i64>(),
                "allocations": proposed.len(),
                "would_exceed_capacity": would_exceed,
            },
        }));
    }
    out.sort_by_key(|v| -v["allocated_percent"].as_i64().unwrap_or(0));
    format::json(serde_json::json!({ "from": from, "to": to, "people": out }))
}

/// `POST /api/reports` body (PPM-9).
#[derive(Debug, Deserialize)]
struct ReportPayload {
    name: String,
    collection: String,
    #[serde(default)]
    filters: serde_json::Value,
    fields: Vec<String>,
    /// Optional one-field grouping (T-28e); see [`GROUP_FIELDS`].
    #[serde(default)]
    group_by: Option<String>,
}

/// Columns a report may project.
const REPORT_FIELDS: &[&str] = &[
    "pid",
    "kind",
    "name",
    "stage",
    "status",
    "start_date",
    "target_date",
    "created_at",
];

/// Columns a report may group by — the categorical ones; `pid`, `name`
/// and `created_at` are unique-ish and would make one group per plan.
const GROUP_FIELDS: &[&str] = &["kind", "stage", "status", "start_date", "target_date"];

#[debug_handler]
async fn create_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ReportPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if payload.name.trim().is_empty() || payload.name.len() > MAX_TEXT_LEN {
        problems.push("name is required (and capped)".to_string());
    }
    // `collection` is legacy metadata now that there is one recursive
    // `plans` collection: blank means "all plans", and a non-blank value
    // is accepted only as an optional kind filter
    // (portfolio/project/product/program/practice/process/purpose/pathway/proposal).
    if !payload.collection.trim().is_empty() && parse_kind_label(&payload.collection).is_none() {
        problems.push(
            "collection must be blank (all plans) or a kind (portfolio/project/product/program/practice/process/purpose/pathway/proposal)"
                .to_string(),
        );
    }
    if payload.fields.is_empty() {
        problems.push("fields must name at least one column".to_string());
    }
    for field in &payload.fields {
        if !REPORT_FIELDS.contains(&field.as_str()) {
            problems.push(format!(
                "unknown field {field:?} (allowed: {REPORT_FIELDS:?})"
            ));
        }
    }
    if let Some(group) = payload
        .group_by
        .as_deref()
        .filter(|g| !GROUP_FIELDS.contains(g))
    {
        problems.push(format!(
            "unknown group_by {group:?} (allowed: {GROUP_FIELDS:?})"
        ));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let row = report_definitions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(payload.name.clone()),
        collection: ActiveValue::set(payload.collection.clone()),
        filters: ActiveValue::set(payload.filters.clone()),
        fields: ActiveValue::set(serde_json::json!(payload.fields)),
        group_by: ActiveValue::set(payload.group_by.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row.pid, "report_saved", caller.actor(), None).await;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/reports` — saved definitions.
#[debug_handler]
async fn list_reports(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = report_definitions::Entity::find()
        .filter(report_definitions::Column::DeletedAt.is_null())
        .order_by_asc(report_definitions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    format::json(rows)
}

/// `DELETE /api/reports/{pid}` — soft-delete a definition.
#[debug_handler]
async fn delete_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let row = vis_models::find_report(&ctx.db, pid).await?;
    let row_pid = row.pid;
    let mut active: report_definitions::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    active
        .update(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record_or_log(&ctx.db, row_pid, "report_deleted", caller.actor(), None).await;
    format::empty_json()
}

/// One projected report cell.
fn report_cell(model: &plans::Model, field: &str) -> String {
    match field {
        "pid" => model.pid.to_string(),
        "kind" => model
            .kind
            .clone()
            .unwrap_or_else(|| "unspecified".to_string()),
        "name" => model.name.clone(),
        "stage" => model.stage.clone().unwrap_or_default(),
        "created_at" => model.created_at.to_rfc3339(),
        "status" | "start_date" | "target_date" => model
            .data
            .get(field)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        _ => String::new(),
    }
}

/// `GET /api/reports/{pid}/run?format=json|csv` — execute a saved
/// report synchronously (row cap 1000; scheduled/bulk-artifact runs
/// are the family bulk machinery, roadmap).
#[derive(Debug, Deserialize)]
struct RunParams {
    #[serde(default)]
    format: Option<String>,
}

#[debug_handler]
async fn run_report(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(params): Query<RunParams>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let def = vis_models::find_report(&ctx.db, pid).await?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    execute_report(&ctx, &def, params.format.as_deref(), &scope).await
}

/// Run a saved report definition to a response (shared by the on-demand run and
/// the scheduled run, T-28e, so the two can never produce different output).
pub(crate) async fn execute_report(
    ctx: &AppContext,
    def: &report_definitions::Model,
    format: Option<&str>,
    scope: &super::plan_access::HiddenScope,
) -> Result<Response> {
    let params = RunParams {
        format: format.map(str::to_string),
    };
    // The report's legacy `collection` is now an optional kind filter over
    // the single recursive `plans` collection: blank (or unrecognized) ⇒
    // all plans; a kind label ⇒ that kind only (filtered in-memory below).
    let kind_filter = parse_kind_label(&def.collection);
    let fields: Vec<String> = serde_json::from_value(def.fields.clone()).unwrap_or_default();
    let stage = def.filters.get("stage").and_then(|v| v.as_str());
    let status = def.filters.get("status").and_then(|v| v.as_str());
    let name_like = def
        .filters
        .get("name_like")
        .and_then(|v| v.as_str())
        .map(str::to_lowercase);
    let rows = scope.keep(crate::models::plans::Model::list(&ctx.db, 1000).await?);
    let selected: Vec<&plans::Model> = rows
        .iter()
        .filter(|m| {
            kind_filter
                .is_none_or(|want| m.kind.as_deref().and_then(parse_kind_label) == Some(want))
        })
        .filter(|m| stage.is_none_or(|want| m.stage.as_deref() == Some(want)))
        .filter(|m| {
            status.is_none_or(|want| m.data.get("status").and_then(|v| v.as_str()) == Some(want))
        })
        .filter(|m| {
            name_like
                .as_deref()
                .is_none_or(|want| m.name.to_lowercase().contains(want))
        })
        .collect();
    if let Some(group_field) = def.group_by.as_deref() {
        return run_grouped(
            ctx,
            &def.name,
            group_field,
            &selected,
            params.format.as_deref(),
        )
        .await;
    }
    if params.format.as_deref() == Some("csv") {
        let mut csv = fields
            .iter()
            .map(|f| rules::csv_field(f))
            .collect::<Vec<_>>()
            .join(",");
        csv.push('\n');
        for model in &selected {
            let line = fields
                .iter()
                .map(|f| rules::csv_field(&report_cell(model, f)))
                .collect::<Vec<_>>()
                .join(",");
            csv.push_str(&line);
            csv.push('\n');
        }
        return Ok((
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/csv"),
            )],
            csv,
        )
            .into_response());
    }
    let objects: Vec<serde_json::Value> = selected
        .iter()
        .map(|model| {
            fields
                .iter()
                .map(|f| (f.clone(), serde_json::json!(report_cell(model, f))))
                .collect::<serde_json::Map<String, serde_json::Value>>()
                .into()
        })
        .collect();
    format::json(serde_json::json!({
        "name": def.name,
        "collection": def.collection,
        "rows": objects.len(),
        "data": objects,
    }))
}

/// A grouped run (T-28e): one row per group value with the plan count
/// and the group's budget lines summed **per currency** — currencies are
/// never merged, and a group with no lines reports an empty `money`.
async fn run_grouped(
    ctx: &AppContext,
    name: &str,
    group_field: &str,
    selected: &[&plans::Model],
    format: Option<&str>,
) -> Result<Response> {
    let pids: Vec<Uuid> = selected.iter().map(|m| m.pid).collect();
    let lines = budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .filter(budget_lines::Column::PlanPid.is_in(pids))
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let entries = selected
        .iter()
        .map(|m| {
            let value = match report_cell(m, group_field) {
                v if v.is_empty() => "unspecified".to_string(),
                v => v,
            };
            let money = lines
                .iter()
                .filter(|b| b.plan_pid == m.pid)
                .map(|b| crate::insights::MoneyLine {
                    currency: b.currency.clone(),
                    planned_minor: b.planned_minor,
                    actual_minor: b.actual_minor,
                })
                .collect();
            (value, money)
        })
        .collect();
    let groups = crate::insights::group_report(entries);
    if format == Some("csv") {
        let mut csv = format!(
            "{},plans,currency,planned_minor,actual_minor,remaining_minor,line_count\n",
            rules::csv_field(group_field)
        );
        for g in &groups {
            let head = format!("{},{}", rules::csv_field(&g.value), g.count);
            if g.money.is_empty() {
                let _ = writeln!(csv, "{head},,,,,");
            }
            for r in &g.money {
                let _ = writeln!(
                    csv,
                    "{head},{},{},{},{},{}",
                    rules::csv_field(&r.currency),
                    r.planned_minor,
                    r.actual_minor,
                    r.remaining_minor,
                    r.line_count
                );
            }
        }
        return Ok((
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/csv"),
            )],
            csv,
        )
            .into_response());
    }
    format::json(serde_json::json!({
        "name": name,
        "group_by": group_field,
        "rows": selected.len(),
        "groups": groups,
    }))
}

/// `GET /api/at-a-glance` — the portfolio dashboard (PPM-7): per
/// collection, counts by RAG / stage / status plus risk exposure and
/// per-currency budget variance; site tiles headline the whole
/// estate. ETag-conditional (the tag excludes `as_of`).
#[debug_handler]
#[allow(clippy::too_many_lines)] // one pass over the whole estate
async fn at_a_glance(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let today = chrono::Utc::now().date_naive();
    let items = scope.keep(
        plans::Entity::find()
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let risk_rows = scope.keep(
        risks::Entity::find()
            .filter(risks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let budget_rows = scope.keep(
        budget_lines::Entity::find()
            .filter(budget_lines::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let proposal_rows = scope.keep(
        crate::models::_entities::proposals::Entity::find()
            .filter(crate::models::_entities::proposals::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let dependency_rows = scope.keep_edges(vis_models::all_dependencies(&ctx.db).await?, |e| {
        (e.predecessor_pid, e.successor_pid)
    });
    let allocation_rows = scope.keep(vis_models::all_allocations(&ctx.db).await?);

    // Global schedule violations (per successor item).
    let facts: Vec<rules::ScheduleItem> = items.iter().map(schedule_item).collect();
    let edges: Vec<rules::ScheduleEdge> = dependency_rows
        .iter()
        .map(|e| rules::ScheduleEdge {
            pid: e.pid,
            predecessor: e.predecessor_pid,
            successor: e.successor_pid,
            lag_days: e.lag_days,
            kind: crate::cpm::Kind::parse(&e.kind).unwrap_or(crate::cpm::Kind::Fs),
        })
        .collect();
    let all_violations = rules::violations(&facts, &edges);
    let violated: std::collections::HashSet<Uuid> =
        all_violations.iter().map(|v| v.successor).collect();

    // Per-item RAG, grouped by the optional `kind` label (one recursive
    // collection now; a missing label groups under "unspecified").
    let mut per_collection: std::collections::BTreeMap<String, Vec<&plans::Model>> =
        std::collections::BTreeMap::new();
    for item in &items {
        per_collection
            .entry(
                item.kind
                    .clone()
                    .unwrap_or_else(|| "unspecified".to_string()),
            )
            .or_default()
            .push(item);
    }
    let mut collections = Vec::new();
    for (kind, members) in &per_collection {
        let mut rag_counts =
            std::collections::BTreeMap::from([("red", 0), ("amber", 0), ("green", 0)]);
        let mut stages: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        for member in members {
            let member_risks: Vec<_> = risk_rows
                .iter()
                .filter(|r| r.plan_pid == member.pid)
                .collect();
            let materialized = member_risks
                .iter()
                .filter(|r| r.status == "materialized")
                .count();
            let max_exposure = member_risks
                .iter()
                .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
                .map(|r| r.probability * r.impact)
                .max()
                .unwrap_or(0);
            let member_budgets: Vec<_> = budget_rows
                .iter()
                .filter(|b| b.plan_pid == member.pid)
                .collect();
            let overrun = {
                let mut currencies: Vec<&str> =
                    member_budgets.iter().map(|b| b.currency.as_str()).collect();
                currencies.sort_unstable();
                currencies.dedup();
                currencies.into_iter().any(|c| {
                    let planned: i64 = member_budgets
                        .iter()
                        .filter(|b| b.currency == c)
                        .map(|b| b.planned_minor)
                        .sum();
                    let actual: i64 = member_budgets
                        .iter()
                        .filter(|b| b.currency == c)
                        .map(|b| b.actual_minor)
                        .sum();
                    actual > planned
                })
            };
            let target = member
                .data
                .get("target_date")
                .and_then(|v| v.as_str())
                .and_then(|s| rules::parse_flex_date(s, true));
            let colour = rules::rag(
                is_finished(member),
                target,
                today,
                materialized,
                max_exposure,
                overrun,
                violated.contains(&member.pid),
            );
            *rag_counts.get_mut(colour).expect("fixed keys") += 1;
            *stages
                .entry(
                    member
                        .stage
                        .clone()
                        .unwrap_or_else(|| "pre_gate".to_string()),
                )
                .or_default() += 1;
        }
        collections.push(serde_json::json!({
            "collection": kind,
            "total": members.len(),
            "rag": rag_counts,
            "stages": stages,
        }));
    }

    // Site tiles.
    let open_risk_exposure: i32 = risk_rows
        .iter()
        .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
        .map(|r| r.probability * r.impact)
        .sum();
    let proposals_open = proposal_rows
        .iter()
        .filter(|p| matches!(p.status.as_str(), "draft" | "submitted" | "in_review"))
        .count();
    let over_allocated = {
        let mut people: Vec<&str> = allocation_rows
            .iter()
            .map(|a| a.person_ref.as_str())
            .collect();
        people.sort_unstable();
        people.dedup();
        people
            .into_iter()
            .filter(|person| {
                let theirs: Vec<_> = allocation_rows
                    .iter()
                    .filter(|a| a.person_ref == *person)
                    .map(|a| (a.percent, a.start_date, a.end_date))
                    .collect();
                rules::summed_percent(&theirs, today, today + chrono::Days::new(30)) > 100
            })
            .count()
    };
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "collections": collections,
        "site_tiles": {
            "plans": items.len(),
            "proposals_open": proposals_open,
            "materialized_risks": risk_rows.iter().filter(|r| r.status == "materialized").count(),
            "open_risk_exposure": open_risk_exposure,
            "schedule_violations": all_violations.len(),
            "over_allocated_people": over_allocated,
        },
    });
    let mut fingerprint = body.clone();
    if let Some(map) = fingerprint.as_object_mut() {
        map.remove("as_of");
    }
    super::conditional_json(&headers, &super::etag_of(&fingerprint), &body)
}

/// The visibility routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/dependencies", post(create_dependency))
        .add("/dependencies", get(list_dependencies))
        .add("/dependencies/{pid}", delete(delete_dependency))
        .add("/plans/{pid}/schedule", get(portfolio_schedule))
        .add("/plans/{pid}/milestones", post(create_milestone))
        .add("/plans/{pid}/milestones", get(list_milestones))
        .add("/plans/{pid}/milestones/{m_pid}", put(reschedule_milestone))
        .add(
            "/plans/{pid}/milestones/{m_pid}/complete",
            post(complete_milestone),
        )
        .add("/plans/{pid}/allocations", post(create_allocation))
        .add("/plans/{pid}/allocations", get(list_allocations))
        .add(
            "/plans/{pid}/allocations/{a_pid}/commit",
            post(commit_allocation),
        )
        .add(
            "/plans/{pid}/allocations/{a_pid}",
            delete(delete_allocation),
        )
        .add("/plans/{pid}/skill-gap", get(skill_gap))
        .add("/capacity", get(capacity))
        .add("/reports", post(create_report))
        .add("/reports", get(list_reports))
        .add("/reports/{pid}", delete(delete_report))
        .add("/reports/{pid}/run", get(run_report))
        .add("/at-a-glance", get(at_a_glance))
}

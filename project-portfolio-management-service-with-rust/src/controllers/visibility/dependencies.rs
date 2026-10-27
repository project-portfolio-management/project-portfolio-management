//! Cross-item dependencies and the portfolio schedule view.

use super::super::unprocessable;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::{find_any_item, schedule_item};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{plan_dependencies, plans};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::visibility as rules;
use loco_rs::prelude::*;

/// `POST /api/dependencies` body: a dependency between two plans, two tasks,
/// or one of each (SC-1): a relationship `kind` (`fs` default, `ss`, `ff`,
/// `sf`) and a lag in business days, **negative for a lead**.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DependencyPayload {
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
pub(super) async fn create_dependency(
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
    let readable = super::super::cpm::readable_endpoints(
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
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = plan_dependencies::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        predecessor_pid: ActiveValue::set(payload.predecessor_pid),
        successor_pid: ActiveValue::set(payload.successor_pid),
        lag_days: ActiveValue::set(payload.lag_days),
        kind: ActiveValue::set(kind),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "dependency_added", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/dependencies?item={pid}` — edges touching one item (or
/// all, capped 500).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DependencyParams {
    #[serde(default)]
    item: Option<Uuid>,
}

#[debug_handler]
pub(super) async fn list_dependencies(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<DependencyParams>,
) -> Result<Response> {
    // Filtering by an item the caller cannot read answers exactly as for an item with no
    // links: an empty list, so the filter cannot be used to probe for ids.
    if let Some(item) = params.item
        && !super::super::cpm::readable_endpoints(&ctx, &caller, &[item])
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
    let readable = super::super::cpm::readable_endpoints(&ctx, &caller, &ends).await?;
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
pub(super) async fn delete_dependency(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = super::super::parse_pid(&pid)?;
    let row = vis_models::find_dependency(&ctx.db, pid).await?;
    // Removing a link needs to be able to read both ends; otherwise it is as if it were not there.
    let readable = super::super::cpm::readable_endpoints(
        &ctx,
        &caller,
        &[row.predecessor_pid, row.successor_pid],
    )
    .await?;
    if !readable.contains(&row.predecessor_pid) || !readable.contains(&row.successor_pid) {
        return Err(Error::NotFound);
    }
    let row_pid = row.pid;
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    plan_dependencies::Entity::delete_by_id(row.id)
        .exec(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row_pid, "dependency_removed", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::empty_json()
}

/// `GET /api/plans/{pid}/schedule` — the roadmap view for one plan: its
/// children's timeframes, the dependency edges among them, finish-start
/// violations, and the critical path. Any plan may contain children, so
/// this is no longer restricted to a `Portfolio`-kind plan.
#[debug_handler]
pub(super) async fn portfolio_schedule(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let parent_pid = super::super::parse_pid(&pid)?;
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
    let (plan_cal, cal_source) = super::super::calendars::plan_calendar(&ctx, parent_pid).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpm::{Kind, WorkCalendar};
    use chrono::NaiveDate;
    use serde_json::json;

    fn day(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, d).unwrap()
    }

    fn item(n: u128, start: u32, end: u32) -> rules::ScheduleItem {
        rules::ScheduleItem {
            pid: Uuid::from_u128(n),
            start: Some(day(start)),
            end: Some(day(end)),
        }
    }

    fn edge(n: u128, from: u128, to: u128) -> rules::ScheduleEdge {
        rules::ScheduleEdge {
            pid: Uuid::from_u128(n),
            predecessor: Uuid::from_u128(from),
            successor: Uuid::from_u128(to),
            lag_days: 0,
            kind: Kind::Fs,
        }
    }

    fn members(n: u128) -> Vec<plans::Model> {
        let at = chrono::DateTime::parse_from_rfc3339("2026-03-01T00:00:00+00:00").unwrap();
        vec![plans::Model {
            created_at: at,
            updated_at: at,
            id: 1,
            pid: Uuid::from_u128(n),
            kind: None,
            name: "Container".to_string(),
            data: json!({}),
            parent_pid: None,
            active: true,
            stage: None,
            phase: None,
            deleted_at: None,
            content_hash: None,
            content_hash_sha3: None,
            content_mac: None,
        }]
    }

    #[test]
    fn the_container_is_a_summary_not_a_node() {
        // 100 is the container (first member); 1 and 2 are the work.
        let facts = [item(100, 1, 20), item(1, 1, 5), item(2, 6, 10)];
        let out = plan_level_cpm(
            &members(100),
            &facts,
            &[edge(9, 1, 2)],
            &WorkCalendar::seven_day(),
            "default",
        );
        assert_eq!(out["calendar"], "default");
        let nodes = out["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 2, "the container is left out");
        assert!(
            nodes
                .iter()
                .all(|n| n["pid"] != json!(Uuid::from_u128(100)))
        );
    }

    #[test]
    fn a_dependency_cycle_is_reported_not_raised() {
        let facts = [item(100, 1, 20), item(1, 1, 5), item(2, 6, 10)];
        let out = plan_level_cpm(
            &members(100),
            &facts,
            &[edge(8, 1, 2), edge(9, 2, 1)],
            &WorkCalendar::seven_day(),
            "default",
        );
        assert_eq!(out["error"], "cycle");
    }

    #[test]
    fn an_item_with_no_usable_dates_is_unscheduled() {
        let mut undated = item(2, 6, 10);
        undated.end = None;
        let facts = [item(100, 1, 20), item(1, 1, 5), undated];
        let out = plan_level_cpm(
            &members(100),
            &facts,
            &[],
            &WorkCalendar::seven_day(),
            "default",
        );
        assert!(
            out["unscheduled"]
                .to_string()
                .contains(&Uuid::from_u128(2).to_string()),
            "{out}"
        );
    }
}

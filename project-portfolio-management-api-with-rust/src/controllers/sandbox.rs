//! The what-if sandbox (PJ-9). Rules in [`crate::sandbox`]; this is the wiring.
//!
//! * `POST /api/plans/{pid}/sandbox/preview` applies a list of edits (move, change
//!   duration, add or remove a link, change a constraint) to an **in-memory copy**
//!   of the plan and answers with the resulting dates, float, critical path and
//!   labour-cost impact. It writes **nothing** — no row, no audit entry — and the
//!   same edits give the same answer. Discarding a preview is simply not
//!   committing it.
//! * `POST /api/plans/{pid}/sandbox/commit` applies the same edits for real, in one
//!   transaction, one audit entry per changed item. It needs `If-Match: <plan_etag
//!   from the preview>`; if the plan changed since, the answer is `409`, never a
//!   merge. The leveling apply (SC-6) goes through [`commit`] too.
//!
//! The permission to set a hard constraint, the masking of who does the work, and
//! the finance gate on cost are the same as everywhere else, and a preview applies
//! them, so a preview never shows more than a commit would allow.

use std::collections::{HashMap, HashSet};

use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::cpm::{Network, load_network, may_set_hard_constraint};
use crate::auth::MaybeAuthUser;
use crate::cpm::{self, CpmResult};
use crate::models::_entities::{plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::sandbox::{self, Change, Edit};

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn forbidden(message: &str) -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new("forbidden", message),
    )
}

fn stale() -> Error {
    Error::CustomError(
        StatusCode::CONFLICT,
        ErrorDetail::new(
            "stale_preview",
            "the plan changed since this preview: preview again, then commit",
        ),
    )
}

/// What evaluating a list of edits produces.
struct Evaluated {
    before_net: Network,
    before: CpmResult,
    after_net: Network,
    after: CpmResult,
    changes: Vec<Change>,
    /// Tasks whose units / work were recomputed because their duration changed (PJ-6).
    recomputed: HashMap<Uuid, super::assignments::Recomputed>,
    plan_etag: String,
    result_etag: String,
    calendars: String,
}

/// The schedule edges of a plan as stored: `(predecessor, successor, kind, lag)`, in
/// the order the network is built (stored dependencies, then `precedes` relations
/// not already covered). Read through any connection, so a transaction can re-read
/// exactly what the preview read.
async fn read_edges<C: sea_orm::ConnectionTrait>(
    conn: &C,
    pids: &HashSet<Uuid>,
) -> Result<Vec<(Uuid, Uuid, String, i64)>> {
    let mut seen: HashSet<(Uuid, Uuid)> = HashSet::new();
    let mut out = Vec::new();
    for d in plan_dependencies::Entity::find()
        .order_by_asc(plan_dependencies::Column::Id)
        .all(conn)
        .await
        .map_err(db_err)?
    {
        if pids.contains(&d.predecessor_pid) && pids.contains(&d.successor_pid) {
            seen.insert((d.predecessor_pid, d.successor_pid));
            out.push((
                d.predecessor_pid,
                d.successor_pid,
                d.kind,
                i64::from(d.lag_days),
            ));
        }
    }
    for r in task_relations::Entity::find()
        .filter(task_relations::Column::DeletedAt.is_null())
        .filter(task_relations::Column::Kind.eq("precedes"))
        .order_by_asc(task_relations::Column::Id)
        .all(conn)
        .await
        .map_err(db_err)?
    {
        if pids.contains(&r.from_task_pid)
            && pids.contains(&r.to_task_pid)
            && seen.insert((r.from_task_pid, r.to_task_pid))
        {
            out.push((r.from_task_pid, r.to_task_pid, "fs".to_string(), 0));
        }
    }
    Ok(out)
}

/// The plan's task rows, as the network is built from them.
async fn read_rows<C: sea_orm::ConnectionTrait>(conn: &C, plan: Uuid) -> Result<Vec<tasks::Model>> {
    tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .order_by_asc(tasks::Column::Id)
        .all(conn)
        .await
        .map_err(db_err)
}

/// A digest of a plan's state: every field an edit can change or depend on, the
/// schedule edges, and the calendars.
fn digest(rows: &[tasks::Model], edges: &[(Uuid, Uuid, String, i64)], calendars: &str) -> String {
    let mut parts: Vec<String> = rows
        .iter()
        .map(|t| {
            format!(
                "{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
                t.pid,
                t.start_date,
                t.duration_days,
                t.constraint_kind,
                t.constraint_date,
                t.deadline,
                t.assignee_ref,
                t.units_percent,
                t.work_minutes
            )
        })
        .collect();
    parts.extend(edges.iter().map(|e| format!("{e:?}")));
    parts.push(calendars.to_string());
    sandbox::fingerprint(&parts)
}

fn calendars_of(net: &Network) -> String {
    format!("{:?}|{:?}", net.plan_cal, net.own)
}

fn analyse(net: &Network, what: &str) -> Result<CpmResult> {
    net.analyze(0).map_err(|_| {
        unprocessable(&format!(
            "{what} has a dependency cycle, so it cannot be analysed"
        ))
    })
}

/// Dependency pairs from outside this network, for the cycle check.
async fn outside_pairs(ctx: &AppContext, net: &Network) -> Result<Vec<(Uuid, Uuid)>> {
    let here: HashSet<(Uuid, Uuid)> = net.edges.iter().map(|e| (e.pred, e.succ)).collect();
    let mut all: Vec<(Uuid, Uuid)> = plan_dependencies::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|d| (d.predecessor_pid, d.successor_pid))
        .collect();
    all.extend(
        task_relations::Entity::find()
            .filter(task_relations::Column::DeletedAt.is_null())
            .filter(task_relations::Column::Kind.eq("precedes"))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|r| (r.from_task_pid, r.to_task_pid)),
    );
    all.retain(|p| !here.contains(p));
    Ok(all)
}

async fn evaluate(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    edits: &[Edit],
) -> Result<Evaluated> {
    let problems = sandbox::check(edits);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    if sandbox::needs_hard_permission(edits) && !may_set_hard_constraint(caller) {
        return Err(forbidden(
            "a hard constraint (mso, mfo) overrides the schedule's logic and needs access=admin or scheduling=hard_constraints",
        ));
    }
    let before_net = load_network(ctx, plan).await?;
    let before = analyse(&before_net, "the plan")?;
    let outside = outside_pairs(ctx, &before_net).await?;
    let edge_pids: Vec<Uuid> = before_net.edges.iter().map(|e| e.pid).collect();
    let removable: HashSet<Uuid> = plan_dependencies::Entity::find()
        .filter(plan_dependencies::Column::Pid.is_in(edge_pids))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|d| d.pid)
        .collect();
    let mut nodes = before_net.nodes.clone();
    let mut edges = before_net.edges.clone();
    let changes = sandbox::apply(&mut nodes, &mut edges, &outside, &removable, edits)
        .map_err(|m| unprocessable(&m))?;
    // The copy's task rows follow, including the units / work a duration change recomputes.
    let mut rows = before_net.rows.clone();
    let mut recomputed = HashMap::new();
    for c in &changes {
        let Change::Task { pid, to, .. } = c else {
            continue;
        };
        let Some(row) = rows.iter_mut().find(|r| r.pid == *pid) else {
            continue;
        };
        let new_duration = to.duration_days.and_then(|d| i32::try_from(d).ok());
        if let Some(r) = super::assignments::recompute_for_duration(row, new_duration)
            .map_err(|m| unprocessable(&m))?
        {
            row.units_percent = i32::try_from(r.triple.units_percent).ok();
            row.work_minutes = Some(r.triple.work_minutes);
            recomputed.insert(*pid, r);
        }
        row.start_date = to.start_date;
        row.duration_days = new_duration;
        row.constraint_kind = to.constraint_kind.map(str::to_string);
        row.constraint_date = to.constraint_date;
    }
    let after_net = Network {
        nodes,
        edges,
        rows,
        plan_cal: before_net.plan_cal.clone(),
        cal_source: before_net.cal_source,
        own: before_net.own.clone(),
        conflicts: before_net.conflicts.clone(),
        external: before_net.external.clone(),
        restricted: before_net.restricted.clone(),
        truncated: before_net.truncated,
    };
    let after = analyse(&after_net, "the plan with these edits")?;
    let cal = calendars_of(&before_net);
    let pids: HashSet<Uuid> = before_net.rows.iter().map(|r| r.pid).collect();
    let stored = read_edges(&ctx.db, &pids).await?;
    let after_edges: Vec<(Uuid, Uuid, String, i64)> = after_net
        .edges
        .iter()
        .map(|x| {
            (
                x.pred,
                x.succ,
                sandbox::kind_token(x.kind).to_string(),
                x.lag,
            )
        })
        .collect();
    Ok(Evaluated {
        plan_etag: digest(&before_net.rows, &stored, &cal),
        result_etag: digest(&after_net.rows, &after_edges, &cal),
        calendars: cal,
        before_net,
        before,
        after_net,
        after,
        changes,
        recomputed,
    })
}

/// The preview body. Deterministic: nothing in it depends on the clock.
async fn body(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    e: &Evaluated,
) -> Result<serde_json::Value> {
    let see_people = super::portfolio_resources::may_manage_people(caller);
    let titles: HashMap<Uuid, (&str, Option<&str>)> = e
        .after_net
        .rows
        .iter()
        .map(|t| (t.pid, (t.title.as_str(), t.assignee_ref.as_deref())))
        .collect();
    let deltas: Vec<serde_json::Value> = sandbox::deltas(&e.before, &e.after)
        .into_iter()
        .map(|d| {
            let (title, who) = titles.get(&d.pid).copied().unwrap_or(("", None));
            let mut v = serde_json::json!({ "pid": d.pid, "title": title, "before": d.before, "after": d.after });
            if see_people {
                v["assignee"] = serde_json::json!(who);
            }
            v
        })
        .collect();
    let (cb, ca) = (
        sandbox::critical_path(&e.before),
        sandbox::critical_path(&e.after),
    );
    let cost = if super::cost_rates::may_see_rates(caller) {
        let before = super::cost_rates::labour_totals(ctx, &e.before_net, Some(&e.before)).await?;
        let after = super::cost_rates::labour_totals(ctx, &e.after_net, Some(&e.after)).await?;
        let mut currencies: Vec<&String> = before
            .by_currency
            .keys()
            .chain(after.by_currency.keys())
            .collect();
        currencies.sort();
        currencies.dedup();
        if currencies.is_empty() && before.unrated_minutes == 0 && after.unrated_minutes == 0 {
            serde_json::json!({ "available": false, "reason": "no_costed_work" })
        } else {
            serde_json::json!({
                "available": true,
                "labour": currencies.iter().map(|c| {
                    let (b, a) = (before.by_currency.get(*c).copied().unwrap_or(0), after.by_currency.get(*c).copied().unwrap_or(0));
                    serde_json::json!({ "currency": c, "before_minor": b, "after_minor": a, "delta_minor": a - b })
                }).collect::<Vec<_>>(),
                "unrated_minutes_before": before.unrated_minutes,
                "unrated_minutes_after": after.unrated_minutes,
                "note": "Planned labour cost: each task's work spread over its working days and priced day by day at the rate in force, per currency. A shift across a rate change, or a duration change on a task with work, changes it. Work with no rate is counted apart (unknown, never zero); expenses are unchanged by edits and not shown.",
            })
        }
    } else {
        serde_json::json!({ "available": false, "reason": "no_finance_access" })
    };
    Ok(serde_json::json!({
        "plan_pid": plan,
        "plan_etag": e.plan_etag,
        "result_etag": e.result_etag,
        "changes": e.changes,
        "finish": { "before": e.before.project_finish, "after": e.after.project_finish },
        "tasks": deltas,
        "moved": cpm::diff(&e.before, &e.after),
        "critical_path": {
            "before": cb, "after": ca,
            "added": ca.iter().filter(|p| !cb.contains(p)).collect::<Vec<_>>(),
            "removed": cb.iter().filter(|p| !ca.contains(p)).collect::<Vec<_>>(),
        },
        "assignments_recomputed": e.recomputed.iter().map(|(p, r)| serde_json::json!({
            "task_pid": p, "derived": r.derived, "exact": r.exact,
            "units_percent": r.triple.units_percent, "work_minutes": r.triple.work_minutes })).collect::<Vec<_>>(),
        "cost_impact": cost,
        "masked": !see_people,
        "note": "Nothing has been changed. Send these edits to …/sandbox/commit with If-Match set to plan_etag to apply exactly this; if the plan changes first, the commit is refused (409), never merged. To discard, do nothing.",
    }))
}

/// Per-task audit overrides (action and detail), for a caller that audits in its own words.
pub(crate) type AuditOverrides = HashMap<Uuid, (&'static str, serde_json::Value)>;

/// What a commit did.
pub(crate) struct Committed {
    pub changes: Vec<Change>,
    pub audit_entries: usize,
    pub finish_before: Option<chrono::NaiveDate>,
    pub finish_after: Option<chrono::NaiveDate>,
}

/// Apply `edits` for real, in one transaction, one audit entry per changed item.
/// `expected` is the preview's `plan_etag` (`None` skips the freshness check, for a
/// caller that has already verified the plan against its own proposal).
///
/// # Errors
///
/// `409` when the plan changed since `expected`; `422`/`403` as the preview.
#[allow(clippy::too_many_lines)]
pub(crate) async fn commit(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    edits: &[Edit],
    expected: Option<&str>,
    reason: Option<&str>,
    overrides: &AuditOverrides,
) -> Result<Committed> {
    let e = evaluate(ctx, caller, plan, edits).await?;
    if expected.is_some_and(|x| x != e.plan_etag) {
        return Err(stale());
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    // One commit at a time per plan: a second waits here, then re-reads the plan
    // through this transaction and is refused as stale if the first changed it.
    sea_orm::ConnectionTrait::execute_unprepared(
        &txn,
        &format!("SELECT pg_advisory_xact_lock(hashtext('sandbox:{plan}'))"),
    )
    .await
    .map_err(db_err)?;
    let rows = read_rows(&txn, plan).await?;
    let pids: HashSet<Uuid> = rows.iter().map(|r| r.pid).collect();
    let edges = read_edges(&txn, &pids).await?;
    if digest(&rows, &edges, &e.calendars) != e.plan_etag {
        return Err(stale());
    }
    let mut audit_entries = 0;
    for c in &e.changes {
        match c {
            Change::Task { pid, from, to } => {
                let row = tasks::Entity::find()
                    .filter(tasks::Column::Pid.eq(*pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: tasks::ActiveModel = row.into();
                a.start_date = ActiveValue::set(to.start_date);
                a.duration_days =
                    ActiveValue::set(to.duration_days.and_then(|d| i32::try_from(d).ok()));
                a.constraint_kind = ActiveValue::set(to.constraint_kind.map(str::to_string));
                a.constraint_date = ActiveValue::set(to.constraint_date);
                if let Some(r) = e.recomputed.get(pid) {
                    a.units_percent = ActiveValue::set(i32::try_from(r.triple.units_percent).ok());
                    a.work_minutes = ActiveValue::set(Some(r.triple.work_minutes));
                }
                a.update(&txn).await.map_err(db_err)?;
                let (action, detail) = overrides.get(pid).cloned().unwrap_or_else(|| {
                    (
                        "task_schedule_updated",
                        serde_json::json!({ "via": "sandbox", "reason": reason,
                            "from": from, "to": to,
                            "assignment": e.recomputed.get(pid).map(|r| serde_json::json!({
                                "derived": r.derived, "units_percent": r.triple.units_percent,
                                "work_minutes": r.triple.work_minutes })) }),
                    )
                });
                AuditModel::record(&txn, *pid, action, caller.actor(), Some(detail))
                    .await
                    .map_err(Error::Model)?;
                audit_entries += 1;
            }
            Change::LinkAdded {
                predecessor_pid,
                successor_pid,
                kind,
                lag_days,
                ..
            } => {
                let row = plan_dependencies::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    predecessor_pid: ActiveValue::set(*predecessor_pid),
                    successor_pid: ActiveValue::set(*successor_pid),
                    lag_days: ActiveValue::set(i32::try_from(*lag_days).unwrap_or(0)),
                    kind: ActiveValue::set((*kind).to_string()),
                    ..Default::default()
                }
                .insert(&txn)
                .await
                .map_err(db_err)?;
                AuditModel::record(&txn, row.pid, "dependency_added", caller.actor(),
                    Some(serde_json::json!({ "via": "sandbox", "reason": reason, "predecessor_pid": predecessor_pid,
                        "successor_pid": successor_pid, "kind": kind, "lag_days": lag_days })))
                    .await
                    .map_err(Error::Model)?;
                audit_entries += 1;
            }
            Change::LinkRemoved {
                pid,
                predecessor_pid,
                successor_pid,
                kind,
                lag_days,
            } => {
                plan_dependencies::Entity::delete_many()
                    .filter(plan_dependencies::Column::Pid.eq(*pid))
                    .exec(&txn)
                    .await
                    .map_err(db_err)?;
                AuditModel::record(&txn, *pid, "dependency_removed", caller.actor(),
                    Some(serde_json::json!({ "via": "sandbox", "reason": reason, "predecessor_pid": predecessor_pid,
                        "successor_pid": successor_pid, "kind": kind, "lag_days": lag_days })))
                    .await
                    .map_err(Error::Model)?;
                audit_entries += 1;
            }
        }
    }
    txn.commit().await.map_err(db_err)?;
    Ok(Committed {
        changes: e.changes,
        audit_entries,
        finish_before: e.before.project_finish,
        finish_after: e.after.project_finish,
    })
}

#[derive(Debug, Deserialize)]
struct EditsBody {
    edits: Vec<Edit>,
    /// Why (audited with each changed item).
    #[serde(default)]
    reason: Option<String>,
}

/// `POST /api/plans/{pid}/sandbox/preview` — what the edits would do. Writes nothing.
#[debug_handler]
async fn preview(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<EditsBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let e = evaluate(&ctx, &caller, plan.pid, &b.edits).await?;
    let json = body(&ctx, &caller, plan.pid, &e).await?;
    let mut res = format::json(json)?;
    if let Ok(v) = header::HeaderValue::from_str(&e.plan_etag) {
        res.headers_mut().insert(header::ETAG, v);
    }
    Ok(res)
}

/// `POST /api/plans/{pid}/sandbox/commit` — apply exactly the previewed edits.
#[debug_handler]
async fn commit_handler(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Json(b): Json<EditsBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if b.reason.as_deref().is_some_and(|r| r.chars().count() > 500) {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    let Some(expected) = headers.get(header::IF_MATCH).and_then(|v| v.to_str().ok()) else {
        return Ok((
            StatusCode::PRECONDITION_REQUIRED,
            axum::Json(serde_json::json!({
                "error": "precondition_required",
                "description": "send If-Match with the plan_etag of the preview you are committing",
            })),
        )
            .into_response());
    };
    let done = commit(
        &ctx,
        &caller,
        plan.pid,
        &b.edits,
        Some(expected.trim()),
        b.reason.as_deref(),
        &AuditOverrides::new(),
    )
    .await?;
    format::json(serde_json::json!({
        "committed": true,
        "changes": done.changes,
        "audit_entries": done.audit_entries,
        "finish": { "before": done.finish_before, "after": done.finish_after },
    }))
}

/// The sandbox routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/sandbox/preview", post(preview))
        .add("/plans/{pid}/sandbox/commit", post(commit_handler))
}

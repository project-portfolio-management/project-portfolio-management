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
//!   merge. The leveling apply (SC-6) goes through `commit` too.
//!
//! The permission to set a hard constraint, the masking of who does the work, and
//! the finance gate on cost are the same as everywhere else, and a preview applies
//! them, so a preview never shows more than a commit would allow.

use super::forbidden;
use super::unprocessable;
use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use super::cpm::{Network, load_network, may_set_hard_constraint};
use crate::auth::MaybeAuthUser;
use crate::cpm::{self, CpmResult};
use crate::models::_entities::{plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::sandbox::{self, Change, Edit};

/// The `409` returned when the plan changed between preview and commit;
/// a stale preview is refused, never merged.
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
    /// The plan's network before the edits, as stored.
    before_net: Network,
    /// Critical-path analysis of the plan before the edits.
    before: CpmResult,
    /// A copy of the network with the edits applied; the stored plan is untouched.
    after_net: Network,
    /// Critical-path analysis of the edited copy.
    after: CpmResult,
    /// The per-item differences the edits produce, as reported to the caller.
    changes: Vec<Change>,
    /// Tasks whose units / work were recomputed because their duration changed (PJ-6).
    recomputed: HashMap<Uuid, super::assignments::Recomputed>,
    /// Digest of the stored plan state; the preview's `ETag` and the commit's
    /// `If-Match` precondition.
    plan_etag: String,
    /// Digest of the plan state the edits would produce.
    result_etag: String,
    /// Debug rendering of the plan and per-task calendars, used to detect a
    /// calendar change between preview and commit.
    calendars: String,
}

/// The schedule edges of a plan as stored: `(predecessor, successor, kind, lag)`, in
/// the order the network is built (stored dependencies, then `precedes` relations
/// not already covered). Read through any connection, so a transaction can re-read
/// exactly what the preview read.
async fn read_edges(
    conn: &mut impl crate::db::Session,
    pids: &HashSet<Uuid>,
) -> Result<Vec<(Uuid, Uuid, String, i64)>> {
    let mut seen: HashSet<(Uuid, Uuid)> = HashSet::new();
    let mut out = Vec::new();
    let dependencies: Vec<plan_dependencies::Model> =
        sqlx::query_as("SELECT * FROM plan_dependencies ORDER BY id")
            .fetch_all(conn.ex())
            .await?;
    for d in dependencies {
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
    let relations: Vec<task_relations::Model> = sqlx::query_as(
        "SELECT * FROM task_relations WHERE deleted_at IS NULL AND kind = $1 ORDER BY id",
    )
    .bind("precedes")
    .fetch_all(conn.ex())
    .await?;
    for r in relations {
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
async fn read_rows(conn: &mut impl crate::db::Session, plan: Uuid) -> Result<Vec<tasks::Model>> {
    // Hammocks are not in the network the preview digests, so the recheck must not see them either.
    Ok(sqlx::query_as(
        "SELECT * FROM tasks
         WHERE plan_pid = $1 AND deleted_at IS NULL AND is_hammock = $2
         ORDER BY id",
    )
    .bind(plan)
    .bind(false)
    .fetch_all(conn.ex())
    .await?)
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

/// A stable string form of the calendars in force, folded into the digest.
fn calendars_of(net: &Network) -> String {
    format!("{:?}|{:?}", net.plan_cal, net.own)
}

/// Run the critical-path analysis; a dependency cycle becomes a `422`
/// naming `what` was being analysed.
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
    let mut all: Vec<(Uuid, Uuid)> =
        sqlx::query_as("SELECT predecessor_pid, successor_pid FROM plan_dependencies")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    all.extend(
        sqlx::query_as::<_, (Uuid, Uuid)>(
            "SELECT from_task_pid, to_task_pid FROM task_relations
             WHERE deleted_at IS NULL AND kind = $1",
        )
        .bind("precedes")
        .fetch_all(crate::db::pool(ctx))
        .await?,
    );
    all.retain(|p| !here.contains(p));
    Ok(all)
}

/// Check, authorise and apply `edits` to an in-memory copy of the plan and
/// analyse both versions. Writes nothing. Hard constraints (`mso`, `mfo`)
/// override schedule logic, so they need elevated permission.
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
    // Hard constraints bypass dependency logic, so gate them before any work is done.
    if sandbox::needs_hard_permission(edits) && !may_set_hard_constraint(caller) {
        return Err(forbidden(
            "a hard constraint (mso, mfo) overrides the schedule's logic and needs access=admin or scheduling=hard_constraints",
        ));
    }
    let before_net = load_network(ctx, plan).await?;
    let before = analyse(&before_net, "the plan")?;
    let outside = outside_pairs(ctx, &before_net).await?;
    let edge_pids: Vec<Uuid> = before_net.edges.iter().map(|e| e.pid).collect();
    let removable: HashSet<Uuid> =
        sqlx::query_scalar::<_, Uuid>("SELECT pid FROM plan_dependencies WHERE pid = ANY($1)")
            .bind(edge_pids)
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
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
    let stored = read_edges(&mut crate::db::pool(ctx), &pids).await?;
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
                "note": "Planned labour cost: each task's work spread over its business days and priced day by day at the rate in force, per currency. A shift across a rate change, or a duration change on a task with work, changes it. Work with no rate is counted apart (unknown, never zero); expenses are unchanged by edits and not shown.",
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
    /// The changes that were applied, as previewed.
    pub changes: Vec<Change>,
    /// How many audit entries were written (one per changed item).
    pub audit_entries: usize,
    /// Plan finish date before the commit; absent when it could not be computed.
    pub finish_before: Option<chrono::NaiveDate>,
    /// Plan finish date after the commit; absent when it could not be computed.
    pub finish_after: Option<chrono::NaiveDate>,
}

/// Apply `edits` for real, in one transaction, one audit entry per changed item.
/// `expected` is the preview's `plan_etag` (`None` skips the freshness check, for a
/// caller that has already verified the plan against its own proposal).
///
/// # Errors
///
/// `409` when the plan changed since `expected`; `422`/`403` as the preview.
#[expect(clippy::too_many_lines)]
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
    let mut tx = crate::db::begin(ctx).await?;
    // One commit at a time per plan: a second waits here, then re-reads the plan
    // through this transaction and is refused as stale if the first changed it.
    super::advisory_lock_on(&mut tx, "sandbox", plan).await?;
    let rows = read_rows(&mut tx, plan).await?;
    let pids: HashSet<Uuid> = rows.iter().map(|r| r.pid).collect();
    let edges = read_edges(&mut tx, &pids).await?;
    if digest(&rows, &edges, &e.calendars) != e.plan_etag {
        return Err(stale());
    }
    let mut audit_entries = 0;
    for c in &e.changes {
        match c {
            Change::Task { pid, from, to } => {
                let id: i32 = sqlx::query_scalar("SELECT id FROM tasks WHERE pid = $1 LIMIT 1")
                    .bind(*pid)
                    .fetch_optional(tx.conn())
                    .await?
                    .ok_or(Error::NotFound)?;
                let duration = to.duration_days.and_then(|d| i32::try_from(d).ok());
                let constraint_kind = to.constraint_kind.map(str::to_string);
                // `fetch_one`: an update that finds no row is an error, as before SQ-1.
                if let Some(r) = e.recomputed.get(pid) {
                    sqlx::query_scalar::<_, i32>(
                        "UPDATE tasks
                         SET start_date = $1, duration_days = $2, constraint_kind = $3, constraint_date = $4,
                             units_percent = $5, work_minutes = $6
                         WHERE id = $7
                         RETURNING id",
                    )
                    .bind(to.start_date)
                    .bind(duration)
                    .bind(&constraint_kind)
                    .bind(to.constraint_date)
                    .bind(i32::try_from(r.triple.units_percent).ok())
                    .bind(Some(r.triple.work_minutes))
                    .bind(id)
                    .fetch_one(tx.conn())
                    .await?;
                } else {
                    sqlx::query_scalar::<_, i32>(
                        "UPDATE tasks
                         SET start_date = $1, duration_days = $2, constraint_kind = $3, constraint_date = $4
                         WHERE id = $5
                         RETURNING id",
                    )
                    .bind(to.start_date)
                    .bind(duration)
                    .bind(&constraint_kind)
                    .bind(to.constraint_date)
                    .bind(id)
                    .fetch_one(tx.conn())
                    .await?;
                }
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
                AuditModel::record_on(tx.conn(), *pid, action, caller.actor(), Some(detail))
                    .await?;
                audit_entries += 1;
            }
            Change::LinkAdded {
                predecessor_pid,
                successor_pid,
                kind,
                lag_days,
                ..
            } => {
                let row_pid: Uuid = sqlx::query_scalar(
                    "INSERT INTO plan_dependencies (pid, predecessor_pid, successor_pid, lag_days, kind)
                     VALUES ($1, $2, $3, $4, $5)
                     RETURNING pid",
                )
                .bind(Uuid::new_v4())
                .bind(*predecessor_pid)
                .bind(*successor_pid)
                .bind(i32::try_from(*lag_days).unwrap_or(0))
                .bind((*kind).to_string())
                .fetch_one(tx.conn())
                .await?;
                AuditModel::record_on(tx.conn(), row_pid, "dependency_added", caller.actor(),
                    Some(serde_json::json!({ "via": "sandbox", "reason": reason, "predecessor_pid": predecessor_pid,
                        "successor_pid": successor_pid, "kind": kind, "lag_days": lag_days })))
                    .await?;
                audit_entries += 1;
            }
            Change::LinkRemoved {
                pid,
                predecessor_pid,
                successor_pid,
                kind,
                lag_days,
            } => {
                sqlx::query("DELETE FROM plan_dependencies WHERE pid = $1")
                    .bind(*pid)
                    .execute(tx.conn())
                    .await?;
                AuditModel::record_on(tx.conn(), *pid, "dependency_removed", caller.actor(),
                    Some(serde_json::json!({ "via": "sandbox", "reason": reason, "predecessor_pid": predecessor_pid,
                        "successor_pid": successor_pid, "kind": kind, "lag_days": lag_days })))
                    .await?;
                audit_entries += 1;
            }
        }
    }
    tx.commit().await?;
    Ok(Committed {
        changes: e.changes,
        audit_entries,
        finish_before: e.before.project_finish,
        finish_after: e.after.project_finish,
    })
}

/// The preview of `edits` on `plan` as JSON (the same answer `POST …/sandbox/preview` gives), for a caller that
/// holds the edits itself, such as a change request (IR-5). Writes nothing.
pub(crate) async fn preview_json(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    edits: &[Edit],
) -> Result<serde_json::Value> {
    let e = evaluate(ctx, caller, plan, edits).await?;
    body(ctx, caller, plan, &e).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `POST /api/plans/{pid}/sandbox/preview` and `.../commit` body.
pub(crate) struct EditsBody {
    /// The proposed edits, applied in order to a copy of the plan.
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
        return Ok(super::problem(
            StatusCode::PRECONDITION_REQUIRED,
            "precondition_required",
            "send If-Match with the plan_etag of the preview you are committing",
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

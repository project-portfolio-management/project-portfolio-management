//! Buffer management (CC-4). Rules in [`crate::buffer_management`].
//!
//! * `POST /api/plans/{pid}/buffer-plan` freezes the critical chain and its buffers (CC-3) as planned, and
//!   `GET` returns the latest and the history;
//! * `GET /api/plans/{pid}/buffer-management` reads every buffer against the live schedule: chain completed,
//!   buffer consumed, zone, alerts for a buffer in the red, and the recovery actions on record;
//! * `POST …/buffer-management/snapshots` records the readings (append-only history);
//! * `POST …/buffer-recovery-actions` and `PUT …/buffer-recovery-actions/{aid}/status` record what people
//!   agreed to do and close it once. Everything here is audited.
//!
//! Report, a person decides: nothing is re-planned and no action is invented.

use super::db_err;
use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::buffer_management::{self as rules, Reading, Spec};
use crate::fever;
use crate::models::_entities::{
    buffer_plans, buffer_recovery_actions as actions, buffer_snapshots,
};
use crate::models::audit_logs::Model as AuditModel;

fn json_err(e: &serde_json::Error) -> Error {
    Error::string(&e.to_string())
}

async fn latest(ctx: &AppContext, plan: Uuid) -> Result<Option<buffer_plans::Model>> {
    buffer_plans::Entity::find()
        .filter(buffer_plans::Column::PlanPid.eq(plan))
        .order_by_desc(buffer_plans::Column::Id)
        .one(&ctx.db)
        .await
        .map_err(db_err)
}

fn parse_doc(row: &buffer_plans::Model) -> Result<Value> {
    serde_json::from_str(&row.document).map_err(|e| json_err(&e))
}

/// The buffers in a frozen plan, as rules see them, with a label for each.
fn specs(doc: &Value) -> Vec<(Spec, Value)> {
    let mut out = Vec::new();
    let one = |v: &Value| {
        let tasks: Vec<(Uuid, i64)> = v["tasks"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|t| {
                Some((
                    Uuid::parse_str(t["pid"].as_str()?).ok()?,
                    t["days"].as_i64()?,
                ))
            })
            .collect();
        let planned = v["planned_finish"]
            .as_str()
            .and_then(|s| s.parse::<NaiveDate>().ok())?;
        let spec = Spec {
            key: v["key"].as_str()?.to_string(),
            days: v["days"].as_i64(),
            tasks,
            planned_finish: planned,
        };
        let label = json!({
            "tasks": v["tasks"].as_array().into_iter().flatten().map(|t| t["title"].clone()).collect::<Vec<_>>(),
            "joins": v["joins"]["title"],
        });
        Some((spec, label))
    };
    if let Some(p) = one(&doc["project"]) {
        out.push(p);
    }
    for f in doc["feeding"].as_array().into_iter().flatten() {
        if let Some(p) = one(f) {
            out.push(p);
        }
    }
    out
}

/// Every buffer in the frozen plan, read against the live schedule.
async fn readings(ctx: &AppContext, plan: Uuid, doc: &Value) -> Result<Vec<(Reading, Value)>> {
    let c = super::leveling::compute_with(ctx, plan, true, 100, true).await?;
    let (_, _, levelled) = super::critical_chain::schedules(&c)?;
    let done: HashSet<Uuid> = c
        .net
        .rows
        .iter()
        .filter(|t| t.status == "done")
        .map(|t| t.pid)
        .collect();
    let finish: HashMap<Uuid, NaiveDate> = levelled
        .nodes
        .iter()
        .map(|n| (n.pid, n.early_finish))
        .collect();
    let (lines, _) = fever::configured_lines();
    Ok(specs(doc)
        .into_iter()
        .map(|(spec, label)| {
            let forecast = spec.tasks.last().and_then(|(t, _)| finish.get(t)).copied();
            (
                rules::read(&spec, &done, forecast, &c.net.plan_cal, &lines),
                label,
            )
        })
        .collect())
}

fn reading_json(r: &Reading, label: &Value) -> Value {
    let mut v = serde_json::to_value(r).unwrap_or_default();
    v["kind"] = json!(if r.key == "project" {
        "project"
    } else {
        "feeding"
    });
    v["chain"] = label["tasks"].clone();
    v["joins"] = label["joins"].clone();
    if let Some(why) = r.no_point {
        v["no_point_reason"] = json!(why.reason());
    }
    v
}

fn alerts(rs: &[(Reading, Value)]) -> Vec<Value> {
    rs.iter()
        .filter(|(r, _)| r.zone() == Some("red"))
        .map(|(r, label)| {
            json!({
                "kind": if r.key == "project" { "project_buffer_red" } else { "feeding_buffer_red" },
                "buffer_key": r.key, "chain": label["tasks"], "joins": label["joins"],
                "buffer_consumed_pct": r.point.as_ref().map(|p| p.buffer_consumed_pct),
                "slip_days": r.slip_days, "buffer_days": r.buffer_days,
            })
        })
        .collect()
}

/// The project buffer's reading from the latest frozen buffer plan, if the plan has one. This is what
/// supersedes the fever chart's declared buffer and baselined critical path (DP-7).
pub(crate) async fn project_reading(
    ctx: &AppContext,
    plan: Uuid,
) -> Result<Option<(buffer_plans::Model, Reading)>> {
    let Some(row) = latest(ctx, plan).await? else {
        return Ok(None);
    };
    let rs = readings(ctx, plan, &parse_doc(&row)?).await?;
    Ok(rs
        .into_iter()
        .map(|(r, _)| r)
        .find(|r| r.key == "project")
        .map(|r| (row, r)))
}

/// `POST /api/plans/{pid}/buffer-plan` — freeze the chain and buffers as they are now planned.
#[debug_handler]
async fn freeze(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let (answer, doc) = super::buffers::build(&ctx, &caller, plan.pid).await?;
    let Some(doc) = doc else {
        return Err(unprocessable(&format!(
            "there are no buffers to freeze: {} (see critical-chain-buffers)",
            answer["reason"].as_str().unwrap_or("no critical chain")
        )));
    };
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = buffer_plans::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        taken_on: ActiveValue::set(crate::tz::deployment_today()),
        document: ActiveValue::set(doc.to_string()),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan.pid,
        "buffer_plan_frozen",
        caller.actor(),
        Some(json!({ "buffer_plan": row.pid })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(json!({ "pid": row.pid, "taken_on": row.taken_on, "document": doc }))
}

/// `GET /api/plans/{pid}/buffer-plan` — the latest frozen plan and when each was taken.
#[debug_handler]
async fn show_plan(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let all = buffer_plans::Entity::find()
        .filter(buffer_plans::Column::PlanPid.eq(plan.pid))
        .order_by_desc(buffer_plans::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let Some(first) = all.first() else {
        return format::json(
            json!({ "state": "no_buffer_plan", "reason": "no buffer plan has been frozen for this plan" }),
        );
    };
    format::json(json!({
        "state": "buffer_plan", "pid": first.pid, "taken_on": first.taken_on, "document": parse_doc(first)?,
        "history": all.iter().map(|r| json!({ "pid": r.pid, "taken_on": r.taken_on })).collect::<Vec<_>>(),
    }))
}

/// `GET /api/plans/{pid}/buffer-management`
#[debug_handler]
async fn manage(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let Some(row) = latest(&ctx, plan.pid).await? else {
        return format::json(
            json!({ "state": "no_buffer_plan", "reason": "no buffer plan has been frozen for this plan: POST buffer-plan once the chain and buffers are agreed" }),
        );
    };
    let doc = parse_doc(&row)?;
    let rs = readings(&ctx, plan.pid, &doc).await?;
    let (lines, unusable) = fever::configured_lines();
    let acts = actions::Entity::find()
        .filter(actions::Column::PlanPid.eq(plan.pid))
        .filter(actions::Column::BufferPlanPid.eq(row.pid))
        .order_by_asc(actions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let open: HashSet<&str> = acts
        .iter()
        .filter(|a| a.status == "open")
        .map(|a| a.buffer_key.as_str())
        .collect();
    let red: Vec<&str> = rs
        .iter()
        .filter(|(r, _)| r.zone() == Some("red"))
        .map(|(r, _)| r.key.as_str())
        .collect();
    let snaps = buffer_snapshots::Entity::find()
        .filter(buffer_snapshots::Column::PlanPid.eq(plan.pid))
        .filter(buffer_snapshots::Column::BufferPlanPid.eq(row.pid))
        .order_by_desc(buffer_snapshots::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let history: Vec<Value> = snaps
        .iter()
        .take(50)
        .map(|s| {
            let d: Value = serde_json::from_str(&s.document).unwrap_or_default();
            let zones: BTreeMap<String, Value> = d["readings"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|r| {
                    (
                        r["key"].as_str().unwrap_or("").to_string(),
                        r["point"]["zone"].clone(),
                    )
                })
                .collect();
            json!({ "pid": s.pid, "taken_on": s.taken_on, "zones": zones })
        })
        .collect();
    let people: Vec<&str> = acts
        .iter()
        .filter_map(|a| a.owner_ref.as_deref())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let label: HashMap<&str, String> = people
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, format!("person-{}", i + 1)))
        .collect();
    format::json(json!({
        "state": "readings",
        "buffer_plan": { "pid": row.pid, "taken_on": row.taken_on },
        "lines": lines, "configured_lines_unusable_default_used": unusable,
        "readings": rs.iter().map(|(r, l)| reading_json(r, l)).collect::<Vec<_>>(),
        "alerts": alerts(&rs),
        "red_without_open_recovery_action": red.iter().filter(|k| !open.contains(*k)).collect::<Vec<_>>(),
        "recovery_actions": acts.iter().map(|a| json!({
            "pid": a.pid, "buffer_key": a.buffer_key, "action": a.action, "due_date": a.due_date,
            "status": a.status, "closed_on": a.closed_on,
            "owner": a.owner_ref.as_deref().map(|o| label[o].clone()),
            "owner_ref": a.owner_ref.as_deref().filter(|o| super::availability::may_manage_person(&caller, o)),
        })).collect::<Vec<_>>(),
        "snapshots": history,
        "note": "Buffer consumed is the forecast slip of the chain's end over the buffer: the live schedule (every task at its aggressive figure, levelled for people's time) less the planned finish, in business days of the plan's calendar. Chain completed is the share of the chain's planned days whose tasks are done. Overruns only show when someone records them (a duration, a start date or an aggressive figure). Below the lower line is green and a point on a line takes the kinder zone; recovery (below 0) and overrun (above 100) keep their true figure. The service reports; a person decides what to do, and records it as a recovery action.",
    }))
}

/// `POST /api/plans/{pid}/buffer-management/snapshots` — record every buffer's reading now.
#[debug_handler]
async fn snapshot(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let Some(row) = latest(&ctx, plan.pid).await? else {
        return Err(unprocessable("freeze a buffer plan first"));
    };
    let rs = readings(&ctx, plan.pid, &parse_doc(&row)?).await?;
    let document = json!({
        "readings": rs.iter().map(|(r, l)| reading_json(r, l)).collect::<Vec<_>>(),
        "alerts": alerts(&rs),
    });
    let snap = buffer_snapshots::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        buffer_plan_pid: ActiveValue::set(row.pid),
        taken_on: ActiveValue::set(crate::tz::deployment_today()),
        document: ActiveValue::set(document.to_string()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "buffer_snapshot_taken",
        caller.actor(),
        Some(json!({ "snapshot": snap.pid })),
    )
    .await;
    for a in alerts(&rs) {
        AuditModel::record_or_log(&ctx.db, plan.pid, "buffer_red", caller.actor(), Some(a)).await;
    }
    format::json(json!({ "pid": snap.pid, "taken_on": snap.taken_on, "document": document }))
}

#[derive(Debug, Deserialize)]
struct ActionBody {
    buffer_key: String,
    action: String,
    owner_ref: String,
    #[serde(default)]
    due_date: Option<NaiveDate>,
}

/// `POST /api/plans/{pid}/buffer-recovery-actions`
#[debug_handler]
async fn add_action(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ActionBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let Some(row) = latest(&ctx, plan.pid).await? else {
        return Err(unprocessable("freeze a buffer plan first"));
    };
    let keys: Vec<String> = specs(&parse_doc(&row)?)
        .into_iter()
        .map(|(s, _)| s.key)
        .collect();
    let mut problems = Vec::new();
    if !keys.contains(&b.buffer_key) {
        problems.push(format!("buffer_key must be one of: {}", keys.join(", ")));
    }
    let text = b.action.trim();
    if !(5..=2000).contains(&text.chars().count()) {
        problems.push("action must be 5 to 2000 characters".to_string());
    }
    let owner = b.owner_ref.trim();
    if owner.is_empty() || owner.chars().count() > 200 {
        problems.push("owner_ref is required (up to 200 characters)".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let a = actions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        buffer_plan_pid: ActiveValue::set(row.pid),
        buffer_key: ActiveValue::set(b.buffer_key.clone()),
        action: ActiveValue::set(text.to_string()),
        owner_ref: ActiveValue::set(Some(owner.to_string())),
        due_date: ActiveValue::set(b.due_date),
        status: ActiveValue::set("open".to_string()),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan.pid,
        "buffer_recovery_action_recorded",
        caller.actor(),
        Some(json!({ "action": a.pid, "buffer_key": a.buffer_key })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(json!({ "pid": a.pid, "buffer_key": a.buffer_key, "status": a.status }))
}

#[derive(Debug, Deserialize)]
struct StatusBody {
    status: String,
}

/// `PUT /api/plans/{pid}/buffer-recovery-actions/{aid}/status` — close an open action as done or dropped.
#[debug_handler]
async fn close_action(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, aid)): Path<(String, String)>,
    Json(b): Json<StatusBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let aid = super::parse_pid(&aid)?;
    if !["done", "dropped"].contains(&b.status.as_str()) {
        return Err(unprocessable("status must be done or dropped"));
    }
    let a = actions::Entity::find()
        .filter(actions::Column::Pid.eq(aid))
        .filter(actions::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if a.status != "open" {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "already_closed",
                "this action is already closed; record a new one",
            ),
        ));
    }
    let mut m: actions::ActiveModel = a.into();
    m.status = ActiveValue::set(b.status.clone());
    m.closed_on = ActiveValue::set(Some(crate::tz::deployment_today()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = m.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        plan.pid,
        "buffer_recovery_action_closed",
        caller.actor(),
        Some(json!({ "action": row.pid, "status": row.status })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(json!({ "pid": row.pid, "status": row.status, "closed_on": row.closed_on }))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/buffer-plan", post(freeze))
        .add("/plans/{pid}/buffer-plan", get(show_plan))
        .add("/plans/{pid}/buffer-management", get(manage))
        .add("/plans/{pid}/buffer-management/snapshots", post(snapshot))
        .add("/plans/{pid}/buffer-recovery-actions", post(add_action))
        .add(
            "/plans/{pid}/buffer-recovery-actions/{aid}/status",
            put(close_action),
        )
}

//! Handover of a person's work on a plan (onboarding and offboarding).
//!
//! `POST /api/plans/{pid}/handover/preview` lists what a person holds on the plan
//! that is still live — **unfinished tasks, live bookings, RACI roles and open
//! risks they own** — and what each would become if handed to someone else (or
//! released, when no one is named). It writes **nothing**.
//! `POST …/handover/commit` applies exactly that, in one transaction, one audit
//! entry per item, and needs `If-Match: <handover_etag from the preview>`: if the
//! person's holdings changed since, it is refused `409` and nothing is written.
//!
//! About people, so it needs the permission to manage people. It moves ownership;
//! it does not check the new person's capacity (the leveling view does that).

use super::{db_err, unprocessable};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use chrono::{NaiveDate, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, QueryOrder, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::portfolio_resources::may_manage_people;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{allocations, raci_assignments, risks, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::workflow::WorkflowDef;

/// One thing a person holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Holding {
    /// `task`, `allocation`, `raci` or `risk`.
    kind: &'static str,
    pid: Uuid,
    /// What it is, in words.
    label: String,
}

/// What a person holds on a plan, ordered by kind then pid (so it is repeatable).
async fn holdings<C: ConnectionTrait>(
    conn: &C,
    plan: Uuid,
    person: &str,
    def: &WorkflowDef,
    today: NaiveDate,
) -> Result<Vec<Holding>> {
    let mut out = Vec::new();
    for t in tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::AssigneeRef.eq(person))
        .order_by_asc(tasks::Column::Pid)
        .all(conn)
        .await
        .map_err(db_err)?
    {
        if !crate::workflow::is_done(def, &t.status) {
            out.push(Holding {
                kind: "task",
                pid: t.pid,
                label: t.title,
            });
        }
    }
    for a in allocations::Entity::find()
        .filter(allocations::Column::PlanPid.eq(plan))
        .filter(allocations::Column::DeletedAt.is_null())
        .filter(allocations::Column::PersonRef.eq(person))
        .order_by_asc(allocations::Column::Pid)
        .all(conn)
        .await
        .map_err(db_err)?
    {
        if a.end_date.is_none_or(|e| e >= today) {
            out.push(Holding {
                kind: "allocation",
                pid: a.pid,
                label: format!(
                    "{}% booking{}",
                    a.percent,
                    a.role
                        .as_ref()
                        .map_or(String::new(), |r| format!(" as {r}"))
                ),
            });
        }
    }
    for r in raci_assignments::Entity::find()
        .filter(raci_assignments::Column::PlanPid.eq(plan))
        .filter(raci_assignments::Column::DeletedAt.is_null())
        .filter(raci_assignments::Column::PersonRef.eq(person))
        .order_by_asc(raci_assignments::Column::Pid)
        .all(conn)
        .await
        .map_err(db_err)?
    {
        out.push(Holding {
            kind: "raci",
            pid: r.pid,
            label: format!("RACI role {}", r.role),
        });
    }
    for r in risks::Entity::find()
        .filter(risks::Column::PlanPid.eq(plan))
        .filter(risks::Column::DeletedAt.is_null())
        .filter(risks::Column::OwnerRef.eq(person))
        .filter(risks::Column::Status.is_in(["open", "mitigating"]))
        .order_by_asc(risks::Column::Pid)
        .all(conn)
        .await
        .map_err(db_err)?
    {
        out.push(Holding {
            kind: "risk",
            pid: r.pid,
            label: r.title,
        });
    }
    Ok(out)
}

fn etag(person: &str, to: Option<&str>, h: &[Holding]) -> String {
    let mut parts = vec![person.to_string(), to.unwrap_or("-").to_string()];
    parts.extend(
        h.iter()
            .map(|x| format!("{}|{}|{}", x.kind, x.pid, x.label)),
    );
    crate::sandbox::fingerprint(&parts)
}

#[derive(Debug, Deserialize)]
struct Body {
    from_person: String,
    /// Who takes it; null releases the work (tasks unassigned, bookings ended).
    #[serde(default)]
    to_person: Option<String>,
    #[serde(default)]
    reason: Option<String>,
}

fn validate(caller: &MaybeAuthUser, b: &Body) -> Result<()> {
    if !may_manage_people(caller) {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new(
                "forbidden",
                "handover needs the permission to manage people",
            ),
        ));
    }
    let valid = |r: &str| super::governance::valid_ref(r, &["person", "worker"]);
    if !valid(&b.from_person) || b.to_person.as_deref().is_some_and(|t| !valid(t)) {
        return Err(unprocessable(
            "from_person and to_person must be person:/worker: URNs",
        ));
    }
    if b.to_person.as_deref() == Some(b.from_person.as_str()) {
        return Err(unprocessable("hand over to someone else"));
    }
    if b.reason.as_deref().is_some_and(|r| r.chars().count() > 500) {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    Ok(())
}

/// `POST /api/plans/{pid}/handover/preview` — what would move. Writes nothing.
#[debug_handler]
async fn preview(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Body>,
) -> Result<Response> {
    validate(&caller, &b)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let def = super::workflow::in_force(&ctx, plan.pid, "task").await?;
    let h = holdings(
        &ctx.db,
        plan.pid,
        &b.from_person,
        &def,
        crate::tz::deployment_today(),
    )
    .await?;
    let tag = etag(&b.from_person, b.to_person.as_deref(), &h);
    let counts: std::collections::BTreeMap<String, usize> = ["task", "allocation", "raci", "risk"]
        .iter()
        .map(|k| ((*k).to_string(), h.iter().filter(|x| x.kind == *k).count()))
        .collect();
    let mut res = format::json(serde_json::json!({
        "plan_pid": plan.pid, "from_person": b.from_person, "to_person": b.to_person,
        "handover_etag": tag,
        "action": if b.to_person.is_some() { "reassign" } else { "release" },
        "holdings": h,
        "counts": counts,
        "note": "Nothing has been changed. Unfinished tasks, live bookings, RACI roles and open risks held by this person move to the new person; with no new person, tasks and risks are unassigned, bookings and roles are removed. The new person's capacity is not checked here. Commit with If-Match set to handover_etag.",
    }))?;
    if let Ok(v) = header::HeaderValue::from_str(&tag) {
        res.headers_mut().insert(header::ETAG, v);
    }
    Ok(res)
}

/// `POST /api/plans/{pid}/handover/commit` — apply exactly the previewed handover.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn commit(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Json(b): Json<Body>,
) -> Result<Response> {
    validate(&caller, &b)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let Some(expected) = headers
        .get(header::IF_MATCH)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
    else {
        return Ok((
            StatusCode::PRECONDITION_REQUIRED,
            axum::Json(serde_json::json!({
                "error": "precondition_required",
                "description": "send If-Match with the handover_etag of the preview you are committing",
            })),
        )
            .into_response());
    };
    let stale = || {
        Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "stale_preview",
                "what this person holds changed since the preview: preview again, then commit",
            ),
        )
    };
    let def = super::workflow::in_force(&ctx, plan.pid, "task").await?;
    let today = crate::tz::deployment_today();
    let to = b.to_person.as_deref();
    // The freshness check is made through the transaction that writes, behind a lock
    // on the plan, so a second commit waits and then sees the first one's changes.
    let txn = ctx.db.begin().await.map_err(db_err)?;
    super::advisory_lock(&txn, "handover", plan.pid).await?;
    let h = holdings(&txn, plan.pid, &b.from_person, &def, today).await?;
    if etag(&b.from_person, to, &h) != expected {
        return Err(stale());
    }
    let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
    let mut entries = 0;
    for x in &h {
        let action = match (x.kind, to) {
            ("task", _) => {
                let row = tasks::Entity::find()
                    .filter(tasks::Column::Pid.eq(x.pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: tasks::ActiveModel = row.into();
                a.assignee_ref = ActiveValue::set(to.map(str::to_string));
                a.update(&txn).await.map_err(db_err)?;
                "task_handed_over"
            }
            ("risk", _) => {
                let row = risks::Entity::find()
                    .filter(risks::Column::Pid.eq(x.pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: risks::ActiveModel = row.into();
                a.owner_ref = ActiveValue::set(to.map(str::to_string));
                a.update(&txn).await.map_err(db_err)?;
                "risk_handed_over"
            }
            ("allocation", Some(t)) => {
                let row = allocations::Entity::find()
                    .filter(allocations::Column::Pid.eq(x.pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: allocations::ActiveModel = row.into();
                a.person_ref = ActiveValue::set(t.to_string());
                a.update(&txn).await.map_err(db_err)?;
                "allocation_handed_over"
            }
            ("allocation", None) => {
                let row = allocations::Entity::find()
                    .filter(allocations::Column::Pid.eq(x.pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: allocations::ActiveModel = row.into();
                a.deleted_at = ActiveValue::set(Some(now));
                a.update(&txn).await.map_err(db_err)?;
                "allocation_released"
            }
            ("raci", Some(t)) => {
                let row = raci_assignments::Entity::find()
                    .filter(raci_assignments::Column::Pid.eq(x.pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: raci_assignments::ActiveModel = row.into();
                a.person_ref = ActiveValue::set(t.to_string());
                a.update(&txn).await.map_err(db_err)?;
                "raci_handed_over"
            }
            _ => {
                let row = raci_assignments::Entity::find()
                    .filter(raci_assignments::Column::Pid.eq(x.pid))
                    .one(&txn)
                    .await
                    .map_err(db_err)?
                    .ok_or(Error::NotFound)?;
                let mut a: raci_assignments::ActiveModel = row.into();
                a.deleted_at = ActiveValue::set(Some(now));
                a.update(&txn).await.map_err(db_err)?;
                "raci_released"
            }
        };
        AuditModel::record(
            &txn,
            x.pid,
            action,
            caller.actor(),
            Some(
                serde_json::json!({ "plan_pid": plan.pid, "from_person": b.from_person,
                "to_person": b.to_person, "what": x.label, "reason": b.reason }),
            ),
        )
        .await
        .map_err(Error::Model)?;
        entries += 1;
    }
    txn.commit().await.map_err(db_err)?;
    format::json(serde_json::json!({
        "committed": true, "audit_entries": entries, "holdings": h,
        "from_person": b.from_person, "to_person": b.to_person,
    }))
}

/// The handover routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/handover/preview", post(preview))
        .add("/plans/{pid}/handover/commit", post(commit))
}

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

use super::unprocessable;
use crate::db::Session;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use chrono::{NaiveDate, Utc};
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
    /// Identifier of the held item (task, allocation, RACI row or risk).
    pid: Uuid,
    /// What it is, in words.
    label: String,
}

/// What a person holds on a plan, ordered by kind then pid (so it is repeatable).
async fn holdings(
    db: &mut impl Session,
    plan: Uuid,
    person: &str,
    def: &WorkflowDef,
    today: NaiveDate,
) -> Result<Vec<Holding>> {
    let mut out = Vec::new();
    for t in sqlx::query_as::<_, tasks::Model>(
        "SELECT * FROM tasks
         WHERE plan_pid = $1 AND deleted_at IS NULL AND assignee_ref = $2
         ORDER BY pid",
    )
    .bind(plan)
    .bind(person)
    .fetch_all(db.ex())
    .await?
    {
        if !crate::workflow::is_done(def, &t.status) {
            out.push(Holding {
                kind: "task",
                pid: t.pid,
                label: t.title,
            });
        }
    }
    for a in sqlx::query_as::<_, allocations::Model>(
        "SELECT * FROM allocations
         WHERE plan_pid = $1 AND deleted_at IS NULL AND person_ref = $2
         ORDER BY pid",
    )
    .bind(plan)
    .bind(person)
    .fetch_all(db.ex())
    .await?
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
    for r in sqlx::query_as::<_, raci_assignments::Model>(
        "SELECT * FROM raci_assignments
         WHERE plan_pid = $1 AND deleted_at IS NULL AND person_ref = $2
         ORDER BY pid",
    )
    .bind(plan)
    .bind(person)
    .fetch_all(db.ex())
    .await?
    {
        out.push(Holding {
            kind: "raci",
            pid: r.pid,
            label: format!("RACI role {}", r.role),
        });
    }
    for r in sqlx::query_as::<_, risks::Model>(
        "SELECT * FROM risks
         WHERE plan_pid = $1 AND deleted_at IS NULL AND owner_ref = $2
           AND status IN ('open', 'mitigating')
         ORDER BY pid",
    )
    .bind(plan)
    .bind(person)
    .fetch_all(db.ex())
    .await?
    {
        out.push(Holding {
            kind: "risk",
            pid: r.pid,
            label: r.title,
        });
    }
    Ok(out)
}

/// What a handed-over row's moved column becomes.
#[derive(Debug, Clone, Copy)]
enum Moved<'a> {
    /// The new holder (or nobody, for a released task or risk).
    Person(Option<&'a str>),
    /// Released: the row is soft-deleted at this moment.
    Deleted(chrono::DateTime<chrono::FixedOffset>),
}

/// Fingerprint of the person, target and the exact holdings found, so a commit with `If-Match` fails when anything changed since the preview.
fn etag(person: &str, to: Option<&str>, h: &[Holding]) -> String {
    let mut parts = vec![person.to_string(), to.unwrap_or("-").to_string()];
    parts.extend(
        h.iter()
            .map(|x| format!("{}|{}|{}", x.kind, x.pid, x.label)),
    );
    crate::sandbox::fingerprint(&parts)
}

/// Request body for previewing or committing a handover.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Body {
    /// `person:` or `worker:` URN of the person handing over work.
    from_person: String,
    /// Who takes it; null releases the work (tasks unassigned, bookings ended).
    #[serde(default)]
    to_person: Option<String>,
    /// Optional reason recorded in the audit entry, at most 500 characters.
    #[serde(default)]
    reason: Option<String>,
}

/// Check the caller may manage people and that the body is well formed: valid person URNs, a different recipient, and a short reason. Returns Forbidden or 422 otherwise.
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
        &mut crate::db::pool(&ctx),
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
        return Ok(super::problem(
            StatusCode::PRECONDITION_REQUIRED,
            "precondition_required",
            "send If-Match with the handover_etag of the preview you are committing",
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
    let mut tx = crate::db::begin(&ctx).await?;
    super::advisory_lock_on(&mut tx, "handover", plan.pid).await?;
    let h = holdings(&mut tx, plan.pid, &b.from_person, &def, today).await?;
    if etag(&b.from_person, to, &h) != expected {
        return Err(stale());
    }
    let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
    let mut entries = 0;
    for x in &h {
        // Each row is read by its pid, then written by its id naming only the column that moves.
        let (table, set, value, action) = match (x.kind, to) {
            ("task", _) => (
                "tasks",
                "assignee_ref",
                Moved::Person(to),
                "task_handed_over",
            ),
            ("risk", _) => ("risks", "owner_ref", Moved::Person(to), "risk_handed_over"),
            ("allocation", Some(t)) => (
                "allocations",
                "person_ref",
                Moved::Person(Some(t)),
                "allocation_handed_over",
            ),
            ("allocation", None) => (
                "allocations",
                "deleted_at",
                Moved::Deleted(now),
                "allocation_released",
            ),
            ("raci", Some(t)) => (
                "raci_assignments",
                "person_ref",
                Moved::Person(Some(t)),
                "raci_handed_over",
            ),
            _ => (
                "raci_assignments",
                "deleted_at",
                Moved::Deleted(now),
                "raci_released",
            ),
        };
        // The table and column come from the match above, never from the request.
        let id: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT id::int8 FROM {table} WHERE pid = $1 LIMIT 1"
        )))
        .bind(x.pid)
        .fetch_optional(tx.conn())
        .await?
        .ok_or(Error::NotFound)?;
        let update = sqlx::query(sqlx::AssertSqlSafe(format!(
            "UPDATE {table} SET {set} = $1 WHERE id = $2 RETURNING id"
        )));
        let update = match value {
            Moved::Person(p) => update.bind(p.map(str::to_string)),
            Moved::Deleted(at) => update.bind(Some(at)),
        };
        update.bind(id).fetch_one(tx.conn()).await?;
        AuditModel::record_on(
            tx.conn(),
            x.pid,
            action,
            caller.actor(),
            Some(
                serde_json::json!({ "plan_pid": plan.pid, "from_person": b.from_person,
                "to_person": b.to_person, "what": x.label, "reason": b.reason }),
            ),
        )
        .await?;
        entries += 1;
    }
    tx.commit().await?;
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

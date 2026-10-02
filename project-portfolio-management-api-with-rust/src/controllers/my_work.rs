//! "My work" (LT-3): `GET /api/me/tasks` — the caller's open tasks,
//! bucketed by due date in **the caller's own zone** (`?tz=`). Pure
//! bucketing in [`crate::my_work`], zone arithmetic in [`crate::tz`].
//!
//! Identity is the token `sub`, addressed as `person:<sub>` or
//! `worker:<sub>` — the same `EntityRef` forms a task's `assignee_ref`
//! takes — and only when the `sub` is a UUID; anything else cannot be
//! mapped to a reference and gets empty buckets with a stated reason,
//! never a guess. A task in a plan the caller may not read is **omitted**,
//! not shown masked.

use std::collections::{BTreeMap, HashMap};

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::models::_entities::{plans, tasks};
use crate::my_work::{self as rules, Bucket};
use crate::tz;

/// Rows read per request; a person with more open tasks is told so.
const ROW_CAP: u64 = 500;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

/// `GET /api/me/tasks?tz=&as_of=` query.
#[derive(Debug, Deserialize)]
struct MyTasksParams {
    /// IANA zone, e.g. `Europe/London`. Required: the server will not
    /// guess a zone for you (time-zone doctrine).
    tz: String,
    /// Evaluate as of this RFC 3339 instant instead of now — for "what
    /// will my day look like at …" and for deterministic tests.
    #[serde(default)]
    as_of: Option<String>,
}

/// `GET /api/me/tasks` — see the module docs.
#[debug_handler]
#[allow(clippy::too_many_lines)] // one linear pipeline: resolve, fetch, filter, group
async fn my_tasks(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(params): Query<MyTasksParams>,
) -> Result<Response> {
    let zone = tz::parse_zone(&params.tz).map_err(|e| unprocessable(&e))?;
    let now = match params.as_of.as_deref() {
        Some(raw) => chrono::DateTime::parse_from_rfc3339(raw)
            .map_err(|_| unprocessable("as_of must be an RFC 3339 instant"))?
            .to_utc(),
        None => chrono::Utc::now(),
    };
    let today = tz::local_today(now, zone);

    let refs: Vec<String> = Uuid::parse_str(&claims.sub)
        .map(|u| vec![format!("person:{u}"), format!("worker:{u}")])
        .unwrap_or_default();
    let (rows, truncated) = if refs.is_empty() {
        (Vec::new(), false)
    } else {
        let mut rows = tasks::Entity::find()
            .filter(tasks::Column::AssigneeRef.is_in(refs.clone()))
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::DoneAt.is_null())
            .order_by_asc(tasks::Column::Id)
            .limit(ROW_CAP + 1)
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?;
        let truncated = rows.len() as u64 > ROW_CAP;
        rows.truncate(usize::try_from(ROW_CAP).unwrap_or(usize::MAX));
        (rows, truncated)
    };

    // Resolve each distinct plan once: its name, and whether this caller
    // may read it (record-level ABAC, keyed on the plan's stage).
    let plan_pids: Vec<Uuid> = {
        let mut v: Vec<Uuid> = rows.iter().map(|t| t.plan_pid).collect();
        v.sort();
        v.dedup();
        v
    };
    let plan_rows = if plan_pids.is_empty() {
        Vec::new()
    } else {
        plans::Entity::find()
            .filter(plans::Column::Pid.is_in(plan_pids))
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?
    };
    let caller = crate::auth::MaybeAuthUser(Some(claims.clone()));
    // RM-8: tasks in a private plan the caller is not a member of are omitted.
    let hidden = super::plan_access::hidden_plans(
        &ctx,
        &plan_rows.iter().map(|p| p.pid).collect::<Vec<_>>(),
        &caller,
    )
    .await;
    let readable: HashMap<Uuid, String> = plan_rows
        .into_iter()
        .filter(|p| !hidden.contains(&p.pid))
        .filter(|p| {
            crate::auth::authorize_record(
                &caller,
                authentication_verifier::Action::Read,
                &crate::auth::plan_resource_attrs(p.stage.as_deref()),
            )
            .is_ok()
        })
        .map(|p| (p.pid, p.name))
        .collect();

    let items: Vec<(Option<chrono::NaiveDate>, serde_json::Value)> = rows
        .iter()
        .filter_map(|t| {
            readable.get(&t.plan_pid).map(|plan_name| {
                (
                    t.due_date,
                    serde_json::json!({
                        "pid": t.pid,
                        "title": t.title,
                        "status": t.status,
                        "due_date": t.due_date,
                        "points": t.points,
                        "plan_pid": t.plan_pid,
                        "plan_name": plan_name,
                    }),
                )
            })
        })
        .collect();
    let omitted = rows.len() - items.len();
    let grouped = rules::group(items, today);
    let counts: BTreeMap<&str, usize> = grouped.iter().map(|(b, v)| (b.key(), v.len())).collect();
    let buckets: BTreeMap<&str, &Vec<serde_json::Value>> =
        grouped.iter().map(|(b, v)| (b.key(), v)).collect();
    debug_assert_eq!(buckets.len(), Bucket::ALL.len());

    format::json(serde_json::json!({
        "tz": zone.name(),
        "as_of": now,
        "today": today,
        "buckets": buckets,
        "counts": counts,
        "total": counts.values().sum::<usize>(),
        // Disclosed, never silent: rows left out because the plan is not
        // readable by this caller, or because the row cap was hit.
        "omitted_unreadable": omitted,
        "truncated": truncated,
        "reason": refs.is_empty().then_some("sub_not_addressable"),
    }))
}

/// The personal-view routes.
pub fn routes() -> Routes {
    Routes::new().prefix("/api").add("/me/tasks", get(my_tasks))
}

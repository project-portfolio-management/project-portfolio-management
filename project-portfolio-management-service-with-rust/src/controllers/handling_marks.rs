//! Handling marks on plans (RG-7). Rules and the policy statement in [`crate::handling_marks`]; the
//! hiding itself is in `plan_access` (the guard layer and the shared scope), so every plan route, list
//! and aggregate follows one decision.
//!
//! * `GET /api/handling-marks` — the configured scheme (empty list when there is none).
//! * `GET /api/plans/{pid}/handling-mark` — a plan's mark.
//! * `PUT /api/plans/{pid}/handling-mark` — set it, or reset it with `{ "mark": null }`.

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use super::plan_access::clearance_of;
use crate::auth::MaybeAuthUser;
use crate::handling_marks::scheme;
use crate::models::_entities::plan_handling_marks as marks;
use crate::models::audit_logs::Model as AuditModel;

/// The stored handling mark row for a plan, if one was ever set; None means the plan is at the scheme's lowest level.
async fn current(ctx: &AppContext, plan: Uuid) -> Result<Option<marks::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plan_handling_marks WHERE plan_pid = $1 LIMIT 1")
            .bind(plan)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// JSON view of a plan's handling mark: the scheme levels, the stored mark (null when none) and the `effective` level the scheme resolves it to.
fn view(plan: Uuid, stored: Option<&str>) -> serde_json::Value {
    let s = scheme();
    serde_json::json!({
        "plan_pid": plan,
        "scheme": s.levels(),
        "mark": stored,
        "effective": s.effective(stored),
    })
}

/// `GET /api/handling-marks` — the scheme, lowest first.
#[debug_handler]
async fn get_scheme() -> Result<Response> {
    format::json(serde_json::json!({ "scheme": scheme().levels(), "on": scheme().is_on() }))
}

/// `GET /api/plans/{pid}/handling-mark`.
#[debug_handler]
async fn get_mark(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = current(&ctx, plan.pid).await?;
    format::json(view(plan.pid, row.as_ref().map(|r| r.mark.as_str())))
}

/// Request body for setting a plan's handling mark.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SetBody {
    /// A code in the scheme, or `null` to reset to the lowest level.
    mark: Option<String>,
}

/// `PUT /api/plans/{pid}/handling-mark`.
#[debug_handler]
async fn put_mark(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<SetBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let existing = current(&ctx, plan.pid).await?;
    let clearance = clearance_of(caller.claims());
    scheme()
        .can_set(
            existing.as_ref().map(|r| r.mark.as_str()),
            b.mark.as_deref(),
            clearance.as_deref(),
        )
        .map_err(|m| {
            // A scheme that is off is "not found"; a clearance problem is a refusal the caller may read.
            let status = if scheme().is_on() {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::NOT_FOUND
            };
            Error::CustomError(status, ErrorDetail::new("handling_mark", m))
        })?;
    let was = existing.as_ref().map(|r| r.mark.clone());
    let mut tx = crate::db::begin(&ctx).await?;
    match (existing, b.mark.as_deref()) {
        (Some(row), Some(new)) => {
            // The two columns `SeaORM`'s update set, by id; no row back is an error, as before.
            let _: i64 = sqlx::query_scalar(
                "UPDATE plan_handling_marks SET mark = $1, set_by = $2 WHERE id = $3 RETURNING id",
            )
            .bind(new)
            .bind(caller.actor())
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
        }
        (Some(row), None) => {
            // As `SeaORM`'s `delete_by_id`: no row deleted is not an error.
            sqlx::query("DELETE FROM plan_handling_marks WHERE id = $1")
                .bind(row.id)
                .execute(tx.conn())
                .await?;
        }
        (None, Some(new)) => {
            sqlx::query(
                "INSERT INTO plan_handling_marks (pid, plan_pid, mark, set_by) VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::new_v4())
            .bind(plan.pid)
            .bind(new)
            .bind(caller.actor())
            .execute(tx.conn())
            .await?;
        }
        (None, None) => {}
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "handling_mark_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": b.mark })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(plan.pid, b.mark.as_deref()))
}

/// The handling-mark routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/handling-marks", get(get_scheme))
        .add("/plans/{pid}/handling-mark", get(get_mark))
        .add("/plans/{pid}/handling-mark", put(put_mark))
}

//! Handling marks on plans (RG-7). Rules and the policy statement in [`crate::handling_marks`]; the
//! hiding itself is in `plan_access` (the guard layer and the shared scope), so every plan route, list
//! and aggregate follows one decision.
//!
//! * `GET /api/handling-marks` — the configured scheme (empty list when there is none).
//! * `GET /api/plans/{pid}/handling-mark` — a plan's mark.
//! * `PUT /api/plans/{pid}/handling-mark` — set it, or reset it with `{ "mark": null }`.

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use super::governance::find_item;
use super::plan_access::clearance_of;
use crate::auth::MaybeAuthUser;
use crate::handling_marks::scheme;
use crate::models::_entities::plan_handling_marks as marks;
use crate::models::audit_logs::Model as AuditModel;

async fn current(ctx: &AppContext, plan: Uuid) -> Result<Option<marks::Model>> {
    marks::Entity::find()
        .filter(marks::Column::PlanPid.eq(plan))
        .one(&ctx.db)
        .await
        .map_err(db_err)
}

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

#[derive(Debug, Deserialize)]
struct SetBody {
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
    match (existing, b.mark.as_deref()) {
        (Some(row), Some(new)) => {
            let mut a: marks::ActiveModel = row.into();
            a.mark = ActiveValue::set(new.to_string());
            a.set_by = ActiveValue::set(caller.actor().map(str::to_string));
            a.update(&ctx.db).await.map_err(db_err)?;
        }
        (Some(row), None) => {
            marks::Entity::delete_by_id(row.id)
                .exec(&ctx.db)
                .await
                .map_err(db_err)?;
        }
        (None, Some(new)) => {
            marks::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                mark: ActiveValue::set(new.to_string()),
                set_by: ActiveValue::set(caller.actor().map(str::to_string)),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
        (None, None) => {}
    }
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "handling_mark_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": b.mark })),
    )
    .await;
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

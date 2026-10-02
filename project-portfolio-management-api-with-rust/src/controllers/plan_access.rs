//! Private plans (RM-8): mark a plan private and name who may see it. Pure
//! rules in [`crate::plan_access`].
//!
//! **What is enforced.** Any request addressed to a plan —
//! `/api/plans/{pid}` and everything beneath it — passes a guard layer that
//! answers `404` to a non-member. Plan **lists, search and duplicate checks**
//! omit private plans for non-members. Discussion, watchers, approvals,
//! custom fields and "My work" resolve their subjects through the same
//! check. A member (or an `access=admin` caller) sees the plan everywhere.
//!
//! **What is NOT enforced — read this before relying on it.** Cross-plan
//! *aggregate* views (at-a-glance, executive, board, financials, reports,
//! capacity, scenarios, …) are computed over every plan and **still include a
//! private plan's figures**. "Private" here hides a plan's *records*, not its
//! contribution to portfolio totals. Closing that gap means filtering every
//! aggregate query and is tracked in the repo `tasks.md`.

use std::collections::HashSet;

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::plan_access_settings as plan_access;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_access as rules;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn members_of(row: &plan_access::Model) -> Vec<String> {
    serde_json::from_value(row.members.clone()).unwrap_or_default()
}

/// Whether a set of claims holds `access=admin`.
fn is_admin(claims: Option<&authentication_verifier::Claims>) -> bool {
    claims.is_some_and(|c| {
        c.attrs
            .get("access")
            .is_some_and(|v| v.iter().any(|a| a == "admin"))
    })
}

/// The plan pids among `pids` that are private **and** hidden from `caller`.
pub(crate) async fn hidden_plans(
    ctx: &AppContext,
    pids: &[Uuid],
    caller: &MaybeAuthUser,
) -> HashSet<Uuid> {
    if pids.is_empty() {
        return HashSet::new();
    }
    let Ok(rows) = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.is_in(pids.to_vec()))
        .filter(plan_access::Column::Private.eq(true))
        .all(&ctx.db)
        .await
    else {
        // Fail closed on a lookup error: hide what we could not check.
        return pids.iter().copied().collect();
    };
    let admin = is_admin(caller.claims());
    rows.into_iter()
        .filter(|r| !rules::can_see(true, &members_of(r), caller.actor(), admin))
        .map(|r| r.plan_pid)
        .collect()
}

/// The plan a path addresses: `/api/plans/{uuid}` or anything beneath it.
fn plan_pid_of(path: &str) -> Option<Uuid> {
    let rest = path.strip_prefix("/api/plans/")?;
    Uuid::parse_str(rest.split('/').next()?).ok()
}

/// The guard layer: a request addressed to a private plan by a non-member is
/// `404` (existence is not disclosed). Every other request passes untouched,
/// and a lookup failure fails **closed** for a private plan only.
pub async fn privacy_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> AxumResponse {
    if let Some(plan_pid) = plan_pid_of(req.uri().path()) {
        let private = plan_access::Entity::find()
            .filter(plan_access::Column::PlanPid.eq(plan_pid))
            .filter(plan_access::Column::Private.eq(true))
            .one(&ctx.db)
            .await;
        if let Ok(Some(row)) = private {
            let claims = crate::auth::request_claims(req.headers()).await.ok();
            let sub = claims.as_ref().map(|c| c.sub.as_str());
            if !rules::can_see(true, &members_of(&row), sub, is_admin(claims.as_ref())) {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(serde_json::json!({ "error": "not_found" })),
                )
                    .into_response();
            }
        }
    }
    next.run(req).await
}

/// `GET /api/plans/{pid}/access`.
#[debug_handler]
async fn get_access(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "private": row.as_ref().is_some_and(|r| r.private),
        "members": row.as_ref().map(members_of).unwrap_or_default(),
    }))
}

/// `PUT /api/plans/{pid}/access` body.
#[derive(Debug, Deserialize)]
struct AccessPayload {
    private: bool,
    #[serde(default)]
    members: Vec<String>,
}

/// `PUT /api/plans/{pid}/access` — make a plan private (with its members) or
/// public again. The caller is always added as a member when making a plan
/// private, so a plan can never be locked away from the person who locked it.
/// Only someone who can already see the plan reaches this (the guard layer).
#[debug_handler]
async fn put_access(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(p): Json<AccessPayload>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let mut members = rules::normalise_members(&p.members).map_err(|m| unprocessable(&m))?;
    if p.private {
        let me = Uuid::parse_str(&claims.sub)
            .map_err(|_| {
                unprocessable(
                    "sub_not_addressable: a private plan needs a member identity that is a user id",
                )
            })?
            .to_string();
        if !members.contains(&me) {
            members.push(me);
        }
    }
    let existing = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let before = existing.as_ref().map(|r| (r.private, members_of(r)));
    match existing {
        Some(row) => {
            let mut active: plan_access::ActiveModel = row.into();
            active.private = ActiveValue::set(p.private);
            active.members = ActiveValue::set(serde_json::json!(members));
            active.update(&ctx.db).await.map_err(db_err)?;
        }
        None => {
            plan_access::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                private: ActiveValue::set(p.private),
                members: ActiveValue::set(serde_json::json!(members)),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
    }
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "plan_access_changed",
        Some(&claims.sub),
        Some(serde_json::json!({
            "before": before,
            "after": { "private": p.private, "members": members },
        })),
    )
    .await
    .ok();
    format::json(
        serde_json::json!({ "plan_pid": plan.pid, "private": p.private, "members": members }),
    )
}

/// The plan-access routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/access", get(get_access))
        .add("/plans/{pid}/access", put(put_access))
}

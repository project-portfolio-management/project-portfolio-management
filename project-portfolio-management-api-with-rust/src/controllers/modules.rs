//! Per-plan feature modules (RM-7): read and set which modules a plan
//! exposes, and the guard that hides a disabled module's plan-scoped
//! routes. Pure rules in [`crate::modules`].
//!
//! The guard is a middleware layer, not a check in each handler, so **no
//! gated route can forget it** — and it runs only for paths that belong to
//! a module, so every other request pays nothing.

use std::collections::BTreeSet;

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::plan_modules;
use crate::models::audit_logs::Model as AuditModel;
use crate::modules as rules;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

/// The plan's explicit module set, if it has chosen one.
async fn explicit_modules(
    db: &DatabaseConnection,
    plan_pid: Uuid,
) -> std::result::Result<Option<BTreeSet<String>>, sea_orm::DbErr> {
    let row = plan_modules::Entity::find()
        .filter(plan_modules::Column::PlanPid.eq(plan_pid))
        .one(db)
        .await?;
    Ok(row.map(|r| {
        serde_json::from_value::<Vec<String>>(r.modules)
            .unwrap_or_default()
            .into_iter()
            .collect()
    }))
}

/// The modules in force for a plan, and whether they are its own choice.
pub(crate) async fn modules_in_force(
    db: &DatabaseConnection,
    plan_pid: Uuid,
) -> std::result::Result<(BTreeSet<String>, bool), sea_orm::DbErr> {
    let explicit = explicit_modules(db, plan_pid).await?;
    let own = explicit.is_some();
    Ok((
        rules::effective(explicit.as_ref(), &rules::deployment_default()),
        own,
    ))
}

/// The guard: a plan-scoped path owned by a module that is off for that
/// plan answers `404 module_disabled`. Everything else passes through.
pub async fn modules_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> AxumResponse {
    let gated = rules::parse_plan_path(req.uri().path())
        .and_then(|(pid, seg)| rules::module_for_segment(seg).map(|m| (pid, seg.to_string(), m)));
    if let Some((plan_pid, segment, module)) = gated {
        match modules_in_force(&ctx.db, plan_pid).await {
            Ok((modules, _)) if !rules::segment_enabled(&modules, &segment) => {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(serde_json::json!({
                        "error": "module_disabled",
                        "module": module,
                        "message": format!("the {module} module is off for this plan"),
                    })),
                )
                    .into_response();
            }
            // Enabled — or the lookup failed: fail open. Hiding a feature
            // because of a transient error would be worse than showing it.
            Ok(_) | Err(_) => {}
        }
    }
    next.run(req).await
}

/// `GET /api/plans/{pid}/modules`.
#[debug_handler]
async fn get_modules(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let (modules, own) = modules_in_force(&ctx.db, plan.pid).await.map_err(db_err)?;
    format::json(view(plan.pid, &modules, own))
}

fn view(plan_pid: Uuid, modules: &BTreeSet<String>, own: bool) -> serde_json::Value {
    serde_json::json!({
        "plan_pid": plan_pid,
        "source": if own { "plan" } else { "default" },
        "modules": modules,
        "available": rules::MODULES.iter().map(|m| serde_json::json!({
            "key": m.key,
            "enabled": modules.contains(m.key),
            "segments": m.segments,
        })).collect::<Vec<_>>(),
        "default": rules::deployment_default(),
        "personal_preset": rules::PERSONAL,
    })
}

/// `PUT /api/plans/{pid}/modules` body: exactly one of `modules`,
/// `preset`, or `reset`.
#[derive(Debug, Deserialize)]
struct ModulesPayload {
    #[serde(default)]
    modules: Option<Vec<String>>,
    #[serde(default)]
    preset: Option<String>,
    #[serde(default)]
    reset: Option<bool>,
}

/// Persist (or clear) a plan's set and audit the change.
pub(crate) async fn set_modules(
    ctx: &AppContext,
    plan_pid: Uuid,
    new: Option<BTreeSet<String>>,
    actor: Option<&str>,
) -> Result<()> {
    let before = explicit_modules(&ctx.db, plan_pid).await.map_err(db_err)?;
    match &new {
        Some(set) => {
            let value = serde_json::json!(set);
            let existing = plan_modules::Entity::find()
                .filter(plan_modules::Column::PlanPid.eq(plan_pid))
                .one(&ctx.db)
                .await
                .map_err(db_err)?;
            if let Some(row) = existing {
                let mut active: plan_modules::ActiveModel = row.into();
                active.modules = ActiveValue::set(value);
                active.update(&ctx.db).await.map_err(db_err)?;
            } else {
                plan_modules::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    plan_pid: ActiveValue::set(plan_pid),
                    modules: ActiveValue::set(value),
                    ..Default::default()
                }
                .insert(&ctx.db)
                .await
                .map_err(db_err)?;
            }
        }
        None => {
            plan_modules::Entity::delete_many()
                .filter(plan_modules::Column::PlanPid.eq(plan_pid))
                .exec(&ctx.db)
                .await
                .map_err(db_err)?;
        }
    }
    AuditModel::record(
        &ctx.db,
        plan_pid,
        "plan_modules_changed",
        actor,
        Some(serde_json::json!({ "before": before, "after": new })),
    )
    .await
    .ok();
    Ok(())
}

/// `PUT /api/plans/{pid}/modules` — set the plan's modules, apply the
/// personal preset, or reset to the deployment default.
#[debug_handler]
async fn put_modules(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<ModulesPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let choices = usize::from(payload.modules.is_some())
        + usize::from(payload.preset.is_some())
        + usize::from(payload.reset == Some(true));
    if choices != 1 {
        return Err(unprocessable(
            "send exactly one of: modules (a list), preset (\"personal\"), reset (true)",
        ));
    }
    let new: Option<BTreeSet<String>> = if let Some(list) = payload.modules {
        let problems = rules::problems(&list);
        if !problems.is_empty() {
            return Err(unprocessable(&problems.join("; ")));
        }
        Some(list.into_iter().collect())
    } else if let Some(preset) = payload.preset {
        if preset != "personal" {
            return Err(unprocessable("unknown preset (allowed: \"personal\")"));
        }
        Some(rules::PERSONAL.iter().map(|s| (*s).to_string()).collect())
    } else {
        None
    };
    set_modules(&ctx, plan.pid, new, caller.actor()).await?;
    let (modules, own) = modules_in_force(&ctx.db, plan.pid).await.map_err(db_err)?;
    format::json(view(plan.pid, &modules, own))
}

/// The module routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/modules", get(get_modules))
        .add("/plans/{pid}/modules", put(put_modules))
}

//! Per-plan feature modules (RM-7): read and set which modules a plan
//! exposes, and the guard that hides a disabled module's plan-scoped
//! routes. Pure rules in [`crate::modules`].
//!
//! The guard is a middleware layer, not a check in each handler, so **no
//! gated route can forget it** — and it runs only for paths that belong to
//! a module, so every other request pays nothing.

use super::unprocessable;
use std::collections::BTreeSet;

use crate::prelude::*;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::audit_logs::Model as AuditModel;
use crate::modules as rules;

/// The plan's explicit module set, if it has chosen one.
async fn explicit_modules(
    ctx: &AppContext,
    plan_pid: Uuid,
) -> std::result::Result<Option<BTreeSet<String>>, sqlx::Error> {
    // Only the set is read.
    let row: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT modules FROM plan_modules WHERE plan_pid = $1 LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?;
    Ok(row.map(|modules| {
        serde_json::from_value::<Vec<String>>(modules)
            .unwrap_or_default()
            .into_iter()
            .collect()
    }))
}

/// The modules in force for a plan, and whether they are its own choice.
pub(crate) async fn modules_in_force(
    ctx: &AppContext,
    plan_pid: Uuid,
) -> std::result::Result<(BTreeSet<String>, bool), sqlx::Error> {
    let explicit = explicit_modules(ctx, plan_pid).await?;
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
        match modules_in_force(&ctx, plan_pid).await {
            Ok((modules, _)) if !rules::segment_enabled(&modules, &segment) => {
                return super::problem(
                    StatusCode::NOT_FOUND,
                    "module_disabled",
                    format!("the {module} module is off for this plan"),
                )
                .with_extension("module", serde_json::json!(module))
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
    let (modules, own) = modules_in_force(&ctx, plan.pid).await?;
    format::json(view(plan.pid, &modules, own))
}

/// JSON view of a plan's modules: the enabled set, whether it comes from the plan itself (`own`) or the deployment default, and every available module with its enabled flag and URL segments.
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ModulesPayload {
    /// Explicit list of module keys to enable.
    #[serde(default)]
    modules: Option<Vec<String>>,
    /// Name of a preset set of modules to apply.
    #[serde(default)]
    preset: Option<String>,
    /// True clears the plan's own set so it falls back to the deployment default.
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
    let before = explicit_modules(ctx, plan_pid).await?;
    let mut tx = crate::db::begin(ctx).await?;
    match &new {
        Some(set) => {
            let value = serde_json::json!(set);
            // Only the id is read.
            let existing: Option<i64> =
                sqlx::query_scalar("SELECT id FROM plan_modules WHERE plan_pid = $1 LIMIT 1")
                    .bind(plan_pid)
                    .fetch_optional(tx.conn())
                    .await?;
            if let Some(id) = existing {
                // The one column the update set before SQ-1, by id; no row back is an error, as before.
                let _: i64 = sqlx::query_scalar(
                    "UPDATE plan_modules SET modules = $1 WHERE id = $2 RETURNING id",
                )
                .bind(&value)
                .bind(id)
                .fetch_one(tx.conn())
                .await?;
            } else {
                sqlx::query(
                    "INSERT INTO plan_modules (pid, plan_pid, modules) VALUES ($1, $2, $3)",
                )
                .bind(Uuid::new_v4())
                .bind(plan_pid)
                .bind(&value)
                .execute(tx.conn())
                .await?;
            }
        }
        None => {
            sqlx::query("DELETE FROM plan_modules WHERE plan_pid = $1")
                .bind(plan_pid)
                .execute(tx.conn())
                .await?;
        }
    }
    AuditModel::record_on(
        tx.conn(),
        plan_pid,
        "plan_modules_changed",
        actor,
        Some(serde_json::json!({ "before": before, "after": new })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    let (modules, own) = modules_in_force(&ctx, plan.pid).await?;
    format::json(view(plan.pid, &modules, own))
}

/// The module routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/modules", get(get_modules))
        .add("/plans/{pid}/modules", put(put_modules))
}

//! A programme's profile: `PUT`/`GET /api/plans/{pid}/delivery-model` (EX-2). The delivery model and the
//! organization that owns the delivery, for a plan of kind Program. Changes are audited. The portfolio map
//! and so the director pack show both.

use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::delivery_model as rules;
use crate::models::_entities::programme_profiles as profiles;
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/delivery-model", put(set))
        .add("/plans/{pid}/delivery-model", get(show))
        .add("/plans/{pid}/reporting-method", put(set_method))
}

/// Request body setting a programme profile.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Body {
    /// One of the three models; `null` clears it.
    #[serde(default)]
    delivery_model: Option<String>,
    /// The organization that owns the delivery; `null` clears it.
    #[serde(default)]
    delivery_owner_organization_pid: Option<Uuid>,
}

/// Load the profile row of a plan, if one has been set.
async fn find(ctx: &AppContext, plan: Uuid) -> Result<Option<profiles::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM programme_profiles WHERE plan_pid = $1 LIMIT 1")
            .bind(plan)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// A profile as JSON, with the owner's name.
pub(crate) fn view(p: Option<&profiles::Model>, names: &HashMap<Uuid, String>) -> Value {
    let model = p.and_then(|p| p.delivery_model.as_deref());
    json!({
        "delivery_model": model,
        "delivery_owner": p.and_then(|p| p.delivery_organization_pid).map(|o| json!({ "organization_pid": o, "name": names.get(&o) })),
        "delivered_by_organization": model.map(rules::delivered_by_organization),
        "reporting_method": p.map_or("evm", |p| p.method.as_str()),
    })
}

/// Every profile, and the organization names, for the portfolio map.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn all(
    ctx: &AppContext,
    plans: &HashSet<Uuid>,
) -> Result<(HashMap<Uuid, profiles::Model>, HashMap<Uuid, String>)> {
    let rows: HashMap<Uuid, profiles::Model> =
        sqlx::query_as::<_, profiles::Model>("SELECT * FROM programme_profiles")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .filter(|p| plans.contains(&p.plan_pid))
            .map(|p| (p.plan_pid, p))
            .collect();
    let names = sqlx::query_as::<_, (Uuid, String)>("SELECT pid, name FROM organizations")
        .fetch_all(crate::db::pool(ctx))
        .await?
        .into_iter()
        .collect();
    Ok((rows, names))
}

/// `PUT /api/plans/{pid}/delivery-model`
#[debug_handler]
async fn set(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Body>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(
            "only a plan of kind Program has a delivery model",
        ));
    }
    if let Some(m) = &b.delivery_model
        && !rules::MODELS.contains(&m.as_str())
    {
        return Err(unprocessable(&format!(
            "delivery_model must be one of {:?}",
            rules::MODELS
        )));
    }
    if let Some(o) = b.delivery_owner_organization_pid {
        let ok = sqlx::query("SELECT 1 FROM organizations WHERE pid = $1 AND active = $2 LIMIT 1")
            .bind(o)
            .bind(true)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .is_some();
        if !ok {
            return Err(unprocessable(
                "delivery_owner_organization_pid is not an active organization",
            ));
        }
    }
    let existing = find(&ctx, plan.pid).await?;
    let was = view(existing.as_ref(), &HashMap::new());
    let mut tx = crate::db::begin(&ctx).await?;
    let row: profiles::Model = if let Some(e) = existing {
        sqlx::query_as(
            "UPDATE programme_profiles SET delivery_model = $1, delivery_organization_pid = $2
             WHERE id = $3
             RETURNING *",
        )
        .bind(&b.delivery_model)
        .bind(b.delivery_owner_organization_pid)
        .bind(e.id)
        .fetch_one(tx.conn())
        .await?
    } else {
        sqlx::query_as(
            "INSERT INTO programme_profiles (pid, plan_pid, delivery_model, delivery_organization_pid, method)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(plan.pid)
        .bind(&b.delivery_model)
        .bind(b.delivery_owner_organization_pid)
        .bind("evm")
        .fetch_one(tx.conn())
        .await?
    };
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "delivery_model_set",
        caller.actor(),
        Some(json!({ "from": was["delivery_model"], "to": b.delivery_model, "owner": b.delivery_owner_organization_pid })),
    ).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (_, names) = all(&ctx, &HashSet::from([plan.pid])).await?;
    format::json(view(Some(&row), &names))
}

/// `GET /api/plans/{pid}/delivery-model`
#[debug_handler]
async fn show(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let row = find(&ctx, plan.pid).await?;
    let (_, names) = all(&ctx, &HashSet::from([plan.pid])).await?;
    format::json(view(row.as_ref(), &names))
}

/// Request body choosing how the plan reports progress.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MethodBody {
    /// `evm` or `product`.
    method: String,
}

/// `PUT /api/plans/{pid}/reporting-method` (EX-14) — how a programme reports in the pack: earned value, or
/// burn-up and flow for a product programme. A change is audited with what it was.
#[debug_handler]
async fn set_method(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<MethodBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(
            "only a plan of kind Program has a reporting method",
        ));
    }
    if !["evm", "product"].contains(&b.method.as_str()) {
        return Err(unprocessable("method must be evm or product"));
    }
    let existing = find(&ctx, plan.pid).await?;
    let was = existing
        .as_ref()
        .map_or("evm".to_string(), |e| e.method.clone());
    // As before SQ-1: with no change of method the transaction is dropped uncommitted, so nothing is written.
    let mut tx = crate::db::begin(&ctx).await?;
    let row: profiles::Model = if let Some(e) = existing {
        sqlx::query_as("UPDATE programme_profiles SET method = $1 WHERE id = $2 RETURNING *")
            .bind(&b.method)
            .bind(e.id)
            .fetch_one(tx.conn())
            .await?
    } else {
        sqlx::query_as(
            "INSERT INTO programme_profiles (pid, plan_pid, method) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(plan.pid)
        .bind(&b.method)
        .fetch_one(tx.conn())
        .await?
    };
    if was != b.method {
        AuditModel::record_on(
            tx.conn(),
            plan.pid,
            "reporting_method_changed",
            caller.actor(),
            Some(json!({ "from": was, "to": b.method })),
        )
        .await
        .map_err(super::audit_failed)?;
        tx.commit().await?;
    }
    format::json(json!({ "plan_pid": plan.pid, "reporting_method": row.method }))
}

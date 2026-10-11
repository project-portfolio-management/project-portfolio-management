//! A programme's profile: `PUT`/`GET /api/plans/{pid}/delivery-model` (EX-2). The delivery model and the
//! organization that owns the delivery, for a plan of kind Program. Changes are audited. The portfolio map
//! and so the director pack show both.

use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::delivery_model as rules;
use crate::models::_entities::{organizations, programme_profiles as profiles};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/delivery-model", put(set))
        .add("/plans/{pid}/delivery-model", get(show))
}

#[derive(Debug, Deserialize)]
struct Body {
    /// One of the three models; `null` clears it.
    #[serde(default)]
    delivery_model: Option<String>,
    /// The organization that owns the delivery; `null` clears it.
    #[serde(default)]
    delivery_owner_organization_pid: Option<Uuid>,
}

async fn find(ctx: &AppContext, plan: Uuid) -> Result<Option<profiles::Model>> {
    profiles::Entity::find()
        .filter(profiles::Column::PlanPid.eq(plan))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))
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
    let rows: HashMap<Uuid, profiles::Model> = profiles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|p| plans.contains(&p.plan_pid))
        .map(|p| (p.plan_pid, p))
        .collect();
    let names = organizations::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|o| (o.pid, o.name))
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
        return Err(unprocessable("only a plan of kind Program has a delivery model"));
    }
    if let Some(m) = &b.delivery_model
        && !rules::MODELS.contains(&m.as_str())
    {
        return Err(unprocessable(&format!("delivery_model must be one of {:?}", rules::MODELS)));
    }
    if let Some(o) = b.delivery_owner_organization_pid {
        let ok = organizations::Entity::find()
            .filter(organizations::Column::Pid.eq(o))
            .filter(organizations::Column::Active.eq(true))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .is_some();
        if !ok {
            return Err(unprocessable("delivery_owner_organization_pid is not an active organization"));
        }
    }
    let existing = find(&ctx, plan.pid).await?;
    let was = view(existing.as_ref(), &HashMap::new());
    let row = if let Some(e) = existing {
        let mut a: profiles::ActiveModel = e.into();
        a.delivery_model = ActiveValue::set(b.delivery_model.clone());
        a.delivery_organization_pid = ActiveValue::set(b.delivery_owner_organization_pid);
        a.update(&ctx.db).await.map_err(|e| db_err(&e))?
    } else {
        profiles::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            delivery_model: ActiveValue::set(b.delivery_model.clone()),
            delivery_organization_pid: ActiveValue::set(b.delivery_owner_organization_pid),
            method: ActiveValue::set("evm".to_string()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    };
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "delivery_model_set",
        caller.actor(),
        Some(json!({ "from": was["delivery_model"], "to": b.delivery_model, "owner": b.delivery_owner_organization_pid })),
    )
    .await
    .ok();
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

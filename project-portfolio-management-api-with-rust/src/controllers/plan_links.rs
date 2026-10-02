//! Typed links on a plan (LT-8). Pure rules in [`crate::plan_links`]. The
//! service stores a link and returns it; it never fetches or renders one.

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::plan_links;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_links as rules;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

async fn live(ctx: &AppContext, plan: Uuid) -> Result<Vec<plan_links::Model>> {
    plan_links::Entity::find()
        .filter(plan_links::Column::PlanPid.eq(plan))
        .filter(plan_links::Column::DeletedAt.is_null())
        .order_by_asc(plan_links::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// `POST /api/plans/{pid}/links` body.
#[derive(Debug, Deserialize)]
struct LinkBody {
    kind: String,
    url: String,
    label: String,
}

/// `POST /api/plans/{pid}/links`.
#[debug_handler]
async fn add_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<LinkBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let problems = rules::problems(&b.kind, &b.url, &b.label);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = live(&ctx, plan.pid).await?;
    if existing.len() >= rules::MAX_PER_PLAN {
        return Err(unprocessable(&format!(
            "a plan holds at most {} links",
            rules::MAX_PER_PLAN
        )));
    }
    if existing.iter().any(|l| l.url == b.url) {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "this URL is already linked from the plan"),
        ));
    }
    let row = plan_links::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        kind: ActiveValue::set(b.kind.clone()),
        url: ActiveValue::set(b.url.clone()),
        label: ActiveValue::set(b.label.trim().to_string()),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(&ctx.db, row.pid, "plan_link_added", caller.actor(), None)
        .await
        .ok();
    format::json(row)
}

/// `GET /api/plans/{pid}/links`.
#[debug_handler]
async fn list_links(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    format::json(live(&ctx, plan.pid).await?)
}

/// `POST /api/plans/{pid}/links/{lpid}/delete` (and `DELETE`).
#[debug_handler]
async fn remove_link(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, lpid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let lpid = Uuid::parse_str(&lpid).map_err(|_| Error::NotFound)?;
    let row = plan_links::Entity::find()
        .filter(plan_links::Column::Pid.eq(lpid))
        .filter(plan_links::Column::PlanPid.eq(plan.pid))
        .filter(plan_links::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut a: plan_links::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(&ctx.db, lpid, "plan_link_removed", caller.actor(), None)
        .await
        .ok();
    format::empty_json()
}

/// The plan-link routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/links", post(add_link))
        .add("/plans/{pid}/links", get(list_links))
        .add("/plans/{pid}/links/{lpid}", delete(remove_link))
        .add("/plans/{pid}/links/{lpid}/delete", post(remove_link))
}

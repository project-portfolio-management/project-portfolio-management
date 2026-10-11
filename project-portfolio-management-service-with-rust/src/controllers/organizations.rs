//! Organizations (EX-3): `POST`/`GET /api/organizations` and `PUT /api/organizations/{pid}`.
//!
//! An organization is a record: the delivery organization, a partner body, a government sponsor or a
//! supplier. A capacity commitment may name one, an organization can be the audience of a director pack
//! (`org:<pid>`), and a caller whose token carries the `organization` attribute is that organization:
//! the attribute is for policy to use, and the service itself uses it in two places (a partner's list of
//! commitments, and the packs addressed to its organization).

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::organizations as orgs;
use crate::models::audit_logs::Model as AuditModel;

/// The kinds of organization.
pub const KINDS: [&str; 4] = ["delivery", "partner", "sponsor", "supplier"];

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    if e.to_string().contains("org_name_once") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "an organization with that name exists"),
        );
    }
    Error::string(&e.to_string())
}

fn show(o: &orgs::Model) -> serde_json::Value {
    json!({ "pid": o.pid, "name": o.name, "kind": o.kind, "active": o.active, "audience": format!("org:{}", o.pid) })
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/organizations", get(list))
        .add("/organizations", post(create))
        .add("/organizations/{pid}", put(update))
}

/// `GET /api/organizations`
#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = orgs::Entity::find()
        .order_by_asc(orgs::Column::Name)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(json!({ "organizations": rows.iter().map(show).collect::<Vec<_>>() }))
}

#[derive(Debug, Deserialize)]
struct NewOrg {
    name: String,
    kind: String,
}

fn validate(name: &str, kind: &str) -> Result<()> {
    let mut problems = Vec::new();
    if !(1..=200).contains(&name.trim().chars().count()) {
        problems.push("name must be 1 to 200 characters".to_string());
    }
    if !KINDS.contains(&kind) {
        problems.push(format!("kind must be one of {KINDS:?}"));
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(unprocessable(&problems.join("; ")))
    }
}

/// `POST /api/organizations`
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewOrg>,
) -> Result<Response> {
    validate(&b.name, &b.kind)?;
    let row = orgs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(b.name.trim().to_string()),
        kind: ActiveValue::set(b.kind),
        active: ActiveValue::set(true),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record(&ctx.db, row.pid, "organization_created", caller.actor(), Some(json!({ "kind": row.kind })))
        .await
        .ok();
    format::json(show(&row))
}

#[derive(Debug, Deserialize)]
struct Change {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    active: Option<bool>,
}

/// `PUT /api/organizations/{pid}` — rename, change kind, retire or restore.
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Change>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let row = orgs::Entity::find()
        .filter(orgs::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    validate(b.name.as_deref().unwrap_or(&row.name), b.kind.as_deref().unwrap_or(&row.kind))?;
    let mut a: orgs::ActiveModel = row.into();
    if let Some(n) = b.name {
        a.name = ActiveValue::set(n.trim().to_string());
    }
    if let Some(k) = b.kind {
        a.kind = ActiveValue::set(k);
    }
    if let Some(v) = b.active {
        a.active = ActiveValue::set(v);
    }
    let row = a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
    AuditModel::record(&ctx.db, row.pid, "organization_changed", caller.actor(), Some(json!({ "active": row.active })))
        .await
        .ok();
    format::json(show(&row))
}

/// Whether `audience` is one a pack can be addressed to: a fixed audience, or `org:<pid>` of an active
/// organization.
///
/// # Errors
///
/// When the database read fails.
pub(crate) async fn audience_valid(ctx: &AppContext, audience: &str) -> Result<bool> {
    if crate::director_pack::AUDIENCES.contains(&audience) {
        return Ok(true);
    }
    let Some(pid) = audience.strip_prefix("org:").and_then(|p| Uuid::parse_str(p).ok()) else {
        return Ok(false);
    };
    Ok(orgs::Entity::find()
        .filter(orgs::Column::Pid.eq(pid))
        .filter(orgs::Column::Active.eq(true))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .is_some())
}

/// The organizations a caller belongs to: the values of their `organization` attribute.
pub(crate) fn caller_organizations(caller: &MaybeAuthUser) -> Vec<Uuid> {
    caller
        .claims()
        .and_then(|c| c.attrs.get("organization").cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|v| Uuid::parse_str(v).ok())
        .collect()
}

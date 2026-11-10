//! Who holds which role on a plan (OC-3), and what that lets them do there. The rules are in [`crate::plan_roles`].
//!
//! Plan-scoped, so the privacy guard answers for a hidden plan. Reading is open to anyone who can see the plan; only an
//! administrator changes membership. Every change is audited against the plan.

use crate::prelude::*;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use super::{conflict, forbidden, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    organization_roles, plan_access_settings, plan_role_members as members,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_roles as rules;

/// The most memberships one plan's list returns; `truncated` says when there were more.
const MEMBER_LIST_MAX: i64 = 1_000;

/// The roles a token carries.
#[must_use]
pub(crate) fn token_roles(caller: &MaybeAuthUser) -> Vec<String> {
    caller
        .claims()
        .and_then(|c| c.attrs.get("role").cloned())
        .unwrap_or_default()
}

/// The catalogue as the rules need it. A stored list that cannot be read carries nothing.
pub(crate) fn role_of(row: &organization_roles::Model) -> rules::Role {
    rules::Role {
        key: row.key.clone(),
        permissions: serde_json::from_value(row.permissions.clone()).unwrap_or_default(),
        retired: row.retired_at.is_some(),
    }
}

/// What `caller` holds on `plan`. A lookup failure is an error, so an unknown standing never opens anything.
pub(crate) async fn standing(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
) -> Result<rules::Standing> {
    let held: Vec<rules::Member> =
        sqlx::query_as("SELECT person_ref, role_key FROM plan_role_members WHERE plan_pid = $1")
            .bind(plan)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let catalogue: Vec<rules::Role> =
        sqlx::query_as::<_, organization_roles::Model>("SELECT * FROM organization_roles")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .iter()
            .map(role_of)
            .collect();
    Ok(rules::standing(
        super::plan_access::is_admin(caller.claims()),
        &token_roles(caller),
        caller.actor(),
        &held,
        &catalogue,
    ))
}

/// `403` unless the standing allows every permission named, naming the first one missing.
pub(crate) fn require(standing: &rules::Standing, needed: &[&str]) -> Result<()> {
    match needed.iter().find(|p| !standing.may(p)) {
        Some(missing) => Err(forbidden(&format!(
            "this plan requires the `{missing}` permission, and none of your roles here carries it"
        ))),
        None => Ok(()),
    }
}

/// The live plan a path names; `404` otherwise.
async fn require_plan(ctx: &AppContext, plan: &str) -> Result<Uuid> {
    let pid = super::parse_pid(plan)?;
    sqlx::query_scalar::<_, i64>(
        "SELECT id FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)?;
    Ok(pid)
}

/// JSON shape of one membership.
fn view(m: &members::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": m.pid, "plan_pid": m.plan_pid, "person_ref": m.person_ref, "role_key": m.role_key,
        "added_by": m.added_by, "added_at": m.created_at,
    })
}

/// `GET /api/plans/{pid}/role-members` — who holds which role here, and whether the plan is enforced.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(plan): Path<String>) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let mut rows: Vec<members::Model> =
        sqlx::query_as("SELECT * FROM plan_role_members WHERE plan_pid = $1 ORDER BY id LIMIT $2")
            .bind(plan)
            .bind(MEMBER_LIST_MAX + 1)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let truncated = i64::try_from(rows.len()).unwrap_or(i64::MAX) > MEMBER_LIST_MAX;
    rows.truncate(usize::try_from(MEMBER_LIST_MAX).unwrap_or(1000));
    format::json(serde_json::json!({
        "plan_pid": plan, "enforced": !rows.is_empty(),
        "members": rows.iter().map(view).collect::<Vec<_>>(),
        "truncated": truncated,
    }))
}

/// `POST /api/plans/{pid}/role-members` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewMember {
    /// The person's user id, as their token's subject carries it.
    person_ref: String,
    /// A role in `/api/organization-roles` that is not retired.
    role_key: String,
}

/// `422` if `person` could not see this plan. A role does not open a private plan, so the membership would do nothing.
async fn require_can_see(ctx: &AppContext, plan: Uuid, person: &str) -> Result<()> {
    let private: Option<plan_access_settings::Model> = sqlx::query_as(
        "SELECT * FROM plan_access_settings WHERE plan_pid = $1 AND private = $2 LIMIT 1",
    )
    .bind(plan)
    .bind(true)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    let Some(row) = private else {
        return Ok(());
    };
    let listed: Vec<String> = serde_json::from_value(row.members).unwrap_or_default();
    if crate::plan_access::can_see(true, &listed, Some(person), false) {
        Ok(())
    } else {
        Err(unprocessable(
            "this plan is private: add the person to its access list first",
        ))
    }
}

/// `POST /api/plans/{pid}/role-members` — give someone a role on this plan (an administrator). The plan's first member
/// turns enforcement on, so everyone else who can write loses issue actions here. `409` if they already hold it.
#[debug_handler]
async fn add(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(plan): Path<String>,
    Json(b): Json<NewMember>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    if !super::plan_access::is_admin(caller.claims()) {
        return Err(forbidden(
            "only an administrator can change who holds a role",
        ));
    }
    let person = Uuid::parse_str(b.person_ref.trim())
        .map_err(|_| unprocessable("person_ref must be a user id (a UUID)"))?
        .to_string();
    let role: organization_roles::Model = sqlx::query_as(
        "SELECT * FROM organization_roles WHERE key = $1 AND retired_at IS NULL LIMIT 1",
    )
    .bind(b.role_key.as_str())
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or_else(|| unprocessable("role_key must be a role that is not retired"))?;
    require_can_see(&ctx, plan, &person).await?;
    let held: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM plan_role_members
         WHERE plan_pid = $1 AND person_ref = $2 AND role_key = $3
         LIMIT 1",
    )
    .bind(plan)
    .bind(person.as_str())
    .bind(role.key.as_str())
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    if held.is_some() {
        return Err(conflict("this person already holds this role here"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: members::Model = sqlx::query_as(
        "INSERT INTO plan_role_members (pid, plan_pid, person_ref, role_key, added_by)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan)
    .bind(person)
    .bind(role.key)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan,
        "plan_role_added",
        caller.actor(),
        Some(serde_json::json!({ "person_ref": row.person_ref, "role_key": row.role_key })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, format::json(view(&row))?).into_response())
}

/// `DELETE /api/plans/{pid}/role-members/{member_pid}` — take a role away (an administrator). Removing the plan's last
/// member turns enforcement off. The audit row keeps who held it.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((plan, pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    if !super::plan_access::is_admin(caller.claims()) {
        return Err(forbidden(
            "only an administrator can change who holds a role",
        ));
    }
    let pid = super::parse_pid(&pid)?;
    let row: members::Model =
        sqlx::query_as("SELECT * FROM plan_role_members WHERE pid = $1 AND plan_pid = $2 LIMIT 1")
            .bind(pid)
            .bind(plan)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    let detail = serde_json::json!({ "person_ref": row.person_ref, "role_key": row.role_key });
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("DELETE FROM plan_role_members WHERE id = $1")
        .bind(row.id)
        .execute(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        plan,
        "plan_role_removed",
        caller.actor(),
        Some(detail),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// The plan role routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/role-members", get(list))
        .add("/plans/{pid}/role-members", post(add))
        .add("/plans/{pid}/role-members/{member_pid}", delete(remove))
}

//! Organization-defined issue types and roles (OC-1), and what a plan's issue form may offer.
//!
//! An administrator changes the catalogues; anyone signed in can read them. Statuses are not here: an issue's statuses
//! are the states of its workflow ([`super::workflow`]), which also says who may make each move.
//!
//! **A role only ever narrows.** Its permissions decide what a caller may do to issues on a plan with role members, within
//! what the token already allows (OC-3, [`crate::plan_roles`]); it never makes anyone an administrator. The catalogue also
//! names the roles a deployment recognizes, so that a transition cannot be restricted to a misspelt one, which would lock
//! everyone out of that move.
//!
//! Types and roles are retired, never deleted: an issue or a workflow that names one stays valid.

use axa::prelude::*;
use axum::http::StatusCode;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::{conflict, db_err, forbidden, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::issues as rules;
use crate::models::_entities::{issue_types, organization_roles, plan_issue_types, plans};
use crate::models::audit_logs::Model as AuditModel;

use crate::issues::LABEL_MAX;

/// A label per language.
type Labels = std::collections::BTreeMap<String, String>;
/// Longest role description, in characters.
const DESCRIPTION_MAX: usize = 500;

/// `403` unless the caller holds the administrator attribute.
fn require_admin(caller: &MaybeAuthUser) -> Result<()> {
    if super::plan_access::is_admin(caller.claims()) {
        Ok(())
    } else {
        Err(forbidden("only an administrator can change this catalogue"))
    }
}

/// `422` unless a label is present and short enough. Returns it trimmed.
fn usable_label(label: &str) -> Result<String> {
    let label = label.trim();
    if label.is_empty() || label.chars().count() > LABEL_MAX {
        return Err(unprocessable(&format!(
            "label is required, at most {LABEL_MAX} characters"
        )));
    }
    Ok(label.to_string())
}

/// `422` unless translated labels can be stored. Each is trimmed.
pub(crate) fn usable_labels(labels: &Labels) -> Result<Labels> {
    let problems = rules::label_problems(labels);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    Ok(labels
        .iter()
        .map(|(code, text)| (code.clone(), text.trim().to_string()))
        .collect())
}

/// `422` unless `key` is a type that is not retired: what a person may choose for an issue or a workflow.
pub(crate) async fn require_usable_type(ctx: &AppContext, key: &str) -> Result<()> {
    let found = issue_types::Entity::find()
        .filter(issue_types::Column::Key.eq(key))
        .filter(issue_types::Column::RetiredAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    if found.is_some() {
        return Ok(());
    }
    let usable = issue_types::Entity::find()
        .filter(issue_types::Column::RetiredAt.is_null())
        .order_by_asc(issue_types::Column::Position)
        .order_by_asc(issue_types::Column::Key)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let keys: Vec<&str> = usable.iter().map(|t| t.key.as_str()).collect();
    Err(unprocessable(&format!("kind must be one of {keys:?}")))
}

/// The types a plan has limited itself to, in key order. Empty means every type.
async fn chosen_types(ctx: &AppContext, plan: Uuid) -> Result<Vec<String>> {
    Ok(plan_issue_types::Entity::find()
        .filter(plan_issue_types::Column::PlanPid.eq(plan))
        .order_by_asc(plan_issue_types::Column::TypeKey)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|t| t.type_key)
        .collect())
}

/// `422` unless a person may choose `key` for an issue on this plan: a usable type the plan has not left out. An issue
/// that already carries a type stays valid when the plan stops offering it, as it does when the type is retired.
pub(crate) async fn require_offered_type(ctx: &AppContext, plan: Uuid, key: &str) -> Result<()> {
    require_usable_type(ctx, key).await?;
    let chosen = chosen_types(ctx, plan).await?;
    if chosen.is_empty() || chosen.iter().any(|t| t == key) {
        Ok(())
    } else {
        Err(unprocessable(&format!(
            "this plan offers only these issue types: {chosen:?}"
        )))
    }
}

/// `422` if the catalogue is in use and a role is not in it. An empty catalogue checks nothing, so a deployment that
/// has not defined its roles is unchanged, and a retired role is refused, since no new rule should depend on it.
pub(crate) async fn require_known_roles(ctx: &AppContext, named: &[&str]) -> Result<()> {
    let catalogue = organization_roles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if catalogue.is_empty() {
        return Ok(());
    }
    let unknown: Vec<&str> = named
        .iter()
        .copied()
        .filter(|name| {
            !catalogue
                .iter()
                .any(|r| r.key == *name && r.retired_at.is_none())
        })
        .collect();
    if unknown.is_empty() {
        Ok(())
    } else {
        Err(unprocessable(&format!(
            "not roles this organization recognizes: {unknown:?}"
        )))
    }
}

/// `?include_retired=` on a catalogue.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// Also list what has been retired; the default lists only what can be chosen.
    #[serde(default)]
    include_retired: bool,
}

/// `POST /api/issue-types` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewType {
    /// Stable key, stored on issues: a lowercase letter, then lowercase letters, digits or `_`, up to 40 characters.
    key: String,
    /// Name shown to people.
    label: String,
    /// The name by language code, for languages that should not fall back to `label`.
    #[serde(default)]
    labels: Labels,
    /// Order in lists, lowest first.
    #[serde(default)]
    position: i32,
}

/// `PUT /api/issue-types/{key}` body. The key cannot change: issues store it.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ChangeType {
    /// Name shown to people.
    label: String,
    /// The name by language code; absent keeps them.
    #[serde(default)]
    labels: Option<Labels>,
    /// Order in lists, lowest first; absent keeps it.
    #[serde(default)]
    position: Option<i32>,
    /// `true` retires it (it can no longer be chosen; issues that carry it keep it), `false` restores it; absent keeps it.
    #[serde(default)]
    retired: Option<bool>,
}

/// JSON shape of one type.
fn type_view(t: &issue_types::Model) -> serde_json::Value {
    serde_json::json!({
        "key": t.key, "label": t.label, "labels": t.labels, "position": t.position,
        "retired": t.retired_at.is_some(),
    })
}

/// `GET /api/issue-types` — the issue types, in order.
#[debug_handler]
async fn list_types(
    State(ctx): State<AppContext>,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let mut find = issue_types::Entity::find();
    if !q.include_retired {
        find = find.filter(issue_types::Column::RetiredAt.is_null());
    }
    let rows = find
        .order_by_asc(issue_types::Column::Position)
        .order_by_asc(issue_types::Column::Key)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(type_view).collect::<Vec<_>>())
}

/// `POST /api/issue-types` — add a type (an administrator). `409` if the key is taken, retired types included.
#[debug_handler]
async fn create_type(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewType>,
) -> Result<Response> {
    require_admin(&caller)?;
    let key = b.key.trim();
    if !rules::is_type_key(key) {
        return Err(unprocessable(
            "key must be a lowercase letter, then lowercase letters, digits or `_`, up to 40 characters",
        ));
    }
    let label = usable_label(&b.label)?;
    let labels = usable_labels(&b.labels)?;
    let taken = issue_types::Entity::find()
        .filter(issue_types::Column::Key.eq(key))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    if taken.is_some() {
        return Err(conflict("an issue type with this key already exists"));
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = issue_types::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        key: ActiveValue::set(key.to_string()),
        label: ActiveValue::set(label),
        labels: ActiveValue::set(serde_json::json!(labels)),
        position: ActiveValue::set(b.position),
        retired_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "issue_type_added",
        caller.actor(),
        Some(serde_json::json!({ "key": row.key, "label": row.label })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    Ok((StatusCode::CREATED, format::json(type_view(&row))?).into_response())
}

/// `PUT /api/issue-types/{key}` — rename, reorder, retire or restore a type (an administrator).
#[debug_handler]
async fn change_type(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(key): Path<String>,
    Json(b): Json<ChangeType>,
) -> Result<Response> {
    require_admin(&caller)?;
    let label = usable_label(&b.label)?;
    let labels = b.labels.as_ref().map(usable_labels).transpose()?;
    let row = issue_types::Entity::find()
        .filter(issue_types::Column::Key.eq(key.as_str()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let was = type_view(&row);
    let position = b.position.unwrap_or(row.position);
    let retired_at = match (b.retired, row.retired_at) {
        (Some(true), None) => Some(chrono::Utc::now().into()),
        (Some(false), _) => None,
        (_, kept) => kept,
    };
    let mut a: issue_types::ActiveModel = row.into();
    a.label = ActiveValue::set(label);
    if let Some(labels) = labels {
        a.labels = ActiveValue::set(serde_json::json!(labels));
    }
    a.position = ActiveValue::set(position);
    a.retired_at = ActiveValue::set(retired_at);
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "issue_type_changed",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": type_view(&row) })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(type_view(&row))
}

/// `POST /api/organization-roles` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewRole {
    /// The name exactly as a token's `role` attribute carries it: lowercase letters, digits, `_` or `-`, up to 64
    /// characters, starting with a letter.
    key: String,
    /// Name shown to people.
    label: String,
    /// The name by language code, for languages that should not fall back to `label`.
    #[serde(default)]
    labels: Labels,
    /// What the role is for.
    #[serde(default)]
    description: Option<String>,
    /// What the role allows on a plan with role members: `issue_add`, `issue_edit`, `issue_move`, `issue_delete`.
    #[serde(default)]
    permissions: Vec<String>,
    /// Order in lists, lowest first.
    #[serde(default)]
    position: i32,
}

/// `PUT /api/organization-roles/{key}` body. The key cannot change: tokens and workflows carry it.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ChangeRole {
    /// Name shown to people.
    label: String,
    /// The name by language code; absent keeps them.
    #[serde(default)]
    labels: Option<Labels>,
    /// What the role is for.
    #[serde(default)]
    description: Option<String>,
    /// What the role allows on a plan with role members; absent keeps it.
    #[serde(default)]
    permissions: Option<Vec<String>>,
    /// Order in lists, lowest first; absent keeps it.
    #[serde(default)]
    position: Option<i32>,
    /// `true` retires it (no new workflow rule can name it, it carries no permissions, and holding it on a plan counts for
    /// nothing; a gate a token's role already satisfies keeps working), `false` restores it; absent keeps it.
    #[serde(default)]
    retired: Option<bool>,
}

/// JSON shape of one role.
fn role_view(r: &organization_roles::Model) -> serde_json::Value {
    serde_json::json!({
        "key": r.key, "label": r.label, "labels": r.labels, "description": r.description,
        "permissions": r.permissions,
        "position": r.position, "retired": r.retired_at.is_some(),
    })
}

/// A role's description, trimmed; blank is absent. `422` if too long.
fn usable_description(text: Option<&String>) -> Result<Option<String>> {
    let text = text.map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    if text
        .as_ref()
        .is_some_and(|t| t.chars().count() > DESCRIPTION_MAX)
    {
        return Err(unprocessable(&format!(
            "description is at most {DESCRIPTION_MAX} characters"
        )));
    }
    Ok(text)
}

/// `422` unless every entry is a permission. Returns them deduplicated, in a stable order.
fn usable_permissions(requested: &[String]) -> Result<Vec<String>> {
    let unknown = crate::plan_roles::unknown_permissions(requested);
    if !unknown.is_empty() {
        return Err(unprocessable(&format!(
            "not permissions: {unknown:?}; must be among {:?}",
            crate::plan_roles::PERMISSIONS
        )));
    }
    let mut kept = requested.to_vec();
    kept.sort();
    kept.dedup();
    Ok(kept)
}

/// `GET /api/organization-roles` — the roles this deployment recognizes, with what each allows, in order.
#[debug_handler]
async fn list_roles(
    State(ctx): State<AppContext>,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let mut find = organization_roles::Entity::find();
    if !q.include_retired {
        find = find.filter(organization_roles::Column::RetiredAt.is_null());
    }
    let rows = find
        .order_by_asc(organization_roles::Column::Position)
        .order_by_asc(organization_roles::Column::Key)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(role_view).collect::<Vec<_>>())
}

/// `POST /api/organization-roles` — add a role (an administrator). `409` if the key is taken.
#[debug_handler]
async fn create_role(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewRole>,
) -> Result<Response> {
    require_admin(&caller)?;
    let key = b.key.trim();
    if !rules::is_role_key(key) {
        return Err(unprocessable(
            "key must be a lowercase letter, then lowercase letters, digits, `_` or `-`, up to 64 characters",
        ));
    }
    let label = usable_label(&b.label)?;
    let labels = usable_labels(&b.labels)?;
    let description = usable_description(b.description.as_ref())?;
    let permissions = usable_permissions(&b.permissions)?;
    let taken = organization_roles::Entity::find()
        .filter(organization_roles::Column::Key.eq(key))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    if taken.is_some() {
        return Err(conflict("a role with this key already exists"));
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = organization_roles::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        key: ActiveValue::set(key.to_string()),
        label: ActiveValue::set(label),
        labels: ActiveValue::set(serde_json::json!(labels)),
        description: ActiveValue::set(description),
        permissions: ActiveValue::set(serde_json::json!(permissions)),
        position: ActiveValue::set(b.position),
        retired_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "organization_role_added",
        caller.actor(),
        Some(serde_json::json!({
            "key": row.key, "label": row.label, "permissions": row.permissions,
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    Ok((StatusCode::CREATED, format::json(role_view(&row))?).into_response())
}

/// `PUT /api/organization-roles/{key}` — rename, reorder, retire or restore a role (an administrator).
#[debug_handler]
async fn change_role(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(key): Path<String>,
    Json(b): Json<ChangeRole>,
) -> Result<Response> {
    require_admin(&caller)?;
    let label = usable_label(&b.label)?;
    let labels = b.labels.as_ref().map(usable_labels).transpose()?;
    let description = usable_description(b.description.as_ref())?;
    let permissions = b
        .permissions
        .as_deref()
        .map(usable_permissions)
        .transpose()?;
    let row = organization_roles::Entity::find()
        .filter(organization_roles::Column::Key.eq(key.as_str()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let was = role_view(&row);
    let position = b.position.unwrap_or(row.position);
    let retired_at = match (b.retired, row.retired_at) {
        (Some(true), None) => Some(chrono::Utc::now().into()),
        (Some(false), _) => None,
        (_, kept) => kept,
    };
    let mut a: organization_roles::ActiveModel = row.into();
    a.label = ActiveValue::set(label);
    if let Some(labels) = labels {
        a.labels = ActiveValue::set(serde_json::json!(labels));
    }
    a.description = ActiveValue::set(description);
    if let Some(permissions) = permissions {
        a.permissions = ActiveValue::set(serde_json::json!(permissions));
    }
    a.position = ActiveValue::set(position);
    a.retired_at = ActiveValue::set(retired_at);
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "organization_role_changed",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": role_view(&row) })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(role_view(&row))
}

/// The live plan a path names; `404` otherwise.
async fn require_plan(ctx: &AppContext, plan: &str) -> Result<Uuid> {
    let plan = super::parse_pid(plan)?;
    plans::Entity::find()
        .filter(plans::Column::Pid.eq(plan))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    Ok(plan)
}

/// `GET /api/plans/{pid}/issue-types` — the types this plan has limited itself to. Empty means every type.
#[debug_handler]
async fn plan_types(State(ctx): State<AppContext>, Path(plan): Path<String>) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let types = chosen_types(&ctx, plan).await?;
    format::json(
        serde_json::json!({ "plan_pid": plan, "limited": !types.is_empty(), "types": types }),
    )
}

/// `PUT /api/plans/{pid}/issue-types` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PlanTypes {
    /// The type keys this plan offers, none of them retired. Empty offers every type.
    types: Vec<String>,
}

/// `PUT /api/plans/{pid}/issue-types` — choose the types a plan offers (an administrator). Issues that already carry a type
/// left out keep it.
#[debug_handler]
async fn set_plan_types(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(plan): Path<String>,
    Json(b): Json<PlanTypes>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    require_admin(&caller)?;
    let mut wanted = b.types;
    wanted.sort();
    wanted.dedup();
    for key in &wanted {
        require_usable_type(&ctx, key).await?;
    }
    let was = chosen_types(&ctx, plan).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    plan_issue_types::Entity::delete_many()
        .filter(plan_issue_types::Column::PlanPid.eq(plan))
        .exec(&txn)
        .await
        .map_err(db_err)?;
    for key in &wanted {
        plan_issue_types::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan),
            type_key: ActiveValue::set(key.clone()),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?;
    }
    AuditModel::record(
        &txn,
        plan,
        "plan_issue_types_changed",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": wanted })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(
        serde_json::json!({ "plan_pid": plan, "limited": !wanted.is_empty(), "types": wanted }),
    )
}

/// `GET /api/plans/{pid}/issues/options` — what the plan's issue form offers: every type with the workflow in force for
/// it, and the caller's standing on the plan, so a form can leave out what the service would refuse. Types a person cannot choose
/// are included, marked, so an issue that carries one can still show its name.
#[debug_handler]
async fn plan_options(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(plan): Path<String>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let chosen = chosen_types(&ctx, plan).await?;
    let types = issue_types::Entity::find()
        .order_by_asc(issue_types::Column::Position)
        .order_by_asc(issue_types::Column::Key)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut out = Vec::with_capacity(types.len());
    for t in &types {
        let workflow = super::workflow::in_force_issue(&ctx, plan, &t.key).await?;
        let mut item = type_view(t);
        item["workflow"] = serde_json::json!(workflow);
        item["offered"] = serde_json::json!(
            t.retired_at.is_none() && (chosen.is_empty() || chosen.contains(&t.key))
        );
        out.push(item);
    }
    let standing = super::plan_roles::standing(&ctx, &caller, plan).await?;
    let allowed: Vec<&str> = crate::plan_roles::PERMISSIONS
        .iter()
        .copied()
        .filter(|p| standing.may(p))
        .collect();
    format::json(serde_json::json!({
        "types": out, "caller_roles": standing.roles,
        "enforced": standing.enforced, "caller_permissions": allowed, "caller_admin": standing.admin,
    }))
}

/// The issue configuration routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/issue-types", get(list_types))
        .add("/issue-types", post(create_type))
        .add("/issue-types/{key}", put(change_type))
        .add("/organization-roles", get(list_roles))
        .add("/organization-roles", post(create_role))
        .add("/organization-roles/{key}", put(change_role))
        .add("/plans/{pid}/issues/options", get(plan_options))
        .add("/plans/{pid}/issue-types", get(plan_types))
        .add("/plans/{pid}/issue-types", put(set_plan_types))
}

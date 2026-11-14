//! Organizations (EX-3): `POST`/`GET /api/organizations` and `PUT /api/organizations/{pid}`.
//!
//! An organization is a record: the delivery organization, a partner body, a government sponsor or a
//! supplier. A capacity commitment may name one, an organization can be the audience of a director pack
//! (`org:<pid>`), and a caller whose token carries the `organization` attribute is that organization:
//! the attribute is for policy to use, and the service itself uses it in two places (a partner's list of
//! commitments, and the packs addressed to its organization).

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::organizations as orgs;
use crate::models::audit_logs::Model as AuditModel;

/// The kinds of organization.
pub const KINDS: [&str; 4] = ["delivery", "partner", "sponsor", "supplier"];

/// Map a database error to an API error; a duplicate organization name becomes a 409 conflict.
fn db_err(e: &sqlx::Error) -> Error {
    if e.to_string().contains("org_name_once") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "an organization with that name exists"),
        );
    }
    if e.to_string().contains("org_code_once") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "an organization with that code exists"),
        );
    }
    if e.to_string().contains("org_one_national") {
        return unprocessable("another organization is already national");
    }
    Error::string(&e.to_string())
}

/// JSON view of an organization; the audience is `org:<pid>`, the audience key packs use.
fn show(o: &orgs::Model) -> serde_json::Value {
    json!({ "pid": o.pid, "name": o.name, "kind": o.kind, "active": o.active, "audience": format!("org:{}", o.pid),
            "code": o.code, "national": o.national })
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
    let rows: Vec<orgs::Model> =
        sqlx::query_as("SELECT * FROM organizations ORDER BY national DESC, name")
            .fetch_all(crate::db::pool(&ctx))
            .await
            .map_err(|e| db_err(&e))?;
    format::json(json!({ "organizations": rows.iter().map(show).collect::<Vec<_>>() }))
}

/// Request body creating an organization.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewOrg {
    /// Organization name, 1 to 200 characters after trimming, unique.
    name: String,
    /// Organization kind; one of KINDS.
    kind: String,
    /// A short code (RE-7): a letter or digit, then up to 19 letters, digits or hyphens; unique ignoring case.
    #[serde(default)]
    code: Option<String>,
    /// Whether its scope is national (RE-7); one organization at most.
    #[serde(default)]
    national: bool,
}

/// Whether `code` is a short organization code: a letter or digit, then up to 19 letters, digits or hyphens.
#[must_use]
pub fn is_code(code: &str) -> bool {
    let mut chars = code.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphanumeric())
        && code.len() <= 20
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Check name length, that the kind is in KINDS and the code's shape, collecting every problem into one 422.
fn validate(name: &str, kind: &str, code: Option<&str>) -> Result<()> {
    let mut problems = Vec::new();
    if code.is_some_and(|c| !is_code(c)) {
        problems.push(
            "code is a letter or digit, then up to 19 letters, digits or hyphens".to_string(),
        );
    }
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
    let code = b.code.as_deref().map(str::trim).filter(|c| !c.is_empty());
    validate(&b.name, &b.kind, code)?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: orgs::Model = sqlx::query_as(
        "INSERT INTO organizations (pid, name, kind, active, code, national)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.name.trim())
    .bind(&b.kind)
    .bind(true)
    .bind(code)
    .bind(b.national)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "organization_created",
        caller.actor(),
        Some(json!({ "kind": row.kind })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(show(&row))
}

/// Request body changing an organization; absent fields are left as they are.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Change {
    /// New name.
    #[serde(default)]
    name: Option<String>,
    /// New kind; one of KINDS.
    #[serde(default)]
    kind: Option<String>,
    /// False retires the organization, true restores it.
    #[serde(default)]
    active: Option<bool>,
    /// A new short code; an empty string clears it (RE-7).
    #[serde(default)]
    code: Option<String>,
    /// Whether its scope is national (RE-7); one organization at most.
    #[serde(default)]
    national: Option<bool>,
}

/// `PUT /api/organizations/{pid}` — rename, change kind, retire or restore.
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<Change>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let row: orgs::Model = sqlx::query_as("SELECT * FROM organizations WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    // `Some("")` clears the code; `None` leaves it.
    let code: Option<Option<String>> = b.code.as_deref().map(|c| {
        let c = c.trim();
        (!c.is_empty()).then(|| c.to_string())
    });
    validate(
        b.name.as_deref().unwrap_or(&row.name),
        b.kind.as_deref().unwrap_or(&row.kind),
        code.as_ref().map_or(row.code.as_deref(), Option::as_deref),
    )?;
    // Only the fields given are written; with none, the row is read back unchanged (as the empty update did before
    // SQ-1).
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE organizations SET ");
    let mut set = q.separated(", ");
    let mut any = false;
    if let Some(n) = b.name {
        set.push("name = ")
            .push_bind_unseparated(n.trim().to_string());
        any = true;
    }
    if let Some(k) = b.kind {
        set.push("kind = ").push_bind_unseparated(k);
        any = true;
    }
    if let Some(v) = b.active {
        set.push("active = ").push_bind_unseparated(v);
        any = true;
    }
    if let Some(c) = code {
        set.push("code = ").push_bind_unseparated(c);
        any = true;
    }
    if let Some(v) = b.national {
        set.push("national = ").push_bind_unseparated(v);
        any = true;
    }
    q.push(" WHERE id = ")
        .push_bind(row.id)
        .push(" RETURNING *");
    let mut tx = crate::db::begin(&ctx).await?;
    let row: orgs::Model = if any {
        q.build_query_as()
            .fetch_one(tx.conn())
            .await
            .map_err(|e| db_err(&e))?
    } else {
        sqlx::query_as("SELECT * FROM organizations WHERE id = $1 LIMIT 1")
            .bind(row.id)
            .fetch_one(tx.conn())
            .await
            .map_err(|e| db_err(&e))?
    };
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "organization_changed",
        caller.actor(),
        Some(json!({ "active": row.active })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    let Some(pid) = audience
        .strip_prefix("org:")
        .and_then(|p| Uuid::parse_str(p).ok())
    else {
        return Ok(false);
    };
    Ok(
        sqlx::query("SELECT 1 FROM organizations WHERE pid = $1 AND active = true LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await
            .map_err(|e| db_err(&e))?
            .is_some(),
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_code_is_short_and_plain() {
        for good in ["SBUHB", "National", "7A1", "cardiff-vale"] {
            assert!(is_code(good), "{good}");
        }
        for bad in ["", "-SBUHB", "S B", "SBUHB!", &"A".repeat(21)] {
            assert!(!is_code(bad), "{bad}");
        }
    }
}

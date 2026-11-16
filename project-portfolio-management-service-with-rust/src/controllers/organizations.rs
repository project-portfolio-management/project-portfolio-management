//! Organizations (EX-3): `POST`/`GET /api/organizations` and `PUT /api/organizations/{pid}`.
//!
//! An organization is a record: the delivery organization, a partner body, a government sponsor or a
//! supplier. A capacity commitment may name one, an organization can be the audience of a director pack
//! (`org:<pid>`), and a caller whose token carries the `organization` attribute is that organization:
//! the attribute is for policy to use, and the service itself uses it in two places (a partner's list of
//! commitments, and the packs addressed to its organization).
//!
//! An organization may have a short name, and names in other languages (`translations`, by language code, each with
//! a name and an optional short name), so a reader in Welsh or Mandarin sees the name it goes by there (RE-7).

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::organizations as orgs;
use crate::models::audit_logs::Model as AuditModel;

/// Longest name, in characters, in any language.
pub const NAME_MAX: usize = 200;
/// Longest short name, in characters, in any language.
pub const SHORT_NAME_MAX: usize = 50;
/// Most languages an organization's names are given in.
pub const LANGUAGES_MAX: usize = 50;

/// An organization's names in one language.
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Names {
    /// Its name in this language, 1 to 200 characters after trimming.
    name: String,
    /// Its short name in this language, at most 50 characters after trimming; absent, `null` or empty for none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
}

/// What is wrong with an organization's names in other languages, in plain words; empty means they can be saved.
#[must_use]
pub(crate) fn translation_problems(translations: &BTreeMap<String, Names>) -> Vec<String> {
    let mut out = Vec::new();
    if translations.len() > LANGUAGES_MAX {
        out.push(format!("names in at most {LANGUAGES_MAX} languages"));
    }
    for (lang, names) in translations {
        if !crate::issues::is_language_code(lang) {
            out.push(format!("`{lang}` is not a language code"));
        }
        if !(1..=NAME_MAX).contains(&names.name.trim().chars().count()) {
            out.push(format!(
                "the `{lang}` name must be 1 to {NAME_MAX} characters"
            ));
        }
        if names
            .short_name
            .as_deref()
            .is_some_and(|s| s.trim().chars().count() > SHORT_NAME_MAX)
        {
            out.push(format!(
                "the `{lang}` short name is at most {SHORT_NAME_MAX} characters"
            ));
        }
    }
    out
}

/// The names as stored: trimmed, with an empty short name left out.
fn stored(translations: &BTreeMap<String, Names>) -> serde_json::Value {
    let trimmed: BTreeMap<&String, Names> = translations
        .iter()
        .map(|(lang, n)| {
            (
                lang,
                Names {
                    name: n.name.trim().to_string(),
                    short_name: short(n.short_name.as_deref()).map(str::to_string),
                },
            )
        })
        .collect();
    json!(trimmed)
}

/// A short name as given: trimmed, and `None` when empty.
fn short(s: Option<&str>) -> Option<&str> {
    s.map(str::trim).filter(|s| !s.is_empty())
}

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
            "code": o.code, "national": o.national, "short_name": o.short_name, "translations": o.translations })
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
    /// A short name (RE-7), at most 50 characters after trimming, not unique; absent or empty for none.
    #[serde(default)]
    short_name: Option<String>,
    /// Its names in other languages (RE-7), by language code, such as `cy` or `zh-hans`.
    #[serde(default)]
    translations: Option<BTreeMap<String, Names>>,
}

/// Whether `code` is a short organization code: a letter or digit, then up to 19 letters, digits or hyphens.
#[must_use]
pub fn is_code(code: &str) -> bool {
    let mut chars = code.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphanumeric())
        && code.len() <= 20
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Check name length, that the kind is in KINDS, the code's shape, the short name's length and the names in other
/// languages, collecting every problem into one 422.
fn validate(
    name: &str,
    kind: &str,
    code: Option<&str>,
    short_name: Option<&str>,
    translations: Option<&BTreeMap<String, Names>>,
) -> Result<()> {
    let mut problems = Vec::new();
    if code.is_some_and(|c| !is_code(c)) {
        problems.push(
            "code is a letter or digit, then up to 19 letters, digits or hyphens".to_string(),
        );
    }
    if !(1..=NAME_MAX).contains(&name.trim().chars().count()) {
        problems.push(format!("name must be 1 to {NAME_MAX} characters"));
    }
    if short_name.is_some_and(|s| s.chars().count() > SHORT_NAME_MAX) {
        problems.push(format!("short name is at most {SHORT_NAME_MAX} characters"));
    }
    if let Some(t) = translations {
        problems.extend(translation_problems(t));
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
    let short_name = short(b.short_name.as_deref());
    validate(&b.name, &b.kind, code, short_name, b.translations.as_ref())?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: orgs::Model = sqlx::query_as(
        "INSERT INTO organizations (pid, name, kind, active, code, national, short_name, translations)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.name.trim())
    .bind(&b.kind)
    .bind(true)
    .bind(code)
    .bind(b.national)
    .bind(short_name)
    .bind(b.translations.as_ref().map_or_else(|| json!({}), stored))
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
    /// A new short name (RE-7); an empty string clears it.
    #[serde(default)]
    short_name: Option<String>,
    /// Its names in other languages (RE-7), replacing those it has; `{}` removes them all.
    #[serde(default)]
    translations: Option<BTreeMap<String, Names>>,
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
    // As for the code, `Some("")` clears the short name and `None` leaves it.
    let short_name: Option<Option<String>> = b
        .short_name
        .as_deref()
        .map(|s| short(Some(s)).map(str::to_string));
    validate(
        b.name.as_deref().unwrap_or(&row.name),
        b.kind.as_deref().unwrap_or(&row.kind),
        code.as_ref().map_or(row.code.as_deref(), Option::as_deref),
        short_name
            .as_ref()
            .map_or(row.short_name.as_deref(), Option::as_deref),
        b.translations.as_ref(),
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
    if let Some(s) = short_name {
        set.push("short_name = ").push_bind_unseparated(s);
        any = true;
    }
    if let Some(t) = &b.translations {
        set.push("translations = ").push_bind_unseparated(stored(t));
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

    fn names(name: &str, short_name: Option<&str>) -> Names {
        Names {
            name: name.to_string(),
            short_name: short_name.map(str::to_string),
        }
    }

    #[test]
    fn names_in_other_languages_are_checked_and_stored_trimmed() {
        let good: BTreeMap<String, Names> = [
            (
                "cy".to_string(),
                names(
                    " Bwrdd Iechyd Prifysgol Bae Abertawe ",
                    Some(" Bae Abertawe "),
                ),
            ),
            ("zh-hans".to_string(), names("斯旺西湾大学卫生局", None)),
            (
                "en".to_string(),
                names("Swansea Bay University Health Board", Some("")),
            ),
        ]
        .into();
        assert!(translation_problems(&good).is_empty());
        assert_eq!(
            stored(&good),
            json!({
                "cy": { "name": "Bwrdd Iechyd Prifysgol Bae Abertawe", "short_name": "Bae Abertawe" },
                "en": { "name": "Swansea Bay University Health Board" },
                "zh-hans": { "name": "斯旺西湾大学卫生局" },
            })
        );
        let bad: BTreeMap<String, Names> = [
            ("Welsh".to_string(), names("Bwrdd", None)),
            ("cy".to_string(), names("  ", None)),
            (
                "zh".to_string(),
                names("卫生局", Some(&"长".repeat(SHORT_NAME_MAX + 1))),
            ),
        ]
        .into();
        let problems = translation_problems(&bad);
        assert_eq!(problems.len(), 3, "{problems:?}");
        // A name of 200 characters in Chinese is 200 characters, not 600 bytes.
        let long: BTreeMap<String, Names> =
            [("zh".to_string(), names(&"局".repeat(NAME_MAX), None))].into();
        assert!(translation_problems(&long).is_empty());
    }

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

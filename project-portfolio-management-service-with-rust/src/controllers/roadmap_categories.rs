//! Roadmap categories (RE-7): `GET`/`POST /api/roadmap-categories` and `PUT /api/roadmap-categories/{pid}`.
//!
//! A category is defined once and a public product roadmap links to it by id, so renaming a category renames it
//! wherever it is used; nothing holds a copy that could drift. Its name is in the deployment's own language, with
//! translations as an object of language code to text (the same rule as an issue type's labels). A retired category
//! can no longer be chosen; a roadmap that already names it keeps it. The list holds no person and reads no plan.

use std::collections::BTreeMap;

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::roadmap_categories as cats;
use crate::models::audit_logs::Model as AuditModel;

/// Longest name, in characters.
pub const NAME_MAX: usize = 100;

/// Map a database error; a duplicate name becomes a 409.
fn conflict_or(e: &sqlx::Error) -> Error {
    if e.to_string().contains("roadmap_category_name_once") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "a category with that name exists"),
        );
    }
    Error::string(&e.to_string())
}

/// JSON view of a category.
fn show(c: &cats::Model) -> serde_json::Value {
    json!({ "pid": c.pid, "name": c.name, "translations": c.translations, "active": c.active })
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/roadmap-categories", get(list))
        .add("/roadmap-categories", post(create))
        .add("/roadmap-categories/{pid}", put(update))
}

/// What is wrong with a name and its translations, in plain words; empty means it can be saved.
#[must_use]
pub fn problems(name: &str, translations: &BTreeMap<String, String>) -> Vec<String> {
    let mut out = Vec::new();
    if !(1..=NAME_MAX).contains(&name.trim().chars().count()) {
        out.push(format!("name must be 1 to {NAME_MAX} characters"));
    }
    out.extend(crate::issues::label_problems(translations));
    out
}

/// Trimmed translations, as stored.
fn stored(translations: &BTreeMap<String, String>) -> serde_json::Value {
    json!(
        translations
            .iter()
            .map(|(k, v)| (k.clone(), v.trim().to_string()))
            .collect::<BTreeMap<_, _>>()
    )
}

/// `GET /api/roadmap-categories` — every category, active first, then by name.
#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let rows: Vec<cats::Model> =
        sqlx::query_as("SELECT * FROM roadmap_categories ORDER BY active DESC, lower(name)")
            .fetch_all(crate::db::pool(&ctx))
            .await
            .map_err(|e| conflict_or(&e))?;
    format::json(json!({ "categories": rows.iter().map(show).collect::<Vec<_>>() }))
}

/// Request body creating a category.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NewCategory {
    /// The name, 1 to 100 characters after trimming, unique ignoring case.
    name: String,
    /// The name in other languages: language code (`cy`, `zh-cn`) to text, 1 to 100 characters each.
    #[serde(default)]
    translations: BTreeMap<String, String>,
}

/// `POST /api/roadmap-categories`
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<NewCategory>,
) -> Result<Response> {
    let found = problems(&b.name, &b.translations);
    if !found.is_empty() {
        return Err(unprocessable(&found.join("; ")));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: cats::Model = sqlx::query_as(
        "INSERT INTO roadmap_categories (pid, name, translations) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.name.trim())
    .bind(stored(&b.translations))
    .fetch_one(tx.conn())
    .await
    .map_err(|e| conflict_or(&e))?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "roadmap_category_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(show(&row))
}

/// Request body changing a category; absent fields are left as they are.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CategoryChange {
    /// A new name.
    #[serde(default)]
    name: Option<String>,
    /// The whole set of translations, replacing the old one.
    #[serde(default)]
    translations: Option<BTreeMap<String, String>>,
    /// False retires the category, true restores it.
    #[serde(default)]
    active: Option<bool>,
}

/// `PUT /api/roadmap-categories/{pid}` — rename, translate, retire or restore.
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<CategoryChange>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: cats::Model =
        sqlx::query_as("SELECT * FROM roadmap_categories WHERE pid = $1 FOR UPDATE")
            .bind(pid)
            .fetch_optional(tx.conn())
            .await?
            .ok_or(Error::NotFound)?;
    let old: BTreeMap<String, String> =
        serde_json::from_value(row.translations.clone()).unwrap_or_default();
    let name = b.name.as_deref().unwrap_or(&row.name);
    let translations = b.translations.as_ref().unwrap_or(&old);
    let found = problems(name, translations);
    if !found.is_empty() {
        return Err(unprocessable(&found.join("; ")));
    }
    let row: cats::Model = sqlx::query_as(
        "UPDATE roadmap_categories SET name = $2, translations = $3, active = $4, updated_at = now()
         WHERE id = $1 RETURNING *",
    )
    .bind(row.id)
    .bind(name.trim())
    .bind(stored(translations))
    .bind(b.active.unwrap_or(row.active))
    .fetch_one(tx.conn())
    .await
    .map_err(|e| conflict_or(&e))?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "roadmap_category_changed",
        caller.actor(),
        Some(json!({ "active": row.active })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(show(&row))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_is_required_and_translations_follow_the_label_rule() {
        let none = BTreeMap::new();
        assert!(problems("Patients and public", &none).is_empty());
        assert_eq!(problems("   ", &none).len(), 1);
        assert_eq!(problems(&"x".repeat(101), &none).len(), 1);
        let welsh = BTreeMap::from([("cy".to_string(), "Cleifion a'r cyhoedd".to_string())]);
        assert!(problems("Patients and public", &welsh).is_empty());
        let bad = BTreeMap::from([("Welsh".to_string(), "Cleifion".to_string())]);
        assert_eq!(problems("Patients", &bad).len(), 1);
        let blank = BTreeMap::from([("cy".to_string(), " ".to_string())]);
        assert_eq!(problems("Patients", &blank).len(), 1);
    }
}

//! Per-user saved views (T-28l): route-scoped filter/sort/column
//! presets, keyed by the caller's token `sub` and nothing else
//! identity-shaped. Served through the BFF, so the browser never
//! holds the list itself — the front-end server calls this API with
//! the caller's own bearer token, exactly as any other authenticated
//! call. Pure validation lives in [`crate::saved_views`]; a view is
//! always scoped to the `route` it was saved from and to the caller
//! who saved it — never another user's, never another route's.

use super::unprocessable;
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::models::_entities::saved_views;
use crate::saved_views as rules;

/// `POST /api/saved-views` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SavedViewPayload {
    /// Front-end route the view belongs to; required, capped at `MAX_ROUTE_LEN`.
    route: String,
    /// Human-facing view name; required, capped at `MAX_NAME_LEN`.
    name: String,
    /// Opaque filter state the front end restores; size-capped so this endpoint cannot store arbitrary documents. Defaults to null.
    #[serde(default)]
    filter: serde_json::Value,
    /// Opaque sort state, size-capped like `filter`. Defaults to null.
    #[serde(default)]
    sort: serde_json::Value,
    /// Visible column keys in display order; each must be non-blank and the list length is capped.
    #[serde(default)]
    columns: Vec<String>,
}

/// `POST /api/saved-views` — save a view under the caller's own
/// `sub`. Requires authentication ([`AuthUser`]): a saved view with no
/// owner is not a saved view.
#[debug_handler]
async fn create_saved_view(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<SavedViewPayload>,
) -> Result<Response> {
    let mut problems = rules::problems(&payload.route, &payload.name, &payload.columns);
    if rules::json_blob_too_large(&payload.filter) {
        problems.push("filter: too large".to_string());
    }
    if rules::json_blob_too_large(&payload.sort) {
        problems.push("sort: too large".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }

    let row: saved_views::Model = sqlx::query_as(
        "INSERT INTO saved_views (pid, sub, route, name, filter, sort, columns)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&claims.sub)
    .bind(&payload.route)
    .bind(&payload.name)
    .bind(&payload.filter)
    .bind(&payload.sort)
    .bind(serde_json::json!(payload.columns))
    .fetch_one(crate::db::pool(&ctx))
    .await?;
    format::json(row)
}

/// `GET /api/saved-views?route=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// Narrow to one route; omitted ⇒ every one of the caller's own
    /// views, across routes (a "manage my views" listing).
    #[serde(default)]
    route: Option<String>,
}

/// `GET /api/saved-views?route=` — the caller's own views, optionally
/// narrowed to one route. Never returns another `sub`'s rows.
#[debug_handler]
async fn list_saved_views(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(params): Query<ListParams>,
) -> Result<Response> {
    let rows: Vec<saved_views::Model> = sqlx::query_as(
        "SELECT * FROM saved_views
         WHERE sub = $1 AND deleted_at IS NULL AND ($2::text IS NULL OR route = $2)
         ORDER BY id",
    )
    .bind(&claims.sub)
    .bind(&params.route)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(rows)
}

/// Find one of the **caller's own** live saved views by pid. Another
/// user's view — or an unknown pid — is `404`, identically: a saved
/// view's existence is not disclosed across users.
async fn find_own(ctx: &AppContext, sub: &str, pid: &str) -> Result<saved_views::Model> {
    let pid = super::parse_pid(pid)?;
    sqlx::query_as("SELECT * FROM saved_views WHERE pid = $1 AND sub = $2 AND deleted_at IS NULL")
        .bind(pid)
        .bind(sub)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// `DELETE /api/saved-views/{pid}` — soft-delete, scoped to the
/// caller's own `sub`.
#[debug_handler]
async fn delete_saved_view(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_own(&ctx, &claims.sub, &pid).await?;
    sqlx::query("UPDATE saved_views SET deleted_at = now() WHERE id = $1")
        .bind(row.id)
        .execute(crate::db::pool(&ctx))
        .await?;
    format::empty_json()
}

/// The saved-views routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/saved-views", post(create_saved_view))
        .add("/saved-views", get(list_saved_views))
        .add("/saved-views/{pid}", delete(delete_saved_view))
        // POST twin: DELETE needs the admin attribute under the default
        // policy, and a person must always be able to remove their own view.
        .add("/saved-views/{pid}/delete", post(delete_saved_view))
}

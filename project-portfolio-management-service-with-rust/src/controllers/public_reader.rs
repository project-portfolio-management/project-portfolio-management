//! The public reader of public product roadmaps (RE-4): `GET /api/public/roadmaps` and
//! `GET /api/public/roadmaps/{slug}`, with no token.
//!
//! **Off unless the deployment turns it on** (`PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS=on`); off, both routes
//! answer `404`. On, they serve only what a person chose to publish, and only from a plan that is
//!
//! * live (not deleted),
//! * not private (no plan access setting with `private`), and
//! * at the lowest level of the handling-mark scheme (no mark, or the lowest), as fixed rules no caller can widen.
//!
//! A roadmap that is unknown, unpublished, private, marked or of a deleted plan answers the same `404`. Unplaced
//! and removed items are never served.
//!
//! **A whitelist of fields.** [`ROADMAP_FIELDS`] and [`ITEM_FIELDS`] say, for every column of the two tables,
//! whether the reader serves it and why; a unit test fails when a column is added to either table without a
//! decision. The reader serves no person, no internal id, no plan id and no audit data. An organization appears as
//! its code, name and national flag; a category as its name and translations.
//!
//! **Translations** are served with `needs_review`. With
//! `PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS_REVIEWED_ONLY=on`, a translation that needs review is left out, so
//! a reader falls back to the source text (RE-6).
//!
//! Responses carry a weak `ETag` (an `If-None-Match` that matches is `304`) and `Cache-Control: public,
//! max-age=60`, so an unpublish shows within a minute in a cache and at once without one. With
//! `PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS_NOINDEX=on` they also carry `X-Robots-Tag: noindex`. The path is
//! outside the login guard (`auth::enforcement::is_public_path`) and inside the rate limiter, like every request.

use std::collections::{BTreeMap, HashMap};

use crate::prelude::*;
use axum::http::{HeaderMap, HeaderValue, header};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::models::_entities::public_roadmap_items as items;
use crate::models::_entities::public_roadmaps as roadmaps;
use crate::public_roadmap as rules;

/// The setting that turns the reader on.
pub const ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS";
/// The setting that leaves out translations needing review.
pub const REVIEWED_ONLY_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS_REVIEWED_ONLY";
/// The setting that adds `X-Robots-Tag: noindex`.
pub const NOINDEX_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS_NOINDEX";

/// Whether a reader may see a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    /// Served as it is.
    Public,
    /// Served in another form (named in the reason).
    Derived,
    /// Never served.
    Internal,
}

/// Every column of `public_roadmaps`, and whether the reader serves it.
pub const ROADMAP_FIELDS: &[(&str, Exposure, &str)] = &[
    ("created_at", Exposure::Internal, "row bookkeeping"),
    (
        "updated_at",
        Exposure::Internal,
        "row bookkeeping; reviewed_on is the date a reader is given",
    ),
    ("id", Exposure::Internal, "internal key"),
    (
        "pid",
        Exposure::Internal,
        "internal id; the slug is the public address",
    ),
    ("plan_pid", Exposure::Internal, "the plan is not public"),
    ("slug", Exposure::Public, "the public address"),
    ("title", Exposure::Public, "published text"),
    ("vision", Exposure::Public, "published text"),
    ("service_description", Exposure::Public, "published text"),
    (
        "category_pid",
        Exposure::Derived,
        "served as the category's name and translations",
    ),
    (
        "published",
        Exposure::Internal,
        "only published roadmaps are served, so it is always true",
    ),
    (
        "reviewed_on",
        Exposure::Public,
        "a reader is told how current it is",
    ),
    (
        "translations",
        Exposure::Derived,
        "served as value and needs_review, without the source hash",
    ),
];

/// Every column of `public_roadmap_items`, and whether the reader serves it.
pub const ITEM_FIELDS: &[(&str, Exposure, &str)] = &[
    ("created_at", Exposure::Internal, "row bookkeeping"),
    ("updated_at", Exposure::Internal, "row bookkeeping"),
    ("id", Exposure::Internal, "internal key"),
    (
        "pid",
        Exposure::Internal,
        "internal id; public_id is the public one",
    ),
    ("roadmap_id", Exposure::Internal, "internal key"),
    ("public_id", Exposure::Public, "the item's public id"),
    ("title", Exposure::Public, "published text"),
    ("description", Exposure::Public, "published text"),
    ("expected_outcome", Exposure::Public, "published text"),
    ("phase", Exposure::Public, "published text"),
    ("tags", Exposure::Public, "published text"),
    (
        "organization_pids",
        Exposure::Derived,
        "served as each active organization's code, name and national flag",
    ),
    ("details", Exposure::Public, "published text"),
    (
        "placement",
        Exposure::Public,
        "where it shows; unplaced items are not served",
    ),
    (
        "position",
        Exposure::Derived,
        "served as the order of the list",
    ),
    (
        "translations",
        Exposure::Derived,
        "served as value and needs_review, without the source hash",
    ),
    (
        "deleted_at",
        Exposure::Internal,
        "removed items are not served",
    ),
];

fn on(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| matches!(v.trim(), "on" | "true" | "1"))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/public")
        .add("/roadmaps", get(list))
        .add("/roadmaps/{slug}", get(show))
}

/// The roadmaps the reader may serve: published, of a live plan that is not private, at the lowest mark.
async fn servable(ctx: &AppContext, slug: Option<&str>) -> Result<Vec<roadmaps::Model>> {
    let rows: Vec<roadmaps::Model> = sqlx::query_as(
        "SELECT r.* FROM public_roadmaps r
         JOIN plans p ON p.pid = r.plan_pid AND p.active
         WHERE r.published
           AND NOT EXISTS (SELECT 1 FROM plan_access_settings a WHERE a.plan_pid = r.plan_pid AND a.private)
           AND ($1::varchar IS NULL OR r.slug = $1)
         ORDER BY lower(r.title), r.id",
    )
    .bind(slug)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let pids: Vec<Uuid> = rows.iter().map(|r| r.plan_pid).collect();
    let marks: HashMap<Uuid, String> = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT plan_pid, mark FROM plan_handling_marks WHERE plan_pid = ANY($1)",
    )
    .bind(&pids)
    .fetch_all(crate::db::pool(ctx))
    .await?
    .into_iter()
    .collect();
    let scheme = crate::handling_marks::scheme();
    Ok(rows
        .into_iter()
        .filter(|r| scheme.plan_level(marks.get(&r.plan_pid).map(String::as_str)) == 0)
        .collect())
}

/// Translations as a reader sees them; with reviewed-only on, those needing review are left out.
fn public_translations(
    stored: &Value,
    sources: &BTreeMap<&str, Value>,
    reviewed_only: bool,
) -> Value {
    let mut shown = rules::shown_translations(stored, sources);
    if reviewed_only && let Some(langs) = shown.as_object_mut() {
        for fields in langs.values_mut() {
            if let Some(f) = fields.as_object_mut() {
                f.retain(|_, t| t["needs_review"] != true);
            }
        }
        langs.retain(|_, f| f.as_object().is_some_and(|f| !f.is_empty()));
    }
    shown
}

/// A category as a reader sees it.
async fn category(ctx: &AppContext, pid: Option<Uuid>) -> Result<Value> {
    let Some(pid) = pid else {
        return Ok(Value::Null);
    };
    let row: Option<(String, Value)> =
        sqlx::query_as("SELECT name, translations FROM roadmap_categories WHERE pid = $1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?;
    Ok(row.map_or(
        Value::Null,
        |(name, translations)| json!({ "name": name, "translations": translations }),
    ))
}

/// Serve `body` with an `ETag`, caching and, if asked, `noindex`.
fn respond(headers: &HeaderMap, body: &Value) -> Result<Response> {
    let etag = super::etag_of(body);
    let mut resp = super::conditional_json(headers, &etag, body)?;
    resp.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=60"),
    );
    if on(NOINDEX_ENV) {
        resp.headers_mut()
            .insert("x-robots-tag", HeaderValue::from_static("noindex"));
    }
    Ok(resp)
}

/// `GET /api/public/roadmaps` — the published roadmaps, by title.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, headers: HeaderMap) -> Result<Response> {
    if !on(ENV) {
        return Err(Error::NotFound);
    }
    let today = crate::tz::deployment_today();
    let days = rules::stale_days();
    let mut out = Vec::new();
    for r in servable(&ctx, None).await? {
        out.push(json!({
            "slug": r.slug, "title": r.title, "category": category(&ctx, r.category_pid).await?,
            "reviewed_on": r.reviewed_on, "freshness": rules::freshness(r.reviewed_on, today, days),
        }));
    }
    respond(&headers, &json!({ "roadmaps": out }))
}

/// `GET /api/public/roadmaps/{slug}` — one published roadmap and its placed items, in page order.
#[debug_handler]
async fn show(
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response> {
    if !on(ENV) || !rules::is_slug(&slug) {
        return Err(Error::NotFound);
    }
    let r = servable(&ctx, Some(&slug))
        .await?
        .into_iter()
        .next()
        .ok_or(Error::NotFound)?;
    let reviewed_only = on(REVIEWED_ONLY_ENV);
    let mut list: Vec<items::Model> = sqlx::query_as(
        "SELECT * FROM public_roadmap_items
         WHERE roadmap_id = $1 AND deleted_at IS NULL AND placement IS NOT NULL",
    )
    .bind(r.id)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    list.sort_by_key(|i| {
        (
            rules::placement_rank(i.placement.as_deref()),
            i.position,
            i.id,
        )
    });
    let org_pids: Vec<Uuid> = list
        .iter()
        .flat_map(|i| i.organization_pids.clone())
        .collect();
    let orgs: HashMap<Uuid, Value> = sqlx::query_as::<_, (Uuid, Option<String>, String, bool)>(
        "SELECT pid, code, name, national FROM organizations WHERE pid = ANY($1) AND active",
    )
    .bind(&org_pids)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .into_iter()
    .map(|(pid, code, name, national)| {
        (
            pid,
            json!({ "code": code, "name": name, "national": national }),
        )
    })
    .collect();
    let today = crate::tz::deployment_today();
    let days = rules::stale_days();
    let body = json!({
        "slug": r.slug, "title": r.title, "vision": r.vision, "service_description": r.service_description,
        "category": category(&ctx, r.category_pid).await?,
        "reviewed_on": r.reviewed_on, "freshness": rules::freshness(r.reviewed_on, today, days),
        "translations": public_translations(&r.translations, &super::public_roadmaps::roadmap_sources(&r), reviewed_only),
        "items": list.iter().map(|i| json!({
            "public_id": i.public_id, "title": i.title, "description": i.description,
            "expected_outcome": i.expected_outcome, "phase": i.phase, "tags": i.tags, "details": i.details,
            "placement": i.placement,
            "organizations": i.organization_pids.iter().filter_map(|o| orgs.get(o).cloned()).collect::<Vec<_>>(),
            "translations": public_translations(&i.translations, &super::public_roadmaps::item_sources(i), reviewed_only),
        })).collect::<Vec<_>>(),
    });
    respond(&headers, &body)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `pub` fields of a row struct, read from its source file.
    fn fields_of(file: &str) -> Vec<String> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file);
        let text = std::fs::read_to_string(path).expect("read entity");
        text.lines()
            .filter_map(|l| l.trim().strip_prefix("pub "))
            .filter(|l| !l.starts_with("struct") && !l.starts_with("use"))
            .filter_map(|l| l.split(':').next())
            .map(|n| n.trim().to_string())
            .collect()
    }

    #[test]
    fn every_column_is_classified_for_the_public_reader() {
        for (file, listed) in [
            ("src/models/_entities/public_roadmaps.rs", ROADMAP_FIELDS),
            ("src/models/_entities/public_roadmap_items.rs", ITEM_FIELDS),
        ] {
            let mut columns = fields_of(file);
            let mut names: Vec<String> = listed.iter().map(|(n, _, _)| (*n).to_string()).collect();
            columns.sort();
            names.sort();
            assert_eq!(
                columns, names,
                "{file}: classify each column in public_reader.rs"
            );
        }
    }

    #[test]
    fn nothing_internal_or_personal_is_public() {
        for (name, exposure, _) in ROADMAP_FIELDS.iter().chain(ITEM_FIELDS) {
            if name.ends_with("_at")
                || *name == "id"
                || *name == "pid"
                || name.ends_with("_id") && *name != "public_id"
            {
                assert_eq!(*exposure, Exposure::Internal, "{name}");
            }
        }
    }

    #[test]
    fn a_reviewed_only_reader_drops_translations_that_need_review() {
        let src = json!("Book online");
        let stored = json!({
            "cy": { "title": rules::stored_translation(&json!("Book by phone"), &json!("Ffoniwch")) },
            "fr": { "title": rules::stored_translation(&src, &json!("Réserver en ligne")) },
        });
        let sources = BTreeMap::from([("title", src)]);
        let all = public_translations(&stored, &sources, false);
        assert_eq!(all["cy"]["title"]["needs_review"], true);
        let reviewed = public_translations(&stored, &sources, true);
        assert!(reviewed.get("cy").is_none());
        assert_eq!(reviewed["fr"]["title"]["value"], "Réserver en ligne");
    }
}

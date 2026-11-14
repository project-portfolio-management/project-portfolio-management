//! A product plan's public product roadmap (RE-1, RE-6). The rules are in [`crate::public_roadmap`].
//!
//! Every route is beneath `/api/plans/{pid}`, so the plan guard decides who may read and write (a private plan's
//! roadmap is a `404` to a non-member), and the `roadmap` module can turn it off for a plan.
//!
//! * `GET /api/plans/{pid}/roadmap`: the roadmap, with its items in page order (`404` before it is made);
//! * `PUT /api/plans/{pid}/roadmap`: make it (a `Product` plan only) or change its own fields;
//! * `POST /api/plans/{pid}/roadmap/items`: add an item, last in its placement;
//! * `PUT /api/plans/{pid}/roadmap/items/{ipid}`: change an item's text, tags, organizations or details;
//! * `POST /api/plans/{pid}/roadmap/items/{ipid}/move`: move it to a placement (or none) and a place in it;
//! * `POST /api/plans/{pid}/roadmap/items/{ipid}/delete`: remove it (its public id is never reused);
//! * `PUT /api/plans/{pid}/roadmap/translations/{lang}` and `…/items/{ipid}/translations/{lang}`: a translation
//!   of some fields, recorded against the source text it was made from (RE-6).
//!
//! Every write is audited in its own transaction.

use std::collections::BTreeMap;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::public_roadmap_items as items;
use crate::models::_entities::public_roadmaps as roadmaps;
use crate::models::audit_logs::Model as AuditModel;
use crate::public_roadmap::{self as rules, Detail};

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/roadmap", get(show_roadmap))
        .add("/plans/{pid}/roadmap", put(put_roadmap))
        .add("/plans/{pid}/roadmap/items", post(add_item))
        .add("/plans/{pid}/roadmap/items/{ipid}", put(change_item))
        .add("/plans/{pid}/roadmap/items/{ipid}/move", post(move_item))
        .add(
            "/plans/{pid}/roadmap/items/{ipid}/delete",
            post(delete_item),
        )
        .add("/plans/{pid}/roadmap/reviewed", put(mark_reviewed))
        .add("/public-roadmaps/stale", get(stale_list))
        .add(
            "/plans/{pid}/roadmap/translations/{lang}",
            put(translate_roadmap),
        )
        .add(
            "/plans/{pid}/roadmap/items/{ipid}/translations/{lang}",
            put(translate_item),
        )
}

/// The current source value of each translatable field of a roadmap.
pub(crate) fn roadmap_sources(r: &roadmaps::Model) -> BTreeMap<&'static str, Value> {
    BTreeMap::from([
        ("title", json!(r.title)),
        ("vision", json!(r.vision)),
        ("service_description", json!(r.service_description)),
    ])
}

/// The current source value of each translatable field of an item.
pub(crate) fn item_sources(i: &items::Model) -> BTreeMap<&'static str, Value> {
    let none_if_empty = |v: &Value| {
        if v.as_array().is_some_and(Vec::is_empty) {
            Value::Null
        } else {
            v.clone()
        }
    };
    BTreeMap::from([
        ("title", json!(i.title)),
        ("description", json!(i.description)),
        ("expected_outcome", json!(i.expected_outcome)),
        ("phase", json!(i.phase)),
        ("tags", none_if_empty(&i.tags)),
        ("details", none_if_empty(&i.details)),
    ])
}

/// JSON view of an item, as its editors see it.
pub(crate) fn item_view(i: &items::Model) -> Value {
    json!({
        "pid": i.pid, "public_id": i.public_id, "title": i.title, "description": i.description,
        "expected_outcome": i.expected_outcome, "phase": i.phase, "tags": i.tags,
        "organization_pids": i.organization_pids, "details": i.details, "placement": i.placement,
        "position": i.position,
        "translations": rules::shown_translations(&i.translations, &item_sources(i)),
    })
}

/// How fresh a roadmap is today, in the deployment's time zone, with the rule that says so (RE-3).
pub(crate) fn freshness_view(r: &roadmaps::Model) -> Value {
    let days = rules::stale_days();
    let today = crate::tz::deployment_today();
    json!({
        "state": rules::freshness(r.reviewed_on, today, days),
        "stale_after_days": days,
        "stale_from": r.reviewed_on.map(|d| d + chrono::Duration::days(days + 1)),
        "today": today,
    })
}

/// JSON view of a roadmap and its items, in page order, unplaced last.
pub(crate) fn roadmap_view(r: &roadmaps::Model, list: &[items::Model]) -> Value {
    let mut list: Vec<&items::Model> = list.iter().collect();
    list.sort_by_key(|i| {
        (
            rules::placement_rank(i.placement.as_deref()),
            i.position,
            i.id,
        )
    });
    json!({
        "pid": r.pid, "plan_pid": r.plan_pid, "slug": r.slug, "title": r.title, "vision": r.vision,
        "service_description": r.service_description, "category_pid": r.category_pid,
        "published": r.published, "reviewed_on": r.reviewed_on, "freshness": freshness_view(r),
        "translations": rules::shown_translations(&r.translations, &roadmap_sources(r)),
        "items": list.into_iter().map(item_view).collect::<Vec<_>>(),
    })
}

/// The roadmap of a plan, if it has one.
pub(crate) async fn find_roadmap(
    tx: &mut crate::db::Tx,
    plan_pid: Uuid,
    lock: bool,
) -> Result<Option<roadmaps::Model>> {
    let sql = if lock {
        "SELECT * FROM public_roadmaps WHERE plan_pid = $1 FOR UPDATE"
    } else {
        "SELECT * FROM public_roadmaps WHERE plan_pid = $1"
    };
    Ok(sqlx::query_as(sql)
        .bind(plan_pid)
        .fetch_optional(tx.conn())
        .await?)
}

/// A roadmap's live items.
pub(crate) async fn live_items(
    tx: &mut crate::db::Tx,
    roadmap_id: i64,
) -> Result<Vec<items::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM public_roadmap_items WHERE roadmap_id = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(roadmap_id)
    .fetch_all(tx.conn())
    .await?)
}

/// `GET /api/plans/{pid}/roadmap`
#[debug_handler]
async fn show_roadmap(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = find_roadmap(&mut tx, plan.pid, false)
        .await?
        .ok_or(Error::NotFound)?;
    let list = live_items(&mut tx, r.id).await?;
    tx.commit().await?;
    format::json(roadmap_view(&r, &list))
}

/// `PUT /api/plans/{pid}/roadmap` body: absent fields are left as they are; on creation `title` is required.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RoadmapPayload {
    /// The title, 1 to 200 characters; required when the roadmap is made.
    #[serde(default)]
    title: Option<String>,
    /// The public address; made from the title when the roadmap is made without one.
    #[serde(default)]
    slug: Option<String>,
    /// Where the product is going; an empty string clears it.
    #[serde(default)]
    vision: Option<String>,
    /// What the product is, for a reader who has never heard of it; an empty string clears it.
    #[serde(default)]
    service_description: Option<String>,
    /// An active roadmap category; `null` clears it, absent keeps it.
    #[serde(default, deserialize_with = "present")]
    #[expect(
        clippy::option_option,
        reason = "left out and `null` mean different things here"
    )]
    category_pid: Option<Option<Uuid>>,
    /// Whether the public reader shows it.
    #[serde(default)]
    published: Option<bool>,
}

/// Tells a field sent as `null` from one left out.
fn present<'de, T, D>(d: D) -> std::result::Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    T::deserialize(d).map(Some)
}

/// Map a slug clash to a 409.
fn slug_clash(e: sqlx::Error) -> Error {
    if e.to_string().contains("public_roadmap_slug_once") {
        return Error::CustomError(
            axum::http::StatusCode::CONFLICT,
            axa::error::ErrorDetail::new("conflict", "another roadmap has that slug"),
        );
    }
    e.into()
}

/// Check a roadmap's own fields against what it has; the title and slug it will have.
async fn roadmap_checked(
    tx: &mut crate::db::Tx,
    existing: Option<&roadmaps::Model>,
    b: &RoadmapPayload,
) -> Result<(String, String)> {
    let title = b.title.as_deref().or(existing.map(|r| r.title.as_str()));
    let mut problems = Vec::new();
    rules::title_problems(title, &mut problems);
    rules::text_problems("vision", b.vision.as_deref(), &mut problems);
    rules::text_problems(
        "service_description",
        b.service_description.as_deref(),
        &mut problems,
    );
    let slug = match (&b.slug, existing) {
        (Some(s), _) => Some(s.trim().to_string()),
        (None, Some(r)) => Some(r.slug.clone()),
        (None, None) => title.and_then(rules::slug_from),
    };
    match &slug {
        Some(s) if rules::is_slug(s) => {}
        Some(_) => problems.push(
            "slug is lower-case letters, digits and single hyphens, at most 64 characters"
                .to_string(),
        ),
        None => problems.push(
            "slug is required when the title has no letters or digits to make one from".to_string(),
        ),
    }
    if let Some(Some(c)) = b.category_pid {
        let active: Option<bool> =
            sqlx::query_scalar("SELECT active FROM roadmap_categories WHERE pid = $1")
                .bind(c)
                .fetch_optional(tx.conn())
                .await?;
        if active != Some(true) {
            problems.push(format!("{c} is not an active roadmap category"));
        }
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    Ok((
        rules::stored_text(title).unwrap_or_default(),
        slug.unwrap_or_default(),
    ))
}

/// `PUT /api/plans/{pid}/roadmap`
#[debug_handler]
async fn put_roadmap(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<RoadmapPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let existing = find_roadmap(&mut tx, plan.pid, true).await?;
    if existing.is_none() && plan.kind.as_deref() != Some("Product") {
        return Err(unprocessable(
            "a public product roadmap belongs to a plan of kind Product",
        ));
    }
    let (title, slug) = roadmap_checked(&mut tx, existing.as_ref(), &b).await?;
    let vision = match &b.vision {
        Some(v) => rules::stored_text(Some(v)),
        None => existing.as_ref().and_then(|r| r.vision.clone()),
    };
    let service = match &b.service_description {
        Some(v) => rules::stored_text(Some(v)),
        None => existing
            .as_ref()
            .and_then(|r| r.service_description.clone()),
    };
    let category = match b.category_pid {
        Some(c) => c,
        None => existing.as_ref().and_then(|r| r.category_pid),
    };
    let published = b
        .published
        .or(existing.as_ref().map(|r| r.published))
        .unwrap_or(false);
    let (row, event): (roadmaps::Model, &str) = if let Some(r) = &existing {
        let row = sqlx::query_as(
            "UPDATE public_roadmaps SET title = $2, slug = $3, vision = $4, service_description = $5,
                 category_pid = $6, published = $7, updated_at = now()
             WHERE id = $1 RETURNING *",
        )
        .bind(r.id)
        .bind(&title)
        .bind(&slug)
        .bind(&vision)
        .bind(&service)
        .bind(category)
        .bind(published)
        .fetch_one(tx.conn())
        .await
        .map_err(slug_clash)?;
        (row, "public_roadmap_changed")
    } else {
        let row = sqlx::query_as(
            "INSERT INTO public_roadmaps
                 (pid, plan_pid, title, slug, vision, service_description, category_pid, published)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(plan.pid)
        .bind(&title)
        .bind(&slug)
        .bind(&vision)
        .bind(&service)
        .bind(category)
        .bind(published)
        .fetch_one(tx.conn())
        .await
        .map_err(slug_clash)?;
        (row, "public_roadmap_created")
    };
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        event,
        caller.actor(),
        Some(json!({ "roadmap": row.pid, "published": row.published })),
    )
    .await
    .map_err(super::audit_failed)?;
    let list = live_items(&mut tx, row.id).await?;
    tx.commit().await?;
    format::json(roadmap_view(&row, &list))
}

/// An item's fields, for adding one (title required) or changing one (absent fields are left as they are).
#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
pub(crate) struct ItemPayload {
    /// The title, 1 to 200 characters.
    #[serde(default)]
    pub(crate) title: Option<String>,
    /// What the work is; an empty string clears it.
    #[serde(default)]
    pub(crate) description: Option<String>,
    /// What changes, and for whom; an empty string clears it.
    #[serde(default)]
    pub(crate) expected_outcome: Option<String>,
    /// The phase it is in, in the product's own words; an empty string clears it.
    #[serde(default)]
    pub(crate) phase: Option<String>,
    /// Tags in order, each 1 to 50 characters, no comma, no repeats; the whole list.
    #[serde(default)]
    pub(crate) tags: Option<Vec<String>>,
    /// The organizations the work is for, each an active organization; the whole set.
    #[serde(default)]
    pub(crate) organization_pids: Option<Vec<Uuid>>,
    /// Details nested up to three levels; the whole list.
    #[serde(default)]
    pub(crate) details: Option<Vec<Detail>>,
    /// On adding: a placement, or absent for unplaced.
    #[serde(default)]
    pub(crate) placement: Option<String>,
}

/// Check an item's fields; `title` is the title it will have.
pub(crate) async fn item_problems(
    tx: &mut crate::db::Tx,
    title: Option<&str>,
    b: &ItemPayload,
) -> Result<Vec<String>> {
    let mut out = Vec::new();
    rules::title_problems(title, &mut out);
    rules::text_problems("description", b.description.as_deref(), &mut out);
    rules::text_problems("expected_outcome", b.expected_outcome.as_deref(), &mut out);
    rules::phase_problems(b.phase.as_deref(), &mut out);
    if let Some(t) = &b.tags {
        rules::tag_problems(t, &mut out);
    }
    if let Some(d) = &b.details {
        rules::detail_problems(d, &mut out);
    }
    if let Some(p) = &b.placement
        && !rules::is_placement(p)
    {
        out.push(format!(
            "placement is one of {}",
            rules::PLACEMENTS.join(", ")
        ));
    }
    if let Some(orgs) = &b.organization_pids {
        if orgs.len() > rules::ORGANIZATIONS_MAX {
            out.push(format!(
                "at most {} organizations",
                rules::ORGANIZATIONS_MAX
            ));
        }
        let known: Vec<Uuid> = sqlx::query_scalar(
            "SELECT pid FROM organizations WHERE pid = ANY($1) AND active = true",
        )
        .bind(orgs)
        .fetch_all(tx.conn())
        .await?;
        for o in orgs {
            if !known.contains(o) {
                out.push(format!("{o} is not an active organization"));
            }
        }
    }
    Ok(out)
}

/// The organizations as stored: no repeats, in the order given.
fn stored_orgs(orgs: &[Uuid]) -> Vec<Uuid> {
    let mut out = Vec::new();
    for o in orgs {
        if !out.contains(o) {
            out.push(*o);
        }
    }
    out
}

/// A fresh public id, drawn again on the rare clash with one in use or ever used.
async fn fresh_public_id(tx: &mut crate::db::Tx) -> Result<String> {
    loop {
        let id = rules::public_id_from(Uuid::new_v4().as_bytes());
        let used: Option<i32> =
            sqlx::query_scalar("SELECT 1 FROM public_roadmap_items WHERE public_id = $1")
                .bind(&id)
                .fetch_optional(tx.conn())
                .await?;
        if used.is_none() {
            return Ok(id);
        }
    }
}

/// The plan's roadmap for a write, locked, or `404`.
async fn roadmap_for_write(tx: &mut crate::db::Tx, plan_pid: Uuid) -> Result<roadmaps::Model> {
    find_roadmap(tx, plan_pid, true)
        .await?
        .ok_or(Error::NotFound)
}

/// A live item of a roadmap, or `404`.
async fn find_live_item(
    tx: &mut crate::db::Tx,
    roadmap_id: i64,
    ipid: &str,
) -> Result<items::Model> {
    let ipid = super::parse_pid(ipid)?;
    sqlx::query_as(
        "SELECT * FROM public_roadmap_items WHERE pid = $1 AND roadmap_id = $2 AND deleted_at IS NULL",
    )
    .bind(ipid)
    .bind(roadmap_id)
    .fetch_optional(tx.conn())
    .await?
    .ok_or(Error::NotFound)
}

/// `POST /api/plans/{pid}/roadmap/items`
#[debug_handler]
async fn add_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ItemPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let problems = item_problems(&mut tx, b.title.as_deref(), &b).await?;
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let row = insert_item(&mut tx, r.id, &b).await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_item_added",
        caller.actor(),
        Some(json!({ "item": row.pid, "placement": row.placement })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(item_view(&row))
}

/// Insert an item, already checked, last in its placement.
pub(crate) async fn insert_item(
    tx: &mut crate::db::Tx,
    roadmap_id: i64,
    b: &ItemPayload,
) -> Result<items::Model> {
    let public_id = fresh_public_id(tx).await?;
    let position: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(position) + 1, 0) FROM public_roadmap_items
         WHERE roadmap_id = $1 AND placement IS NOT DISTINCT FROM $2 AND deleted_at IS NULL",
    )
    .bind(roadmap_id)
    .bind(&b.placement)
    .fetch_one(tx.conn())
    .await?;
    Ok(sqlx::query_as(
        "INSERT INTO public_roadmap_items
             (pid, roadmap_id, public_id, title, description, expected_outcome, phase, tags,
              organization_pids, details, placement, position)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(roadmap_id)
    .bind(public_id)
    .bind(rules::stored_text(b.title.as_deref()))
    .bind(rules::stored_text(b.description.as_deref()))
    .bind(rules::stored_text(b.expected_outcome.as_deref()))
    .bind(rules::stored_text(b.phase.as_deref()))
    .bind(json!(rules::stored_tags(
        b.tags.as_deref().unwrap_or_default()
    )))
    .bind(stored_orgs(
        b.organization_pids.as_deref().unwrap_or_default(),
    ))
    .bind(json!(rules::stored_details(
        b.details.as_deref().unwrap_or_default()
    )))
    .bind(&b.placement)
    .bind(position)
    .fetch_one(tx.conn())
    .await?)
}

/// Write an item's changed fields, already checked. `None` leaves a field; `Some("")` clears a text field.
pub(crate) async fn update_item(
    tx: &mut crate::db::Tx,
    i: &items::Model,
    b: &ItemPayload,
) -> Result<items::Model> {
    let text = |new: &Option<String>, old: &Option<String>| match new {
        Some(v) => rules::stored_text(Some(v)),
        None => old.clone(),
    };
    Ok(sqlx::query_as(
        "UPDATE public_roadmap_items SET title = $2, description = $3, expected_outcome = $4, phase = $5,
             tags = $6, organization_pids = $7, details = $8, updated_at = now()
         WHERE id = $1 RETURNING *",
    )
    .bind(i.id)
    .bind(
        b.title
            .as_deref()
            .and_then(|t| rules::stored_text(Some(t)))
            .unwrap_or_else(|| i.title.clone()),
    )
    .bind(text(&b.description, &i.description))
    .bind(text(&b.expected_outcome, &i.expected_outcome))
    .bind(text(&b.phase, &i.phase))
    .bind(
        b.tags
            .as_deref()
            .map_or_else(|| i.tags.clone(), |t| json!(rules::stored_tags(t))),
    )
    .bind(
        b.organization_pids
            .as_deref()
            .map_or_else(|| i.organization_pids.clone(), stored_orgs),
    )
    .bind(
        b.details
            .as_deref()
            .map_or_else(|| i.details.clone(), |d| json!(rules::stored_details(d))),
    )
    .fetch_one(tx.conn())
    .await?)
}

/// `PUT /api/plans/{pid}/roadmap/items/{ipid}`
#[debug_handler]
async fn change_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, ipid)): Path<(String, String)>,
    Json(b): Json<ItemPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let i = find_live_item(&mut tx, r.id, &ipid).await?;
    if b.placement.is_some() {
        return Err(unprocessable(
            "move an item with …/move, not by changing it",
        ));
    }
    let title = b.title.as_deref().unwrap_or(&i.title);
    let problems = item_problems(&mut tx, Some(title), &b).await?;
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let row = update_item(&mut tx, &i, &b).await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_item_changed",
        caller.actor(),
        Some(json!({ "item": row.pid })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(item_view(&row))
}

/// `POST …/items/{ipid}/move` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MovePayload {
    /// The placement to move to; `null` makes the item unplaced.
    placement: Option<String>,
    /// Its place in that placement, counted from 0; absent or past the end puts it last.
    #[serde(default)]
    index: Option<usize>,
}

/// Put `item` at `index` among the other live items of its new placement, numbering them 0, 1, 2 …
pub(crate) async fn place_item(
    tx: &mut crate::db::Tx,
    i: &items::Model,
    placement: Option<&str>,
    index: Option<usize>,
) -> Result<items::Model> {
    let others: Vec<i64> = sqlx::query_scalar(
        "SELECT id FROM public_roadmap_items
         WHERE roadmap_id = $1 AND placement IS NOT DISTINCT FROM $2 AND deleted_at IS NULL AND id <> $3
         ORDER BY position, id",
    )
    .bind(i.roadmap_id)
    .bind(placement)
    .bind(i.id)
    .fetch_all(tx.conn())
    .await?;
    let mut order = others;
    let at = index.unwrap_or(order.len()).min(order.len());
    order.insert(at, i.id);
    for (n, id) in order.iter().enumerate() {
        sqlx::query(
            "UPDATE public_roadmap_items SET position = $2, placement = $3, updated_at = now() WHERE id = $1",
        )
        .bind(id)
        .bind(i32::try_from(n).unwrap_or(i32::MAX))
        .bind(placement)
        .execute(tx.conn())
        .await?;
    }
    Ok(
        sqlx::query_as("SELECT * FROM public_roadmap_items WHERE id = $1")
            .bind(i.id)
            .fetch_one(tx.conn())
            .await?,
    )
}

/// `POST /api/plans/{pid}/roadmap/items/{ipid}/move`
#[debug_handler]
async fn move_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, ipid)): Path<(String, String)>,
    Json(b): Json<MovePayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if let Some(p) = &b.placement
        && !rules::is_placement(p)
    {
        return Err(unprocessable(&format!(
            "placement is one of {}, or null",
            rules::PLACEMENTS.join(", ")
        )));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let i = find_live_item(&mut tx, r.id, &ipid).await?;
    let from = i.placement.clone();
    let row = place_item(&mut tx, &i, b.placement.as_deref(), b.index).await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_item_moved",
        caller.actor(),
        Some(
            json!({ "item": row.pid, "from": from, "to": row.placement, "position": row.position }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(item_view(&row))
}

/// `POST /api/plans/{pid}/roadmap/items/{ipid}/delete`
#[debug_handler]
async fn delete_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, ipid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let i = find_live_item(&mut tx, r.id, &ipid).await?;
    sqlx::query(
        "UPDATE public_roadmap_items SET deleted_at = now(), updated_at = now() WHERE id = $1",
    )
    .bind(i.id)
    .execute(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_item_removed",
        caller.actor(),
        Some(json!({ "item": i.pid, "public_id": i.public_id })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "pid": i.pid, "removed": true }))
}

/// A translation into one language: field to value; `null` or an empty string removes that field's translation.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(transparent)]
pub(crate) struct TranslationPayload(BTreeMap<String, Value>);

/// Merge `body` (field to value; `null` or `""` removes that field's translation) into `stored` for `lang`.
fn merged_translations(
    stored: &Value,
    lang: &str,
    fields: &[&str],
    sources: &BTreeMap<&str, Value>,
    body: &BTreeMap<String, Value>,
) -> std::result::Result<Value, Vec<String>> {
    let mut problems = Vec::new();
    if !crate::issues::is_language_code(lang) {
        problems.push(format!("`{lang}` is not a language code"));
    }
    let mut all = stored.as_object().cloned().unwrap_or_default();
    let mut mine = all
        .get(lang)
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (field, value) in body {
        let removes = value.is_null() || value.as_str().is_some_and(|s| s.trim().is_empty());
        if removes {
            mine.remove(field);
            continue;
        }
        let source = sources.get(field.as_str()).cloned().unwrap_or(Value::Null);
        match rules::translation_problem(fields, field, &source, value) {
            Some(p) => problems.push(p),
            None => {
                mine.insert(field.clone(), rules::stored_translation(&source, value));
            }
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    if mine.is_empty() {
        all.remove(lang);
    } else {
        all.insert(lang.to_string(), Value::Object(mine));
    }
    Ok(Value::Object(all))
}

/// `PUT /api/plans/{pid}/roadmap/translations/{lang}` — body: field to text (`title`, `vision`,
/// `service_description`); `null` or an empty string removes that translation.
#[debug_handler]
async fn translate_roadmap(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, lang)): Path<(String, String)>,
    Json(payload): Json<TranslationPayload>,
) -> Result<Response> {
    let TranslationPayload(body) = payload;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let t = merged_translations(
        &r.translations,
        &lang,
        &rules::ROADMAP_FIELDS,
        &roadmap_sources(&r),
        &body,
    )
    .map_err(|p| unprocessable(&p.join("; ")))?;
    let row: roadmaps::Model = sqlx::query_as(
        "UPDATE public_roadmaps SET translations = $2, updated_at = now() WHERE id = $1 RETURNING *",
    )
    .bind(r.id)
    .bind(t)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_translated",
        caller.actor(),
        Some(json!({ "roadmap": row.pid, "language": lang, "fields": body.keys().collect::<Vec<_>>() })),
    )
    .await
    .map_err(super::audit_failed)?;
    let list = live_items(&mut tx, row.id).await?;
    tx.commit().await?;
    format::json(roadmap_view(&row, &list))
}

/// `PUT /api/plans/{pid}/roadmap/items/{ipid}/translations/{lang}` — body: field to value (`title`,
/// `description`, `expected_outcome`, `phase` as text; `tags` as a list as long as the source's; `details` as the
/// source's lines at the same levels); `null` or an empty string removes that translation.
#[debug_handler]
async fn translate_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, ipid, lang)): Path<(String, String, String)>,
    Json(payload): Json<TranslationPayload>,
) -> Result<Response> {
    let TranslationPayload(body) = payload;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let i = find_live_item(&mut tx, r.id, &ipid).await?;
    let t = merged_translations(
        &i.translations,
        &lang,
        &rules::ITEM_FIELDS,
        &item_sources(&i),
        &body,
    )
    .map_err(|p| unprocessable(&p.join("; ")))?;
    let row: items::Model = sqlx::query_as(
        "UPDATE public_roadmap_items SET translations = $2, updated_at = now() WHERE id = $1 RETURNING *",
    )
    .bind(i.id)
    .bind(t)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_item_translated",
        caller.actor(),
        Some(
            json!({ "item": row.pid, "language": lang, "fields": body.keys().collect::<Vec<_>>() }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(item_view(&row))
}

/// `PUT …/roadmap/reviewed` body: `today` true marks it reviewed today in the deployment's time zone; otherwise
/// `reviewed_on` is the date, or `null` to clear it.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReviewedPayload {
    /// Mark it reviewed today.
    #[serde(default)]
    today: bool,
    /// The date it was reviewed, not after today; `null` clears it. Ignored when `today` is true.
    #[serde(default)]
    reviewed_on: Option<chrono::NaiveDate>,
}

/// `PUT /api/plans/{pid}/roadmap/reviewed` — the only write that changes `reviewed_on` (RE-3).
#[debug_handler]
async fn mark_reviewed(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ReviewedPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let today = crate::tz::deployment_today();
    let date = if b.today { Some(today) } else { b.reviewed_on };
    if date.is_some_and(|d| d > today) {
        return Err(unprocessable(&format!(
            "reviewed_on is not after today ({today})"
        )));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let r = roadmap_for_write(&mut tx, plan.pid).await?;
    let row: roadmaps::Model = sqlx::query_as(
        "UPDATE public_roadmaps SET reviewed_on = $2, updated_at = now() WHERE id = $1 RETURNING *",
    )
    .bind(r.id)
    .bind(date)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "public_roadmap_reviewed",
        caller.actor(),
        Some(json!({ "roadmap": row.pid, "from": r.reviewed_on, "to": row.reviewed_on })),
    )
    .await
    .map_err(super::audit_failed)?;
    let list = live_items(&mut tx, row.id).await?;
    tx.commit().await?;
    format::json(roadmap_view(&row, &list))
}

/// `GET /api/public-roadmaps/stale` — the roadmaps that are stale or were never reviewed, oldest review first,
/// leaving out plans the caller may not read and plans no longer live.
#[debug_handler]
async fn stale_list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let rows: Vec<roadmaps::Model> = sqlx::query_as(
        "SELECT r.* FROM public_roadmaps r JOIN plans p ON p.pid = r.plan_pid AND p.active
         WHERE NOT (r.plan_pid = ANY($1))
         ORDER BY r.reviewed_on NULLS FIRST, r.id",
    )
    .bind(hidden.plan_pids())
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let days = rules::stale_days();
    let today = crate::tz::deployment_today();
    let list: Vec<Value> = rows
        .iter()
        .filter(|r| rules::freshness(r.reviewed_on, today, days) != "fresh")
        .map(|r| {
            json!({ "plan_pid": r.plan_pid, "title": r.title, "slug": r.slug, "published": r.published,
                    "reviewed_on": r.reviewed_on, "state": rules::freshness(r.reviewed_on, today, days) })
        })
        .collect();
    format::json(
        json!({ "stale_after_days": days, "today": today, "count": list.len(), "roadmaps": list }),
    )
}

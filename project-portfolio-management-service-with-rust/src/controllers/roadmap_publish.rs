//! Publishing a public product roadmap's pending changes as one change set (RE-2).
//!
//! * `POST /api/plans/{pid}/roadmap/publish/preview`: what publishing this change set would change, and what
//!   conflicts; writes nothing.
//! * `POST /api/plans/{pid}/roadmap/publish`: every change, in one transaction with one audit row per record, or
//!   none of them.
//!
//! An editor gathers changes in the browser. Each changed field carries its **base**, the value the editor first saw,
//! and its **value**, what the editor wants. A field **conflicts** when its stored value is now neither the base nor
//! the value: someone else changed it to something else meanwhile. (A field someone else already changed to the same
//! value is not a conflict; it is simply already done.) Any conflict refuses the whole set with `409`, naming each
//! conflicting field with its base, value and current value. Values are compared as they would be stored: trimmed
//! text, an empty text as none, lists as given.
//!
//! A change set may also add items (`new`). The routes are `POST`s, so an `Idempotency-Key` makes a retry whose first
//! answer was lost return that answer instead of writing twice (BR-1).

use std::collections::BTreeMap;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::public_roadmaps::{self as pr, ItemPayload};
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::public_roadmap_items as items;
use crate::models::_entities::public_roadmaps as roadmaps;
use crate::models::audit_logs::Model as AuditModel;
use crate::public_roadmap::{self as rules, Detail};

/// The roadmap's own fields a change set may change.
pub const ROADMAP_FIELDS: [&str; 5] = [
    "title",
    "vision",
    "service_description",
    "category_pid",
    "published",
];
/// An item's fields a change set may change.
pub const ITEM_FIELDS: [&str; 8] = [
    "title",
    "description",
    "expected_outcome",
    "phase",
    "tags",
    "organization_pids",
    "details",
    "placement",
];

/// One changed field: what the editor first saw, and what it wants.
#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub(crate) struct FieldChange {
    /// The value when the editor first read it.
    base: Value,
    /// The value the editor wants.
    value: Value,
}

/// A change to an existing item.
#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub(crate) struct ItemChange {
    /// The item.
    pid: Uuid,
    /// Field name to change.
    fields: BTreeMap<String, FieldChange>,
}

/// A change set.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ChangeSet {
    /// The roadmap's own fields: field name to change.
    #[serde(default)]
    roadmap: BTreeMap<String, FieldChange>,
    /// Changes to existing items.
    #[serde(default)]
    items: Vec<ItemChange>,
    /// Items to add, each as `POST …/roadmap/items` takes it.
    #[serde(default)]
    new: Vec<ItemPayload>,
}

/// A field's value as it would be stored, or what is wrong with it.
///
/// # Errors
///
/// A message when the value has the wrong type for the field.
pub fn normal(field: &str, v: &Value) -> std::result::Result<Value, String> {
    let text = |v: &Value| match v {
        Value::Null => Ok(Value::Null),
        Value::String(s) => Ok(json!(rules::stored_text(Some(s)))),
        _ => Err(format!("{field} is text or null")),
    };
    match field {
        "title"
        | "description"
        | "expected_outcome"
        | "phase"
        | "vision"
        | "service_description" => text(v),
        "tags" => serde_json::from_value::<Vec<String>>(v.clone())
            .map(|t| json!(rules::stored_tags(&t)))
            .map_err(|_| "tags is a list of text".to_string()),
        "details" => serde_json::from_value::<Vec<Detail>>(v.clone())
            .map(|d| json!(rules::stored_details(&d)))
            .map_err(|_| "details is a list of { text, level }".to_string()),
        "organization_pids" => serde_json::from_value::<Vec<Uuid>>(v.clone())
            .map(|o| {
                let mut seen = Vec::new();
                for p in o {
                    if !seen.contains(&p) {
                        seen.push(p);
                    }
                }
                json!(seen)
            })
            .map_err(|_| "organization_pids is a list of ids".to_string()),
        "category_pid" => serde_json::from_value::<Option<Uuid>>(v.clone())
            .map(|c| json!(c))
            .map_err(|_| "category_pid is an id or null".to_string()),
        "published" => v
            .as_bool()
            .map(|b| json!(b))
            .ok_or_else(|| "published is true or false".to_string()),
        "placement" => match v {
            Value::Null => Ok(Value::Null),
            Value::String(p) if rules::is_placement(p) => Ok(json!(p)),
            _ => Err(format!(
                "placement is one of {}, or null",
                rules::PLACEMENTS.join(", ")
            )),
        },
        other => Err(format!("`{other}` cannot be changed here")),
    }
}

/// Whether a field conflicts: its current value is neither the base the editor saw nor the value it wants.
#[must_use]
pub fn conflicts(base: &Value, value: &Value, current: &Value) -> bool {
    current != base && current != value
}

/// The roadmap's current value of a field, as stored.
fn roadmap_current(r: &roadmaps::Model, field: &str) -> Value {
    match field {
        "title" => json!(r.title),
        "vision" => json!(r.vision),
        "service_description" => json!(r.service_description),
        "category_pid" => json!(r.category_pid),
        "published" => json!(r.published),
        _ => Value::Null,
    }
}

/// An item's current value of a field, as stored.
fn item_current(i: &items::Model, field: &str) -> Value {
    match field {
        "title" => json!(i.title),
        "description" => json!(i.description),
        "expected_outcome" => json!(i.expected_outcome),
        "phase" => json!(i.phase),
        "tags" => i.tags.clone(),
        "organization_pids" => json!(i.organization_pids),
        "details" => i.details.clone(),
        "placement" => json!(i.placement),
        _ => Value::Null,
    }
}

/// What a change set would do: the fields that change, the items added, and the conflicts and problems that refuse
/// it. Built once and used by both the preview and the publish, so the two cannot disagree.
#[derive(Debug, Default)]
struct Plan {
    /// Roadmap field to its new value.
    roadmap: BTreeMap<String, Value>,
    /// Item to its changed fields and their new values.
    items: Vec<(items::Model, BTreeMap<String, Value>)>,
    /// What the preview reports: one entry per changed field.
    changes: Vec<Value>,
    conflicts: Vec<Value>,
    problems: Vec<String>,
}

/// Work out a change set against what is stored now.
async fn plan(
    tx: &mut crate::db::Tx,
    r: &roadmaps::Model,
    list: &[items::Model],
    set: &ChangeSet,
) -> Result<Plan> {
    let mut p = Plan::default();
    for (field, c) in &set.roadmap {
        if !ROADMAP_FIELDS.contains(&field.as_str()) {
            p.problems
                .push(format!("roadmap field `{field}` cannot be changed here"));
            continue;
        }
        let (Ok(base), Ok(value)) = (normal(field, &c.base), normal(field, &c.value)) else {
            p.problems.extend(normal(field, &c.value).err());
            p.problems
                .extend(normal(field, &c.base).err().map(|e| format!("base: {e}")));
            continue;
        };
        let current = roadmap_current(r, field);
        if conflicts(&base, &value, &current) {
            p.conflicts.push(json!({ "subject": "roadmap", "field": field, "base": base, "value": value, "current": current }));
        } else if current != value {
            p.changes.push(
                json!({ "subject": "roadmap", "field": field, "from": current, "to": value }),
            );
            p.roadmap.insert(field.clone(), value);
        }
    }
    if let Some(t) = p.roadmap.get("title") {
        rules::title_problems(t.as_str(), &mut p.problems);
    }
    for f in ["vision", "service_description"] {
        if let Some(v) = p.roadmap.get(f) {
            rules::text_problems(f, v.as_str(), &mut p.problems);
        }
    }
    if let Some(Value::String(c)) = p.roadmap.get("category_pid") {
        let active: Option<bool> =
            sqlx::query_scalar("SELECT active FROM roadmap_categories WHERE pid = $1::uuid")
                .bind(c)
                .fetch_optional(tx.conn())
                .await?;
        if active != Some(true) {
            p.problems
                .push(format!("{c} is not an active roadmap category"));
        }
    }
    let mut seen = Vec::new();
    for change in &set.items {
        let Some(i) = list.iter().find(|i| i.pid == change.pid) else {
            p.conflicts.push(
                json!({ "subject": "item", "item": change.pid, "field": null, "gone": true }),
            );
            continue;
        };
        if seen.contains(&change.pid) {
            p.problems
                .push(format!("item {} is changed twice", change.pid));
            continue;
        }
        seen.push(change.pid);
        let mut new_values = BTreeMap::new();
        for (field, c) in &change.fields {
            if !ITEM_FIELDS.contains(&field.as_str()) {
                p.problems
                    .push(format!("item field `{field}` cannot be changed here"));
                continue;
            }
            let (Ok(base), Ok(value)) = (normal(field, &c.base), normal(field, &c.value)) else {
                p.problems.extend(normal(field, &c.value).err());
                p.problems
                    .extend(normal(field, &c.base).err().map(|e| format!("base: {e}")));
                continue;
            };
            let current = item_current(i, field);
            if conflicts(&base, &value, &current) {
                p.conflicts.push(json!({ "subject": "item", "item": i.pid, "field": field, "base": base, "value": value, "current": current }));
            } else if current != value {
                p.changes.push(json!({ "subject": "item", "item": i.pid, "field": field, "from": current, "to": value }));
                new_values.insert(field.clone(), value);
            }
        }
        if !new_values.is_empty() {
            let payload = payload_of(&new_values);
            let title = payload.title.clone().unwrap_or_else(|| i.title.clone());
            p.problems
                .extend(pr::item_problems(tx, Some(&title), &payload).await?);
            p.items.push((i.clone(), new_values));
        }
    }
    for (n, item) in set.new.iter().enumerate() {
        let found = pr::item_problems(tx, item.title.as_deref(), item).await?;
        p.problems.extend(
            found
                .into_iter()
                .map(|e| format!("new item {}: {e}", n + 1)),
        );
        p.changes.push(json!({ "subject": "new_item", "index": n, "title": rules::stored_text(item.title.as_deref()), "placement": item.placement }));
    }
    Ok(p)
}

/// The changed fields of an item as the item writer takes them; `placement` is applied by a move.
fn payload_of(values: &BTreeMap<String, Value>) -> ItemPayload {
    let text = |f: &str| {
        values
            .get(f)
            .map(|v| v.as_str().unwrap_or_default().to_string())
    };
    ItemPayload {
        title: text("title"),
        description: text("description"),
        expected_outcome: text("expected_outcome"),
        phase: text("phase"),
        tags: values
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        organization_pids: values
            .get("organization_pids")
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        details: values
            .get("details")
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        placement: None,
    }
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/roadmap/publish/preview", post(preview))
        .add("/plans/{pid}/roadmap/publish", post(publish))
}

/// `POST /api/plans/{pid}/roadmap/publish/preview`
#[debug_handler]
async fn preview(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Json(set): Json<ChangeSet>,
) -> Result<Response> {
    let plan_row = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = pr::roadmap_for_write(&mut tx, plan_row.pid).await?;
    let list = pr::live_items(&mut tx, r.id).await?;
    let p = plan(&mut tx, &r, &list, &set).await?;
    // Nothing is written; the transaction only held the roadmap still while it was read.
    drop(tx);
    format::json(
        json!({ "changes": p.changes, "conflicts": p.conflicts, "problems": p.problems,
                         "would_publish": p.conflicts.is_empty() && p.problems.is_empty() }),
    )
}

/// `POST /api/plans/{pid}/roadmap/publish`
#[debug_handler]
async fn publish(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(set): Json<ChangeSet>,
) -> Result<Response> {
    let plan_row = super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let r = pr::roadmap_for_write(&mut tx, plan_row.pid).await?;
    let list = pr::live_items(&mut tx, r.id).await?;
    let p = plan(&mut tx, &r, &list, &set).await?;
    if !p.conflicts.is_empty() {
        use axum::response::IntoResponse;
        // Nothing is written: the transaction is dropped, and the answer names every conflict.
        return Ok((
            axum::http::StatusCode::CONFLICT,
            axum::Json(json!({
                "error": "conflict",
                "description": "someone else changed what this change set changes; nothing was published",
                "conflicts": p.conflicts,
            })),
        )
            .into_response());
    }
    if !p.problems.is_empty() {
        return Err(unprocessable(&p.problems.join("; ")));
    }
    if !p.roadmap.is_empty() {
        let get = |f: &str| p.roadmap.get(f).cloned();
        sqlx::query(
            "UPDATE public_roadmaps SET
                 title = COALESCE($2, title),
                 vision = CASE WHEN $3 THEN $4 ELSE vision END,
                 service_description = CASE WHEN $5 THEN $6 ELSE service_description END,
                 category_pid = CASE WHEN $7 THEN $8 ELSE category_pid END,
                 published = COALESCE($9, published),
                 updated_at = now()
             WHERE id = $1",
        )
        .bind(r.id)
        .bind(get("title").and_then(|v| v.as_str().map(str::to_string)))
        .bind(p.roadmap.contains_key("vision"))
        .bind(get("vision").and_then(|v| v.as_str().map(str::to_string)))
        .bind(p.roadmap.contains_key("service_description"))
        .bind(get("service_description").and_then(|v| v.as_str().map(str::to_string)))
        .bind(p.roadmap.contains_key("category_pid"))
        .bind(
            get("category_pid")
                .and_then(|v| serde_json::from_value::<Option<Uuid>>(v).ok().flatten()),
        )
        .bind(get("published").and_then(|v| v.as_bool()))
        .execute(tx.conn())
        .await?;
        AuditModel::record_on(
            tx.conn(),
            plan_row.pid,
            "public_roadmap_changed",
            caller.actor(),
            Some(json!({ "roadmap": r.pid, "fields": p.roadmap.keys().collect::<Vec<_>>(), "by": "publish" })),
        )
        .await
        .map_err(super::audit_failed)?;
    }
    for (i, values) in &p.items {
        let mut row = pr::update_item(&mut tx, i, &payload_of(values)).await?;
        if let Some(placement) = values.get("placement") {
            row = pr::place_item(&mut tx, &row, placement.as_str(), None).await?;
        }
        AuditModel::record_on(
            tx.conn(),
            plan_row.pid,
            "public_roadmap_item_changed",
            caller.actor(),
            Some(json!({ "item": row.pid, "fields": values.keys().collect::<Vec<_>>(), "by": "publish" })),
        )
        .await
        .map_err(super::audit_failed)?;
    }
    for item in &set.new {
        let row = pr::insert_item(&mut tx, r.id, item).await?;
        AuditModel::record_on(
            tx.conn(),
            plan_row.pid,
            "public_roadmap_item_added",
            caller.actor(),
            Some(json!({ "item": row.pid, "placement": row.placement, "by": "publish" })),
        )
        .await
        .map_err(super::audit_failed)?;
    }
    let row: roadmaps::Model = sqlx::query_as("SELECT * FROM public_roadmaps WHERE id = $1")
        .bind(r.id)
        .fetch_one(tx.conn())
        .await?;
    let list = pr::live_items(&mut tx, r.id).await?;
    tx.commit().await?;
    format::json(json!({ "applied": p.changes, "roadmap": pr::roadmap_view(&row, &list) }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_field_conflicts_only_when_someone_else_changed_it_to_something_else() {
        let (a, b, c) = (json!("Now"), json!("Next"), json!("Later"));
        assert!(!conflicts(&a, &b, &a), "unchanged since read");
        assert!(!conflicts(&a, &b, &b), "already changed to the same value");
        assert!(conflicts(&a, &b, &c), "changed to something else");
    }

    #[test]
    fn values_are_compared_as_they_are_stored() {
        assert_eq!(normal("title", &json!("  Now  ")), Ok(json!("Now")));
        assert_eq!(normal("description", &json!("   ")), Ok(Value::Null));
        assert_eq!(
            normal("tags", &json!([" Patients "])),
            Ok(json!(["Patients"]))
        );
        let o = uuid::Uuid::new_v4();
        assert_eq!(normal("organization_pids", &json!([o, o])), Ok(json!([o])));
        assert_eq!(normal("placement", &json!("now")), Ok(json!("now")));
        assert!(normal("placement", &json!("soon")).is_err());
        assert!(normal("published", &json!("yes")).is_err());
        assert!(normal("slug", &json!("x")).is_err());
        assert!(normal("title", &json!(42)).is_err());
    }
}

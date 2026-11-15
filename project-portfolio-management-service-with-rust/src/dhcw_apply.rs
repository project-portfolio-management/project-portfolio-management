//! The apply step after the DHCW dry run (RE-5): write the spreadsheet's roadmaps onto existing `Product` plans.
//!
//! The spreadsheet names no plan, so a person says which plan each tab belongs to (`Sheet` to plan id). A tab with no
//! plan named is left alone. The apply makes nothing else: no plan, no category, no organization. It never changes a
//! roadmap that exists: a plan that already has one is skipped. Every roadmap it makes is **unpublished**, whatever
//! the sheet's `Enabled` says (the report gives it), so a person reads it before the public reader can serve it.
//!
//! Each roadmap is written in one transaction with its items and translations, and audited once on its plan
//! (`public_roadmap_imported`, actor `system:dhcw-import`) with the sheet, the rows and the item ids. A roadmap that
//! fails writes nothing; the others still land. An item whose fields this service refuses is skipped with the
//! reasons, and a Welsh cell that does not have its source's shape is not kept, as the dry run reports.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use uuid::Uuid;

use crate::controllers::public_roadmaps::{self as roadmap, ItemPayload};
use crate::dhcw_import::{ParsedItem, ParsedRoadmap};
use crate::models::_entities::public_roadmap_items as items;
use crate::models::_entities::public_roadmaps as roadmaps;
use crate::models::audit_logs::Model as AuditModel;
use crate::prelude::*;
use crate::public_roadmap as rules;

/// The actor the import's audit rows name.
pub const ACTOR: &str = "system:dhcw-import";

/// The plans file (`Sheet,Plan`, a header row then one row per tab) as `Sheet` to plan id.
///
/// # Errors
///
/// When the file cannot be read as CSV, has no `Sheet` and `Plan` columns, names a sheet twice, or gives a plan id
/// that is not a UUID.
pub fn plans_file(text: &str) -> std::result::Result<BTreeMap<String, Uuid>, String> {
    let all = crate::tracker_import::read_csv(text).map_err(|e| format!("{e:?}"))?;
    let mut it = all.into_iter();
    let header: Vec<String> = it
        .next()
        .ok_or("the plans file is empty")?
        .into_iter()
        .map(|h| h.trim().to_string())
        .collect();
    let col = |name: &str| {
        header
            .iter()
            .position(|h| h == name)
            .ok_or(format!("the plans file has no `{name}` column"))
    };
    let (s, p) = (col("Sheet")?, col("Plan")?);
    let mut out = BTreeMap::new();
    for (i, row) in it.enumerate() {
        let sheet = row.get(s).map_or("", |c| c.trim());
        let plan = row.get(p).map_or("", |c| c.trim());
        if sheet.is_empty() && plan.is_empty() {
            continue;
        }
        let pid = Uuid::parse_str(plan)
            .map_err(|_| format!("row {}: `{plan}` is not a plan id", i + 2))?;
        if out.insert(sheet.to_string(), pid).is_some() {
            return Err(format!("row {}: the sheet `{sheet}` is named twice", i + 2));
        }
    }
    Ok(out)
}

/// Why a roadmap was not made, as a report entry.
fn skipped(r: &ParsedRoadmap, reason: impl Into<String>) -> Value {
    json!({ "row": r.row, "sheet": r.sheet, "outcome": "skipped", "reason": reason.into() })
}

/// Write every parsed roadmap whose sheet `plans` names; the report accounts for every one.
///
/// # Errors
///
/// When the database cannot be reached; a roadmap that cannot be written is reported, not an error.
pub async fn apply(
    ctx: &AppContext,
    parsed: &[ParsedRoadmap],
    plans: &BTreeMap<String, Uuid>,
) -> Result<Value> {
    let mut out = Vec::new();
    let (mut made, mut items_made, mut items_skipped) = (0, 0, 0);
    for r in parsed {
        let Some(plan) = plans.get(&r.sheet) else {
            out.push(skipped(r, "the plans file names no plan for this sheet"));
            continue;
        };
        let entry = match one(ctx, r, *plan).await {
            Ok(v) => v,
            Err(Error::CustomError(_, detail)) => skipped(r, detail.detail.unwrap_or_default()),
            Err(e) => return Err(e),
        };
        if entry["outcome"] == "made" {
            made += 1;
            items_made += entry["items_made"].as_u64().unwrap_or(0);
            items_skipped += entry["items_skipped"].as_array().map_or(0, Vec::len);
        }
        out.push(entry);
    }
    let unknown: Vec<&String> = plans
        .keys()
        .filter(|s| !parsed.iter().any(|r| &r.sheet == *s))
        .collect();
    Ok(json!({
        "written": true,
        "totals": { "roadmaps": parsed.len(), "roadmaps_made": made, "roadmaps_skipped": parsed.len() - made,
                    "items_made": items_made, "items_skipped": items_skipped },
        "roadmaps": out,
        "sheets_in_the_plans_file_not_read": unknown,
    }))
}

/// A refusal of one roadmap, rolled back.
fn refuse(reason: impl Into<String>) -> Error {
    Error::CustomError(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        axa::error::ErrorDetail::new("skipped", reason.into()),
    )
}

/// Refuse a roadmap the import may not make: the plan is not a live `Product`, already has a roadmap, or the
/// roadmap's own fields or slug are refused.
async fn checked(tx: &mut crate::db::Tx, r: &ParsedRoadmap, plan_pid: Uuid) -> Result<()> {
    let kind: Option<Option<String>> = sqlx::query_scalar(
        "SELECT kind FROM plans WHERE pid = $1 AND active AND deleted_at IS NULL FOR UPDATE",
    )
    .bind(plan_pid)
    .fetch_optional(tx.conn())
    .await?;
    match kind {
        None => return Err(refuse(format!("{plan_pid} is not a live plan"))),
        Some(k) if k.as_deref() != Some("Product") => {
            return Err(refuse(format!(
                "{plan_pid} is not a Product plan; a public product roadmap belongs to one"
            )));
        }
        Some(_) => {}
    }
    if roadmap::find_roadmap(tx, plan_pid, true).await?.is_some() {
        return Err(refuse(
            "the plan already has a roadmap; the import never changes one",
        ));
    }
    let mut problems = Vec::new();
    rules::title_problems(Some(&r.title), &mut problems);
    rules::text_problems("vision", r.vision.as_deref(), &mut problems);
    rules::text_problems(
        "service_description",
        r.service_description.as_deref(),
        &mut problems,
    );
    if !rules::is_slug(&r.slug) {
        problems.push(format!("the slug `{}` is not one", r.slug));
    }
    if !problems.is_empty() {
        return Err(refuse(problems.join("; ")));
    }
    let held: Option<i32> = sqlx::query_scalar(
        "SELECT 1 FROM public_roadmaps WHERE slug = $1
         UNION ALL SELECT 1 FROM public_roadmap_slugs WHERE slug = $1 LIMIT 1",
    )
    .bind(&r.slug)
    .fetch_optional(tx.conn())
    .await?;
    if held.is_some() {
        return Err(refuse(format!(
            "another roadmap has or had the slug `{}`",
            r.slug
        )));
    }
    Ok(())
}

/// Write one roadmap, its items and its translations, in one transaction.
async fn one(ctx: &AppContext, r: &ParsedRoadmap, plan_pid: Uuid) -> Result<Value> {
    let items_in = r.items.as_ref().map_err(|e| refuse(e.clone()))?;
    let mut tx = crate::db::begin(ctx).await?;
    checked(&mut tx, r, plan_pid).await?;
    let category: Option<Uuid> = match &r.category {
        Some(name) => {
            sqlx::query_scalar(
                "SELECT pid FROM roadmap_categories WHERE active AND lower(name) = lower($1)",
            )
            .bind(name)
            .fetch_optional(tx.conn())
            .await?
        }
        None => None,
    };
    let mut row: roadmaps::Model = sqlx::query_as(
        "INSERT INTO public_roadmaps
             (pid, plan_pid, title, slug, vision, service_description, category_pid, published, reviewed_on)
         VALUES ($1, $2, $3, $4, $5, $6, $7, false, $8) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan_pid)
    .bind(rules::stored_text(Some(&r.title)))
    .bind(&r.slug)
    .bind(rules::stored_text(r.vision.as_deref()))
    .bind(rules::stored_text(r.service_description.as_deref()))
    .bind(category)
    .bind(r.reviewed_on.filter(|d| *d <= crate::tz::deployment_today()))
    .fetch_one(tx.conn())
    .await?;
    let mut dropped = Vec::new();
    let t = welsh(
        &r.welsh,
        &rules::ROADMAP_FIELDS,
        &roadmap::roadmap_sources(&row),
        &mut dropped,
        None,
    );
    if let Some(t) = t {
        row = sqlx::query_as(
            "UPDATE public_roadmaps SET translations = $2 WHERE id = $1 RETURNING *",
        )
        .bind(row.id)
        .bind(t)
        .fetch_one(tx.conn())
        .await?;
    }
    let mut made_items = Vec::new();
    let mut skipped_items: Vec<Value> = r
        .items_skipped
        .iter()
        .map(|(n, why)| json!({ "row": n, "reason": why }))
        .collect();
    let mut kept_ids = 0;
    for it in items_in {
        match item(&mut tx, row.id, it, &mut dropped).await? {
            Ok((i, kept)) => {
                kept_ids += usize::from(kept);
                made_items.push(i.pid);
            }
            Err(why) => skipped_items.push(json!({ "row": it.row, "reason": why })),
        }
    }
    AuditModel::record_on(
        tx.conn(),
        plan_pid,
        "public_roadmap_imported",
        Some(ACTOR),
        Some(
            json!({ "roadmap": row.pid, "sheet": r.sheet, "index_row": r.row, "items": made_items,
                     "items_skipped": skipped_items.len(), "published": false }),
        ),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    Ok(json!({
        "row": r.row, "sheet": r.sheet, "outcome": "made", "plan_pid": plan_pid, "roadmap_pid": row.pid,
        "slug": row.slug, "published": false, "enabled_in_sheet": r.enabled,
        "category": r.category.as_ref().map(|c| json!({ "name": c, "linked": category.is_some() })),
        "reviewed_on": row.reviewed_on, "items_made": made_items.len(), "item_ids_kept": kept_ids,
        "items_skipped": skipped_items, "welsh_not_kept": dropped,
    }))
}

/// The Welsh translations that have their source's shape, as stored; each that does not is named in `dropped`.
fn welsh(
    cells: &BTreeMap<&'static str, Value>,
    fields: &[&str],
    sources: &BTreeMap<&str, Value>,
    dropped: &mut Vec<String>,
    row: Option<usize>,
) -> Option<Value> {
    let mut mine = serde_json::Map::new();
    for (field, value) in cells {
        let source = sources.get(field).cloned().unwrap_or(Value::Null);
        match rules::translation_problem(fields, field, &source, value) {
            None => {
                mine.insert(
                    (*field).to_string(),
                    rules::stored_translation(&source, value),
                );
            }
            Some(p) => dropped.push(match row {
                Some(n) => format!("item row {n}: {field}: {p}"),
                None => format!("{field}: {p}"),
            }),
        }
    }
    (!mine.is_empty()).then(|| json!({ "cy": mine }))
}

/// Write one item; `Err` with the reasons when this service refuses its fields. The flag says its id was kept.
async fn item(
    tx: &mut crate::db::Tx,
    roadmap_id: i64,
    it: &ParsedItem,
    dropped: &mut Vec<String>,
) -> Result<std::result::Result<(items::Model, bool), String>> {
    let codes: Vec<String> = it.codes.iter().map(|c| c.to_lowercase()).collect();
    let orgs: Vec<Uuid> = sqlx::query_scalar(
        "SELECT pid FROM organizations WHERE active AND lower(code) = ANY($1) ORDER BY array_position($1, lower(code))",
    )
    .bind(&codes)
    .fetch_all(tx.conn())
    .await?;
    let b = ItemPayload {
        title: Some(it.title.clone()),
        description: it.description.clone(),
        expected_outcome: it.expected_outcome.clone(),
        phase: it.phase.clone(),
        tags: Some(it.tags.clone()),
        organization_pids: Some(orgs),
        details: Some(it.details.clone()),
        placement: it.placement.map(str::to_string),
    };
    let problems = roadmap::item_problems(tx, b.title.as_deref(), &b).await?;
    if !problems.is_empty() {
        return Ok(Err(problems.join("; ")));
    }
    let mut row = roadmap::insert_item_keeping(tx, roadmap_id, &b, it.public_id.as_deref()).await?;
    let kept = it.public_id.as_deref() == Some(row.public_id.as_str());
    if let Some(t) = welsh(
        &it.welsh,
        &rules::ITEM_FIELDS,
        &roadmap::item_sources(&row),
        dropped,
        Some(it.row),
    ) {
        row = sqlx::query_as(
            "UPDATE public_roadmap_items SET translations = $2 WHERE id = $1 RETURNING *",
        )
        .bind(row.id)
        .bind(t)
        .fetch_one(tx.conn())
        .await?;
    }
    Ok(Ok((row, kept)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_plans_file_names_one_plan_per_sheet() {
        let a = "3f2c6b7e-0000-4000-8000-00000000000a";
        let m = plans_file(&format!("Sheet,Plan\nProductA,{a}\n,\n")).unwrap();
        assert_eq!(m.len(), 1);
        assert_eq!(m["ProductA"].to_string(), a);
        assert!(
            plans_file("Sheet\nProductA\n")
                .unwrap_err()
                .contains("`Plan`")
        );
        assert!(
            plans_file("Sheet,Plan\nProductA,nope\n")
                .unwrap_err()
                .contains("row 2")
        );
        assert!(
            plans_file(&format!("Sheet,Plan\nA,{a}\nA,{a}\n"))
                .unwrap_err()
                .contains("twice")
        );
        assert!(plans_file("").is_err());
    }
}

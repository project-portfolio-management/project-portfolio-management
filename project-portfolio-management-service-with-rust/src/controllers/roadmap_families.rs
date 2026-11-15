//! Product families' colours (RE-5, RE-7): each family's editors choose its colour.
//!
//! A product family is a plan that holds product plans (RE-7), so its name is the plan's name and its members are
//! its `Product` children. What it adds is its own: a colour, any `#rrggbb` (`#RRGGBB` is stored lower case), and its
//! name in other languages. Nothing chooses the colour for them, and nothing refuses one for its contrast: every view
//! says how it reads under white and under black text (WCAG 2 contrast ratio, AA at 4.5:1 for body text), so an editor
//! can see a poor choice and the public site can pick the text colour that reads.
//!
//! * `GET /api/plans/{pid}/roadmap-family`: that plan as a family, its colour (or `null`) and its products;
//! * `PUT /api/plans/{pid}/roadmap-family`: set its colour and translations; `colour: null` removes both;
//! * `GET /api/roadmap-families`: every live plan that holds a live `Product` plan or has a colour, that the caller may
//!   read, by name, paged.
//!
//! The first two are beneath `/api/plans/{pid}`, so the plan guard decides who may read and write, and the `roadmap`
//! module turns them off with the roadmap. Every write is audited.

use std::collections::BTreeMap;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::roadmap_families as fams;
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/roadmap-family", get(show_family))
        .add("/plans/{pid}/roadmap-family", put(put_family))
        .add("/roadmap-families", get(list_families))
}

/// The colour as stored, lower case, when it is `#rrggbb` (any case, spaces around it ignored).
#[must_use]
pub fn colour(s: &str) -> Option<String> {
    let s = s.trim();
    let hex = s.strip_prefix('#')?;
    (hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()))
        .then(|| format!("#{}", hex.to_lowercase()))
}

/// The WCAG 2 relative luminance of a stored `#rrggbb` colour.
fn luminance(colour: &str) -> f64 {
    let channel = |i: usize| {
        let v = f64::from(u8::from_str_radix(&colour[i..i + 2], 16).unwrap_or(0)) / 255.0;
        if v <= 0.040_45 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.0722_f64.mul_add(
        channel(5),
        0.2126_f64.mul_add(channel(1), 0.7152 * channel(3)),
    )
}

/// The contrast ratio of two luminances, 1 to 21.
fn ratio(a: f64, b: f64) -> f64 {
    let (hi, lo) = if a > b { (a, b) } else { (b, a) };
    (hi + 0.05) / (lo + 0.05)
}

/// How a stored colour reads as a background under white and under black text: each ratio to two decimals, whether
/// it meets AA for body text (4.5:1), and the text colour that reads better.
#[must_use]
pub fn contrast(colour: &str) -> Value {
    let l = luminance(colour);
    let (white, black) = (ratio(l, 1.0), ratio(l, 0.0));
    let round = |x: f64| (x * 100.0).round() / 100.0;
    json!({
        "with_white_text": round(white), "with_black_text": round(black),
        "white_text_meets_aa": white >= 4.5, "black_text_meets_aa": black >= 4.5,
        "better_text": if white >= black { "white" } else { "black" },
    })
}

/// JSON view of a plan as a family: its colour (with how it reads) and translations, or `null` for both.
fn family_view(plan_pid: Uuid, name: &str, row: Option<&fams::Model>, products: i64) -> Value {
    json!({
        "plan_pid": plan_pid, "name": name,
        "colour": row.map(|f| &f.colour),
        "contrast": row.map(|f| contrast(&f.colour)),
        "translations": row.map_or_else(|| json!({}), |f| f.translations.clone()),
        "products": products,
    })
}

/// A family's row, if it has a colour.
async fn find_family(
    tx: &mut crate::db::Tx,
    plan_pid: Uuid,
    lock: bool,
) -> Result<Option<fams::Model>> {
    let sql = if lock {
        "SELECT * FROM roadmap_families WHERE plan_pid = $1 FOR UPDATE"
    } else {
        "SELECT * FROM roadmap_families WHERE plan_pid = $1"
    };
    Ok(sqlx::query_as(sql)
        .bind(plan_pid)
        .fetch_optional(tx.conn())
        .await?)
}

/// How many live `Product` plans a family holds that the caller may read.
async fn products_of(tx: &mut crate::db::Tx, plan_pid: Uuid, hidden: &[Uuid]) -> Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT count(*) FROM plans WHERE parent_pid = $1 AND kind = 'Product' AND active AND deleted_at IS NULL
           AND NOT (pid = ANY($2))",
    )
    .bind(plan_pid)
    .bind(hidden)
    .fetch_one(tx.conn())
    .await?)
}

/// `GET /api/plans/{pid}/roadmap-family`
#[debug_handler]
async fn show_family(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row = find_family(&mut tx, plan.pid, false).await?;
    let products = products_of(&mut tx, plan.pid, &hidden.plan_pids()).await?;
    tx.commit().await?;
    format::json(family_view(plan.pid, &plan.name, row.as_ref(), products))
}

/// `PUT /api/plans/{pid}/roadmap-family` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct FamilyPayload {
    /// `#rrggbb`, any case; `null` removes the colour and the translations.
    colour: Option<String>,
    /// The family's name in other languages: language code to text, 1 to 100 characters each; absent keeps them.
    #[serde(default)]
    translations: Option<BTreeMap<String, String>>,
}

/// `PUT /api/plans/{pid}/roadmap-family`
#[debug_handler]
async fn put_family(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<FamilyPayload>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let old = find_family(&mut tx, plan.pid, true).await?;
    let row = match &b.colour {
        None => {
            if b.translations.as_ref().is_some_and(|t| !t.is_empty()) {
                return Err(unprocessable(
                    "a family with no colour keeps no translations; give a colour",
                ));
            }
            sqlx::query("DELETE FROM roadmap_families WHERE plan_pid = $1")
                .bind(plan.pid)
                .execute(tx.conn())
                .await?;
            None
        }
        Some(c) => {
            let Some(c) = colour(c) else {
                return Err(unprocessable(
                    "colour is #rrggbb: a hash and six hexadecimal digits",
                ));
            };
            let kept: BTreeMap<String, String> = old
                .as_ref()
                .and_then(|f| serde_json::from_value(f.translations.clone()).ok())
                .unwrap_or_default();
            let translations = b.translations.as_ref().unwrap_or(&kept);
            let mut problems = crate::issues::label_problems(translations);
            if translations
                .values()
                .any(|v| v.trim().chars().count() > super::roadmap_categories::NAME_MAX)
            {
                problems.push(format!(
                    "each translation is at most {} characters",
                    super::roadmap_categories::NAME_MAX
                ));
            }
            if !problems.is_empty() {
                return Err(unprocessable(&problems.join("; ")));
            }
            let stored: BTreeMap<&String, &str> =
                translations.iter().map(|(k, v)| (k, v.trim())).collect();
            Some(
                sqlx::query_as::<_, fams::Model>(
                    "INSERT INTO roadmap_families (plan_pid, colour, translations) VALUES ($1, $2, $3)
                     ON CONFLICT (plan_pid) DO UPDATE SET colour = $2, translations = $3, updated_at = now()
                     RETURNING *",
                )
                .bind(plan.pid)
                .bind(&c)
                .bind(json!(stored))
                .fetch_one(tx.conn())
                .await?,
            )
        }
    };
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "roadmap_family_changed",
        caller.actor(),
        Some(json!({ "from": old.as_ref().map(|f| &f.colour), "to": row.as_ref().map(|f| &f.colour) })),
    )
    .await
    .map_err(super::audit_failed)?;
    let products = products_of(&mut tx, plan.pid, &hidden.plan_pids()).await?;
    tx.commit().await?;
    format::json(family_view(plan.pid, &plan.name, row.as_ref(), products))
}

/// A family as the list reads it.
#[derive(sqlx::FromRow)]
struct ListedFamily {
    pid: Uuid,
    name: String,
    colour: Option<String>,
    translations: Option<Value>,
    products: i64,
}

/// `GET /api/roadmap-families` — every live plan that holds a live `Product` plan or has a colour, that the caller may
/// read, by name ignoring case, paged; a product the caller may not read is not counted.
#[debug_handler]
async fn list_families(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    cursor: super::pagination::Cursor,
) -> Result<Response> {
    let page = cursor.page(200)?;
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let rows: Vec<ListedFamily> = sqlx::query_as(
        "SELECT p.pid, p.name, f.colour, f.translations,
                (SELECT count(*) FROM plans c WHERE c.parent_pid = p.pid AND c.kind = 'Product' AND c.active
                   AND c.deleted_at IS NULL AND NOT (c.pid = ANY($1))) AS products
         FROM plans p LEFT JOIN roadmap_families f ON f.plan_pid = p.pid
         WHERE p.active AND p.deleted_at IS NULL AND NOT (p.pid = ANY($1))
           AND (f.id IS NOT NULL OR EXISTS (SELECT 1 FROM plans c WHERE c.parent_pid = p.pid AND c.kind = 'Product'
                AND c.active AND c.deleted_at IS NULL AND NOT (c.pid = ANY($1))))
         ORDER BY lower(p.name), p.id
         LIMIT $2 OFFSET $3",
    )
    .bind(hidden.plan_pids())
    .bind(i64::try_from(page.fetch()).unwrap_or(i64::MAX))
    .bind(i64::try_from(page.offset()).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let list: Vec<Value> = rows
        .iter()
        .map(|f| {
            json!({
                "plan_pid": f.pid, "name": f.name, "colour": f.colour,
                "contrast": f.colour.as_deref().map(contrast),
                "translations": f.translations.clone().unwrap_or_else(|| json!({})),
                "products": f.products,
            })
        })
        .collect();
    Ok(page.respond(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_six_digit_colour_is_kept_in_lower_case() {
        assert_eq!(colour("#325083").as_deref(), Some("#325083"));
        assert_eq!(colour(" #00A3E0 ").as_deref(), Some("#00a3e0"));
        for bad in ["325083", "#fff", "#12345g", "#1234567", "red", "", "#"] {
            assert_eq!(colour(bad), None, "{bad}");
        }
    }

    #[test]
    fn contrast_follows_wcag() {
        let black = contrast("#000000");
        assert_eq!(black["with_white_text"], 21.0);
        assert_eq!(black["with_black_text"], 1.0);
        assert_eq!(black["better_text"], "white");
        let white = contrast("#ffffff");
        assert_eq!(white["with_black_text"], 21.0);
        assert_eq!(white["white_text_meets_aa"], false);
        assert_eq!(white["better_text"], "black");
        // DHCW's navy from its example spreadsheet: white text reads.
        let navy = contrast("#325083");
        assert_eq!(navy["white_text_meets_aa"], true);
        assert_eq!(navy["with_white_text"], 8.04);
        // Mid grey #777777 is about 4.48:1 with white: just short of AA.
        let grey = contrast("#777777");
        assert_eq!(grey["white_text_meets_aa"], false);
        assert_eq!(grey["with_white_text"], 4.48);
    }
}

//! The public product roadmap (RE-1, RE-6, RE-8): the pure rules. DB-free, unit-tested.
//!
//! A product plan may have one public product roadmap: a short, outcome-worded statement of what the product is
//! doing now, next and later, written for readers outside the organization. It is not a schedule: an item is never
//! derived from a task or a milestone.
//!
//! * **Placement.** An item is in one of three horizons (`now`, `next`, `later`) or one of three sections around them
//!   (`recently_delivered`, `delivered_this_year`, `not_doing`), or in none: an unplaced item is kept for its editors
//!   and never shown publicly. [`PLACEMENTS`] is in page order.
//! * **Details** are a list nested at most three levels: the first line is at the top, and each line is at most one
//!   level deeper than the line above.
//! * **Text is stored as typed, trimmed** at its ends, never parsed as a number, date or formula.
//! * **Translations know when they are out of date (RE-6).** A translation records the hash of the source text it
//!   was made from. When the source changes, the translation is kept and reads as `needs_review`; saving it again
//!   clears that. The service never writes one language from another.
//! * **Public ids (RE-8)** are eight lower-case letters and digits starting with a letter, never reused.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Every placement, in page order.
pub const PLACEMENTS: [&str; 6] = [
    "recently_delivered",
    "now",
    "next",
    "later",
    "delivered_this_year",
    "not_doing",
];

/// The placements that are horizons; the others are sections.
pub const HORIZONS: [&str; 3] = ["now", "next", "later"];

/// Longest title, in characters.
pub const TITLE_MAX: usize = 200;
/// Longest description, outcome, vision or service description, in characters.
pub const TEXT_MAX: usize = 2000;
/// Longest phase, in characters.
pub const PHASE_MAX: usize = 100;
/// Most tags on an item.
pub const TAGS_MAX: usize = 20;
/// Longest tag, in characters.
pub const TAG_MAX: usize = 50;
/// Most detail lines on an item.
pub const DETAILS_MAX: usize = 50;
/// Longest detail line, in characters.
pub const DETAIL_MAX: usize = 300;
/// Deepest detail level (0 is the top).
pub const DETAIL_LEVEL_MAX: u8 = 2;
/// Most organizations an item is for.
pub const ORGANIZATIONS_MAX: usize = 50;
/// Longest slug.
pub const SLUG_MAX: usize = 64;

/// The fields of a roadmap that can be translated.
pub const ROADMAP_FIELDS: [&str; 3] = ["title", "vision", "service_description"];
/// The fields of an item that can be translated.
pub const ITEM_FIELDS: [&str; 6] = [
    "title",
    "description",
    "expected_outcome",
    "phase",
    "tags",
    "details",
];

/// One line of an item's details.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Detail {
    /// The line, 1 to 300 characters, with no line break.
    pub text: String,
    /// 0 is the top level; at most one deeper than the line above, at most 2.
    pub level: u8,
}

/// Whether `p` is a placement.
#[must_use]
pub fn is_placement(p: &str) -> bool {
    PLACEMENTS.contains(&p)
}

/// The page-order index of a placement; unplaced items sort last.
#[must_use]
pub fn placement_rank(p: Option<&str>) -> usize {
    p.and_then(|p| PLACEMENTS.iter().position(|x| *x == p))
        .unwrap_or(PLACEMENTS.len())
}

/// Text as stored: trimmed at its ends; `None` when nothing is left.
#[must_use]
pub fn stored_text(s: Option<&str>) -> Option<String> {
    s.map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_string)
}

fn length(name: &str, value: Option<&str>, max: usize, required: bool, out: &mut Vec<String>) {
    let n = value.map_or(0, |v| v.trim().chars().count());
    if required && n == 0 {
        out.push(format!("{name} is required"));
    } else if n > max {
        out.push(format!("{name} is at most {max} characters"));
    }
}

/// What is wrong with a title, which is required.
pub fn title_problems(title: Option<&str>, out: &mut Vec<String>) {
    length("title", title, TITLE_MAX, true, out);
}

/// What is wrong with an optional long text field.
pub fn text_problems(name: &str, value: Option<&str>, out: &mut Vec<String>) {
    length(name, value, TEXT_MAX, false, out);
}

/// What is wrong with a phase.
pub fn phase_problems(value: Option<&str>, out: &mut Vec<String>) {
    length("phase", value, PHASE_MAX, false, out);
}

/// What is wrong with a list of tags: each 1 to 50 characters with no comma or line break, no repeats ignoring
/// case, at most 20.
pub fn tag_problems(tags: &[String], out: &mut Vec<String>) {
    if tags.len() > TAGS_MAX {
        out.push(format!("at most {TAGS_MAX} tags"));
    }
    let mut seen = BTreeSet::new();
    for t in tags {
        let t = t.trim();
        if t.is_empty() || t.chars().count() > TAG_MAX || t.contains([',', '\n', '\r']) {
            out.push(format!(
                "a tag is 1 to {TAG_MAX} characters, with no comma or line break"
            ));
        } else if !seen.insert(t.to_lowercase()) {
            out.push(format!("the tag `{t}` is repeated"));
        }
    }
}

/// What is wrong with a list of details: see the module rules.
pub fn detail_problems(details: &[Detail], out: &mut Vec<String>) {
    if details.len() > DETAILS_MAX {
        out.push(format!("at most {DETAILS_MAX} detail lines"));
    }
    let mut above: Option<u8> = None;
    for (i, d) in details.iter().enumerate() {
        let n = i + 1;
        let t = d.text.trim();
        if t.is_empty() || t.chars().count() > DETAIL_MAX || t.contains(['\n', '\r']) {
            out.push(format!(
                "detail {n} is 1 to {DETAIL_MAX} characters, with no line break"
            ));
        }
        let deepest = above.map_or(0, |a| (a + 1).min(DETAIL_LEVEL_MAX));
        if d.level > deepest {
            out.push(format!(
                "detail {n} is at level {} but can be at most {deepest}",
                d.level
            ));
        }
        above = Some(d.level);
    }
}

/// Details as stored: each line trimmed.
#[must_use]
pub fn stored_details(details: &[Detail]) -> Vec<Detail> {
    details
        .iter()
        .map(|d| Detail {
            text: d.text.trim().to_string(),
            level: d.level,
        })
        .collect()
}

/// Tags as stored: each trimmed, in the order given.
#[must_use]
pub fn stored_tags(tags: &[String]) -> Vec<String> {
    tags.iter().map(|t| t.trim().to_string()).collect()
}

/// Whether `slug` is a roadmap's public address: lower-case letters, digits and single hyphens, starting and ending
/// with a letter or digit, at most 64 characters.
#[must_use]
pub fn is_slug(slug: &str) -> bool {
    let b = slug.as_bytes();
    !b.is_empty()
        && b.len() <= SLUG_MAX
        && b.iter()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
        && b.first().is_some_and(u8::is_ascii_alphanumeric)
        && b.last().is_some_and(u8::is_ascii_alphanumeric)
        && !slug.contains("--")
}

/// A slug made from a title: lower case, runs of anything but letters and digits become one hyphen. `None` when the
/// title has no ASCII letter or digit.
#[must_use]
pub fn slug_from(title: &str) -> Option<String> {
    let mut out = String::new();
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    let out: String = out.trim_end_matches('-').chars().take(SLUG_MAX).collect();
    let out = out.trim_end_matches('-').to_string();
    is_slug(&out).then_some(out)
}

/// Whether `id` is an item's public id: eight lower-case letters and digits, starting with a letter.
#[must_use]
pub fn is_public_id(id: &str) -> bool {
    id.len() == 8
        && id.starts_with(|c: char| c.is_ascii_lowercase())
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

/// A public id from random bytes: a letter, then seven letters or digits. The caller supplies the randomness, so a
/// test can be deterministic; the service passes the bytes of a v4 UUID.
#[must_use]
pub fn public_id_from(bytes: &[u8; 16]) -> String {
    const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const ALNUM: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut id = String::with_capacity(8);
    id.push(char::from(LETTERS[usize::from(bytes[0]) % LETTERS.len()]));
    for b in &bytes[1..8] {
        id.push(char::from(ALNUM[usize::from(*b) % ALNUM.len()]));
    }
    id
}

/// The hash a translation records of its source: SHA-256 of the source value's canonical JSON, in hex.
#[must_use]
pub fn source_hash(source: &Value) -> String {
    use std::fmt::Write;
    let text = serde_json::to_string(source).unwrap_or_default();
    Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

/// What is wrong with a translation of one field: the field must be translatable, the source must exist, and the
/// value must have the source's shape (text for text, a list of the same length for tags, the same lines and
/// levels for details).
#[must_use]
pub fn translation_problem(
    fields: &[&str],
    field: &str,
    source: &Value,
    value: &Value,
) -> Option<String> {
    if !fields.contains(&field) {
        return Some(format!(
            "`{field}` cannot be translated; one of {}",
            fields.join(", ")
        ));
    }
    if source.is_null() {
        return Some(format!("`{field}` has no text to translate"));
    }
    match (source, value) {
        (Value::String(_), Value::String(t)) => {
            let max = match field {
                "title" => TITLE_MAX,
                "phase" => PHASE_MAX,
                _ => TEXT_MAX,
            };
            let n = t.trim().chars().count();
            (n == 0 || n > max)
                .then(|| format!("the `{field}` translation is 1 to {max} characters"))
        }
        (Value::Array(src), Value::Array(t)) if field == "tags" => {
            let tags: Vec<String> = t
                .iter()
                .map(|v| v.as_str().unwrap_or_default().to_string())
                .collect();
            let mut out = Vec::new();
            tag_problems(&tags, &mut out);
            if t.len() != src.len() || t.iter().any(|v| !v.is_string()) {
                out.insert(
                    0,
                    format!(
                        "the tags translation has {} tags, as the source does",
                        src.len()
                    ),
                );
            }
            out.into_iter().next()
        }
        (Value::Array(src), Value::Array(t)) if field == "details" => {
            let parsed: Option<Vec<Detail>> = serde_json::from_value(Value::Array(t.clone())).ok();
            let source: Option<Vec<Detail>> =
                serde_json::from_value(Value::Array(src.clone())).ok();
            match (parsed, source) {
                (Some(p), Some(s))
                    if p.len() == s.len() && p.iter().zip(&s).all(|(a, b)| a.level == b.level) =>
                {
                    let mut out = Vec::new();
                    detail_problems(&p, &mut out);
                    out.into_iter().next()
                }
                _ => Some(
                    "the details translation has the source's lines, at the same levels"
                        .to_string(),
                ),
            }
        }
        _ => Some(format!("the `{field}` translation has the source's shape")),
    }
}

/// A translation as stored: the value (trimmed) and the hash of the source it was made from.
#[must_use]
pub fn stored_translation(source: &Value, value: &Value) -> Value {
    let value = match value {
        Value::String(s) => Value::String(s.trim().to_string()),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| match v {
                    Value::String(s) => Value::String(s.trim().to_string()),
                    Value::Object(o) => {
                        let mut o = o.clone();
                        if let Some(Value::String(t)) = o.get("text") {
                            let t = t.trim().to_string();
                            o.insert("text".to_string(), Value::String(t));
                        }
                        Value::Object(o)
                    }
                    other => other.clone(),
                })
                .collect(),
        ),
        other => other.clone(),
    };
    json!({ "value": value, "source": source_hash(source) })
}

/// The translations of a record as a reader sees them: for each language and field, the value and whether it
/// `needs_review` because the source has changed since. `sources` gives each field's current source value.
#[must_use]
pub fn shown_translations(stored: &Value, sources: &BTreeMap<&str, Value>) -> Value {
    let mut out = serde_json::Map::new();
    let Some(langs) = stored.as_object() else {
        return Value::Object(out);
    };
    for (lang, fields) in langs {
        let mut shown = serde_json::Map::new();
        for (field, t) in fields.as_object().into_iter().flatten() {
            let Some(source) = sources.get(field.as_str()) else {
                continue;
            };
            if source.is_null() {
                continue;
            }
            let current =
                t.get("source").and_then(Value::as_str) == Some(source_hash(source).as_str());
            shown.insert(
                field.clone(),
                json!({ "value": t.get("value").cloned().unwrap_or(Value::Null), "needs_review": !current }),
            );
        }
        if !shown.is_empty() {
            out.insert(lang.clone(), Value::Object(shown));
        }
    }
    Value::Object(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(text: &str, level: u8) -> Detail {
        Detail {
            text: text.to_string(),
            level,
        }
    }

    #[test]
    fn placements_are_in_page_order_and_unplaced_sorts_last() {
        assert!(PLACEMENTS.iter().all(|p| is_placement(p)));
        assert!(!is_placement("soon"));
        assert!(placement_rank(Some("recently_delivered")) < placement_rank(Some("now")));
        assert!(placement_rank(Some("later")) < placement_rank(Some("not_doing")));
        assert_eq!(placement_rank(None), PLACEMENTS.len());
        assert!(HORIZONS.iter().all(|h| is_placement(h)));
    }

    #[test]
    fn details_start_at_the_top_and_go_at_most_one_level_deeper() {
        let mut out = Vec::new();
        detail_problems(
            &[
                d("Follow-up", 0),
                d("Dermatology", 1),
                d("Adults first", 2),
                d("Back up", 0),
            ],
            &mut out,
        );
        assert!(out.is_empty(), "{out:?}");
        let mut out = Vec::new();
        detail_problems(&[d("Starts deep", 1)], &mut out);
        assert_eq!(out.len(), 1);
        let mut out = Vec::new();
        detail_problems(&[d("Top", 0), d("Skips a level", 2)], &mut out);
        assert_eq!(out.len(), 1);
        let mut out = Vec::new();
        detail_problems(
            &[d("a", 0), d("b", 1), d("c", 2), d("too deep", 3)],
            &mut out,
        );
        assert_eq!(out.len(), 1);
        let mut out = Vec::new();
        detail_problems(&[d("two\nlines", 0), d(" ", 0)], &mut out);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn tags_are_short_have_no_comma_and_do_not_repeat_ignoring_case() {
        let mut out = Vec::new();
        tag_problems(&["Patients".into(), "Video appointments".into()], &mut out);
        assert!(out.is_empty());
        let mut out = Vec::new();
        tag_problems(
            &[
                "Patients".into(),
                "patients".into(),
                "a, b".into(),
                " ".into(),
            ],
            &mut out,
        );
        assert_eq!(out.len(), 3, "{out:?}");
    }

    #[test]
    fn text_is_trimmed_and_a_title_is_required() {
        assert_eq!(stored_text(Some("  Now  ")), Some("Now".to_string()));
        assert_eq!(stored_text(Some("   ")), None);
        assert_eq!(
            stored_text(Some("=SUM(A1)")),
            Some("=SUM(A1)".to_string()),
            "never parsed"
        );
        let mut out = Vec::new();
        title_problems(Some("  "), &mut out);
        title_problems(Some(&"x".repeat(201)), &mut out);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn slugs_are_plain_and_made_from_titles() {
        assert!(is_slug("video-appointments"));
        for bad in ["", "-a", "a-", "a--b", "Caps", "a b", &"a".repeat(65)] {
            assert!(!is_slug(bad), "{bad}");
        }
        assert_eq!(
            slug_from("Video Appointments: Phase 2!").as_deref(),
            Some("video-appointments-phase-2")
        );
        assert_eq!(
            slug_from("Apwyntiadau fideo").as_deref(),
            Some("apwyntiadau-fideo")
        );
        assert_eq!(slug_from("—"), None);
    }

    #[test]
    fn public_ids_are_eight_characters_starting_with_a_letter() {
        let id = public_id_from(&[255; 16]);
        assert!(is_public_id(&id), "{id}");
        assert!(is_public_id("k7q2xwbn"));
        for bad in ["7kq2xwbn", "k7q2xwb", "K7Q2XWBN", "k7q2xwb!"] {
            assert!(!is_public_id(bad), "{bad}");
        }
        let mut seen = BTreeSet::new();
        for _ in 0..10_000 {
            let id = public_id_from(uuid::Uuid::new_v4().as_bytes());
            assert!(is_public_id(&id), "{id}");
            seen.insert(id);
        }
        // 26 × 36^7 ≈ 2 × 10^12 ids: 10,000 draws collide with odds of about 1 in 40,000. A collision is refused by
        // the database's unique index, and the service draws again.
        assert!(seen.len() >= 9_999);
    }

    #[test]
    fn a_translation_needs_review_once_its_source_changes() {
        let source = json!("Video appointments for follow-up");
        let stored = json!({ "cy": { "title": stored_translation(&source, &json!(" Apwyntiadau fideo ")) } });
        let now = BTreeMap::from([("title", source.clone())]);
        let shown = shown_translations(&stored, &now);
        assert_eq!(shown["cy"]["title"]["value"], "Apwyntiadau fideo");
        assert_eq!(shown["cy"]["title"]["needs_review"], false);
        let changed = BTreeMap::from([("title", json!("Video appointments for every clinic"))]);
        let shown = shown_translations(&stored, &changed);
        assert_eq!(shown["cy"]["title"]["value"], "Apwyntiadau fideo", "kept");
        assert_eq!(shown["cy"]["title"]["needs_review"], true);
        // A field whose source was cleared shows no translation.
        let cleared = BTreeMap::from([("title", Value::Null)]);
        assert_eq!(shown_translations(&stored, &cleared), json!({}));
    }

    #[test]
    fn a_translation_has_the_shape_of_its_source() {
        let f = &ITEM_FIELDS;
        assert!(translation_problem(f, "title", &json!("Now"), &json!("Nawr")).is_none());
        assert!(translation_problem(f, "title", &json!("Now"), &json!(" ")).is_some());
        assert!(translation_problem(f, "slug", &json!("x"), &json!("y")).is_some());
        assert!(
            translation_problem(f, "phase", &Value::Null, &json!("Cam 2")).is_some(),
            "no source"
        );
        assert!(
            translation_problem(
                f,
                "tags",
                &json!(["Patients", "Carers"]),
                &json!(["Cleifion", "Gofalwyr"])
            )
            .is_none()
        );
        assert!(
            translation_problem(
                f,
                "tags",
                &json!(["Patients", "Carers"]),
                &json!(["Cleifion"])
            )
            .is_some()
        );
        let src =
            json!([{ "text": "Follow-up", "level": 0 }, { "text": "Dermatology", "level": 1 }]);
        let ok = json!([{ "text": "Dilynol", "level": 0 }, { "text": "Dermatoleg", "level": 1 }]);
        let moved =
            json!([{ "text": "Dilynol", "level": 0 }, { "text": "Dermatoleg", "level": 0 }]);
        assert!(translation_problem(f, "details", &src, &ok).is_none());
        assert!(translation_problem(f, "details", &src, &moved).is_some());
        assert!(translation_problem(f, "title", &json!("Now"), &json!(["Nawr"])).is_some());
    }
}

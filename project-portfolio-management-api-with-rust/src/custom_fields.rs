//! Typed custom fields (RM-1): the pure rules. DB-free, unit-tested.
//!
//! An administrator declares a field once (`key`, `label`, a type, what it
//! applies to, optionally which plan `kinds`, whether it is `required`, and
//! for lists the allowed `options`); values are then set per plan or task and
//! **validated against the definition**. Values live in their own table, so
//! they can never become part of the matcher payload.
//!
//! Types: `text`, `date` (`YYYY-MM-DD`), `bool`, `integer`, `list` (one of
//! `options`), `multi_list` (a distinct subset of `options`).

use chrono::NaiveDate;
use serde_json::Value;

/// Field types.
pub const TYPES: &[&str] = &["text", "date", "bool", "integer", "list", "multi_list"];
/// What a field may apply to.
pub const APPLIES_TO: &[&str] = &["plan", "task"];
/// Cap on a text value.
pub const MAX_TEXT: usize = 1_000;
/// Cap on options of a list field.
pub const MAX_OPTIONS: usize = 100;
/// Cap on a key / label / option.
pub const MAX_SHORT: usize = 100;

/// A field definition, as validation sees it.
#[derive(Debug, Clone)]
pub struct Def {
    /// Machine key.
    pub key: String,
    /// One of [`TYPES`].
    pub field_type: String,
    /// Allowed options (`list` / `multi_list`).
    pub options: Vec<String>,
}

/// A key is `[a-z][a-z0-9_]*`, at most [`MAX_SHORT`] characters.
#[must_use]
pub fn valid_key(key: &str) -> bool {
    let mut chars = key.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && key.len() <= MAX_SHORT
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Every problem with a new definition (not just the first).
#[must_use]
pub fn def_problems(
    key: &str,
    label: &str,
    field_type: &str,
    applies_to: &str,
    kinds: &[String],
    options: &[String],
) -> Vec<String> {
    let mut out = Vec::new();
    if !valid_key(key) {
        out.push(
            "key must be lowercase letters, digits and underscores, starting with a letter"
                .to_string(),
        );
    }
    if label.trim().is_empty() || label.chars().count() > MAX_SHORT {
        out.push(format!(
            "label is required (at most {MAX_SHORT} characters)"
        ));
    }
    if !TYPES.contains(&field_type) {
        out.push(format!("unknown type {field_type:?} (allowed: {TYPES:?})"));
    }
    if !APPLIES_TO.contains(&applies_to) {
        out.push(format!(
            "unknown applies_to {applies_to:?} (allowed: {APPLIES_TO:?})"
        ));
    }
    if !kinds.is_empty() && applies_to != "plan" {
        out.push("kinds only applies to plan fields".to_string());
    }
    let is_list = matches!(field_type, "list" | "multi_list");
    if is_list && options.is_empty() {
        out.push("a list field needs at least one option".to_string());
    }
    if !is_list && !options.is_empty() {
        out.push("options are only for list fields".to_string());
    }
    if options.len() > MAX_OPTIONS
        || options
            .iter()
            .any(|o| o.trim().is_empty() || o.chars().count() > MAX_SHORT)
    {
        out.push(format!(
            "options: at most {MAX_OPTIONS}, non-blank, each ≤ {MAX_SHORT} characters"
        ));
    }
    let mut seen = std::collections::HashSet::new();
    if options.iter().any(|o| !seen.insert(o.as_str())) {
        out.push("options must be distinct".to_string());
    }
    out
}

/// Validate (and normalise) one value against its definition.
///
/// # Errors
///
/// A message naming the field and what is wrong.
pub fn validate_value(def: &Def, value: &Value) -> Result<Value, String> {
    let k = &def.key;
    match def.field_type.as_str() {
        "text" => match value.as_str() {
            Some(s) if s.chars().count() <= MAX_TEXT => Ok(Value::String(s.to_string())),
            Some(_) => Err(format!("{k}: exceeds {MAX_TEXT} characters")),
            None => Err(format!("{k}: expected text")),
        },
        "date" => value
            .as_str()
            .and_then(|s| {
                NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .ok()
                    .map(|d| d.to_string())
            })
            .map(Value::String)
            .ok_or_else(|| format!("{k}: expected a date as YYYY-MM-DD")),
        "bool" => value
            .as_bool()
            .map(Value::Bool)
            .ok_or_else(|| format!("{k}: expected true or false")),
        "integer" => value
            .as_i64()
            .map(|n| Value::Number(n.into()))
            .ok_or_else(|| format!("{k}: expected a whole number")),
        "list" => match value.as_str() {
            Some(s) if def.options.iter().any(|o| o == s) => Ok(Value::String(s.to_string())),
            _ => Err(format!("{k}: must be one of {:?}", def.options)),
        },
        "multi_list" => {
            let Some(items) = value.as_array() else {
                return Err(format!("{k}: expected a list of {:?}", def.options));
            };
            let mut out: Vec<String> = Vec::new();
            for item in items {
                match item.as_str() {
                    Some(s) if def.options.iter().any(|o| o == s) => {
                        if !out.iter().any(|x| x == s) {
                            out.push(s.to_string());
                        }
                    }
                    _ => return Err(format!("{k}: every entry must be one of {:?}", def.options)),
                }
            }
            out.sort();
            Ok(serde_json::json!(out))
        }
        other => Err(format!("{k}: unknown field type {other:?}")),
    }
}

/// Whether a definition scoped to `kinds` applies to a plan of `plan_kind`.
/// An empty `kinds` applies to every kind (and to a plan with none).
#[must_use]
pub fn kind_applies(kinds: &[String], plan_kind: Option<&str>) -> bool {
    kinds.is_empty() || plan_kind.is_some_and(|k| kinds.iter().any(|x| x.eq_ignore_ascii_case(k)))
}

/// Whether a stored value matches a wanted one given as a query string.
/// `multi_list` matches when the stored list **contains** it; every other
/// type is equality after the same normalisation a write gets.
#[must_use]
pub fn matches(def: &Def, stored: &Value, wanted: &str) -> bool {
    let parsed = match def.field_type.as_str() {
        "bool" => match wanted {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => return false,
        },
        "integer" => match wanted.parse::<i64>() {
            Ok(n) => Value::Number(n.into()),
            Err(_) => return false,
        },
        "multi_list" => {
            return stored
                .as_array()
                .is_some_and(|a| a.iter().any(|v| v.as_str() == Some(wanted)));
        }
        _ => Value::String(wanted.to_string()),
    };
    *stored == parsed
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn def(t: &str, options: &[&str]) -> Def {
        Def {
            key: "f".to_string(),
            field_type: t.to_string(),
            options: options.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    #[test]
    fn keys_are_lowercase_identifiers() {
        assert!(valid_key("cost_centre"));
        assert!(valid_key("a1"));
        for bad in ["", "Cost", "1a", "a-b", "a b", &"x".repeat(MAX_SHORT + 1)] {
            assert!(!valid_key(bad), "{bad:?}");
        }
    }

    #[test]
    fn definition_problems_are_all_reported() {
        assert!(def_problems("k", "K", "list", "plan", &[], &["a".into()]).is_empty());
        let p = def_problems("Bad Key", " ", "nope", "nope", &[], &[]);
        assert!(p.len() >= 4, "{p:?}");
        assert!(
            !def_problems("k", "K", "list", "plan", &[], &[]).is_empty(),
            "list needs options"
        );
        assert!(
            !def_problems("k", "K", "text", "plan", &[], &["a".into()]).is_empty(),
            "options only for lists"
        );
        assert!(
            !def_problems("k", "K", "text", "task", &["Project".into()], &[]).is_empty(),
            "kinds only for plans"
        );
        assert!(
            !def_problems("k", "K", "list", "plan", &[], &["a".into(), "a".into()]).is_empty(),
            "distinct"
        );
    }

    #[test]
    fn each_type_accepts_the_right_shape_and_refuses_the_wrong_one() {
        assert_eq!(
            validate_value(&def("text", &[]), &json!("hi")).unwrap(),
            json!("hi")
        );
        assert!(validate_value(&def("text", &[]), &json!(5)).is_err());
        assert_eq!(
            validate_value(&def("date", &[]), &json!("2026-10-03")).unwrap(),
            json!("2026-10-03")
        );
        for bad in [json!("03/10/2026"), json!("2026-02-30"), json!(1)] {
            assert!(validate_value(&def("date", &[]), &bad).is_err(), "{bad}");
        }
        assert!(validate_value(&def("bool", &[]), &json!("true")).is_err());
        assert_eq!(
            validate_value(&def("integer", &[]), &json!(7)).unwrap(),
            json!(7)
        );
        assert!(validate_value(&def("integer", &[]), &json!(1.5)).is_err());
        assert!(validate_value(&def("integer", &[]), &json!("7")).is_err());
    }

    #[test]
    fn lists_are_closed_over_their_options() {
        let l = def("list", &["a", "b"]);
        assert!(validate_value(&l, &json!("a")).is_ok());
        assert!(validate_value(&l, &json!("c")).is_err());
        let m = def("multi_list", &["a", "b", "c"]);
        assert_eq!(
            validate_value(&m, &json!(["c", "a", "a"])).unwrap(),
            json!(["a", "c"]),
            "deduped and ordered"
        );
        assert!(validate_value(&m, &json!(["a", "z"])).is_err());
        assert!(validate_value(&m, &json!("a")).is_err());
    }

    #[test]
    fn kind_scoping_applies_only_to_listed_kinds() {
        let kinds = vec!["Project".to_string()];
        assert!(kind_applies(&kinds, Some("project")), "case-insensitive");
        assert!(!kind_applies(&kinds, Some("Program")));
        assert!(
            !kind_applies(&kinds, None),
            "a scoped field never applies to an unkinded plan"
        );
        assert!(kind_applies(&[], None), "unscoped applies to everything");
    }

    #[test]
    fn matching_normalises_like_a_write_and_multi_list_means_contains() {
        assert!(matches(&def("integer", &[]), &json!(7), "7"));
        assert!(!matches(&def("integer", &[]), &json!(7), "seven"));
        assert!(matches(&def("bool", &[]), &json!(true), "true"));
        assert!(matches(
            &def("multi_list", &["a", "b"]),
            &json!(["a", "b"]),
            "b"
        ));
        assert!(!matches(
            &def("multi_list", &["a", "b"]),
            &json!(["a"]),
            "b"
        ));
        assert!(matches(&def("text", &[]), &json!("x"), "x"));
    }
}

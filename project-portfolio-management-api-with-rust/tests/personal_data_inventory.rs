//! The personal-data inventory stays true to the schema (HD-11).
//!
//! `governance/personal-data-inventory.csv` names every column that can hold
//! personal data, with its class, purpose and retention class. This test reads
//! the entity files (`src/models/_entities/*.rs`) and fails when
//!
//! - a column that **can** hold personal data (by the rule below) is not in the
//!   inventory: a migration added one and nobody decided what it is;
//! - the inventory names a table or column that no longer exists;
//! - a row has an unknown class or retention class, or leaves a field blank;
//! - a retention class used by the inventory is missing from the retention
//!   schedule (`governance/retention-schedule.md`).
//!
//! The rule is deliberately broad (it errs towards listing); a column judged not
//! personal is still listed, with class `opaque`, so a change is a decision.
//! No database: this runs in a normal `cargo test`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Substrings that mark a column name as person-related.
const SUBSTR: &[&str] = &[
    "actor",
    "author",
    "owner",
    "lead",
    "person",
    "email",
    "assignee",
    "recipient",
    "reviewer",
    "approver",
    "requester",
    "facilitator",
    "sponsor",
    "respondent",
    "member",
    "holder",
    "user",
];
/// Exact column names that carry free text or documents.
const EXACT: &[&str] = &[
    "note",
    "notes",
    "comment",
    "comments",
    "body",
    "message",
    "reason",
    "rationale",
    "description",
    "summary",
    "snapshot",
    "payload",
    "data",
    "conditions",
    "filename",
    "label",
    "url",
    "token_hash",
    "previous_body",
    "change_summary",
    "sub",
    "destination",
    "settings",
    "title",
    "pitch",
    "tags",
    "votes",
    "mitigation",
    "value",
    "detail",
    "last_error",
    "error",
    "filters",
    "source_line",
    "candidate",
    "matches",
    "params",
    "name",
    "reference",
];
/// Name endings that mark a reference to a person or a free-text field.
const SUFFIX: &[&str] = &[
    "_ref",
    "_by",
    "_sub",
    "_note",
    "_notes",
    "_reason",
    "_comment",
    "_summary",
    "_body",
    "_rationale",
];

const CLASSES: &[&str] = &[
    "identifier",
    "free_text",
    "structured",
    "credential",
    "opaque",
];
const RETENTION_CLASSES: &[&str] = &[
    "AUD", "OPS", "PPL", "COM", "DOC", "GEN", "CRD", "EVT", "DRV",
];

fn is_candidate(column: &str) -> bool {
    EXACT.contains(&column)
        || SUFFIX.iter().any(|s| column.ends_with(s))
        || SUBSTR.iter().any(|s| column.contains(s))
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits inside the repository")
        .to_path_buf()
}

/// `table -> columns` of every entity's `Model`.
fn schema() -> BTreeMap<String, Vec<String>> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/models/_entities");
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(&dir).expect("entities directory") {
        let path = entry.expect("entry").path();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        if path.extension().is_none_or(|e| e != "rs") || name == "mod" || name == "prelude" {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("read entity");
        let Some(start) = text.find("pub struct Model {") else {
            continue;
        };
        let body = &text[start..];
        let end = body.find("\n}").expect("end of the Model struct");
        let columns: Vec<String> = body[..end]
            .lines()
            .filter_map(|l| {
                let l = l.trim();
                let rest = l.strip_prefix("pub ")?;
                let (col, _) = rest.split_once(':')?;
                Some(col.trim().to_string())
            })
            .collect();
        out.insert(name, columns);
    }
    out
}

/// Minimal CSV: quoted fields with `""` escapes, `\n` rows.
fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            ('"', true) => quoted = false,
            ('"', false) if field.is_empty() => quoted = true,
            (',', false) => row.push(std::mem::take(&mut field)),
            ('\n', false) => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            _ => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

struct Inventory {
    /// `(table, column)` -> `(class, retention class)`
    rows: BTreeMap<(String, String), (String, String)>,
    blanks: Vec<String>,
}

fn inventory() -> Inventory {
    let path = root().join("governance/personal-data-inventory.csv");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is missing: {e}", path.display()));
    let mut rows = BTreeMap::new();
    let mut blanks = Vec::new();
    let mut table = parse_csv(&text).into_iter();
    let header = table.next().expect("a header row");
    assert_eq!(
        header,
        [
            "table",
            "column",
            "class",
            "data_subjects",
            "purpose",
            "retention_class",
            "notes"
        ],
        "the inventory header changed"
    );
    for r in table {
        assert_eq!(r.len(), 7, "malformed inventory row: {r:?}");
        if r.iter().any(|f| f.trim().is_empty()) {
            blanks.push(format!("{}.{}", r[0], r[1]));
        }
        let key = (r[0].clone(), r[1].clone());
        assert!(
            rows.insert(key, (r[2].clone(), r[5].clone())).is_none(),
            "duplicate inventory row {}.{}",
            r[0],
            r[1]
        );
    }
    Inventory { rows, blanks }
}

#[test]
fn every_column_that_can_hold_personal_data_is_in_the_inventory() {
    let inv = inventory();
    let mut missing = Vec::new();
    for (table, columns) in schema() {
        for column in columns.iter().filter(|c| is_candidate(c)) {
            if !inv.rows.contains_key(&(table.clone(), column.clone())) {
                missing.push(format!("{table}.{column}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "columns that can hold personal data are not in governance/personal-data-inventory.csv \
         (add a row with its class, purpose and retention class, and review the DPIA): {missing:?}"
    );
}

#[test]
fn the_inventory_names_only_real_columns() {
    let inv = inventory();
    let schema = schema();
    let stale: Vec<String> = inv
        .rows
        .keys()
        .filter(|(t, c)| !schema.get(t).is_some_and(|cols| cols.contains(c)))
        .map(|(t, c)| format!("{t}.{c}"))
        .collect();
    assert!(
        stale.is_empty(),
        "inventory rows name tables or columns that no longer exist: {stale:?}"
    );
}

#[test]
fn every_row_is_complete_and_uses_known_classes() {
    let inv = inventory();
    assert!(inv.blanks.is_empty(), "blank fields in: {:?}", inv.blanks);
    for ((t, c), (class, retention)) in &inv.rows {
        assert!(
            CLASSES.contains(&class.as_str()),
            "{t}.{c}: class {class:?}"
        );
        assert!(
            RETENTION_CLASSES.contains(&retention.as_str()),
            "{t}.{c}: retention class {retention:?}"
        );
    }
}

#[test]
fn the_retention_schedule_defines_every_class_the_inventory_uses() {
    let inv = inventory();
    let text = std::fs::read_to_string(root().join("governance/retention-schedule.md"))
        .expect("governance/retention-schedule.md exists");
    let used: BTreeSet<&str> = inv.rows.values().map(|(_, r)| r.as_str()).collect();
    for class in used {
        assert!(
            text.contains(&format!("| `{class}` |")),
            "retention class {class} is used by the inventory but not defined in the retention schedule"
        );
    }
}

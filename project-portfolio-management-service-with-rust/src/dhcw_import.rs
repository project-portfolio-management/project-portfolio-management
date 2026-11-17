//! A dry run of moving the DHCW Roadmap Editor's spreadsheet into public product roadmaps (RE-5 part b). DB-free.
//!
//! The input is the spreadsheet as CSV exports: the roadmap index (its first tab, read by its row-1 headers) and one
//! file per roadmap tab, named by the index's `Sheet` column. The format is the editor's own, as its README and
//! ADR-009 and ADR-011 describe it. The output says, row by row, what would be made and what would not, and why, so
//! every row is accounted for: `rows = mapped + skipped`. **Nothing is written** by the dry run; [`parse`] gives the
//! same rows as values for the apply step (`crate::dhcw_apply`), which writes them.
//!
//! What maps: `Sheet` (the slug, lower case), `Enabled` (exactly `Yes` is published), `Title`, `Vision`, `Service
//! description`, `Category` (matched by name to a roadmap category, or reported as new), `Last updated` (as
//! `reviewed_on`, from `YYYY-MM-DD` or `DD/MM/YYYY`), and the `(cy)` columns as Welsh translations; for an item,
//! `Title`, `Description`, `Outcome`, `Horizon`, `Phase`, `Labels`, `Organisations` (matched by code), `Details`
//! (a hyphen per level), `Item ID` (kept when it is a valid public id), and their `(cy)` columns.
//!
//! What does not: `Family`, `Status label` and `Colour` (a family is the parent plan, and its colour is chosen there), and
//! `Product Lead`, which names people, and this service holds people as references, not names.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use crate::public_roadmap::{self as rules, Detail};

/// The spreadsheet's `Horizon` values and the placement each becomes.
pub const HORIZONS: [(&str, &str); 6] = [
    ("recently delivered", "recently_delivered"),
    ("now", "now"),
    ("next", "next"),
    ("later", "later"),
    ("delivered this year", "delivered_this_year"),
    ("not doing", "not_doing"),
];

/// The index columns that have no field here, with why.
pub const NOT_MAPPED: [(&str, &str); 6] = [
    (
        "Family",
        "a family would be the product's parent plan; not made by the dry run",
    ),
    ("Family (cy)", "as Family"),
    ("Status label", "no field"),
    ("Status label (cy)", "no field"),
    (
        "Colour",
        "the family's colour, chosen on the family plan (PUT /api/plans/{pid}/roadmap-family); not set by the import",
    ),
    (
        "Product Lead",
        "names people; this service holds people as references, not names",
    ),
];

/// The spreadsheet, as CSV text.
#[derive(Debug, Default)]
pub struct Input {
    /// The roadmap index (the first tab).
    pub index: String,
    /// Each roadmap tab, by the name the index's `Sheet` column gives it.
    pub tabs: BTreeMap<String, String>,
    /// The organization codes this deployment has, in any case.
    pub known_codes: Vec<String>,
    /// The roadmap category names this deployment has.
    pub known_categories: Vec<String>,
}

/// A row as header to cell, trimmed.
type Row = BTreeMap<String, String>;

fn rows(text: &str) -> Result<Vec<Row>, String> {
    let all = crate::tracker_import::read_csv(text).map_err(|e| format!("{e:?}"))?;
    let mut it = all.into_iter();
    let header: Vec<String> = it
        .next()
        .ok_or_else(|| "the file is empty".to_string())?
        .into_iter()
        .map(|h| h.trim().to_string())
        .collect();
    Ok(it
        .map(|cells| {
            header
                .iter()
                .cloned()
                .zip(cells.into_iter().map(|c| c.trim().to_string()))
                .collect()
        })
        .collect())
}

fn cell<'a>(row: &'a Row, name: &str) -> Option<&'a str> {
    row.get(name).map(String::as_str).filter(|s| !s.is_empty())
}

/// A `Last updated` cell as a date: `YYYY-MM-DD` or `DD/MM/YYYY`.
#[must_use]
pub fn date(s: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(s, "%d/%m/%Y"))
        .ok()
}

/// A `Details` cell as lines: each leading hyphen is one level deeper, then an optional space.
#[must_use]
pub fn details(s: &str) -> Vec<Detail> {
    s.lines()
        .map(str::trim_end)
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let depth = l.chars().take_while(|c| *c == '-').count();
            Detail {
                text: l[depth..].trim().to_string(),
                level: u8::try_from(depth).unwrap_or(u8::MAX),
            }
        })
        .collect()
}

/// A list cell (`Labels`, `Organisations`): split on commas, trimmed, blanks dropped.
#[must_use]
pub fn list(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_string)
        .collect()
}

/// The placement for a `Horizon` cell, ignoring case; `None` for blank or unknown (unplaced).
#[must_use]
pub fn placement(h: &str) -> Option<&'static str> {
    let h = h.trim().to_lowercase();
    HORIZONS.iter().find(|(k, _)| *k == h).map(|(_, p)| *p)
}

/// One item row, as the dry run reads it.
fn item(n: usize, row: &Row, known: &BTreeSet<String>, seen_ids: &mut BTreeSet<String>) -> Value {
    let Some(title) = cell(row, "Title") else {
        return json!({ "row": n, "outcome": "skipped", "reason": "no title, so the editor and the public site skip it too" });
    };
    let mut problems = Vec::new();
    rules::title_problems(Some(title), &mut problems);
    for (col, field) in [
        ("Description", "description"),
        ("Outcome", "expected_outcome"),
    ] {
        rules::text_problems(field, cell(row, col), &mut problems);
    }
    rules::phase_problems(cell(row, "Phase"), &mut problems);
    let tags = cell(row, "Labels").map_or_default(list);
    rules::tag_problems(&tags, &mut problems);
    let lines = cell(row, "Details").map_or_default(details);
    rules::detail_problems(&lines, &mut problems);
    let horizon = cell(row, "Horizon").unwrap_or_default();
    let placed = placement(horizon);
    let codes = cell(row, "Organisations").map_or_default(list);
    let (matched, unknown): (Vec<String>, Vec<String>) = codes
        .into_iter()
        .partition(|c| known.contains(&c.to_lowercase()));
    let public_id = cell(row, "Item ID")
        .filter(|id| rules::is_public_id(id) && !seen_ids.contains(*id))
        .map(str::to_string);
    if let Some(id) = &public_id {
        seen_ids.insert(id.clone());
    }
    let mut welsh = Vec::new();
    for (col, field, source, value) in [
        (
            "Title (cy)",
            "title",
            json!(title),
            cell(row, "Title (cy)").map(|v| json!(v)),
        ),
        (
            "Description (cy)",
            "description",
            json!(cell(row, "Description")),
            cell(row, "Description (cy)").map(|v| json!(v)),
        ),
        (
            "Outcome (cy)",
            "expected_outcome",
            json!(cell(row, "Outcome")),
            cell(row, "Outcome (cy)").map(|v| json!(v)),
        ),
        (
            "Phase (cy)",
            "phase",
            json!(cell(row, "Phase")),
            cell(row, "Phase (cy)").map(|v| json!(v)),
        ),
        (
            "Labels (cy)",
            "tags",
            if tags.is_empty() {
                Value::Null
            } else {
                json!(tags)
            },
            cell(row, "Labels (cy)").map(|v| json!(list(v))),
        ),
        (
            "Details (cy)",
            "details",
            if lines.is_empty() {
                Value::Null
            } else {
                json!(lines)
            },
            cell(row, "Details (cy)").map(|v| json!(details(v))),
        ),
    ] {
        let Some(value) = value else { continue };
        match rules::translation_problem(&rules::ITEM_FIELDS, field, &source, &value) {
            None => welsh.push(field),
            Some(p) => problems.push(format!("{col} not kept: {p}")),
        }
    }
    json!({
        "row": n, "outcome": "mapped", "title": title, "placement": placed,
        "horizon_as_written": if placed.is_none() && !horizon.is_empty() { json!(horizon) } else { Value::Null },
        "tags": tags.len(), "details": lines.len(), "organizations": matched, "unknown_organization_codes": unknown,
        "public_id": public_id.map_or_else(|| json!("new"), |id| json!(id)), "welsh": welsh, "problems": problems,
    })
}

/// What one roadmap tab's rows would become: the report, how many rows, how many mapped.
fn tab_report(
    sheet: &str,
    tab: Option<&String>,
    known: &BTreeSet<String>,
    seen_ids: &mut BTreeSet<String>,
) -> (Value, usize, usize) {
    match tab.map(|t| rows(t)) {
        None => (
            json!({ "error": format!("no file for the tab `{sheet}`") }),
            0,
            0,
        ),
        Some(Err(e)) => (
            json!({ "error": format!("the tab `{sheet}` cannot be read: {e}") }),
            0,
            0,
        ),
        Some(Ok(item_rowset)) => {
            let list: Vec<Value> = item_rowset
                .iter()
                .enumerate()
                .map(|(j, r)| item(j + 2, r, known, seen_ids))
                .collect();
            let m = list.iter().filter(|r| r["outcome"] == "mapped").count();
            let unplaced = list
                .iter()
                .filter(|r| r["outcome"] == "mapped" && r["placement"].is_null())
                .count();
            let n = list.len();
            (
                json!({ "rows": n, "mapped": m, "skipped": n - m, "unplaced": unplaced, "each": list }),
                n,
                m,
            )
        }
    }
}

/// The dry run: what each row of the index and of each roadmap tab would become.
#[must_use]
pub fn dry_run(input: &Input) -> Value {
    let index = match rows(&input.index) {
        Ok(r) => r,
        Err(e) => return json!({ "error": format!("the roadmap index cannot be read: {e}") }),
    };
    let known: BTreeSet<String> = input.known_codes.iter().map(|c| c.to_lowercase()).collect();
    let categories: BTreeSet<String> = input
        .known_categories
        .iter()
        .map(|c| c.to_lowercase())
        .collect();
    let mut seen_ids = BTreeSet::new();
    let mut slugs = BTreeSet::new();
    let mut read_tabs = BTreeSet::new();
    let (mut mapped, mut skipped, mut item_rows, mut items_mapped, mut items_skipped) =
        (0, 0, 0, 0, 0);
    let mut out = Vec::new();
    for (i, row) in index.iter().enumerate() {
        let n = i + 2; // row 1 is the header
        let Some(sheet) = cell(row, "Sheet") else {
            skipped += 1;
            out.push(
                json!({ "row": n, "outcome": "skipped", "reason": "no Sheet, so no roadmap tab" }),
            );
            continue;
        };
        if sheet.eq_ignore_ascii_case("Template") {
            skipped += 1;
            out.push(json!({ "row": n, "sheet": sheet, "outcome": "skipped", "reason": "the Template tab is never read" }));
            continue;
        }
        let slug = rules::slug_from(sheet);
        let title = cell(row, "Title").unwrap_or(sheet);
        let mut problems = Vec::new();
        rules::title_problems(Some(title), &mut problems);
        rules::text_problems("vision", cell(row, "Vision"), &mut problems);
        rules::text_problems(
            "service_description",
            cell(row, "Service description"),
            &mut problems,
        );
        let Some(slug) = slug.filter(|s| slugs.insert(s.clone())) else {
            skipped += 1;
            out.push(json!({ "row": n, "sheet": sheet, "outcome": "skipped", "reason": "no slug can be made from Sheet, or another row has it" }));
            continue;
        };
        let reviewed = cell(row, "Last updated");
        let reviewed_on = reviewed.and_then(date);
        if reviewed.is_some() && reviewed_on.is_none() {
            problems.push(format!(
                "Last updated `{}` is not a date; not kept",
                reviewed.unwrap_or_default()
            ));
        }
        let category = cell(row, "Category")
            .map(|c| json!({ "name": c, "known": categories.contains(&c.to_lowercase()) }));
        let welsh: Vec<&str> = [
            ("Title (cy)", "title"),
            ("Service description (cy)", "service_description"),
            ("Vision (cy)", "vision"),
        ]
        .into_iter()
        .filter(|(col, _)| cell(row, col).is_some())
        .map(|(_, f)| f)
        .collect();
        let not_mapped: BTreeMap<&str, &str> = NOT_MAPPED
            .iter()
            .filter(|(col, _)| cell(row, col).is_some())
            .map(|(col, why)| (*col, *why))
            .collect();
        let tab = input.tabs.get(sheet);
        read_tabs.insert(sheet.to_string());
        let (item_report, rows_in_tab, mapped_in_tab) =
            tab_report(sheet, tab, &known, &mut seen_ids);
        item_rows += rows_in_tab;
        items_mapped += mapped_in_tab;
        items_skipped += rows_in_tab - mapped_in_tab;
        mapped += 1;
        out.push(json!({
            "row": n, "sheet": sheet, "outcome": "mapped", "slug": slug, "title": title,
            "published": cell(row, "Enabled") == Some("Yes"), "reviewed_on": reviewed_on, "category": category,
            "welsh": welsh, "not_mapped": not_mapped, "problems": problems, "items": item_report,
        }));
    }
    let unread: Vec<&String> = input
        .tabs
        .keys()
        .filter(|t| !read_tabs.contains(*t))
        .collect();
    json!({
        "written": false,
        "totals": { "index_rows": index.len(), "roadmaps_mapped": mapped, "roadmaps_skipped": skipped,
                    "item_rows": item_rows, "items_mapped": items_mapped, "items_skipped": items_skipped },
        "roadmaps": out,
        "tabs_not_in_the_index": unread,
    })
}

/// One roadmap row of the index, as the apply step writes it.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedRoadmap {
    /// Its row in the index (row 1 is the header).
    pub row: usize,
    /// The `Sheet` cell, as written.
    pub sheet: String,
    /// The slug made from `Sheet`.
    pub slug: String,
    /// `Title`, or `Sheet` when it is blank.
    pub title: String,
    /// `Vision`.
    pub vision: Option<String>,
    /// `Service description`.
    pub service_description: Option<String>,
    /// `Category`, as written.
    pub category: Option<String>,
    /// Whether `Enabled` is exactly `Yes`.
    pub enabled: bool,
    /// `Last updated`, when it is a date.
    pub reviewed_on: Option<chrono::NaiveDate>,
    /// The Welsh cells: field to text.
    pub welsh: BTreeMap<&'static str, Value>,
    /// The tab's rows that have a title; `None` when the tab is missing or cannot be read, with why.
    pub items: Result<Vec<ParsedItem>, String>,
    /// The tab's rows skipped, by row, with why.
    pub items_skipped: Vec<(usize, &'static str)>,
}

/// One item row of a roadmap tab, as the apply step writes it.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedItem {
    /// Its row in the tab (row 1 is the header).
    pub row: usize,
    /// `Title`.
    pub title: String,
    /// `Description`.
    pub description: Option<String>,
    /// `Outcome`.
    pub expected_outcome: Option<String>,
    /// `Phase`.
    pub phase: Option<String>,
    /// `Labels`.
    pub tags: Vec<String>,
    /// `Details`.
    pub details: Vec<Detail>,
    /// `Horizon` as a placement; `None` is unplaced.
    pub placement: Option<&'static str>,
    /// `Organisations`, as written.
    pub codes: Vec<String>,
    /// `Item ID`, when it is a valid public id not used by an earlier row.
    pub public_id: Option<String>,
    /// The Welsh cells: field to value, in the shape the field has (text, a tag list, detail lines).
    pub welsh: BTreeMap<&'static str, Value>,
}

/// The roadmap index's rows that the dry run maps, as values, with their items. Rows the dry run skips are left out;
/// the dry run's report says why.
///
/// # Errors
///
/// When the roadmap index cannot be read.
pub fn parse(input: &Input) -> Result<Vec<ParsedRoadmap>, String> {
    let index = rows(&input.index).map_err(|e| format!("the roadmap index cannot be read: {e}"))?;
    let mut seen_ids = BTreeSet::new();
    let mut slugs = BTreeSet::new();
    let mut out = Vec::new();
    for (i, row) in index.iter().enumerate() {
        let Some(sheet) = cell(row, "Sheet") else {
            continue;
        };
        if sheet.eq_ignore_ascii_case("Template") {
            continue;
        }
        let Some(slug) = rules::slug_from(sheet).filter(|s| slugs.insert(s.clone())) else {
            continue;
        };
        let welsh = [
            ("Title (cy)", "title"),
            ("Vision (cy)", "vision"),
            ("Service description (cy)", "service_description"),
        ]
        .into_iter()
        .filter_map(|(col, f)| cell(row, col).map(|v| (f, json!(v))))
        .collect();
        let (items, items_skipped) = match input.tabs.get(sheet).map(|t| rows(t)) {
            None => (Err(format!("no file for the tab `{sheet}`")), Vec::new()),
            Some(Err(e)) => (
                Err(format!("the tab `{sheet}` cannot be read: {e}")),
                Vec::new(),
            ),
            Some(Ok(tab)) => {
                let mut list = Vec::new();
                let mut skipped = Vec::new();
                for (j, r) in tab.iter().enumerate() {
                    match parsed_item(j + 2, r, &mut seen_ids) {
                        Some(it) => list.push(it),
                        None => skipped.push((j + 2, "no title")),
                    }
                }
                (Ok(list), skipped)
            }
        };
        out.push(ParsedRoadmap {
            row: i + 2,
            sheet: sheet.to_string(),
            slug,
            title: cell(row, "Title").unwrap_or(sheet).to_string(),
            vision: cell(row, "Vision").map(str::to_string),
            service_description: cell(row, "Service description").map(str::to_string),
            category: cell(row, "Category").map(str::to_string),
            enabled: cell(row, "Enabled") == Some("Yes"),
            reviewed_on: cell(row, "Last updated").and_then(date),
            welsh,
            items,
            items_skipped,
        });
    }
    Ok(out)
}

/// One item row as a value; `None` when it has no title.
fn parsed_item(n: usize, row: &Row, seen_ids: &mut BTreeSet<String>) -> Option<ParsedItem> {
    let title = cell(row, "Title")?;
    let public_id = cell(row, "Item ID")
        .filter(|id| rules::is_public_id(id) && !seen_ids.contains(*id))
        .map(str::to_string);
    if let Some(id) = &public_id {
        seen_ids.insert(id.clone());
    }
    let mut welsh = BTreeMap::new();
    for (col, field) in [
        ("Title (cy)", "title"),
        ("Description (cy)", "description"),
        ("Outcome (cy)", "expected_outcome"),
        ("Phase (cy)", "phase"),
    ] {
        if let Some(v) = cell(row, col) {
            welsh.insert(field, json!(v));
        }
    }
    if let Some(v) = cell(row, "Labels (cy)") {
        welsh.insert("tags", json!(list(v)));
    }
    if let Some(v) = cell(row, "Details (cy)") {
        welsh.insert("details", json!(details(v)));
    }
    Some(ParsedItem {
        row: n,
        title: title.to_string(),
        description: cell(row, "Description").map(str::to_string),
        expected_outcome: cell(row, "Outcome").map(str::to_string),
        phase: cell(row, "Phase").map(str::to_string),
        tags: cell(row, "Labels").map_or_default(list),
        details: cell(row, "Details").map_or_default(details),
        placement: placement(cell(row, "Horizon").unwrap_or_default()),
        codes: cell(row, "Organisations").map_or_default(list),
        public_id,
        welsh,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/dhcw-roadmaps")
            .join(name);
        std::fs::read_to_string(path).expect("read fixture")
    }

    fn input() -> Input {
        Input {
            index: fixture("Roadmaps.csv"),
            tabs: ["ProductA", "ProductB", "ProductC"]
                .into_iter()
                .map(|t| (t.to_string(), fixture(&format!("{t}.csv"))))
                .collect(),
            known_codes: vec!["SBUHB".into(), "National".into()],
            known_categories: vec!["GP & Community Services".into()],
        }
    }

    #[test]
    fn every_row_of_the_example_spreadsheet_is_accounted_for() {
        let r = dry_run(&input());
        assert_eq!(r["written"], false);
        let t = &r["totals"];
        assert_eq!(
            t["index_rows"],
            t["roadmaps_mapped"].as_u64().unwrap() + t["roadmaps_skipped"].as_u64().unwrap()
        );
        assert_eq!(
            t["item_rows"],
            t["items_mapped"].as_u64().unwrap() + t["items_skipped"].as_u64().unwrap()
        );
        assert_eq!(t["index_rows"], 3, "three roadmaps in Roadmaps.csv");
        assert_eq!(t["roadmaps_mapped"], 3);
        assert!(t["items_mapped"].as_u64().unwrap() > 0);
        assert_eq!(r["tabs_not_in_the_index"], json!([]));
        for roadmap in r["roadmaps"].as_array().unwrap() {
            let items = &roadmap["items"];
            assert!(items.get("error").is_none(), "{roadmap}");
            assert_eq!(
                items["rows"],
                items["mapped"].as_u64().unwrap() + items["skipped"].as_u64().unwrap()
            );
        }
    }

    #[test]
    fn the_example_maps_as_its_readme_describes() {
        let r = dry_run(&input());
        let a = &r["roadmaps"][0];
        assert_eq!(a["slug"], "producta");
        assert_eq!(a["published"], true);
        assert_eq!(a["reviewed_on"], "2026-10-01");
        assert_eq!(
            a["category"],
            json!({ "name": "GP & Community Services", "known": true })
        );
        assert!(
            a["not_mapped"].get("Product Lead").is_some(),
            "a person is not imported"
        );
        let b = &r["roadmaps"][1];
        assert_eq!(b["reviewed_on"], "2026-09-15", "the day-first format");
        assert!(b["welsh"].as_array().unwrap().contains(&json!("title")));
        let c = &r["roadmaps"][2];
        assert_eq!(c["published"], false, "Enabled is No");
        let first = &a["items"]["each"][0];
        assert_eq!(first["placement"], "now");
        assert_eq!(first["organizations"], json!(["SBUHB"]));
        assert_eq!(first["unknown_organization_codes"], json!(["CAVUHB"]));
    }

    #[test]
    fn parse_gives_the_rows_the_dry_run_maps() {
        let input = input();
        let parsed = parse(&input).unwrap();
        let report = dry_run(&input);
        assert_eq!(
            parsed.len() as u64,
            report["totals"]["roadmaps_mapped"].as_u64().unwrap()
        );
        let items: usize = parsed.iter().map(|r| r.items.as_ref().unwrap().len()).sum();
        assert_eq!(
            items as u64,
            report["totals"]["items_mapped"].as_u64().unwrap()
        );
        for (p, r) in parsed.iter().zip(report["roadmaps"].as_array().unwrap()) {
            assert_eq!(json!(p.slug), r["slug"]);
            assert_eq!(json!(p.enabled), r["published"]);
            assert_eq!(json!(p.reviewed_on), r["reviewed_on"]);
            for (it, ri) in p.items.as_ref().unwrap().iter().zip(
                r["items"]["each"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|x| x["outcome"] == "mapped"),
            ) {
                assert_eq!(json!(it.title), ri["title"]);
                assert_eq!(json!(it.placement), ri["placement"]);
                assert_eq!(
                    it.public_id.clone().map_or(json!("new"), |v| json!(v)),
                    ri["public_id"]
                );
            }
        }
        assert!(parsed[1].welsh.contains_key("title"));
        let first = &parsed[0].items.as_ref().unwrap()[0];
        assert_eq!(first.codes, ["CAVUHB", "SBUHB"]);
    }

    #[test]
    fn cells_read_as_the_editor_reads_them() {
        assert_eq!(
            date("15/09/2026"),
            chrono::NaiveDate::from_ymd_opt(2026, 9, 15)
        );
        assert_eq!(
            date("2026-10-01"),
            chrono::NaiveDate::from_ymd_opt(2026, 10, 1)
        );
        assert_eq!(date("Sept 2026"), None);
        assert_eq!(
            placement(" Delivered this year "),
            Some("delivered_this_year")
        );
        assert_eq!(placement("Soon"), None);
        let d = details("Follow-up\n- Dermatology\n-- Adults first\n");
        assert_eq!(d.iter().map(|x| x.level).collect::<Vec<_>>(), [0, 1, 2]);
        assert_eq!(d[2].text, "Adults first");
        assert_eq!(list("National, SBUHB, "), ["National", "SBUHB"]);
    }
}

//! Jira and Asana CSV import (T-28n): the pure reader. DB-free and deterministic.
//!
//! Both trackers export a table of work items. This module reads the CSV (RFC 4180:
//! quoted fields, doubled quotes, newlines inside quotes, BOM, CRLF), recognizes the
//! columns of each tracker by name, and turns every row into an [`Item`] — a key,
//! a title, a status bucket, optional dates, a parent and the items it blocks.
//!
//! **Nothing is silently dropped.** A column, row or link that cannot come across is
//! listed in [`Parsed::unmapped`] with the reason. Assignees are **names, not
//! identities**: they are kept in the description and never matched to a person.
//! The file is bounded (size, rows, columns, field length) and never trusted.

use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use serde::Serialize;

/// The largest file read.
pub const MAX_BYTES: usize = 5 * 1024 * 1024;
/// The most item rows read.
pub const MAX_ROWS: usize = 20_000;
/// The most columns in a row.
pub const MAX_COLUMNS: usize = 200;
/// The longest single field kept (longer is cut, and said).
pub const MAX_FIELD: usize = 20_000;
/// The deepest parent chain kept (the same cap as sub-tasks everywhere).
pub const MAX_DEPTH: usize = 10;

/// Which tracker's columns to expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// Jira issue export (CSV).
    Jira,
    /// Asana project export (CSV).
    Asana,
}

impl Source {
    /// Parse the URL token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "jira" => Some(Self::Jira),
            "asana" => Some(Self::Asana),
            _ => None,
        }
    }

    /// The prefix of the stored `external_ref` (`jira:PROJ-1`).
    #[must_use]
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Jira => "jira",
            Self::Asana => "asana",
        }
    }
}

/// Where an item is in its life, as far as the export says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bucket {
    /// Not started.
    Initial,
    /// Under way.
    Active,
    /// Finished.
    Done,
}

/// One work item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// The tracker's own id (`PROJ-1`, an Asana task id): the identity for re-import.
    pub key: String,
    /// The title.
    pub title: String,
    /// The description, with assignee and labels appended as plain lines.
    pub description: Option<String>,
    /// Status bucket.
    pub bucket: Bucket,
    /// A start date, if the export has one.
    pub start: Option<NaiveDate>,
    /// A due date, kept as the task's deadline.
    pub due: Option<NaiveDate>,
    /// The parent item's key (resolved), or `None`.
    pub parent: Option<String>,
    /// Keys of items this one must finish before (this blocks them).
    pub blocks: Vec<String>,
}

/// One thing that did not come across.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Unmapped {
    /// `column`, `row`, `link`, `parent`, `date`, `field`.
    pub what: &'static str,
    /// The item key, when there is one.
    pub key: Option<String>,
    /// Why.
    pub reason: String,
}

/// A parsed export.
#[derive(Debug, Default)]
pub struct Parsed {
    /// The items, in file order, keys unique.
    pub items: Vec<Item>,
    /// What was left out.
    pub unmapped: Vec<Unmapped>,
}

/// Why a file was refused outright.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// Larger than [`MAX_BYTES`].
    TooLarge,
    /// No header row.
    Empty,
    /// A quote that never closes.
    UnclosedQuote,
    /// More than [`MAX_ROWS`] rows.
    TooManyRows,
    /// A row wider than [`MAX_COLUMNS`].
    TooWide,
    /// The header lacks the columns this tracker's export always has.
    MissingColumns(Vec<&'static str>),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLarge => write!(f, "the file is larger than {MAX_BYTES} bytes"),
            Self::Empty => write!(f, "the file is empty"),
            Self::UnclosedQuote => write!(f, "the CSV has a quoted field that never closes"),
            Self::TooManyRows => write!(f, "the file has more than {MAX_ROWS} rows"),
            Self::TooWide => write!(f, "a row has more than {MAX_COLUMNS} columns"),
            Self::MissingColumns(c) => write!(
                f,
                "this does not look like that export: missing column(s) {}",
                c.join(", ")
            ),
        }
    }
}

/// Read RFC 4180 CSV into rows of fields. A leading BOM is dropped; `\r\n`, `\n` and `\r`
/// all end a row; blank lines are skipped.
///
/// # Errors
///
/// [`ParseError`] for an oversize file, an unclosed quote, too many rows or columns.
pub fn read_csv(text: &str) -> Result<Vec<Vec<String>>, ParseError> {
    if text.len() > MAX_BYTES {
        return Err(ParseError::TooLarge);
    }
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut was_quoted = false;
    let mut chars = text.chars().peekable();
    let end_field = |row: &mut Vec<String>, field: &mut String| -> Result<(), ParseError> {
        if row.len() >= MAX_COLUMNS {
            return Err(ParseError::TooWide);
        }
        row.push(std::mem::take(field));
        Ok(())
    };
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            } else {
                field.push(c);
            }
            continue;
        }
        match c {
            '"' if field.is_empty() => {
                quoted = true;
                was_quoted = true;
            }
            ',' => end_field(&mut row, &mut field)?,
            '\r' | '\n' => {
                if c == '\r' && chars.peek() == Some(&'\n') {
                    chars.next();
                }
                let blank = row.is_empty() && field.is_empty() && !was_quoted;
                if !blank {
                    end_field(&mut row, &mut field)?;
                    rows.push(std::mem::take(&mut row));
                    if rows.len() > MAX_ROWS + 1 {
                        return Err(ParseError::TooManyRows);
                    }
                }
                was_quoted = false;
            }
            _ => field.push(c),
        }
    }
    if quoted {
        return Err(ParseError::UnclosedQuote);
    }
    if !(row.is_empty() && field.is_empty() && !was_quoted) {
        end_field(&mut row, &mut field)?;
        rows.push(row);
    }
    if rows.len() > MAX_ROWS + 1 {
        return Err(ParseError::TooManyRows);
    }
    Ok(rows)
}

const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

/// A date in the forms the two exports use: `2026-10-15` (Asana, and Jira's ISO option) and
/// Jira's `15/Oct/26 12:00 AM` or `15/Oct/2026`. Day-first numeric forms (`15/10/2026`,
/// `10/15/2026`) are **not** guessed: the day and month are ambiguous.
#[must_use]
pub fn parse_date(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(head) = s.get(..10)
        && let Ok(d) = NaiveDate::parse_from_str(head, "%Y-%m-%d")
        && (s.len() == 10 || s[10..].starts_with(['T', ' ']))
    {
        return Some(d);
    }
    let first = s.split_whitespace().next()?;
    let mut parts = first.split('/');
    let (day, month, year) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let m = MONTHS
        .iter()
        .position(|x| month.to_ascii_lowercase().starts_with(x))?
        + 1;
    let day: u32 = day.parse().ok()?;
    let year: i32 = year.parse().ok()?;
    let year = if year < 100 { 2000 + year } else { year };
    NaiveDate::from_ymd_opt(year, u32::try_from(m).ok()?, day)
}

/// Where a Jira status text belongs. Words, not a fixed list, because statuses are
/// per-project: anything not recognized starts in the initial state.
#[must_use]
pub fn jira_bucket(status: &str) -> Bucket {
    let s = status.trim().to_ascii_lowercase();
    if [
        "done",
        "closed",
        "resolved",
        "complete",
        "completed",
        "released",
        "fixed",
    ]
    .iter()
    .any(|w| s == *w)
    {
        Bucket::Done
    } else if [
        "progress",
        "review",
        "testing",
        "development",
        "doing",
        "qa",
    ]
    .iter()
    .any(|w| s.contains(w))
    {
        Bucket::Active
    } else {
        Bucket::Initial
    }
}

struct Header {
    names: Vec<String>,
}

impl Header {
    fn new(row: &[String]) -> Self {
        Self {
            names: row.iter().map(|h| h.trim().to_ascii_lowercase()).collect(),
        }
    }

    fn first(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    fn all<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item = (usize, &'a str)> + 'a {
        self.names
            .iter()
            .enumerate()
            .filter_map(move |(i, n)| n.strip_prefix(prefix).map(|rest| (i, rest)))
    }
}

fn cell(row: &[String], i: Option<usize>) -> &str {
    i.and_then(|i| row.get(i)).map_or("", |s| s.trim())
}

fn clip(s: &str, key: &str, unmapped: &mut Vec<Unmapped>) -> String {
    if s.chars().count() > MAX_FIELD {
        unmapped.push(Unmapped {
            what: "field",
            key: Some(key.to_string()),
            reason: format!("a field was cut to {MAX_FIELD} characters"),
        });
        return s.chars().take(MAX_FIELD).collect();
    }
    s.to_string()
}

fn describe(base: &str, assignee: &str, labels: &str, tracker: &str) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if !base.is_empty() {
        parts.push(base.to_string());
    }
    if !assignee.is_empty() {
        parts.push(format!("Assigned in {tracker}: {assignee}"));
    }
    if !labels.is_empty() {
        parts.push(format!("Labels in {tracker}: {labels}"));
    }
    (!parts.is_empty()).then(|| parts.join("\n\n"))
}

/// Read an export.
///
/// # Errors
///
/// [`ParseError`] when the file cannot be read or is not that tracker's export.
pub fn parse(source: Source, text: &str) -> Result<Parsed, ParseError> {
    let rows = read_csv(text)?;
    let Some((head, body)) = rows.split_first() else {
        return Err(ParseError::Empty);
    };
    let header = Header::new(head);
    let mut out = Parsed::default();
    match source {
        Source::Jira => jira(&header, body, &mut out)?,
        Source::Asana => asana(&header, body, &mut out)?,
    }
    finish(&mut out);
    Ok(out)
}

fn missing(header: &Header, need: &[&'static str]) -> Result<(), ParseError> {
    let lacking: Vec<&'static str> = need
        .iter()
        .copied()
        .filter(|n| header.first(n).is_none())
        .collect();
    if lacking.is_empty() {
        Ok(())
    } else {
        Err(ParseError::MissingColumns(lacking))
    }
}

#[expect(clippy::too_many_lines)]
fn jira(h: &Header, body: &[Vec<String>], out: &mut Parsed) -> Result<(), ParseError> {
    missing(h, &["issue key", "summary"])?;
    let (key_c, sum_c) = (h.first("issue key"), h.first("summary"));
    let (desc_c, status_c, who_c, due_c) = (
        h.first("description"),
        h.first("status"),
        h.first("assignee"),
        h.first("due date"),
    );
    let parent_c = h
        .first("parent")
        .or_else(|| h.first("parent id"))
        .or_else(|| h.first("parent key"));
    let label_cols: Vec<usize> = h.all("labels").map(|(i, _)| i).collect();
    let known = [
        "issue key",
        "summary",
        "description",
        "status",
        "assignee",
        "due date",
        "parent",
        "parent id",
        "parent key",
        "labels",
    ];
    for (i, n) in h.names.iter().enumerate() {
        let handled = known.contains(&n.as_str())
            || n.starts_with("outward issue link (")
            || n.starts_with("inward issue link (")
            || label_cols.contains(&i);
        if !handled && !n.is_empty() {
            out.unmapped.push(Unmapped {
                what: "column",
                key: None,
                reason: format!("{}: not imported", head_name(h, i)),
            });
        }
    }
    // Links the export names: (this, other, direction, type).
    let mut links: Vec<(String, String, bool, String)> = Vec::new();
    for row in body {
        let key = cell(row, key_c).to_string();
        if key.is_empty() {
            out.unmapped.push(Unmapped {
                what: "row",
                key: None,
                reason: "a row with no issue key was skipped".into(),
            });
            continue;
        }
        let title = clip(cell(row, sum_c), &key, &mut out.unmapped);
        if title.is_empty() {
            out.unmapped.push(Unmapped {
                what: "row",
                key: Some(key),
                reason: "a row with no summary was skipped".into(),
            });
            continue;
        }
        let labels: Vec<&str> = label_cols
            .iter()
            .map(|i| cell(row, Some(*i)))
            .filter(|s| !s.is_empty())
            .collect();
        let due = date_cell(cell(row, due_c), &key, &mut out.unmapped);
        for (i, rest) in h.all("outward issue link (") {
            for other in cell(row, Some(i))
                .split([',', ' '])
                .filter(|s| !s.is_empty())
            {
                links.push((
                    key.clone(),
                    other.to_string(),
                    true,
                    rest.trim_end_matches(')').to_string(),
                ));
            }
        }
        for (i, rest) in h.all("inward issue link (") {
            for other in cell(row, Some(i))
                .split([',', ' '])
                .filter(|s| !s.is_empty())
            {
                links.push((
                    key.clone(),
                    other.to_string(),
                    false,
                    rest.trim_end_matches(')').to_string(),
                ));
            }
        }
        let desc = clip(cell(row, desc_c), &key, &mut out.unmapped);
        out.items.push(Item {
            key: key.clone(),
            title,
            description: describe(&desc, cell(row, who_c), &labels.join(", "), "Jira"),
            bucket: jira_bucket(cell(row, status_c)),
            start: None,
            due,
            parent: Some(cell(row, parent_c).to_string()).filter(|p| !p.is_empty()),
            blocks: Vec::new(),
        });
    }
    let keys: HashSet<String> = out.items.iter().map(|i| i.key.clone()).collect();
    for (this, other, outward, kind) in links {
        // Only "blocks" is a precedence. Outward "Blocks": this blocks other; inward: other blocks this.
        if !kind.eq_ignore_ascii_case("blocks") {
            out.unmapped.push(Unmapped {
                what: "link",
                key: Some(this),
                reason: format!("a \"{kind}\" link to {other} is not a precedence; not imported"),
            });
            continue;
        }
        let (pred, succ) = if outward {
            (this.clone(), other.clone())
        } else {
            (other.clone(), this.clone())
        };
        if !keys.contains(&pred) || !keys.contains(&succ) {
            out.unmapped.push(Unmapped {
                what: "link",
                key: Some(this),
                reason: format!("the link with {other} was skipped: that issue is not in the file"),
            });
            continue;
        }
        if let Some(p) = out.items.iter_mut().find(|i| i.key == pred)
            && !p.blocks.contains(&succ)
        {
            p.blocks.push(succ);
        }
    }
    Ok(())
}

fn head_name(h: &Header, i: usize) -> String {
    h.names.get(i).cloned().unwrap_or_default()
}

fn date_cell(raw: &str, key: &str, unmapped: &mut Vec<Unmapped>) -> Option<NaiveDate> {
    if raw.is_empty() {
        return None;
    }
    let d = parse_date(raw);
    if d.is_none() {
        unmapped.push(Unmapped {
            what: "date",
            key: Some(key.to_string()),
            reason: format!("the date \"{raw}\" was not understood (use YYYY-MM-DD); not imported"),
        });
    }
    d
}

#[expect(clippy::too_many_lines)]
fn asana(h: &Header, body: &[Vec<String>], out: &mut Parsed) -> Result<(), ParseError> {
    missing(h, &["task id", "name"])?;
    let (id_c, name_c) = (h.first("task id"), h.first("name"));
    let (notes_c, who_c, due_c, start_c) = (
        h.first("notes"),
        h.first("assignee"),
        h.first("due date"),
        h.first("start date"),
    );
    let (done_c, parent_c) = (h.first("completed at"), h.first("parent task"));
    let blocked_by_c = h.names.iter().position(|n| n.starts_with("blocked by"));
    let blocking_c = h.names.iter().position(|n| n.starts_with("blocking"));
    let tags_c = h.first("tags");
    let known = [
        "task id",
        "name",
        "notes",
        "assignee",
        "due date",
        "start date",
        "completed at",
        "parent task",
        "tags",
    ];
    for (i, n) in h.names.iter().enumerate() {
        let handled =
            known.contains(&n.as_str()) || Some(i) == blocked_by_c || Some(i) == blocking_c;
        if !handled && !n.is_empty() {
            out.unmapped.push(Unmapped {
                what: "column",
                key: None,
                reason: format!("{}: not imported", head_name(h, i)),
            });
        }
    }
    let mut parent_names: Vec<(String, String)> = Vec::new();
    let mut links: Vec<(String, String, bool)> = Vec::new();
    for row in body {
        let key = cell(row, id_c).to_string();
        if key.is_empty() {
            out.unmapped.push(Unmapped {
                what: "row",
                key: None,
                reason: "a row with no task id was skipped".into(),
            });
            continue;
        }
        let title = clip(cell(row, name_c), &key, &mut out.unmapped);
        if title.is_empty() {
            out.unmapped.push(Unmapped {
                what: "row",
                key: Some(key),
                reason: "a row with no name was skipped".into(),
            });
            continue;
        }
        let due = date_cell(cell(row, due_c), &key, &mut out.unmapped);
        let start = date_cell(cell(row, start_c), &key, &mut out.unmapped);
        if let (Some(s), Some(d)) = (start, due)
            && s > d
        {
            out.unmapped.push(Unmapped {
                what: "date",
                key: Some(key.clone()),
                reason: "the start is after the due date; the start was not imported".into(),
            });
        }
        let start = match (start, due) {
            (Some(s), Some(d)) if s > d => None,
            (s, _) => s,
        };
        if !cell(row, parent_c).is_empty() {
            parent_names.push((key.clone(), cell(row, parent_c).to_string()));
        }
        for other in cell(row, blocked_by_c)
            .split([',', ';'])
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            links.push((key.clone(), other.to_string(), false));
        }
        for other in cell(row, blocking_c)
            .split([',', ';'])
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            links.push((key.clone(), other.to_string(), true));
        }
        let notes = clip(cell(row, notes_c), &key, &mut out.unmapped);
        out.items.push(Item {
            key,
            title,
            description: describe(&notes, cell(row, who_c), cell(row, tags_c), "Asana"),
            bucket: if cell(row, done_c).is_empty() {
                Bucket::Initial
            } else {
                Bucket::Done
            },
            start,
            due,
            parent: None,
            blocks: Vec::new(),
        });
    }
    // Asana exports the parent's *name*: it is a parent only if exactly one task has that name.
    let mut by_name: HashMap<String, Vec<String>> = HashMap::new();
    for i in &out.items {
        by_name
            .entry(i.title.clone())
            .or_default()
            .push(i.key.clone());
    }
    for (child, name) in parent_names {
        match by_name.get(&name).map(Vec::as_slice) {
            Some([one]) if *one != child => {
                if let Some(c) = out.items.iter_mut().find(|i| i.key == child) {
                    c.parent = Some(one.clone());
                }
            }
            Some(many) if many.len() > 1 => out.unmapped.push(Unmapped { what: "parent", key: Some(child), reason: format!("{} tasks are named \"{name}\", so the parent cannot be told; imported at the top level", many.len()) }),
            _ => out.unmapped.push(Unmapped { what: "parent", key: Some(child), reason: format!("the parent \"{name}\" is not a task in the file; imported at the top level") }),
        }
    }
    let keys: HashSet<String> = out.items.iter().map(|i| i.key.clone()).collect();
    for (this, other, blocking) in links {
        let (pred, succ) = if blocking {
            (this.clone(), other.clone())
        } else {
            (other.clone(), this.clone())
        };
        if !keys.contains(&pred) || !keys.contains(&succ) {
            out.unmapped.push(Unmapped {
                what: "link",
                key: Some(this),
                reason: format!(
                    "the dependency with {other} was skipped: that task is not in the file"
                ),
            });
            continue;
        }
        if let Some(p) = out.items.iter_mut().find(|i| i.key == pred)
            && !p.blocks.contains(&succ)
        {
            p.blocks.push(succ);
        }
    }
    Ok(())
}

/// Clean up what both readers share: unique keys, parents that exist, no parent cycles,
/// no chain deeper than [`MAX_DEPTH`], no self-links.
fn finish(out: &mut Parsed) {
    let mut seen: HashSet<String> = HashSet::new();
    let mut kept: Vec<Item> = Vec::new();
    for it in std::mem::take(&mut out.items) {
        if seen.insert(it.key.clone()) {
            kept.push(it);
        } else {
            out.unmapped.push(Unmapped {
                what: "row",
                key: Some(it.key),
                reason: "a second row with the same key was skipped".into(),
            });
        }
    }
    out.items = kept;
    let keys: HashSet<String> = out.items.iter().map(|i| i.key.clone()).collect();
    let parent_of: HashMap<String, String> = out
        .items
        .iter()
        .filter_map(|i| i.parent.clone().map(|p| (i.key.clone(), p)))
        .collect();
    for it in &mut out.items {
        let Some(p) = it.parent.clone() else { continue };
        let mut reason: Option<String> = None;
        if !keys.contains(&p) {
            reason = Some(format!(
                "the parent {p} is not in the file; imported at the top level"
            ));
        } else if p == it.key {
            reason = Some("an item cannot be its own parent".into());
        } else {
            // Walk up: a cycle, or a chain longer than the cap, drops this parent.
            let mut cur = p.clone();
            let mut depth = 1;
            loop {
                if cur == it.key {
                    reason = Some("the parent chain loops; imported at the top level".into());
                    break;
                }
                let Some(next) = parent_of.get(&cur) else {
                    break;
                };
                depth += 1;
                if depth > MAX_DEPTH {
                    reason = Some(format!(
                        "the parent chain is deeper than {MAX_DEPTH}; imported at the top level"
                    ));
                    break;
                }
                cur = next.clone();
            }
        }
        if let Some(r) = reason {
            // A parent that was never in the file was already reported by the Asana reader.
            let already = out
                .unmapped
                .iter()
                .any(|u| u.what == "parent" && u.key.as_deref() == Some(&it.key));
            if !already {
                out.unmapped.push(Unmapped {
                    what: "parent",
                    key: Some(it.key.clone()),
                    reason: r,
                });
            }
            it.parent = None;
        }
    }
    for it in &mut out.items {
        let me = it.key.clone();
        it.blocks.retain(|b| *b != me);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn csv_handles_quotes_newlines_bom_and_blank_lines() {
        let text = "\u{feff}a,b,c\r\n1,\"x, \"\"y\"\"\",3\r\n\r\n\"line1\nline2\",,\"\"\r\n";
        let rows = read_csv(text).unwrap();
        assert_eq!(rows[0], ["a", "b", "c"]);
        assert_eq!(rows[1], ["1", "x, \"y\"", "3"]);
        assert_eq!(rows[2], ["line1\nline2", "", ""]);
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn csv_refuses_what_it_cannot_trust() {
        assert_eq!(read_csv("a,\"b").unwrap_err(), ParseError::UnclosedQuote);
        assert_eq!(
            read_csv(&"x".repeat(MAX_BYTES + 1)).unwrap_err(),
            ParseError::TooLarge
        );
        assert_eq!(
            read_csv(&",".repeat(MAX_COLUMNS + 1)).unwrap_err(),
            ParseError::TooWide
        );
        let many = "a\n".repeat(MAX_ROWS + 2);
        assert_eq!(read_csv(&many).unwrap_err(), ParseError::TooManyRows);
        assert_eq!(parse(Source::Jira, "").unwrap_err(), ParseError::Empty);
        assert!(matches!(
            parse(Source::Jira, "Name,Notes\nx,y\n").unwrap_err(),
            ParseError::MissingColumns(_)
        ));
        assert!(matches!(
            parse(Source::Asana, "Issue key,Summary\nA-1,x\n").unwrap_err(),
            ParseError::MissingColumns(_)
        ));
    }

    #[test]
    fn dates_are_read_in_the_forms_the_exports_use_and_never_guessed() {
        let d = |y, m, day| NaiveDate::from_ymd_opt(y, m, day);
        assert_eq!(parse_date("2026-10-15"), d(2026, 10, 15));
        assert_eq!(parse_date("2026-10-15T09:30:00Z"), d(2026, 10, 15));
        assert_eq!(parse_date("15/Oct/26 12:00 AM"), d(2026, 10, 15));
        assert_eq!(parse_date("5/Jan/2027"), d(2027, 1, 5));
        assert_eq!(
            parse_date("15/10/2026"),
            None,
            "day-first numeric is ambiguous"
        );
        assert_eq!(parse_date("10/15/2026"), None);
        assert_eq!(parse_date("31/Feb/26"), None);
        assert_eq!(parse_date("2026-13-01"), None);
        assert_eq!(parse_date(""), None);
        assert_eq!(parse_date("soon"), None);
    }

    #[test]
    fn jira_statuses_fall_into_three_buckets() {
        for (s, b) in [
            ("Done", Bucket::Done),
            ("Closed", Bucket::Done),
            ("In Progress", Bucket::Active),
            ("In Review", Bucket::Active),
            ("To Do", Bucket::Initial),
            ("Backlog", Bucket::Initial),
            ("", Bucket::Initial),
            ("Weird", Bucket::Initial),
        ] {
            assert_eq!(jira_bucket(s), b, "{s}");
        }
    }

    const JIRA: &str = "Issue key,Summary,Status,Assignee,Due date,Description,Parent,Labels,Labels,Outward issue link (Blocks),Inward issue link (Blocks),Outward issue link (Relates),Sprint\n\
P-1,Epic,In Progress,Ann,15/Oct/26 12:00 AM,Big thing,,a,b,,,,S1\n\
P-2,Build it,To Do,Bob,2026-10-20,\"Do it,\nwell\",P-1,,,P-3,,,\n\
P-3,Ship it,Done,,,,P-1,,,,,P-2,\n\
P-4,Odd,To Do,,31/13/26,,P-9,,,P-99,,,\n";

    #[test]
    fn a_jira_export_becomes_items_with_parents_links_and_reasons() {
        let p = parse(Source::Jira, JIRA).unwrap();
        let by = |k: &str| p.items.iter().find(|i| i.key == k).unwrap();
        assert_eq!(p.items.len(), 4);
        assert_eq!(by("P-1").bucket, Bucket::Active);
        assert_eq!(by("P-1").due, NaiveDate::from_ymd_opt(2026, 10, 15));
        assert_eq!(
            by("P-1").description.as_deref(),
            Some("Big thing\n\nAssigned in Jira: Ann\n\nLabels in Jira: a, b")
        );
        assert_eq!(
            by("P-2").description.as_deref(),
            Some("Do it,\nwell\n\nAssigned in Jira: Bob")
        );
        assert_eq!(by("P-2").parent.as_deref(), Some("P-1"));
        assert_eq!(by("P-3").bucket, Bucket::Done);
        assert_eq!(
            by("P-2").blocks,
            ["P-3"],
            "an outward Blocks link: P-2 blocks P-3"
        );
        assert!(by("P-3").blocks.is_empty());
        let why = |what: &str| {
            p.unmapped
                .iter()
                .filter(|u| u.what == what)
                .map(|u| u.reason.clone())
                .collect::<Vec<_>>()
        };
        assert!(why("column").iter().any(|r| r.starts_with("sprint")));
        assert!(
            why("link").iter().any(|r| r.contains("relates")),
            "{:?}",
            why("link")
        );
        assert!(why("link").iter().any(|r| r.contains("P-99")));
        assert!(why("date").iter().any(|r| r.contains("31/13/26")));
        assert!(why("parent").iter().any(|r| r.contains("P-9")));
        assert_eq!(by("P-4").parent, None);
    }

    #[test]
    fn an_inward_blocks_link_points_the_other_way() {
        let csv = "Issue key,Summary,Inward issue link (Blocks)\nA-1,One,A-2\nA-2,Two,\n";
        let p = parse(Source::Jira, csv).unwrap();
        let a2 = p.items.iter().find(|i| i.key == "A-2").unwrap();
        assert_eq!(a2.blocks, ["A-1"], "A-1 is blocked by A-2");
    }

    const ASANA: &str = "Task ID,Created At,Completed At,Name,Assignee,Due Date,Start Date,Notes,Parent Task,Blocked By (Dependencies),Blocking (Dependencies),Tags,Projects\n\
1,2026-10-01,,Plan,Ann,2026-10-20,2026-10-05,Notes here,,,,x,P\n\
2,2026-10-01,2026-10-09,Design,,2026-10-08,,,Plan,,3,,P\n\
3,2026-10-01,,Build,,2026-10-12,2026-10-14,,Plan,2,,,P\n\
4,2026-10-01,,Dup,,,,,,,,,P\n\
5,2026-10-01,,Dup,,,,,,,,,P\n\
6,2026-10-01,,Child of dup,,,,,Dup,9,,,P\n\
7,2026-10-01,,Orphan,,,,,Missing,,,,P\n";

    #[test]
    fn an_asana_export_resolves_parents_by_unique_name_and_reports_the_rest() {
        let p = parse(Source::Asana, ASANA).unwrap();
        let by = |k: &str| p.items.iter().find(|i| i.key == k).unwrap();
        assert_eq!(by("2").parent.as_deref(), Some("1"));
        assert_eq!(by("2").bucket, Bucket::Done);
        assert_eq!(by("1").bucket, Bucket::Initial);
        assert_eq!(
            by("2").blocks,
            ["3"],
            "task 2 blocks 3 (both columns say so, once)"
        );
        assert_eq!(
            by("3").start,
            None,
            "a start after the due date is not imported"
        );
        assert_eq!(by("1").start, NaiveDate::from_ymd_opt(2026, 10, 5));
        assert_eq!(
            by("1").description.as_deref(),
            Some("Notes here\n\nAssigned in Asana: Ann\n\nLabels in Asana: x")
        );
        assert_eq!(by("6").parent, None);
        assert_eq!(by("7").parent, None);
        let reasons: Vec<&str> = p.unmapped.iter().map(|u| u.reason.as_str()).collect();
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("2 tasks are named \"Dup\"")),
            "{reasons:?}"
        );
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("\"Missing\" is not a task"))
        );
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("9 was skipped") || r.contains("with 9"))
        );
        assert!(reasons.iter().any(|r| r.starts_with("projects")));
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("start is after the due date"))
        );
    }

    #[test]
    fn duplicate_keys_loops_deep_chains_and_self_links_are_cut() {
        let csv = "Issue key,Summary,Parent,Outward issue link (Blocks)\nA,a,B,\nB,b,A,\nC,c,C,C\nA,again,,\n";
        let p = parse(Source::Jira, csv).unwrap();
        assert_eq!(p.items.len(), 3, "the second A is skipped");
        for i in &p.items {
            assert_eq!(i.parent, None, "{}", i.key);
            assert!(i.blocks.is_empty());
        }
        assert!(p.unmapped.iter().any(|u| u.reason.contains("same key")));
        assert!(p.unmapped.iter().any(|u| u.reason.contains("loops")));
        // A chain deeper than the cap loses the parent that would exceed it.
        let mut deep = String::from("Issue key,Summary,Parent\n");
        for n in 0..=MAX_DEPTH + 2 {
            let parent = if n == 0 {
                String::new()
            } else {
                format!("K{}", n - 1)
            };
            writeln!(deep, "K{n},t,{parent}").unwrap();
        }
        let p = parse(Source::Jira, &deep).unwrap();
        assert!(
            p.unmapped.iter().any(|u| u.reason.contains("deeper")),
            "{:?}",
            p.unmapped
        );
    }

    #[test]
    fn rows_without_a_key_or_title_are_reported_not_dropped_quietly() {
        let p = parse(Source::Jira, "Issue key,Summary\n,No key\nA-1,\nA-2,Fine\n").unwrap();
        assert_eq!(p.items.len(), 1);
        assert_eq!(p.unmapped.iter().filter(|u| u.what == "row").count(), 2);
    }

    #[test]
    fn a_long_field_is_cut_and_said() {
        let csv = format!(
            "Issue key,Summary,Description\nA-1,t,{}\n",
            "x".repeat(MAX_FIELD + 5)
        );
        let p = parse(Source::Jira, &csv).unwrap();
        assert_eq!(
            p.items[0].description.as_ref().unwrap().chars().count(),
            MAX_FIELD
        );
        assert!(p.unmapped.iter().any(|u| u.what == "field"));
    }

    #[test]
    fn the_same_file_parses_the_same_way_every_time() {
        let a = parse(Source::Asana, ASANA).unwrap();
        let b = parse(Source::Asana, ASANA).unwrap();
        assert_eq!(a.items, b.items);
        assert_eq!(a.unmapped, b.unmapped);
    }
}

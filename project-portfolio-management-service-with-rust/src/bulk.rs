//! Bulk import / export of plans (T-8): the pure codecs and rules.
//! DB-free, unit-tested. The design is `spec/bulk-import-export.md`.
//!
//! **Two lossless formats.** JSONL is one plan object per line; CSV is
//! RFC 4180 with every repeated or nested field (`alternate_names`, `goals`,
//! `keywords`, `tags`, `identifiers`, `same_as`, `relationships`) as **compact
//! JSON in its cell**, so a CSV round-trips a plan exactly as JSONL does.
//!
//! **One bad row never aborts a load.** Every problem becomes a [`RowError`]
//! (`row_number, source_line, field, code, message`); the job's counters always
//! reconcile: `created + updated + unchanged + queued_for_review + failed ==
//! total`. The error report names the offending *field* and never echoes the
//! source value, so it can be shared without leaking personal data.

use project_portfolio_management_matcher::Plan;
use serde::Serialize;
use serde_json::{Map, Value};
use uuid::Uuid;

/// The CSV columns, in order. `pid` is the stable key on re-import.
pub const CSV_COLUMNS: &[&str] = &[
    "pid",
    "kind",
    "name",
    "alternate_names",
    "code",
    "owner_org_id",
    "owner_org_name",
    "lead_ref",
    "parent_ref",
    "status",
    "phase",
    "goals",
    "start_date",
    "target_date",
    "keywords",
    "tags",
    "identifiers",
    "same_as",
    "in_language",
    "relationships",
];

/// Columns whose cell is always compact JSON.
const JSON_COLUMNS: &[&str] = &[
    "alternate_names",
    "goals",
    "keywords",
    "tags",
    "identifiers",
    "same_as",
    "relationships",
];

/// Cap on rows in one import.
pub const MAX_ROWS: usize = 100_000;

/// One problem with one source row.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RowError {
    /// 1-based ordinal among data rows.
    pub row_number: usize,
    /// The physical line the row starts on (differs for multi-line CSV cells
    /// and because of the CSV header).
    pub source_line: usize,
    /// The offending field, or empty for a whole-row failure.
    pub field: String,
    /// A stable machine code.
    pub code: &'static str,
    /// A message for people; never contains the source value.
    pub message: String,
}

/// The error report as CSV: `row_number,source_line,field,code,message`.
#[must_use]
pub fn errors_csv(errors: &[RowError]) -> String {
    let mut out = String::from("row_number,source_line,field,code,message\n");
    for e in errors {
        out.push_str(&csv_row(&[
            e.row_number.to_string(),
            e.source_line.to_string(),
            e.field.clone(),
            e.code.to_string(),
            e.message.clone(),
        ]));
    }
    out
}

/// The error report as JSONL.
#[must_use]
pub fn errors_jsonl(errors: &[RowError]) -> String {
    errors
        .iter()
        .map(|e| serde_json::to_string(e).unwrap_or_default() + "\n")
        .collect()
}

/// One CSV record, terminated by `\n`; each cell through [`crate::csv_cell::field`].
#[must_use]
pub fn csv_row(cells: &[String]) -> String {
    cells
        .iter()
        .map(|c| crate::csv_cell::field(c))
        .collect::<Vec<_>>()
        .join(",")
        + "\n"
}

/// One parsed CSV record and the physical line it starts on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvRecord {
    /// 1-based physical line of the record's first character.
    pub line: usize,
    /// The cells.
    pub cells: Vec<String>,
}

/// Parse RFC 4180 CSV: quoted fields, doubled quotes, `\n` or `\r\n`, and line
/// breaks inside quotes. A blank line is skipped.
///
/// # Errors
///
/// A message when a quoted field is never closed.
pub fn csv_parse(text: &str) -> Result<Vec<CsvRecord>, String> {
    let mut out: Vec<CsvRecord> = Vec::new();
    let mut cells: Vec<String> = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut line = 1usize;
    let mut record_line = 1usize;
    let mut chars = text.chars().peekable();
    let mut touched = false;
    while let Some(c) = chars.next() {
        if c == '\n' {
            line += 1;
        }
        if in_quotes {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    cell.push('"');
                    chars.next();
                }
                '"' => in_quotes = false,
                c => cell.push(c),
            }
            continue;
        }
        match c {
            '"' if cell.is_empty() => {
                in_quotes = true;
                touched = true;
            }
            ',' => {
                cells.push(std::mem::take(&mut cell));
                touched = true;
            }
            '\r' => {}
            '\n' => {
                if touched || !cell.is_empty() || !cells.is_empty() {
                    cells.push(std::mem::take(&mut cell));
                    out.push(CsvRecord {
                        line: record_line,
                        cells: std::mem::take(&mut cells),
                    });
                }
                touched = false;
                record_line = line;
            }
            c => {
                cell.push(c);
                touched = true;
            }
        }
    }
    if in_quotes {
        return Err("a quoted field is never closed".to_string());
    }
    if touched || !cell.is_empty() || !cells.is_empty() {
        cells.push(cell);
        out.push(CsvRecord {
            line: record_line,
            cells,
        });
    }
    Ok(out)
}

/// One decoded source row.
#[derive(Debug)]
pub struct Decoded {
    /// 1-based ordinal among data rows.
    pub row_number: usize,
    /// Physical line the row starts on.
    pub source_line: usize,
    /// The plan and its optional `pid`, or every problem found.
    pub result: Result<(Option<String>, Plan), Vec<RowError>>,
}

fn err(row: usize, line: usize, field: &str, code: &'static str, message: String) -> RowError {
    RowError {
        row_number: row,
        source_line: line,
        field: field.to_string(),
        code,
        message,
    }
}

fn plan_of(
    mut obj: Map<String, Value>,
    row: usize,
    line: usize,
) -> Result<(Option<String>, Plan), Vec<RowError>> {
    let pid = match obj.remove("pid") {
        Some(Value::String(s)) if !s.trim().is_empty() => Some(s.trim().to_string()),
        _ => None,
    };
    serde_json::from_value::<Plan>(Value::Object(obj))
        .map(|p| (pid, p))
        .map_err(|e| {
            // The serde message may quote a value; keep only its shape.
            let msg = e.to_string();
            let field = msg
                .split('`')
                .nth(1)
                .filter(|f| !f.contains(' '))
                .unwrap_or_default()
                .to_string();
            vec![err(
                row,
                line,
                &field,
                "decode",
                "the row is not a valid plan".to_string(),
            )]
        })
}

/// Decode a JSONL source: one plan object per non-blank line.
#[must_use]
pub fn decode_jsonl(text: &str) -> Vec<Decoded> {
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        let row = out.len() + 1;
        let line = i + 1;
        let result = match serde_json::from_str::<Value>(raw) {
            Ok(Value::Object(o)) => plan_of(o, row, line),
            Ok(_) => Err(vec![err(
                row,
                line,
                "",
                "decode",
                "each line must be a JSON object".into(),
            )]),
            Err(_) => Err(vec![err(
                row,
                line,
                "",
                "decode",
                "the line is not valid JSON".into(),
            )]),
        };
        out.push(Decoded {
            row_number: row,
            source_line: line,
            result,
        });
    }
    out
}

/// Decode a CSV source (header row required).
///
/// # Errors
///
/// A message when the CSV is malformed or has no header.
pub fn decode_csv(text: &str) -> Result<Vec<Decoded>, String> {
    let mut records = csv_parse(text)?.into_iter();
    let header = records
        .next()
        .ok_or_else(|| "the CSV has no header row".to_string())?;
    let columns: Vec<String> = header.cells.iter().map(|c| c.trim().to_string()).collect();
    let mut out = Vec::new();
    for rec in records {
        let row = out.len() + 1;
        let mut errors = Vec::new();
        let mut obj = Map::new();
        for (i, name) in columns.iter().enumerate() {
            // The `'` that export put before a formula-like cell is taken off again.
            let cell = crate::csv_cell::read(rec.cells.get(i).map_or("", String::as_str).trim());
            // An empty cell loses nothing, whatever its column.
            if cell.is_empty() {
                continue;
            }
            if !CSV_COLUMNS.contains(&name.as_str()) {
                // Reported once per row, never dropped silently.
                errors.push(err(
                    row,
                    rec.line,
                    name,
                    "unmapped_field",
                    "this column is not part of the plan format".into(),
                ));
                continue;
            }
            let value = if JSON_COLUMNS.contains(&name.as_str()) {
                let Ok(v) = serde_json::from_str::<Value>(cell) else {
                    errors.push(err(
                        row,
                        rec.line,
                        name,
                        "decode",
                        "the cell is not valid JSON".into(),
                    ));
                    continue;
                };
                v
            } else if cell.starts_with('{') || cell.starts_with('[') {
                serde_json::from_str(cell).unwrap_or_else(|_| Value::String(cell.to_string()))
            } else {
                Value::String(cell.to_string())
            };
            obj.insert(name.clone(), value);
        }
        let result = if errors.is_empty() {
            plan_of(obj, row, rec.line)
        } else {
            Err(errors)
        };
        out.push(Decoded {
            row_number: row,
            source_line: rec.line,
            result,
        });
    }
    Ok(out)
}

/// Encode plans (with their pids) as JSONL: the plan object plus `pid`.
#[must_use]
pub fn encode_jsonl(items: &[(Uuid, Value)]) -> String {
    items
        .iter()
        .map(|(pid, plan)| {
            let mut obj = plan.as_object().cloned().unwrap_or_default();
            obj.insert("pid".to_string(), Value::String(pid.to_string()));
            serde_json::to_string(&Value::Object(obj)).unwrap_or_default() + "\n"
        })
        .collect()
}

/// Encode plans (with their pids) as CSV, JSON-in-cell for nested fields.
#[must_use]
pub fn encode_csv(items: &[(Uuid, Value)]) -> String {
    let mut out = csv_row(
        &CSV_COLUMNS
            .iter()
            .map(|c| (*c).to_string())
            .collect::<Vec<_>>(),
    );
    for (pid, plan) in items {
        let cells: Vec<String> = CSV_COLUMNS
            .iter()
            .map(|col| {
                if *col == "pid" {
                    return pid.to_string();
                }
                match plan.get(*col) {
                    None | Some(Value::Null) => String::new(),
                    Some(Value::String(s)) if !JSON_COLUMNS.contains(col) => s.clone(),
                    Some(v) => {
                        // An empty list is an empty cell: nothing to carry.
                        if v.as_array().is_some_and(Vec::is_empty) {
                            String::new()
                        } else {
                            v.to_string()
                        }
                    }
                }
            })
            .collect();
        out.push_str(&csv_row(&cells));
    }
    out
}

/// The ways an existing plan may be identified, strongest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// The plan's own pid.
    Pid(Uuid),
    /// A globally-unique external identifier (`scheme` as its JSON name).
    Identifier {
        /// The scheme, as serialized (e.g. `JiraProjectKey`).
        scheme: String,
        /// The value.
        value: String,
    },
    /// An owner-scoped code — only meaningful with its owner.
    OwnerCode {
        /// The owning organization id.
        owner: String,
        /// The code.
        code: String,
    },
}

/// The keys a row offers, in precedence order: `pid`; every **deterministic**
/// identifier; then `(owner_org_id, code)` — and only when the owner is
/// present (a bare `code` is **not** a key). An empty result means keyless.
#[must_use]
pub fn keys_of(pid: Option<&str>, plan: &Plan) -> Vec<Key> {
    let mut out = Vec::new();
    if let Some(p) = pid.and_then(|p| Uuid::parse_str(p).ok()) {
        out.push(Key::Pid(p));
    }
    for id in &plan.identifiers {
        if id.scheme.is_deterministic() && !id.value.trim().is_empty() {
            let scheme = serde_json::to_value(&id.scheme)
                .ok()
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_default();
            out.push(Key::Identifier {
                scheme,
                value: id.value.trim().to_string(),
            });
        }
    }
    if let (Some(owner), Some(code)) = (plan.owner_org_id.as_deref(), plan.code.as_deref())
        && !owner.trim().is_empty()
        && !code.trim().is_empty()
    {
        out.push(Key::OwnerCode {
            owner: owner.trim().to_string(),
            code: code.trim().to_string(),
        });
    }
    out
}

/// The job's counters. They **always reconcile**.
#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq)]
pub struct Counters {
    /// Rows read.
    pub total: i64,
    /// Plans created.
    pub created: i64,
    /// Plans updated.
    pub updated: i64,
    /// Plans already identical.
    pub unchanged: i64,
    /// Rows held for review as probable duplicates.
    pub queued_for_review: i64,
    /// Rows that failed.
    pub failed: i64,
}

impl Counters {
    /// `created + updated + unchanged + queued_for_review + failed == total`.
    #[must_use]
    pub const fn reconciles(&self) -> bool {
        self.created + self.updated + self.unchanged + self.queued_for_review + self.failed
            == self.total
    }

    /// The status of a job that was cancelled part-way: always `cancelled`, never
    /// `completed`, whatever was finished — it says it stopped early.
    #[must_use]
    pub const fn cancelled_status() -> &'static str {
        "cancelled"
    }

    /// The terminal status of a job that ran to the end.
    #[must_use]
    pub const fn status(&self) -> &'static str {
        if self.failed > 0 {
            "completed_with_errors"
        } else {
            "completed"
        }
    }
}

/// How far a running job is, as a whole percent. It never reaches 100 while the job
/// is still running (100 means finished), is never below 1 once a row is done, and
/// is `0` for a job with nothing to do yet.
#[must_use]
pub fn progress_percent(done: i64, total: i64) -> i32 {
    if total <= 0 || done <= 0 {
        return 0;
    }
    let p = (done.min(total) * 100) / total;
    i32::try_from(p.clamp(1, 99)).unwrap_or(99)
}

/// How many rows are processed between looks at the cancel flag.
pub const CANCEL_CHECK_EVERY: usize = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_is_whole_monotone_and_never_100_until_finished() {
        assert_eq!(progress_percent(0, 10), 0);
        assert_eq!(progress_percent(5, 10), 50);
        assert_eq!(
            progress_percent(10, 10),
            99,
            "100 is reserved for a finished job"
        );
        assert_eq!(progress_percent(1, 10_000), 1, "the first row shows");
        assert_eq!(progress_percent(0, 0), 0);
        assert_eq!(progress_percent(5, 0), 0, "no total, no claim");
        assert_eq!(progress_percent(50, 10), 99, "never past the total");
        let mut last = 0;
        for done in 0..=200 {
            let p = progress_percent(done, 200);
            assert!(p >= last, "progress went backwards at {done}");
            last = p;
        }
    }

    use serde_json::json;

    fn rich() -> Value {
        json!({
            "kind": "Project", "name": "Apollo, \"phase 1\"\nline two",
            "alternate_names": ["Moon shot", "Артемида"], "code": "PROJ-1",
            "owner_org_id": "organization:3f2c6b7e-0000-4000-8000-0000000000aa",
            "lead_ref": "person:3f2c6b7e-0000-4000-8000-000000000001",
            "start_date": "2026-10-05", "keywords": ["space"], "tags": ["a", "b"],
            "goals": [{ "title": "Land" }]
        })
    }

    #[test]
    fn csv_quotes_commas_newlines_and_utf8_round_trip() {
        let row = vec![
            "a,b".to_string(),
            "say \"hi\"".to_string(),
            "two\nlines".to_string(),
            "Артемида".to_string(),
            String::new(),
        ];
        let parsed = csv_parse(&csv_row(&row)).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].cells, row);
    }

    #[test]
    fn csv_line_numbers_are_physical_and_a_blank_line_is_skipped() {
        let text = "h1,h2\r\nx,\"a\nb\"\r\n\r\ny,z\n";
        let r = csv_parse(text).unwrap();
        assert_eq!(
            r.iter().map(|x| x.line).collect::<Vec<_>>(),
            [1, 2, 5],
            "the multi-line cell shifts the next record"
        );
        assert_eq!(r.len(), 3);
        assert!(csv_parse("a,\"unterminated").is_err());
    }

    #[test]
    fn a_plan_survives_jsonl_and_csv_exactly() {
        let pid = Uuid::from_u128(7);
        let plan: Plan = serde_json::from_value(rich()).unwrap();
        let canonical = serde_json::to_value(&plan).unwrap();
        for text in [
            encode_jsonl(&[(pid, canonical.clone())]),
            encode_csv(&[(pid, canonical.clone())]),
        ] {
            let rows = if text.starts_with("pid,") {
                decode_csv(&text).unwrap()
            } else {
                decode_jsonl(&text)
            };
            assert_eq!(rows.len(), 1);
            let (got_pid, got) = rows.into_iter().next().unwrap().result.expect("decodes");
            assert_eq!(
                got_pid.as_deref(),
                Some(pid.to_string().as_str()),
                "the pid travels"
            );
            assert_eq!(serde_json::to_value(&got).unwrap(), canonical, "lossless");
        }
    }

    /// CQ-1: a formula-like name is written defused and comes back unchanged.
    #[test]
    fn a_formula_like_name_is_defused_in_csv_and_survives_import() {
        let pid = Uuid::from_u128(8);
        for name in [
            "=HYPERLINK(\"http://x\")",
            "@SUM(A1)",
            "'=already quoted",
            "-1",
        ] {
            let mut v = rich();
            v["name"] = json!(name);
            v["code"] = json!("+cmd");
            let plan: Plan = serde_json::from_value(v).unwrap();
            let canonical = serde_json::to_value(&plan).unwrap();
            let text = encode_csv(&[(pid, canonical.clone())]);
            let row = text.lines().nth(1).unwrap();
            assert!(
                row.split(',')
                    .all(|c| !c.trim_start_matches('"').starts_with(['=', '+', '@'])),
                "{row}"
            );
            let (_, got) = decode_csv(&text)
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
                .result
                .expect("decodes");
            assert_eq!(serde_json::to_value(&got).unwrap(), canonical, "{name}");
        }
    }

    #[test]
    fn bad_rows_are_errors_with_the_field_and_never_the_value() {
        let bad_json = decode_jsonl("{not json}\n[1]\n{\"name\":\"ok\"}\n");
        assert_eq!(bad_json.len(), 3);
        assert!(
            bad_json[0].result.is_err()
                && bad_json[1].result.is_err()
                && bad_json[2].result.is_ok()
        );
        let csv = "name,bogus,tags\nAlpha,x,\"[not json\"\nBeta,,\"[\"\"t\"\"]\"\n";
        let rows = decode_csv(csv).unwrap();
        let first = rows[0].result.as_ref().unwrap_err();
        let codes: Vec<(&str, &str)> = first.iter().map(|e| (e.field.as_str(), e.code)).collect();
        assert!(
            codes.contains(&("bogus", "unmapped_field")) && codes.contains(&("tags", "decode")),
            "{codes:?}"
        );
        assert!(
            first.iter().all(|e| !e.message.contains("not json")),
            "the report never echoes a value"
        );
        assert!(rows[1].result.is_ok(), "the next row is unaffected");
        assert_eq!(rows[1].source_line, 3);
        let report = errors_csv(first);
        assert!(report.starts_with("row_number,source_line,field,code,message\n"));
    }

    #[test]
    fn keys_follow_precedence_and_a_bare_code_is_not_a_key() {
        let mut p: Plan = serde_json::from_value(json!({
            "name": "x", "code": "C-1",
            "identifiers": [{ "scheme": "JiraProjectKey", "value": "PROJ" }, { "scheme": "Code", "value": "C-1" }]
        })).unwrap();
        let pid = Uuid::from_u128(1).to_string();
        let k = keys_of(Some(&pid), &p);
        assert_eq!(k[0], Key::Pid(Uuid::from_u128(1)));
        assert_eq!(
            k[1],
            Key::Identifier {
                scheme: "JiraProjectKey".into(),
                value: "PROJ".into()
            }
        );
        assert_eq!(
            k.len(),
            2,
            "an owner-scoped Code identifier and a bare code are not keys"
        );
        p.owner_org_id = Some("organization:x".into());
        assert_eq!(
            keys_of(None, &p).last(),
            Some(&Key::OwnerCode {
                owner: "organization:x".into(),
                code: "C-1".into()
            })
        );
        assert!(keys_of(None, &Plan::new("keyless")).is_empty());
    }

    #[test]
    fn counters_reconcile_and_name_the_terminal_status() {
        let ok = Counters {
            total: 5,
            created: 2,
            updated: 1,
            unchanged: 1,
            queued_for_review: 1,
            failed: 0,
        };
        assert!(ok.reconciles() && ok.status() == "completed");
        let bad = Counters {
            failed: 1,
            total: 6,
            ..ok
        };
        assert!(bad.reconciles() && bad.status() == "completed_with_errors");
        assert!(
            !Counters {
                total: 3,
                ..Counters::default()
            }
            .reconciles()
        );
    }
}

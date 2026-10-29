//! The portfolio ledger (EX-12). Pure rules; the database work is in [`crate::controllers::ledger`].
//!
//! Two things live here. **Reading a ledger export**: a small CSV reader for a finance system's export with
//! the columns `cost_code`, `amount_minor` and `currency` (other columns are ignored). **Reconciling it**: each
//! row's cost code is looked up in the map from cost codes to budget lines (the register's identifier); the
//! rows for a code are summed per currency and set beside what the register holds as spent. A code not in the
//! map is **listed as unmatched with its amount**, never dropped; a row in a different currency from its
//! line is a mismatch, and is not summed into it.

#![expect(missing_docs, clippy::many_single_char_names, clippy::implicit_hasher)]

use std::collections::{BTreeMap, HashMap};

use serde::Serialize;
use uuid::Uuid;

/// Most rows one import may carry.
pub const MAX_ROWS: usize = 5_000;

/// One row of the export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The line number in the file (the header is line 1).
    pub line: usize,
    /// The finance cost code.
    pub cost_code: String,
    /// The amount in minor units; negative for a credit.
    pub amount_minor: i64,
    /// The currency, upper case.
    pub currency: String,
}

/// Split one CSV line into fields: commas separate, and a field may be wrapped in double quotes (with `""` for
/// a quote inside).
fn fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                cur.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => out.push(std::mem::take(&mut cur)),
            (c, _) => cur.push(c),
        }
    }
    out.push(cur);
    out.into_iter().map(|f| f.trim().to_string()).collect()
}

/// Read an export. The first non-empty line is the header and must name `cost_code`, `amount_minor` and
/// `currency`. Every problem is reported with its line number; nothing is guessed.
///
/// # Errors
///
/// The list of problems, when the header is wrong, a row is malformed, or there are too many rows.
pub fn parse(text: &str) -> Result<Vec<Row>, Vec<String>> {
    let mut lines = text
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty());
    let Some((_, header)) = lines.next() else {
        return Err(vec!["the file is empty".to_string()]);
    };
    let head: Vec<String> = fields(header)
        .into_iter()
        .map(|h| h.to_lowercase())
        .collect();
    let col = |name: &str| head.iter().position(|h| h == name);
    let (Some(c), Some(a), Some(u)) = (col("cost_code"), col("amount_minor"), col("currency"))
    else {
        return Err(vec![
            "the header must name cost_code, amount_minor and currency".to_string(),
        ]);
    };
    let mut rows = Vec::new();
    let mut problems = Vec::new();
    for (i, l) in lines {
        let f = fields(l);
        let n = i + 1;
        let get = |ix: usize| f.get(ix).map_or("", String::as_str);
        let code = get(c);
        let currency = get(u).to_uppercase();
        let amount = get(a).parse::<i64>();
        if code.is_empty() || code.chars().count() > 60 {
            problems.push(format!("line {n}: cost_code is 1 to 60 characters"));
        } else if amount.is_err() {
            problems.push(format!(
                "line {n}: amount_minor is not a whole number of minor units"
            ));
        } else if !crate::currency::is_code(&currency) {
            problems.push(format!("line {n}: currency is a three-letter code"));
        } else {
            rows.push(Row {
                line: n,
                cost_code: code.to_string(),
                amount_minor: amount.unwrap_or(0),
                currency,
            });
        }
        if rows.len() + problems.len() > MAX_ROWS {
            return Err(vec![format!("more than {MAX_ROWS} rows: split the file")]);
        }
    }
    if problems.is_empty() {
        Ok(rows)
    } else {
        Err(problems)
    }
}

/// What the register holds for a mapped cost code.
#[derive(Debug, Clone)]
pub struct Line {
    /// The budget line.
    pub line_pid: Uuid,
    /// Its currency.
    pub currency: String,
    /// Spent so far, in minor units.
    pub actual_minor: i64,
}

/// A mapped cost code with the ledger's total and the register's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Matched {
    pub cost_code: String,
    pub line_pid: Uuid,
    pub currency: String,
    pub ledger_minor: i64,
    pub register_minor: i64,
    /// Ledger less register; zero means they agree.
    pub difference_minor: i64,
    pub rows: usize,
}

/// A cost code the map does not know.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Unmatched {
    pub cost_code: String,
    pub currency: String,
    pub amount_minor: i64,
    pub rows: usize,
}

/// A row whose currency is not its line's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Mismatch {
    pub cost_code: String,
    pub row_currency: String,
    pub line_currency: String,
    pub amount_minor: i64,
}

/// The reconciliation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct Report {
    pub matched: Vec<Matched>,
    pub unmatched: Vec<Unmatched>,
    pub currency_mismatches: Vec<Mismatch>,
    /// Whether every row matched a line, in the line's currency, with no difference.
    pub reconciled: bool,
}

/// Reconcile `rows` against `map` (lower-cased cost code to line).
#[must_use]
pub fn reconcile(rows: &[Row], map: &HashMap<String, Line>) -> Report {
    let mut matched: BTreeMap<String, Matched> = BTreeMap::new();
    let mut unmatched: BTreeMap<(String, String), Unmatched> = BTreeMap::new();
    let mut mismatches = Vec::new();
    for r in rows {
        let key = r.cost_code.to_lowercase();
        match map.get(&key) {
            None => {
                let u = unmatched
                    .entry((key, r.currency.clone()))
                    .or_insert_with(|| Unmatched {
                        cost_code: r.cost_code.clone(),
                        currency: r.currency.clone(),
                        amount_minor: 0,
                        rows: 0,
                    });
                u.amount_minor += r.amount_minor;
                u.rows += 1;
            }
            Some(l) if l.currency != r.currency => mismatches.push(Mismatch {
                cost_code: r.cost_code.clone(),
                row_currency: r.currency.clone(),
                line_currency: l.currency.clone(),
                amount_minor: r.amount_minor,
            }),
            Some(l) => {
                let m = matched.entry(key).or_insert_with(|| Matched {
                    cost_code: r.cost_code.clone(),
                    line_pid: l.line_pid,
                    currency: l.currency.clone(),
                    ledger_minor: 0,
                    register_minor: l.actual_minor,
                    difference_minor: 0,
                    rows: 0,
                });
                m.ledger_minor += r.amount_minor;
                m.rows += 1;
            }
        }
    }
    let mut matched: Vec<Matched> = matched.into_values().collect();
    for m in &mut matched {
        m.difference_minor = m.ledger_minor - m.register_minor;
    }
    let unmatched: Vec<Unmatched> = unmatched.into_values().collect();
    let reconciled = unmatched.is_empty()
        && mismatches.is_empty()
        && matched.iter().all(|m| m.difference_minor == 0);
    Report {
        matched,
        unmatched,
        currency_mismatches: mismatches,
        reconciled,
    }
}

/// Untraced money: what was allocated less what budget lines drawing on it plan, per currency. Positive is
/// money not yet attached to any line; negative is lines planned beyond the allocation.
#[must_use]
pub fn untraced(allocated_minor: i64, planned_on_lines_minor: i64) -> i64 {
    allocated_minor - planned_on_lines_minor
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(cur: &str, actual: i64) -> Line {
        Line {
            line_pid: Uuid::new_v4(),
            currency: cur.into(),
            actual_minor: actual,
        }
    }

    #[test]
    fn a_csv_is_read_with_quotes_extra_columns_and_credits() {
        let rows = parse("period, Cost_Code ,amount_minor,currency\n2026-09,\"CC-1,A\",12500,gbp\n2026-09,CC-2,-300,GBP\n\n").unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            (
                rows[0].cost_code.as_str(),
                rows[0].amount_minor,
                rows[0].currency.as_str(),
                rows[0].line
            ),
            ("CC-1,A", 12500, "GBP", 2)
        );
        assert_eq!(rows[1].amount_minor, -300, "a credit");
    }

    #[test]
    fn every_problem_is_reported_with_its_line() {
        assert_eq!(
            parse("a,b\n1,2").unwrap_err(),
            ["the header must name cost_code, amount_minor and currency"]
        );
        let e = parse("cost_code,amount_minor,currency\nCC-1,12.5,GBP\n,5,GBP\nCC-3,5,pounds")
            .unwrap_err();
        assert_eq!(e.len(), 3);
        assert!(
            e[0].starts_with("line 2:")
                && e[1].starts_with("line 3:")
                && e[2].starts_with("line 4:"),
            "{e:?}"
        );
        assert!(parse("").is_err());
    }

    #[test]
    fn matched_codes_reconcile_and_unmatched_ones_are_listed_with_their_amount() {
        let map = HashMap::from([
            ("cc-1".to_string(), line("GBP", 15_000)),
            ("cc-2".to_string(), line("GBP", 100)),
        ]);
        let rows = parse("cost_code,amount_minor,currency\nCC-1,10000,GBP\ncc-1,5000,GBP\nCC-9,700,GBP\nCC-9,300,GBP\nCC-2,100,GBP").unwrap();
        let r = reconcile(&rows, &map);
        assert_eq!(r.matched.len(), 2);
        assert!(r.matched.iter().all(|m| m.difference_minor == 0), "{r:?}");
        assert_eq!(
            r.unmatched,
            [Unmatched {
                cost_code: "CC-9".into(),
                currency: "GBP".into(),
                amount_minor: 1000,
                rows: 2
            }]
        );
        assert!(
            !r.reconciled,
            "an unmatched code means it does not reconcile"
        );
        let clean = reconcile(&rows[..2], &map);
        assert!(clean.reconciled);
    }

    #[test]
    fn a_difference_and_a_currency_mismatch_are_reported_not_hidden() {
        let map = HashMap::from([("cc-1".to_string(), line("GBP", 10_000))]);
        let rows = parse("cost_code,amount_minor,currency\nCC-1,9000,GBP\nCC-1,500,USD").unwrap();
        let r = reconcile(&rows, &map);
        assert_eq!(r.matched[0].difference_minor, -1000);
        assert_eq!(r.currency_mismatches.len(), 1);
        assert_eq!(r.currency_mismatches[0].line_currency, "GBP");
        assert!(!r.reconciled);
    }

    #[test]
    fn untraced_money_is_the_allocation_less_what_lines_plan() {
        assert_eq!(untraced(1_000_000, 750_000), 250_000);
        assert_eq!(untraced(1_000_000, 1_000_000), 0);
        assert_eq!(
            untraced(1_000_000, 1_200_000),
            -200_000,
            "lines planned beyond the allocation"
        );
    }
}

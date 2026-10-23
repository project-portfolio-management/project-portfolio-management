//! Financial years and the money view grouped by funding source (DP-6). Pure rules; the database work
//! is in [`crate::controllers::funding`].
//!
//! A financial year starts on the first of a configured month (default April, so 1 April to 31 March).
//! A budget line belongs to the year its **period start** falls in, or its period end when it has no
//! start; a line with neither is **undated** and sits outside every year. Currencies are never summed,
//! and capital (`capex`) and revenue (`opex`) are never summed unless the caller asks.

#![expect(missing_docs)]

use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};
use uuid::Uuid;

/// The month a financial year starts in: `PROJECT_PORTFOLIO_MANAGEMENT_FINANCIAL_YEAR_START_MONTH`,
/// 1 to 12, default 4 (April).
#[must_use]
pub fn start_month() -> u32 {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_FINANCIAL_YEAR_START_MONTH")
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|m| (1..=12).contains(m))
        .unwrap_or(4)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Year {
    pub start: NaiveDate,
    /// The last day of the year.
    pub end: NaiveDate,
    /// `2026` for a calendar year, `2026/27` otherwise.
    pub label: String,
}

/// The financial year a date falls in.
///
/// # Panics
///
/// Never for a real date: the dates built are the first of a month.
#[must_use]
pub fn year_of(date: NaiveDate, start_month: u32) -> Year {
    let y = if date.month() >= start_month {
        date.year()
    } else {
        date.year() - 1
    };
    let start = NaiveDate::from_ymd_opt(y, start_month, 1).expect("first of a month");
    let next = NaiveDate::from_ymd_opt(y + 1, start_month, 1).expect("first of a month");
    let label = if start_month == 1 {
        y.to_string()
    } else {
        format!("{y}/{:02}", (y + 1) % 100)
    };
    Year {
        start,
        end: next.pred_opt().expect("a day before"),
        label,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    InYear,
    OtherYear,
    Undated,
}

/// Where a line with this period sits against a year.
#[must_use]
pub fn placement(start: Option<NaiveDate>, end: Option<NaiveDate>, year: &Year) -> Placement {
    match start.or(end) {
        None => Placement::Undated,
        Some(d) if d >= year.start && d <= year.end => Placement::InYear,
        Some(_) => Placement::OtherYear,
    }
}

/// Whether a line's period runs past the end of the year it is placed in.
#[must_use]
pub fn spans_years(start: Option<NaiveDate>, end: Option<NaiveDate>, year: &Year) -> bool {
    start.is_some() && end.is_some_and(|e| e > year.end)
}

/// A budget line, reduced to what the view needs.
#[derive(Debug, Clone)]
pub struct Line {
    pub programme: Uuid,
    pub source: Option<Uuid>,
    /// `capex` or `opex`.
    pub class: String,
    pub currency: String,
    pub planned_minor: i64,
    pub actual_minor: i64,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
}

/// One row: a programme, a funding source (or none), a class and a currency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub programme: Uuid,
    pub source: Option<Uuid>,
    /// `capex`, `opex`, or `all` when the caller asked for the classes to be combined.
    pub class: String,
    pub currency: String,
    pub budget_minor: i64,
    pub spend_minor: i64,
    pub lines: usize,
}

impl Row {
    #[must_use]
    pub fn variance_minor(&self) -> i64 {
        self.budget_minor - self.spend_minor
    }
}

/// What was set aside, and why.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Quality {
    pub undated_lines: usize,
    pub other_year_lines: usize,
    pub spanning_lines: usize,
    pub no_source_lines: usize,
}

/// Group lines for one financial year. Rows are ordered by programme, source, class, currency.
#[must_use]
pub fn group(lines: &[Line], year: &Year, combine_classes: bool) -> (Vec<Row>, Quality) {
    let mut q = Quality::default();
    let mut rows: BTreeMap<(Uuid, Option<Uuid>, String, String), Row> = BTreeMap::new();
    for l in lines {
        match placement(l.period_start, l.period_end, year) {
            Placement::Undated => {
                q.undated_lines += 1;
                continue;
            }
            Placement::OtherYear => {
                q.other_year_lines += 1;
                continue;
            }
            Placement::InYear => {}
        }
        if spans_years(l.period_start, l.period_end, year) {
            q.spanning_lines += 1;
        }
        if l.source.is_none() {
            q.no_source_lines += 1;
        }
        let class = if combine_classes {
            "all".to_string()
        } else {
            l.class.clone()
        };
        let r = rows
            .entry((l.programme, l.source, class.clone(), l.currency.clone()))
            .or_insert_with(|| Row {
                programme: l.programme,
                source: l.source,
                class,
                currency: l.currency.clone(),
                budget_minor: 0,
                spend_minor: 0,
                lines: 0,
            });
        r.budget_minor += l.planned_minor;
        r.spend_minor += l.actual_minor;
        r.lines += 1;
    }
    (rows.into_values().collect(), q)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }
    #[expect(clippy::too_many_arguments)]
    fn line(
        prog: Uuid,
        source: Option<Uuid>,
        class: &str,
        cur: &str,
        plan: i64,
        act: i64,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Line {
        Line {
            programme: prog,
            source,
            class: class.into(),
            currency: cur.into(),
            planned_minor: plan,
            actual_minor: act,
            period_start: from.map(d),
            period_end: to.map(d),
        }
    }

    #[test]
    fn the_boundary_is_the_last_of_march_and_the_first_of_april() {
        let before = year_of(d("2026-03-31"), 4);
        let after = year_of(d("2026-04-01"), 4);
        assert_eq!(
            (before.start, before.end, before.label.as_str()),
            (d("2025-04-01"), d("2026-03-31"), "2025/26")
        );
        assert_eq!(
            (after.start, after.end, after.label.as_str()),
            (d("2026-04-01"), d("2027-03-31"), "2026/27")
        );
        assert_eq!(
            placement(Some(d("2026-03-31")), None, &after),
            Placement::OtherYear
        );
        assert_eq!(
            placement(Some(d("2026-04-01")), None, &after),
            Placement::InYear
        );
        assert_eq!(
            placement(Some(d("2027-03-31")), None, &after),
            Placement::InYear
        );
        assert_eq!(
            placement(Some(d("2027-04-01")), None, &after),
            Placement::OtherYear
        );
    }

    #[test]
    fn a_calendar_year_and_a_leap_day() {
        let y = year_of(d("2028-02-29"), 1);
        assert_eq!(
            (y.start, y.end, y.label.as_str()),
            (d("2028-01-01"), d("2028-12-31"), "2028")
        );
        let y = year_of(d("2028-02-29"), 4);
        assert_eq!(y.end, d("2028-03-31"));
        assert_eq!(
            year_of(d("2099-12-31"), 4).label,
            "2099/00",
            "the century rolls over"
        );
    }

    #[test]
    fn a_line_with_only_an_end_is_placed_by_its_end_and_none_is_undated() {
        let y = year_of(d("2026-06-01"), 4);
        assert_eq!(
            placement(None, Some(d("2026-09-01")), &y),
            Placement::InYear
        );
        assert_eq!(placement(None, None, &y), Placement::Undated);
        assert!(spans_years(
            Some(d("2026-10-01")),
            Some(d("2027-06-30")),
            &y
        ));
        assert!(!spans_years(
            Some(d("2026-10-01")),
            Some(d("2027-03-31")),
            &y
        ));
        assert!(
            !spans_years(None, Some(d("2027-06-30")), &y),
            "no start, no spanning claim"
        );
    }

    #[test]
    fn currencies_and_classes_are_never_summed_unless_asked() {
        let (p, s) = (Uuid::from_u128(1), Some(Uuid::from_u128(9)));
        let y = year_of(d("2026-06-01"), 4);
        let lines = [
            line(p, s, "capex", "GBP", 100, 40, Some("2026-05-01"), None),
            line(p, s, "opex", "GBP", 50, 10, Some("2026-05-01"), None),
            line(p, s, "capex", "EUR", 7, 1, Some("2026-05-01"), None),
            line(p, None, "opex", "GBP", 5, 5, Some("2026-05-01"), None),
        ];
        let (rows, q) = group(&lines, &y, false);
        assert_eq!(rows.len(), 4);
        assert_eq!(q.no_source_lines, 1);
        let capex_gbp = rows
            .iter()
            .find(|r| r.class == "capex" && r.currency == "GBP")
            .unwrap();
        assert_eq!(
            (
                capex_gbp.budget_minor,
                capex_gbp.spend_minor,
                capex_gbp.variance_minor()
            ),
            (100, 40, 60)
        );
        let (combined, _) = group(&lines, &y, true);
        let all_gbp = combined
            .iter()
            .find(|r| r.source == s && r.currency == "GBP")
            .unwrap();
        assert_eq!(
            (all_gbp.class.as_str(), all_gbp.budget_minor, all_gbp.lines),
            ("all", 150, 2)
        );
        assert!(
            combined
                .iter()
                .any(|r| r.currency == "EUR" && r.budget_minor == 7),
            "euros stay apart"
        );
    }

    #[test]
    fn lines_outside_the_year_or_undated_are_counted_not_dropped_silently() {
        let p = Uuid::from_u128(1);
        let y = year_of(d("2026-06-01"), 4);
        let lines = [
            line(p, None, "opex", "GBP", 1, 0, Some("2026-03-31"), None),
            line(p, None, "opex", "GBP", 1, 0, None, None),
            line(
                p,
                None,
                "opex",
                "GBP",
                1,
                0,
                Some("2026-12-01"),
                Some("2027-08-01"),
            ),
        ];
        let (rows, q) = group(&lines, &y, false);
        assert_eq!(
            (q.other_year_lines, q.undated_lines, q.spanning_lines),
            (1, 1, 1)
        );
        assert_eq!(rows.len(), 1);
    }
}

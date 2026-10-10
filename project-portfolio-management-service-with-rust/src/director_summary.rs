//! Issues and change requests for the monthly director pack (IR-6): the pure rules.
//!
//! Two rules from the pack's brief hold throughout: a figure that cannot be known is **absent with a
//! reason**, never a zero, and "nothing recorded" is said in words. A reader who can see no plan at all
//! gets `available: false` and the reason, not a page of zeros that look like good news.

use chrono::NaiveDate;
use serde::Serialize;

/// What the summary needs of an issue.
#[derive(Debug, Clone)]
pub struct IssueLite {
    /// Plan it belongs to.
    pub plan_pid: String,
    /// Title.
    pub title: String,
    /// `low`, `med`, `high` or `critical`.
    pub severity: String,
    /// `open`, `in_progress`, `resolved` or `closed`.
    pub status: String,
    /// When raised.
    pub raised_on: NaiveDate,
    /// When resolved or closed.
    pub resolved_on: Option<NaiveDate>,
    /// When due.
    pub due_date: Option<NaiveDate>,
}

/// What the summary needs of a change request.
#[derive(Debug, Clone)]
pub struct ChangeLite {
    /// Plan it belongs to.
    pub plan_pid: String,
    /// Title.
    pub title: String,
    /// Its status.
    pub status: String,
    /// When requested.
    pub requested_on: NaiveDate,
    /// When decided.
    pub decided_on: Option<NaiveDate>,
}

/// The oldest open item, named.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Oldest {
    /// Plan.
    pub plan_pid: String,
    /// Title.
    pub title: String,
    /// Calendar days since raised or requested.
    pub age_calendar_days: i64,
}

/// The issues half.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IssuesSummary {
    /// Every issue in the readable plans.
    pub total: usize,
    /// Not resolved or closed.
    pub open: usize,
    /// Open, by severity (every severity listed, zeros included, because these are real counts).
    pub open_by_severity: std::collections::BTreeMap<String, usize>,
    /// Open and past due.
    pub overdue: usize,
    /// Raised within the window.
    pub raised_in_window: usize,
    /// Resolved or closed within the window.
    pub resolved_in_window: usize,
    /// The oldest open issue.
    pub oldest_open: Option<Oldest>,
    /// A sentence for the reader.
    pub statement: String,
}

/// The change requests half.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ChangesSummary {
    /// Every change request in the readable plans.
    pub total: usize,
    /// By status (every status listed).
    pub by_status: std::collections::BTreeMap<String, usize>,
    /// Submitted or under review: waiting for the approver.
    pub awaiting_decision: usize,
    /// Approved and not yet implemented.
    pub approved_not_implemented: usize,
    /// Decided (approved or rejected) within the window.
    pub decided_in_window: usize,
    /// The oldest request still open.
    pub oldest_open: Option<Oldest>,
    /// A sentence for the reader.
    pub statement: String,
}

const SEVERITIES: [&str; 4] = ["critical", "high", "med", "low"];
const CR_STATUSES: [&str; 7] = [
    "draft",
    "submitted",
    "under_review",
    "approved",
    "rejected",
    "implemented",
    "withdrawn",
];

fn in_window(d: NaiveDate, today: NaiveDate, window_days: i64) -> bool {
    d <= today && (today - d).num_days() < window_days
}

/// Summarise the issues of the plans a reader can see.
#[must_use]
pub fn issues(rows: &[IssueLite], today: NaiveDate, window_days: i64) -> IssuesSummary {
    let is_done = |s: &str| matches!(s, "resolved" | "closed");
    let open: Vec<&IssueLite> = rows.iter().filter(|i| !is_done(&i.status)).collect();
    let mut by: std::collections::BTreeMap<String, usize> =
        SEVERITIES.iter().map(|s| ((*s).to_string(), 0)).collect();
    for i in &open {
        if let Some(c) = by.get_mut(&i.severity) {
            *c += 1;
        }
    }
    let oldest_open = open
        .iter()
        .max_by_key(|i| {
            (
                (today - i.raised_on).num_days(),
                std::cmp::Reverse(&i.title),
            )
        })
        .map(|i| Oldest {
            plan_pid: i.plan_pid.clone(),
            title: i.title.clone(),
            age_calendar_days: (today - i.raised_on).num_days().max(0),
        });
    let statement = if rows.is_empty() {
        "No issues recorded.".to_string()
    } else if open.is_empty() {
        "No open issues.".to_string()
    } else {
        format!("{} open, {} of them critical.", open.len(), by["critical"])
    };
    IssuesSummary {
        total: rows.len(),
        open: open.len(),
        open_by_severity: by,
        overdue: open
            .iter()
            .filter(|i| i.due_date.is_some_and(|d| d < today))
            .count(),
        raised_in_window: rows
            .iter()
            .filter(|i| in_window(i.raised_on, today, window_days))
            .count(),
        resolved_in_window: rows
            .iter()
            .filter(|i| {
                i.resolved_on
                    .is_some_and(|d| in_window(d, today, window_days))
            })
            .count(),
        oldest_open,
        statement,
    }
}

/// Summarise the change requests of the plans a reader can see.
#[must_use]
pub fn changes(rows: &[ChangeLite], today: NaiveDate, window_days: i64) -> ChangesSummary {
    let mut by: std::collections::BTreeMap<String, usize> =
        CR_STATUSES.iter().map(|s| ((*s).to_string(), 0)).collect();
    for r in rows {
        if let Some(c) = by.get_mut(&r.status) {
            *c += 1;
        }
    }
    let open = |s: &str| matches!(s, "draft" | "submitted" | "under_review" | "approved");
    let oldest_open = rows
        .iter()
        .filter(|r| open(&r.status))
        .max_by_key(|r| {
            (
                (today - r.requested_on).num_days(),
                std::cmp::Reverse(&r.title),
            )
        })
        .map(|r| Oldest {
            plan_pid: r.plan_pid.clone(),
            title: r.title.clone(),
            age_calendar_days: (today - r.requested_on).num_days().max(0),
        });
    let awaiting = by["submitted"] + by["under_review"];
    let statement = if rows.is_empty() {
        "No change requests recorded.".to_string()
    } else if awaiting == 0 {
        "No change requests are waiting for a decision.".to_string()
    } else {
        format!("{awaiting} waiting for a decision.")
    };
    ChangesSummary {
        total: rows.len(),
        awaiting_decision: awaiting,
        approved_not_implemented: by["approved"],
        decided_in_window: rows
            .iter()
            .filter(|r| {
                matches!(r.status.as_str(), "approved" | "rejected" | "implemented")
                    && r.decided_on
                        .is_some_and(|d| in_window(d, today, window_days))
            })
            .count(),
        by_status: by,
        oldest_open,
        statement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, day).unwrap()
    }
    fn issue(title: &str, sev: &str, status: &str, raised: NaiveDate) -> IssueLite {
        IssueLite {
            plan_pid: "p".into(),
            title: title.into(),
            severity: sev.into(),
            status: status.into(),
            raised_on: raised,
            resolved_on: None,
            due_date: None,
        }
    }

    #[test]
    fn nothing_recorded_is_said_in_words_and_with_real_zeros() {
        let s = issues(&[], d(10, 10), 30);
        assert_eq!(s.statement, "No issues recorded.");
        assert_eq!(s.total, 0);
        assert!(s.oldest_open.is_none());
        assert_eq!(s.open_by_severity.len(), 4, "every severity is listed");
        let c = changes(&[], d(10, 10), 30);
        assert_eq!(c.statement, "No change requests recorded.");
        assert_eq!(c.by_status.len(), 7);
    }

    #[test]
    fn open_counts_severity_overdue_and_the_oldest_is_named() {
        let mut late = issue("Build down", "critical", "open", d(9, 1));
        late.due_date = Some(d(9, 20));
        let rows = vec![
            late,
            issue("Typo", "low", "in_progress", d(10, 5)),
            issue("Old and done", "high", "closed", d(1, 1)),
        ];
        let s = issues(&rows, d(10, 10), 30);
        assert_eq!((s.total, s.open, s.overdue), (3, 2, 1));
        assert_eq!(s.open_by_severity["critical"], 1);
        assert_eq!(s.open_by_severity["high"], 0, "a closed issue is not open");
        assert_eq!(s.statement, "2 open, 1 of them critical.");
        let o = s.oldest_open.unwrap();
        assert_eq!((o.title.as_str(), o.age_calendar_days), ("Build down", 39));
    }

    #[test]
    fn an_all_closed_register_says_so_rather_than_looking_empty() {
        let s = issues(&[issue("Done", "med", "closed", d(8, 1))], d(10, 10), 30);
        assert_eq!(s.statement, "No open issues.");
        assert_eq!(s.total, 1);
    }

    #[test]
    fn the_window_counts_raised_and_resolved_by_calendar_day() {
        let mut a = issue("A", "low", "closed", d(9, 20));
        a.resolved_on = Some(d(10, 9));
        let b = issue("B", "low", "open", d(8, 1));
        let s = issues(&[a, b], d(10, 10), 30);
        assert_eq!((s.raised_in_window, s.resolved_in_window), (1, 1));
        // The edge: exactly `window_days` ago is outside.
        let edge = issue("E", "low", "open", d(9, 10));
        assert_eq!(issues(&[edge], d(10, 10), 30).raised_in_window, 0);
    }

    #[test]
    fn changes_waiting_and_approved_but_not_done_are_counted_apart() {
        let cr = |t: &str, st: &str, req: NaiveDate, dec: Option<NaiveDate>| ChangeLite {
            plan_pid: "p".into(),
            title: t.into(),
            status: st.into(),
            requested_on: req,
            decided_on: dec,
        };
        let rows = vec![
            cr("Waiting", "submitted", d(9, 1), None),
            cr("Reviewing", "under_review", d(10, 1), None),
            cr("Approved", "approved", d(9, 15), Some(d(10, 2))),
            cr("Done", "implemented", d(7, 1), Some(d(7, 20))),
            cr("No", "rejected", d(10, 3), Some(d(10, 8))),
        ];
        let s = changes(&rows, d(10, 10), 30);
        assert_eq!(s.awaiting_decision, 2);
        assert_eq!(s.approved_not_implemented, 1);
        assert_eq!(
            s.decided_in_window, 2,
            "approved on 2 Oct and rejected on 8 Oct"
        );
        assert_eq!(s.statement, "2 waiting for a decision.");
        let o = s.oldest_open.unwrap();
        assert_eq!((o.title.as_str(), o.age_calendar_days), ("Waiting", 39));
    }
}

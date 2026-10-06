//! Controlled documents (PL-6): the pure rules. DB-free and deterministic.
//!
//! A document's **versions** move `draft → in_review → approved → effective →
//! superseded`:
//!
//! * a draft is edited freely; once it leaves draft its text never changes (an edit
//!   is the *next* draft);
//! * it is submitted for review, and an **approval** (PL-1) moves it to `approved`
//!   (a rejection sends it back to draft);
//! * only an approved version — one whose approval request was in fact approved —
//!   becomes `effective`, **from today** (no back-dating, no future-dating), and in
//!   the same step the version it replaces is `superseded` on that day, naming its
//!   successor;
//! * so at any date **exactly one version, or none, is in force** ([`in_force`]).

use chrono::NaiveDate;
use serde::Serialize;

/// Where a version is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Being written; the only status whose text may change.
    Draft,
    /// Submitted; an approval request is out.
    InReview,
    /// Approved, not yet in force.
    Approved,
    /// In force.
    Effective,
    /// Replaced by a later effective version.
    Superseded,
}

impl Status {
    /// Parse the stored token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(Self::Draft),
            "in_review" => Some(Self::InReview),
            "approved" => Some(Self::Approved),
            "effective" => Some(Self::Effective),
            "superseded" => Some(Self::Superseded),
            _ => None,
        }
    }

    /// The stored token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::InReview => "in_review",
            Self::Approved => "approved",
            Self::Effective => "effective",
            Self::Superseded => "superseded",
        }
    }

    /// Whether the text may still change.
    #[must_use]
    pub const fn is_editable(self) -> bool {
        matches!(self, Self::Draft)
    }
}

/// Whether a version may move from one status to another. The only way back is
/// `in_review → draft` (a rejected or withdrawn review); nothing leaves `superseded`.
///
/// # Errors
///
/// A sentence saying why not.
pub fn check_move(from: Status, to: Status) -> Result<(), String> {
    use Status::{Approved, Draft, Effective, InReview, Superseded};
    match (from, to) {
        (Draft, InReview)
        | (InReview, Approved | Draft)
        | (Approved, Effective)
        | (Effective, Superseded) => Ok(()),
        (f, t) => Err(format!(
            "a version cannot go from {} to {}",
            f.token(),
            t.token()
        )),
    }
}

/// Problems with a document's or version's text (empty when fine).
#[must_use]
pub fn problems(title: Option<&str>, body: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    if title.is_some_and(|t| t.trim().is_empty() || t.chars().count() > 200) {
        out.push("title is required (up to 200 characters)".to_string());
    }
    if body.is_some_and(|b| b.chars().count() > 200_000) {
        out.push("body is at most 200000 characters".to_string());
    }
    out
}

/// The date a version becomes effective: today. It may not be earlier than the day the
/// version it replaces became effective (a later one never starts before an earlier one).
///
/// # Errors
///
/// A sentence saying why not.
pub fn effective_date(
    today: NaiveDate,
    previous_from: Option<NaiveDate>,
) -> Result<NaiveDate, String> {
    match previous_from {
        Some(p) if today < p => Err(format!(
            "the version in force became effective on {p}, which is after today"
        )),
        _ => Ok(today),
    }
}

/// When a version was in force: from `effective_from`, until (not including) `superseded_on`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The version number.
    pub version: i32,
    /// The first day it was in force.
    pub effective_from: NaiveDate,
    /// The day its successor took over; `None` while it is still in force.
    pub superseded_on: Option<NaiveDate>,
}

/// The version in force on `date`, or `None`. At most one is, by construction; if the
/// spans somehow overlapped this answers `None` rather than choosing.
#[must_use]
pub fn in_force(spans: &[Span], date: NaiveDate) -> Option<i32> {
    let mut hit = spans
        .iter()
        .filter(|s| s.effective_from <= date && s.superseded_on.is_none_or(|e| date < e));
    let first = hit.next()?;
    hit.next().is_none().then_some(first.version)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, day).unwrap()
    }

    #[test]
    fn the_moves_are_a_line_with_one_way_back() {
        use Status::{Approved, Draft, Effective, InReview, Superseded};
        for (a, b) in [
            (Draft, InReview),
            (InReview, Approved),
            (InReview, Draft),
            (Approved, Effective),
            (Effective, Superseded),
        ] {
            assert!(check_move(a, b).is_ok(), "{a:?} -> {b:?}");
        }
        for (a, b) in [
            (Draft, Approved),
            (Draft, Effective),
            (InReview, Effective),
            (Approved, Draft),
            (Approved, InReview),
            (Effective, Draft),
            (Effective, Approved),
            (Superseded, Effective),
            (Superseded, Draft),
            (Draft, Draft),
        ] {
            assert!(check_move(a, b).is_err(), "{a:?} -> {b:?} must be refused");
        }
        for s in [Draft, InReview, Approved, Effective, Superseded] {
            assert_eq!(Status::parse(s.token()), Some(s));
        }
        assert_eq!(Status::parse("published"), None);
    }

    #[test]
    fn only_a_draft_may_be_edited() {
        assert!(Status::Draft.is_editable());
        for s in [
            Status::InReview,
            Status::Approved,
            Status::Effective,
            Status::Superseded,
        ] {
            assert!(!s.is_editable(), "{s:?}");
        }
    }

    #[test]
    fn a_version_becomes_effective_today_and_never_before_its_predecessor() {
        assert_eq!(effective_date(d(10), None), Ok(d(10)));
        assert_eq!(effective_date(d(10), Some(d(5))), Ok(d(10)));
        assert_eq!(effective_date(d(10), Some(d(10))), Ok(d(10)));
        assert!(effective_date(d(10), Some(d(11))).is_err());
    }

    #[test]
    fn exactly_one_version_or_none_is_in_force_on_any_date() {
        // v1 from the 5th, replaced on the 12th by v2, replaced on the 20th by v3 (still in force).
        let spans = [
            Span {
                version: 1,
                effective_from: d(5),
                superseded_on: Some(d(12)),
            },
            Span {
                version: 2,
                effective_from: d(12),
                superseded_on: Some(d(20)),
            },
            Span {
                version: 3,
                effective_from: d(20),
                superseded_on: None,
            },
        ];
        assert_eq!(in_force(&spans, d(4)), None, "before the first");
        assert_eq!(in_force(&spans, d(5)), Some(1));
        assert_eq!(in_force(&spans, d(11)), Some(1));
        assert_eq!(
            in_force(&spans, d(12)),
            Some(2),
            "the successor takes over on its first day, not a day later"
        );
        assert_eq!(in_force(&spans, d(19)), Some(2));
        assert_eq!(in_force(&spans, d(20)), Some(3));
        assert_eq!(in_force(&spans, d(30)), Some(3));
        for day in 1..=31 {
            let n = spans
                .iter()
                .filter(|s| {
                    s.effective_from <= d(day) && s.superseded_on.is_none_or(|e| d(day) < e)
                })
                .count();
            assert!(n <= 1, "{n} versions in force on the {day}th");
        }
    }

    #[test]
    fn overlapping_spans_answer_nothing_rather_than_choosing() {
        let spans = [
            Span {
                version: 1,
                effective_from: d(5),
                superseded_on: None,
            },
            Span {
                version: 2,
                effective_from: d(8),
                superseded_on: None,
            },
        ];
        assert_eq!(in_force(&spans, d(6)), Some(1));
        assert_eq!(in_force(&spans, d(9)), None);
    }

    #[test]
    fn text_is_checked() {
        assert!(problems(Some("Policy"), Some("text")).is_empty());
        assert_eq!(problems(Some(" "), None).len(), 1);
        assert_eq!(problems(Some(&"a".repeat(201)), None).len(), 1);
        assert_eq!(problems(None, Some(&"a".repeat(200_001))).len(), 1);
    }
}

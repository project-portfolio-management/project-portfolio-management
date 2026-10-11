//! Validated benefits, per year and non-financial (DP-9). Pure rules; the database work is in
//! [`crate::controllers::benefit_validation`].
//!
//! * A benefit is **recurring** (a yearly amount, `annualized_minor`) or **one-off** (`target_minor`).
//!   The two are never added into one "per year" figure.
//! * What was **realized** is a claim. It becomes **validated** only through a recorded validation by
//!   someone who is not a member of the programme. A validation states the amount validated to date;
//!   the latest one counts and the earlier ones stay on record.
//! * A non-financial benefit reports the status of its KPI, or **not measured** when it has none.

#![allow(missing_docs)]

use std::collections::BTreeMap;

use uuid::Uuid;

pub const RECURRENCES: &[&str] = &["recurring", "one_off"];
pub const CLASSIFICATIONS: &[&str] = &["cash_releasing", "non_cash_releasing", "societal"];

/// Whether `actor` (a token subject) is one of the people named by `refs` (`person:<id>`, `worker:<id>`
/// or a bare id), ignoring case and any `kind:` prefix.
#[must_use]
pub fn is_member(actor: &str, refs: &[String]) -> bool {
    let norm = |s: &str| s.rsplit(':').next().unwrap_or(s).trim().to_lowercase();
    let a = norm(actor);
    !a.is_empty() && refs.iter().any(|r| norm(r) == a)
}

/// How a benefit stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// Nothing realized yet.
    NotClaimed,
    /// Realized and not validated at all.
    Claimed,
    /// Validated for less than the claim.
    PartlyValidated,
    /// Validated for at least the claim.
    Validated,
}

impl Standing {
    #[must_use]
    pub fn word(self) -> &'static str {
        match self {
            Self::NotClaimed => "not_claimed",
            Self::Claimed => "claimed",
            Self::PartlyValidated => "partly_validated",
            Self::Validated => "validated",
        }
    }
}

/// Claimed against validated.
#[must_use]
pub fn standing(claimed_minor: i64, validated_minor: Option<i64>) -> Standing {
    let v = validated_minor.unwrap_or(0);
    if claimed_minor <= 0 && v <= 0 {
        Standing::NotClaimed
    } else if v <= 0 {
        Standing::Claimed
    } else if v < claimed_minor {
        Standing::PartlyValidated
    } else {
        Standing::Validated
    }
}

/// A financial benefit, reduced to what the view needs.
#[derive(Debug, Clone)]
pub struct Financial {
    pub programme: Uuid,
    pub currency: String,
    pub recurrence: Option<String>,
    pub classification: Option<String>,
    /// Yearly amount for a recurring benefit, the target for a one-off one; none when not stated.
    pub planned_minor: Option<i64>,
    pub claimed_minor: i64,
    pub validated_minor: Option<i64>,
}

/// A group: a programme, a currency, a recurrence and a classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub programme: Uuid,
    pub currency: String,
    /// `recurring`, `one_off` or `not_stated`.
    pub recurrence: String,
    /// A classification or `not_stated`.
    pub classification: String,
    pub planned_minor: i64,
    pub claimed_minor: i64,
    pub validated_minor: i64,
    /// Claimed and not validated: the part of the claim nobody has checked.
    pub unvalidated_claim_minor: i64,
    pub benefits: usize,
    pub with_no_planned_amount: usize,
    pub claimed_only: usize,
}

/// Group financial benefits. Recurring and one-off are separate rows, as are classifications and
/// currencies, so no row adds unlike things.
#[must_use]
pub fn group(items: &[Financial]) -> Vec<Row> {
    let mut rows: BTreeMap<(Uuid, String, String, String), Row> = BTreeMap::new();
    for b in items {
        let rec = b
            .recurrence
            .clone()
            .unwrap_or_else(|| "not_stated".to_string());
        let cls = b
            .classification
            .clone()
            .unwrap_or_else(|| "not_stated".to_string());
        let r = rows
            .entry((b.programme, b.currency.clone(), rec.clone(), cls.clone()))
            .or_insert_with(|| Row {
                programme: b.programme,
                currency: b.currency.clone(),
                recurrence: rec,
                classification: cls,
                planned_minor: 0,
                claimed_minor: 0,
                validated_minor: 0,
                unvalidated_claim_minor: 0,
                benefits: 0,
                with_no_planned_amount: 0,
                claimed_only: 0,
            });
        r.benefits += 1;
        match b.planned_minor {
            Some(p) => r.planned_minor = r.planned_minor.saturating_add(p),
            None => r.with_no_planned_amount += 1,
        }
        let v = b.validated_minor.unwrap_or(0);
        r.claimed_minor = r.claimed_minor.saturating_add(b.claimed_minor);
        r.validated_minor = r.validated_minor.saturating_add(v);
        r.unvalidated_claim_minor = r
            .unvalidated_claim_minor
            .saturating_add((b.claimed_minor - v).max(0));
        if standing(b.claimed_minor, b.validated_minor) == Standing::Claimed {
            r.claimed_only += 1;
        }
    }
    rows.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fin(
        rec: Option<&str>,
        cls: Option<&str>,
        cur: &str,
        plan: Option<i64>,
        claimed: i64,
        validated: Option<i64>,
    ) -> Financial {
        Financial {
            programme: Uuid::from_u128(1),
            currency: cur.into(),
            recurrence: rec.map(Into::into),
            classification: cls.map(Into::into),
            planned_minor: plan,
            claimed_minor: claimed,
            validated_minor: validated,
        }
    }

    #[test]
    fn realized_but_unvalidated_is_claimed_never_validated() {
        assert_eq!(standing(500, None), Standing::Claimed);
        assert_eq!(standing(500, Some(0)), Standing::Claimed);
        assert_eq!(standing(500, Some(200)), Standing::PartlyValidated);
        assert_eq!(standing(500, Some(500)), Standing::Validated);
        assert_eq!(standing(500, Some(700)), Standing::Validated);
        assert_eq!(standing(0, None), Standing::NotClaimed);
        assert_eq!(Standing::Claimed.word(), "claimed");
    }

    #[test]
    fn one_off_and_recurring_are_never_summed() {
        let rows = group(&[
            fin(
                Some("recurring"),
                Some("cash_releasing"),
                "GBP",
                Some(1000),
                400,
                Some(300),
            ),
            fin(
                Some("one_off"),
                Some("cash_releasing"),
                "GBP",
                Some(5000),
                5000,
                Some(5000),
            ),
            fin(
                Some("recurring"),
                Some("cash_releasing"),
                "GBP",
                Some(250),
                0,
                None,
            ),
        ]);
        assert_eq!(rows.len(), 2);
        let rec = rows.iter().find(|r| r.recurrence == "recurring").unwrap();
        assert_eq!(
            (
                rec.planned_minor,
                rec.claimed_minor,
                rec.validated_minor,
                rec.unvalidated_claim_minor,
                rec.benefits
            ),
            (1250, 400, 300, 100, 2)
        );
        let one = rows.iter().find(|r| r.recurrence == "one_off").unwrap();
        assert_eq!(
            (
                one.planned_minor,
                one.validated_minor,
                one.unvalidated_claim_minor
            ),
            (5000, 5000, 0)
        );
    }

    #[test]
    fn currencies_classes_and_unstated_stay_apart() {
        let rows = group(&[
            fin(
                Some("one_off"),
                Some("cash_releasing"),
                "GBP",
                Some(10),
                0,
                None,
            ),
            fin(
                Some("one_off"),
                Some("cash_releasing"),
                "EUR",
                Some(10),
                0,
                None,
            ),
            fin(Some("one_off"), Some("societal"), "GBP", Some(10), 0, None),
            fin(None, None, "GBP", None, 0, None),
        ]);
        assert_eq!(rows.len(), 4);
        let loose = rows.iter().find(|r| r.recurrence == "not_stated").unwrap();
        assert_eq!(
            (loose.classification.as_str(), loose.with_no_planned_amount),
            ("not_stated", 1)
        );
    }

    #[test]
    fn claimed_only_is_counted_apart_from_partly_validated() {
        let rows = group(&[
            fin(Some("one_off"), None, "GBP", Some(100), 100, None),
            fin(Some("one_off"), None, "GBP", Some(100), 100, Some(40)),
        ]);
        assert_eq!(
            (rows[0].claimed_only, rows[0].unvalidated_claim_minor),
            (1, 160)
        );
    }

    #[test]
    fn membership_ignores_case_and_the_kind_prefix() {
        let refs = vec![
            "person:3F2C6B7E-0000-4000-8000-0000000000A1".to_string(),
            "worker:other".to_string(),
        ];
        assert!(is_member("3f2c6b7e-0000-4000-8000-0000000000a1", &refs));
        assert!(is_member("worker:OTHER", &refs));
        assert!(!is_member("3f2c6b7e-0000-4000-8000-0000000000a2", &refs));
        assert!(!is_member("", &refs));
        assert!(!is_member("anyone", &[]));
    }
}

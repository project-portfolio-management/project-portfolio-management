//! Access review (RG-9): who holds what access, since when, who granted it, and which grants are old
//! enough to be reviewed. The pure rules; the controller is `controllers/access_review.rs`.
//!
//! The source is the **access ledger** (`access_records`): an append-only list of `granted` and
//! `revoked` entries recorded by people during joiner, mover and leaver work. A person holds an access
//! **now** when the latest entry for their (plan, access) pair is a grant. This is a record of what
//! people said was granted, not a read of what the identity provider enforces; a review should compare
//! the two.

use std::collections::BTreeMap;

use chrono::NaiveDate;
use uuid::Uuid;

/// The default age, in calendar days, after which a standing grant is flagged for review.
pub const DEFAULT_STALE_DAYS: i64 = 365;

/// One ledger entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Row id (breaks a tie between two entries recorded on the same day).
    pub id: i64,
    /// The plan.
    pub plan_pid: Uuid,
    /// The person.
    pub person_ref: String,
    /// What access, in words.
    pub access: String,
    /// `granted` or `revoked`.
    pub action: String,
    /// The day it was recorded.
    pub recorded_on: NaiveDate,
    /// Who recorded it.
    pub recorded_by: Option<String>,
    /// A note.
    pub note: Option<String>,
}

/// One grant that stands today.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standing {
    /// The latest entry (a grant).
    pub entry: Entry,
    /// Calendar days since it was granted.
    pub age_calendar_days: i64,
    /// Whether it is old enough to review.
    pub review_due: bool,
}

/// The grants that stand at `today`, oldest first. `stale_days` is the review age in calendar days.
#[must_use]
pub fn standing(entries: &[Entry], today: NaiveDate, stale_days: i64) -> Vec<Standing> {
    let mut latest: BTreeMap<(Uuid, String, String), &Entry> = BTreeMap::new();
    for e in entries {
        let key = (
            e.plan_pid,
            e.person_ref.clone(),
            e.access.trim().to_lowercase(),
        );
        match latest.get(&key) {
            Some(prev) if (prev.recorded_on, prev.id) >= (e.recorded_on, e.id) => {}
            _ => {
                latest.insert(key, e);
            }
        }
    }
    let mut out: Vec<Standing> = latest
        .into_values()
        .filter(|e| e.action == "granted")
        .map(|e| {
            let age = (today - e.recorded_on).num_days().max(0);
            Standing {
                entry: e.clone(),
                age_calendar_days: age,
                review_due: age >= stale_days,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.age_calendar_days
            .cmp(&a.age_calendar_days)
            .then(a.entry.person_ref.cmp(&b.entry.person_ref))
            .then(a.entry.id.cmp(&b.entry.id))
    });
    out
}

/// The review as CSV, with a column for the reviewer's decision to be filled in.
#[must_use]
pub fn to_csv(rows: &[Standing]) -> String {
    let mut out = String::from(
        "plan_pid,person_ref,access,granted_on,granted_by,age_calendar_days,review_due,note,decision (keep/revoke)\n",
    );
    for r in rows {
        let e = &r.entry;
        let fields = [
            e.plan_pid.to_string(),
            e.person_ref.clone(),
            e.access.clone(),
            e.recorded_on.to_string(),
            e.recorded_by.clone().unwrap_or_default(),
            r.age_calendar_days.to_string(),
            r.review_due.to_string(),
            e.note.clone().unwrap_or_default(),
            String::new(),
        ];
        out.push_str(
            &fields
                .iter()
                .map(|f| crate::csv_cell::field(f))
                .collect::<Vec<_>>()
                .join(","),
        );
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, d).unwrap()
    }

    fn entry(id: i64, person: &str, access: &str, action: &str, on: NaiveDate) -> Entry {
        Entry {
            id,
            plan_pid: Uuid::nil(),
            person_ref: person.into(),
            access: access.into(),
            action: action.into(),
            recorded_on: on,
            recorded_by: Some("person:admin".into()),
            note: None,
        }
    }

    #[test]
    fn a_revoked_grant_no_longer_stands() {
        let rows = vec![
            entry(1, "person:a", "edit", "granted", day(1, 1)),
            entry(2, "person:a", "edit", "revoked", day(2, 1)),
            entry(3, "person:b", "edit", "granted", day(1, 5)),
        ];
        let s = standing(&rows, day(10, 10), 365);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].entry.person_ref, "person:b");
    }

    #[test]
    fn a_regrant_after_a_revoke_stands_and_a_same_day_tie_goes_to_the_later_row() {
        let rows = vec![
            entry(1, "person:a", "edit", "granted", day(1, 1)),
            entry(2, "person:a", "edit", "revoked", day(2, 1)),
            entry(3, "person:a", "edit", "granted", day(3, 1)),
            entry(4, "person:c", "view", "granted", day(5, 1)),
            entry(5, "person:c", "view", "revoked", day(5, 1)),
        ];
        let s = standing(&rows, day(10, 10), 365);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].entry.id, 3);
    }

    #[test]
    fn access_names_are_compared_without_case_and_each_access_stands_on_its_own() {
        let rows = vec![
            entry(1, "person:a", "Edit", "granted", day(1, 1)),
            entry(2, "person:a", "edit", "revoked", day(2, 1)),
            entry(3, "person:a", "view", "granted", day(1, 1)),
        ];
        let s = standing(&rows, day(10, 10), 365);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].entry.access, "view");
    }

    #[test]
    fn age_is_in_calendar_days_oldest_first_and_flagged_at_the_threshold() {
        let rows = vec![
            entry(1, "person:new", "edit", "granted", day(10, 1)),
            entry(2, "person:old", "edit", "granted", day(1, 1)),
        ];
        let s = standing(&rows, day(10, 10), 100);
        assert_eq!(s[0].entry.person_ref, "person:old");
        assert_eq!(s[0].age_calendar_days, 282);
        assert!(s[0].review_due);
        assert_eq!(s[1].age_calendar_days, 9);
        assert!(!s[1].review_due);
        // Exactly at the threshold is due.
        assert!(standing(&rows, day(10, 10), 9)[1].review_due);
    }

    #[test]
    fn csv_quotes_and_defuses_formulas() {
        // Cell escaping and formula defusing are tested in `csv_cell`; this checks the review's CSV uses them.
        let rows = standing(
            &[entry(1, "=cmd()", "edit", "granted", day(1, 1))],
            day(10, 10),
            365,
        );
        let csv = to_csv(&rows);
        assert!(csv.starts_with("plan_pid,person_ref,access,"));
        assert_eq!(csv.lines().count(), 2);
        assert!(csv.contains(",'=cmd(),"), "{csv}");
    }
}

//! Onboarding and offboarding: the pure rules. DB-free and deterministic.
//!
//! * A **case** is one onboarding or offboarding: a person joining or leaving a
//!   plan, or a plan being taken in (intake) or closed out. Each kind has a
//!   built-in checklist ([`builtin`]) that a custom template can replace.
//! * An item's due date is the case's effective date plus an offset in calendar days
//!   (negative is before it) — [`due_date`].
//! * An item is open, `done`, or `skipped` (with a note: skipping is a decision,
//!   not a way to lose an item). A case can complete only when none is open.
//! * **Access** is an append-only record of grants and revocations; what is
//!   *outstanding* is whatever was granted and not revoked since ([`outstanding`]).
//! * A plan is **ready to close** when nothing is left open ([`closeout`]).

use std::collections::BTreeMap;

use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

/// What a case is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// A person joining a plan.
    PersonJoin,
    /// A person leaving a plan.
    PersonLeave,
    /// A plan being taken in.
    PlanIntake,
    /// A plan being closed out.
    PlanCloseout,
}

impl Kind {
    /// Parse the token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "person_join" => Some(Self::PersonJoin),
            "person_leave" => Some(Self::PersonLeave),
            "plan_intake" => Some(Self::PlanIntake),
            "plan_closeout" => Some(Self::PlanCloseout),
            _ => None,
        }
    }

    /// The token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::PersonJoin => "person_join",
            Self::PersonLeave => "person_leave",
            Self::PlanIntake => "plan_intake",
            Self::PlanCloseout => "plan_closeout",
        }
    }

    /// Whether the case is about a person (and so needs a subject).
    #[must_use]
    pub const fn is_person(self) -> bool {
        matches!(self, Self::PersonJoin | Self::PersonLeave)
    }
}

/// One checklist item of a template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Seed {
    /// What to do.
    pub title: String,
    /// Calendar days from the effective date (negative is before).
    pub offset_days: i64,
}

fn seeds(items: &[(&str, i64)]) -> Vec<Seed> {
    items
        .iter()
        .map(|(t, d)| Seed {
            title: (*t).to_string(),
            offset_days: *d,
        })
        .collect()
}

/// The built-in checklist for a kind.
#[must_use]
pub fn builtin(kind: Kind) -> Vec<Seed> {
    match kind {
        Kind::PersonJoin => seeds(&[
            ("Confirm the role and start date with their manager", -7),
            ("Book their time on the plan", -3),
            ("Grant access to the plan's tools and documents", -1),
            ("Add them to the plan's roles and channels", 0),
            ("Walk them through the goals, schedule and risks", 2),
            ("Check they can start their first task", 5),
        ]),
        Kind::PersonLeave => seeds(&[
            ("Agree the last day and tell the team", -14),
            ("Hand over open tasks, bookings and ownerships", -5),
            ("Hand over documents and knowledge", -3),
            ("Revoke access to the plan's tools and documents", 0),
            ("Release their time on the plan", 0),
            ("Record what access was revoked", 0),
        ]),
        Kind::PlanIntake => seeds(&[
            ("Name an owner and a sponsor", 0),
            ("State the objective and how it will be measured", 2),
            ("Set the first budget baseline", 5),
            ("Draft the schedule and take a baseline", 7),
            ("Open the risk register", 7),
            ("Book the people and roles needed", 10),
        ]),
        Kind::PlanCloseout => seeds(&[
            ("Finish or reassign every open task", -7),
            ("Close or hand over every open risk", -5),
            ("Release every booking", -3),
            ("Revoke the access that is still outstanding", 0),
            ("Record what was learned", 0),
            ("Take the final baseline and archive the plan", 2),
        ]),
    }
}

/// Problems with a custom template's items (empty when fine).
#[must_use]
pub fn template_problems(items: &[Seed]) -> Vec<String> {
    let mut out = Vec::new();
    if items.is_empty() || items.len() > 50 {
        out.push("a template has 1 to 50 items".to_string());
    }
    for (i, s) in items.iter().enumerate() {
        if s.title.trim().is_empty() || s.title.chars().count() > 200 {
            out.push(format!(
                "item {}: title is required (up to 200 characters)",
                i + 1
            ));
        }
        if !(-365..=365).contains(&s.offset_days) {
            out.push(format!("item {}: offset_days must be -365 to 365", i + 1));
        }
    }
    out
}

/// An item's due date: the effective date plus its offset.
#[must_use]
pub fn due_date(effective: NaiveDate, offset_days: i64) -> Option<NaiveDate> {
    effective.checked_add_signed(Duration::days(offset_days))
}

/// What progress needs to know of an item.
#[derive(Debug, Clone, Copy)]
pub struct ItemState<'a> {
    /// `done`, `skipped`, or none while open.
    pub resolution: Option<&'a str>,
    /// When it is due.
    pub due: Option<NaiveDate>,
}

/// How far a checklist has got.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Progress {
    /// All items.
    pub total: usize,
    /// Done.
    pub done: usize,
    /// Skipped (with a note).
    pub skipped: usize,
    /// Still open.
    pub open: usize,
    /// Open and past due on `today`.
    pub overdue: usize,
    /// Resolved (done or skipped) as basis points of the total; `None` for no items.
    pub resolved_bp: Option<i64>,
}

/// Progress as of `today`.
#[must_use]
pub fn progress(items: &[ItemState<'_>], today: NaiveDate) -> Progress {
    let done = items
        .iter()
        .filter(|i| i.resolution == Some("done"))
        .count();
    let skipped = items
        .iter()
        .filter(|i| i.resolution == Some("skipped"))
        .count();
    let open = items.len() - done - skipped;
    let overdue = items
        .iter()
        .filter(|i| i.resolution.is_none() && i.due.is_some_and(|d| d < today))
        .count();
    let resolved = i64::try_from(done + skipped).unwrap_or(0);
    let total = i64::try_from(items.len()).unwrap_or(0);
    Progress {
        total: items.len(),
        done,
        skipped,
        open,
        overdue,
        resolved_bp: (total > 0).then(|| resolved * 10_000 / total),
    }
}

/// Why a case cannot complete (empty when it can): the titles still open.
#[must_use]
pub fn completion_problems(open_titles: &[String]) -> Vec<String> {
    if open_titles.is_empty() {
        Vec::new()
    } else {
        vec![format!(
            "{} item(s) are still open: {}",
            open_titles.len(),
            open_titles.join("; ")
        )]
    }
}

/// One access record, as the rules see it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// Row order, to break a tie on the same day.
    pub id: i64,
    /// Whose access.
    pub person: String,
    /// What access.
    pub access: String,
    /// `granted` or `revoked`.
    pub granted: bool,
    /// The day it was recorded.
    pub on: NaiveDate,
}

/// Access that was granted and not revoked since: `(person, access, granted on)`,
/// ordered by person then access. The latest record for a (person, access) wins.
#[must_use]
pub fn outstanding(records: &[Record]) -> Vec<(String, String, NaiveDate)> {
    let mut latest: BTreeMap<(&str, &str), &Record> = BTreeMap::new();
    for r in records {
        let key = (r.person.as_str(), r.access.as_str());
        if latest.get(&key).is_none_or(|c| (c.on, c.id) < (r.on, r.id)) {
            latest.insert(key, r);
        }
    }
    latest
        .into_values()
        .filter(|r| r.granted)
        .map(|r| (r.person.clone(), r.access.clone(), r.on))
        .collect()
}

/// What is left on a plan.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    /// Unfinished tasks.
    pub open_tasks: usize,
    /// Risks still open or being mitigated.
    pub open_risks: usize,
    /// Bookings still live.
    pub active_allocations: usize,
    /// Access granted and not revoked.
    pub outstanding_access: usize,
    /// Onboarding / offboarding cases still open.
    pub open_cases: usize,
}

/// A reason a plan is not ready to close.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    /// A stable code.
    pub code: &'static str,
    /// How many.
    pub count: usize,
    /// The sentence.
    pub text: String,
}

/// Why a plan is not ready to close; empty means ready.
#[must_use]
pub fn closeout(c: Counts) -> Vec<Finding> {
    let f = |code, count: usize, what: &str| {
        (count > 0).then(|| Finding {
            code,
            count,
            text: format!("{count} {what}"),
        })
    };
    [
        f("open_tasks", c.open_tasks, "task(s) are not finished"),
        f("open_risks", c.open_risks, "risk(s) are still open"),
        f(
            "active_allocations",
            c.active_allocations,
            "booking(s) are still live",
        ),
        f(
            "outstanding_access",
            c.outstanding_access,
            "access grant(s) have not been revoked",
        ),
        f(
            "open_cases",
            c.open_cases,
            "onboarding or offboarding checklist(s) are still open",
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, day).unwrap()
    }

    #[test]
    fn kinds_round_trip_and_know_whether_they_are_about_a_person() {
        for k in [
            Kind::PersonJoin,
            Kind::PersonLeave,
            Kind::PlanIntake,
            Kind::PlanCloseout,
        ] {
            assert_eq!(Kind::parse(k.token()), Some(k));
        }
        assert_eq!(Kind::parse("hire"), None);
        assert!(Kind::PersonLeave.is_person() && !Kind::PlanCloseout.is_person());
    }

    #[test]
    fn every_built_in_checklist_is_valid_and_ordered_sensibly() {
        for k in [
            Kind::PersonJoin,
            Kind::PersonLeave,
            Kind::PlanIntake,
            Kind::PlanCloseout,
        ] {
            let items = builtin(k);
            assert!(template_problems(&items).is_empty(), "{k:?}");
            assert!(items.len() >= 5);
            let mut sorted = items.clone();
            sorted.sort_by_key(|s| s.offset_days);
            assert_eq!(items, sorted, "{k:?} lists items in date order");
        }
        // Leaving: access is revoked on the last day, handover comes before it.
        let leave = builtin(Kind::PersonLeave);
        let at = |needle: &str| {
            leave
                .iter()
                .find(|s| s.title.contains(needle))
                .unwrap()
                .offset_days
        };
        assert!(at("Hand over open tasks") < at("Revoke access"));
        assert_eq!(at("Revoke access"), 0);
    }

    #[test]
    fn due_dates_are_the_effective_date_plus_the_offset() {
        assert_eq!(due_date(d(10), -3), Some(d(7)));
        assert_eq!(due_date(d(10), 0), Some(d(10)));
        assert_eq!(due_date(d(10), 5), Some(d(15)));
    }

    #[test]
    fn templates_are_checked() {
        let ok = vec![Seed {
            title: "x".into(),
            offset_days: 0,
        }];
        assert!(template_problems(&ok).is_empty());
        assert_eq!(template_problems(&[]).len(), 1);
        assert_eq!(
            template_problems(&[Seed {
                title: " ".into(),
                offset_days: 0
            }])
            .len(),
            1
        );
        assert_eq!(
            template_problems(&[Seed {
                title: "x".into(),
                offset_days: 999
            }])
            .len(),
            1
        );
    }

    #[test]
    fn progress_counts_done_skipped_open_and_overdue() {
        let items = [
            ItemState {
                resolution: Some("done"),
                due: Some(d(1)),
            },
            ItemState {
                resolution: Some("skipped"),
                due: Some(d(1)),
            },
            ItemState {
                resolution: None,
                due: Some(d(3)),
            },
            ItemState {
                resolution: None,
                due: Some(d(20)),
            },
            ItemState {
                resolution: None,
                due: None,
            },
        ];
        let p = progress(&items, d(10));
        assert_eq!(
            (p.total, p.done, p.skipped, p.open, p.overdue),
            (5, 1, 1, 3, 1),
            "a resolved item is never overdue; an undated one cannot be"
        );
        assert_eq!(p.resolved_bp, Some(4000));
        assert_eq!(
            progress(&[], d(10)).resolved_bp,
            None,
            "no items is not 100%"
        );
    }

    #[test]
    fn a_case_completes_only_when_nothing_is_open() {
        assert!(completion_problems(&[]).is_empty());
        let p = completion_problems(&["Revoke access".to_string(), "Hand over".to_string()]);
        assert!(p[0].contains("2 item(s)") && p[0].contains("Revoke access"));
    }

    #[test]
    fn outstanding_access_is_what_was_granted_and_not_revoked_since() {
        let r = |id, who: &str, access: &str, granted, day| Record {
            id,
            person: who.into(),
            access: access.into(),
            granted,
            on: d(day),
        };
        let recs = vec![
            r(1, "a", "repo", true, 1),
            r(2, "a", "repo", false, 5), // revoked
            r(3, "a", "wiki", true, 2),  // still granted
            r(4, "b", "repo", true, 3),
            r(5, "b", "repo", false, 4),
            r(6, "b", "repo", true, 4), // regranted the same day: later row wins
        ];
        let out = outstanding(&recs);
        assert_eq!(
            out,
            vec![
                ("a".into(), "wiki".into(), d(2)),
                ("b".into(), "repo".into(), d(4))
            ]
        );
        // Order of the input does not matter.
        let mut rev = recs.clone();
        rev.reverse();
        assert_eq!(outstanding(&rev), out);
    }

    #[test]
    fn a_plan_is_ready_to_close_only_when_nothing_is_left() {
        assert!(closeout(Counts::default()).is_empty());
        let f = closeout(Counts {
            open_tasks: 2,
            outstanding_access: 1,
            ..Counts::default()
        });
        assert_eq!(
            f.iter().map(|x| x.code).collect::<Vec<_>>(),
            vec!["open_tasks", "outstanding_access"]
        );
        assert_eq!(f[0].text, "2 task(s) are not finished");
    }
}

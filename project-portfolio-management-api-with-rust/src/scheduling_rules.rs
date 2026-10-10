//! Scheduling rules (PJ-6): the pure arithmetic between a task's duration, its
//! units and its work. DB-free, deterministic, integer throughout.
//!
//! The three are tied by one relation, for one assignee:
//!
//! ```text
//! work (minutes) = duration (business days) × units (% of a day) × 480 / 100
//! ```
//!
//! (a full business day is [`MINUTES_PER_DAY`] = 480 minutes). Given two, the third
//! follows. When a person edits one, a **rule** says which of the others moves:
//!
//! | edited ↓ / rule → | fixed units | fixed duration | fixed work |
//! |---|---|---|---|
//! | **work**     | duration moves | **units** move | duration moves |
//! | **duration** | work moves     | work moves     | **units** move |
//! | **units**    | work moves     | work moves     | duration moves |
//!
//! So changing work under `fixed_duration` changes units and never duration; the
//! thing the rule names as fixed is kept, except that an explicit edit of it is
//! taken at its word (the other one is recomputed around it).
//!
//! Rounding is stated, never silent: a **duration** derived from work and units
//! rounds *up* to whole business days (a part day still takes a whole one);
//! **units** derived from work and duration round to the nearest whole percent;
//! **work** derived from the other two rounds to the nearest minute. The edited and the kept quantities are
//! never changed by the rounding, and `exact` says whether the relation then holds
//! to the minute.

use serde::Serialize;

/// Minutes in a full business day at 100% units.
pub const MINUTES_PER_DAY: i64 = 480;
/// Most business days a task may last (as the schedule fields).
pub const MAX_DAYS: i64 = 3_650;
/// Most units, percent (ten full-time people's worth is the ceiling for one assignee).
pub const MAX_UNITS: i64 = 1_000;
/// Most work, minutes.
pub const MAX_WORK: i64 = MAX_DAYS * MINUTES_PER_DAY * 10;

/// Which quantity stays fixed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rule {
    /// Units stay; duration or work follows.
    FixedUnits,
    /// Duration stays; units or work follows.
    FixedDuration,
    /// Work stays; duration or units follows.
    FixedWork,
}

impl Rule {
    /// Parse `fixed_units` / `fixed_duration` / `fixed_work`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fixed_units" => Some(Self::FixedUnits),
            "fixed_duration" => Some(Self::FixedDuration),
            "fixed_work" => Some(Self::FixedWork),
            _ => None,
        }
    }

    /// The stored token.
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            Self::FixedUnits => "fixed_units",
            Self::FixedDuration => "fixed_duration",
            Self::FixedWork => "fixed_work",
        }
    }
}

/// One of the three related quantities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    /// Business days.
    Duration,
    /// Percent of the assignee's day.
    Units,
    /// Minutes of work.
    Work,
}

/// The three quantities together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Triple {
    /// Business days.
    pub duration_days: i64,
    /// Percent of a day.
    pub units_percent: i64,
    /// Minutes of work.
    pub work_minutes: i64,
}

/// The result of recomputing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Recomputed {
    /// The three quantities after.
    pub triple: Triple,
    /// Which one was recomputed.
    pub derived: Field,
    /// Whether `work = duration × units × 480 / 100` holds to the minute.
    pub exact: bool,
}

/// Why a recomputation was refused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Problem {
    /// A quantity is outside its range.
    OutOfRange(&'static str),
    /// A milestone (zero duration) cannot carry work or units.
    MilestoneWithWork,
}

impl Problem {
    /// A sentence for the person.
    #[must_use]
    pub fn message(&self) -> &'static str {
        match self {
            Self::OutOfRange(m) => m,
            Self::MilestoneWithWork => {
                "a milestone (zero duration) cannot carry work: give it a duration first"
            }
        }
    }
}

/// `work` implied by a duration and units, rounded to the nearest minute.
#[must_use]
pub fn work_for(duration_days: i64, units_percent: i64) -> i64 {
    // 480 / 100 = 4.8 minutes per day per percent: multiply first, round half up.
    (duration_days * units_percent * MINUTES_PER_DAY * 2 + 100) / 200
}

/// Whether a triple satisfies the relation exactly.
#[must_use]
pub fn is_exact(t: Triple) -> bool {
    (t.duration_days * t.units_percent * MINUTES_PER_DAY) % 100 == 0
        && t.duration_days * t.units_percent * MINUTES_PER_DAY / 100 == t.work_minutes
}

fn check(t: Triple) -> Result<(), Problem> {
    if !(0..=MAX_DAYS).contains(&t.duration_days) {
        return Err(Problem::OutOfRange(
            "duration must be 0 to 3650 business days",
        ));
    }
    if !(1..=MAX_UNITS).contains(&t.units_percent) {
        return Err(Problem::OutOfRange("units must be 1 to 1000 percent"));
    }
    if !(0..=MAX_WORK).contains(&t.work_minutes) {
        return Err(Problem::OutOfRange("work is out of range"));
    }
    Ok(())
}

/// Which quantity the rule recomputes when `changed` is edited.
#[must_use]
pub fn derives(rule: Rule, changed: Field) -> Field {
    // Reading across the rule table in the module docs: the units move when work
    // is edited under fixed duration, or duration under fixed work; the duration
    // moves when work is edited otherwise, or units under fixed work; and in
    // every other case the work moves.
    match (changed, rule) {
        (Field::Work, Rule::FixedDuration) | (Field::Duration, Rule::FixedWork) => Field::Units,
        (Field::Work, _) | (Field::Units, Rule::FixedWork) => Field::Duration,
        (Field::Duration | Field::Units, _) => Field::Work,
    }
}

/// Recompute the quantity the rule moves, given the triple **with the edit
/// already applied** to `changed`.
///
/// # Errors
///
/// [`Problem`] when a quantity is out of range, or work is asked of a milestone.
pub fn recompute(rule: Rule, changed: Field, edited: Triple) -> Result<Recomputed, Problem> {
    check(edited)?;
    let derived = derives(rule, changed);
    let mut t = edited;
    match derived {
        Field::Work => t.work_minutes = work_for(t.duration_days, t.units_percent),
        Field::Duration => {
            // Whole business days, rounded up: a part day still takes a whole one.
            let per_day = t.units_percent * MINUTES_PER_DAY;
            t.duration_days = (t.work_minutes * 100 + per_day - 1) / per_day;
        }
        Field::Units => {
            if t.duration_days == 0 {
                return Err(if t.work_minutes == 0 {
                    Problem::OutOfRange("a milestone has no units")
                } else {
                    Problem::MilestoneWithWork
                });
            }
            let per_percent = t.duration_days * MINUTES_PER_DAY;
            // Nearest whole percent, at least 1 when there is work.
            let u = (t.work_minutes * 100 * 2 + per_percent) / (per_percent * 2);
            t.units_percent = u.max(1);
        }
    }
    check(t)?;
    Ok(Recomputed {
        triple: t,
        derived,
        exact: is_exact(t),
    })
}

/// Establish the third quantity when only two are known (a first assignment):
/// whichever is missing is derived from the other two.
///
/// # Errors
///
/// [`Problem`] when a quantity is out of range, or work is asked of a milestone.
pub fn complete(
    duration_days: Option<i64>,
    units_percent: Option<i64>,
    work_minutes: Option<i64>,
) -> Result<Recomputed, Problem> {
    match (duration_days, units_percent, work_minutes) {
        (Some(d), Some(u), None) => {
            let t = Triple {
                duration_days: d,
                units_percent: u,
                work_minutes: 0,
            };
            check(t)?;
            let t = Triple {
                work_minutes: work_for(d, u),
                ..t
            };
            Ok(Recomputed {
                triple: t,
                derived: Field::Work,
                exact: is_exact(t),
            })
        }
        (Some(d), None, Some(w)) => {
            let t = Triple {
                duration_days: d,
                units_percent: 100,
                work_minutes: w,
            };
            // Reuse the units derivation (rule irrelevant: work is the edit).
            recompute(Rule::FixedDuration, Field::Work, t)
        }
        (None, Some(u), Some(w)) => {
            let t = Triple {
                duration_days: 0,
                units_percent: u,
                work_minutes: w,
            };
            check(t)?;
            let mut r = recompute(Rule::FixedUnits, Field::Work, t)?;
            r.derived = Field::Duration;
            Ok(r)
        }
        _ => Err(Problem::OutOfRange(
            "give exactly two of duration, units and work (or all three if one is already stored)",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(d: i64, u: i64, w: i64) -> Triple {
        Triple {
            duration_days: d,
            units_percent: u,
            work_minutes: w,
        }
    }

    #[test]
    fn the_relation_is_work_equals_duration_times_units_times_a_day() {
        assert_eq!(work_for(5, 100), 2_400);
        assert_eq!(work_for(5, 50), 1_200);
        assert_eq!(work_for(10, 25), 1_200);
        assert_eq!(work_for(1, 1), 5, "4.8 minutes rounds to 5");
        assert_eq!(work_for(0, 100), 0);
        assert!(is_exact(t(5, 100, 2_400)));
        assert!(!is_exact(t(1, 1, 5)));
    }

    #[test]
    fn fixed_units_worked_example_changing_work_moves_duration() {
        // 5 business days at 100% is 2 400 minutes. Raise work to 3 600: 7.5
        // business days at 100%, which takes 8 whole business days. Units and the
        // new work are kept.
        let r = recompute(Rule::FixedUnits, Field::Work, t(5, 100, 3_600)).unwrap();
        assert_eq!(r.derived, Field::Duration);
        assert_eq!(r.triple, t(8, 100, 3_600));
        assert!(!r.exact, "7.5 business days was rounded up to 8");
        // Changing duration moves work.
        let r = recompute(Rule::FixedUnits, Field::Duration, t(10, 100, 2_400)).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Work, t(10, 100, 4_800)));
        assert!(r.exact);
    }

    #[test]
    fn fixed_duration_worked_example_changing_work_moves_units_never_duration() {
        // 5 business days, 100%, 2 400 minutes. Halve the work: units fall to 50%.
        let r = recompute(Rule::FixedDuration, Field::Work, t(5, 100, 1_200)).unwrap();
        assert_eq!(r.derived, Field::Units);
        assert_eq!(r.triple, t(5, 50, 1_200));
        assert!(r.exact);
        // Work that does not divide evenly: units round to the nearest percent.
        let r = recompute(Rule::FixedDuration, Field::Work, t(5, 100, 1_000)).unwrap();
        assert_eq!(r.triple.duration_days, 5, "duration never moves");
        assert_eq!(r.triple.units_percent, 42, "1000 / 2400 = 41.67%");
        assert_eq!(
            r.triple.work_minutes, 1_000,
            "the edited work is kept as entered"
        );
        assert!(!r.exact);
        // Editing units moves work, duration stays.
        let r = recompute(Rule::FixedDuration, Field::Units, t(5, 200, 2_400)).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Work, t(5, 200, 4_800)));
    }

    #[test]
    fn fixed_work_worked_example_changing_duration_moves_units() {
        // 2 400 minutes of work over 5 business days is 100%. Stretch to 10 business days: 50%.
        let r = recompute(Rule::FixedWork, Field::Duration, t(10, 100, 2_400)).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Units, t(10, 50, 2_400)));
        assert!(r.exact);
        // Editing units moves duration: double the units, half the business days.
        let r = recompute(Rule::FixedWork, Field::Units, t(5, 200, 2_400)).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Duration, t(3, 200, 2_400)));
        assert!(!r.exact, "2.5 business days rounds up to 3");
        // Work is the kept quantity in every one of those.
        assert_eq!(r.triple.work_minutes, 2_400);
    }

    #[test]
    fn the_rule_table_is_exactly_the_documented_one() {
        use Field::{Duration, Units, Work};
        use Rule::{FixedDuration, FixedUnits, FixedWork};
        for (rule, changed, derived) in [
            (FixedUnits, Work, Duration),
            (FixedDuration, Work, Units),
            (FixedWork, Work, Duration),
            (FixedUnits, Duration, Work),
            (FixedDuration, Duration, Work),
            (FixedWork, Duration, Units),
            (FixedUnits, Units, Work),
            (FixedDuration, Units, Work),
            (FixedWork, Units, Duration),
        ] {
            assert_eq!(derives(rule, changed), derived, "{rule:?} {changed:?}");
        }
    }

    #[test]
    fn a_rule_never_changes_the_edited_quantity() {
        for rule in [Rule::FixedUnits, Rule::FixedDuration, Rule::FixedWork] {
            for (changed, edited) in [
                (Field::Work, t(7, 60, 2_000)),
                (Field::Duration, t(9, 40, 1_700)),
                (Field::Units, t(4, 75, 1_500)),
            ] {
                let r = recompute(rule, changed, edited).unwrap();
                match changed {
                    Field::Work => assert_eq!(r.triple.work_minutes, edited.work_minutes),
                    Field::Duration => assert_eq!(r.triple.duration_days, edited.duration_days),
                    Field::Units => assert_eq!(r.triple.units_percent, edited.units_percent),
                }
                // And of the other two, only the derived one moved.
                let kept: Vec<bool> = [
                    (Field::Work, r.triple.work_minutes == edited.work_minutes),
                    (
                        Field::Duration,
                        r.triple.duration_days == edited.duration_days,
                    ),
                    (Field::Units, r.triple.units_percent == edited.units_percent),
                ]
                .iter()
                .filter(|(f, _)| *f != r.derived)
                .map(|(_, same)| *same)
                .collect();
                assert!(kept.iter().all(|k| *k), "{rule:?} {changed:?}");
            }
        }
    }

    #[test]
    fn a_first_assignment_derives_the_missing_quantity() {
        let r = complete(Some(5), Some(100), None).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Work, t(5, 100, 2_400)));
        let r = complete(Some(5), None, Some(1_200)).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Units, t(5, 50, 1_200)));
        let r = complete(None, Some(50), Some(1_200)).unwrap();
        assert_eq!((r.derived, r.triple), (Field::Duration, t(5, 50, 1_200)));
        assert!(complete(Some(5), None, None).is_err());
        assert!(complete(None, None, None).is_err());
    }

    #[test]
    fn nonsense_is_refused_in_words() {
        assert!(matches!(
            recompute(Rule::FixedUnits, Field::Work, t(5, 0, 100)),
            Err(Problem::OutOfRange(_))
        ));
        assert!(matches!(
            recompute(Rule::FixedUnits, Field::Work, t(5, 1_001, 100)),
            Err(Problem::OutOfRange(_))
        ));
        assert!(matches!(
            recompute(Rule::FixedUnits, Field::Work, t(3_651, 100, 100)),
            Err(Problem::OutOfRange(_))
        ));
        assert!(matches!(
            recompute(Rule::FixedUnits, Field::Work, t(5, 100, -1)),
            Err(Problem::OutOfRange(_))
        ));
        // Work on a milestone has nowhere to go under fixed duration.
        let e = recompute(Rule::FixedDuration, Field::Work, t(0, 100, 600)).unwrap_err();
        assert_eq!(e, Problem::MilestoneWithWork);
        assert!(e.message().contains("milestone"));
    }

    #[test]
    fn the_same_inputs_always_give_the_same_answer() {
        let a = recompute(Rule::FixedDuration, Field::Work, t(7, 100, 1_111)).unwrap();
        let b = recompute(Rule::FixedDuration, Field::Work, t(7, 100, 1_111)).unwrap();
        assert_eq!(a, b);
    }
}

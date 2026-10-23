//! Portfolio resource management (SC-7): the pure rules. DB-free, unit-tested.
//!
//! Demand is what the plans' **committed** allocations ask for, by role (or by
//! skill tag) and period; supply is what the people whose primary role it is can
//! give, read **per day** from their availability ([`crate::availability`]).
//! Both are summed as **percent-days** (an integer: percent × calendar days) and
//! only then turned into **FTE**
//! (`percent_days / (100 × calendar days in the period)`), so a role gap is
//! always stated in full-time equivalents, never as a percentage and never mixed
//! with headcount (which is its own field).
//!
//! * A **placeholder** is demand with a role and no person yet; it counts toward
//!   its role's demand and is reported apart (`placeholder_*`) so it can be seen
//!   and resolved. An allocation with no role is the `unclassified` row, never
//!   dropped.
//! * **Proposed** bookings are a separate series and never enter demand, gap or
//!   totals (PJ-4).
//! * Days are calendar days on both sides, so demand and supply share one basis.
//!   Skills are demand-only (a person's skills are resolved live from the worker
//!   service, not stored), and a multi-tag allocation counts fully toward each of
//!   its tags, so skill rows overlap and are flagged as such.
//! * Hours: a named allocation plans `percent × 8h` on each Monday–Friday of the
//!   window; actuals are recorded time entries. No entries is `null`, not zero.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::Serialize;
use uuid::Uuid;

use crate::availability::{Range, capacity_on};

/// Most periods one read returns.
pub const MAX_PERIODS: usize = 120;
/// Longest window, in calendar days.
pub const MAX_DAYS: i64 = 3_660;
/// Minutes a full-time person works on a weekday, for planned hours.
pub const MINUTES_PER_DAY: i64 = 480;
/// The row for demand or supply that names no role.
pub const UNCLASSIFIED: &str = "unclassified";
/// The row for demand that names no skill.
pub const UNTAGGED: &str = "untagged";
/// Prefix of a placeholder's `person_ref`.
pub const PLACEHOLDER_PREFIX: &str = "placeholder:";

/// Whether a `person_ref` is a placeholder.
#[must_use]
pub fn is_placeholder(person_ref: &str) -> bool {
    person_ref.starts_with(PLACEHOLDER_PREFIX)
}

/// One allocation as the engine sees it.
#[derive(Debug, Clone)]
pub struct Alloc {
    /// The plan it books against.
    pub plan: Uuid,
    /// Who; `None` for a placeholder.
    pub person: Option<String>,
    /// The role it asks for.
    pub role: Option<String>,
    /// Skill tags it requires.
    pub skills: Vec<String>,
    /// Percent of one person (1–100).
    pub percent: i32,
    /// First day; open if absent.
    pub start: Option<NaiveDate>,
    /// Last day; open if absent.
    pub end: Option<NaiveDate>,
    /// Hoped for rather than firm.
    pub proposed: bool,
}

/// A person on the supply side.
#[derive(Debug, Clone)]
pub struct Person {
    /// Who.
    pub person: String,
    /// Their primary role, if one is set.
    pub role: Option<String>,
    /// Their availability ranges (100% where none applies).
    pub ranges: Vec<Range>,
}

/// An inclusive run of days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Period {
    /// First day.
    pub from: NaiveDate,
    /// Last day.
    pub to: NaiveDate,
}

impl Period {
    /// Calendar days in the period.
    #[must_use]
    pub fn days(&self) -> i64 {
        (self.to - self.from).num_days() + 1
    }
}

/// Period size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grain {
    /// Monday-to-Sunday weeks, clipped to the window.
    Week,
    /// Calendar months, clipped to the window.
    Month,
}

impl Grain {
    /// Parse `week` or `month`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "week" => Some(Self::Week),
            "month" => Some(Self::Month),
            _ => None,
        }
    }
}

/// The periods covering `[from, to]`.
///
/// # Errors
///
/// A message when the window is reversed, too long, or has too many periods.
pub fn periods(from: NaiveDate, to: NaiveDate, grain: Grain) -> Result<Vec<Period>, String> {
    if to < from {
        return Err("to is before from".to_string());
    }
    if (to - from).num_days() > MAX_DAYS {
        return Err("the window may span at most ten years".to_string());
    }
    let mut out = Vec::new();
    let mut start = from;
    while start <= to {
        let natural_end = match grain {
            Grain::Week => {
                start + Duration::days(6 - i64::from(start.weekday().num_days_from_monday()))
            }
            Grain::Month => {
                let (y, m) = if start.month() == 12 {
                    (start.year() + 1, 1)
                } else {
                    (start.year(), start.month() + 1)
                };
                NaiveDate::from_ymd_opt(y, m, 1).map_or(to, |d| d - Duration::days(1))
            }
        };
        let end = natural_end.min(to);
        out.push(Period {
            from: start,
            to: end,
        });
        if out.len() > MAX_PERIODS {
            return Err(format!("the window has more than {MAX_PERIODS} periods"));
        }
        start = end + Duration::days(1);
    }
    Ok(out)
}

/// Calendar days of `[start, end]` (either open) inside `p`.
fn overlap(start: Option<NaiveDate>, end: Option<NaiveDate>, p: Period) -> i64 {
    let s = start.map_or(p.from, |s| s.max(p.from));
    let e = end.map_or(p.to, |e| e.min(p.to));
    ((e - s).num_days() + 1).max(0)
}

/// What demand is grouped by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBy {
    /// The allocation's `role`.
    Role,
    /// Each of the allocation's `skills_required` tags.
    Skill,
}

/// One key (role or skill) in one period.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Row {
    /// The role or skill; [`UNCLASSIFIED`] / [`UNTAGGED`] when none is named.
    pub key: String,
    /// Committed demand, percent-days (named + placeholder).
    pub demand_percent_days: i64,
    /// Of which, named people.
    pub named_percent_days: i64,
    /// Of which, placeholders (no person yet).
    pub placeholder_percent_days: i64,
    /// Proposed bookings, apart from the above.
    pub proposed_percent_days: i64,
    /// What the role's people can give, percent-days; `None` for skills.
    pub supply_percent_days: Option<i64>,
    /// How many people supply it (headcount, never mixed with FTE).
    pub headcount: usize,
}

/// One period of the plan.
#[derive(Debug, Clone, Serialize)]
pub struct PeriodRows {
    /// The period.
    pub period: Period,
    /// One row per key, A–Z.
    pub rows: Vec<Row>,
    /// Committed demand counting each allocation once (skill rows overlap).
    pub total_demand_percent_days: i64,
    /// Placeholder part of the total.
    pub total_placeholder_percent_days: i64,
    /// Proposed bookings.
    pub total_proposed_percent_days: i64,
}

/// A plan's demand per period (for reconciling against the rows).
#[derive(Debug, Clone, Serialize)]
pub struct PlanSeries {
    /// The plan.
    pub plan: Uuid,
    /// Committed demand per period, percent-days.
    pub demand_percent_days: Vec<i64>,
}

/// The whole resource plan.
#[derive(Debug, Clone, Serialize)]
pub struct ResourcePlan {
    /// Periods, in order, with their rows.
    pub periods: Vec<PeriodRows>,
    /// Per-plan demand, summing to the totals.
    pub by_plan: Vec<PlanSeries>,
}

/// Compute demand against supply.
#[must_use]
pub fn plan(
    periods: &[Period],
    allocs: &[Alloc],
    people: &[Person],
    group: GroupBy,
) -> ResourcePlan {
    let keys_of = |a: &Alloc| -> Vec<String> {
        match group {
            GroupBy::Role => vec![a.role.clone().unwrap_or_else(|| UNCLASSIFIED.to_string())],
            GroupBy::Skill if a.skills.is_empty() => vec![UNTAGGED.to_string()],
            GroupBy::Skill => {
                let mut s: Vec<String> = a.skills.clone();
                s.sort();
                s.dedup();
                s
            }
        }
    };
    let mut out = Vec::with_capacity(periods.len());
    let mut by_plan: BTreeMap<Uuid, Vec<i64>> = BTreeMap::new();
    for (pi, p) in periods.iter().enumerate() {
        let mut rows: BTreeMap<String, Row> = BTreeMap::new();
        let (mut total, mut total_ph, mut total_prop) = (0_i64, 0_i64, 0_i64);
        for a in allocs {
            let pd = overlap(a.start, a.end, *p) * i64::from(a.percent);
            if pd == 0 {
                continue;
            }
            if a.proposed {
                total_prop += pd;
            } else {
                total += pd;
                if a.person.is_none() {
                    total_ph += pd;
                }
                by_plan
                    .entry(a.plan)
                    .or_insert_with(|| vec![0; periods.len()])[pi] += pd;
            }
            for key in keys_of(a) {
                let row = rows.entry(key.clone()).or_insert_with(|| Row {
                    key,
                    ..Row::default()
                });
                if a.proposed {
                    row.proposed_percent_days += pd;
                } else {
                    row.demand_percent_days += pd;
                    if a.person.is_none() {
                        row.placeholder_percent_days += pd;
                    } else {
                        row.named_percent_days += pd;
                    }
                }
            }
        }
        if group == GroupBy::Role {
            let mut heads: HashMap<String, HashSet<&str>> = HashMap::new();
            for person in people {
                let key = person
                    .role
                    .clone()
                    .unwrap_or_else(|| UNCLASSIFIED.to_string());
                let mut supply = 0_i64;
                let mut day = p.from;
                while day <= p.to {
                    supply += i64::from(capacity_on(&person.ranges, day));
                    day += Duration::days(1);
                }
                let row = rows.entry(key.clone()).or_insert_with(|| Row {
                    key: key.clone(),
                    ..Row::default()
                });
                *row.supply_percent_days.get_or_insert(0) += supply;
                heads.entry(key).or_default().insert(person.person.as_str());
            }
            for (key, set) in heads {
                if let Some(r) = rows.get_mut(&key) {
                    r.headcount = set.len();
                }
            }
            // A role with demand but no supplier has supply of zero people,
            // which is a real answer (a gap), not an absent one.
            for r in rows.values_mut() {
                r.supply_percent_days.get_or_insert(0);
            }
        }
        out.push(PeriodRows {
            period: *p,
            rows: rows.into_values().collect(),
            total_demand_percent_days: total,
            total_placeholder_percent_days: total_ph,
            total_proposed_percent_days: total_prop,
        });
    }
    ResourcePlan {
        periods: out,
        by_plan: by_plan
            .into_iter()
            .map(|(plan, demand_percent_days)| PlanSeries {
                plan,
                demand_percent_days,
            })
            .collect(),
    }
}

/// `percent_days` as FTE over `days`, to two places. FTE is
/// `percent_days / (100 × days)`.
#[must_use]
pub fn fte(percent_days: i64, days: i64) -> f64 {
    if days <= 0 {
        return 0.0;
    }
    #[expect(clippy::cast_precision_loss)]
    let v = percent_days as f64 / (100.0 * days as f64);
    (v * 100.0).round() / 100.0
}

/// Planned and actual minutes for one plan and person.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HoursRow {
    /// The plan.
    pub plan: Uuid,
    /// The person; `None` for placeholder demand.
    pub person: Option<String>,
    /// Planned minutes (Monday–Friday, `percent × 8h`); zero for unplanned work.
    pub planned_minutes: i64,
    /// Recorded minutes, or `None` when nothing was recorded.
    pub actual_minutes: Option<i64>,
    /// There is time recorded but no allocation of this person on this plan.
    pub unplanned: bool,
}

/// Weekdays of `[start, end]` inside `[from, to]`.
fn weekdays(
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    from: NaiveDate,
    to: NaiveDate,
) -> i64 {
    let s = start.map_or(from, |s| s.max(from));
    let e = end.map_or(to, |e| e.min(to));
    let mut n = 0;
    let mut day = s;
    while day <= e {
        if !matches!(day.weekday(), Weekday::Sat | Weekday::Sun) {
            n += 1;
        }
        day += Duration::days(1);
    }
    n
}

/// One recorded time entry.
#[derive(Debug, Clone)]
pub struct Actual {
    /// The plan.
    pub plan: Uuid,
    /// Who recorded it.
    pub person: String,
    /// Minutes.
    pub minutes: i64,
}

/// Planned against actual, per plan and person. Rows for placeholders carry
/// planned minutes only. A person with an allocation but no time recorded has
/// `actual_minutes: None`; time with no allocation at all is `unplanned`.
/// `any_allocation` names every (plan, person) with a live allocation, proposed
/// or not, that overlaps the window — that is what "matching" means.
#[must_use]
pub fn hours<S: std::hash::BuildHasher>(
    from: NaiveDate,
    to: NaiveDate,
    allocs: &[Alloc],
    any_allocation: &HashSet<(Uuid, String), S>,
    actuals: &[Actual],
) -> Vec<HoursRow> {
    let mut planned: BTreeMap<(Uuid, Option<String>), i64> = BTreeMap::new();
    for a in allocs.iter().filter(|a| !a.proposed) {
        let days = weekdays(a.start, a.end, from, to);
        if days == 0 {
            continue;
        }
        *planned.entry((a.plan, a.person.clone())).or_default() +=
            days * i64::from(a.percent) * MINUTES_PER_DAY;
    }
    let mut spent: BTreeMap<(Uuid, String), i64> = BTreeMap::new();
    for a in actuals {
        *spent.entry((a.plan, a.person.clone())).or_default() += a.minutes;
    }
    let mut keys: BTreeSet<(Uuid, Option<String>)> = planned.keys().cloned().collect();
    keys.extend(spent.keys().map(|(p, who)| (*p, Some(who.clone()))));
    keys.into_iter()
        .map(|(plan, person)| {
            // percent × minutes is in hundredths of a minute.
            let planned_minutes = planned
                .get(&(plan, person.clone()))
                .map_or(0, |v| (v + 50) / 100);
            let actual_minutes = person
                .as_ref()
                .and_then(|who| spent.get(&(plan, who.clone())).copied());
            let unplanned = person.as_ref().is_some_and(|who| {
                actual_minutes.is_some() && !any_allocation.contains(&(plan, who.clone()))
            });
            HoursRow {
                plan,
                person,
                planned_minutes,
                actual_minutes,
                unplanned,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn alloc(plan: u128, who: Option<&str>, role: Option<&str>, pct: i32) -> Alloc {
        Alloc {
            plan: id(plan),
            person: who.map(str::to_string),
            role: role.map(str::to_string),
            skills: vec![],
            percent: pct,
            start: None,
            end: None,
            proposed: false,
        }
    }
    fn person(who: &str, role: Option<&str>) -> Person {
        Person {
            person: who.to_string(),
            role: role.map(str::to_string),
            ranges: vec![],
        }
    }
    fn week() -> Vec<Period> {
        // Mon 2026-10-05 .. Sun 2026-10-11: one full week, seven calendar days.
        periods(d(2026, 10, 5), d(2026, 10, 11), Grain::Week).unwrap()
    }

    #[test]
    fn periods_are_clipped_weeks_and_months() {
        let w = periods(d(2026, 10, 7), d(2026, 10, 20), Grain::Week).unwrap();
        assert_eq!(w.len(), 3);
        assert_eq!((w[0].from, w[0].to), (d(2026, 10, 7), d(2026, 10, 11)));
        assert_eq!((w[1].from, w[1].to), (d(2026, 10, 12), d(2026, 10, 18)));
        assert_eq!((w[2].from, w[2].to), (d(2026, 10, 19), d(2026, 10, 20)));
        let m = periods(d(2026, 11, 15), d(2027, 1, 10), Grain::Month).unwrap();
        assert_eq!(m.len(), 3);
        assert_eq!((m[0].from, m[0].to), (d(2026, 11, 15), d(2026, 11, 30)));
        assert_eq!((m[1].from, m[1].to), (d(2026, 12, 1), d(2026, 12, 31)));
        assert_eq!((m[2].from, m[2].to), (d(2027, 1, 1), d(2027, 1, 10)));
        assert!(periods(d(2026, 1, 2), d(2026, 1, 1), Grain::Week).is_err());
        assert!(periods(d(2020, 1, 1), d(2029, 1, 1), Grain::Week).is_err());
    }

    #[test]
    fn a_role_gap_is_stated_in_fte() {
        // Two developers (supply 2.00 FTE); demand 150% + 100% = 2.50 FTE.
        let allocs = [
            alloc(1, Some("person:a"), Some("dev"), 100),
            alloc(2, Some("person:b"), Some("dev"), 50),
            alloc(2, None, Some("dev"), 100),
        ];
        let people = [
            person("person:a", Some("dev")),
            person("person:b", Some("dev")),
        ];
        let p = plan(&week(), &allocs, &people, GroupBy::Role);
        let row = &p.periods[0].rows[0];
        assert_eq!(row.key, "dev");
        assert_eq!(row.demand_percent_days, 7 * 250);
        assert_eq!(
            row.placeholder_percent_days, 700,
            "the placeholder is its own part"
        );
        assert_eq!(row.named_percent_days, 1050);
        assert_eq!(row.supply_percent_days, Some(1400));
        assert_eq!(row.headcount, 2);
        let near = |got: f64, want: f64| assert!((got - want).abs() < 1e-9, "{got} != {want}");
        near(fte(row.demand_percent_days, 7), 2.5);
        near(fte(row.supply_percent_days.unwrap(), 7), 2.0);
        // Short by half an FTE.
        near(
            fte(
                row.supply_percent_days.unwrap() - row.demand_percent_days,
                7,
            ),
            -0.5,
        );
    }

    #[test]
    fn leave_lowers_supply_by_day() {
        let mut a = person("person:a", Some("dev"));
        a.ranges = vec![Range {
            from: d(2026, 10, 5),
            to: d(2026, 10, 8),
            max_percent: 0,
        }];
        let p = plan(&week(), &[], &[a], GroupBy::Role);
        assert_eq!(p.periods[0].rows[0].supply_percent_days, Some(300));
    }

    #[test]
    fn no_role_is_its_own_row_never_dropped() {
        let allocs = [
            alloc(1, Some("person:a"), None, 40),
            alloc(1, None, Some("qa"), 10),
        ];
        let people = [person("person:z", None)];
        let p = plan(&week(), &allocs, &people, GroupBy::Role);
        let keys: Vec<&str> = p.periods[0].rows.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(keys, vec!["qa", "unclassified"]);
        let u = &p.periods[0].rows[1];
        assert_eq!(u.demand_percent_days, 280);
        assert_eq!(u.supply_percent_days, Some(700));
        let qa = &p.periods[0].rows[0];
        assert_eq!(
            qa.supply_percent_days,
            Some(0),
            "a role nobody supplies is a real zero"
        );
        assert_eq!(qa.headcount, 0);
    }

    #[test]
    fn proposed_bookings_never_enter_demand_or_totals() {
        let mut prop = alloc(1, Some("person:a"), Some("dev"), 100);
        prop.proposed = true;
        let p = plan(
            &week(),
            &[prop, alloc(1, Some("person:a"), Some("dev"), 20)],
            &[],
            GroupBy::Role,
        );
        let row = &p.periods[0].rows[0];
        assert_eq!(row.demand_percent_days, 140);
        assert_eq!(row.proposed_percent_days, 700);
        assert_eq!(p.periods[0].total_demand_percent_days, 140);
        assert_eq!(p.periods[0].total_proposed_percent_days, 700);
    }

    #[test]
    fn totals_reconcile_with_the_sum_of_per_plan_allocations() {
        let months = periods(d(2026, 10, 1), d(2026, 12, 31), Grain::Month).unwrap();
        let mut first = alloc(1, Some("person:a"), Some("dev"), 60);
        first.start = Some(d(2026, 10, 15));
        first.end = Some(d(2026, 11, 20));
        let mut second = alloc(2, None, Some("qa"), 100);
        second.start = Some(d(2026, 11, 1));
        let third = alloc(3, Some("person:c"), None, 25);
        let p = plan(&months, &[first, second, third], &[], GroupBy::Role);
        for (i, per) in p.periods.iter().enumerate() {
            let by_role: i64 = per.rows.iter().map(|r| r.demand_percent_days).sum();
            let by_plan: i64 = p.by_plan.iter().map(|s| s.demand_percent_days[i]).sum();
            assert_eq!(
                by_role, per.total_demand_percent_days,
                "roles sum to the total"
            );
            assert_eq!(
                by_plan, per.total_demand_percent_days,
                "plans sum to the total"
            );
        }
        // October: a is 17 calendar days at 60, b nothing yet, c is 31 calendar days at 25.
        assert_eq!(p.periods[0].total_demand_percent_days, 17 * 60 + 31 * 25);
    }

    #[test]
    fn skills_overlap_and_are_demand_only() {
        let mut a = alloc(1, Some("person:a"), Some("dev"), 50);
        a.skills = vec!["rust".into(), "sql".into()];
        let b = alloc(1, Some("person:b"), Some("dev"), 10);
        let p = plan(
            &week(),
            &[a, b],
            &[person("person:a", Some("dev"))],
            GroupBy::Skill,
        );
        let per = &p.periods[0];
        let get = |k: &str| per.rows.iter().find(|r| r.key == k).unwrap();
        assert_eq!(get("rust").demand_percent_days, 350);
        assert_eq!(get("sql").demand_percent_days, 350);
        assert_eq!(get("untagged").demand_percent_days, 70);
        assert!(per.rows.iter().all(|r| r.supply_percent_days.is_none()));
        assert_eq!(per.total_demand_percent_days, 420, "each allocation once");
    }

    #[test]
    fn hours_list_unplanned_actuals_and_never_invent_zero() {
        let from = d(2026, 10, 5);
        let to = d(2026, 10, 11);
        let allocs = [
            alloc(1, Some("person:a"), Some("dev"), 50),
            alloc(1, None, Some("dev"), 100),
        ];
        let any: HashSet<(Uuid, String)> = [(id(1), "person:a".to_string())].into_iter().collect();
        let actuals = [Actual {
            plan: id(2),
            person: "person:z".into(),
            minutes: 90,
        }];
        let rows = hours(from, to, &allocs, &any, &actuals);
        let a = rows
            .iter()
            .find(|r| r.person.as_deref() == Some("person:a"))
            .unwrap();
        assert_eq!(a.planned_minutes, 5 * 240, "five weekdays at half of 8h");
        assert_eq!(a.actual_minutes, None, "nothing recorded is null, not zero");
        assert!(!a.unplanned);
        let ph = rows.iter().find(|r| r.person.is_none()).unwrap();
        assert_eq!(ph.planned_minutes, 5 * 480);
        assert_eq!(ph.actual_minutes, None);
        let z = rows
            .iter()
            .find(|r| r.person.as_deref() == Some("person:z"))
            .unwrap();
        assert!(z.unplanned, "time with no allocation is listed");
        assert_eq!((z.planned_minutes, z.actual_minutes), (0, Some(90)));
    }
}

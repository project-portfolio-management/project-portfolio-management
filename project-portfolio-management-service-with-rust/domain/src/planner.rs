//! Personal planner (PL-4): the pure rules. DB-free, unit-tested.
//!
//! A *slot* blocks time for one task. Instants are UTC; a **day** is the
//! caller's local calendar day in their zone, whose length is 24 hours except
//! across a daylight-saving change (23 or 25), so the day window is built
//! from local midnights, never from "+24h".
//!
//! Slots that overlap are **allowed and reported** as clashes — a person may
//! well double-book on purpose. Touching slots (one ends as the next starts)
//! are not a clash.

use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
use uuid::Uuid;

use crate::tz::Tz;

/// Longest single slot.
pub const MAX_SLOT: Duration = Duration::hours(12);

/// Every problem with a slot's times.
#[must_use]
pub fn slot_problems(starts: DateTime<Utc>, ends: DateTime<Utc>) -> Vec<String> {
    let mut out = Vec::new();
    if ends <= starts {
        out.push("ends_at must be after starts_at".to_string());
    } else if ends - starts > MAX_SLOT {
        out.push("a slot is at most 12 hours".to_string());
    }
    out
}

/// The UTC instants bounding the local calendar `day` in `tz`:
/// `[start, end)`. Uses the first valid instant of each local midnight, so
/// the window is right on a 23- or 25-hour day.
#[must_use]
pub fn day_window(day: NaiveDate, tz: Tz) -> (DateTime<Utc>, DateTime<Utc>) {
    let midnight = |d: NaiveDate| {
        // A local midnight can be skipped (a clock that jumps over it) or
        // repeated; take the earliest real instant at or after it.
        let mut t = d.and_hms_opt(0, 0, 0).unwrap_or_default();
        loop {
            match tz.from_local_datetime(&t) {
                chrono::LocalResult::Single(x) => return x.with_timezone(&Utc),
                chrono::LocalResult::Ambiguous(first, _) => return first.with_timezone(&Utc),
                chrono::LocalResult::None => t += Duration::minutes(15),
            }
        }
    };
    (midnight(day), midnight(day.succ_opt().unwrap_or(day)))
}

/// One slot's span, as the clash finder sees it.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    /// The slot's id.
    pub id: Uuid,
    /// Start.
    pub start: DateTime<Utc>,
    /// End (exclusive).
    pub end: DateTime<Utc>,
}

/// Groups of slots that overlap one another, transitively (A overlaps B and
/// B overlaps C ⇒ one group of three). Each group has at least two members
/// and lists ids in start order; groups are ordered by their first start.
#[must_use]
pub fn clashes(spans: &[Span]) -> Vec<Vec<Uuid>> {
    let mut sorted: Vec<Span> = spans.to_vec();
    sorted.sort_by_key(|s| (s.start, s.end, s.id));
    let mut groups: Vec<Vec<Uuid>> = Vec::new();
    let mut current: Vec<Uuid> = Vec::new();
    let mut reach: Option<DateTime<Utc>> = None;
    for s in sorted {
        match reach {
            Some(r) if s.start < r => {
                current.push(s.id);
                reach = Some(r.max(s.end));
            }
            _ => {
                if current.len() > 1 {
                    groups.push(std::mem::take(&mut current));
                }
                current = vec![s.id];
                reach = Some(s.end);
            }
        }
    }
    if current.len() > 1 {
        groups.push(current);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tz::parse_zone;

    fn at(h: u32, m: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 3, h, m, 0).unwrap()
    }
    fn span(n: u128, a: (u32, u32), b: (u32, u32)) -> Span {
        Span {
            id: Uuid::from_u128(n),
            start: at(a.0, a.1),
            end: at(b.0, b.1),
        }
    }

    #[test]
    fn a_slot_must_run_forward_and_stay_within_twelve_hours() {
        assert!(slot_problems(at(9, 0), at(10, 0)).is_empty());
        assert!(!slot_problems(at(10, 0), at(10, 0)).is_empty());
        assert!(!slot_problems(at(10, 0), at(9, 0)).is_empty());
        assert!(!slot_problems(at(0, 0), at(13, 0)).is_empty());
    }

    #[test]
    fn a_day_window_follows_the_callers_zone_and_dst() {
        let d = NaiveDate::from_ymd_opt(2026, 10, 3).unwrap();
        let (s, e) = day_window(d, parse_zone("UTC").unwrap());
        assert_eq!((e - s).num_hours(), 24);
        let (s, _) = day_window(d, parse_zone("Pacific/Auckland").unwrap());
        assert_eq!(
            s,
            Utc.with_ymd_and_hms(2026, 10, 2, 11, 0, 0).unwrap(),
            "NZDT is UTC+13"
        );
        // The day the US falls back (2026-11-01) is 25 hours long.
        let fall = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
        let (s, e) = day_window(fall, parse_zone("America/Los_Angeles").unwrap());
        assert_eq!((e - s).num_hours(), 25);
        let spring = NaiveDate::from_ymd_opt(2026, 3, 8).unwrap();
        let (s, e) = day_window(spring, parse_zone("America/Los_Angeles").unwrap());
        assert_eq!((e - s).num_hours(), 23);
    }

    #[test]
    fn overlaps_clash_but_touching_slots_do_not() {
        let spans = [
            span(1, (9, 0), (10, 0)),
            span(2, (10, 0), (11, 0)),
            span(3, (13, 0), (14, 0)),
        ];
        assert!(clashes(&spans).is_empty(), "back-to-back is fine");
        let spans = [
            span(1, (9, 0), (10, 30)),
            span(2, (10, 0), (11, 0)),
            span(3, (13, 0), (14, 0)),
        ];
        assert_eq!(
            clashes(&spans),
            vec![vec![Uuid::from_u128(1), Uuid::from_u128(2)]]
        );
    }

    #[test]
    fn a_clash_chains_transitively_into_one_group() {
        let spans = [
            span(3, (10, 30), (12, 0)),
            span(1, (9, 0), (10, 40)),
            span(2, (10, 0), (11, 0)),
            span(9, (15, 0), (16, 0)),
        ];
        let g = clashes(&spans);
        assert_eq!(g.len(), 1);
        assert_eq!(
            g[0],
            [Uuid::from_u128(1), Uuid::from_u128(2), Uuid::from_u128(3)],
            "in start order"
        );
    }
}

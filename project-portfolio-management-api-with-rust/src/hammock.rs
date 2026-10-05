//! Hammock activities (PJ-7): the pure rules. A hammock is a task whose span is
//! derived from the tasks it is linked to: it starts when the earliest of them
//! starts and finishes when the latest finishes. It is **not work** and not in the
//! dependency network, so it can never drive anyone's float; it differs from a
//! roll-up (which follows a task's children) in following its *links*.
//!
//! * A hammock may link to another hammock, so spans nest; a hammock cannot link
//!   to itself or close a cycle of hammocks ([`would_cycle`]).
//! * A hammock with no links is unscheduled (`no_links`); one whose linked tasks
//!   are all unscheduled is unscheduled too (`members_unscheduled`) — never a
//!   made-up span.

use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

/// A derived span, or why there is none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Span {
    /// Earliest start to latest finish of what it links to.
    Scheduled {
        /// The earliest start among its links.
        start: NaiveDate,
        /// The latest finish among its links.
        finish: NaiveDate,
    },
    /// No span could be derived.
    Unscheduled {
        /// `no_links` or `members_unscheduled`.
        reason: &'static str,
    },
}

/// Whether adding `hammock → member` would link a hammock to itself or close a
/// cycle among hammocks. `links` are the existing `(hammock, member)` pairs.
#[must_use]
pub fn would_cycle(links: &[(Uuid, Uuid)], hammock: Uuid, member: Uuid) -> bool {
    if hammock == member {
        return true;
    }
    // A cycle exists if `member` already (transitively) spans `hammock`.
    let mut by_hammock: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for (h, m) in links {
        by_hammock.entry(*h).or_default().push(*m);
    }
    let mut stack = vec![member];
    let mut seen = HashSet::new();
    while let Some(n) = stack.pop() {
        if n == hammock {
            return true;
        }
        if seen.insert(n)
            && let Some(next) = by_hammock.get(&n)
        {
            stack.extend(next.iter().copied());
        }
    }
    false
}

#[allow(clippy::implicit_hasher)]
fn span_of(
    h: Uuid,
    set: &HashSet<Uuid>,
    members: &HashMap<Uuid, Vec<Uuid>>,
    spans: &HashMap<Uuid, (NaiveDate, NaiveDate)>,
    out: &mut HashMap<Uuid, Span>,
    path: &mut HashSet<Uuid>,
) -> Option<(NaiveDate, NaiveDate)> {
    if let Some(Span::Scheduled { start, finish }) = out.get(&h) {
        return Some((*start, *finish));
    }
    if out.contains_key(&h) || !path.insert(h) {
        return None; // already known unscheduled, or a cycle that the rules refuse to create
    }
    let Some(ms) = members.get(&h) else {
        out.insert(h, Span::Unscheduled { reason: "no_links" });
        path.remove(&h);
        return None;
    };
    let mut lo: Option<NaiveDate> = None;
    let mut hi: Option<NaiveDate> = None;
    for m in ms {
        let s = if set.contains(m) {
            span_of(*m, set, members, spans, out, path)
        } else {
            spans.get(m).copied()
        };
        if let Some((a, b)) = s {
            lo = Some(lo.map_or(a, |x| x.min(a)));
            hi = Some(hi.map_or(b, |x| x.max(b)));
        }
    }
    path.remove(&h);
    if let Some((start, finish)) = lo.zip(hi) {
        out.insert(h, Span::Scheduled { start, finish });
        Some((start, finish))
    } else {
        out.insert(
            h,
            Span::Unscheduled {
                reason: "members_unscheduled",
            },
        );
        None
    }
}

/// The span of every hammock. `spans` holds the scheduled tasks' `(start, finish)`;
/// `links` the `(hammock, member)` pairs; `hammocks` every hammock's pid.
///
/// A member that is itself a hammock contributes its own derived span.
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn resolve(
    hammocks: &[Uuid],
    links: &[(Uuid, Uuid)],
    spans: &HashMap<Uuid, (NaiveDate, NaiveDate)>,
) -> HashMap<Uuid, Span> {
    let set: HashSet<Uuid> = hammocks.iter().copied().collect();
    let mut members: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for (h, m) in links {
        if set.contains(h) {
            members.entry(*h).or_default().push(*m);
        }
    }
    let mut out: HashMap<Uuid, Span> = HashMap::new();
    for h in hammocks {
        span_of(*h, &set, &members, spans, &mut out, &mut HashSet::new());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, day).unwrap()
    }
    fn spans() -> HashMap<Uuid, (NaiveDate, NaiveDate)> {
        HashMap::from([
            (id(1), (d(5), d(7))),
            (id(2), (d(8), d(14))),
            (id(3), (d(6), d(6))),
        ])
    }

    #[test]
    fn a_hammock_spans_the_earliest_start_to_the_latest_finish_of_its_links() {
        let r = resolve(
            &[id(100)],
            &[(id(100), id(1)), (id(100), id(2)), (id(100), id(3))],
            &spans(),
        );
        assert_eq!(
            r[&id(100)],
            Span::Scheduled {
                start: d(5),
                finish: d(14)
            }
        );
        let one = resolve(&[id(100)], &[(id(100), id(3))], &spans());
        assert_eq!(
            one[&id(100)],
            Span::Scheduled {
                start: d(6),
                finish: d(6)
            }
        );
    }

    #[test]
    fn a_hammock_with_no_links_is_unscheduled_not_a_made_up_span() {
        let r = resolve(&[id(100)], &[], &spans());
        assert_eq!(r[&id(100)], Span::Unscheduled { reason: "no_links" });
    }

    #[test]
    fn unscheduled_members_are_ignored_and_all_unscheduled_means_no_span() {
        // id(9) is not scheduled.
        let r = resolve(
            &[id(100), id(101)],
            &[(id(100), id(1)), (id(100), id(9)), (id(101), id(9))],
            &spans(),
        );
        assert_eq!(
            r[&id(100)],
            Span::Scheduled {
                start: d(5),
                finish: d(7)
            }
        );
        assert_eq!(
            r[&id(101)],
            Span::Unscheduled {
                reason: "members_unscheduled"
            }
        );
    }

    #[test]
    fn removing_a_link_recomputes() {
        let with = resolve(&[id(100)], &[(id(100), id(1)), (id(100), id(2))], &spans());
        let without = resolve(&[id(100)], &[(id(100), id(1))], &spans());
        assert_eq!(
            with[&id(100)],
            Span::Scheduled {
                start: d(5),
                finish: d(14)
            }
        );
        assert_eq!(
            without[&id(100)],
            Span::Scheduled {
                start: d(5),
                finish: d(7)
            }
        );
    }

    #[test]
    fn hammocks_nest() {
        let r = resolve(
            &[id(100), id(101)],
            &[(id(101), id(100)), (id(100), id(2)), (id(101), id(1))],
            &spans(),
        );
        assert_eq!(
            r[&id(100)],
            Span::Scheduled {
                start: d(8),
                finish: d(14)
            }
        );
        assert_eq!(
            r[&id(101)],
            Span::Scheduled {
                start: d(5),
                finish: d(14)
            },
            "the outer one spans the inner one and its own link"
        );
    }

    #[test]
    fn a_hammock_cannot_link_to_itself_or_close_a_cycle() {
        let links = [(id(100), id(101)), (id(101), id(102))];
        assert!(would_cycle(&links, id(100), id(100)));
        assert!(
            would_cycle(&links, id(102), id(100)),
            "100 → 101 → 102 → 100"
        );
        assert!(would_cycle(&links, id(101), id(100)));
        assert!(
            !would_cycle(&links, id(100), id(102)),
            "a second path is not a cycle"
        );
        assert!(!would_cycle(&links, id(100), id(1)));
    }

    #[test]
    fn a_cycle_that_somehow_exists_does_not_hang_or_invent_a_span() {
        let r = resolve(
            &[id(100), id(101)],
            &[(id(100), id(101)), (id(101), id(100))],
            &spans(),
        );
        assert!(matches!(r[&id(100)], Span::Unscheduled { .. }));
    }
}

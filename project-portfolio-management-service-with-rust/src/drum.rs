//! The drum and the pipeline (CC-5). Pure rules; the database work is in [`crate::controllers::drum`].
//!
//! Across a portfolio the **drum** is the constrained resource: here a skill pool (DP-10). Each project
//! asks the drum for some capacity on some days; when the projects together ask for more than the drum
//! has, the projects are **staggered**: a later project's start is proposed to move until the drum can
//! carry it, leaving a **capacity buffer** (calendar days) after the earlier project's last drum use.
//! Projects are taken in order of when they first need the drum, then by name. The proposal is a pure
//! function of its input; nothing is ever moved.
//!
//! Capacity is counted in milli-FTE and days are calendar days, as in the capacity heatmap.

#![expect(missing_docs)]

use chrono::{Duration, NaiveDate};
use serde::Serialize;
use uuid::Uuid;

/// Capacity asked of the drum over a range of days (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub milli_fte: i64,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub plan: Uuid,
    pub name: String,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    /// The drum can carry it where it is.
    AsPlanned,
    /// Proposed to start later.
    Staggered,
    /// It asks more than the drum has even alone; staggering cannot help.
    ExceedsDrum,
    /// No start within the search was found.
    NoSlot,
}

#[derive(Debug, Clone, Serialize)]
pub struct Placement {
    pub plan: Uuid,
    pub name: String,
    pub outcome: Outcome,
    pub first_use: NaiveDate,
    pub last_use: NaiveDate,
    pub peak_milli_fte: i64,
    pub delay_days: i64,
    pub proposed_first_use: NaiveDate,
    /// In words: what it needs, what it overlaps and what moved.
    pub explanation: String,
}

const MAX_CANDIDATES: usize = 400;
const MAX_DELAY_DAYS: i64 = 1_500;

fn shift(s: &Segment, days: i64) -> Segment {
    Segment {
        start: s.start + Duration::days(days),
        end: s.end + Duration::days(days),
        milli_fte: s.milli_fte,
    }
}

fn load_at(day: NaiveDate, segs: &[Segment]) -> i64 {
    segs.iter()
        .filter(|s| s.start <= day && day <= s.end)
        .map(|s| s.milli_fte)
        .sum()
}

/// The highest load, taken at every day a segment starts.
fn peak(segs: &[Segment]) -> i64 {
    segs.iter()
        .map(|s| load_at(s.start, segs))
        .max()
        .unwrap_or(0)
}

fn fte(m: i64) -> String {
    format!("{}.{:02}", m / 1000, (m.abs() % 1000) / 10)
}

/// Stagger `projects` against a drum of `capacity_milli` with `buffer_days` between successive projects.
#[must_use]
#[expect(clippy::missing_panics_doc, clippy::too_many_lines)]
pub fn stagger(projects: &[Project], capacity_milli: i64, buffer_days: i64) -> Vec<Placement> {
    let mut order: Vec<&Project> = projects.iter().filter(|p| !p.segments.is_empty()).collect();
    order.sort_by_key(|p| {
        (
            p.segments.iter().map(|s| s.start).min(),
            p.name.to_lowercase(),
            p.plan,
        )
    });
    let mut placed: Vec<(Uuid, String, Vec<Segment>)> = Vec::new();
    let mut out = Vec::new();
    for p in order {
        let first = p.segments.iter().map(|s| s.start).min().expect("non-empty");
        let last = p.segments.iter().map(|s| s.end).max().expect("non-empty");
        let own_peak = peak(&p.segments);
        let all_placed: Vec<Segment> = placed
            .iter()
            .flat_map(|(_, _, s)| s.iter().copied())
            .collect();
        let fits = |delay: i64| {
            let mut all = all_placed.clone();
            all.extend(p.segments.iter().map(|s| shift(s, delay)));
            peak(&all) <= capacity_milli
        };
        let placement = |outcome: Outcome, delay: i64, explanation: String| Placement {
            plan: p.plan,
            name: p.name.clone(),
            outcome,
            first_use: first,
            last_use: last,
            peak_milli_fte: own_peak,
            delay_days: delay,
            proposed_first_use: first + Duration::days(delay),
            explanation,
        };
        let needs = format!(
            "{} needs up to {} FTE of the drum from {first} to {last}",
            p.name,
            fte(own_peak)
        );
        if own_peak > capacity_milli {
            placed.push((p.plan, p.name.clone(), p.segments.clone()));
            out.push(placement(
                Outcome::ExceedsDrum,
                0,
                format!(
                    "{needs}, and the drum has {} FTE: staggering cannot fix it, because the project alone asks for more than the drum has.",
                    fte(capacity_milli)
                ),
            ));
            continue;
        }
        if fits(0) {
            placed.push((p.plan, p.name.clone(), p.segments.clone()));
            out.push(placement(
                Outcome::AsPlanned,
                0,
                format!("{needs}; the drum ({} FTE) carries it with the projects already placed, so it keeps its dates.", fte(capacity_milli)),
            ));
            continue;
        }
        // Who it collides with where it is.
        let blockers: Vec<String> = placed
            .iter()
            .filter(|(_, _, segs)| {
                segs.iter().any(|a| {
                    p.segments
                        .iter()
                        .any(|b| a.start <= b.end && b.start <= a.end)
                })
            })
            .map(|(_, n, _)| n.clone())
            .collect();
        let mut candidates: Vec<(i64, String)> = placed
            .iter()
            .flat_map(|(_, name, segs)| {
                segs.iter().map(move |s| {
                    (
                        (s.end + Duration::days(1 + buffer_days) - first).num_days(),
                        name.clone(),
                    )
                })
            })
            .filter(|(d, _)| *d > 0 && *d <= MAX_DELAY_DAYS)
            .collect();
        candidates.sort();
        candidates.dedup_by_key(|c| c.0);
        candidates.truncate(MAX_CANDIDATES);
        if let Some((d, after)) = candidates.into_iter().find(|(d, _)| fits(*d)) {
            {
                let moved: Vec<Segment> = p.segments.iter().map(|s| shift(s, d)).collect();
                placed.push((p.plan, p.name.clone(), moved));
                out.push(placement(
                    Outcome::Staggered,
                    d,
                    format!(
                        "{needs}, while {} already use{} the drum in those days and it has {} FTE. Proposed: start {d} day{} later, after {after}'s last drum use plus a {buffer_days}-day capacity buffer. This is a proposal; no date has been changed.",
                        blockers.join(" and "),
                        if blockers.len() == 1 { "s" } else { "" },
                        fte(capacity_milli),
                        if d == 1 { "" } else { "s" },
                    ),
                ));
            }
        } else {
            {
                placed.push((p.plan, p.name.clone(), p.segments.clone()));
                out.push(placement(
                    Outcome::NoSlot,
                    0,
                    format!(
                        "{needs}, while {} use the drum in those days; no start within {MAX_DELAY_DAYS} days fits, so it is left where it is.",
                        blockers.join(" and ")
                    ),
                ));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, day).unwrap()
    }

    fn project(n: u128, name: &str, from: NaiveDate, to: NaiveDate, milli: i64) -> Project {
        Project {
            plan: Uuid::from_u128(n),
            name: name.into(),
            segments: vec![Segment {
                start: from,
                end: to,
                milli_fte: milli,
            }],
        }
    }

    #[test]
    fn two_projects_needing_the_drum_at_once_are_staggered_and_it_is_explained() {
        // The drum has 1.0 FTE. Both ask for 1.0 FTE in March; the later one waits, with a 7-day buffer.
        let ps = [
            project(1, "Apollo", d(3, 2), d(3, 31), 1000),
            project(2, "Gemini", d(3, 9), d(4, 3), 1000),
        ];
        let out = stagger(&ps, 1000, 7);
        assert_eq!(out[0].outcome, Outcome::AsPlanned);
        assert_eq!(out[1].outcome, Outcome::Staggered);
        // Apollo's last use is 31 March; one day after plus 7 is 8 April; Gemini's first use is 9 March.
        assert_eq!(out[1].proposed_first_use, d(4, 8));
        assert_eq!(out[1].delay_days, 30);
        assert!(
            out[1].explanation.contains("Apollo")
                && out[1].explanation.contains("7-day capacity buffer"),
            "{}",
            out[1].explanation
        );
        assert!(out[1].explanation.contains("no date has been changed"));
    }

    #[test]
    fn projects_that_fit_together_keep_their_dates() {
        let ps = [
            project(1, "A", d(3, 2), d(3, 31), 400),
            project(2, "B", d(3, 9), d(4, 3), 500),
        ];
        let out = stagger(&ps, 1000, 7);
        assert!(
            out.iter()
                .all(|p| p.outcome == Outcome::AsPlanned && p.delay_days == 0)
        );
    }

    #[test]
    fn a_project_that_alone_exceeds_the_drum_is_not_staggered() {
        let out = stagger(&[project(1, "Big", d(3, 2), d(3, 31), 2500)], 1000, 7);
        assert_eq!(out[0].outcome, Outcome::ExceedsDrum);
        assert_eq!(out[0].delay_days, 0);
        assert!(out[0].explanation.contains("alone"));
    }

    #[test]
    fn the_order_is_by_first_need_then_name_and_does_not_depend_on_input_order() {
        let a = project(1, "Zed", d(3, 2), d(3, 31), 1000);
        let b = project(2, "Alpha", d(3, 2), d(3, 31), 1000);
        let one = stagger(&[a.clone(), b.clone()], 1000, 0);
        let two = stagger(&[b, a], 1000, 0);
        let names = |v: &[Placement]| {
            v.iter()
                .map(|p| (p.name.clone(), p.delay_days))
                .collect::<Vec<_>>()
        };
        assert_eq!(names(&one), names(&two));
        assert_eq!(one[0].name, "Alpha", "ties go to the name");
        assert_eq!(
            one[1].delay_days, 30,
            "with no buffer it starts the day after"
        );
    }

    #[test]
    fn a_third_project_waits_for_both() {
        let ps = [
            project(1, "A", d(3, 2), d(3, 10), 1000),
            project(2, "B", d(3, 2), d(3, 20), 1000),
            project(3, "C", d(3, 2), d(3, 5), 1000),
        ];
        let out = stagger(&ps, 1000, 0);
        let c = out.iter().find(|p| p.name == "C").unwrap();
        // A is placed first (earliest start, then name), B waits for A, C waits for both.
        assert!(c.proposed_first_use > d(3, 20), "{c:?}");
    }
}

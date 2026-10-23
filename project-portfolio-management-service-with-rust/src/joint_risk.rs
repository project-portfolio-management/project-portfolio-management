//! Joint cost and schedule Monte Carlo: the pure engine. DB-free, seeded.
//!
//! One simulation in which the schedule and the cost move together. Each
//! iteration draws the schedule ([`crate::schedule_risk`]) and, from the **same
//! random stream**, the cost of each budget line ([`crate::cost_risk`]); a
//! register risk that has both a schedule impact and a cost impact is **one
//! event that strikes once and drives both**. A finish later than the
//! critical-path finish adds a **delay cost** per business day, at a rate the
//! caller supplies (zero by default, and then the two outcomes are independent).
//!
//! What it adds over the two separate analyses is the **joint** confidence: how
//! often the plan is *both* within budget *and* on time, which is never more than
//! the smaller of the two on their own. It is single-currency (the delay cost has
//! one currency), and inputs are treated as independent, as in both parents.
//! Finishing early gives no credit: delay cost is only charged for business days
//! late.

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::cost_risk::{self, CostEvent, Line, Source};
use crate::cpm::{Edge, Node, WorkCalendar};
use crate::schedule_risk::{self, Estimate, Event, Params, RunError, nearest_rank};

/// Most points kept for the scatter.
const MAX_POINTS: usize = 300;

/// How the iterations fall around "on budget" and "on time".
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Quadrants {
    /// Within budget and not later than the critical-path finish.
    pub both: usize,
    /// Within budget but late.
    pub budget_only: usize,
    /// On time but over budget.
    pub time_only: usize,
    /// Over budget and late.
    pub neither: usize,
}

/// The joint outcome.
#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    /// The schedule side, exactly as the schedule simulation reports it.
    pub schedule: schedule_risk::Outcome,
    /// The currency of everything below.
    pub currency: String,
    /// The budget: the sum of the lines' planned amounts.
    pub budget_minor: i64,
    /// Already spent.
    pub spent_minor: i64,
    /// Delay cost per business day late.
    pub delay_cost_per_day_minor: i64,
    /// Mean total cost, delay cost included.
    pub mean_cost_minor: i64,
    /// Mean delay cost on its own.
    pub mean_delay_cost_minor: i64,
    /// Total cost at each percentile (10, 50, 80, 90), delay cost included.
    pub cost_percentiles: Vec<(u32, i64)>,
    /// Share within budget, basis points.
    pub budget_confidence_bp: i64,
    /// Share not later than the critical-path finish, basis points.
    pub finish_confidence_bp: i64,
    /// Share both within budget and on time, basis points.
    pub joint_confidence_bp: i64,
    /// Iteration counts in each quadrant.
    pub quadrants: Quadrants,
    /// A thinned sample of `(business days late, total cost)` for a scatter.
    pub points: Vec<(i64, i64)>,
    /// Per-line results (as in the cost simulation).
    pub lines: Vec<cost_risk::LineResult>,
}

/// Why the run could not start.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum JointError {
    /// The schedule side could not run.
    Schedule(RunError),
    /// There is no budget line to cost.
    NoLines,
}

/// Run both models together.
///
/// `lines` and `cost_events` must be in one currency.
///
/// # Errors
///
/// [`JointError`] when either side cannot run, or the work is over the caps.
#[expect(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn simulate<S: std::hash::BuildHasher>(
    nodes: &[Node],
    edges: &[Edge],
    calendar: &WorkCalendar,
    own: &[WorkCalendar],
    estimates: &HashMap<Uuid, Estimate, S>,
    schedule_events: &[Event],
    lines: &[Line],
    cost_events: &[CostEvent],
    delay_cost_per_day_minor: i64,
    params: Params,
) -> Result<Outcome, JointError> {
    let Some(first) = lines.first() else {
        return Err(JointError::NoLines);
    };
    let currency = first.currency.clone();
    if params
        .iterations
        .saturating_mul(lines.len() + cost_events.len())
        > cost_risk::MAX_WORK
    {
        return Err(JointError::Schedule(RunError::TooMuchWork(format!(
            "iterations x (lines + events) may be at most {}; lower the iterations",
            cost_risk::MAX_WORK
        ))));
    }
    let spreads: Vec<(i64, i64, i64, Source, bool)> = lines
        .iter()
        .map(|l| cost_risk::spread(l, params.fallback_band_percent))
        .collect();
    let schedule_risks: std::collections::HashSet<Uuid> =
        schedule_events.iter().map(|e| e.risk).collect();
    let budget: i64 = lines.iter().map(|l| l.likely).sum();
    let spent: i64 = lines.iter().map(|l| l.actual).sum();

    let n = params.iterations;
    let mut costs: Vec<i64> = Vec::with_capacity(n);
    let mut delays: Vec<i64> = Vec::with_capacity(n);
    let mut delay_costs: Vec<i64> = Vec::with_capacity(n);
    let mut line_costs: Vec<Vec<f64>> = vec![Vec::with_capacity(n); lines.len()];
    let mut quad = Quadrants::default();
    let schedule = schedule_risk::simulate_with(
        nodes,
        edges,
        calendar,
        own,
        estimates,
        schedule_events,
        params,
        |_, _, delay, struck, rng| {
            let mut total = 0_i64;
            for (i, l) in lines.iter().enumerate() {
                let (lo, m, hi, _, _) = spreads[i];
                #[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
                let drawn = if hi <= lo {
                    m
                } else {
                    schedule_risk::draw(rng, lo as f64, m as f64, hi as f64, params.distribution)
                        .round() as i64
                };
                let cost = drawn.max(l.actual);
                total += cost;
                #[expect(clippy::cast_precision_loss)]
                line_costs[i].push(cost as f64);
            }
            for e in cost_events.iter().filter(|e| e.probability_bp > 0) {
                // A risk with a schedule effect strikes once, for both models.
                let hit = if schedule_risks.contains(&e.risk) {
                    struck.contains(&e.risk)
                } else {
                    #[expect(clippy::cast_precision_loss)]
                    let p = e.probability_bp as f64 / 10_000.0;
                    rng.unit() < p
                };
                if hit {
                    total += e.impact_minor;
                }
            }
            let delay_cost = delay.max(0) * delay_cost_per_day_minor;
            total += delay_cost;
            let (ok_cost, ok_time) = (total <= budget, delay <= 0);
            match (ok_cost, ok_time) {
                (true, true) => quad.both += 1,
                (true, false) => quad.budget_only += 1,
                (false, true) => quad.time_only += 1,
                (false, false) => quad.neither += 1,
            }
            costs.push(total);
            delays.push(delay);
            delay_costs.push(delay_cost);
        },
    )
    .map_err(JointError::Schedule)?;

    let n_i = i64::try_from(n).unwrap_or(1);
    let mut sorted = costs.clone();
    sorted.sort_unstable();
    let within = i64::try_from(sorted.iter().filter(|c| **c <= budget).count()).unwrap_or(0);
    let on_time = i64::try_from(delays.iter().filter(|d| **d <= 0).count()).unwrap_or(0);
    let total_f: Vec<f64> = {
        #[expect(clippy::cast_precision_loss)]
        let v: Vec<f64> = costs.iter().map(|c| *c as f64).collect();
        v
    };
    let step = n.div_ceil(MAX_POINTS).max(1);
    Ok(Outcome {
        schedule,
        currency,
        budget_minor: budget,
        spent_minor: spent,
        delay_cost_per_day_minor,
        mean_cost_minor: costs.iter().sum::<i64>() / n_i,
        mean_delay_cost_minor: delay_costs.iter().sum::<i64>() / n_i,
        cost_percentiles: [10_u32, 50, 80, 90]
            .iter()
            .map(|p| (*p, nearest_rank(&sorted, *p)))
            .collect(),
        budget_confidence_bp: within * 10_000 / n_i,
        finish_confidence_bp: on_time * 10_000 / n_i,
        joint_confidence_bp: i64::try_from(quad.both).unwrap_or(0) * 10_000 / n_i,
        quadrants: quad,
        points: (0..n)
            .step_by(step)
            .map(|i| (delays[i], costs[i]))
            .collect(),
        lines: lines
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let (lo, m, hi, source, widened) = spreads[i];
                cost_risk::LineResult {
                    pid: l.pid,
                    plan: l.plan,
                    low: lo,
                    likely: m,
                    high: hi,
                    actual: l.actual,
                    source,
                    widened,
                    sensitivity: schedule_risk::correlation(&line_costs[i], &total_f),
                }
            })
            .collect(),
    })
}

/// The finish date as text for a response, if there is one.
#[must_use]
pub fn finish_text(d: Option<NaiveDate>) -> Option<String> {
    d.map(|d| d.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpm::Kind;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn monday() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, 5).unwrap()
    }
    fn node(n: u128, dur: i64, start: bool) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            planned_start: start.then(monday),
            ..Node::default()
        }
    }
    fn chain() -> (Vec<Node>, Vec<Edge>) {
        (
            vec![node(1, 5, true), node(2, 3, false)],
            vec![Edge {
                pid: id(100),
                pred: id(1),
                succ: id(2),
                kind: Kind::Fs,
                lag: 0,
            }],
        )
    }
    fn line(n: u128, likely: i64, lo: i64, hi: i64) -> Line {
        Line {
            pid: id(n),
            plan: id(1),
            currency: "GBP".into(),
            actual: 0,
            likely,
            low: Some(lo),
            high: Some(hi),
        }
    }
    fn est(v: &[(u128, i64, i64)]) -> HashMap<Uuid, Estimate> {
        v.iter()
            .map(|(n, o, p)| {
                (
                    id(*n),
                    Estimate {
                        optimistic: *o,
                        pessimistic: *p,
                    },
                )
            })
            .collect()
    }
    fn run(
        est: &HashMap<Uuid, Estimate>,
        sev: &[Event],
        lines: &[Line],
        cev: &[CostEvent],
        rate: i64,
        params: Params,
    ) -> Outcome {
        let (nodes, edges) = chain();
        simulate(
            &nodes,
            &edges,
            &WorkCalendar::mon_fri(),
            &[],
            est,
            sev,
            lines,
            cev,
            rate,
            params,
        )
        .unwrap()
    }

    #[test]
    fn the_scatter_sample_is_capped() {
        let e = est(&[(1, 3, 9), (2, 2, 6)]);
        let l = [line(10, 10_000, 8_000, 16_000)];
        for it in [100, 2_000, 4_999, 20_000] {
            let o = run(
                &e,
                &[],
                &l,
                &[],
                0,
                Params {
                    iterations: it,
                    ..Params::default()
                },
            );
            assert!(
                !o.points.is_empty() && o.points.len() <= MAX_POINTS,
                "{it}: {}",
                o.points.len()
            );
        }
    }

    #[test]
    fn same_seed_same_result() {
        let e = est(&[(1, 3, 9), (2, 2, 6)]);
        let l = [line(10, 10_000, 8_000, 16_000)];
        let a = run(&e, &[], &l, &[], 500, Params::default());
        let b = run(&e, &[], &l, &[], 500, Params::default());
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }

    #[test]
    fn with_no_uncertainty_the_budget_and_the_date_are_both_met() {
        let e = est(&[(1, 5, 5), (2, 3, 3)]);
        let l = [line(10, 10_000, 10_000, 10_000)];
        let o = run(&e, &[], &l, &[], 1_000, Params::default());
        assert!(o.cost_percentiles.iter().all(|(_, c)| *c == 10_000));
        assert_eq!(
            (
                o.budget_confidence_bp,
                o.finish_confidence_bp,
                o.joint_confidence_bp
            ),
            (10_000, 10_000, 10_000)
        );
        assert_eq!(o.quadrants.both, o.schedule.iterations);
        assert_eq!(o.mean_delay_cost_minor, 0);
    }

    #[test]
    fn the_joint_confidence_is_never_above_either_marginal() {
        let e = est(&[(1, 3, 12), (2, 2, 9)]);
        let l = [line(10, 10_000, 7_000, 18_000)];
        let o = run(&e, &[], &l, &[], 400, Params::default());
        assert!(o.joint_confidence_bp <= o.budget_confidence_bp.min(o.finish_confidence_bp));
        let q = o.quadrants;
        assert_eq!(
            q.both + q.budget_only + q.time_only + q.neither,
            o.schedule.iterations
        );
        // The marginals are the quadrants' sums.
        assert_eq!(
            i64::try_from(q.both + q.budget_only).unwrap() * 10_000
                / i64::try_from(o.schedule.iterations).unwrap(),
            o.budget_confidence_bp
        );
    }

    #[test]
    fn a_delay_costs_money_only_when_late() {
        // Certain 4-business-day slip on the first task, cost certain: every run is
        // 4 business days late.
        let e = est(&[(1, 5, 5), (2, 3, 3)]);
        let l = [line(10, 10_000, 10_000, 10_000)];
        let sev = [Event {
            risk: id(50),
            task: id(1),
            probability_bp: 10_000,
            impact_days: 4,
        }];
        let o = run(&e, &sev, &l, &[], 1_000, Params::default());
        assert!(
            o.cost_percentiles.iter().all(|(_, c)| *c == 14_000),
            "10 000 + 4 x 1 000"
        );
        assert_eq!(o.mean_delay_cost_minor, 4_000);
        assert_eq!(o.quadrants.neither, o.schedule.iterations);
        assert_eq!(o.joint_confidence_bp, 0);
        // Rate zero: late, but no delay cost, so still within budget.
        let free = run(&e, &sev, &l, &[], 0, Params::default());
        assert_eq!(free.budget_confidence_bp, 10_000);
        assert_eq!(free.finish_confidence_bp, 0);
        assert_eq!(free.quadrants.budget_only, free.schedule.iterations);
    }

    #[test]
    fn a_risk_with_both_effects_strikes_once_and_drives_both() {
        let e = est(&[(1, 5, 5), (2, 3, 3)]);
        let l = [line(10, 10_000, 10_000, 10_000)];
        let risk = id(77);
        let sev = [Event {
            risk,
            task: id(1),
            probability_bp: 5_000,
            impact_days: 3,
        }];
        let cev = [CostEvent {
            risk,
            plan: id(1),
            currency: "GBP".into(),
            probability_bp: 5_000,
            impact_minor: 2_000,
        }];
        let o = run(
            &e,
            &sev,
            &l,
            &cev,
            0,
            Params {
                iterations: 4_000,
                ..Params::default()
            },
        );
        // Strikes together: every late run is also over budget, every on-time run within it.
        assert_eq!(o.quadrants.budget_only, 0);
        assert_eq!(o.quadrants.time_only, 0);
        assert!(o.quadrants.both > 1_500 && o.quadrants.neither > 1_500);
    }

    #[test]
    fn a_separate_cost_risk_is_independent_of_the_schedule() {
        let e = est(&[(1, 5, 5), (2, 3, 3)]);
        let l = [line(10, 10_000, 10_000, 10_000)];
        let cev = [CostEvent {
            risk: id(78),
            plan: id(1),
            currency: "GBP".into(),
            probability_bp: 5_000,
            impact_minor: 2_000,
        }];
        let o = run(
            &e,
            &[],
            &l,
            &cev,
            0,
            Params {
                iterations: 4_000,
                ..Params::default()
            },
        );
        assert_eq!(o.finish_confidence_bp, 10_000, "nothing delays it");
        assert_eq!(o.quadrants.budget_only, 0);
        assert!(o.quadrants.both > 1_500 && o.quadrants.time_only > 1_500);
    }

    #[test]
    fn no_lines_or_a_cycle_is_refused() {
        let (nodes, mut edges) = chain();
        let e = est(&[]);
        let params = Params::default();
        assert_eq!(
            simulate(
                &nodes,
                &edges,
                &WorkCalendar::mon_fri(),
                &[],
                &e,
                &[],
                &[],
                &[],
                0,
                params
            )
            .unwrap_err(),
            JointError::NoLines
        );
        edges.push(Edge {
            pid: id(101),
            pred: id(2),
            succ: id(1),
            kind: Kind::Fs,
            lag: 0,
        });
        let l = [line(10, 100, 100, 100)];
        assert!(matches!(
            simulate(
                &nodes,
                &edges,
                &WorkCalendar::mon_fri(),
                &[],
                &e,
                &[],
                &l,
                &[],
                0,
                params
            ),
            Err(JointError::Schedule(RunError::Cycle(_)))
        ));
    }
}

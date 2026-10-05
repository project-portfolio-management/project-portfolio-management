//! Cost-risk Monte Carlo: the pure engine. DB-free, seeded, deterministic.
//!
//! Each iteration draws a **final cost** for every budget line from its low /
//! likely / high estimate (triangular or PERT; the likely one is the line's
//! planned amount), never below what is already spent, then lets each **risk
//! event** strike with its probability and add its cost. Totals are kept **per
//! currency and never added across currencies**.
//!
//! The same engine serves one plan or a whole portfolio (the caller decides which
//! lines and events go in). It reports, per currency: the budget (the sum of the
//! likely amounts), the cost at P10 / P50 / P80 / P90, the chance of finishing
//! within budget, a histogram, a sensitivity correlation per line, and how often
//! each event struck.
//!
//! What it does not claim:
//!
//! * **Inputs are independent.** There is no correlation between lines or events,
//!   so a shared cause (inflation moving every line together) is not modelled and
//!   the spread is narrower than a correlated one would be.
//! * **A line with no estimate** uses a stated fallback band around its planned
//!   amount and is marked; a risk with no probability, impact or currency is not
//!   an event and is named by the caller.
//! * Same seed, same result; with no uncertainty every percentile is the budget
//!   exactly (or the amount already spent, if that is more).

// The sampling maths uses the standard names (o, m, p, u, x).
#![allow(clippy::many_single_char_names)]

use std::collections::BTreeMap;

use serde::Serialize;
use uuid::Uuid;

use crate::schedule_risk::{Distribution, Rng, correlation, draw, nearest_rank};

/// Iterations when none are asked for.
pub const DEFAULT_ITERATIONS: usize = 5_000;
/// The most iterations one run may ask for.
pub const MAX_ITERATIONS: usize = 50_000;
/// The most work (iterations × (lines + events)) one run may ask for.
pub const MAX_WORK: usize = 5_000_000;
/// Histogram bins per currency.
pub const BINS: usize = 20;

/// One budget line.
#[derive(Debug, Clone)]
pub struct Line {
    /// Identity.
    pub pid: Uuid,
    /// The plan it belongs to.
    pub plan: Uuid,
    /// ISO 4217 code; lines in different currencies are never added.
    pub currency: String,
    /// Already spent: the final cost cannot be less.
    pub actual: i64,
    /// The planned amount, used as the likely final cost.
    pub likely: i64,
    /// Low estimate, if given.
    pub low: Option<i64>,
    /// High estimate, if given.
    pub high: Option<i64>,
}

/// A risk that adds cost with a probability.
#[derive(Debug, Clone)]
pub struct CostEvent {
    /// The risk.
    pub risk: Uuid,
    /// Its plan.
    pub plan: Uuid,
    /// The currency of the impact.
    pub currency: String,
    /// Chance it happens, in basis points (0–10 000).
    pub probability_bp: i64,
    /// Cost added, in minor units (positive).
    pub impact_minor: i64,
}

/// Run options.
#[derive(Debug, Clone, Copy)]
pub struct Params {
    /// Iterations (1..=[`MAX_ITERATIONS`]).
    pub iterations: usize,
    /// The seed.
    pub seed: u64,
    /// The draw distribution.
    pub distribution: Distribution,
    /// Band around the planned amount for a line with no estimate, in percent.
    pub fallback_band_percent: i64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            iterations: DEFAULT_ITERATIONS,
            seed: 1,
            distribution: Distribution::Triangular,
            fallback_band_percent: 20,
        }
    }
}

/// Where a line's spread came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// Its own low and/or high.
    Estimated,
    /// The fallback band: it has no estimate.
    Fallback,
}

/// One line's result.
#[derive(Debug, Clone, Serialize)]
pub struct LineResult {
    /// The line.
    pub pid: Uuid,
    /// Its plan.
    pub plan: Uuid,
    /// Low used.
    pub low: i64,
    /// Likely (planned).
    pub likely: i64,
    /// High used.
    pub high: i64,
    /// Already spent.
    pub actual: i64,
    /// Where the spread came from.
    pub source: Source,
    /// The estimate was widened to include the planned amount.
    pub widened: bool,
    /// Correlation of this line's cost with the currency's total (−1..1, two
    /// places), or `None` when either did not vary.
    pub sensitivity: Option<f64>,
}

/// One event's result.
#[derive(Debug, Clone, Serialize)]
pub struct EventResult {
    /// The risk.
    pub risk: Uuid,
    /// Its plan.
    pub plan: Uuid,
    /// Stated probability, basis points.
    pub probability_bp: i64,
    /// Cost added when it strikes.
    pub impact_minor: i64,
    /// How often it struck across the iterations, basis points.
    pub hit_bp: i64,
}

/// One histogram bin.
#[derive(Debug, Clone, Serialize)]
pub struct Bin {
    /// Inclusive lower edge, minor units.
    pub from_minor: i64,
    /// Exclusive upper edge (inclusive for the last bin).
    pub to_minor: i64,
    /// Iterations in it.
    pub count: usize,
}

/// A plan's mean cost within one currency.
#[derive(Debug, Clone, Serialize)]
pub struct PlanShare {
    /// The plan.
    pub plan: Uuid,
    /// Its budget (sum of planned amounts).
    pub budget_minor: i64,
    /// Its mean simulated cost.
    pub mean_minor: i64,
}

/// One currency's outcome.
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyOutcome {
    /// The currency.
    pub currency: String,
    /// The budget: the sum of the lines' planned amounts.
    pub budget_minor: i64,
    /// Already spent.
    pub spent_minor: i64,
    /// Mean simulated cost.
    pub mean_minor: i64,
    /// Cost at each percentile (10, 50, 80, 90).
    pub percentiles: Vec<(u32, i64)>,
    /// Share of iterations at or under the budget, basis points.
    pub budget_confidence_bp: i64,
    /// Lowest simulated cost.
    pub min_minor: i64,
    /// Highest simulated cost.
    pub max_minor: i64,
    /// Equal-width histogram.
    pub histogram: Vec<Bin>,
    /// Per-line results.
    pub lines: Vec<LineResult>,
    /// Per-event results.
    pub events: Vec<EventResult>,
    /// Per-plan mean cost.
    pub plans: Vec<PlanShare>,
}

/// The run.
#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    /// Iterations run.
    pub iterations: usize,
    /// One block per currency, A–Z.
    pub currencies: Vec<CurrencyOutcome>,
}

/// Why the run could not start.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RunError {
    /// Nothing to simulate: no budget lines.
    NoLines,
    /// `iterations` is 0 or over the cap, or the work is over the cap.
    TooMuchWork(String),
}

/// The three-point estimate for a line, and where it came from.
pub(crate) fn spread(l: &Line, band_percent: i64) -> (i64, i64, i64, Source, bool) {
    let m = l.likely.max(0);
    if l.low.is_none() && l.high.is_none() {
        let band = (m * band_percent + 50) / 100;
        return (
            (m - band).max(l.actual.min(m)),
            m,
            m + band,
            Source::Fallback,
            false,
        );
    }
    let lo = l.low.unwrap_or(m);
    let hi = l.high.unwrap_or(m);
    let widened = lo > m || hi < m;
    (lo.min(m), m, hi.max(m), Source::Estimated, widened)
}

/// Run the simulation.
///
/// # Errors
///
/// [`RunError`] when there are no lines or the requested work is over the caps.
#[allow(clippy::too_many_lines)]
pub fn simulate(lines: &[Line], events: &[CostEvent], params: Params) -> Result<Outcome, RunError> {
    if lines.is_empty() {
        return Err(RunError::NoLines);
    }
    if params.iterations == 0 || params.iterations > MAX_ITERATIONS {
        return Err(RunError::TooMuchWork(format!(
            "iterations must be 1 to {MAX_ITERATIONS}"
        )));
    }
    if params.iterations.saturating_mul(lines.len() + events.len()) > MAX_WORK {
        return Err(RunError::TooMuchWork(format!(
            "iterations x (lines + events) may be at most {MAX_WORK}; lower the iterations"
        )));
    }
    let spreads: Vec<(i64, i64, i64, Source, bool)> = lines
        .iter()
        .map(|l| spread(l, params.fallback_band_percent))
        .collect();
    let currencies: Vec<String> = {
        let mut c: Vec<String> = lines.iter().map(|l| l.currency.clone()).collect();
        c.sort();
        c.dedup();
        c
    };
    let cur_ix = |c: &str| currencies.iter().position(|x| x == c);
    // Events in a currency with no budget line have nothing to be added to.
    let events: Vec<&CostEvent> = events
        .iter()
        .filter(|e| e.probability_bp > 0 && cur_ix(&e.currency).is_some())
        .collect();

    let n = params.iterations;
    let mut totals: Vec<Vec<i64>> = vec![Vec::with_capacity(n); currencies.len()];
    let mut line_costs: Vec<Vec<f64>> = vec![Vec::with_capacity(n); lines.len()];
    let mut hits = vec![0usize; events.len()];
    let mut plan_sum: BTreeMap<(usize, Uuid), i128> = BTreeMap::new();
    let mut rng = Rng::new(params.seed);
    for _ in 0..n {
        let mut sum = vec![0_i64; currencies.len()];
        let mut by_plan: BTreeMap<(usize, Uuid), i64> = BTreeMap::new();
        for (i, l) in lines.iter().enumerate() {
            let (lo, m, hi, _, _) = spreads[i];
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            let drawn = if hi <= lo {
                m
            } else {
                draw(
                    &mut rng,
                    lo as f64,
                    m as f64,
                    hi as f64,
                    params.distribution,
                )
                .round() as i64
            };
            // What is spent is spent.
            let cost = drawn.max(l.actual);
            let ci = cur_ix(&l.currency).unwrap_or(0);
            sum[ci] += cost;
            *by_plan.entry((ci, l.plan)).or_default() += cost;
            #[allow(clippy::cast_precision_loss)]
            line_costs[i].push(cost as f64);
        }
        for (k, e) in events.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let p = e.probability_bp as f64 / 10_000.0;
            if rng.unit() < p {
                hits[k] += 1;
                let ci = cur_ix(&e.currency).unwrap_or(0);
                sum[ci] += e.impact_minor;
                *by_plan.entry((ci, e.plan)).or_default() += e.impact_minor;
            }
        }
        for (ci, t) in sum.iter().enumerate() {
            totals[ci].push(*t);
        }
        for (k, v) in by_plan {
            *plan_sum.entry(k).or_default() += i128::from(v);
        }
    }

    let n_i = i128::try_from(n).unwrap_or(1);
    let mut out = Vec::with_capacity(currencies.len());
    for (ci, cur) in currencies.iter().enumerate() {
        let mine: Vec<usize> = (0..lines.len())
            .filter(|i| lines[*i].currency == *cur)
            .collect();
        let budget: i64 = mine.iter().map(|i| lines[*i].likely).sum();
        let spent: i64 = mine.iter().map(|i| lines[*i].actual).sum();
        let mut sorted = totals[ci].clone();
        sorted.sort_unstable();
        let (min, max) = (sorted[0], sorted[sorted.len() - 1]);
        let mean = i64::try_from(totals[ci].iter().map(|v| i128::from(*v)).sum::<i128>() / n_i)
            .unwrap_or(0);
        let within = sorted.iter().filter(|v| **v <= budget).count();
        let width = ((max - min) / i64::try_from(BINS).unwrap_or(20)).max(1);
        let mut histogram: Vec<Bin> = (0..BINS)
            .map(|b| {
                let from = min + width * i64::try_from(b).unwrap_or(0);
                Bin {
                    from_minor: from,
                    to_minor: from + width,
                    count: 0,
                }
            })
            .collect();
        for v in &sorted {
            let b = usize::try_from((v - min) / width)
                .unwrap_or(0)
                .min(BINS - 1);
            histogram[b].count += 1;
        }
        let total_f: Vec<f64> = {
            #[allow(clippy::cast_precision_loss)]
            let v: Vec<f64> = totals[ci].iter().map(|t| *t as f64).collect();
            v
        };
        out.push(CurrencyOutcome {
            currency: cur.clone(),
            budget_minor: budget,
            spent_minor: spent,
            mean_minor: mean,
            percentiles: [10_u32, 50, 80, 90]
                .iter()
                .map(|p| (*p, nearest_rank(&sorted, *p)))
                .collect(),
            budget_confidence_bp: i64::try_from(within * 10_000 / n).unwrap_or(0),
            min_minor: min,
            max_minor: max,
            histogram,
            lines: mine
                .iter()
                .map(|i| {
                    let (lo, m, hi, source, widened) = spreads[*i];
                    LineResult {
                        pid: lines[*i].pid,
                        plan: lines[*i].plan,
                        low: lo,
                        likely: m,
                        high: hi,
                        actual: lines[*i].actual,
                        source,
                        widened,
                        sensitivity: correlation(&line_costs[*i], &total_f),
                    }
                })
                .collect(),
            events: events
                .iter()
                .enumerate()
                .filter(|(_, e)| e.currency == *cur)
                .map(|(k, e)| EventResult {
                    risk: e.risk,
                    plan: e.plan,
                    probability_bp: e.probability_bp,
                    impact_minor: e.impact_minor,
                    hit_bp: i64::try_from(hits[k] * 10_000 / n).unwrap_or(0),
                })
                .collect(),
            plans: {
                let mut plans: Vec<Uuid> = lines
                    .iter()
                    .filter(|l| l.currency == *cur)
                    .map(|l| l.plan)
                    .collect();
                plans.sort();
                plans.dedup();
                plans
                    .into_iter()
                    .map(|plan| PlanShare {
                        plan,
                        budget_minor: lines
                            .iter()
                            .filter(|l| l.currency == *cur && l.plan == plan)
                            .map(|l| l.likely)
                            .sum(),
                        mean_minor: i64::try_from(
                            plan_sum.get(&(ci, plan)).copied().unwrap_or(0) / n_i,
                        )
                        .unwrap_or(0),
                    })
                    .collect()
            },
        });
    }
    Ok(Outcome {
        iterations: n,
        currencies: out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn line(n: u128, plan: u128, cur: &str, likely: i64) -> Line {
        Line {
            pid: id(n),
            plan: id(plan),
            currency: cur.to_string(),
            actual: 0,
            likely,
            low: None,
            high: None,
        }
    }
    fn est(mut l: Line, lo: i64, hi: i64) -> Line {
        l.low = Some(lo);
        l.high = Some(hi);
        l
    }
    fn p(o: &Outcome, cur: &str, pct: u32) -> i64 {
        o.currencies
            .iter()
            .find(|c| c.currency == cur)
            .unwrap()
            .percentiles
            .iter()
            .find(|(q, _)| *q == pct)
            .unwrap()
            .1
    }
    fn cur<'a>(o: &'a Outcome, c: &str) -> &'a CurrencyOutcome {
        o.currencies.iter().find(|x| x.currency == c).unwrap()
    }

    #[test]
    fn same_seed_same_result_another_seed_differs() {
        let lines = [
            est(line(1, 1, "GBP", 10_000), 8_000, 15_000),
            est(line(2, 1, "GBP", 5_000), 4_000, 9_000),
        ];
        let a = simulate(&lines, &[], Params::default()).unwrap();
        let b = simulate(&lines, &[], Params::default()).unwrap();
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
        let c = simulate(
            &lines,
            &[],
            Params {
                seed: 9,
                ..Params::default()
            },
        )
        .unwrap();
        assert_ne!(
            serde_json::to_string(&a.currencies[0].histogram).unwrap(),
            serde_json::to_string(&c.currencies[0].histogram).unwrap()
        );
    }

    #[test]
    fn with_no_uncertainty_every_percentile_is_the_budget() {
        let lines = [
            est(line(1, 1, "GBP", 10_000), 10_000, 10_000),
            est(line(2, 1, "GBP", 5_000), 5_000, 5_000),
        ];
        for dist in [Distribution::Triangular, Distribution::Pert] {
            let o = simulate(
                &lines,
                &[],
                Params {
                    distribution: dist,
                    ..Params::default()
                },
            )
            .unwrap();
            for q in [10, 50, 80, 90] {
                assert_eq!(p(&o, "GBP", q), 15_000);
            }
            assert_eq!(cur(&o, "GBP").budget_confidence_bp, 10_000);
            assert_eq!(cur(&o, "GBP").budget_minor, 15_000);
        }
        let o = simulate(
            &[line(1, 1, "GBP", 10_000)],
            &[],
            Params {
                fallback_band_percent: 0,
                ..Params::default()
            },
        )
        .unwrap();
        assert_eq!(p(&o, "GBP", 90), 10_000);
    }

    #[test]
    fn percentiles_are_ordered_and_inside_the_range() {
        let lines = [est(line(1, 1, "GBP", 10_000), 5_000, 30_000)];
        let o = simulate(&lines, &[], Params::default()).unwrap();
        let c = cur(&o, "GBP");
        let v: Vec<i64> = c.percentiles.iter().map(|(_, x)| *x).collect();
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
        assert!(c.min_minor <= v[0] && v[3] <= c.max_minor);
        assert!(c.min_minor >= 5_000 && c.max_minor <= 30_000);
        assert_eq!(
            c.histogram.iter().map(|b| b.count).sum::<usize>(),
            o.iterations
        );
        assert_eq!(c.histogram.len(), BINS);
    }

    #[test]
    fn what_is_spent_is_never_given_back() {
        // Planned 10 000, low 4 000, but 9 000 is already spent: no outcome is under 9 000.
        let mut l = est(line(1, 1, "GBP", 10_000), 4_000, 12_000);
        l.actual = 9_000;
        let o = simulate(&[l], &[], Params::default()).unwrap();
        assert!(cur(&o, "GBP").min_minor >= 9_000);
        // Spent more than planned and no spread: the total is what was spent.
        let mut over = est(line(2, 1, "GBP", 10_000), 10_000, 10_000);
        over.actual = 13_000;
        let o = simulate(&[over], &[], Params::default()).unwrap();
        assert_eq!(p(&o, "GBP", 50), 13_000);
        assert_eq!(
            cur(&o, "GBP").budget_confidence_bp,
            0,
            "already over budget"
        );
    }

    #[test]
    fn currencies_are_never_added_together() {
        let lines = [
            est(line(1, 1, "GBP", 10_000), 9_000, 12_000),
            est(line(2, 1, "USD", 500), 400, 900),
        ];
        let o = simulate(&lines, &[], Params::default()).unwrap();
        assert_eq!(o.currencies.len(), 2);
        assert_eq!(cur(&o, "GBP").budget_minor, 10_000);
        assert_eq!(cur(&o, "USD").budget_minor, 500);
        assert!(cur(&o, "GBP").max_minor <= 12_000 && cur(&o, "USD").max_minor <= 900);
    }

    #[test]
    fn a_risk_event_adds_its_cost_with_its_probability() {
        let lines = [est(line(1, 1, "GBP", 10_000), 10_000, 10_000)];
        let ev = |bp: i64| {
            [CostEvent {
                risk: id(9),
                plan: id(1),
                currency: "GBP".into(),
                probability_bp: bp,
                impact_minor: 4_000,
            }]
        };
        let none = simulate(&lines, &ev(0), Params::default()).unwrap();
        assert_eq!(p(&none, "GBP", 90), 10_000);
        assert!(
            cur(&none, "GBP").events.is_empty(),
            "probability zero is not an event"
        );
        let always = simulate(&lines, &ev(10_000), Params::default()).unwrap();
        assert_eq!(p(&always, "GBP", 10), 14_000);
        assert_eq!(cur(&always, "GBP").budget_confidence_bp, 0);
        assert_eq!(cur(&always, "GBP").events[0].hit_bp, 10_000);
        let half = simulate(&lines, &ev(5_000), Params::default()).unwrap();
        let c = cur(&half, "GBP");
        assert!(
            (4_500..=5_500).contains(&c.events[0].hit_bp),
            "{}",
            c.events[0].hit_bp
        );
        assert!((4_500..=5_500).contains(&c.budget_confidence_bp));
    }

    #[test]
    fn an_event_in_a_currency_with_no_budget_is_dropped() {
        let lines = [line(1, 1, "GBP", 1_000)];
        let ev = [CostEvent {
            risk: id(9),
            plan: id(1),
            currency: "EUR".into(),
            probability_bp: 10_000,
            impact_minor: 500,
        }];
        let o = simulate(&lines, &ev, Params::default()).unwrap();
        assert_eq!(o.currencies.len(), 1);
        assert!(cur(&o, "GBP").events.is_empty());
    }

    #[test]
    fn the_fallback_band_is_used_and_marked_for_a_line_with_no_estimate() {
        let lines = [
            line(1, 1, "GBP", 10_000),
            est(line(2, 1, "GBP", 5_000), 4_000, 6_000),
        ];
        let o = simulate(&lines, &[], Params::default()).unwrap();
        let c = cur(&o, "GBP");
        let a = c.lines.iter().find(|l| l.pid == id(1)).unwrap();
        assert_eq!(
            (a.source, a.low, a.likely, a.high),
            (Source::Fallback, 8_000, 10_000, 12_000)
        );
        assert_eq!(
            c.lines.iter().find(|l| l.pid == id(2)).unwrap().source,
            Source::Estimated
        );
    }

    #[test]
    fn an_estimate_not_around_the_planned_amount_is_widened() {
        let l = est(line(1, 1, "GBP", 10_000), 11_000, 9_000);
        let o = simulate(&[l], &[], Params::default()).unwrap();
        let r = &cur(&o, "GBP").lines[0];
        assert!(r.widened);
        assert_eq!((r.low, r.high), (10_000, 10_000));
    }

    #[test]
    fn the_driver_line_is_the_sensitive_one_and_plans_reconcile() {
        let lines = [
            est(line(1, 1, "GBP", 10_000), 5_000, 40_000),
            est(line(2, 2, "GBP", 5_000), 5_000, 5_000),
        ];
        let o = simulate(&lines, &[], Params::default()).unwrap();
        let c = cur(&o, "GBP");
        assert!(
            c.lines
                .iter()
                .find(|l| l.pid == id(1))
                .unwrap()
                .sensitivity
                .unwrap()
                > 0.9
        );
        assert_eq!(
            c.lines.iter().find(|l| l.pid == id(2)).unwrap().sensitivity,
            None
        );
        assert_eq!(c.plans.len(), 2);
        let by_plan: i64 = c.plans.iter().map(|x| x.mean_minor).sum();
        assert!(
            (by_plan - c.mean_minor).abs() <= 1,
            "per-plan means sum to the total mean"
        );
        assert_eq!(
            c.plans.iter().map(|x| x.budget_minor).sum::<i64>(),
            c.budget_minor
        );
    }

    #[test]
    fn caps_are_refused() {
        let lines = [line(1, 1, "GBP", 100)];
        let go = |it| {
            simulate(
                &lines,
                &[],
                Params {
                    iterations: it,
                    ..Params::default()
                },
            )
        };
        assert!(matches!(go(0), Err(RunError::TooMuchWork(_))));
        assert!(matches!(
            go(MAX_ITERATIONS + 1),
            Err(RunError::TooMuchWork(_))
        ));
        assert_eq!(
            simulate(&[], &[], Params::default()).unwrap_err(),
            RunError::NoLines
        );
    }
}

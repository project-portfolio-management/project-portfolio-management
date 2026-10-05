//! Schedule-risk Monte Carlo (SC-8): the pure engine. DB-free, seeded,
//! deterministic.
//!
//! Each iteration draws a duration for every task from its optimistic / likely /
//! pessimistic estimate (triangular or PERT), lets each **risk event** strike with
//! its probability (adding its working days to the task it names), and re-runs the
//! critical-path analysis ([`crate::cpm`]) on those durations. Across iterations
//! it reports the finish-date percentiles, how often each task was critical (its
//! **criticality index**), and how closely each task's duration tracks the finish
//! (a **sensitivity** correlation).
//!
//! # What it does not claim
//!
//! * **Inputs are independent.** Every draw is made without regard to any other:
//!   there is no correlation between tasks, so a shared cause (one team slipping
//!   everything) is not modelled and the spread is narrower than a correlated
//!   one would be. The response says so.
//! * **A task with no estimate** uses a stated fallback — the likely duration
//!   plus and minus a band — and is listed as such, never treated as certain.
//! * **A risk event with no probability** (or no task, or no impact) is skipped
//!   and named, never guessed at.
//! * The same seed gives the same result; with no uncertainty at all every
//!   iteration is the deterministic CPM schedule, so every percentile equals the
//!   CPM finish exactly.

// The sampling maths uses the standard names (o, m, p, u, a, b, c, d, x, v).
#![allow(clippy::many_single_char_names)]

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::cpm::{CpmError, Edge, Node, Options, WorkCalendar, analyze_with};

/// Iterations when none are asked for.
pub const DEFAULT_ITERATIONS: usize = 2_000;
/// The most iterations one run may ask for.
pub const MAX_ITERATIONS: usize = 20_000;
/// The most work (iterations × nodes) one run may ask for.
pub const MAX_WORK: usize = 5_000_000;
/// The seed when none is given.
pub const DEFAULT_SEED: u64 = 1;
/// The fallback band for a task with no estimate, in percent of its duration.
pub const DEFAULT_BAND_PERCENT: i64 = 20;
/// Most distinct finish dates returned in the histogram.
const MAX_HISTOGRAM: usize = 400;

/// How an estimate is turned into a draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    /// Triangular over optimistic, likely, pessimistic.
    Triangular,
    /// Beta-PERT (weight 4): likelier to stay near the likely value.
    Pert,
}

impl Distribution {
    /// Parse `triangular` or `pert`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "triangular" => Some(Self::Triangular),
            "pert" => Some(Self::Pert),
            _ => None,
        }
    }
}

/// One task's three-point estimate, in working days.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Estimate {
    /// Optimistic.
    pub optimistic: i64,
    /// Pessimistic.
    pub pessimistic: i64,
}

/// A risk that strikes a named task with a probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    /// The risk's identity.
    pub risk: Uuid,
    /// The task it adds time to.
    pub task: Uuid,
    /// Chance it happens, in basis points (0–10 000).
    pub probability_bp: i64,
    /// Working days it adds.
    pub impact_days: i64,
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
    /// The band for a task with no estimate, in percent of its likely duration.
    pub fallback_band_percent: i64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            iterations: DEFAULT_ITERATIONS,
            seed: DEFAULT_SEED,
            distribution: Distribution::Triangular,
            fallback_band_percent: DEFAULT_BAND_PERCENT,
        }
    }
}

/// Where a task's spread came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// Its own optimistic and/or pessimistic estimate.
    Estimated,
    /// The fallback band: it has no estimate.
    Fallback,
    /// A milestone (zero duration): nothing to vary.
    Milestone,
}

/// One task's result.
#[derive(Debug, Clone, Serialize)]
pub struct TaskResult {
    /// The task.
    pub pid: Uuid,
    /// Optimistic duration used.
    pub optimistic: i64,
    /// Likely duration.
    pub likely: i64,
    /// Pessimistic duration used.
    pub pessimistic: i64,
    /// Where the spread came from.
    pub source: Source,
    /// The estimate was not ordered around the likely value and was widened to
    /// include it.
    pub widened: bool,
    /// Share of iterations in which the task had no float, in basis points.
    pub criticality_bp: i64,
    /// Correlation of its duration with the project finish (−1..1, rounded to
    /// 2 places), or `None` when either did not vary.
    pub sensitivity: Option<f64>,
}

/// The finish at a percentile.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Percentile {
    /// The percentile (10, 50, 80, 90).
    pub p: u32,
    /// The finish date at least this share of iterations finished by.
    pub finish: NaiveDate,
}

/// The run.
#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    /// Iterations run.
    pub iterations: usize,
    /// The deterministic critical-path finish.
    pub cpm_finish: Option<NaiveDate>,
    /// Finish percentiles, ascending.
    pub percentiles: Vec<Percentile>,
    /// Share of iterations finishing on or before the CPM finish, in basis
    /// points: how likely the plan's own date is.
    pub cpm_finish_confidence_bp: i64,
    /// Earliest simulated finish.
    pub earliest: Option<NaiveDate>,
    /// Latest simulated finish.
    pub latest: Option<NaiveDate>,
    /// Finish date counts, in date order.
    pub histogram: Vec<(NaiveDate, usize)>,
    /// Per-task results, in input order.
    pub tasks: Vec<TaskResult>,
}

/// Why the run could not start.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RunError {
    /// The dependencies contain a cycle.
    Cycle(Vec<Uuid>),
    /// Nothing could be scheduled.
    NothingScheduled,
    /// `iterations` is 0 or over the cap, or the work is over the cap.
    TooMuchWork(String),
}

/// A deterministic xorshift64* generator, its seed scrambled so that nearby
/// seeds give unrelated streams.
pub struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        // SplitMix64 finaliser; never zero.
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        Self(if z == 0 { 0x9E37_79B9_7F4A_7C15 } else { z })
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in `(0, 1)`.
    pub(crate) fn unit(&mut self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let v = ((self.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
        v
    }

    /// Standard normal (Box–Muller).
    fn normal(&mut self) -> f64 {
        let (u1, u2) = (self.unit(), self.unit());
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }

    /// Gamma(shape >= 1, 1) (Marsaglia–Tsang).
    fn gamma(&mut self, shape: f64) -> f64 {
        let d = shape - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = self.normal();
            let v = (1.0 + c * x).powi(3);
            if v <= 0.0 {
                continue;
            }
            let u = self.unit();
            if u.ln() < 0.5 * x * x + d - d * v + d * v.ln() {
                return d * v;
            }
        }
    }
}

/// Draw a duration in `[o, p]` with mode `m`.
pub(crate) fn draw(rng: &mut Rng, o: f64, m: f64, p: f64, dist: Distribution) -> f64 {
    if p <= o {
        return o;
    }
    match dist {
        Distribution::Triangular => {
            let u = rng.unit();
            let f = (m - o) / (p - o);
            if u < f {
                o + (u * (p - o) * (m - o)).sqrt()
            } else {
                p - ((1.0 - u) * (p - o) * (p - m)).sqrt()
            }
        }
        Distribution::Pert => {
            let a = 1.0 + 4.0 * (m - o) / (p - o);
            let b = 1.0 + 4.0 * (p - m) / (p - o);
            let (x, y) = (rng.gamma(a), rng.gamma(b));
            o + (p - o) * x / (x + y)
        }
    }
}

/// The three-point estimate each task is simulated with, and where it came from.
fn resolve<S: std::hash::BuildHasher>(
    nodes: &[Node],
    estimates: &HashMap<Uuid, Estimate, S>,
    band_percent: i64,
) -> Vec<(i64, i64, i64, Source, bool)> {
    nodes
        .iter()
        .map(|n| {
            let m = n.duration.unwrap_or(0).max(0);
            if m == 0 {
                return (0, 0, 0, Source::Milestone, false);
            }
            if let Some(e) = estimates.get(&n.pid) {
                let (o, p) = (e.optimistic, e.pessimistic);
                let widened = o > m || p < m;
                (o.min(m), m, p.max(m), Source::Estimated, widened)
            } else {
                let band = (m * band_percent + 50) / 100;
                ((m - band).max(0), m, m + band, Source::Fallback, false)
            }
        })
        .collect()
}

/// Pearson correlation of two series, or `None` when either has no variance.
pub(crate) fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    let n = x.len() as f64;
    if n < 2.0 {
        return None;
    }
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for (a, b) in x.iter().zip(y) {
        sxy += (a - mx) * (b - my);
        sxx += (a - mx).powi(2);
        syy += (b - my).powi(2);
    }
    if sxx < 1e-12 || syy < 1e-12 {
        return None;
    }
    Some(((sxy / (sxx * syy).sqrt()).clamp(-1.0, 1.0) * 100.0).round() / 100.0)
}

/// Nearest-rank percentile of an ascending slice.
pub(crate) fn nearest_rank<T: Copy>(sorted: &[T], p: u32) -> T {
    let n = sorted.len();
    let rank = (n * usize::try_from(p).unwrap_or(100)).div_ceil(100).max(1);
    sorted[rank.min(n) - 1]
}

/// Run the simulation.
///
/// # Errors
///
/// [`RunError`] when the network has a cycle, nothing can be scheduled, or the
/// requested work is over the caps.
#[allow(clippy::too_many_lines)]
pub fn simulate<S: std::hash::BuildHasher>(
    nodes: &[Node],
    edges: &[Edge],
    calendar: &WorkCalendar,
    own: &[WorkCalendar],
    estimates: &HashMap<Uuid, Estimate, S>,
    events: &[Event],
    params: Params,
) -> Result<Outcome, RunError> {
    simulate_with(
        nodes,
        edges,
        calendar,
        own,
        estimates,
        events,
        params,
        |_, _, _, _, _| {},
    )
}

/// [`simulate`], calling `hook(iteration, finish, delay_days, struck, rng)` after each
/// iteration's schedule is drawn and analysed. `delay_days` is how many working
/// days later than the critical-path finish this iteration finished (negative if
/// earlier) and `struck` the risks that struck in it. The hook draws from the **same** random stream, so a second model
/// (cost) rides along deterministically with the first.
///
/// # Errors
///
/// As [`simulate`].
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn simulate_with<
    S: std::hash::BuildHasher,
    H: FnMut(usize, NaiveDate, i64, &[Uuid], &mut Rng),
>(
    nodes: &[Node],
    edges: &[Edge],
    calendar: &WorkCalendar,
    own: &[WorkCalendar],
    estimates: &HashMap<Uuid, Estimate, S>,
    events: &[Event],
    params: Params,
    mut hook: H,
) -> Result<Outcome, RunError> {
    if params.iterations == 0 || params.iterations > MAX_ITERATIONS {
        return Err(RunError::TooMuchWork(format!(
            "iterations must be 1 to {MAX_ITERATIONS}"
        )));
    }
    if params.iterations.saturating_mul(nodes.len().max(1)) > MAX_WORK {
        return Err(RunError::TooMuchWork(format!(
            "iterations x tasks may be at most {MAX_WORK}; lower the iterations"
        )));
    }
    let options = Options {
        near_critical_days: 0,
        ..Options::default()
    };
    let base = analyze_with(nodes, edges, calendar, own, options)
        .map_err(|CpmError::Cycle(on)| RunError::Cycle(on))?;
    let Some(cpm_finish) = base.project_finish else {
        return Err(RunError::NothingScheduled);
    };
    let spread = resolve(nodes, estimates, params.fallback_band_percent);
    let index_of: HashMap<Uuid, usize> =
        nodes.iter().enumerate().map(|(i, n)| (n.pid, i)).collect();
    // Events in a fixed order, only those that can fire on a task we know.
    let mut events: Vec<Event> = events
        .iter()
        .copied()
        .filter(|e| index_of.contains_key(&e.task) && e.probability_bp > 0)
        .collect();
    events.sort_by_key(|e| e.risk);

    let mut rng = Rng::new(params.seed);
    let n = nodes.len();
    let mut finishes: Vec<NaiveDate> = Vec::with_capacity(params.iterations);
    let mut finish_idx: Vec<f64> = Vec::with_capacity(params.iterations);
    let mut critical = vec![0usize; n];
    let mut durations: Vec<Vec<f64>> = vec![Vec::with_capacity(params.iterations); n];
    let mut trial: Vec<Node> = nodes.to_vec();
    for _ in 0..params.iterations {
        let mut drawn: Vec<i64> = Vec::with_capacity(n);
        for (i, node) in trial.iter_mut().enumerate() {
            let (o, m, p, source, _) = spread[i];
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            let d = if source == Source::Milestone || p <= o {
                m
            } else {
                draw(&mut rng, o as f64, m as f64, p as f64, params.distribution).round() as i64
            };
            drawn.push(d);
            node.duration = Some(d);
        }
        let mut struck: Vec<Uuid> = Vec::new();
        for e in &events {
            let hit = {
                #[allow(clippy::cast_precision_loss)]
                let p = e.probability_bp as f64 / 10_000.0;
                rng.unit() < p
            };
            if hit && let Some(&i) = index_of.get(&e.task) {
                drawn[i] += e.impact_days;
                trial[i].duration = Some(drawn[i]);
                struck.push(e.risk);
            }
        }
        let res = analyze_with(&trial, edges, calendar, own, options)
            .map_err(|CpmError::Cycle(on)| RunError::Cycle(on))?;
        let Some(f) = res.project_finish else {
            return Err(RunError::NothingScheduled);
        };
        finishes.push(f);
        let delay = calendar.index_of(f).unwrap_or(0) - calendar.index_of(cpm_finish).unwrap_or(0);
        hook(finishes.len() - 1, f, delay, &struck, &mut rng);
        finish_idx.push(f64::from(chrono::Datelike::num_days_from_ce(&f)));
        for r in &res.nodes {
            if r.critical
                && let Some(&i) = index_of.get(&r.pid)
            {
                critical[i] += 1;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        for (i, d) in drawn.iter().enumerate() {
            durations[i].push(*d as f64);
        }
    }

    let mut sorted = finishes.clone();
    sorted.sort_unstable();
    let percentiles = [10_u32, 50, 80, 90]
        .iter()
        .map(|p| Percentile {
            p: *p,
            finish: nearest_rank(&sorted, *p),
        })
        .collect();
    let on_time = sorted.iter().filter(|f| **f <= cpm_finish).count();
    let mut histogram: Vec<(NaiveDate, usize)> = Vec::new();
    for f in &sorted {
        match histogram.last_mut() {
            Some((d, c)) if d == f => *c += 1,
            _ => histogram.push((*f, 1)),
        }
    }
    histogram.truncate(MAX_HISTOGRAM);
    let tasks = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let (o, m, p, source, widened) = spread[i];
            TaskResult {
                pid: node.pid,
                optimistic: o,
                likely: m,
                pessimistic: p,
                source,
                widened,
                criticality_bp: i64::try_from(critical[i] * 10_000 / params.iterations)
                    .unwrap_or(0),
                sensitivity: correlation(&durations[i], &finish_idx),
            }
        })
        .collect();
    Ok(Outcome {
        iterations: params.iterations,
        cpm_finish: Some(cpm_finish),
        percentiles,
        cpm_finish_confidence_bp: i64::try_from(on_time * 10_000 / params.iterations).unwrap_or(0),
        earliest: sorted.first().copied(),
        latest: sorted.last().copied(),
        histogram,
        tasks,
    })
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
    fn fs(n: u128, a: u128, b: u128) -> Edge {
        Edge {
            pid: id(1000 + n),
            pred: id(a),
            succ: id(b),
            kind: Kind::Fs,
            lag: 0,
        }
    }
    /// A (5) -> C (3) and B (4) -> C: A is the longer chain.
    fn diamond() -> (Vec<Node>, Vec<Edge>) {
        (
            vec![node(1, 5, true), node(2, 4, true), node(3, 3, false)],
            vec![fs(1, 1, 3), fs(2, 2, 3)],
        )
    }
    fn run(
        nodes: &[Node],
        edges: &[Edge],
        est: &HashMap<Uuid, Estimate>,
        events: &[Event],
        params: Params,
    ) -> Outcome {
        simulate(
            nodes,
            edges,
            &WorkCalendar::mon_fri(),
            &[],
            est,
            events,
            params,
        )
        .unwrap()
    }
    fn estimates(v: &[(u128, i64, i64)]) -> HashMap<Uuid, Estimate> {
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

    #[test]
    fn same_seed_gives_the_same_result_and_another_seed_a_different_one() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 3, 9), (2, 2, 10), (3, 2, 6)]);
        let p = Params::default();
        let (a, b) = (
            run(&nodes, &edges, &est, &[], p),
            run(&nodes, &edges, &est, &[], p),
        );
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
        let c = run(&nodes, &edges, &est, &[], Params { seed: 99, ..p });
        assert_ne!(
            serde_json::to_string(&a.histogram).unwrap(),
            serde_json::to_string(&c.histogram).unwrap()
        );
    }

    #[test]
    fn with_no_uncertainty_every_percentile_is_the_cpm_finish() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 5, 5), (2, 4, 4), (3, 3, 3)]);
        for dist in [Distribution::Triangular, Distribution::Pert] {
            let o = run(
                &nodes,
                &edges,
                &est,
                &[],
                Params {
                    distribution: dist,
                    ..Params::default()
                },
            );
            assert_eq!(o.cpm_finish, o.percentiles.first().map(|p| p.finish));
            assert!(o.percentiles.iter().all(|p| Some(p.finish) == o.cpm_finish));
            assert_eq!(o.cpm_finish_confidence_bp, 10_000);
            assert_eq!(o.histogram.len(), 1);
        }
        // The fallback band at zero is the same thing.
        let o = run(
            &nodes,
            &edges,
            &HashMap::new(),
            &[],
            Params {
                fallback_band_percent: 0,
                ..Params::default()
            },
        );
        assert!(o.percentiles.iter().all(|p| Some(p.finish) == o.cpm_finish));
    }

    #[test]
    fn percentiles_are_ordered_and_bracket_the_range() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 3, 12), (2, 2, 12), (3, 1, 8)]);
        let o = run(&nodes, &edges, &est, &[], Params::default());
        let p: Vec<NaiveDate> = o.percentiles.iter().map(|x| x.finish).collect();
        assert!(
            p.windows(2).all(|w| w[0] <= w[1]),
            "P10 <= P50 <= P80 <= P90"
        );
        assert!(o.earliest.unwrap() <= p[0] && p[3] <= o.latest.unwrap());
        assert_eq!(
            o.histogram.iter().map(|(_, c)| c).sum::<usize>(),
            o.iterations
        );
    }

    #[test]
    fn criticality_follows_the_longer_chain_and_shifts_with_uncertainty() {
        let (nodes, edges) = diamond();
        // Certain: A is always critical, B never, C always.
        let est = estimates(&[(1, 5, 5), (2, 4, 4), (3, 3, 3)]);
        let o = run(&nodes, &edges, &est, &[], Params::default());
        let ci = |o: &Outcome, n: u128| {
            o.tasks
                .iter()
                .find(|t| t.pid == id(n))
                .unwrap()
                .criticality_bp
        };
        assert_eq!((ci(&o, 1), ci(&o, 2), ci(&o, 3)), (10_000, 0, 10_000));
        // B wildly uncertain: it is critical some of the time, A less than always.
        let est = estimates(&[(1, 5, 5), (2, 1, 14), (3, 3, 3)]);
        let o = run(&nodes, &edges, &est, &[], Params::default());
        assert!(ci(&o, 2) > 0 && ci(&o, 2) < 10_000);
        assert!(ci(&o, 1) < 10_000);
        assert_eq!(ci(&o, 3), 10_000, "the join is always on the path");
    }

    #[test]
    fn sensitivity_is_high_for_the_driver_and_absent_without_variance() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 2, 14), (2, 4, 4), (3, 3, 3)]);
        let o = run(&nodes, &edges, &est, &[], Params::default());
        let s = |n: u128| o.tasks.iter().find(|t| t.pid == id(n)).unwrap().sensitivity;
        assert!(s(1).unwrap() > 0.5, "A drives the finish");
        assert_eq!(s(2), None, "B never varies");
        assert_eq!(s(3), None);
    }

    #[test]
    fn a_risk_event_pushes_the_finish_and_probability_zero_does_nothing() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 5, 5), (2, 4, 4), (3, 3, 3)]);
        let ev = |bp: i64| {
            [Event {
                risk: id(500),
                task: id(1),
                probability_bp: bp,
                impact_days: 5,
            }]
        };
        let none = run(&nodes, &edges, &est, &ev(0), Params::default());
        assert!(
            none.percentiles
                .iter()
                .all(|p| Some(p.finish) == none.cpm_finish)
        );
        let always = run(&nodes, &edges, &est, &ev(10_000), Params::default());
        assert!(
            always
                .percentiles
                .iter()
                .all(|p| Some(p.finish) > always.cpm_finish)
        );
        assert_eq!(always.cpm_finish_confidence_bp, 0);
        let half = run(&nodes, &edges, &est, &ev(5_000), Params::default());
        let c = half.cpm_finish_confidence_bp;
        assert!((4_000..=6_000).contains(&c), "about half on time, got {c}");
    }

    #[test]
    fn a_task_with_no_estimate_uses_the_stated_fallback_and_is_marked() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 3, 9)]);
        let o = run(&nodes, &edges, &est, &[], Params::default());
        let t = |n: u128| o.tasks.iter().find(|t| t.pid == id(n)).unwrap();
        assert_eq!(t(1).source, Source::Estimated);
        assert_eq!(t(2).source, Source::Fallback);
        assert_eq!(
            (t(2).optimistic, t(2).likely, t(2).pessimistic),
            (3, 4, 5),
            "4 +/- 20% rounds to 1"
        );
        assert_eq!(t(3).source, Source::Fallback);
    }

    #[test]
    fn an_estimate_not_around_the_likely_value_is_widened_and_flagged() {
        let (nodes, edges) = diamond();
        let est = estimates(&[(1, 6, 9)]); // likely is 5, below optimistic 6
        let o = run(&nodes, &edges, &est, &[], Params::default());
        let a = o.tasks.iter().find(|t| t.pid == id(1)).unwrap();
        assert!(a.widened);
        assert_eq!((a.optimistic, a.pessimistic), (5, 9));
    }

    #[test]
    fn caps_and_cycles_are_refused() {
        let (nodes, edges) = diamond();
        let cal = WorkCalendar::mon_fri();
        let go = |it: usize| {
            simulate(
                &nodes,
                &edges,
                &cal,
                &[],
                &HashMap::new(),
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
        let cyc = [fs(1, 1, 3), fs(2, 3, 1)];
        assert!(matches!(
            simulate(
                &nodes,
                &cyc,
                &cal,
                &[],
                &HashMap::new(),
                &[],
                Params::default()
            ),
            Err(RunError::Cycle(_))
        ));
    }

    #[test]
    fn pert_stays_nearer_the_likely_value_than_triangular() {
        let nodes = vec![node(1, 10, true)];
        let est = estimates(&[(1, 2, 30)]);
        let spread = |d: Distribution| {
            let o = run(
                &nodes,
                &[],
                &est,
                &[],
                Params {
                    distribution: d,
                    iterations: 4000,
                    ..Params::default()
                },
            );
            let p = |q: u32| o.percentiles.iter().find(|x| x.p == q).unwrap().finish;
            (p(90) - p(10)).num_days()
        };
        assert!(spread(Distribution::Pert) < spread(Distribution::Triangular));
    }
}

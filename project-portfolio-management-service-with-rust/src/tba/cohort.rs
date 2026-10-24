//! Cohort statistics (spec §7).

use super::{BTreeMap, DAY_MS, Ratio, Serialize, StatusShare, TaskAnalysis, as_days, waste_for};

/// Nearest-rank percentile over a **sorted** sample (spec §7.1).
///
/// Nearest-rank always returns an observed value, so "which item is the
/// p85?" has an answer — with interpolation it does not.
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)] // rank arithmetic over a bounded sample
pub fn percentile(sorted: &[i64], p: f64) -> Option<i64> {
    if sorted.is_empty() || !(0.0..=1.0).contains(&p) || p.is_nan() {
        return None;
    }
    let n = sorted.len();
    let rank = (p * n as f64).ceil().max(1.0);
    let index = (rank as usize).min(n).saturating_sub(1);
    sorted.get(index).copied()
}

/// A long-tailed duration distribution, reported by percentile rather
/// than by mean (spec §7.1).
#[derive(Clone, Debug, Serialize)]
pub struct Distribution {
    /// Sample size.
    pub n: usize,
    /// Smallest observation, milliseconds.
    pub min_ms: i64,
    /// Median.
    pub p50_ms: i64,
    /// 75th percentile.
    pub p75_ms: i64,
    /// 85th percentile — the service-level-expectation convention.
    pub p85_ms: i64,
    /// 95th percentile.
    pub p95_ms: i64,
    /// Largest observation.
    pub max_ms: i64,
    /// Arithmetic mean — reported, but skew-sensitive and describing no
    /// actual item.
    pub mean_ms: i64,
    /// The median in calendar days, for display.
    pub p50_days: f64,
    /// The 85th percentile in calendar days, for display.
    pub p85_days: f64,
    /// The percentile method in use.
    pub method: &'static str,
}

/// Build a distribution from unsorted durations. `None` on an empty
/// sample.
#[must_use]
pub fn distribution(values: &[i64]) -> Option<Distribution> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let n = sorted.len();
    let sum = sorted.iter().fold(0i64, |a, v| a.saturating_add(*v));
    #[expect(clippy::cast_possible_wrap)]
    let mean = sum / (n as i64).max(1);
    let at = |p: f64| percentile(&sorted, p).unwrap_or(0);
    Some(Distribution {
        n,
        min_ms: sorted.first().copied().unwrap_or(0),
        p50_ms: at(0.50),
        p75_ms: at(0.75),
        p85_ms: at(0.85),
        p95_ms: at(0.95),
        max_ms: sorted.last().copied().unwrap_or(0),
        mean_ms: mean,
        p50_days: as_days(at(0.50)),
        p85_days: as_days(at(0.85)),
        method: "nearest_rank",
    })
}

/// The minimum finished-item sample below which a service level
/// expectation is refused rather than computed from noise (spec §7.3).
pub const MIN_SLE_SAMPLE: usize = 10;

/// A service level expectation: "p% of items finish within N calendar days",
/// derived from the team's own history (spec §7.3).
#[derive(Clone, Debug, Serialize)]
pub struct ServiceLevelExpectation {
    /// The percentile the expectation is stated at.
    pub percentile: f64,
    /// The cycle time at that percentile, milliseconds.
    pub within_ms: Option<i64>,
    /// The same in calendar days — the number a team actually quotes.
    pub within_days: Option<f64>,
    /// Finished items the expectation was computed from.
    pub sample: usize,
    /// Why the expectation is null, when it is.
    pub reason: Option<String>,
    /// A caller-supplied commitment to score against, in calendar days.
    pub target_days: Option<f64>,
    /// The share of items that met that commitment.
    pub target_achieved_ratio: Option<f64>,
    /// Whether the commitment was met at the stated percentile.
    pub target_met: Option<bool>,
}

/// Compute the service level expectation from finished items' cycle
/// times, optionally scoring an explicit commitment.
#[must_use]
#[expect(clippy::cast_precision_loss)] // display ratio over a bounded sample
pub fn service_level_expectation(
    cycle_times_ms: &[i64],
    percentile_p: f64,
    target_days: Option<f64>,
) -> ServiceLevelExpectation {
    let mut sorted = cycle_times_ms.to_vec();
    sorted.sort_unstable();
    let sample = sorted.len();
    let enough = sample >= MIN_SLE_SAMPLE;
    let within = enough.then(|| percentile(&sorted, percentile_p)).flatten();

    let (achieved, met) = match target_days {
        Some(days) if days > 0.0 && !sorted.is_empty() => {
            #[expect(clippy::cast_possible_truncation)] // bounded by the guard
            let threshold = (days * DAY_MS as f64) as i64;
            let within_target = sorted.iter().filter(|ms| **ms <= threshold).count();
            let ratio = within_target as f64 / sorted.len() as f64;
            (Some(ratio), Some(ratio >= percentile_p))
        }
        _ => (None, None),
    };

    ServiceLevelExpectation {
        percentile: percentile_p,
        within_ms: within,
        within_days: within.map(as_days),
        sample,
        reason: (!enough).then(|| {
            format!(
                "only {sample} finished items; an expectation needs at least \
                 {MIN_SLE_SAMPLE} or it is a number computed from noise"
            )
        }),
        target_days,
        target_achieved_ratio: achieved,
        target_met: met,
    }
}

/// The plan-level analysis (spec §7).
#[derive(Clone, Debug, Serialize)]
pub struct PlanAnalysis {
    /// Tasks analysed.
    pub tasks: usize,
    /// Tasks finished.
    pub finished: usize,
    /// Tasks started and not finished (κ / WIP).
    pub work_in_progress: usize,
    /// Tasks still in the backlog.
    pub not_started: usize,
    /// Cycle-time distribution over finished tasks.
    pub cycle_time: Option<Distribution>,
    /// Lead-time distribution over finished tasks — always reported
    /// beside the cycle time so the flattering number cannot travel
    /// alone (spec §12.3).
    pub lead_time: Option<Distribution>,
    /// `Σ work / Σ cycle` — the system's ratio, dominated by the
    /// longest-running items.
    pub aggregate_flow_efficiency: Ratio,
    /// The median of the per-item ratios — the typical item.
    pub median_flow_efficiency: Option<f64>,
    /// `concentrated` when the two diverge (the waste sits in a
    /// minority of items), `uniform` when they agree.
    pub waste_shape: &'static str,
    /// Rolled first pass yield: the share of finished items that never
    /// moved backwards. Always reported beside throughput, so shipping
    /// work back to yourself cannot read as going faster.
    pub rolled_first_pass_yield: Option<f64>,
    /// Backwards moves across the plan.
    pub rework_count: usize,
    /// Per-status time across the plan.
    pub by_status: Vec<StatusShare>,
    /// Share of transitions synthesized by the migration (§5.4).
    pub backfilled_ratio: Option<f64>,
}

/// Aggregate per-task analyses into a plan view.
#[must_use]
pub fn plan(analyses: &[TaskAnalysis]) -> PlanAnalysis {
    let finished: Vec<&TaskAnalysis> = analyses.iter().filter(|a| a.finished).collect();
    let cycle_times: Vec<i64> = finished.iter().filter_map(|a| a.cycle_time_ms).collect();
    let lead_times: Vec<i64> = finished.iter().map(|a| a.lead_time_ms).collect();

    let total_work = analyses
        .iter()
        .fold(0i64, |a, x| a.saturating_add(x.work_time_ms));
    let total_cycle = analyses
        .iter()
        .fold(0i64, |a, x| a.saturating_add(x.cycle_time_ms.unwrap_or(0)));

    let mut ratios: Vec<f64> = analyses
        .iter()
        .filter_map(|a| a.flow_efficiency.value)
        .collect();
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median_ratio = if ratios.is_empty() {
        None
    } else {
        ratios.get(ratios.len() / 2).copied()
    };

    let aggregate = Ratio::new(total_work, total_cycle);
    let waste_shape = match (aggregate.value, median_ratio) {
        (Some(agg), Some(med)) if (agg - med).abs() > 0.05 => "concentrated",
        (Some(_), Some(_)) => "uniform",
        _ => "insufficient_data",
    };

    #[expect(clippy::cast_precision_loss)] // display ratio over bounded counts
    let rfpy = (!finished.is_empty())
        .then(|| finished.iter().filter(|a| a.first_pass).count() as f64 / finished.len() as f64);

    // Roll the per-task status tables up.
    let mut totals: BTreeMap<String, (String, i64, usize)> = BTreeMap::new();
    for analysis in analyses {
        for status in &analysis.by_status {
            let entry =
                totals
                    .entry(status.status.clone())
                    .or_insert((status.category.clone(), 0, 0));
            entry.1 = entry.1.saturating_add(status.ms);
            entry.2 += status.entries;
        }
    }
    let mut by_status: Vec<StatusShare> = totals
        .into_iter()
        .map(|(status, (category, ms, entries))| StatusShare {
            category,
            waste: waste_for(&status).map(ToString::to_string),
            status,
            ms,
            days: as_days(ms),
            share: Ratio::new(ms, total_cycle).value,
            entries,
        })
        .collect();
    by_status.sort_by_key(|s| std::cmp::Reverse(s.ms));

    let total_transitions: usize = analyses.iter().map(|a| a.transitions).sum();
    let total_backfilled: usize = analyses.iter().map(|a| a.backfilled).sum();
    #[expect(clippy::cast_precision_loss)] // display ratio over bounded counts
    let backfilled_ratio =
        (total_transitions > 0).then(|| total_backfilled as f64 / total_transitions as f64);

    PlanAnalysis {
        tasks: analyses.len(),
        finished: finished.len(),
        work_in_progress: analyses
            .iter()
            .filter(|a| !a.finished && a.cycle_time_ms.is_some())
            .count(),
        not_started: analyses
            .iter()
            .filter(|a| !a.finished && a.cycle_time_ms.is_none())
            .count(),
        cycle_time: distribution(&cycle_times),
        lead_time: distribution(&lead_times),
        aggregate_flow_efficiency: aggregate,
        median_flow_efficiency: median_ratio,
        waste_shape,
        rolled_first_pass_yield: rfpy,
        rework_count: analyses.iter().map(|a| a.rework_count).sum(),
        by_status,
        backfilled_ratio,
    }
}

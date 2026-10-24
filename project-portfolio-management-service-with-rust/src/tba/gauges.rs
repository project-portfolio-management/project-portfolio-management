//! Flow gauges (spec §15 TBA-10).

use super::{PlanAnalysis, ServiceLevelExpectation};

/// Default cap on how many plans are exported as individual gauge
/// series. Per-plan labels are unbounded cardinality, and a metric that
/// takes the monitoring down is worse than no metric.
pub const DEFAULT_METRIC_MAX_PLANS: usize = 50;

/// Default minimum task count for a plan to be labelled at all.
pub const DEFAULT_METRIC_MIN_TASKS: usize = 5;

/// One plan's figures, as the exporter consumes them.
#[derive(Clone, Debug)]
pub struct PlanFlowSample {
    /// The plan's public id.
    pub plan_pid: String,
    /// Its rollup.
    pub analysis: PlanAnalysis,
    /// Its service level expectation.
    pub sle: ServiceLevelExpectation,
    /// How many board columns are over their configured WIP cap.
    pub columns_over_limit: usize,
}

/// One plan's flow figures, ready to be written to a labelled gauge.
#[derive(Clone, Debug, PartialEq)]
pub struct FlowMetricRow {
    /// The plan's public id — the gauge label.
    pub plan_pid: String,
    /// Tasks behind the figures.
    pub tasks: usize,
    /// Started and not finished (κ).
    pub work_in_progress: usize,
    /// Aggregate work-over-cycle ratio.
    pub flow_efficiency: Option<f64>,
    /// p85 cycle time in calendar days. `None` below the SLE's own minimum
    /// sample, so the gauge inherits the refusal to forecast from noise
    /// rather than re-deciding it.
    pub cycle_time_p85_days: Option<f64>,
    /// Share of finished items that never moved backwards.
    pub rolled_first_pass_yield: Option<f64>,
    /// Board columns over their configured cap.
    pub columns_over_limit: usize,
}

/// What an export pass decided to publish, and what it held back.
#[derive(Clone, Debug, PartialEq)]
pub struct FlowMetricSet {
    /// The plans exported as their own series, largest board first.
    pub rows: Vec<FlowMetricRow>,
    /// Plans withheld for having too few tasks to aggregate.
    pub suppressed_plans: usize,
    /// Plans dropped because the series cap was reached.
    pub dropped_plans: usize,
}

/// Choose which plans get their own gauge series.
///
/// Two bounds, and neither is silent — both counts ship alongside the
/// rows, because a cap nobody can see reads as "we measured everything".
///
/// 1. **Small boards are suppressed.** A flow efficiency over two tasks
///    describes two people's week, and §12.4 refuses per-person
///    measurement; a label that reaches it by arithmetic is the same
///    thing through a side door. `/metrics.prom` is on the public
///    allow-list, so it stays scrapeable with
///    `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` on.
/// 2. **The series count is capped**, largest board first.
///
/// The label is the **pid**, never the name: a rename would fork the
/// series and silently reset its history.
///
/// **Per-column occupancy is deliberately not exported.** It would be
/// the most useful detail and also five series per plan — the single
/// biggest cardinality contributor here. The over-cap *count* carries
/// the alertable fact ("a column on this plan is over its limit") in
/// one series, and the detail is one API call away.
#[must_use]
pub fn flow_metric_rows(
    samples: &[PlanFlowSample],
    max_plans: usize,
    min_tasks: usize,
) -> FlowMetricSet {
    let mut eligible: Vec<&PlanFlowSample> = Vec::new();
    let mut suppressed = 0usize;
    for sample in samples {
        if sample.analysis.tasks == 0 {
            continue;
        }
        if sample.analysis.tasks < min_tasks {
            suppressed += 1;
        } else {
            eligible.push(sample);
        }
    }
    // Largest board first; the pid breaks ties so a pass is stable and a
    // dashboard does not reshuffle between scrapes.
    eligible.sort_by_key(|sample| {
        (
            std::cmp::Reverse(sample.analysis.tasks),
            sample.plan_pid.clone(),
        )
    });
    let dropped = eligible.len().saturating_sub(max_plans);
    eligible.truncate(max_plans);

    FlowMetricSet {
        rows: eligible
            .into_iter()
            .map(|sample| FlowMetricRow {
                plan_pid: sample.plan_pid.clone(),
                tasks: sample.analysis.tasks,
                work_in_progress: sample.analysis.work_in_progress,
                flow_efficiency: sample.analysis.aggregate_flow_efficiency.value,
                cycle_time_p85_days: sample.sle.within_days,
                rolled_first_pass_yield: sample.analysis.rolled_first_pass_yield,
                columns_over_limit: sample.columns_over_limit,
            })
            .collect(),
        suppressed_plans: suppressed,
        dropped_plans: dropped,
    }
}

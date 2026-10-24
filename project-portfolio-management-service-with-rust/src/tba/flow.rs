//! Flow analysis (spec §9).

use super::{Serialize, as_days};

/// Queueing-theory flow over a window (spec §9).
#[derive(Clone, Debug, Serialize)]
pub struct Flow {
    /// The window measured, calendar days.
    pub window_days: i64,
    /// Tasks created in the window.
    pub arrivals: usize,
    /// Tasks finished in the window.
    pub completions: usize,
    /// λ — arrivals per day.
    pub arrival_rate_per_day: Option<f64>,
    /// μ — completions per day (throughput).
    pub throughput_per_day: Option<f64>,
    /// ρ = λ/μ. Near 1, expected wait grows without bound — a team at
    /// 95% utilization is not 5% from trouble, it is already in it.
    pub utilization: Option<f64>,
    /// Why ρ is null, when it is.
    pub utilization_reason: Option<String>,
    /// κ — items started and not finished.
    pub work_in_progress: usize,
    /// τ̂ = κ/μ — the cycle time Little's Law implies for an item
    /// starting now.
    pub implied_cycle_time_days: Option<f64>,
    /// The observed median cycle time of items finished in the window.
    pub observed_p50_cycle_time_days: Option<f64>,
    /// `wip_growing` | `steady_state` | `queue_draining` |
    /// `insufficient_data` (spec §9.1).
    pub interpretation: &'static str,
    /// What the interpretation means, in words.
    pub detail: String,
}

/// Compute flow analysis. `observed_p50_ms` is the median cycle time of
/// items finished in the window, if any finished.
#[must_use]
#[expect(clippy::cast_precision_loss)] // rates over bounded counts
pub fn flow(
    window_days: i64,
    arrivals: usize,
    completions: usize,
    work_in_progress: usize,
    observed_p50_ms: Option<i64>,
) -> Flow {
    let days = window_days.max(1) as f64;
    let lambda = (window_days > 0).then(|| arrivals as f64 / days);
    let mu = (window_days > 0).then(|| completions as f64 / days);

    let (rho, rho_reason) = match (lambda, mu) {
        (Some(_), Some(m)) if m <= 0.0 => (
            None,
            Some(
                "nothing finished in the window, so throughput is zero and \
                 utilization is undefined — not infinite, and not zero"
                    .to_string(),
            ),
        ),
        (Some(l), Some(m)) => (Some(l / m), None),
        _ => (
            None,
            Some("window must be at least one calendar day".to_string()),
        ),
    };

    // Little's Law rearranged: cycle time = WIP / throughput.
    let implied = match mu {
        Some(m) if m > 0.0 => Some(work_in_progress as f64 / m),
        _ => None,
    };
    let observed_days = observed_p50_ms.map(as_days);

    let (interpretation, detail) = match (implied, observed_days) {
        (Some(implied_days), Some(obs)) if obs > 0.0 => {
            let ratio = implied_days / obs;
            if ratio > 1.25 {
                (
                    "wip_growing",
                    format!(
                        "Little's Law implies {implied_days:.1} calendar days for \
                         an item starting now, against {obs:.1} calendar days \
                         observed for those just finished. Work in progress is \
                         growing faster than it clears, so recent completions \
                         flatter the board. Lowering the WIP limit shortens cycle \
                         time without anyone working faster."
                    ),
                )
            } else if ratio < 0.80 {
                (
                    "queue_draining",
                    format!(
                        "Little's Law implies {implied_days:.1} calendar days against \
                         {obs:.1} observed. Either work in progress is draining, \
                         or this window's completions were disproportionately \
                         old items being cleared."
                    ),
                )
            } else {
                (
                    "steady_state",
                    format!(
                        "Little's Law implies {implied_days:.1} calendar days against \
                         {obs:.1} observed. Arrivals and departures are close to \
                         balanced, so the observed cycle time is predictive."
                    ),
                )
            }
        }
        _ => (
            "insufficient_data",
            "not enough arrivals or completions in the window to compare the \
             implied cycle time against an observed one"
                .to_string(),
        ),
    };

    Flow {
        window_days,
        arrivals,
        completions,
        arrival_rate_per_day: lambda,
        throughput_per_day: mu,
        utilization: rho,
        utilization_reason: rho_reason,
        work_in_progress,
        implied_cycle_time_days: implied,
        observed_p50_cycle_time_days: observed_days,
        interpretation,
        detail,
    }
}

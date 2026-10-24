//! Constraint findings (spec §8).

use super::{
    BACKLOG_STATUS, BLOCKED_STATUS, BTreeMap, CATEGORY_VALUE_ADDING, PlanAnalysis, Serialize,
    TaskAnalysis, UNRECORDED_REASON, as_days,
};

/// The share of non-value-adding time above which a status is reported
/// as dominating it rather than merely contributing.
pub const STATUS_DOMINANCE: f64 = 0.40;

/// Assignee changes at or above which an item is reported as
/// handoff-heavy.
pub const HANDOFF_HEAVY: usize = 3;

/// One disclosed constraint finding, ordered by time recoverable
/// (spec §8). Every finding names its rule; there is no composite
/// score, and nothing here is per-person (§12.4).
#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    /// The rule that fired.
    pub rule: &'static str,
    /// What the rule is about.
    pub subject: String,
    /// Human explanation, including the threshold that fired.
    pub detail: String,
    /// Time this finding accounts for, milliseconds.
    pub recoverable_ms: i64,
    /// The same in calendar days.
    pub recoverable_days: f64,
}

/// Findings for the per-status ranking (spec §8).
fn status_findings(summary: &PlanAnalysis) -> Vec<Finding> {
    let non_va: i64 = summary
        .by_status
        .iter()
        .filter(|s| s.category != CATEGORY_VALUE_ADDING)
        .fold(0i64, |a, s| a.saturating_add(s.ms));
    summary
        .by_status
        .iter()
        .filter(|s| s.category != CATEGORY_VALUE_ADDING && s.ms > 0)
        .map(|s| {
            #[expect(clippy::cast_precision_loss)] // display ratio
            let share = if non_va > 0 {
                s.ms as f64 / non_va as f64
            } else {
                0.0
            };
            Finding {
                rule: if share >= STATUS_DOMINANCE {
                    "status_dominates_wait"
                } else {
                    "status_wait"
                },
                subject: s.status.clone(),
                detail: format!(
                    "{:.1}% of the plan's non-value-adding time sits in `{}` \
                     (reported as `status_dominates_wait` at {:.0}%)",
                    share * 100.0,
                    s.status,
                    STATUS_DOMINANCE * 100.0
                ),
                recoverable_ms: s.ms,
                recoverable_days: as_days(s.ms),
            }
        })
        .collect()
}

/// Rank the plan's constraints (spec §8).
#[must_use]
pub fn constraints(analyses: &[TaskAnalysis], summary: &PlanAnalysis) -> Vec<Finding> {
    let mut findings = status_findings(summary);

    let blocked = analyses
        .iter()
        .fold(0i64, |a, x| a.saturating_add(x.blocked_time_ms));
    if blocked > 0 {
        let mut by_reason: BTreeMap<&str, i64> = BTreeMap::new();
        for analysis in analyses {
            for (reason, ms) in &analysis.blocked_by_reason {
                let slot = by_reason.entry(reason.as_str()).or_insert(0);
                *slot = slot.saturating_add(*ms);
            }
        }
        let reasons = by_reason
            .iter()
            .map(|(reason, ms)| format!("{reason} {:.1} days", as_days(*ms)))
            .collect::<Vec<_>>()
            .join(", ");
        findings.push(Finding {
            rule: "blocked_time",
            subject: BLOCKED_STATUS.to_string(),
            detail: format!(
                "time halted on something external. Separated from the \
                 other waste because it has the shortest path to a fix: \
                 something specific is in the way, and it has a name. By \
                 reason: {reasons} (`{UNRECORDED_REASON}` is time whose move \
                 into `{BLOCKED_STATUS}` recorded none)"
            ),
            recoverable_ms: blocked,
            recoverable_days: as_days(blocked),
        });
    }

    let dwell = analyses
        .iter()
        .fold(0i64, |a, x| a.saturating_add(x.queue_time_ms));
    if dwell > 0 {
        findings.push(Finding {
            rule: "backlog_dwell",
            subject: BACKLOG_STATUS.to_string(),
            detail: "time between creation and start. A large dwell means work \
                     is being started too early, which is a WIP decision \
                     rather than a capacity one"
                .to_string(),
            recoverable_ms: dwell,
            recoverable_days: as_days(dwell),
        });
    }

    if summary.rework_count > 0 {
        findings.push(Finding {
            rule: "rework",
            subject: format!("{} backwards moves", summary.rework_count),
            detail: format!(
                "rolled first pass yield {}. Throughput rising while this falls \
                 is not going faster — it is shipping work back to yourself",
                summary
                    .rolled_first_pass_yield
                    .map_or_else(|| "n/a".to_string(), |y| format!("{:.0}%", y * 100.0))
            ),
            recoverable_ms: 0,
            recoverable_days: 0.0,
        });
    }

    let heavy = analyses
        .iter()
        .filter(|a| a.handoffs >= HANDOFF_HEAVY)
        .count();
    if heavy > 0 {
        findings.push(Finding {
            rule: "handoff_heavy",
            subject: format!("{heavy} items"),
            detail: format!(
                "items whose assignee changed {HANDOFF_HEAVY} or more times. A \
                 property of the item's journey, never of the people — see \
                 spec §12.4"
            ),
            recoverable_ms: 0,
            recoverable_days: 0.0,
        });
    }

    if let Some(ratio) = summary.backfilled_ratio
        && ratio > 0.0
    {
        findings.push(Finding {
            rule: "backfilled_history",
            subject: format!("{:.0}% of transitions", ratio * 100.0),
            detail: "synthesized by the migration rather than observed, because \
                     the board kept no history before it. These figures firm up \
                     as real moves accumulate"
                .to_string(),
            recoverable_ms: 0,
            recoverable_days: 0.0,
        });
    }

    findings.sort_by_key(|f| std::cmp::Reverse(f.recoverable_ms));
    findings
}

/// One open item ranked against the service level expectation
/// (spec §8 `aging_wip`) — the only finding about work that can still
/// be helped.
#[derive(Clone, Debug, Serialize)]
pub struct AgingItem {
    /// How long since the item started, milliseconds.
    pub age_ms: i64,
    /// The same in calendar days.
    pub age_days: f64,
    /// Whether it is already past the expectation.
    pub past_sle: bool,
    /// Age as a fraction of the expectation.
    pub sle_ratio: Option<f64>,
}

/// Score one open item's age against the expectation.
#[must_use]
pub fn aging(age_ms: i64, sle_ms: Option<i64>) -> AgingItem {
    #[expect(clippy::cast_precision_loss)] // display ratio
    let ratio = sle_ms
        .filter(|ms| *ms > 0)
        .map(|ms| age_ms as f64 / ms as f64);
    AgingItem {
        age_ms,
        age_days: as_days(age_ms),
        past_sle: ratio.is_some_and(|r| r > 1.0),
        sle_ratio: ratio,
    }
}

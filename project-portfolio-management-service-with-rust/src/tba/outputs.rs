//! Outputs.

use super::{
    BACKLOG_STATUS, BLOCKED_STATUS, BTreeMap, CATEGORIES, CATEGORY_NECESSARY,
    CATEGORY_VALUE_ADDING, DAY_MS, FINISHED_STATUS, Serialize, TaskClock, Transition,
    blocked_by_reason, classify, is_backwards, is_started, waste_for,
};

/// A ratio reported with the two figures it came from.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Ratio {
    /// The ratio in `[0, 1]`, or `null` when undefined.
    pub value: Option<f64>,
    /// The numerator, milliseconds.
    pub numerator_ms: i64,
    /// The denominator, milliseconds.
    pub denominator_ms: i64,
}

impl Ratio {
    /// Build a ratio, yielding `value: None` on a non-positive
    /// denominator rather than a sentinel zero.
    #[must_use]
    pub fn new(numerator_ms: i64, denominator_ms: i64) -> Self {
        #[expect(clippy::cast_precision_loss)] // display ratio, not arithmetic
        let value = (denominator_ms > 0).then(|| numerator_ms as f64 / denominator_ms as f64);
        Self {
            value,
            numerator_ms,
            denominator_ms,
        }
    }
}

/// Time spent in one board status.
#[derive(Clone, Debug, Serialize)]
pub struct StatusShare {
    /// The status.
    pub status: String,
    /// The VSM category it was classified as, in the map in force.
    pub category: String,
    /// The VSM waste type, where the status represents one.
    pub waste: Option<String>,
    /// Milliseconds spent there.
    pub ms: i64,
    /// The same in calendar days, for display.
    pub days: f64,
    /// Share of lead time — the span the statuses partition.
    pub share: Option<f64>,
    /// How many times the item entered this status.
    pub entries: usize,
}

/// Time in one VSM category. These partition the lead time.
#[derive(Clone, Debug, Serialize)]
pub struct CategoryShare {
    /// One of [`CATEGORIES`].
    pub category: String,
    /// Milliseconds.
    pub ms: i64,
    /// The same in calendar days, for display.
    pub days: f64,
    /// Share of lead time.
    pub share: Option<f64>,
}

/// The per-task analysis (spec §6).
#[derive(Clone, Debug, Serialize)]
pub struct TaskAnalysis {
    /// Created → finished (or `as_of`). What the requester waits.
    pub lead_time_ms: i64,
    /// The same in calendar days.
    pub lead_time_days: f64,
    /// First started → finished (or `as_of`). What the team controls.
    /// `None` when the item never left the backlog.
    pub cycle_time_ms: Option<i64>,
    /// The same in calendar days.
    pub cycle_time_days: Option<f64>,
    /// Why the cycle time is null, when it is.
    pub cycle_time_reason: Option<String>,
    /// Time in value-adding statuses (VT / φ).
    pub work_time_ms: i64,
    /// Time in value-adding + necessary statuses (PT).
    pub process_time_ms: i64,
    /// Cycle time − process time (ω).
    pub wait_time_ms: i64,
    /// Time in `blocked` — the waste with the shortest path to a fix.
    pub blocked_time_ms: i64,
    /// `blocked_time_ms` by reason; time with no recorded reason is under
    /// `unrecorded`. Sums to `blocked_time_ms`.
    pub blocked_by_reason: BTreeMap<String, i64>,
    /// Time in `todo` — the backlog dwell, reported separately so it
    /// cannot vanish behind the cycle-time denominator (§6.1).
    pub queue_time_ms: i64,
    /// Work time over cycle time (%A). The headline 5–15% ratio.
    pub flow_efficiency: Ratio,
    /// Per-status time; partitions the lead time.
    pub by_status: Vec<StatusShare>,
    /// Per-category time; partitions the lead time.
    pub by_category: Vec<CategoryShare>,
    /// Transitions considered.
    pub transitions: usize,
    /// How many of them were synthesized by the migration (§5.4).
    pub backfilled: usize,
    /// Backwards moves on the board.
    pub rework_count: usize,
    /// Whether the item never moved backwards.
    pub first_pass: bool,
    /// Distinct assignees over the item's life.
    pub distinct_assignees: usize,
    /// Assignee changes (#HO).
    pub handoffs: usize,
    /// Whether the item has finished.
    pub finished: bool,
    /// For an open item: how long since it started. The only figure
    /// here that is actionable today.
    pub age_ms: Option<i64>,
    /// The same in calendar days.
    pub age_days: Option<f64>,
}

/// Milliseconds as calendar days, rounded to three places for display.
#[must_use]
#[expect(clippy::cast_precision_loss)] // display only
pub fn as_days(ms: i64) -> f64 {
    (ms as f64 / DAY_MS as f64 * 1000.0).round() / 1000.0
}

/// One derived interval: the task sat in `status` from `from_ms` until
/// `to_ms`.
#[derive(Clone, Debug)]
pub struct Interval {
    /// The board status occupied.
    pub status: String,
    /// Start, epoch milliseconds.
    pub from_ms: i64,
    /// End, epoch milliseconds.
    pub to_ms: i64,
}

impl Interval {
    /// The duration, never negative.
    #[must_use]
    pub fn duration_ms(&self) -> i64 {
        self.to_ms.saturating_sub(self.from_ms).max(0)
    }
}

/// Derive one task's intervals, in time order, **partitioning the whole
/// span from creation to finish** (spec §5.2, §6.3).
///
/// Three edges are handled here rather than left to the caller, because
/// each of them would otherwise produce time that belongs to no status
/// — and time that belongs to no status is time a report can quietly
/// lose:
///
/// - **Before the first recorded transition.** A backfilled row (§5.4)
///   is stamped at `status_changed_at`, which is later than the task's
///   creation, so the span between them is unattributed. It is charged
///   to the transition's `from_status` where one is recorded, and to
///   `todo` otherwise — the pessimistic choice, matching [`classify`]'s
///   fallback, so unknown history can never flatter the figures.
/// - **`done` is terminal.** The clock stops there; a finished task
///   analysed a year later must not have accrued a year.
/// - **A clock skew**, where `as_of` precedes the last transition,
///   yields zero-length intervals rather than negative ones.
#[must_use]
pub fn intervals(transitions: &[Transition], clock: TaskClock, as_of_ms: i64) -> Vec<Interval> {
    let mut sorted: Vec<&Transition> = transitions.iter().collect();
    sorted.sort_by_key(|t| t.at_ms);
    let end = clock.done_ms.unwrap_or(as_of_ms).max(clock.created_ms);
    let mut out: Vec<Interval> = Vec::with_capacity(sorted.len() + 1);
    let mut cursor = clock.created_ms;

    for (index, transition) in sorted.iter().enumerate() {
        let start = transition.at_ms.clamp(clock.created_ms, end);
        if start > cursor {
            out.push(Interval {
                status: transition
                    .from_status
                    .clone()
                    .unwrap_or_else(|| BACKLOG_STATUS.to_string()),
                from_ms: cursor,
                to_ms: start,
            });
            cursor = start;
        }
        if transition.to_status == FINISHED_STATUS {
            cursor = end;
            break;
        }
        let next = sorted
            .get(index + 1)
            .map_or(end, |next| next.at_ms)
            .clamp(cursor, end);
        if next > cursor {
            out.push(Interval {
                status: transition.to_status.clone(),
                from_ms: cursor,
                to_ms: next,
            });
        }
        cursor = next;
    }

    // Anything left — an empty log, or a task whose last recorded move
    // was long ago — is unknown history, charged to `todo` for the same
    // reason as above.
    if cursor < end {
        out.push(Interval {
            status: BACKLOG_STATUS.to_string(),
            from_ms: cursor,
            to_ms: end,
        });
    }
    out
}

/// Total the intervals per board status, shared over the lead time.
fn roll_up_statuses(
    spans: &[Interval],
    classes: &BTreeMap<String, String>,
    lead: i64,
) -> Vec<StatusShare> {
    let mut per_status: BTreeMap<String, (i64, usize)> = BTreeMap::new();
    for span in spans {
        let entry = per_status.entry(span.status.clone()).or_insert((0, 0));
        entry.0 = entry.0.saturating_add(span.duration_ms());
        entry.1 += 1;
    }
    per_status
        .iter()
        .map(|(status, (ms, entries))| StatusShare {
            status: status.clone(),
            category: classify(classes, status).to_string(),
            waste: waste_for(status).map(ToString::to_string),
            ms: *ms,
            days: as_days(*ms),
            share: Ratio::new(*ms, lead).value,
            entries: *entries,
        })
        .collect()
}

/// Fold the per-status totals into the three VSM categories. Every
/// category is emitted even at zero, so a consumer never has to decide
/// whether a missing key means zero or means unknown.
fn roll_up_categories(by_status: &[StatusShare], lead: i64) -> Vec<CategoryShare> {
    CATEGORIES
        .iter()
        .map(|category| {
            let ms = by_status
                .iter()
                .filter(|s| s.category == *category)
                .fold(0i64, |acc, s| acc.saturating_add(s.ms));
            CategoryShare {
                category: (*category).to_string(),
                ms,
                days: as_days(ms),
                share: Ratio::new(ms, lead).value,
            }
        })
        .collect()
}

/// Analyse one task (spec §6).
#[must_use]
pub fn analyze(
    transitions: &[Transition],
    clock: TaskClock,
    classes: &BTreeMap<String, String>,
    as_of_ms: i64,
) -> TaskAnalysis {
    let mut sorted: Vec<&Transition> = transitions.iter().collect();
    sorted.sort_by_key(|t| t.at_ms);

    let end = clock.done_ms.unwrap_or(as_of_ms).max(clock.created_ms);
    let spans = intervals(transitions, clock, as_of_ms);

    // Lead time is what the requester waits: creation to finish. The
    // intervals partition it exactly, which is why it is the
    // denominator for every per-status share (§6.3).
    let lead = end.saturating_sub(clock.created_ms).max(0);

    // Cycle time is what the team controls: from the moment the item
    // first occupied a started status.
    //
    // Taken as the earliest of two candidates, because either alone is
    // wrong in a case that really happens:
    //
    // - the first **started interval**, which catches a pre-history
    //   span (§5.2) that was already started — otherwise work time
    //   could begin before the cycle it is measured against;
    // - the first **transition** into a started status, which catches
    //   an item that started and finished inside one millisecond. Such
    //   an item has no interval at all, and reporting its cycle time as
    //   "never started" rather than as zero would be plainly false.
    let started_interval = spans
        .iter()
        .find(|span| is_started(&span.status))
        .map(|span| span.from_ms);
    let started_transition = sorted
        .iter()
        // A transition **after** the window is filtered out rather than
        // clamped into it: clamping a future move back to `end` would
        // let it retroactively start the clock.
        .filter(|t| t.at_ms <= end)
        .find(|t| is_started(&t.to_status))
        .map(|t| t.at_ms.max(clock.created_ms));
    let first_started = match (started_interval, started_transition) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (only @ Some(_), None) | (None, only) => only,
    };
    let cycle = first_started.map(|start| end.saturating_sub(start).max(0));

    let by_status = roll_up_statuses(&spans, classes, lead);
    let by_category = roll_up_categories(&by_status, lead);
    let total_for = |category: &str| -> i64 {
        by_status
            .iter()
            .filter(|s| s.category == category)
            .fold(0i64, |acc, s| acc.saturating_add(s.ms))
    };
    let work = total_for(CATEGORY_VALUE_ADDING);
    let process = work.saturating_add(total_for(CATEGORY_NECESSARY));

    let status_ms = |status: &str| -> i64 {
        by_status
            .iter()
            .find(|s| s.status == status)
            .map_or(0, |s| s.ms)
    };

    // Rework: backwards moves, excluding anything through `blocked`.
    let rework_count = sorted
        .iter()
        .filter(|t| {
            t.from_status
                .as_deref()
                .is_some_and(|from| is_backwards(from, &t.to_status))
        })
        .count();

    // Handoffs: assignee changes across the item's life.
    let mut handoffs = 0usize;
    for pair in sorted.windows(2) {
        if pair[0].assignee_ref != pair[1].assignee_ref {
            handoffs += 1;
        }
    }
    let distinct_assignees = sorted
        .iter()
        .filter_map(|t| t.assignee_ref.as_ref())
        .collect::<std::collections::BTreeSet<_>>()
        .len();

    let finished = clock.done_ms.is_some();
    let age = (!finished)
        .then(|| first_started.map(|start| as_of_ms.saturating_sub(start).max(0)))
        .flatten();
    let denominator = cycle.unwrap_or(0);

    TaskAnalysis {
        lead_time_ms: lead,
        lead_time_days: as_days(lead),
        cycle_time_ms: cycle,
        cycle_time_days: cycle.map(as_days),
        cycle_time_reason: cycle.is_none().then(|| {
            format!("the task never left `{BACKLOG_STATUS}`, so it has no cycle time yet")
        }),
        work_time_ms: work,
        process_time_ms: process,
        wait_time_ms: denominator.saturating_sub(process).max(0),
        blocked_time_ms: status_ms(BLOCKED_STATUS),
        blocked_by_reason: blocked_by_reason(transitions, &spans),
        queue_time_ms: status_ms(BACKLOG_STATUS),
        flow_efficiency: Ratio::new(work, denominator),
        by_status,
        by_category,
        transitions: sorted.len(),
        backfilled: sorted.iter().filter(|t| t.backfilled).count(),
        rework_count,
        first_pass: rework_count == 0,
        distinct_assignees,
        handoffs,
        finished,
        age_ms: age,
        age_days: age.map(as_days),
    }
}

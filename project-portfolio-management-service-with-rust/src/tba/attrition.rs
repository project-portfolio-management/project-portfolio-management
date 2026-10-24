//! Rollup attrition record (spec §15 TBA-12, repo tasks.md PA-3).

use super::Serialize;

/// The closed, ordered set of labels a [`rollup_attrition_trail`]'s
/// linear chain carries, before the three-way status partition —
/// `rollup_attrition_trail` always produces exactly these, in this
/// order, so a test can enumerate this vocabulary rather than trusting
/// the builder not to have silently dropped one.
///
/// This is a **reinterpretation** of care-pathway's
/// `ATTRITION_STEP_LABELS` (`src/tba.rs`, T-14g), not a port of its
/// literal vocabulary: a rollup's cohort is reached by a containment
/// **tree walk** with a task-count cap per node, not a linear
/// status/window/suppression screen over one flat table of instances.
/// The shape that survives the reinterpretation is the same one — a
/// named, ordered, parent-pointing trail explaining a denominator — but
/// the labels name what this endpoint actually filters.
pub const ROLLUP_ATTRITION_STEP_LABELS: &[&str] = &["root_plan", "walked_plans", "tasks_scanned"];

/// The index of `"tasks_scanned"` in a base [`rollup_attrition_trail`] —
/// the parent every status-partition leaf
/// (`"finished"`/`"work_in_progress"`/`"not_started"`) forks from.
pub const ROLLUP_ATTRITION_PARTITION_PARENT: usize = 2;

/// One step of a rollup's attrition record. Same shape as
/// care-pathway's `AttritionStep` (`label`/`operation`/`instances`/
/// `parent`), copied rather than shared: the two crates have no common
/// dependency this would live in, and the shape is small enough that a
/// shared crate for one struct would be the wrong trade (per
/// `agents/share/architecture.md`'s drift-accepted front-end/FHIR
/// precedent).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AttritionStep {
    /// One of [`ROLLUP_ATTRITION_STEP_LABELS`], or `"finished"` /
    /// `"work_in_progress"` / `"not_started"` (the partition leaves).
    pub label: &'static str,
    /// A human-readable account of what this step actually did (or,
    /// for a step with nothing yet to exclude, why not) — the
    /// denominator explained in the response, not merely stated.
    pub operation: String,
    /// Instances (plans for the first two steps, tasks thereafter)
    /// remaining after this step.
    pub instances: usize,
    /// The index of the step this one narrows from; `None` for the
    /// root only.
    pub parent: Option<usize>,
}

/// Inputs to [`rollup_attrition_trail`], bundled because the
/// alternative — nine positional `usize`/`bool` arguments, several
/// meaning similar things — is exactly the call site clippy's
/// `too_many_arguments` lint exists to catch, and a bundle also gives
/// each field a name at the call site instead of a position to
/// miscount.
///
/// `tasks_capped_plans` is the count of walked plans whose own task
/// load reached `MAX_TASKS` — an approximate signal (a plan with
/// exactly 1000 live tasks and no cap firing is indistinguishable from
/// one truncated at 1000), disclosed as such in
/// [`rollup_attrition_trail`]'s `operation` text rather than silently
/// treated as exact.
#[derive(Clone, Debug)]
pub struct RollupAttritionInputs {
    /// A human-readable name for the root plan.
    pub root_note: String,
    /// Plans reached by the containment walk (`RollupWalk::nodes.len()`).
    pub walked_plans: usize,
    /// Whether a depth or node cap stopped the walk early.
    pub walk_truncated: bool,
    /// Plans reached by more than one path (`RollupWalk::revisits`).
    pub walk_revisits: usize,
    /// Tasks unioned across every walked plan.
    pub tasks_scanned: usize,
    /// Walked plans whose own task load reached `MAX_TASKS`.
    pub tasks_capped_plans: usize,
    /// Tasks with a recorded finish.
    pub finished: usize,
    /// Tasks started and not finished.
    pub work_in_progress: usize,
    /// Tasks never started.
    pub not_started: usize,
}

/// Build a rollup's attrition record (spec §15 TBA-12): the containment
/// walk's own `truncated`/`revisits` bookkeeping, the per-node
/// `MAX_TASKS` cap, and the finished/work-in-progress/not-started
/// three-way partition that `plan()` already computes — named, ordered,
/// and parent-linked, rather than left as scattered fields the caller
/// has to reassemble into a story by hand.
#[must_use]
pub fn rollup_attrition_trail(inputs: &RollupAttritionInputs) -> Vec<AttritionStep> {
    vec![
        AttritionStep {
            label: "root_plan",
            operation: inputs.root_note.clone(),
            instances: 1,
            parent: None,
        },
        AttritionStep {
            label: "walked_plans",
            operation: if inputs.walk_truncated {
                format!(
                    "breadth-first containment walk from the root, {} \
                     revisit(s); a depth or node cap stopped the walk before it \
                     covered the whole subtree",
                    inputs.walk_revisits
                )
            } else {
                format!(
                    "breadth-first containment walk from the root, {} \
                     revisit(s); the walk covered the whole subtree",
                    inputs.walk_revisits
                )
            },
            instances: inputs.walked_plans,
            parent: Some(0),
        },
        AttritionStep {
            label: "tasks_scanned",
            operation: if inputs.tasks_capped_plans == 0 {
                "every walked plan's live tasks, none at the per-plan cap".to_string()
            } else {
                format!(
                    "every walked plan's live tasks; {} plan(s) reached \
                     the per-plan task cap and may hold more tasks than scanned",
                    inputs.tasks_capped_plans
                )
            },
            instances: inputs.tasks_scanned,
            parent: Some(1),
        },
        AttritionStep {
            label: "finished",
            operation: "tasks with a recorded finish".to_string(),
            instances: inputs.finished,
            parent: Some(ROLLUP_ATTRITION_PARTITION_PARENT),
        },
        AttritionStep {
            label: "work_in_progress",
            operation: "started, not yet finished".to_string(),
            instances: inputs.work_in_progress,
            parent: Some(ROLLUP_ATTRITION_PARTITION_PARENT),
        },
        AttritionStep {
            label: "not_started",
            operation: "never started".to_string(),
            instances: inputs.not_started,
            parent: Some(ROLLUP_ATTRITION_PARTITION_PARENT),
        },
    ]
}

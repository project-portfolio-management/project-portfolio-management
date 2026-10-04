//! Schedule health self-check (SC-9): fourteen checks in the style of the DCMA
//! 14-Point Assessment, as pure functions. DB-free, deterministic.
//!
//! This is a **self-check**, never a statement of DCMA compliance: the preset is
//! the commonly published one, checked against two secondary descriptions (not
//! the DCMA Program Analysis Pamphlet itself), and a scheduler should run the
//! real assessment where one is contractually required.
//!
//! Every check returns the same shape — what was counted (`count` of `of`), the
//! `value` that gives, the `threshold` it is held to, whether it `pass`es, and the
//! `offenders` by pid — so a scheduler can fix what failed. A check that cannot be
//! computed (no baseline, a dependency cycle, nothing incomplete to measure)
//! reports `pass: null` with a `reason`, and is **never counted as a pass**.
//!
//! Denominators: the checks on tasks use **incomplete tasks** (those not done),
//! the checks on logic use **relationships whose successor is incomplete**, and
//! the two baseline checks use **tasks baselined to finish by the status date**.
//! Each check says which in its `of_what`.
//!
//! Where the published descriptions disagree, the choice made is stated in the
//! check's `note`: *hard constraints* here are must-start-on, must-finish-on,
//! start-no-later-than and finish-no-later-than; the *critical path test* adds a
//! large delay (default 600 working days) to a critical task and expects the
//! finish to move by at least that; *resources* has no prescribed threshold in
//! the DCMA description, so zero unresourced tasks is this preset's own rule.

// Check bodies use short names (c for the check, t for thresholds, r for the run).
#![allow(clippy::many_single_char_names)]

use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::cpm::{
    ConstraintKind, CpmError, CpmResult, Edge, Kind, Node, Options, WorkCalendar, analyze_with,
};

/// The preset's name, echoed in every response.
pub const PRESET: &str = "dcma-default";

/// The thresholds one run is held to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Thresholds {
    /// Most incomplete tasks missing a predecessor or successor, percent.
    pub missing_logic_pct: i64,
    /// Most relationships with a lag, percent.
    pub lags_pct: i64,
    /// Fewest finish-to-start relationships, percent.
    pub finish_to_start_min_pct: i64,
    /// Most hard-constrained incomplete tasks, percent.
    pub hard_constraints_pct: i64,
    /// Total float above this many working days is "high".
    pub high_float_days: i64,
    /// Most incomplete tasks with high float, percent.
    pub high_float_pct: i64,
    /// A duration above this many working days is "high".
    pub high_duration_days: i64,
    /// Most incomplete tasks with high duration, percent.
    pub high_duration_pct: i64,
    /// Most baselined-due tasks missed, percent.
    pub missed_tasks_pct: i64,
    /// Lowest critical path length index, in thousandths (950 = 0.95).
    pub cpli_min_milli: i64,
    /// Lowest baseline execution index, in thousandths.
    pub bei_min_milli: i64,
    /// Working days added to a critical task by the critical path test.
    pub critical_path_test_days: i64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            missing_logic_pct: 5,
            lags_pct: 5,
            finish_to_start_min_pct: 90,
            hard_constraints_pct: 5,
            high_float_days: 44,
            high_float_pct: 5,
            high_duration_days: 44,
            high_duration_pct: 5,
            missed_tasks_pct: 5,
            cpli_min_milli: 950,
            bei_min_milli: 950,
            critical_path_test_days: 600,
        }
    }
}

/// Parse a deployment's thresholds (a JSON object with any of the field names),
/// falling back to the preset for anything absent. A malformed value, or one out
/// of range, falls back to the **whole** preset; the flag says so.
#[must_use]
pub fn parse_thresholds(raw: Option<&str>) -> (Thresholds, bool) {
    let d = Thresholds::default();
    let Some(raw) = raw.map(str::trim).filter(|r| !r.is_empty()) else {
        return (d, true);
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(raw) else {
        return (d, true);
    };
    let Some(obj) = v.as_object() else {
        return (d, true);
    };
    let get = |k: &str, def: i64, lo: i64, hi: i64| -> Option<i64> {
        match obj.get(k) {
            None => Some(def),
            Some(x) => x.as_i64().filter(|n| (lo..=hi).contains(n)),
        }
    };
    let t = (|| {
        Some(Thresholds {
            missing_logic_pct: get("missing_logic_pct", d.missing_logic_pct, 0, 100)?,
            lags_pct: get("lags_pct", d.lags_pct, 0, 100)?,
            finish_to_start_min_pct: get(
                "finish_to_start_min_pct",
                d.finish_to_start_min_pct,
                0,
                100,
            )?,
            hard_constraints_pct: get("hard_constraints_pct", d.hard_constraints_pct, 0, 100)?,
            high_float_days: get("high_float_days", d.high_float_days, 1, 3650)?,
            high_float_pct: get("high_float_pct", d.high_float_pct, 0, 100)?,
            high_duration_days: get("high_duration_days", d.high_duration_days, 1, 3650)?,
            high_duration_pct: get("high_duration_pct", d.high_duration_pct, 0, 100)?,
            missed_tasks_pct: get("missed_tasks_pct", d.missed_tasks_pct, 0, 100)?,
            cpli_min_milli: get("cpli_min_milli", d.cpli_min_milli, 0, 2000)?,
            bei_min_milli: get("bei_min_milli", d.bei_min_milli, 0, 2000)?,
            critical_path_test_days: get(
                "critical_path_test_days",
                d.critical_path_test_days,
                1,
                3650,
            )?,
        })
    })();
    t.map_or((d, true), |t| (t, false))
}

/// One task as the checks see it.
#[derive(Debug, Clone, Copy)]
pub struct Task {
    /// Identity (matches a [`Node`]).
    pub pid: Uuid,
    /// Done.
    pub complete: bool,
    /// Whether anyone is assigned.
    pub has_resource: bool,
    /// The day it was marked done, if it was.
    pub done_on: Option<NaiveDate>,
    /// Its baselined finish, if a baseline names one.
    pub baseline_finish: Option<NaiveDate>,
}

/// The inputs.
pub struct Input<'a> {
    /// The tasks, in input order.
    pub tasks: &'a [Task],
    /// The schedule nodes (same pids; a task with no duration is simply not
    /// scheduled by the critical-path engine).
    pub nodes: &'a [Node],
    /// Dependencies.
    pub edges: &'a [Edge],
    /// The plan calendar.
    pub calendar: &'a WorkCalendar,
    /// Per-assignee calendars.
    pub own: &'a [WorkCalendar],
    /// The status date.
    pub as_of: NaiveDate,
    /// A baseline exists for this plan.
    pub baseline_exists: bool,
}

/// What a check is held to.
#[derive(Debug, Clone, Serialize)]
pub struct Threshold {
    /// `<=`, `>=`, `==` or `pass`.
    pub op: &'static str,
    /// The limit (percent, thousandths or days — see `unit`); `None` for a
    /// pass/fail test.
    pub limit: Option<i64>,
    /// `percent`, `milli` (thousandths of a ratio), `days` or `result`.
    pub unit: &'static str,
}

/// One check's result.
#[derive(Debug, Clone, Serialize)]
pub struct Check {
    /// 1–14.
    pub id: u8,
    /// A stable key.
    pub key: &'static str,
    /// Its name.
    pub name: &'static str,
    /// What the count is a count of.
    pub of_what: &'static str,
    /// How many met the condition (offenders, or FS relationships for check 4).
    pub count: i64,
    /// Out of how many.
    pub of: i64,
    /// The figure: a percent to one place, or a ratio to two; `None` when it
    /// cannot be computed.
    pub value: Option<f64>,
    /// The limit.
    pub threshold: Threshold,
    /// Pass or fail; `None` when it cannot be computed (never a pass).
    pub pass: Option<bool>,
    /// Why it cannot be computed.
    pub reason: Option<&'static str>,
    /// The tasks to fix, by pid.
    pub offenders: Vec<Uuid>,
    /// A choice the published descriptions leave open.
    pub note: Option<&'static str>,
}

/// The whole report.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    /// The fourteen checks, in order.
    pub checks: Vec<Check>,
    /// How many passed.
    pub passed: usize,
    /// How many failed.
    pub failed: usize,
    /// How many could not be computed.
    pub not_computable: usize,
}

/// Tenths of a percent, as a number to one decimal place.
fn pct_value(count: i64, of: i64) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let tenths = (count * 1000 + of / 2) / of;
    #[allow(clippy::cast_precision_loss)]
    let v = tenths as f64 / 10.0;
    v
}

fn ratio_value(milli: i64) -> f64 {
    // Round to two places from thousandths.
    #[allow(clippy::cast_precision_loss)]
    let v = ((milli + 5).div_euclid(10)) as f64 / 100.0;
    v
}

#[allow(clippy::too_many_arguments)]
fn make(
    id: u8,
    key: &'static str,
    name: &'static str,
    of_what: &'static str,
    threshold: Threshold,
    note: Option<&'static str>,
) -> Check {
    Check {
        id,
        key,
        name,
        of_what,
        count: 0,
        of: 0,
        value: None,
        threshold,
        pass: None,
        reason: None,
        offenders: Vec::new(),
        note,
    }
}

fn unavailable(mut c: Check, reason: &'static str) -> Check {
    c.pass = None;
    c.value = None;
    c.reason = Some(reason);
    c
}

/// A share check: `count` of `of`, held to `<= max_pct` (or `>= min_pct`).
fn share(
    mut c: Check,
    count: i64,
    of: i64,
    offenders: Vec<Uuid>,
    at_most: bool,
    limit: i64,
    none_reason: &'static str,
) -> Check {
    if of == 0 {
        return unavailable(c, none_reason);
    }
    c.count = count;
    c.of = of;
    c.value = Some(pct_value(count, of));
    c.pass = Some(if at_most {
        count * 100 <= limit * of
    } else {
        count * 100 >= limit * of
    });
    c.offenders = offenders;
    c
}

fn pct_threshold(op: &'static str, limit: i64) -> Threshold {
    Threshold {
        op,
        limit: Some(limit),
        unit: "percent",
    }
}

/// Run the fourteen checks.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn assess(input: &Input, t: &Thresholds) -> Report {
    let options = Options {
        near_critical_days: 0,
        ..Options::default()
    };
    let cpm: Result<CpmResult, CpmError> =
        analyze_with(input.nodes, input.edges, input.calendar, input.own, options);
    let known: HashSet<Uuid> = input.tasks.iter().map(|x| x.pid).collect();
    let task_of: HashMap<Uuid, &Task> = input.tasks.iter().map(|x| (x.pid, x)).collect();
    let node_of: HashMap<Uuid, &Node> = input.nodes.iter().map(|n| (n.pid, n)).collect();
    let incomplete: Vec<&Task> = input.tasks.iter().filter(|x| !x.complete).collect();
    let is_incomplete = |pid: Uuid| task_of.get(&pid).is_some_and(|x| !x.complete);
    // Relationships between tasks we know, whose successor is incomplete.
    let rels: Vec<&Edge> = input
        .edges
        .iter()
        .filter(|e| known.contains(&e.pred) && known.contains(&e.succ) && is_incomplete(e.succ))
        .collect();
    let dedup = |v: Vec<Uuid>| {
        let mut seen = HashSet::new();
        v.into_iter()
            .filter(|p| seen.insert(*p))
            .collect::<Vec<_>>()
    };
    let n_incomplete = i64::try_from(incomplete.len()).unwrap_or(0);
    let n_rels = i64::try_from(rels.len()).unwrap_or(0);
    let mut checks = Vec::with_capacity(14);

    // 1. Missing logic.
    {
        let c = make(
            1,
            "missing_logic",
            "Missing logic",
            "incomplete tasks",
            pct_threshold("<=", t.missing_logic_pct),
            Some(
                "A task with no predecessor (or no successor) is exempt only where that is the project's own start (or finish) — the earliest start, or the latest finish, on the critical-path analysis.",
            ),
        );
        let (start, finish) = cpm.as_ref().ok().map_or((None, None), |r| {
            (
                r.nodes.iter().map(|n| n.early_start).min(),
                r.project_finish,
            )
        });
        let placed: HashMap<Uuid, (NaiveDate, NaiveDate)> = cpm
            .as_ref()
            .ok()
            .map(|r| {
                r.nodes
                    .iter()
                    .map(|n| (n.pid, (n.early_start, n.early_finish)))
                    .collect()
            })
            .unwrap_or_default();
        let has_pred: HashSet<Uuid> = input
            .edges
            .iter()
            .filter(|e| known.contains(&e.pred))
            .map(|e| e.succ)
            .collect();
        let has_succ: HashSet<Uuid> = input
            .edges
            .iter()
            .filter(|e| known.contains(&e.succ))
            .map(|e| e.pred)
            .collect();
        let off: Vec<Uuid> = incomplete
            .iter()
            .filter(|x| {
                let es_ef = placed.get(&x.pid);
                let no_pred =
                    !has_pred.contains(&x.pid) && es_ef.is_none_or(|(es, _)| Some(*es) != start);
                let no_succ =
                    !has_succ.contains(&x.pid) && es_ef.is_none_or(|(_, ef)| Some(*ef) != finish);
                no_pred || no_succ
            })
            .map(|x| x.pid)
            .collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            n_incomplete,
            off,
            true,
            t.missing_logic_pct,
            "no_incomplete_tasks",
        ));
    }

    // 2. Leads.
    {
        let c = make(
            2,
            "leads",
            "Leads (negative lag)",
            "relationships into incomplete tasks",
            pct_threshold("==", 0),
            None,
        );
        let off: Vec<Uuid> = rels.iter().filter(|e| e.lag < 0).map(|e| e.succ).collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            n_rels,
            dedup(off),
            true,
            0,
            "no_relationships",
        ));
    }

    // 3. Lags.
    {
        let c = make(
            3,
            "lags",
            "Lags (positive lag)",
            "relationships into incomplete tasks",
            pct_threshold("<=", t.lags_pct),
            None,
        );
        let off: Vec<Uuid> = rels.iter().filter(|e| e.lag > 0).map(|e| e.succ).collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            n_rels,
            dedup(off),
            true,
            t.lags_pct,
            "no_relationships",
        ));
    }

    // 4. Relationship types.
    {
        let c = make(
            4,
            "relationship_types",
            "Relationship types (finish-to-start share)",
            "relationships into incomplete tasks",
            pct_threshold(">=", t.finish_to_start_min_pct),
            None,
        );
        let fs = i64::try_from(rels.iter().filter(|e| e.kind == Kind::Fs).count()).unwrap_or(0);
        let off: Vec<Uuid> = rels
            .iter()
            .filter(|e| e.kind != Kind::Fs)
            .map(|e| e.succ)
            .collect();
        checks.push(share(
            c,
            fs,
            n_rels,
            dedup(off),
            false,
            t.finish_to_start_min_pct,
            "no_relationships",
        ));
    }

    // 5. Hard constraints.
    {
        let c = make(
            5,
            "hard_constraints",
            "Hard constraints",
            "incomplete tasks",
            pct_threshold("<=", t.hard_constraints_pct),
            Some(
                "Counts must-start-on, must-finish-on, start-no-later-than and finish-no-later-than.",
            ),
        );
        let off: Vec<Uuid> = incomplete
            .iter()
            .filter(|x| {
                node_of
                    .get(&x.pid)
                    .and_then(|n| n.constraint)
                    .is_some_and(|c| {
                        matches!(
                            c.kind,
                            ConstraintKind::Mso
                                | ConstraintKind::Mfo
                                | ConstraintKind::Snlt
                                | ConstraintKind::Fnlt
                        )
                    })
            })
            .map(|x| x.pid)
            .collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            n_incomplete,
            off,
            true,
            t.hard_constraints_pct,
            "no_incomplete_tasks",
        ));
    }

    // 6 & 7. High float, negative float (incomplete tasks the engine placed).
    let placed_incomplete: Vec<&crate::cpm::NodeResult> = cpm
        .as_ref()
        .ok()
        .map(|r| r.nodes.iter().filter(|n| is_incomplete(n.pid)).collect())
        .unwrap_or_default();
    let n_placed = i64::try_from(placed_incomplete.len()).unwrap_or(0);
    let cpm_reason = if cpm.is_err() {
        "cycle"
    } else {
        "no_scheduled_tasks"
    };
    {
        let c = make(
            6,
            "high_float",
            "High float",
            "incomplete scheduled tasks",
            pct_threshold("<=", t.high_float_pct),
            None,
        );
        let off: Vec<Uuid> = placed_incomplete
            .iter()
            .filter(|n| n.total_float > t.high_float_days)
            .map(|n| n.pid)
            .collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            n_placed,
            off,
            true,
            t.high_float_pct,
            cpm_reason,
        ));
    }
    {
        let c = make(
            7,
            "negative_float",
            "Negative float",
            "incomplete scheduled tasks",
            pct_threshold("==", 0),
            None,
        );
        let off: Vec<Uuid> = placed_incomplete
            .iter()
            .filter(|n| n.total_float < 0)
            .map(|n| n.pid)
            .collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            n_placed,
            off,
            true,
            0,
            cpm_reason,
        ));
    }

    // 8. High duration.
    {
        let c = make(
            8,
            "high_duration",
            "High duration",
            "incomplete tasks that have a duration",
            pct_threshold("<=", t.high_duration_pct),
            None,
        );
        let with: Vec<&&Task> = incomplete
            .iter()
            .filter(|x| node_of.get(&x.pid).is_some_and(|n| n.duration.is_some()))
            .collect();
        let off: Vec<Uuid> = with
            .iter()
            .filter(|x| {
                node_of
                    .get(&x.pid)
                    .and_then(|n| n.duration)
                    .is_some_and(|d| d > t.high_duration_days)
            })
            .map(|x| x.pid)
            .collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            i64::try_from(with.len()).unwrap_or(0),
            off,
            true,
            t.high_duration_pct,
            "no_durations",
        ));
    }

    // 9. Invalid dates.
    {
        let mut c = make(
            9,
            "invalid_dates",
            "Invalid dates",
            "tasks",
            pct_threshold("==", 0),
            Some(
                "A forecast finish before the status date on a task not done, or a completion dated after the status date.",
            ),
        );
        let mut off: Vec<Uuid> = input
            .tasks
            .iter()
            .filter(|x| x.complete && x.done_on.is_some_and(|d| d > input.as_of))
            .map(|x| x.pid)
            .collect();
        if let Ok(r) = &cpm {
            off.extend(
                r.nodes
                    .iter()
                    .filter(|n| is_incomplete(n.pid) && n.early_finish < input.as_of)
                    .map(|n| n.pid),
            );
            let of = i64::try_from(input.tasks.len()).unwrap_or(0);
            let off = dedup(off);
            checks.push(share(
                c,
                i64::try_from(off.len()).unwrap_or(0),
                of,
                off,
                true,
                0,
                "no_tasks",
            ));
        } else {
            c.offenders = dedup(off);
            checks.push(unavailable(c, "cycle"));
        }
    }

    // 10. Resources.
    {
        let c = make(
            10,
            "resources",
            "Resources",
            "incomplete tasks with a duration",
            pct_threshold("==", 0),
            Some(
                "The DCMA description sets no percentage for this check; zero unassigned tasks is this preset's own rule. A resource here is an assignee.",
            ),
        );
        let with: Vec<&&Task> = incomplete
            .iter()
            .filter(|x| {
                node_of
                    .get(&x.pid)
                    .and_then(|n| n.duration)
                    .is_some_and(|d| d > 0)
            })
            .collect();
        let off: Vec<Uuid> = with
            .iter()
            .filter(|x| !x.has_resource)
            .map(|x| x.pid)
            .collect();
        checks.push(share(
            c,
            i64::try_from(off.len()).unwrap_or(0),
            i64::try_from(with.len()).unwrap_or(0),
            off,
            true,
            0,
            "no_durations",
        ));
    }

    // 11. Missed tasks, and 14. BEI: both need a baseline and tasks due by now.
    let due: Vec<&Task> = input
        .tasks
        .iter()
        .filter(|x| x.baseline_finish.is_some_and(|f| f <= input.as_of))
        .collect();
    let n_due = i64::try_from(due.len()).unwrap_or(0);
    let baseline_reason = if input.baseline_exists {
        "nothing_baselined_due"
    } else {
        "no_baseline"
    };
    {
        let c = make(
            11,
            "missed_tasks",
            "Missed tasks",
            "tasks baselined to finish by the status date",
            pct_threshold("<=", t.missed_tasks_pct),
            Some(
                "A due task is missed when it is not done, or was done after its baselined finish.",
            ),
        );
        let off: Vec<Uuid> = due
            .iter()
            .filter(|x| {
                !x.complete
                    || x.done_on
                        .zip(x.baseline_finish)
                        .is_some_and(|(done, base)| done > base)
            })
            .map(|x| x.pid)
            .collect();
        let done_late_or_open = i64::try_from(off.len()).unwrap_or(0);
        checks.push(if input.baseline_exists {
            share(
                c,
                done_late_or_open,
                n_due,
                off,
                true,
                t.missed_tasks_pct,
                baseline_reason,
            )
        } else {
            unavailable(c, "no_baseline")
        });
    }

    // 12. Critical path test.
    {
        let mut c = make(
            12,
            "critical_path_test",
            "Critical path test",
            "one critical task",
            Threshold {
                op: "pass",
                limit: Some(t.critical_path_test_days),
                unit: "days",
            },
            Some(
                "Adds the threshold's working days to the critical task that finishes last and expects the project finish to move by at least that. The published descriptions give the injected delay as a large number of days; this preset uses 600.",
            ),
        );
        c = match &cpm {
            Err(_) => unavailable(c, "cycle"),
            Ok(r) => {
                let pick = r
                    .nodes
                    .iter()
                    .filter(|n| n.total_float <= 0 && is_incomplete(n.pid))
                    .max_by_key(|n| {
                        (
                            n.ef_index,
                            std::cmp::Reverse(
                                input
                                    .nodes
                                    .iter()
                                    .position(|x| x.pid == n.pid)
                                    .unwrap_or(usize::MAX),
                            ),
                        )
                    });
                match (pick, r.project_finish) {
                    (Some(p), Some(before)) => {
                        let mut nodes = input.nodes.to_vec();
                        if let Some(n) = nodes.iter_mut().find(|n| n.pid == p.pid) {
                            n.duration = Some(n.duration.unwrap_or(0) + t.critical_path_test_days);
                        }
                        match analyze_with(&nodes, input.edges, input.calendar, input.own, options)
                        {
                            Ok(after) => {
                                let moved = after.project_finish.map_or(0, |a| {
                                    input.calendar.index_of(a).unwrap_or(0)
                                        - input.calendar.index_of(before).unwrap_or(0)
                                });
                                c.count = moved;
                                c.of = t.critical_path_test_days;
                                c.value = {
                                    #[allow(clippy::cast_precision_loss)]
                                    let v = moved as f64;
                                    Some(v)
                                };
                                c.pass = Some(moved >= t.critical_path_test_days);
                                if c.pass == Some(false) {
                                    c.offenders = vec![p.pid];
                                }
                                c
                            }
                            Err(_) => unavailable(c, "cycle"),
                        }
                    }
                    _ => unavailable(c, "no_critical_path"),
                }
            }
        };
        checks.push(c);
    }

    // 13. CPLI.
    {
        let c = make(
            13,
            "cpli",
            "Critical path length index",
            "working days to the finish",
            Threshold {
                op: ">=",
                limit: Some(t.cpli_min_milli),
                unit: "milli",
            },
            Some(
                "(remaining critical path length + the least total float) / remaining critical path length, from the status date (or the first incomplete task's start, if later) to the project finish.",
            ),
        );
        let result = match &cpm {
            Err(_) => Err("cycle"),
            Ok(r) => {
                let first = placed_incomplete.iter().map(|n| n.early_start).min();
                match (first, r.project_finish) {
                    (Some(first), Some(finish)) if !placed_incomplete.is_empty() => {
                        let from = first.max(input.as_of);
                        let len = input.calendar.index_of(finish).unwrap_or(0)
                            - input.calendar.index_of(from).unwrap_or(0)
                            + 1;
                        let float = placed_incomplete
                            .iter()
                            .map(|n| n.total_float)
                            .min()
                            .unwrap_or(0);
                        if len <= 0 {
                            Err("no_remaining_path")
                        } else {
                            Ok((len, float))
                        }
                    }
                    _ => Err("no_remaining_path"),
                }
            }
        };
        checks.push(match result {
            Err(reason) => unavailable(c, reason),
            Ok((len, float)) => {
                let milli = (len + float) * 1000 / len;
                let mut c = c;
                c.count = len + float;
                c.of = len;
                c.value = Some(ratio_value(milli));
                c.pass = Some(milli >= t.cpli_min_milli);
                c.offenders = placed_incomplete
                    .iter()
                    .filter(|n| n.total_float < 0)
                    .map(|n| n.pid)
                    .collect();
                c
            }
        });
    }

    // 14. BEI.
    {
        let c = make(
            14,
            "bei",
            "Baseline execution index",
            "tasks baselined to finish by the status date",
            Threshold {
                op: ">=",
                limit: Some(t.bei_min_milli),
                unit: "milli",
            },
            Some("Tasks done, over tasks baselined to be done by the status date."),
        );
        checks.push(if !input.baseline_exists {
            unavailable(c, "no_baseline")
        } else if n_due == 0 {
            unavailable(c, baseline_reason)
        } else {
            let done = i64::try_from(due.iter().filter(|x| x.complete).count()).unwrap_or(0);
            let milli = done * 1000 / n_due;
            let mut c = c;
            c.count = done;
            c.of = n_due;
            c.value = Some(ratio_value(milli));
            c.pass = Some(milli >= t.bei_min_milli);
            c.offenders = due.iter().filter(|x| !x.complete).map(|x| x.pid).collect();
            c
        });
    }

    let passed = checks.iter().filter(|c| c.pass == Some(true)).count();
    let failed = checks.iter().filter(|c| c.pass == Some(false)).count();
    Report {
        not_computable: checks.len() - passed - failed,
        passed,
        failed,
        checks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    /// Monday 2026-10-05.
    fn monday() -> NaiveDate {
        d(2026, 10, 5)
    }
    fn task(n: u128) -> Task {
        Task {
            pid: id(n),
            complete: false,
            has_resource: true,
            done_on: None,
            baseline_finish: None,
        }
    }
    fn node(n: u128, dur: i64, start: bool) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            planned_start: start.then(monday),
            ..Node::default()
        }
    }
    fn edge(n: u128, a: u128, b: u128, kind: Kind, lag: i64) -> Edge {
        Edge {
            pid: id(1000 + n),
            pred: id(a),
            succ: id(b),
            kind,
            lag,
        }
    }
    struct Net {
        tasks: Vec<Task>,
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        as_of: NaiveDate,
        baseline: bool,
    }
    impl Net {
        fn run(&self) -> Report {
            self.run_with(&Thresholds::default())
        }
        fn run_with(&self, t: &Thresholds) -> Report {
            assess(
                &Input {
                    tasks: &self.tasks,
                    nodes: &self.nodes,
                    edges: &self.edges,
                    calendar: &WorkCalendar::mon_fri(),
                    own: &[],
                    as_of: self.as_of,
                    baseline_exists: self.baseline,
                },
                t,
            )
        }
    }
    /// 1 -> 2 -> 3 -> 4, all finish-to-start, no lag, 5 days each: a clean chain.
    fn chain() -> Net {
        Net {
            tasks: (1..=4).map(task).collect(),
            nodes: vec![
                node(1, 5, true),
                node(2, 5, false),
                node(3, 5, false),
                node(4, 5, false),
            ],
            edges: vec![
                edge(1, 1, 2, Kind::Fs, 0),
                edge(2, 2, 3, Kind::Fs, 0),
                edge(3, 3, 4, Kind::Fs, 0),
            ],
            as_of: monday(),
            baseline: false,
        }
    }
    fn get<'a>(r: &'a Report, key: &str) -> &'a Check {
        r.checks.iter().find(|c| c.key == key).unwrap()
    }

    #[test]
    fn a_clean_chain_passes_everything_it_can_and_says_what_it_cannot() {
        let r = chain().run();
        assert_eq!(r.checks.len(), 14);
        for key in [
            "missing_logic",
            "leads",
            "lags",
            "relationship_types",
            "hard_constraints",
            "high_float",
            "negative_float",
            "high_duration",
            "invalid_dates",
            "resources",
            "critical_path_test",
            "cpli",
        ] {
            assert_eq!(get(&r, key).pass, Some(true), "{key}");
        }
        for key in ["missed_tasks", "bei"] {
            let c = get(&r, key);
            assert_eq!((c.pass, c.reason), (None, Some("no_baseline")), "{key}");
        }
        assert_eq!((r.passed, r.failed, r.not_computable), (12, 0, 2));
    }

    #[test]
    fn missing_logic_exempts_only_the_projects_own_start_and_finish() {
        // 5 floats free in the middle of the schedule with no logic at all.
        let mut n = chain();
        n.tasks.push(task(5));
        n.nodes.push(node(5, 2, true));
        // Task 5 starts with the project (exempt from a predecessor) but finishes
        // early, so it has no successor and is not the finish: it is missing logic.
        let r = n.run();
        let c = get(&r, "missing_logic");
        assert_eq!((c.count, c.of), (1, 5));
        assert_eq!(c.offenders, vec![id(5)]);
        assert_eq!(c.pass, Some(false), "20% against a 5% limit");
        assert_eq!(c.value, Some(20.0));
    }

    #[test]
    fn leads_and_lags_count_relationships_into_incomplete_tasks() {
        let mut n = chain();
        n.edges[0].lag = -1;
        n.edges[1].lag = 2;
        let r = n.run();
        let leads = get(&r, "leads");
        assert_eq!((leads.count, leads.of, leads.pass), (1, 3, Some(false)));
        assert_eq!(leads.offenders, vec![id(2)]);
        let lags = get(&r, "lags");
        assert_eq!((lags.count, lags.of, lags.pass), (1, 3, Some(false)));
        assert_eq!(lags.offenders, vec![id(3)]);
        // A relationship into a completed task is not counted.
        n.tasks[1].complete = true;
        let r = n.run();
        assert_eq!(get(&r, "leads").of, 2);
    }

    #[test]
    fn relationship_types_hold_to_a_finish_to_start_share() {
        let mut n = chain();
        n.edges[0].kind = Kind::Ss;
        let r = n.run();
        let c = get(&r, "relationship_types");
        assert_eq!((c.count, c.of, c.pass), (2, 3, Some(false)));
        assert_eq!(c.offenders, vec![id(2)]);
        // 9 of 10 is exactly the 90% line and passes.
        let tasks: Vec<Task> = (1..=11).map(task).collect();
        let nodes: Vec<Node> = (1..=11).map(|i| node(i, 1, i == 1)).collect();
        let mut edges: Vec<Edge> = (1..=10).map(|i| edge(i, i, i + 1, Kind::Fs, 0)).collect();
        edges[4].kind = Kind::Ff;
        let r = Net {
            tasks,
            nodes,
            edges,
            as_of: monday(),
            baseline: false,
        }
        .run();
        let c = get(&r, "relationship_types");
        assert_eq!((c.count, c.of, c.pass), (9, 10, Some(true)));
    }

    #[test]
    fn hard_constraints_are_counted_and_listed() {
        let mut n = chain();
        n.nodes[2].constraint = Some(crate::cpm::Constraint {
            kind: ConstraintKind::Mso,
            date: d(2026, 10, 19),
        });
        n.nodes[1].constraint = Some(crate::cpm::Constraint {
            kind: ConstraintKind::Snet,
            date: monday(),
        });
        let r = n.run();
        let c = get(&r, "hard_constraints");
        assert_eq!((c.count, c.of, c.pass), (1, 4, Some(false)));
        assert_eq!(c.offenders, vec![id(3)], "snet is not hard");
    }

    #[test]
    fn high_float_and_negative_float_use_incomplete_scheduled_tasks() {
        // A long chain (60 days) and a short parallel task (5 days): 55 days of float.
        let n = Net {
            tasks: vec![task(1), task(2)],
            nodes: vec![node(1, 60, true), node(2, 5, true)],
            edges: vec![],
            as_of: monday(),
            baseline: false,
        };
        let r = n.run();
        let hf = get(&r, "high_float");
        assert_eq!((hf.count, hf.of, hf.pass), (1, 2, Some(false)));
        assert_eq!(hf.offenders, vec![id(2)]);
        assert_eq!(get(&r, "negative_float").pass, Some(true));
        // A deadline that cannot be met gives negative float.
        let mut n = chain();
        n.nodes[3].deadline = Some(d(2026, 10, 9));
        let r = n.run();
        let nf = get(&r, "negative_float");
        assert_eq!(nf.pass, Some(false));
        assert!(nf.offenders.contains(&id(4)));
    }

    #[test]
    fn high_duration_uses_incomplete_tasks_with_a_duration() {
        let mut n = chain();
        n.nodes[1].duration = Some(45);
        let r = n.run();
        let c = get(&r, "high_duration");
        assert_eq!((c.count, c.of, c.pass), (1, 4, Some(false)));
        assert_eq!(c.offenders, vec![id(2)]);
        n.nodes[1].duration = Some(44);
        assert_eq!(
            get(&n.run(), "high_duration").pass,
            Some(true),
            "44 is not above 44"
        );
        // A completed long task is not counted.
        n.nodes[1].duration = Some(90);
        n.tasks[1].complete = true;
        assert_eq!(get(&n.run(), "high_duration").of, 3);
    }

    #[test]
    fn invalid_dates_are_forecasts_in_the_past_and_actuals_in_the_future() {
        let mut n = chain();
        n.as_of = d(2026, 10, 12); // the first task should have finished on the 9th
        let r = n.run();
        let c = get(&r, "invalid_dates");
        assert_eq!(c.pass, Some(false));
        assert_eq!(c.offenders, vec![id(1)]);
        // A task marked done in the future is invalid too.
        let mut n = chain();
        n.tasks[0].complete = true;
        n.tasks[0].done_on = Some(d(2026, 10, 20));
        let r = n.run();
        assert!(get(&r, "invalid_dates").offenders.contains(&id(1)));
    }

    #[test]
    fn resources_counts_unassigned_incomplete_tasks_with_a_duration() {
        let mut n = chain();
        n.tasks[2].has_resource = false;
        let r = n.run();
        let c = get(&r, "resources");
        assert_eq!((c.count, c.of, c.pass), (1, 4, Some(false)));
        assert_eq!(c.offenders, vec![id(3)]);
        // A milestone needs no resource.
        let mut n = chain();
        n.nodes[3].duration = Some(0);
        n.tasks[3].has_resource = false;
        assert_eq!(get(&n.run(), "resources").pass, Some(true));
    }

    #[test]
    fn missed_tasks_and_bei_need_a_baseline_and_use_what_is_due() {
        let mut n = chain();
        n.baseline = true;
        n.as_of = d(2026, 10, 30);
        // Baselined: 1 on the 9th (done on time), 2 on the 16th (done late), 3 on the 23rd (open),
        // 4 on Nov 6 (not yet due).
        n.tasks[0].baseline_finish = Some(d(2026, 10, 9));
        n.tasks[0].complete = true;
        n.tasks[0].done_on = Some(d(2026, 10, 9));
        n.tasks[1].baseline_finish = Some(d(2026, 10, 16));
        n.tasks[1].complete = true;
        n.tasks[1].done_on = Some(d(2026, 10, 19));
        n.tasks[2].baseline_finish = Some(d(2026, 10, 23));
        n.tasks[3].baseline_finish = Some(d(2026, 11, 6));
        let r = n.run();
        let missed = get(&r, "missed_tasks");
        assert_eq!((missed.count, missed.of, missed.pass), (2, 3, Some(false)));
        assert_eq!(missed.offenders, vec![id(2), id(3)]);
        let bei = get(&r, "bei");
        assert_eq!(
            (bei.count, bei.of, bei.value, bei.pass),
            (2, 3, Some(0.67), Some(false))
        );
        assert_eq!(bei.offenders, vec![id(3)]);
        // Nothing due yet: not computable, not a pass.
        n.as_of = d(2026, 10, 1);
        let r = n.run();
        for key in ["missed_tasks", "bei"] {
            assert_eq!(get(&r, key).reason, Some("nothing_baselined_due"));
            assert_eq!(get(&r, key).pass, None);
        }
    }

    #[test]
    fn the_critical_path_test_fails_when_the_delay_does_not_reach_the_finish() {
        // A broken path: task 3 is missing a link to 4 (4 starts on its own date),
        // but the critical task that finishes last is 4, so the delay does reach it.
        let r = chain().run();
        let c = get(&r, "critical_path_test");
        assert_eq!((c.pass, c.of), (Some(true), 600));
        assert!(c.count >= 600);
        // Two disconnected chains of equal length: the delay hits one critical
        // task and the finish moves (it is the longest), so it still passes; the
        // test fails only when a hard-pinned task absorbs it.
        let mut n = chain();
        n.nodes[3].constraint = Some(crate::cpm::Constraint {
            kind: ConstraintKind::Mfo,
            date: d(2026, 11, 20),
        });
        let r = n.run();
        let c = get(&r, "critical_path_test");
        assert_eq!(
            c.pass,
            Some(false),
            "a must-finish-on pin swallows the delay"
        );
        assert_eq!(c.offenders, vec![id(4)]);
    }

    #[test]
    fn cpli_is_one_with_no_negative_float_and_falls_below_with_a_missed_deadline() {
        let r = chain().run();
        let c = get(&r, "cpli");
        assert_eq!(c.value, Some(1.0));
        assert_eq!((c.count, c.of), (20, 20));
        let mut n = chain();
        n.nodes[3].deadline = Some(d(2026, 10, 16)); // the chain finishes on the 30th: 10 working days too late
        let r = n.run();
        let c = get(&r, "cpli");
        assert_eq!((c.count, c.of), (10, 20), "(20 - 10) / 20");
        assert_eq!(c.value, Some(0.5));
        assert_eq!(c.pass, Some(false));
    }

    #[test]
    fn a_cycle_makes_the_schedule_checks_unavailable_not_passing() {
        let mut n = chain();
        n.edges.push(edge(9, 4, 1, Kind::Fs, 0));
        let r = n.run();
        for key in [
            "high_float",
            "negative_float",
            "invalid_dates",
            "critical_path_test",
            "cpli",
        ] {
            let c = get(&r, key);
            assert_eq!((c.pass, c.reason), (None, Some("cycle")), "{key}");
        }
        // The non-schedule checks still run.
        assert_eq!(get(&r, "leads").pass, Some(true));
    }

    #[test]
    fn nothing_incomplete_is_not_a_pass() {
        let mut n = chain();
        for t in &mut n.tasks {
            t.complete = true;
        }
        let r = n.run();
        for key in [
            "missing_logic",
            "hard_constraints",
            "high_float",
            "high_duration",
            "resources",
        ] {
            let c = get(&r, key);
            assert_eq!(c.pass, None, "{key}");
            assert!(c.reason.is_some(), "{key}");
        }
    }

    #[test]
    fn thresholds_are_configurable_and_a_bad_value_falls_back_whole() {
        let (t, defaulted) = parse_thresholds(Some(r#"{"high_float_days": 20, "lags_pct": 10}"#));
        assert!(!defaulted);
        assert_eq!(
            (t.high_float_days, t.lags_pct, t.high_duration_days),
            (20, 10, 44)
        );
        assert_eq!(parse_thresholds(Some("{not json")).0, Thresholds::default());
        assert!(
            parse_thresholds(Some(r#"{"lags_pct": 500}"#)).1,
            "out of range"
        );
        assert!(parse_thresholds(None).1);
        // A tighter preset changes a verdict.
        let n = Net {
            tasks: vec![task(1), task(2)],
            nodes: vec![node(1, 30, true), node(2, 5, true)],
            edges: vec![],
            as_of: monday(),
            baseline: false,
        };
        assert_eq!(get(&n.run(), "high_float").pass, Some(true));
        let tight = Thresholds {
            high_float_days: 20,
            ..Thresholds::default()
        };
        assert_eq!(get(&n.run_with(&tight), "high_float").pass, Some(false));
    }
}

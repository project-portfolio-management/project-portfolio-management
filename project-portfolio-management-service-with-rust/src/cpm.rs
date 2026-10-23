//! The critical-path-method engine (SC-1, PJ-8): pure, DB-free, deterministic.
//!
//! A **forward pass** gives each node its earliest start and finish, a
//! **backward pass** its latest, and from the two come **total float** (how long
//! the node can slip before the project finish moves) and **free float** (how
//! long before it delays any successor). All four relationship types (FS, SS,
//! FF, SF) are supported, each with a **lag or a lead** (negative lag).
//!
//! Time is counted in **business days of a [`WorkCalendar`]**: a node of duration
//! 3 starting on a Friday finishes on the following Tuesday under a Monday–Friday
//! calendar. A calendar with every weekday working is the pre-calendar
//! behaviour. Dates are zone-free calendar dates (see [`crate::tz`]); nothing
//! here reads a clock.
//!
//! **A node's planned start is a lower bound, not a command.** The earliest start
//! is the latest of the planned start and what its predecessors force; when logic
//! forces it *later* than planned, `pushed_days` says by how much (the schedule
//! as entered is impossible), instead of the engine silently agreeing with the
//! dates a person typed.
//!
//! **Honest about what it cannot place.** A node with no duration, or with no
//! planned start and no scheduled predecessor, is `unscheduled` with a reason —
//! never given a float of zero. An edge touching one is *ignored* and listed.
//! A cycle is an error, not a hang.
//!
//! Constraints (`mso`, `fnlt` …) are SC-2; they extend this engine and are not
//! here yet, so float here is never negative.

use std::collections::{BTreeSet, HashMap};

use chrono::{Datelike, Duration, NaiveDate};
use serde::Serialize;
use uuid::Uuid;

/// Monday 1970-01-05: the first Monday on or after the Unix epoch, the origin of
/// the business-day index.
const EPOCH: NaiveDate = match NaiveDate::from_ymd_opt(1970, 1, 5) {
    Some(d) => d,
    None => unreachable!(),
};

/// A working calendar: which weekdays work, and the dates that do not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkCalendar {
    /// Monday = index 0 … Sunday = 6.
    working: [bool; 7],
    holidays: BTreeSet<NaiveDate>,
}

/// An unusable calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CalendarError {
    /// No weekday works, so no schedule exists.
    NoWorkingDays,
}

impl WorkCalendar {
    /// Every day works (the behaviour before calendars existed).
    #[must_use]
    pub fn seven_day() -> Self {
        Self {
            working: [true; 7],
            holidays: BTreeSet::new(),
        }
    }

    /// Monday to Friday.
    #[must_use]
    pub fn mon_fri() -> Self {
        Self {
            working: [true, true, true, true, true, false, false],
            holidays: BTreeSet::new(),
        }
    }

    /// A calendar from its working weekdays (Monday first) and holidays.
    ///
    /// # Errors
    ///
    /// [`CalendarError::NoWorkingDays`] when no weekday works.
    pub fn new(
        working: [bool; 7],
        holidays: impl IntoIterator<Item = NaiveDate>,
    ) -> Result<Self, CalendarError> {
        if !working.iter().any(|w| *w) {
            return Err(CalendarError::NoWorkingDays);
        }
        Ok(Self {
            working,
            holidays: holidays.into_iter().collect(),
        })
    }

    /// The calendar on which **both** work: a day works only if it works in each
    /// (a part-time person on a five-day plan), and a holiday of either stands.
    ///
    /// # Errors
    ///
    /// [`CalendarError::NoWorkingDays`] when no weekday works in both.
    pub fn intersect(&self, other: &Self) -> Result<Self, CalendarError> {
        let mut working = [false; 7];
        for (i, w) in working.iter_mut().enumerate() {
            *w = self.working[i] && other.working[i];
        }
        Self::new(
            working,
            self.holidays.iter().chain(other.holidays.iter()).copied(),
        )
    }

    fn per_week(&self) -> i64 {
        i64::try_from(self.working.iter().filter(|w| **w).count()).unwrap_or(7)
    }

    /// Whether `date` is a business day.
    #[must_use]
    pub fn is_working(&self, date: NaiveDate) -> bool {
        self.working[date.weekday().num_days_from_monday() as usize]
            && !self.holidays.contains(&date)
    }

    /// The business-day index of `date`: how many business days lie strictly
    /// before it. For a working date this is its own index; a non-working date
    /// maps to the index of the **next** business day. `None` before 1970-01-05.
    #[must_use]
    pub fn index_of(&self, date: NaiveDate) -> Option<i64> {
        if date < EPOCH {
            return None;
        }
        let days = (date - EPOCH).num_days();
        let (full, rem) = (days / 7, days % 7);
        let partial = self
            .working
            .iter()
            .take(usize::try_from(rem).unwrap_or(0))
            .filter(|w| **w)
            .count();
        let base = full * self.per_week() + i64::try_from(partial).unwrap_or(0);
        let off = self
            .holidays
            .range(EPOCH..date)
            .filter(|h| self.working[h.weekday().num_days_from_monday() as usize])
            .count();
        Some(base - i64::try_from(off).unwrap_or(0))
    }

    /// The date of the business day with `index` (the inverse of
    /// [`index_of`](Self::index_of) on working dates).
    #[must_use]
    pub fn date_of(&self, index: i64) -> NaiveDate {
        let index = index.max(0);
        let span = (index / self.per_week() + 2) * 7
            + 7 * i64::try_from(self.holidays.len()).unwrap_or(0)
            + 7;
        let (mut lo, mut hi) = (0_i64, span);
        // Smallest day offset whose *next* day has index > `index`.
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let next = EPOCH + Duration::days(mid + 1);
            if self.index_of(next).is_some_and(|i| i > index) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        EPOCH + Duration::days(lo)
    }
}

/// A relationship type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Successor starts after the predecessor finishes (the default).
    Fs,
    /// Successor starts after the predecessor starts.
    Ss,
    /// Successor finishes after the predecessor finishes.
    Ff,
    /// Successor finishes after the predecessor starts.
    Sf,
}

impl Kind {
    /// Parse `fs` / `ss` / `ff` / `sf`, case-insensitively.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "fs" => Some(Self::Fs),
            "ss" => Some(Self::Ss),
            "ff" => Some(Self::Ff),
            "sf" => Some(Self::Sf),
            _ => None,
        }
    }
}

/// A date constraint on a node (SC-2).
///
/// `Snet` / `Fnet` push the **early** dates later; `Snlt` / `Fnlt` bound the
/// **late** dates, so a node already past them gets **negative float** instead of
/// being silently moved; `Mso` / `Mfo` pin the node to a date **overriding
/// logic** — those two are *hard*, and a node whose predecessors force it later
/// than the pin is flagged `hard_conflict`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConstraintKind {
    /// Start no earlier than.
    Snet,
    /// Start no later than.
    Snlt,
    /// Finish no earlier than.
    Fnet,
    /// Finish no later than.
    Fnlt,
    /// Must start on (hard).
    Mso,
    /// Must finish on (hard).
    Mfo,
}

impl ConstraintKind {
    /// Parse `snet` … `mfo`, case-insensitively. (`asap` is "no constraint";
    /// `alap` is not supported yet and is not parsed.)
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "snet" => Some(Self::Snet),
            "snlt" => Some(Self::Snlt),
            "fnet" => Some(Self::Fnet),
            "fnlt" => Some(Self::Fnlt),
            "mso" => Some(Self::Mso),
            "mfo" => Some(Self::Mfo),
            _ => None,
        }
    }

    /// Whether it overrides logic (needs the hard-constraint permission).
    #[must_use]
    pub fn is_hard(self) -> bool {
        matches!(self, Self::Mso | Self::Mfo)
    }
}

/// A constraint and its date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constraint {
    /// Which.
    pub kind: ConstraintKind,
    /// The date it applies to.
    pub date: NaiveDate,
}

/// One schedulable node (a plan with a timeframe, or a task).
#[derive(Debug, Clone, Copy, Default)]
pub struct Node {
    /// Identity.
    pub pid: Uuid,
    /// Business days; `0` is a milestone; `None` cannot be scheduled.
    pub duration: Option<i64>,
    /// Planned start: a lower bound on the earliest start.
    pub planned_start: Option<NaiveDate>,
    /// A date constraint, if any.
    pub constraint: Option<Constraint>,
    /// A soft deadline: bounds the **late** dates only (negative float when
    /// threatened), never moves an early date.
    pub deadline: Option<NaiveDate>,
    /// Index into the `own_calendars` slice of [`analyze_with`]: the calendar
    /// this node's *work* runs on (the plan's intersected with the assignee's).
    pub own_calendar: Option<usize>,
}

/// A dependency.
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    /// Identity.
    pub pid: Uuid,
    /// Predecessor.
    pub pred: Uuid,
    /// Successor.
    pub succ: Uuid,
    /// Relationship type.
    pub kind: Kind,
    /// Business days; negative is a lead.
    pub lag: i64,
}

/// Tuning.
#[derive(Debug, Clone, Copy)]
pub struct Options {
    /// A node with `0 < total_float <= this` is near-critical.
    pub near_critical_days: i64,
    /// Most critical chains returned.
    pub max_chains: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            near_critical_days: 5,
            max_chains: 50,
        }
    }
}

/// A scheduled node.
#[derive(Debug, Clone, Serialize)]
pub struct NodeResult {
    /// Identity.
    pub pid: Uuid,
    /// Earliest start date.
    pub early_start: NaiveDate,
    /// Earliest finish date (the last business day; equals the start for a milestone).
    pub early_finish: NaiveDate,
    /// Latest start date.
    pub late_start: NaiveDate,
    /// Latest finish date.
    pub late_finish: NaiveDate,
    /// Business days it can slip before the project finish moves.
    pub total_float: i64,
    /// Business days it can slip before any successor is delayed.
    pub free_float: i64,
    /// `total_float <= 0`.
    pub critical: bool,
    /// `0 < total_float <= near_critical_days`.
    pub near_critical: bool,
    /// The planned start as entered, if any.
    pub planned_start: Option<NaiveDate>,
    /// Business days logic pushes the start **later than planned** (0 if not).
    pub pushed_days: i64,
    /// Business-day index of the earliest start (for [`diff`]).
    pub es_index: i64,
    /// Business-day index one past the last business day of the earliest finish.
    pub ef_index: i64,
    /// The constraint applied, if any.
    pub constraint: Option<ConstraintKind>,
    /// A hard constraint pins this node earlier than its predecessors allow.
    pub hard_conflict: bool,
}

/// A node the engine could not place.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Unscheduled {
    /// Identity.
    pub pid: Uuid,
    /// `no_duration`, `no_anchor` or `date_out_of_range`.
    pub reason: &'static str,
}

/// The analysis.
#[derive(Debug, Clone, Serialize)]
pub struct CpmResult {
    /// Scheduled nodes, in input order.
    pub nodes: Vec<NodeResult>,
    /// Nodes left out, with why.
    pub unscheduled: Vec<Unscheduled>,
    /// Edges that touched an unscheduled or unknown node and were not used.
    pub ignored_edges: Vec<Uuid>,
    /// The latest early finish among scheduled nodes.
    pub project_finish: Option<NaiveDate>,
    /// Every critical chain, along driving edges.
    pub chains: Vec<Vec<Uuid>>,
    /// More chains existed than `max_chains`.
    pub chains_truncated: bool,
}

/// Why the analysis could not run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum CpmError {
    /// The dependencies contain a cycle; these nodes are on it (or behind it).
    Cycle(Vec<Uuid>),
}

fn topo(
    order: &[Uuid],
    edges: &[Edge],
    known: &HashMap<Uuid, usize>,
) -> Result<Vec<usize>, CpmError> {
    let n = order.len();
    let mut indeg = vec![0usize; n];
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in edges {
        if let (Some(&p), Some(&s)) = (known.get(&e.pred), known.get(&e.succ)) {
            out[p].push(s);
            indeg[s] += 1;
        }
    }
    let mut ready: Vec<usize> = (0..n).filter(|i| indeg[*i] == 0).collect();
    let mut sorted = Vec::with_capacity(n);
    while let Some(i) = ready.pop() {
        sorted.push(i);
        for &s in &out[i] {
            indeg[s] -= 1;
            if indeg[s] == 0 {
                ready.push(s);
            }
        }
        // Keep the order deterministic: smallest input index first.
        ready.sort_unstable_by(|a, b| b.cmp(a));
    }
    if sorted.len() == n {
        Ok(sorted)
    } else {
        let done: std::collections::HashSet<usize> = sorted.iter().copied().collect();
        Err(CpmError::Cycle(
            (0..n)
                .filter(|i| !done.contains(i))
                .map(|i| order[i])
                .collect(),
        ))
    }
}

/// Run the analysis with no per-node calendars.
///
/// # Errors
///
/// [`CpmError::Cycle`] when the edges contain a cycle.
pub fn analyze(
    nodes: &[Node],
    edges: &[Edge],
    calendar: &WorkCalendar,
    options: Options,
) -> Result<CpmResult, CpmError> {
    analyze_with(nodes, edges, calendar, &[], options)
}

/// Run the analysis. `calendar` is the network's (the plan's); a node whose
/// `own_calendar` indexes into `own_calendars` has its *work* placed on that
/// calendar (which must work only on days `calendar` works — build it with
/// [`WorkCalendar::intersect`]), while lag and float stay in `calendar`'s
/// business days.
///
/// # Errors
///
/// [`CpmError::Cycle`] when the edges contain a cycle.
#[expect(clippy::too_many_lines)]
pub fn analyze_with(
    nodes: &[Node],
    edges: &[Edge],
    calendar: &WorkCalendar,
    own_calendars: &[WorkCalendar],
    options: Options,
) -> Result<CpmResult, CpmError> {
    let order: Vec<Uuid> = nodes.iter().map(|n| n.pid).collect();
    let known: HashMap<Uuid, usize> = order.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    let sorted = topo(&order, edges, &known)?;

    // Incoming and outgoing edges per node (only between known nodes).
    let mut incoming: Vec<Vec<&Edge>> = vec![Vec::new(); nodes.len()];
    let mut outgoing: Vec<Vec<&Edge>> = vec![Vec::new(); nodes.len()];
    let mut ignored: Vec<Uuid> = Vec::new();
    for e in edges {
        match (known.get(&e.pred), known.get(&e.succ)) {
            (Some(&p), Some(&s)) => {
                outgoing[p].push(e);
                incoming[s].push(e);
            }
            _ => ignored.push(e.pid),
        }
    }

    let dur: Vec<Option<i64>> = nodes
        .iter()
        .map(|n| n.duration.filter(|d| *d >= 0))
        .collect();
    let planned_idx: Vec<Option<i64>> = nodes
        .iter()
        .map(|n| n.planned_start.and_then(|d| calendar.index_of(d)))
        .collect();

    // The business-day index of a constraint date, or `None` out of range.
    let w = |d: NaiveDate| calendar.index_of(d);
    // The calendar a node's work runs on, when it has its own.
    let own = |i: usize| -> Option<&WorkCalendar> {
        nodes[i].own_calendar.and_then(|k| own_calendars.get(k))
    };
    // Place `d` business days of node `i` starting no earlier than plan index
    // `start`; returns (start, finish) plan indices (finish exclusive).
    let place_forward = |i: usize, start: i64, d: i64| -> (i64, i64) {
        match own(i) {
            Some(e) if d > 0 => {
                let first = e.index_of(calendar.date_of(start)).unwrap_or(0);
                let s_date = e.date_of(first);
                let end_date = e.date_of(first + d - 1);
                (
                    calendar.index_of(s_date).unwrap_or(start),
                    calendar.index_of(end_date).map_or(start + d, |x| x + 1),
                )
            }
            _ => (start, start + d),
        }
    };
    // The latest placement of `d` business days finishing no later than plan
    // index `finish` (exclusive); returns (start, finish).
    let place_backward = |i: usize, finish: i64, d: i64| -> (i64, i64) {
        match own(i) {
            Some(e) if d > 0 && finish > 0 => {
                let x = calendar.date_of(finish - 1);
                let last = e.index_of(x + Duration::days(1)).unwrap_or(0) - 1;
                if last < 0 {
                    return (finish - d, finish);
                }
                let end_date = e.date_of(last);
                let s_date = e.date_of((last - (d - 1)).max(0));
                (
                    calendar.index_of(s_date).unwrap_or(finish - d),
                    calendar.index_of(end_date).map_or(finish, |v| v + 1),
                )
            }
            _ => (finish - d, finish),
        }
    };
    // Target finish index for a date (exclusive end), for a node of duration d.
    let finish_at =
        |date: NaiveDate, d: i64| -> Option<i64> { w(date).map(|i| if d > 0 { i + 1 } else { i }) };

    // Forward pass.
    let mut es: Vec<Option<i64>> = vec![None; nodes.len()];
    let mut ef: Vec<Option<i64>> = vec![None; nodes.len()];
    let mut conflict: Vec<bool> = vec![false; nodes.len()];
    let mut unscheduled: HashMap<usize, &'static str> = HashMap::new();
    for &i in &sorted {
        let Some(d) = dur[i] else {
            unscheduled.insert(i, "no_duration");
            continue;
        };
        if (nodes[i].planned_start.is_some() && planned_idx[i].is_none())
            || nodes[i].constraint.is_some_and(|c| w(c.date).is_none())
            || nodes[i].deadline.is_some_and(|x| w(x).is_none())
        {
            unscheduled.insert(i, "date_out_of_range");
            continue;
        }
        let mut lower: Option<i64> = planned_idx[i];
        for e in &incoming[i] {
            let p = known[&e.pred];
            let (Some(pes), Some(pef)) = (es[p], ef[p]) else {
                continue;
            };
            let bound = match e.kind {
                Kind::Fs => pef + e.lag,
                Kind::Ss => pes + e.lag,
                Kind::Ff => pef + e.lag - d,
                Kind::Sf => pes + e.lag - d,
            };
            lower = Some(lower.map_or(bound, |l| l.max(bound)));
        }
        let mut pinned: Option<i64> = None;
        if let Some(c) = nodes[i].constraint {
            let target_start = |date: NaiveDate| w(date);
            match c.kind {
                ConstraintKind::Snet => {
                    let b = target_start(c.date).unwrap_or(0);
                    lower = Some(lower.map_or(b, |l| l.max(b)));
                }
                ConstraintKind::Fnet => {
                    let b = finish_at(c.date, d).unwrap_or(0) - d;
                    lower = Some(lower.map_or(b, |l| l.max(b)));
                }
                ConstraintKind::Mso => pinned = target_start(c.date),
                ConstraintKind::Mfo => pinned = finish_at(c.date, d).map(|f| f - d),
                ConstraintKind::Snlt | ConstraintKind::Fnlt => {}
            }
        }
        let start = if let Some(p) = pinned {
            if lower.is_some_and(|l| l > p) {
                conflict[i] = true;
            }
            p
        } else {
            let Some(start) = lower else {
                unscheduled.insert(i, "no_anchor");
                continue;
            };
            start
        };
        let (s, f) = place_forward(i, start.max(0), d);
        es[i] = Some(s);
        ef[i] = Some(f);
    }
    let scheduled = |i: usize| es[i].is_some();
    for e in edges {
        if let (Some(&p), Some(&s)) = (known.get(&e.pred), known.get(&e.succ))
            && (!scheduled(p) || !scheduled(s))
        {
            ignored.push(e.pid);
        }
    }
    ignored.sort_unstable();
    ignored.dedup();

    let Some(project_finish) = (0..nodes.len()).filter_map(|i| ef[i]).max() else {
        return Ok(CpmResult {
            nodes: Vec::new(),
            unscheduled: nodes
                .iter()
                .enumerate()
                .filter_map(|(i, n)| {
                    unscheduled.get(&i).map(|r| Unscheduled {
                        pid: n.pid,
                        reason: r,
                    })
                })
                .collect(),
            ignored_edges: ignored,
            project_finish: None,
            chains: Vec::new(),
            chains_truncated: false,
        });
    };

    // Backward pass.
    let mut ls: Vec<Option<i64>> = vec![None; nodes.len()];
    let mut lf: Vec<Option<i64>> = vec![None; nodes.len()];
    for &i in sorted.iter().rev() {
        let (Some(d), true) = (dur[i], scheduled(i)) else {
            continue;
        };
        let mut upper = project_finish;
        for e in &outgoing[i] {
            let s = known[&e.succ];
            let (Some(sls), Some(slf)) = (ls[s], lf[s]) else {
                continue;
            };
            let bound = match e.kind {
                Kind::Fs => sls - e.lag,
                Kind::Ss => sls - e.lag + d,
                Kind::Ff => slf - e.lag,
                Kind::Sf => slf - e.lag + d,
            };
            upper = upper.min(bound);
        }
        // Constraints and the deadline bound the LATE dates only.
        if let Some(c) = nodes[i].constraint {
            let b = match c.kind {
                ConstraintKind::Snlt | ConstraintKind::Mso => w(c.date).map(|x| x + d),
                ConstraintKind::Fnlt | ConstraintKind::Mfo => finish_at(c.date, d),
                ConstraintKind::Snet | ConstraintKind::Fnet => None,
            };
            if let Some(b) = b {
                upper = upper.min(b);
            }
        }
        if let Some(x) = nodes[i].deadline.and_then(|x| finish_at(x, d)) {
            upper = upper.min(x);
        }
        let (start, finish) = place_backward(i, upper, d);
        lf[i] = Some(finish);
        ls[i] = Some(start);
    }

    // Free float per node, and each edge's slack.
    let edge_slack = |e: &Edge, p: usize, s: usize| -> Option<i64> {
        let (pes, pef, ses, sef) = (es[p]?, ef[p]?, es[s]?, ef[s]?);
        Some(match e.kind {
            Kind::Fs => ses - (pef + e.lag),
            Kind::Ss => ses - (pes + e.lag),
            Kind::Ff => sef - (pef + e.lag),
            Kind::Sf => sef - (pes + e.lag),
        })
    };
    let mut results: Vec<NodeResult> = Vec::new();
    let mut result_of: HashMap<usize, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let (Some(s0), Some(f0), Some(s1), Some(f1), Some(d)) =
            (es[i], ef[i], ls[i], lf[i], dur[i])
        else {
            continue;
        };
        let total_float = s1 - s0;
        // Free float: slack before any successor is delayed — and never more than
        // the slack before the project finish moves (a lead on a start-to-start
        // edge can otherwise make a successor look further away than the end).
        let mut free = project_finish - f0;
        for e in &outgoing[i] {
            let s = known[&e.succ];
            if let Some(sl) = edge_slack(e, i, s) {
                free = free.min(sl);
            }
        }
        let last = |idx: i64| {
            if d > 0 {
                calendar.date_of(idx - 1)
            } else {
                calendar.date_of(idx)
            }
        };
        let pushed = planned_idx[i].map_or(0, |p| (s0 - p).max(0));
        result_of.insert(i, results.len());
        results.push(NodeResult {
            pid: node.pid,
            early_start: calendar.date_of(s0),
            early_finish: last(f0),
            late_start: calendar.date_of(s1),
            late_finish: last(f1),
            total_float,
            free_float: free,
            critical: total_float <= 0,
            near_critical: total_float > 0 && total_float <= options.near_critical_days,
            planned_start: node.planned_start,
            pushed_days: pushed,
            es_index: s0,
            ef_index: f0,
            constraint: node.constraint.map(|c| c.kind),
            hard_conflict: conflict[i],
        });
    }

    // Critical chains along driving (zero-slack) edges between critical nodes.
    let critical = |i: usize| result_of.get(&i).is_some_and(|r| results[*r].critical);
    let driving = |e: &Edge| -> bool {
        let (p, s) = (known[&e.pred], known[&e.succ]);
        critical(p) && critical(s) && edge_slack(e, p, s) == Some(0)
    };
    let mut chains: Vec<Vec<Uuid>> = Vec::new();
    let mut truncated = false;
    let starts: Vec<usize> = (0..nodes.len())
        .filter(|i| critical(*i) && !incoming[*i].iter().any(|e| driving(e)))
        .collect();
    'outer: for start in starts {
        let mut stack: Vec<Vec<usize>> = vec![vec![start]];
        while let Some(path) = stack.pop() {
            let tip = *path.last().unwrap_or(&start);
            let nexts: Vec<usize> = outgoing[tip]
                .iter()
                .filter(|e| driving(e))
                .map(|e| known[&e.succ])
                .collect();
            if nexts.is_empty() {
                if chains.len() >= options.max_chains {
                    truncated = true;
                    break 'outer;
                }
                chains.push(path.iter().map(|i| order[*i]).collect());
            } else {
                for n in nexts.into_iter().rev() {
                    let mut p = path.clone();
                    p.push(n);
                    stack.push(p);
                }
            }
        }
    }

    Ok(CpmResult {
        nodes: results,
        unscheduled: nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                unscheduled.get(&i).map(|r| Unscheduled {
                    pid: n.pid,
                    reason: r,
                })
            })
            .collect(),
        ignored_edges: ignored,
        project_finish: Some(calendar.date_of(project_finish.max(1) - 1)),
        chains,
        chains_truncated: truncated,
    })
}

/// Most business days a duration or lag may be (about fourteen years).
pub const MAX_DAYS: i64 = 3650;

/// Everything wrong with a task's schedule fields (all of it, not the first).
#[must_use]
pub fn schedule_problems(start: Option<NaiveDate>, duration: Option<i32>) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(d) = duration
        && !(0..=i32::try_from(MAX_DAYS).unwrap_or(i32::MAX)).contains(&d)
    {
        out.push(format!(
            "duration_days must be 0 to {MAX_DAYS} (0 is a milestone)"
        ));
    }
    if let Some(s) = start
        && (s < EPOCH || s.year() > 2100)
    {
        out.push("start_date must be between 1970-01-05 and 2100-12-31".to_string());
    }
    out
}

/// Everything wrong with a constraint, as a pair `(kind, date)` and a deadline:
/// both or neither of kind and date, a known kind (`alap` is not supported
/// yet), and dates in range.
#[must_use]
pub fn constraint_problems(
    kind: Option<&str>,
    date: Option<NaiveDate>,
    deadline: Option<NaiveDate>,
) -> Vec<String> {
    let mut out = Vec::new();
    match (kind, date) {
        (None, None) => {}
        (Some(k), Some(d)) => {
            if ConstraintKind::parse(k).is_none() {
                out.push(format!(
                    "constraint_kind must be one of snet, snlt, fnet, fnlt, mso, mfo (got {k:?}; alap is not supported yet)"
                ));
            }
            if d < EPOCH || d.year() > 2100 {
                out.push("constraint_date must be between 1970-01-05 and 2100-12-31".to_string());
            }
        }
        _ => out.push("constraint_kind and constraint_date go together".to_string()),
    }
    if deadline.is_some_and(|d| d < EPOCH || d.year() > 2100) {
        out.push("deadline must be between 1970-01-05 and 2100-12-31".to_string());
    }
    out
}

/// How a node's dates moved between two analyses (PJ-8).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Moved {
    /// Identity.
    pub pid: Uuid,
    /// `start`, `finish` or `both`.
    pub moved: &'static str,
    /// Business days the start moved (positive = later).
    pub start_days: i64,
    /// Business days the finish moved (positive = later).
    pub finish_days: i64,
}

/// What moved between `before` and `after`: only nodes scheduled in both whose
/// start or finish changed. Empty when nothing did.
#[must_use]
pub fn diff(before: &CpmResult, after: &CpmResult) -> Vec<Moved> {
    let was: HashMap<Uuid, &NodeResult> = before.nodes.iter().map(|n| (n.pid, n)).collect();
    after
        .nodes
        .iter()
        .filter_map(|n| {
            let b = was.get(&n.pid)?;
            let (s, f) = (n.es_index - b.es_index, n.ef_index - b.ef_index);
            let moved = match (s != 0, f != 0) {
                (false, false) => return None,
                (true, false) => "start",
                (false, true) => "finish",
                (true, true) => "both",
            };
            Some(Moved {
                pid: n.pid,
                moved,
                start_days: s,
                finish_days: f,
            })
        })
        .collect()
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
    fn node(n: u128, dur: i64) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            ..Node::default()
        }
    }
    fn anchored(n: u128, dur: i64, start: NaiveDate) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            planned_start: Some(start),
            ..Node::default()
        }
    }
    fn edge(n: u128, p: u128, s: u128, kind: Kind, lag: i64) -> Edge {
        Edge {
            pid: id(1000 + n),
            pred: id(p),
            succ: id(s),
            kind,
            lag,
        }
    }
    fn run(nodes: &[Node], edges: &[Edge]) -> CpmResult {
        analyze(nodes, edges, &WorkCalendar::seven_day(), Options::default()).unwrap()
    }
    fn get(r: &CpmResult, n: u128) -> &NodeResult {
        r.nodes.iter().find(|x| x.pid == id(n)).unwrap()
    }
    const MON: fn() -> NaiveDate = || d(2026, 10, 5); // a Monday

    /// The textbook network: A3; B4 and C2 after A; D5 after B and C.
    fn textbook() -> (Vec<Node>, Vec<Edge>) {
        (
            vec![anchored(1, 3, MON()), node(2, 4), node(3, 2), node(4, 5)],
            vec![
                edge(1, 1, 2, Kind::Fs, 0),
                edge(2, 1, 3, Kind::Fs, 0),
                edge(3, 2, 4, Kind::Fs, 0),
                edge(4, 3, 4, Kind::Fs, 0),
            ],
        )
    }

    #[test]
    fn the_textbook_network_gives_the_published_floats() {
        let (n, e) = textbook();
        let r = run(&n, &e);
        let idx = |k: u128| (get(&r, k).es_index, get(&r, k).ef_index);
        let base = idx(1).0;
        assert_eq!((idx(1).0 - base, idx(1).1 - base), (0, 3));
        assert_eq!((idx(2).0 - base, idx(2).1 - base), (3, 7));
        assert_eq!((idx(3).0 - base, idx(3).1 - base), (3, 5));
        assert_eq!((idx(4).0 - base, idx(4).1 - base), (7, 12));
        let tf = |k: u128| get(&r, k).total_float;
        assert_eq!((tf(1), tf(2), tf(3), tf(4)), (0, 0, 2, 0));
        assert_eq!(
            get(&r, 3).free_float,
            2,
            "C can slip 2 business days before delaying D"
        );
        assert_eq!(get(&r, 2).free_float, 0);
        assert_eq!(r.chains, vec![vec![id(1), id(2), id(4)]]);
        assert!(r.unscheduled.is_empty() && r.ignored_edges.is_empty());
    }

    #[test]
    fn a_lag_lengthens_the_chain_and_can_change_which_one_is_critical() {
        let (n, mut e) = textbook();
        // 3 business days of lag on C→D: C finishes at 5, D can start at 8 > B's 7.
        e[3] = edge(4, 3, 4, Kind::Fs, 3);
        let r = run(&n, &e);
        assert_eq!(
            r.chains,
            vec![vec![id(1), id(3), id(4)]],
            "C is now critical, B is not"
        );
        assert_eq!(get(&r, 2).total_float, 1);
        assert!(get(&r, 3).critical && !get(&r, 2).critical);
    }

    #[test]
    fn a_lead_pulls_the_successor_in() {
        let n = vec![anchored(1, 4, MON()), node(2, 3)];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, -1)]);
        assert_eq!(get(&r, 2).es_index, get(&r, 1).ef_index - 1);
    }

    #[test]
    fn each_relationship_type_behaves_per_definition() {
        let base = |k: Kind, lag: i64| {
            let n = vec![anchored(1, 4, MON()), node(2, 3)];
            let r = run(&n, &[edge(1, 1, 2, k, lag)]);
            (
                get(&r, 2).es_index - get(&r, 1).es_index,
                get(&r, 2).ef_index - get(&r, 1).es_index,
            )
        };
        assert_eq!(
            base(Kind::Fs, 0),
            (4, 7),
            "starts when the predecessor finishes"
        );
        assert_eq!(
            base(Kind::Ss, 2),
            (2, 5),
            "starts 2 after the predecessor starts"
        );
        assert_eq!(
            base(Kind::Ff, 0),
            (1, 4),
            "finishes when the predecessor finishes"
        );
        assert_eq!(
            base(Kind::Sf, 6),
            (3, 6),
            "finishes 6 after the predecessor starts"
        );
    }

    #[test]
    fn two_equal_chains_are_both_critical() {
        let n = vec![anchored(1, 2, MON()), node(2, 3), node(3, 3), node(4, 1)];
        let e = [
            edge(1, 1, 2, Kind::Fs, 0),
            edge(2, 1, 3, Kind::Fs, 0),
            edge(3, 2, 4, Kind::Fs, 0),
            edge(4, 3, 4, Kind::Fs, 0),
        ];
        let r = run(&n, &e);
        assert!(get(&r, 2).critical && get(&r, 3).critical);
        assert_eq!(r.chains.len(), 2);
        assert!(
            r.chains.contains(&vec![id(1), id(2), id(4)])
                && r.chains.contains(&vec![id(1), id(3), id(4)])
        );
    }

    #[test]
    fn near_critical_is_flagged_within_the_threshold() {
        let (mut n, e) = textbook();
        n[2] = node(3, 2);
        let r = analyze(
            &n,
            &e,
            &WorkCalendar::seven_day(),
            Options {
                near_critical_days: 2,
                max_chains: 10,
            },
        )
        .unwrap();
        assert!(get(&r, 3).near_critical, "float 2 <= 2");
        let r = analyze(
            &n,
            &e,
            &WorkCalendar::seven_day(),
            Options {
                near_critical_days: 1,
                max_chains: 10,
            },
        )
        .unwrap();
        assert!(!get(&r, 3).near_critical);
    }

    #[test]
    fn a_cycle_is_an_error_not_a_hang() {
        let n = vec![anchored(1, 1, MON()), node(2, 1), node(3, 1)];
        let e = [
            edge(1, 1, 2, Kind::Fs, 0),
            edge(2, 2, 3, Kind::Fs, 0),
            edge(3, 3, 2, Kind::Fs, 0),
        ];
        let Err(CpmError::Cycle(on)) =
            analyze(&n, &e, &WorkCalendar::seven_day(), Options::default())
        else {
            panic!("a cycle must be an error")
        };
        assert!(on.contains(&id(2)) && on.contains(&id(3)));
    }

    #[test]
    fn what_cannot_be_placed_is_listed_never_given_a_zero_float() {
        let n = vec![
            anchored(1, 2, MON()),
            Node {
                pid: id(2),
                duration: None,
                planned_start: Some(MON()),
                ..Node::default()
            },
            node(3, 2), // no start and no predecessor
            node(4, 2), // reachable only from the unplaceable node 3
        ];
        let e = [edge(1, 3, 4, Kind::Fs, 0), edge(2, 1, 4, Kind::Fs, 0)];
        let r = run(&n, &e);
        let why = |p: u128| {
            r.unscheduled
                .iter()
                .find(|u| u.pid == id(p))
                .map(|u| u.reason)
        };
        assert_eq!(why(2), Some("no_duration"));
        assert_eq!(why(3), Some("no_anchor"));
        assert_eq!(why(4), None, "node 4 is anchored through node 1");
        assert!(
            r.ignored_edges.contains(&id(1001)),
            "the edge from the unplaced node is ignored and listed"
        );
        assert!(r.nodes.iter().all(|x| x.pid != id(3)));
    }

    #[test]
    fn logic_that_pushes_a_start_later_than_planned_is_reported() {
        // B is planned to start at 1 but its predecessor finishes at 4.
        let n = vec![
            anchored(1, 4, MON()),
            anchored(2, 2, MON() + Duration::days(1)),
        ];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]);
        assert_eq!(get(&r, 2).pushed_days, 3);
        assert_eq!(get(&r, 1).pushed_days, 0);
    }

    #[test]
    fn working_days_skip_weekends_and_holidays() {
        // Friday 2026-10-02, 2 business days: Fri and Mon.
        let fri = d(2026, 10, 2);
        let cal = WorkCalendar::mon_fri();
        let r = analyze(&[anchored(1, 2, fri)], &[], &cal, Options::default()).unwrap();
        assert_eq!(get(&r, 1).early_start, fri);
        assert_eq!(
            get(&r, 1).early_finish,
            d(2026, 10, 5),
            "the following Monday"
        );
        // A holiday on Monday pushes the finish to Tuesday.
        let cal = WorkCalendar::new(
            [true, true, true, true, true, false, false],
            [d(2026, 10, 5)],
        )
        .unwrap();
        let r = analyze(&[anchored(1, 2, fri)], &[], &cal, Options::default()).unwrap();
        assert_eq!(get(&r, 1).early_finish, d(2026, 10, 6));
        // A start planned on a Saturday begins on the Monday.
        let r = analyze(
            &[anchored(1, 1, d(2026, 10, 3))],
            &[],
            &WorkCalendar::mon_fri(),
            Options::default(),
        )
        .unwrap();
        assert_eq!(get(&r, 1).early_start, d(2026, 10, 5));
        assert!(WorkCalendar::new([false; 7], []).is_err());
    }

    #[test]
    fn calendar_index_and_date_are_inverse_on_working_days() {
        let cal = WorkCalendar::new(
            [true, true, true, true, false, false, true],
            [d(2026, 12, 25), d(2026, 1, 1)],
        )
        .unwrap();
        let mut day = d(2026, 1, 1);
        while day < d(2027, 3, 1) {
            if cal.is_working(day) {
                assert_eq!(cal.date_of(cal.index_of(day).unwrap()), day, "{day}");
            }
            day += Duration::days(1);
        }
        assert_eq!(cal.index_of(d(1969, 12, 31)), None);
    }

    #[test]
    fn schedule_fields_are_validated_all_at_once() {
        assert!(schedule_problems(Some(d(2026, 10, 5)), Some(3)).is_empty());
        assert!(
            schedule_problems(None, Some(0)).is_empty(),
            "a milestone, floating"
        );
        assert!(schedule_problems(None, None).is_empty());
        assert_eq!(schedule_problems(Some(d(1960, 1, 1)), Some(-1)).len(), 2);
        assert_eq!(schedule_problems(None, Some(3651)).len(), 1);
        assert_eq!(schedule_problems(Some(d(2101, 1, 1)), None).len(), 1);
    }

    #[test]
    fn a_milestone_has_zero_duration_and_one_date() {
        let n = vec![anchored(1, 3, MON()), node(2, 0)];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]);
        assert_eq!(get(&r, 2).early_start, get(&r, 2).early_finish);
        assert!(get(&r, 2).critical);
    }

    #[test]
    fn what_moved_between_two_runs_is_named_and_empty_when_nothing_did() {
        let (n, e) = textbook();
        let before = run(&n, &e);
        assert!(diff(&before, &before).is_empty());
        let mut n2 = n.clone();
        n2[1] = node(2, 6); // B grows by 2
        let after = run(&n2, &e);
        let moved = diff(&before, &after);
        let m = |p: u128| moved.iter().find(|x| x.pid == id(p));
        assert_eq!(
            m(2).map(|x| (x.moved, x.start_days, x.finish_days)),
            Some(("finish", 0, 2))
        );
        assert_eq!(
            m(4).map(|x| (x.moved, x.start_days, x.finish_days)),
            Some(("both", 2, 2))
        );
        assert!(m(1).is_none(), "A did not move");
    }

    fn constrained(mut n: Node, kind: ConstraintKind, date: NaiveDate) -> Node {
        n.constraint = Some(Constraint { kind, date });
        n
    }

    #[test]
    fn snet_pushes_the_early_start_later() {
        let n = vec![constrained(
            anchored(1, 3, MON()),
            ConstraintKind::Snet,
            MON() + Duration::days(4),
        )];
        let r = run(&n, &[]);
        assert_eq!(get(&r, 1).early_start, MON() + Duration::days(4));
        assert_eq!(get(&r, 1).total_float, 0);
    }

    #[test]
    fn fnlt_earlier_than_the_early_finish_gives_negative_float_and_does_not_move_it() {
        // A(3) -> B(4); B must finish no later than day 5 but cannot before day 7.
        let n = vec![
            anchored(1, 3, MON()),
            constrained(node(2, 4), ConstraintKind::Fnlt, MON() + Duration::days(4)),
        ];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]);
        let b = get(&r, 2);
        assert_eq!(
            b.early_finish,
            MON() + Duration::days(6),
            "early dates are untouched by FNLT"
        );
        assert!(b.total_float < 0, "negative float: {b:?}");
        assert!(b.critical);
        assert!(get(&r, 1).total_float < 0, "and it propagates back to A");
    }

    #[test]
    fn snlt_gives_negative_float_when_logic_starts_it_too_late() {
        let n = vec![
            anchored(1, 5, MON()),
            constrained(node(2, 2), ConstraintKind::Snlt, MON() + Duration::days(2)),
        ];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]);
        assert_eq!(get(&r, 2).total_float, -3, "starts at 5, must start by 2");
    }

    #[test]
    fn fnet_pushes_the_finish_out() {
        let n = vec![constrained(
            anchored(1, 2, MON()),
            ConstraintKind::Fnet,
            MON() + Duration::days(9),
        )];
        let r = run(&n, &[]);
        assert_eq!(get(&r, 1).early_finish, MON() + Duration::days(9));
    }

    #[test]
    fn a_hard_start_overrides_logic_and_says_so() {
        // B must start on day 1, but A (4 business days) forces day 4.
        let n = vec![
            anchored(1, 4, MON()),
            constrained(node(2, 2), ConstraintKind::Mso, MON() + Duration::days(1)),
        ];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]);
        let b = get(&r, 2);
        assert_eq!(
            b.early_start,
            MON() + Duration::days(1),
            "the pin wins over logic"
        );
        assert!(b.hard_conflict, "and the conflict is reported");
        assert_eq!(b.constraint, Some(ConstraintKind::Mso));
        // Without a conflict the flag stays off.
        let n = vec![
            anchored(1, 1, MON()),
            constrained(node(2, 2), ConstraintKind::Mso, MON() + Duration::days(5)),
        ];
        assert!(!get(&run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]), 2).hard_conflict);
    }

    #[test]
    fn a_hard_finish_pins_the_end() {
        let n = vec![constrained(
            node(1, 3),
            ConstraintKind::Mfo,
            MON() + Duration::days(7),
        )];
        let r = run(&n, &[]);
        assert_eq!(get(&r, 1).early_finish, MON() + Duration::days(7));
        assert_eq!(get(&r, 1).early_start, MON() + Duration::days(5));
    }

    #[test]
    fn a_deadline_makes_negative_float_but_never_moves_an_early_date() {
        let mut b = node(2, 4);
        b.deadline = Some(MON() + Duration::days(5));
        let n = vec![anchored(1, 3, MON()), b];
        let r = run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]);
        assert_eq!(
            get(&r, 2).early_start,
            MON() + Duration::days(3),
            "early dates ignore the deadline"
        );
        assert_eq!(get(&r, 2).early_finish, MON() + Duration::days(6));
        assert_eq!(
            get(&r, 2).total_float,
            -1,
            "finishes on day 6, deadline day 5"
        );
        // A comfortable deadline leaves float untouched.
        let mut b = node(2, 4);
        b.deadline = Some(MON() + Duration::days(30));
        let n = vec![anchored(1, 3, MON()), b];
        assert_eq!(
            get(&run(&n, &[edge(1, 1, 2, Kind::Fs, 0)]), 2).total_float,
            0,
            "the project finish still bounds it"
        );
    }

    #[test]
    fn a_part_time_assignee_stretches_a_task_across_their_off_days() {
        // Plan works Mon-Fri; the person works Mon-Wed. 4 business days of work from Monday:
        // Mon, Tue, Wed, then NEXT Monday.
        let plan = WorkCalendar::mon_fri();
        let mon_wed =
            WorkCalendar::new([true, true, true, false, false, false, false], []).unwrap();
        let person = plan.intersect(&mon_wed).unwrap();
        let mut t = anchored(1, 4, MON());
        t.own_calendar = Some(0);
        let r = analyze_with(
            &[t, node(2, 1)],
            &[edge(1, 1, 2, Kind::Fs, 0)],
            &plan,
            std::slice::from_ref(&person),
            Options::default(),
        )
        .unwrap();
        assert_eq!(get(&r, 1).early_finish, d(2026, 10, 12), "next Monday");
        assert_eq!(
            get(&r, 2).early_start,
            d(2026, 10, 13),
            "the successor follows on the plan's calendar"
        );
        assert_eq!(
            get(&r, 1).total_float,
            0,
            "backward pass is consistent with the forward one"
        );
        // A task starting on the person's day off waits for their next business day.
        let mut t = anchored(1, 1, d(2026, 10, 8)); // Thursday
        t.own_calendar = Some(0);
        let r = analyze_with(
            &[t],
            &[],
            &plan,
            std::slice::from_ref(&person),
            Options::default(),
        )
        .unwrap();
        assert_eq!(
            get(&r, 1).early_start,
            d(2026, 10, 12),
            "Thursday is the person's day off"
        );
    }

    #[test]
    fn an_absence_is_a_holiday_of_the_person_and_intersection_keeps_both() {
        let plan = WorkCalendar::new([true; 7], [d(2026, 10, 7)]).unwrap();
        let person = WorkCalendar::new([true; 7], [d(2026, 10, 6)]).unwrap();
        let both = plan.intersect(&person).unwrap();
        assert!(!both.is_working(d(2026, 10, 7)) && !both.is_working(d(2026, 10, 6)));
        assert!(both.is_working(d(2026, 10, 5)));
        let weekend =
            WorkCalendar::new([false, false, false, false, false, true, true], []).unwrap();
        assert!(
            WorkCalendar::mon_fri().intersect(&weekend).is_err(),
            "no common business day"
        );
    }

    #[test]
    fn a_constraint_date_out_of_range_leaves_the_node_unscheduled() {
        let n = vec![constrained(
            anchored(1, 1, MON()),
            ConstraintKind::Snet,
            d(1960, 1, 1),
        )];
        let r = run(&n, &[]);
        assert_eq!(r.unscheduled[0].reason, "date_out_of_range");
    }

    #[test]
    fn constraints_are_validated_all_at_once() {
        assert!(constraint_problems(None, None, None).is_empty());
        assert!(
            constraint_problems(Some("fnlt"), Some(d(2026, 10, 5)), Some(d(2026, 12, 1)))
                .is_empty()
        );
        assert_eq!(
            constraint_problems(Some("fnlt"), None, None).len(),
            1,
            "a kind needs a date"
        );
        assert_eq!(
            constraint_problems(None, Some(d(2026, 10, 5)), None).len(),
            1,
            "a date needs a kind"
        );
        assert_eq!(
            constraint_problems(Some("alap"), Some(d(2026, 10, 5)), None).len(),
            1
        );
        assert_eq!(
            constraint_problems(Some("zzz"), Some(d(1960, 1, 1)), Some(d(2200, 1, 1))).len(),
            3
        );
    }

    #[test]
    fn constraint_names_parse() {
        assert_eq!(ConstraintKind::parse("FNLT"), Some(ConstraintKind::Fnlt));
        assert!(ConstraintKind::parse("alap").is_none(), "not supported yet");
        assert!(
            ConstraintKind::Mso.is_hard()
                && ConstraintKind::Mfo.is_hard()
                && !ConstraintKind::Snet.is_hard()
        );
    }

    #[test]
    #[expect(
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation,
        clippy::cast_lossless
    )]
    fn per_node_calendars_and_constraints_keep_the_network_consistent() {
        let mut seed = 0xD1B5_4A32_D192_ED03_u64;
        let mut next = move |m: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % m
        };
        let plan = WorkCalendar::mon_fri();
        let mwf = WorkCalendar::new(
            [true, false, true, false, true, false, false],
            [d(2026, 10, 14)],
        )
        .unwrap();
        let part = plan.intersect(&mwf).unwrap();
        let kinds = [
            ConstraintKind::Snet,
            ConstraintKind::Fnet,
            ConstraintKind::Snlt,
            ConstraintKind::Fnlt,
        ];
        for _ in 0..200 {
            let count = 2 + next(8) as u128;
            let nodes: Vec<Node> = (1..=count)
                .map(|i| {
                    let mut n = if i == 1 {
                        anchored(i, 1 + next(5) as i64, MON())
                    } else {
                        node(i, next(5) as i64)
                    };
                    if next(3) == 0 {
                        n.own_calendar = Some(0);
                    }
                    if next(5) == 0 {
                        n.constraint = Some(Constraint {
                            kind: kinds[next(4) as usize],
                            date: MON() + Duration::days(next(40) as i64),
                        });
                    }
                    if next(6) == 0 {
                        n.deadline = Some(MON() + Duration::days(next(60) as i64));
                    }
                    n
                })
                .collect();
            let mut edges = Vec::new();
            for s in 2..=count {
                for p in 1..s {
                    if next(3) == 0 {
                        let kind = [Kind::Fs, Kind::Ss, Kind::Ff, Kind::Sf][next(4) as usize];
                        edges.push(edge(edges.len() as u128, p, s, kind, next(5) as i64 - 1));
                    }
                }
            }
            let r = analyze_with(
                &nodes,
                &edges,
                &plan,
                std::slice::from_ref(&part),
                Options::default(),
            )
            .unwrap();
            for n in &r.nodes {
                let (ls, es) = (
                    plan.index_of(n.late_start).unwrap(),
                    plan.index_of(n.early_start).unwrap(),
                );
                assert_eq!(
                    n.total_float,
                    ls - es,
                    "float is LS - ES in plan business days: {n:?}"
                );
                assert!(n.free_float >= 0, "{n:?}");
                assert!(plan.is_working(n.early_start) && plan.is_working(n.early_finish));
            }
            for e in &edges {
                let (Some(p), Some(s)) = (
                    r.nodes.iter().find(|n| n.pid == e.pred),
                    r.nodes.iter().find(|n| n.pid == e.succ),
                ) else {
                    continue;
                };
                let ok = match e.kind {
                    Kind::Fs => s.es_index >= p.ef_index + e.lag,
                    Kind::Ss => s.es_index >= p.es_index + e.lag,
                    Kind::Ff => s.ef_index >= p.ef_index + e.lag,
                    Kind::Sf => s.ef_index >= p.es_index + e.lag,
                };
                assert!(ok, "edge {e:?}: {p:?} -> {s:?}");
            }
        }
    }

    #[test]
    #[expect(
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation,
        clippy::cast_lossless
    )]
    fn invariants_hold_on_many_random_networks() {
        // A tiny deterministic generator so the test needs no dependency.
        let mut seed = 0x9E37_79B9_7F4A_7C15_u64;
        let mut next = move |m: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % m
        };
        for _ in 0..200 {
            let count = 2 + next(9) as u128;
            let nodes: Vec<Node> = (1..=count)
                .map(|i| {
                    if i == 1 || next(4) == 0 {
                        anchored(i, next(6) as i64, MON() + Duration::days(next(10) as i64))
                    } else {
                        node(i, next(6) as i64)
                    }
                })
                .collect();
            let mut edges = Vec::new();
            for s in 2..=count {
                for p in 1..s {
                    if next(3) == 0 {
                        let kind = [Kind::Fs, Kind::Ss, Kind::Ff, Kind::Sf][next(4) as usize];
                        edges.push(edge(edges.len() as u128, p, s, kind, next(7) as i64 - 2));
                    }
                }
            }
            let r = analyze(
                &nodes,
                &edges,
                &WorkCalendar::seven_day(),
                Options::default(),
            )
            .unwrap();
            for n in &r.nodes {
                assert!(
                    n.total_float >= 0,
                    "no constraints, so no negative float: {n:?}"
                );
                assert!(
                    n.free_float >= 0 && n.free_float <= n.total_float,
                    "0 <= free <= total: {n:?}"
                );
            }
            if !r.nodes.is_empty() {
                assert!(
                    r.nodes.iter().any(|n| n.critical),
                    "some node is always critical"
                );
            }
            // Every used edge is satisfied by the early dates.
            for e in &edges {
                let (Some(p), Some(s)) = (
                    r.nodes.iter().find(|n| n.pid == e.pred),
                    r.nodes.iter().find(|n| n.pid == e.succ),
                ) else {
                    continue;
                };
                let ok = match e.kind {
                    Kind::Fs => s.es_index >= p.ef_index + e.lag,
                    Kind::Ss => s.es_index >= p.es_index + e.lag,
                    Kind::Ff => s.ef_index >= p.ef_index + e.lag,
                    Kind::Sf => s.ef_index >= p.es_index + e.lag,
                };
                assert!(ok, "edge {e:?} violated by {p:?} -> {s:?}");
            }
            // Deterministic.
            let again = analyze(
                &nodes,
                &edges,
                &WorkCalendar::seven_day(),
                Options::default(),
            )
            .unwrap();
            assert_eq!(
                serde_json::to_string(&r).unwrap(),
                serde_json::to_string(&again).unwrap()
            );
        }
    }

    /// Not a unit test: a timing probe for very large plans (RG-16). Run with
    /// `cargo test --release large_plan_timing -- --ignored --nocapture`.
    #[test]
    #[ignore = "a timing probe; run on demand"]
    fn large_plan_timing() {
        for n in [1_000_u128, 4_000, 12_667, 30_000] {
            let nodes: Vec<Node> = (0..n)
                .map(|i| {
                    let dur = 1 + (i % 9) as i64;
                    if i == 0 {
                        anchored(1, dur, MON())
                    } else {
                        node(i + 1, dur)
                    }
                })
                .collect();
            // A chain with a skip dependency every fifth task, like the load test's seed.
            let mut edges = Vec::new();
            for i in 1..n {
                edges.push(edge(i, i, i + 1, Kind::Fs, 0));
                if i % 5 == 0 && i >= 3 {
                    edges.push(edge(n + i, i - 2, i + 1, Kind::Fs, 0));
                }
            }
            let started = std::time::Instant::now();
            let r = run(&nodes, &edges);
            eprintln!(
                "analyze {n} tasks, {} edges: {:?} ({} results)",
                edges.len(),
                started.elapsed(),
                r.nodes.len()
            );
        }
    }
}

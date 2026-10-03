//! The critical-path-method engine (SC-1, PJ-8): pure, DB-free, deterministic.
//!
//! A **forward pass** gives each node its earliest start and finish, a
//! **backward pass** its latest, and from the two come **total float** (how long
//! the node can slip before the project finish moves) and **free float** (how
//! long before it delays any successor). All four relationship types (FS, SS,
//! FF, SF) are supported, each with a **lag or a lead** (negative lag).
//!
//! Time is counted in **working days of a [`WorkCalendar`]**: a node of duration
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
/// the working-day index.
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

    fn per_week(&self) -> i64 {
        i64::try_from(self.working.iter().filter(|w| **w).count()).unwrap_or(7)
    }

    /// Whether `date` is a working day.
    #[must_use]
    pub fn is_working(&self, date: NaiveDate) -> bool {
        self.working[date.weekday().num_days_from_monday() as usize]
            && !self.holidays.contains(&date)
    }

    /// The working-day index of `date`: how many working days lie strictly
    /// before it. For a working date this is its own index; a non-working date
    /// maps to the index of the **next** working day. `None` before 1970-01-05.
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

    /// The date of the working day with `index` (the inverse of
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

/// One schedulable node (a plan with a timeframe, or a task).
#[derive(Debug, Clone, Copy)]
pub struct Node {
    /// Identity.
    pub pid: Uuid,
    /// Working days; `0` is a milestone; `None` cannot be scheduled.
    pub duration: Option<i64>,
    /// Planned start: a lower bound on the earliest start.
    pub planned_start: Option<NaiveDate>,
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
    /// Working days; negative is a lead.
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
    /// Earliest finish date (the last working day; equals the start for a milestone).
    pub early_finish: NaiveDate,
    /// Latest start date.
    pub late_start: NaiveDate,
    /// Latest finish date.
    pub late_finish: NaiveDate,
    /// Working days it can slip before the project finish moves.
    pub total_float: i64,
    /// Working days it can slip before any successor is delayed.
    pub free_float: i64,
    /// `total_float <= 0`.
    pub critical: bool,
    /// `0 < total_float <= near_critical_days`.
    pub near_critical: bool,
    /// The planned start as entered, if any.
    pub planned_start: Option<NaiveDate>,
    /// Working days logic pushes the start **later than planned** (0 if not).
    pub pushed_days: i64,
    /// Working-day index of the earliest start (for [`diff`]).
    pub es_index: i64,
    /// Working-day index one past the last working day of the earliest finish.
    pub ef_index: i64,
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

/// Run the analysis.
///
/// # Errors
///
/// [`CpmError::Cycle`] when the edges contain a cycle.
#[allow(clippy::too_many_lines)]
pub fn analyze(
    nodes: &[Node],
    edges: &[Edge],
    calendar: &WorkCalendar,
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

    // Forward pass.
    let mut es: Vec<Option<i64>> = vec![None; nodes.len()];
    let mut ef: Vec<Option<i64>> = vec![None; nodes.len()];
    let mut unscheduled: HashMap<usize, &'static str> = HashMap::new();
    for &i in &sorted {
        let Some(d) = dur[i] else {
            unscheduled.insert(i, "no_duration");
            continue;
        };
        if nodes[i].planned_start.is_some() && planned_idx[i].is_none() {
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
        let Some(start) = lower else {
            unscheduled.insert(i, "no_anchor");
            continue;
        };
        let start = start.max(0);
        es[i] = Some(start);
        ef[i] = Some(start + d);
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
        lf[i] = Some(upper);
        ls[i] = Some(upper - d);
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

/// Most working days a duration or lag may be (about fourteen years).
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

/// How a node's dates moved between two analyses (PJ-8).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Moved {
    /// Identity.
    pub pid: Uuid,
    /// `start`, `finish` or `both`.
    pub moved: &'static str,
    /// Working days the start moved (positive = later).
    pub start_days: i64,
    /// Working days the finish moved (positive = later).
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
            planned_start: None,
        }
    }
    fn anchored(n: u128, dur: i64, start: NaiveDate) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            planned_start: Some(start),
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
            "C can slip 2 days before delaying D"
        );
        assert_eq!(get(&r, 2).free_float, 0);
        assert_eq!(r.chains, vec![vec![id(1), id(2), id(4)]]);
        assert!(r.unscheduled.is_empty() && r.ignored_edges.is_empty());
    }

    #[test]
    fn a_lag_lengthens_the_chain_and_can_change_which_one_is_critical() {
        let (n, mut e) = textbook();
        // 3 days of lag on C→D: C finishes at 5, D can start at 8 > B's 7.
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
        // Friday 2026-10-02, 2 working days: Fri and Mon.
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

    #[test]
    #[allow(
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
}

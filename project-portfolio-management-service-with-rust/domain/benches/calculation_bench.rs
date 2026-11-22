#![warn(clippy::pedantic)]

//! Criterion benchmarks for the domain crate's pure calculation engines and the date work done by its modules.
//!
//! Run from the service directory with `cargo bench -p project-portfolio-management-domain --bench
//! calculation_bench`. Nothing here touches PostgreSQL, and nothing here needs the web service: this file depends
//! on the domain crate alone, so building it does not compile axum, axa, sqlx or the service. (It was
//! `benches/calculation_bench.rs` in the service crate until 18 November 2026; the benchmark IDs are unchanged.)
//!
//! Dates are built from `YYYY-MM-DD` text through `Date::parse_from_str(s.trim(), "%Y-%m-%d")`, which is exactly
//! what the service's `plan_templates::full_date` does, so the fixtures are the ones the service file built. Day
//! offsets are computed here on plain integers.
//!
//! Elsewhere: the row structs' JSON (`timestamps/decode_plans/…` and the like) needs the service and is in the
//! service's `benches/service_bench.rs`; serde, parsing and the type-level calendar arithmetic of the date types
//! themselves are in `datetime/benches/datetime_bench.rs`.
//!
//! # The groups
//!
//! - **`schedule_risk`**: the Monte Carlo behind `GET …/schedule-risk` (`schedule_risk::simulate`) at the default
//!   2,000 iterations (`DEFAULT_ITERATIONS`, what a request without `?iterations=` runs), on the same chain as
//!   `service_bench`'s `cpm` group, with three-point estimates on nine tasks in ten and a risk event per twenty
//!   tasks. Each iteration is a critical-path run, so this is about 2,000 times `cpm/analyze`.
//! - **`cost_risk`**: the cost Monte Carlo (`cost_risk::simulate`) at its default 5,000 iterations, over budget
//!   lines in two currencies with a risk event per ten lines. 900 lines and 90 events is close to the largest run
//!   the work cap (`MAX_WORK`, 5,000,000) allows at the default iterations.
//! - **`leveling`**: `leveling::propose` on parallel streams of tasks that share three people, so there are
//!   over-allocations to resolve.
//! - **`schedule_health`**: the fourteen checks (`schedule_health::assess`), which include a second critical-path
//!   run for the critical path test.
//! - **`mspdi`**: reading an MS Project XML file (`mspdi::parse`), the unit tests' sample and generated files of
//!   1,000 and 5,000 tasks (5,000 is `MAX_TASKS`), and writing one back (`mspdi::write`, the export).
//! - **`earned_value`**: the per-request work of `GET …/earned-value` and the EVM views: percent complete over a
//!   plan's tasks, one currency's block over a baseline of monthly or weekly periods, the EAC variants and TCPI.
//! - **`timestamps`**: the domain modules' date work: the Jira and flexible date parsers, ISO weeks, months, and
//!   the calendar arithmetic (business-day index, report schedules, financial years, absence expansion). The date
//!   types' own operations under these are measured one by one in `datetime_bench`.

use std::{collections::HashMap, fmt::Write as _, hint::black_box};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use project_portfolio_management_domain::{
    calendars, capacity_heatmap, cost_risk,
    cpm::{self, ConstraintKind, Edge, Kind, Node},
    datetime::Date,
    earned_value, evm, financial_year, leveling, mspdi, report_schedules, schedule_health,
    schedule_risk, timesheets, tracker_import, visibility,
};
use uuid::Uuid;

/// A date from `YYYY-MM-DD`, read as the service's `plan_templates::full_date` reads one.
fn full_date(s: &str) -> Option<Date> {
    Date::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

/// A date from `YYYY-MM-DD`.
macro_rules! day {
    ($s:expr) => {
        full_date(&$s).expect("a valid ISO date")
    };
}

/// Days since 1970-01-01 of a civil date (Howard Hinnant's algorithm), on plain integers.
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// `YYYY-MM-DD` of a day number since 1970-01-01 (the inverse of [`days_from_civil`]).
fn iso(z: i64) -> String {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// `YYYY-MM-DD` of `offset` days after Monday 2026-11-02, the plans' start.
fn iso_from_start(offset: i64) -> String {
    iso(days_from_civil(2026, 11, 2) + offset)
}

fn count(n: usize) -> u64 {
    u64::try_from(n).unwrap_or(u64::MAX)
}

fn small(i: usize, m: usize) -> i64 {
    i64::try_from(i % m).unwrap_or(0)
}

fn id(i: usize) -> Uuid {
    Uuid::from_u128(u128::try_from(i).unwrap_or(0) + 1)
}

/// The plan calendar: Monday to Friday, with the UK bank holidays of the plans' first year, built through
/// `calendars::build` as the service builds a plan's calendar.
fn plan_calendar() -> cpm::WorkCalendar {
    let holidays: Vec<_> = [
        "2026-12-25",
        "2026-12-28",
        "2027-01-01",
        "2027-03-26",
        "2027-03-29",
        "2027-05-03",
        "2027-05-31",
        "2027-08-30",
        "2027-12-27",
        "2027-12-28",
    ]
    .iter()
    .map(|s| day!(s))
    .collect();
    calendars::build("1111100", &holidays).expect("a working week")
}

/// The `service_bench` `cpm` chain: `n` one-to-nine-day tasks, the first dated, a skip dependency every fifth.
fn chain(n: usize) -> (Vec<Node>, Vec<Edge>) {
    let start = day!("2026-11-02");
    let nodes = (0..n)
        .map(|i| Node {
            pid: id(i),
            duration: Some(1 + small(i, 9)),
            planned_start: (i == 0).then_some(start),
            ..Node::default()
        })
        .collect();
    let edge = |k: usize, a: usize, b: usize| Edge {
        pid: Uuid::from_u128(1 << 64 | u128::try_from(k).unwrap_or(0)),
        pred: id(a),
        succ: id(b),
        kind: Kind::Fs,
        lag: 0,
    };
    let mut edges = Vec::new();
    for i in 1..n {
        edges.push(edge(i, i - 1, i));
        if i % 5 == 0 {
            edges.push(edge(n + i, i - 3, i));
        }
    }
    (nodes, edges)
}

fn bench_schedule_risk(c: &mut Criterion) {
    let mut group = c.benchmark_group("schedule_risk");
    group.sample_size(10);
    let calendar = plan_calendar();
    for n in [100_usize, 1_000] {
        let (nodes, edges) = chain(n);
        // Nine tasks in ten have an estimate (80% to 150% of the likely duration); the tenth uses the fallback band.
        let estimates: HashMap<Uuid, schedule_risk::Estimate> = nodes
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 10 != 9)
            .map(|(_, node)| {
                let likely = node.duration.unwrap_or(1);
                (
                    node.pid,
                    schedule_risk::Estimate {
                        optimistic: (likely * 4 / 5).max(1),
                        pessimistic: likely * 3 / 2 + 1,
                    },
                )
            })
            .collect();
        let events: Vec<schedule_risk::Event> = (0..n)
            .step_by(20)
            .map(|i| schedule_risk::Event {
                risk: Uuid::from_u128(2 << 64 | u128::try_from(i).unwrap_or(0)),
                task: id(i),
                probability_bp: 1_000 + small(i, 3) * 1_000,
                impact_days: 2 + small(i, 4),
            })
            .collect();
        let params = schedule_risk::Params::default();
        assert_eq!(params.iterations, schedule_risk::DEFAULT_ITERATIONS);
        group.throughput(Throughput::Elements(count(n)));
        group.bench_function(BenchmarkId::new("simulate_default_iterations", n), |b| {
            b.iter(|| {
                schedule_risk::simulate(
                    black_box(&nodes),
                    black_box(&edges),
                    &calendar,
                    &[],
                    &estimates,
                    &events,
                    params,
                )
                .expect("a schedulable network")
            });
        });
    }
    group.finish();
}

fn bench_cost_risk(c: &mut Criterion) {
    let mut group = c.benchmark_group("cost_risk");
    group.sample_size(10);
    for n in [100_usize, 900] {
        let currency = |i: usize| if i.is_multiple_of(4) { "EUR" } else { "GBP" }.to_string();
        let lines: Vec<cost_risk::Line> = (0..n)
            .map(|i| {
                let likely = 100_000 + small(i, 50) * 25_000;
                cost_risk::Line {
                    pid: id(i),
                    plan: Uuid::from_u128(3 << 64 | u128::try_from(i % 10).unwrap_or(0)),
                    currency: currency(i),
                    actual: likely * small(i, 5) / 10,
                    likely,
                    // Four lines in five have a range; the fifth uses the fallback band.
                    low: (i % 5 != 4).then_some(likely * 9 / 10),
                    high: (i % 5 != 4).then_some(likely * 3 / 2),
                }
            })
            .collect();
        let events: Vec<cost_risk::CostEvent> = (0..n)
            .step_by(10)
            .map(|i| cost_risk::CostEvent {
                risk: Uuid::from_u128(4 << 64 | u128::try_from(i).unwrap_or(0)),
                plan: lines[i].plan,
                currency: currency(i),
                probability_bp: 500 + small(i, 4) * 1_000,
                impact_minor: 50_000 + small(i, 7) * 10_000,
            })
            .collect();
        let params = cost_risk::Params::default();
        assert_eq!(params.iterations, cost_risk::DEFAULT_ITERATIONS);
        group.throughput(Throughput::Elements(count(n)));
        group.bench_function(BenchmarkId::new("simulate_default_iterations", n), |b| {
            b.iter(|| {
                cost_risk::simulate(black_box(&lines), black_box(&events), params)
                    .expect("within the work cap")
            });
        });
    }
    group.finish();
}

/// `streams` parallel chains of ten tasks from the same start, chain `s` with durations that give it float against
/// the longest; task `k` of every chain is done by person `k % 3`, so the same person is wanted by several
/// chains on the same days.
fn streams(streams: usize) -> (Vec<Node>, Vec<Edge>, Vec<leveling::Demand>) {
    let start = day!("2026-11-02");
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut demands = Vec::new();
    for s in 0..streams {
        for k in 0..10 {
            let i = s * 10 + k;
            nodes.push(Node {
                pid: id(i),
                duration: Some(1 + small(k + s, 4)),
                planned_start: (k == 0).then_some(start),
                ..Node::default()
            });
            if k > 0 {
                edges.push(Edge {
                    pid: Uuid::from_u128(5 << 64 | u128::try_from(i).unwrap_or(0)),
                    pred: id(i - 1),
                    succ: id(i),
                    kind: Kind::Fs,
                    lag: 0,
                });
            }
            demands.push(leveling::Demand {
                pid: id(i),
                person: format!("person:{}", k % 3),
                percent: 50,
                movable: k != 0,
            });
        }
    }
    // One long chain sets the finish, so the others have float to absorb a delay.
    let tail = streams * 10;
    nodes.push(Node {
        pid: id(tail),
        duration: Some(i64::try_from(streams).unwrap_or(1) * 12),
        planned_start: Some(start),
        ..Node::default()
    });
    (nodes, edges, demands)
}

fn bench_leveling(c: &mut Criterion) {
    let mut group = c.benchmark_group("leveling");
    group.sample_size(10);
    let calendar = plan_calendar();
    let resources: HashMap<String, leveling::Resource> = HashMap::new();
    for n in [5_usize, 20] {
        let (nodes, edges, demands) = streams(n);
        let input = leveling::Input {
            nodes: &nodes,
            edges: &edges,
            calendar: &calendar,
            own: &[],
            demands: &demands,
            resources: &resources,
        };
        // Say what the input makes the engine do, so the numbers can be read.
        let once = leveling::propose(&input, leveling::Params::default()).expect("no cycle");
        eprintln!(
            "leveling/{}: {} over-runs before, {} moves, {} unresolved, truncated {}",
            nodes.len(),
            once.over_before.len(),
            once.moves.len(),
            once.unresolved.len(),
            once.truncated
        );
        assert!(
            !once.over_before.is_empty(),
            "the input over-allocates someone"
        );
        group.throughput(Throughput::Elements(count(nodes.len())));
        group.bench_function(BenchmarkId::new("propose", nodes.len()), |b| {
            b.iter(|| {
                leveling::propose(black_box(&input), leveling::Params::default()).expect("no cycle")
            });
        });
    }
    group.finish();
}

fn bench_schedule_health(c: &mut Criterion) {
    let mut group = c.benchmark_group("schedule_health");
    group.sample_size(20);
    let calendar = plan_calendar();
    let (thresholds, _) = schedule_health::parse_thresholds(None);
    for n in [100_usize, 1_000] {
        let (mut nodes, edges) = chain(n);
        // A deadline on every twentieth task and a start-no-earlier-than on every fiftieth.
        for (i, node) in nodes.iter_mut().enumerate() {
            let offset = i64::try_from(i).unwrap_or(0);
            if i % 20 == 10 {
                node.deadline = Some(day!(iso_from_start(offset * 2)));
            }
            if i % 50 == 25 {
                node.constraint = Some(cpm::Constraint {
                    kind: ConstraintKind::Snet,
                    date: day!(iso_from_start(offset)),
                });
            }
        }
        // The first third done, a baseline finish on every task, one in eight unassigned.
        let tasks: Vec<schedule_health::Task> = (0..n)
            .map(|i| {
                let offset = i64::try_from(i).unwrap_or(0);
                schedule_health::Task {
                    pid: id(i),
                    complete: i < n / 3,
                    has_resource: i % 8 != 0,
                    done_on: (i < n / 3).then(|| day!(iso_from_start(offset * 5 / 4))),
                    baseline_finish: Some(day!(iso_from_start(offset * 5 / 4))),
                }
            })
            .collect();
        let as_of = day!(iso_from_start(i64::try_from(n).unwrap_or(0) * 5 / 12));
        let input = schedule_health::Input {
            tasks: &tasks,
            nodes: &nodes,
            edges: &edges,
            calendar: &calendar,
            own: &[],
            as_of,
            baseline_exists: true,
        };
        let once = schedule_health::assess(&input, &thresholds);
        eprintln!(
            "schedule_health/{n}: {} passed, {} failed, {} not computable",
            once.passed, once.failed, once.not_computable
        );
        group.throughput(Throughput::Elements(count(n)));
        group.bench_function(BenchmarkId::new("assess", n), |b| {
            b.iter(|| schedule_health::assess(black_box(&input), &thresholds));
        });
    }
    group.finish();
}

/// A copy of the sample file in `src/mspdi.rs`'s unit tests (a calendar with a holiday, a summary, a constraint,
/// a deadline, four link types, a cross-project link, an inactive task, resources and assignments).
const MSPDI_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Project xmlns="http://schemas.microsoft.com/project">
  <Name>Apollo</Name>
  <MinutesPerDay>480</MinutesPerDay>
  <CalendarUID>1</CalendarUID>
  <Calendars>
    <Calendar>
      <UID>1</UID><IsBaseCalendar>1</IsBaseCalendar>
      <WeekDays>
        <WeekDay><DayType>1</DayType><DayWorking>0</DayWorking></WeekDay>
        <WeekDay><DayType>2</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>3</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>4</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>5</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>6</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>7</DayType><DayWorking>0</DayWorking></WeekDay>
      </WeekDays>
      <Exceptions>
        <Exception><DayWorking>0</DayWorking>
          <TimePeriod><FromDate>2026-12-24T00:00:00</FromDate><ToDate>2026-12-25T23:59:00</ToDate></TimePeriod></Exception>
      </Exceptions>
    </Calendar>
  </Calendars>
  <Tasks>
    <Task><UID>0</UID><Name>Apollo</Name><Summary>1</Summary></Task>
    <Task><UID>1</UID><Name>Design</Name><OutlineLevel>1</OutlineLevel><Summary>1</Summary>
      <Start>2026-10-05T08:00:00</Start><Duration>PT64H0M0S</Duration></Task>
    <Task><UID>2</UID><Name>Draft</Name><OutlineLevel>2</OutlineLevel><Start>2026-10-05T08:00:00</Start>
      <Duration>PT24H0M0S</Duration><PercentComplete>100</PercentComplete><Notes>Use the template</Notes></Task>
    <Task><UID>3</UID><Name>Review</Name><OutlineLevel>2</OutlineLevel><Duration>PT32H0M0S</Duration>
      <ConstraintType>6</ConstraintType><ConstraintDate>2026-10-12T08:00:00</ConstraintDate>
      <Deadline>2026-10-30T17:00:00</Deadline><PercentComplete>40</PercentComplete>
      <PredecessorLink><PredecessorUID>2</PredecessorUID><Type>1</Type><LinkLag>4800</LinkLag><LagFormat>7</LagFormat></PredecessorLink>
    </Task>
    <Task><UID>4</UID><Name>Sign-off</Name><OutlineLevel>1</OutlineLevel><Milestone>1</Milestone><Duration>PT0H0M0S</Duration>
      <ConstraintType>2</ConstraintType><ConstraintDate>2026-11-02T08:00:00</ConstraintDate>
      <PredecessorLink><PredecessorUID>3</PredecessorUID><Type>3</Type></PredecessorLink>
      <PredecessorLink><PredecessorUID>2</PredecessorUID><Type>0</Type><LinkLag>-4800</LinkLag></PredecessorLink>
      <PredecessorLink><PredecessorUID>9</PredecessorUID><Type>2</Type><CrossProject>1</CrossProject></PredecessorLink>
    </Task>
    <Task><UID>5</UID><Name>Old</Name><Active>0</Active></Task>
    <Task><UID>6</UID><Name>Half day</Name><Duration>PT4H0M0S</Duration><ConstraintType>1</ConstraintType></Task>
  </Tasks>
  <Resources>
    <Resource><UID>0</UID><Name>Unassigned</Name></Resource>
    <Resource><UID>1</UID><Name>Alex</Name></Resource>
  </Resources>
  <Assignments>
    <Assignment><TaskUID>2</TaskUID><ResourceUID>1</ResourceUID></Assignment>
    <Assignment><TaskUID>3</TaskUID><ResourceUID>-65535</ResourceUID></Assignment>
  </Assignments>
</Project>"#;

/// A generated MSPDI file of `n` tasks: a summary every tenth task, a chain of finish-to-start links (with a lag
/// every seventh), a start-no-earlier-than every 25th, a deadline every 50th, notes every 3rd, 20 resources and
/// an assignment per task.
fn generated_mspdi(n: usize) -> String {
    let mut x = String::with_capacity(n * 400);
    x.push_str(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
         <Project xmlns=\"http://schemas.microsoft.com/project\">\n  <Name>Generated</Name>\n  \
         <MinutesPerDay>480</MinutesPerDay>\n  <CalendarUID>1</CalendarUID>\n  <Calendars>\n    <Calendar>\n      \
         <UID>1</UID><IsBaseCalendar>1</IsBaseCalendar>\n      <WeekDays>\n",
    );
    for d in 1..=7 {
        let working = u8::from((2..=6).contains(&d));
        let _ = writeln!(
            x,
            "        <WeekDay><DayType>{d}</DayType><DayWorking>{working}</DayWorking></WeekDay>"
        );
    }
    x.push_str("      </WeekDays>\n      <Exceptions>\n");
    for h in ["2026-12-25", "2026-12-28", "2027-01-01"] {
        let _ = writeln!(
            x,
            "        <Exception><DayWorking>0</DayWorking><TimePeriod><FromDate>{h}T00:00:00</FromDate><ToDate>{h}T23:59:00</ToDate></TimePeriod></Exception>"
        );
    }
    x.push_str("      </Exceptions>\n    </Calendar>\n  </Calendars>\n  <Tasks>\n");
    x.push_str("    <Task><UID>0</UID><Name>Generated</Name><Summary>1</Summary></Task>\n");
    for i in 1..=n {
        let offset = i64::try_from(i).unwrap_or(0);
        let summary = i % 10 == 1;
        let _ = write!(
            x,
            "    <Task><UID>{i}</UID><ID>{i}</ID><Name>Task {i} of the generated plan</Name><OutlineLevel>{}</OutlineLevel>",
            if summary { 1 } else { 2 }
        );
        if summary {
            x.push_str("<Summary>1</Summary>");
        } else {
            let _ = write!(
                x,
                "<Start>{}T08:00:00</Start><Duration>PT{}H0M0S</Duration><PercentComplete>{}</PercentComplete>",
                iso_from_start(offset),
                8 * (1 + i % 9),
                (i * 37) % 101
            );
        }
        if i % 25 == 0 {
            let _ = write!(
                x,
                "<ConstraintType>4</ConstraintType><ConstraintDate>{}T08:00:00</ConstraintDate>",
                iso_from_start(offset)
            );
        }
        if i % 50 == 0 {
            let _ = write!(
                x,
                "<Deadline>{}T17:00:00</Deadline>",
                iso_from_start(offset * 2)
            );
        }
        if i % 3 == 0 {
            x.push_str("<Notes>Agree the scope with the service owner first</Notes>");
        }
        if i > 1 && !summary {
            let _ = write!(
                x,
                "<PredecessorLink><PredecessorUID>{}</PredecessorUID><Type>1</Type>",
                i - 1
            );
            if i % 7 == 0 {
                x.push_str("<LinkLag>4800</LinkLag><LagFormat>7</LagFormat>");
            }
            x.push_str("</PredecessorLink>");
        }
        x.push_str("</Task>\n");
    }
    x.push_str("  </Tasks>\n  <Resources>\n");
    for r in 1..=20 {
        let _ = writeln!(
            x,
            "    <Resource><UID>{r}</UID><Name>Person {r}</Name></Resource>"
        );
    }
    x.push_str("  </Resources>\n  <Assignments>\n");
    for i in 1..=n {
        let _ = writeln!(
            x,
            "    <Assignment><TaskUID>{i}</TaskUID><ResourceUID>{}</ResourceUID></Assignment>",
            1 + i % 20
        );
    }
    x.push_str("  </Assignments>\n</Project>\n");
    x
}

fn bench_mspdi(c: &mut Criterion) {
    let mut group = c.benchmark_group("mspdi");
    group.sample_size(20);
    let files = [
        ("sample".to_string(), MSPDI_SAMPLE.to_string()),
        ("generated_1000".to_string(), generated_mspdi(1_000)),
        (
            "generated_5000".to_string(),
            generated_mspdi(mspdi::MAX_TASKS),
        ),
    ];
    for (name, xml) in &files {
        let parsed = mspdi::parse(xml).expect("a valid MSPDI file");
        eprintln!(
            "mspdi/{name}: {} bytes, {} tasks, {} links, {} unmapped",
            xml.len(),
            parsed.tasks.len(),
            parsed.links.len(),
            parsed.unmapped.len()
        );
        group.throughput(Throughput::Bytes(count(xml.len())));
        group.bench_function(BenchmarkId::new("parse", name), |b| {
            b.iter(|| mspdi::parse(black_box(xml)).expect("a valid MSPDI file"));
        });
        group.throughput(Throughput::Elements(count(parsed.tasks.len())));
        group.bench_function(BenchmarkId::new("write", name), |b| {
            b.iter(|| {
                mspdi::write(
                    "Generated",
                    "Costs and baselines are not carried.",
                    parsed.minutes_per_day,
                    black_box(&parsed.tasks),
                    &parsed.links,
                    parsed.calendar.as_ref(),
                )
            });
        });
    }
    group.finish();
}

/// `count` consecutive periods of `days` days each from the plans' start, 100,000 to 600,000 minor units each.
fn periods(count: usize, days: i64) -> Vec<earned_value::Period> {
    (0..count)
        .map(|k| {
            let from = i64::try_from(k).unwrap_or(0) * days;
            earned_value::Period {
                start: day!(iso_from_start(from)),
                end: day!(iso_from_start(from + days - 1)),
                planned_minor: 100_000 + small(k, 6) * 100_000,
            }
        })
        .collect()
}

fn bench_earned_value(c: &mut Criterion) {
    let mut group = c.benchmark_group("earned_value");
    for n in [100_usize, 1_000] {
        let tasks: Vec<earned_value::TaskInput> = (0..n)
            .map(|i| earned_value::TaskInput {
                pid: id(i),
                points: (i % 6 != 5).then(|| i32::try_from(1 + i % 8).unwrap_or(1)),
                state: match i % 3 {
                    0 => earned_value::TaskState::Done,
                    1 => earned_value::TaskState::Started,
                    _ => earned_value::TaskState::Todo,
                },
                method: (i % 10 == 0).then_some(earned_value::Method::Manual),
                manual_bp: (i % 10 == 0).then_some(4_000),
            })
            .collect();
        group.throughput(Throughput::Elements(count(n)));
        group.bench_function(BenchmarkId::new("percent_complete_points", n), |b| {
            b.iter(|| {
                earned_value::percent_complete(
                    earned_value::DefaultMethod::Points,
                    black_box(&tasks),
                )
            });
        });
    }
    group.throughput(Throughput::Elements(1));
    // A year of months, five years of months, and five years of weeks; status about 40% of the way through.
    for (name, baseline) in [
        ("12_monthly_periods", periods(12, 30)),
        ("60_monthly_periods", periods(60, 30)),
        ("260_weekly_periods", periods(260, 7)),
    ] {
        let span = i64::try_from(baseline.len()).unwrap_or(0)
            * if name.contains("weekly") { 7 } else { 30 };
        let as_of = day!(iso_from_start(span * 2 / 5));
        // Ahead of plan (EV above PV): earned schedule then searches past the status date.
        group.bench_function(BenchmarkId::new("block", name), |b| {
            b.iter(|| {
                earned_value::block(
                    black_box(&baseline),
                    55,
                    100,
                    Some(1_000_000),
                    black_box(as_of),
                )
                .expect("a baseline with periods")
            });
        });
        group.bench_function(BenchmarkId::new("pv_after", name), |b| {
            b.iter(|| {
                earned_value::pv_after(black_box(&baseline), baseline[0].start, black_box(span / 2))
            });
        });
    }
    let (thresholds, _) = evm::parse_thresholds(None);
    group.bench_function("eac_variants_tcpi_recommended", |b| {
        b.iter(|| {
            let eacs = evm::eac_variants(
                black_box(12_000_000),
                black_box(5_100_000),
                Some(5_600_000),
                Some(9_107),
                Some(9_400),
                Some(12_900_000),
            );
            let tcpi_budget = evm::tcpi(12_000_000, 5_100_000, Some(5_600_000), 12_000_000);
            let tcpi_estimate = evm::tcpi(12_000_000, 5_100_000, Some(5_600_000), 13_176_000);
            let pick = evm::recommended(Some(9_107), Some(9_400), &thresholds);
            (eacs, tcpi_budget, tcpi_estimate, pick)
        });
    });
    group.finish();
}

#[expect(clippy::too_many_lines)]
fn bench_timestamps(c: &mut Criterion) {
    let mut group = c.benchmark_group("timestamps");
    // A thousand dates (the same thousand as `datetime_bench`'s `timestamps` group).
    let n = 1_000_usize;
    let date_strings: Vec<String> = (0..n)
        .map(|i| iso_from_start(i64::try_from(i).unwrap_or(0) * 3 - 900))
        .collect();
    let dates: Vec<Date> = date_strings.iter().map(|s| day!(s)).collect();
    group.throughput(Throughput::Elements(count(n)));

    // The domain's own date parsers, over the same thousand dates.
    let jira: Vec<String> = (0..n)
        .map(|i| {
            let month = ["Jan", "Mar", "Jun", "Oct"][i % 4];
            format!("{:02}/{month}/26 12:00 AM", 1 + i % 28)
        })
        .collect();
    group.bench_function("tracker_import_parse_date_iso_1000", |b| {
        b.iter(|| {
            date_strings
                .iter()
                .filter_map(|s| tracker_import::parse_date(black_box(s)))
                .count()
        });
    });
    assert_eq!(
        jira.iter()
            .filter_map(|s| tracker_import::parse_date(s))
            .count(),
        n,
        "every Jira-style date parses"
    );
    group.bench_function("tracker_import_parse_date_jira_1000", |b| {
        b.iter(|| {
            jira.iter()
                .filter_map(|s| tracker_import::parse_date(black_box(s)))
                .count()
        });
    });
    let flex: Vec<String> = date_strings
        .iter()
        .enumerate()
        .map(|(i, s)| match i % 3 {
            0 => s.clone(),
            1 => s[..7].to_string(),
            _ => s[..4].to_string(),
        })
        .collect();
    group.bench_function("parse_flex_date_mixed_1000", |b| {
        b.iter(|| {
            flex.iter()
                .filter_map(|s| visibility::parse_flex_date(black_box(s), true))
                .count()
        });
    });
    let weeks: Vec<String> = (0..n)
        .map(|i| format!("{}-W{:02}", 2025 + i % 3, 1 + i % 52))
        .collect();
    group.bench_function("iso_week_start_and_days_1000", |b| {
        b.iter(|| {
            weeks
                .iter()
                .filter_map(|s| timesheets::week_start(black_box(s)))
                .map(|monday| timesheets::days_of(timesheets::monday_of(monday)).len())
                .sum::<usize>()
        });
    });
    let months: Vec<String> = date_strings.iter().map(|s| s[..7].to_string()).collect();
    group.bench_function("parse_month_add_months_1000", |b| {
        b.iter(|| {
            months
                .iter()
                .filter_map(|s| capacity_heatmap::parse_month(black_box(s)))
                .map(|first| capacity_heatmap::add_months(first, 13))
                .max()
        });
    });

    // Calendar arithmetic over the thousand dates.
    let calendar = plan_calendar();
    group.bench_function("business_day_index_1000", |b| {
        b.iter(|| {
            dates
                .iter()
                .filter(|d| calendar.is_working(**d))
                .filter_map(|d| calendar.index_of(black_box(*d)))
                .sum::<i64>()
        });
    });
    group.bench_function("report_schedule_next_monthly_1000", |b| {
        b.iter(|| {
            dates
                .iter()
                .filter_map(|d| report_schedules::next_after("monthly", dates[30], black_box(*d)))
                .count()
        });
    });
    group.bench_function("financial_year_of_1000", |b| {
        b.iter(|| {
            dates
                .iter()
                .map(|d| financial_year::year_of(black_box(*d), 4).is_some())
                .collect::<Vec<_>>()
        });
    });
    // Twenty absences of one to four weeks, expanded to days for a calendar.
    let absences: Vec<_> = (0..20_i64)
        .map(|k| {
            (
                day!(iso_from_start(k * 30)),
                day!(iso_from_start(k * 30 + 7 + (k % 4) * 7)),
            )
        })
        .collect();
    group.throughput(Throughput::Elements(1));
    group.bench_function("expand_absences_20", |b| {
        b.iter(|| calendars::expand_absences(black_box(&absences)));
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_schedule_risk,
    bench_cost_risk,
    bench_leveling,
    bench_schedule_health,
    bench_mspdi,
    bench_earned_value,
    bench_timestamps
);
criterion_main!(benches);

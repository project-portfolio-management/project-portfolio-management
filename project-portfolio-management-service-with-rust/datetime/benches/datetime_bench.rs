#![warn(clippy::pedantic)]

//! Criterion benchmarks for the datetime crate's own types: serde and parsing, and the calendar arithmetic under the
//! service's date code, one operation at a time.
//!
//! Run from the service directory with `cargo bench -p project-portfolio-management-datetime --bench
//! datetime_bench`. It builds this crate alone (no service, no domain crate).
//!
//! # Why the operations one by one
//!
//! The move from `chrono` to `jiff` made the service's JSON and parsing faster and its calendar arithmetic two to
//! three times slower (`BENCHMARKS.md`, "Date handling before and after"): the business-day index, an ISO week to
//! its Monday and seven days, a report schedule's `next_after`, absence expansion, month arithmetic. Those rows
//! time whole domain functions; this file times what they are made of (a weekday, a day step inside and across a
//! month, a date minus a date, a month step, an ISO week, a `Duration`), so a slow row can be attributed to an
//! operation and the operation worked on. Each operation is timed on **one** date (`…/1`) and over a loop of
//! **1,000** dates (`…/1000`, the same thousand dates as the domain crate's `timestamps` group). Where it helps to
//! tell the wrapper's cost from jiff's, the same step is timed on `jiff::civil::Date` directly (`jiff_…`).
//!
//! # The groups
//!
//! - **`timestamps`**: a thousand ISO dates and RFC 3339 instants through serde, and the `YYYY-MM-DD` reader the
//!   service's `plan_templates::full_date` is (`Date::parse_from_str(s.trim(), "%Y-%m-%d")`). These IDs were in the
//!   service's `benches/calculation_bench.rs` until 18 November 2026 and measure the same code.
//! - **`date_ops`**: one `Date` operation per ID.
//! - **`duration_ops`**: one `Duration` operation per ID.

use std::{collections::BTreeSet, hint::black_box};

use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use project_portfolio_management_datetime::{Date, Duration, Months, OffsetDateTime, Weekday};

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

/// An RFC 3339 instant with microseconds (PostgreSQL's precision) and a `+00:00` offset.
fn instant(i: i64) -> String {
    format!(
        "{}T{:02}:{:02}:{:02}.{:06}+00:00",
        iso_from_start(-(i % 400)),
        (i * 7) % 24,
        (i * 13) % 60,
        (i * 17) % 60,
        (i * 7919) % 1_000_000
    )
}

/// The service's `plan_templates::full_date`.
fn full_date(s: &str) -> Option<Date> {
    Date::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

/// The thousand dates every group uses: every third day from 2024-05-16 to 2032-07-30, so about one in thirty crosses
/// a month on a one-day step, and every one does on a thirty-day step.
fn thousand_date_strings() -> Vec<String> {
    (0..1_000_i64)
        .map(|i| iso_from_start(i * 3 - 900))
        .collect()
}

fn bench_timestamps(c: &mut Criterion) {
    let mut group = c.benchmark_group("timestamps");
    let n = 1_000_usize;
    let date_strings = thousand_date_strings();
    let instant_strings: Vec<String> = (0..1_000_i64).map(instant).collect();
    let dates_json = serde_json::to_string(&date_strings).expect("strings encode");
    let instants_json = serde_json::to_string(&instant_strings).expect("strings encode");
    let dates: Vec<Date> = serde_json::from_str(&dates_json).expect("ISO dates decode");
    let instants: Vec<OffsetDateTime> =
        serde_json::from_str(&instants_json).expect("RFC 3339 instants decode");
    group.throughput(Throughput::Elements(u64::try_from(n).unwrap_or(u64::MAX)));
    group.bench_function("decode_iso_dates_1000", |b| {
        b.iter(|| serde_json::from_str::<Vec<Date>>(black_box(&dates_json)).expect("decode"));
    });
    group.bench_function("encode_iso_dates_1000", |b| {
        b.iter(|| serde_json::to_string(black_box(&dates)).expect("encode"));
    });
    group.bench_function("decode_rfc3339_instants_1000", |b| {
        b.iter(|| {
            serde_json::from_str::<Vec<OffsetDateTime>>(black_box(&instants_json)).expect("decode")
        });
    });
    group.bench_function("encode_rfc3339_instants_1000", |b| {
        b.iter(|| serde_json::to_string(black_box(&instants)).expect("encode"));
    });
    group.bench_function("full_date_1000", |b| {
        b.iter(|| {
            date_strings
                .iter()
                .filter_map(|s| full_date(black_box(s)))
                .count()
        });
    });
    group.finish();
}

/// Time `f` on one input (`name/1`) and over every input (`name/1000`).
fn op<T: Copy, O>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    name: &str,
    inputs: &[T],
    f: impl Fn(T) -> O,
) {
    let one = inputs[inputs.len() / 2];
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new(name, 1), |b| b.iter(|| f(black_box(one))));
    group.throughput(Throughput::Elements(
        u64::try_from(inputs.len()).unwrap_or(u64::MAX),
    ));
    group.bench_function(BenchmarkId::new(name, inputs.len()), |b| {
        b.iter(|| {
            for x in inputs {
                black_box(f(black_box(*x)));
            }
        });
    });
}

fn bench_date_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("date_ops");
    let dates: Vec<Date> = thousand_date_strings()
        .iter()
        .map(|s| full_date(s).expect("a valid ISO date"))
        .collect();
    let jiff_dates: Vec<jiff::civil::Date> = dates.iter().map(|d| d.to_jiff()).collect();
    let epoch = Date::from_ymd_opt(1970, 1, 5).expect("a Monday");
    let jiff_epoch = epoch.to_jiff();
    // Ten holidays a year over the thousand dates' span, as a plan calendar holds them.
    let holidays: BTreeSet<Date> = dates.iter().step_by(36).copied().collect();
    let ymd: Vec<(i32, u32, u32)> = dates.iter().map(|d| (d.year(), d.month(), 1)).collect();
    let weeks: Vec<(i32, u32)> = (0..1_000_u32)
        .map(|i| (2025 + i32::try_from(i % 3).unwrap_or(0), 1 + i % 52))
        .collect();

    // What the business-day index and `monday_of` read first.
    op(&mut group, "weekday", &dates, |d| d.weekday());
    op(
        &mut group,
        "jiff_weekday",
        &jiff_dates,
        jiff::civil::Date::weekday,
    );
    // `WorkCalendar::is_working`'s holiday check, and the packed comparison under it.
    op(&mut group, "holiday_contains", &dates, |d| {
        holidays.contains(&d)
    });
    op(&mut group, "cmp", &dates, |d| d < epoch);
    // `index_of`'s and `expand_absences`' day count.
    op(&mut group, "date_minus_date", &dates, |d| {
        (d - epoch).num_days()
    });
    op(&mut group, "jiff_duration_since", &jiff_dates, |d| {
        d.duration_since(jiff_epoch).as_secs() / 86_400
    });
    // Day steps: `expand_absences`, `days_of`, `next_after("daily")`. One day crosses a month about one time in
    // thirty; thirty days always does.
    op(&mut group, "add_one_day", &dates, |d| d + Duration::days(1));
    op(&mut group, "add_seven_days", &dates, |d| {
        d + Duration::days(7)
    });
    op(&mut group, "add_thirty_days", &dates, |d| {
        d + Duration::days(30)
    });
    op(&mut group, "jiff_add_thirty_days", &jiff_dates, |d| {
        d.checked_add(jiff::SignedDuration::from_hours(30 * 24))
            .expect("in range")
    });
    op(&mut group, "succ_opt", &dates, |d| d.succ_opt());
    op(&mut group, "pred_opt", &dates, |d| d.pred_opt());
    // `timesheets::monday_of` and `days_of`: back to the Monday, then the seven days.
    op(&mut group, "monday_of", &dates, |d| {
        d - Duration::days(i64::from(d.weekday().num_days_from_monday()))
    });
    op(&mut group, "days_of_week", &dates, |d| {
        std::array::from_fn::<Date, 7, _>(|i| d + Duration::days(i64::try_from(i).unwrap_or(0)))
    });
    // ISO weeks: `timesheets::week_start` (`from_isoywd_opt`) and a date's week.
    op(&mut group, "from_isoywd_opt", &weeks, |(y, w)| {
        Date::from_isoywd_opt(y, w, Weekday::Mon)
    });
    op(&mut group, "iso_week", &dates, |d| d.iso_week());
    // Months: `capacity_heatmap::add_months`, and what `report_schedules::next_after("monthly")` does per month
    // (`from_ymd_opt`, and the last day of the month as the first of the next minus a day).
    op(&mut group, "add_months_13", &dates, |d| {
        d.checked_add_months(Months::new(13))
    });
    op(&mut group, "from_ymd_opt", &ymd, |(y, m, d)| {
        Date::from_ymd_opt(y, m, d)
    });
    op(&mut group, "last_day_of_month", &dates, |d| {
        let (y, m) = if d.month() == 12 {
            (d.year() + 1, 1)
        } else {
            (d.year(), d.month() + 1)
        };
        (Date::from_ymd_opt(y, m, 1).expect("a first of the month") - Duration::days(1)).day()
    });
    // `financial_year::year_of`'s step back to the last day of a year.
    op(&mut group, "ordinal", &dates, |d| d.ordinal());
    group.finish();
}

fn bench_duration_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("duration_ops");
    let counts: Vec<i64> = (0..1_000_i64).map(|i| i * 3 - 900).collect();
    let durations: Vec<Duration> = counts.iter().map(|n| Duration::days(*n)).collect();
    op(&mut group, "days", &counts, Duration::days);
    op(&mut group, "num_days", &durations, |d| d.num_days());
    op(&mut group, "add", &durations, |d| d + Duration::days(1));
    op(&mut group, "neg", &durations, |d| -d);
    op(&mut group, "hours_to_days", &counts, |n| {
        Duration::hours(n * 7).num_days()
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_timestamps,
    bench_date_ops,
    bench_duration_ops
);
criterion_main!(benches);

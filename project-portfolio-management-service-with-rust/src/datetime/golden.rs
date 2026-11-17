//! Golden forms: what the service's dates, instants and durations look like on the wire, in logs and in
//! exports, and how its date arithmetic behaves, pinned byte for byte.
//!
//! Written while the service still used `chrono` and `chrono-tz`, before the move to `jiff`, so that the
//! move could be checked to change nothing observable. Only the helpers at the top of this file name the
//! types; every expected string below is what `chrono` 0.4.45 and `chrono-tz` 0.10.4 produced. Change an
//! expected string only on purpose, with a spec edit.

use serde_json::json;

use crate::models::_entities::{Date, DateTimeWithTimeZone, key_result_check_ins};

// ---------------------------------------------------------------------------------------------------------
// The helpers: the only lines that name the date and time types.
// ---------------------------------------------------------------------------------------------------------

type Utc = chrono::DateTime<chrono::Utc>;

fn utc(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32, nanos: u32) -> Utc {
    use chrono::TimeZone as _;
    chrono::Utc
        .with_ymd_and_hms(y, mo, d, h, mi, s)
        .unwrap()
        .checked_add_signed(chrono::Duration::nanoseconds(i64::from(nanos)))
        .unwrap()
}

/// The column type (`TIMESTAMP WITH TIME ZONE`), as a row decodes it: offset zero.
fn column(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32, nanos: u32) -> DateTimeWithTimeZone {
    utc(y, mo, d, h, mi, s, nanos).fixed_offset()
}

fn date(y: i32, m: u32, d: u32) -> Date {
    Date::from_ymd_opt(y, m, d).unwrap()
}

fn utc_rfc3339(t: Utc) -> String {
    t.to_rfc3339()
}

fn column_rfc3339(t: DateTimeWithTimeZone) -> String {
    t.to_rfc3339()
}

fn utc_rfc3339_micros_z(t: Utc) -> String {
    t.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
}

fn utc_format(t: Utc, pattern: &str) -> String {
    t.format(pattern).to_string()
}

fn date_format(d: Date, pattern: &str) -> String {
    d.format(pattern).to_string()
}

fn utc_millis(t: Utc) -> i64 {
    t.timestamp_millis()
}

fn utc_micros(t: Utc) -> i64 {
    t.timestamp_micros()
}

fn utc_seconds(t: Utc) -> i64 {
    t.timestamp()
}

fn utc_from_millis(ms: i64) -> Option<Utc> {
    chrono::DateTime::from_timestamp_millis(ms)
}

fn utc_date(t: Utc) -> Date {
    t.date_naive()
}

/// A request body's instant: the JSON string decoded as the service's `DateTime<Utc>` decodes it, and
/// written back; or the decoder's message.
fn decode_utc(s: &str) -> Result<String, String> {
    serde_json::from_value::<Utc>(json!(s))
        .map(|t| {
            serde_json::to_value(t)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        })
        .map_err(|e| e.to_string())
}

/// As [`decode_utc`] for the column type, showing the offset it kept (its `Debug`).
fn decode_column(s: &str) -> Result<String, String> {
    serde_json::from_value::<DateTimeWithTimeZone>(json!(s))
        .map(|t| format!("{t:?}"))
        .map_err(|e| e.to_string())
}

/// The strict RFC 3339 parse (`?as_of=` and other query parameters), as an instant in UTC.
fn parse_rfc3339(s: &str) -> Result<String, String> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|t| format!("{:?}", t.to_utc()))
        .map_err(|e| e.to_string())
}

/// As [`parse_rfc3339`], keeping the offset the text gave.
fn parse_rfc3339_offset(s: &str) -> Result<String, String> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|t| format!("{t:?}"))
        .map_err(|e| e.to_string())
}

fn decode_date(s: &str) -> Result<String, String> {
    serde_json::from_value::<Date>(json!(s))
        .map(|d| d.to_string())
        .map_err(|e| e.to_string())
}

fn parse_date(s: &str, fmt: &str) -> Result<String, String> {
    Date::parse_from_str(s, fmt)
        .map(|d| d.to_string())
        .map_err(|e| e.to_string())
}

fn add_months(d: Date, n: i32) -> Option<Date> {
    if n >= 0 {
        d.checked_add_months(chrono::Months::new(n.unsigned_abs()))
    } else {
        d.checked_sub_months(chrono::Months::new(n.unsigned_abs()))
    }
}

fn add_days(d: Date, n: i64) -> Option<Date> {
    if n >= 0 {
        d.checked_add_days(chrono::Days::new(n.unsigned_abs()))
    } else {
        d.checked_sub_days(chrono::Days::new(n.unsigned_abs()))
    }
}

/// `date + Duration::seconds(secs)`: whole days only, toward zero.
fn date_plus_seconds(d: Date, secs: i64) -> Date {
    d + chrono::Duration::seconds(secs)
}

/// `(a - b).num_days()` for dates.
fn days_between(a: Date, b: Date) -> i64 {
    (a - b).num_days()
}

/// `(a - b)` for instants, as `(num_days, num_hours, num_minutes, num_seconds)`.
fn instant_difference(a: Utc, b: Utc) -> (i64, i64, i64, i64) {
    let d = a - b;
    (
        d.num_days(),
        d.num_hours(),
        d.num_minutes(),
        d.num_seconds(),
    )
}

fn utc_plus_days(t: Utc, n: i64) -> Utc {
    t + chrono::Duration::days(n)
}

fn weekday_from_monday(d: Date) -> u32 {
    use chrono::Datelike as _;
    d.weekday().num_days_from_monday()
}

fn weekday_debug(d: Date) -> String {
    use chrono::Datelike as _;
    format!("{:?}", d.weekday())
}

fn ordinal(d: Date) -> u32 {
    use chrono::Datelike as _;
    d.ordinal()
}

fn days_from_ce(d: Date) -> i32 {
    use chrono::Datelike as _;
    d.num_days_from_ce()
}

fn iso_week(d: Date) -> (i32, u32) {
    use chrono::Datelike as _;
    (d.iso_week().year(), d.iso_week().week())
}

fn from_iso_week_monday(year: i32, week: u32) -> Option<Date> {
    Date::from_isoywd_opt(year, week, chrono::Weekday::Mon)
}

fn next_day(d: Date) -> Option<Date> {
    d.succ_opt()
}

fn previous_day(d: Date) -> Option<Date> {
    d.pred_opt()
}

fn with_day(d: Date, day: u32) -> Option<Date> {
    use chrono::Datelike as _;
    d.with_day(day)
}

/// An instant shown in a zone, as the planner shows a slot (`starts_local`).
fn in_zone_rfc3339(t: Utc, zone: crate::tz::Tz) -> String {
    t.with_timezone(&zone).to_rfc3339()
}

fn zone_name(zone: crate::tz::Tz) -> String {
    zone.name().to_string()
}

// ---------------------------------------------------------------------------------------------------------
// The golden forms.
// ---------------------------------------------------------------------------------------------------------

/// JSON (`serde`) writes `Z` for UTC and groups the fraction in threes: none, milliseconds, microseconds
/// or nanoseconds, never trimmed further.
#[test]
fn instants_serialize_with_z_and_fractions_in_groups_of_three() {
    let cases = [
        (0, "2026-10-30T09:15:00Z"),
        (100_000_000, "2026-10-30T09:15:00.100Z"),
        (123_000_000, "2026-10-30T09:15:00.123Z"),
        (123_400_000, "2026-10-30T09:15:00.123400Z"),
        (123_456_000, "2026-10-30T09:15:00.123456Z"),
        (1_000, "2026-10-30T09:15:00.000001Z"),
        (123_456_789, "2026-10-30T09:15:00.123456789Z"),
        (5, "2026-10-30T09:15:00.000000005Z"),
    ];
    for (nanos, want) in cases {
        let t = utc(2026, 10, 30, 9, 15, 0, nanos);
        assert_eq!(serde_json::to_value(t).unwrap(), json!(want), "{nanos}");
        // The column type writes the same: an offset of zero is `Z` in JSON.
        let c = column(2026, 10, 30, 9, 15, 0, nanos);
        assert_eq!(serde_json::to_value(c).unwrap(), json!(want), "{nanos}");
    }
    // Years before 1000 keep four digits.
    assert_eq!(
        serde_json::to_value(utc(999, 1, 2, 3, 4, 5, 0)).unwrap(),
        json!("0999-01-02T03:04:05Z")
    );
}

/// `to_rfc3339` (feeds, the planner, exports) writes `+00:00`, never `Z`, with the same fraction groups.
#[test]
fn rfc3339_text_writes_plus_zero_offset() {
    assert_eq!(
        utc_rfc3339(utc(2026, 10, 30, 9, 15, 0, 0)),
        "2026-10-30T09:15:00+00:00"
    );
    assert_eq!(
        utc_rfc3339(utc(2026, 10, 30, 9, 15, 0, 123_400_000)),
        "2026-10-30T09:15:00.123400+00:00"
    );
    assert_eq!(
        column_rfc3339(column(2026, 10, 30, 9, 15, 0, 123_000_000)),
        "2026-10-30T09:15:00.123+00:00"
    );
    assert_eq!(
        utc_rfc3339_micros_z(utc(2026, 10, 30, 9, 15, 0, 0)),
        "2026-10-30T09:15:00.000000Z"
    );
    assert_eq!(
        utc_rfc3339_micros_z(utc(2026, 10, 30, 9, 15, 0, 123_456_789)),
        "2026-10-30T09:15:00.123456Z"
    );
}

/// `Display` (`{}` in messages and logs) and `Debug` (`{:?}`, `?field` in tracing).
#[test]
fn display_and_debug() {
    let t = utc(2026, 10, 30, 9, 15, 0, 123_400_000);
    assert_eq!(t.to_string(), "2026-10-30 09:15:00.123400 UTC");
    assert_eq!(format!("{t:?}"), "2026-10-30T09:15:00.123400Z");
    let c = column(2026, 10, 30, 9, 15, 0, 123_400_000);
    assert_eq!(c.to_string(), "2026-10-30 09:15:00.123400 +00:00");
    assert_eq!(format!("{c:?}"), "2026-10-30T09:15:00.123400+00:00");
    let whole = utc(2026, 10, 30, 9, 15, 0, 0);
    assert_eq!(whole.to_string(), "2026-10-30 09:15:00 UTC");
    let d = date(2026, 2, 3);
    assert_eq!(d.to_string(), "2026-02-03");
    assert_eq!(format!("{d:?}"), "2026-02-03");
    assert_eq!(serde_json::to_value(d).unwrap(), json!("2026-02-03"));
    assert_eq!(date(999, 1, 1).to_string(), "0999-01-01");
}

/// A row's JSON, as routes that answer with a row send it.
#[test]
fn a_row_serializes_as_before() {
    let row = key_result_check_ins::Model {
        created_at: column(2026, 10, 30, 9, 15, 0, 0),
        updated_at: column(2026, 10, 30, 9, 15, 0, 123_456_000),
        id: 7,
        pid: uuid::Uuid::nil(),
        key_result_pid: uuid::Uuid::nil(),
        observed_at: column(2026, 10, 30, 9, 15, 0, 120_000_000),
        value: 3,
        confidence: None,
        note: None,
        actor: None,
    };
    assert_eq!(
        serde_json::to_string(&row).unwrap(),
        r#"{"created_at":"2026-10-30T09:15:00Z","updated_at":"2026-10-30T09:15:00.123456Z","id":7,"pid":"00000000-0000-0000-0000-000000000000","key_result_pid":"00000000-0000-0000-0000-000000000000","observed_at":"2026-10-30T09:15:00.120Z","value":3,"confidence":null,"note":null,"actor":null}"#
    );
    let back: key_result_check_ins::Model =
        serde_json::from_str(&serde_json::to_string(&row).unwrap()).unwrap();
    assert_eq!(back, row);
}

/// What a request body may send as an instant: chrono's relaxed RFC 3339 (a space or `t` for `T`, a
/// missing colon in the offset, `UTC`, unpadded fields, spaces between parts, any number of fraction
/// digits). A leap second (`23:59:60`) is not pinned: chrono reads it and writes it back as `:60`, which
/// `jiff` cannot represent (what Postgres stores for it is not yet checked).
#[test]
fn request_instants_are_read_relaxed() {
    let ok = [
        ("2026-10-30T09:15:00Z", "2026-10-30T09:15:00Z"),
        ("2026-10-30t09:15:00z", "2026-10-30T09:15:00Z"),
        ("2026-10-30 09:15:00Z", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00+00:00", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00-00:00", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00+0000", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00+05:30", "2026-10-30T03:45:00Z"),
        ("2026-10-30T09:15:00UTC", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00 utc", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00 +01:00", "2026-10-30T08:15:00Z"),
        ("2026-10-30T09:15:00+01 :00", "2026-10-30T08:15:00Z"),
        ("2026-1-3T9:5:7Z", "2026-01-03T09:05:07Z"),
        (" 2026-10-30T09:15:00Z", "2026-10-30T09:15:00Z"),
        ("2026 - 10 - 30T09 : 15 : 00Z", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00Z ", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00.1Z", "2026-10-30T09:15:00.100Z"),
        ("2026-10-30T09:15:00.1234Z", "2026-10-30T09:15:00.123400Z"),
        (
            "2026-10-30T09:15:00.123456789123Z",
            "2026-10-30T09:15:00.123456789Z",
        ),
        ("+2026-10-30T09:15:00Z", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00−01:00", "2026-10-30T10:15:00Z"),
    ];
    let bad: Vec<_> = ok
        .iter()
        .filter(|(input, want)| decode_utc(input) != Ok((*want).to_string()))
        .map(|(input, _)| (*input, decode_utc(input)))
        .collect();
    assert!(bad.is_empty(), "{bad:#?}");
    let refused = [
        ("", "premature end of input"),
        ("2026-10-30", "premature end of input"),
        ("2026-10-30T09:15", "premature end of input"),
        ("2026-10-30T09:15:00", "premature end of input"),
        ("2026-10-30X09:15:00Z", "input contains invalid characters"),
        ("2026-13-30T09:15:00Z", "input is out of range"),
        ("2026-02-30T09:15:00Z", "input is out of range"),
        ("2026-10-30T24:00:00Z", "input is out of range"),
        ("2026-10-30T09:60:00Z", "input is out of range"),
        ("2026-10-30T09:15:00+24:00", "input is out of range"),
        ("2026-10-30T09:15:00+01:60", "input is out of range"),
        ("2026-10-30T09:15:00Zjunk", "trailing input"),
        ("2026-10-30T09:15:00-07", "premature end of input"),
        ("2026-10-30T09:15:00.Z", "input contains invalid characters"),
        ("20261030T091500Z", "input contains invalid characters"),
        ("x", "input contains invalid characters"),
    ];
    let bad: Vec<_> = refused
        .iter()
        .filter(|(input, want)| decode_utc(input) != Err((*want).to_string()))
        .map(|(input, _)| (*input, decode_utc(input)))
        .collect();
    assert!(bad.is_empty(), "{bad:#?}");
    // The column type keeps the offset it was given.
    assert_eq!(
        decode_column("2026-10-30T09:15:00+05:30"),
        Ok("2026-10-30T09:15:00+05:30".to_string())
    );
    assert_eq!(
        decode_column("2026-10-30T09:15:00Z"),
        Ok("2026-10-30T09:15:00+00:00".to_string())
    );
}

/// `?as_of=` and other query instants: the strict RFC 3339 parse (four-digit year, two-digit fields,
/// `T`, `t` or a space, a colon in the offset).
#[test]
fn query_instants_are_read_strictly() {
    let ok = [
        ("2026-10-30T09:15:00Z", "2026-10-30T09:15:00Z"),
        ("2026-10-30t09:15:00z", "2026-10-30T09:15:00Z"),
        ("2026-10-30 09:15:00Z", "2026-10-30T09:15:00Z"),
        ("2026-10-30T09:15:00+05:30", "2026-10-30T03:45:00Z"),
        ("2026-10-30T09:15:00.5-01:00", "2026-10-30T10:15:00.500Z"),
        (
            "2026-10-30T09:15:00.123456789999Z",
            "2026-10-30T09:15:00.123456789Z",
        ),
    ];
    let bad: Vec<_> = ok
        .iter()
        .filter(|(input, want)| parse_rfc3339(input) != Ok((*want).to_string()))
        .map(|(input, _)| (*input, parse_rfc3339(input)))
        .collect();
    assert!(bad.is_empty(), "{bad:#?}");
    assert_eq!(
        parse_rfc3339_offset("2026-10-30T09:15:00+05:30"),
        Ok("2026-10-30T09:15:00+05:30".to_string())
    );
    let refused = [
        ("", "premature end of input"),
        ("2026-10-30T09:15:0", "premature end of input"),
        ("2026-10-30T09:15:00", "premature end of input"),
        ("2026-1-30T09:15:00Z", "input contains invalid characters"),
        (
            "2026-10-30T09:15:00+0530",
            "input contains invalid characters",
        ),
        ("2026-10-30T09:15:00+05", "premature end of input"),
        ("2026-10-30T09:15:00 Z", "input contains invalid characters"),
        (
            "2026-10-30T09:15:00UTC",
            "input contains invalid characters",
        ),
        ("2026-10-30T09:15:00Z ", "trailing input"),
        ("2026-02-29T09:15:00Z", "input is out of range"),
        ("2026-10-30T09:15:00+24:00", "input is out of range"),
        ("+2026-10-30T09:15:00Z", "input contains invalid characters"),
        ("2026-10-30T09:15:00.Z", "input contains invalid characters"),
    ];
    let bad: Vec<_> = refused
        .iter()
        .filter(|(input, want)| parse_rfc3339(input) != Err((*want).to_string()))
        .map(|(input, _)| (*input, parse_rfc3339(input)))
        .collect();
    assert!(bad.is_empty(), "{bad:#?}");
}

/// A request body's date: chrono's `NaiveDate` reading (unpadded fields, spaces around the dashes, a
/// sign on the year); and `parse_from_str` with `%Y-%m-%d` and `%d/%m/%Y`, as the importers use it.
#[test]
fn dates_are_read_as_before() {
    let ok = [
        ("2026-10-30", "2026-10-30"),
        ("2026-1-3", "2026-01-03"),
        (" 2026-10-30", "2026-10-30"),
        ("2026 - 10 - 30 ", "2026-10-30"),
        ("+2026-10-30", "2026-10-30"),
    ];
    let bad: Vec<_> = ok
        .iter()
        .filter(|(input, want)| decode_date(input) != Ok((*want).to_string()))
        .map(|(input, _)| (*input, decode_date(input)))
        .collect();
    assert!(bad.is_empty(), "{bad:#?}");
    let refused = [
        ("", "premature end of input"),
        ("2026-10", "premature end of input"),
        ("2026-10-30T00:00:00", "trailing input"),
        ("2026-02-29", "input is out of range"),
        ("2026-13-01", "input is out of range"),
        ("2026/10/30", "input contains invalid characters"),
        ("02026-10-30", "input contains invalid characters"),
        ("20261030", "input contains invalid characters"),
    ];
    let bad: Vec<_> = refused
        .iter()
        .filter(|(input, want)| decode_date(input) != Err((*want).to_string()))
        .map(|(input, _)| (*input, decode_date(input)))
        .collect();
    assert!(bad.is_empty(), "{bad:#?}");

    assert_eq!(
        parse_date("2026-10-30", "%Y-%m-%d"),
        Ok("2026-10-30".to_string())
    );
    assert_eq!(
        parse_date("2026-1-3", "%Y-%m-%d"),
        Ok("2026-01-03".to_string())
    );
    assert_eq!(
        parse_date(" 2026- 1- 3", "%Y-%m-%d"),
        Ok("2026-01-03".to_string())
    );
    assert_eq!(
        parse_date("+2026-10-30", "%Y-%m-%d"),
        Ok("2026-10-30".to_string())
    );
    assert_eq!(
        parse_date("2026-10-30 ", "%Y-%m-%d"),
        Err("trailing input".to_string())
    );
    assert_eq!(
        parse_date("2026-10-30T00:00", "%Y-%m-%d"),
        Err("trailing input".to_string())
    );
    assert_eq!(
        parse_date("2026 -10-30", "%Y-%m-%d"),
        Err("input contains invalid characters".to_string())
    );
    assert_eq!(
        parse_date("2026-02-29", "%Y-%m-%d"),
        Err("input is out of range".to_string())
    );
    assert_eq!(
        parse_date("30/10/2026", "%d/%m/%Y"),
        Ok("2026-10-30".to_string())
    );
    assert_eq!(
        parse_date("3/1/2026", "%d/%m/%Y"),
        Ok("2026-01-03".to_string())
    );
    assert_eq!(
        parse_date("2026-10-30", "%d/%m/%Y"),
        Err("input contains invalid characters".to_string())
    );
    assert_eq!(
        parse_date("30-10-2026", "%d/%m/%Y"),
        Err("input contains invalid characters".to_string())
    );
}

/// Month arithmetic clamps to the month's last day (no overflow into the next month), in both
/// directions and across leap years.
#[test]
fn months_clamp_to_the_end_of_the_month() {
    assert_eq!(add_months(date(2024, 1, 31), 1), Some(date(2024, 2, 29)));
    assert_eq!(add_months(date(2023, 1, 31), 1), Some(date(2023, 2, 28)));
    assert_eq!(add_months(date(2024, 3, 31), -1), Some(date(2024, 2, 29)));
    assert_eq!(add_months(date(2024, 2, 29), 12), Some(date(2025, 2, 28)));
    assert_eq!(add_months(date(2024, 2, 29), 48), Some(date(2028, 2, 29)));
    assert_eq!(add_months(date(2026, 8, 31), 1), Some(date(2026, 9, 30)));
    assert_eq!(add_months(date(2026, 10, 30), 0), Some(date(2026, 10, 30)));
    assert_eq!(add_months(date(2026, 1, 15), -13), Some(date(2024, 12, 15)));
}

/// Day arithmetic, the leap day, and whole days from a duration (toward zero).
#[test]
fn days_and_durations_on_dates() {
    assert_eq!(add_days(date(2024, 2, 28), 1), Some(date(2024, 2, 29)));
    assert_eq!(add_days(date(2023, 2, 28), 1), Some(date(2023, 3, 1)));
    assert_eq!(add_days(date(2024, 3, 1), -1), Some(date(2024, 2, 29)));
    assert_eq!(add_days(date(2026, 12, 31), 1), Some(date(2027, 1, 1)));
    assert_eq!(add_days(date(2026, 1, 1), 365), Some(date(2027, 1, 1)));
    assert_eq!(
        date_plus_seconds(date(2026, 10, 30), 86_400),
        date(2026, 10, 31)
    );
    assert_eq!(
        date_plus_seconds(date(2026, 10, 30), -86_400),
        date(2026, 10, 29)
    );
    // A part day is dropped, toward zero, in either direction.
    assert_eq!(
        date_plus_seconds(date(2026, 10, 30), 36 * 3600),
        date(2026, 10, 31)
    );
    assert_eq!(
        date_plus_seconds(date(2026, 10, 30), -36 * 3600),
        date(2026, 10, 29)
    );
    assert_eq!(
        date_plus_seconds(date(2026, 10, 30), 23 * 3600),
        date(2026, 10, 30)
    );
    assert_eq!(days_between(date(2026, 3, 1), date(2026, 2, 1)), 28);
    assert_eq!(days_between(date(2024, 3, 1), date(2024, 2, 1)), 29);
    assert_eq!(days_between(date(2026, 2, 1), date(2026, 3, 1)), -28);
    assert_eq!(next_day(date(2024, 2, 28)), Some(date(2024, 2, 29)));
    assert_eq!(previous_day(date(2026, 1, 1)), Some(date(2025, 12, 31)));
    assert_eq!(with_day(date(2026, 2, 10), 1), Some(date(2026, 2, 1)));
    assert_eq!(with_day(date(2026, 2, 10), 29), None);
    assert_eq!(with_day(date(2024, 2, 10), 29), Some(date(2024, 2, 29)));
}

/// Instant differences truncate toward zero at each unit, negative ones included.
#[test]
fn instant_differences_truncate_toward_zero() {
    let a = utc(2026, 10, 30, 12, 0, 0, 0);
    let b = utc(2026, 10, 29, 0, 0, 30, 0); // 35h59m30s earlier
    assert_eq!(instant_difference(a, b), (1, 35, 2159, 129_570));
    assert_eq!(instant_difference(b, a), (-1, -35, -2159, -129_570));
    let c = utc(2026, 10, 30, 12, 0, 0, 500_000_000);
    assert_eq!(instant_difference(a, c), (0, 0, 0, 0));
    assert_eq!(utc_plus_days(a, -2), utc(2026, 10, 28, 12, 0, 0, 0));
    assert_eq!(utc_plus_days(a, 3), utc(2026, 11, 2, 12, 0, 0, 0));
}

/// Unix counts floor before 1970 (as chrono does), and `from_timestamp_millis` reads them back.
#[test]
fn unix_counts() {
    let t = utc(2026, 10, 30, 9, 15, 0, 123_456_789);
    assert_eq!(utc_seconds(t), 1_793_351_700);
    assert_eq!(utc_millis(t), 1_793_351_700_123);
    assert_eq!(utc_micros(t), 1_793_351_700_123_456);
    let before = utc(1969, 12, 31, 23, 59, 59, 999_500_000);
    assert_eq!(utc_seconds(before), -1);
    assert_eq!(utc_millis(before), -1);
    assert_eq!(utc_micros(before), -500);
    let before = utc(1969, 12, 31, 23, 59, 59, 999_999_500);
    assert_eq!(utc_millis(before), -1);
    assert_eq!(utc_micros(before), -1);
    assert_eq!(
        utc_from_millis(-1).map(|t| serde_json::to_value(t).unwrap()),
        Some(json!("1969-12-31T23:59:59.999Z"))
    );
    assert_eq!(
        utc_from_millis(1_793_351_700_123).map(|t| serde_json::to_value(t).unwrap()),
        Some(json!("2026-10-30T09:15:00.123Z"))
    );
    assert_eq!(
        utc_date(utc(2026, 10, 30, 23, 59, 59, 999_999_999)),
        date(2026, 10, 30)
    );
    assert_eq!(
        utc_date(utc(1969, 12, 31, 23, 59, 59, 0)),
        date(1969, 12, 31)
    );
}

/// The strftime patterns the service uses (feeds, exports, periods, messages).
#[test]
fn the_patterns_the_service_formats_with() {
    let t = utc(2026, 3, 4, 5, 6, 7, 890_000_000);
    assert_eq!(
        utc_format(t, "%Y-%m-%d %H:%M:%S UTC"),
        "2026-03-04 05:06:07 UTC"
    );
    assert_eq!(utc_format(t, "%Y-%m-%d"), "2026-03-04");
    assert_eq!(utc_format(t, "%Y-%m-%dT%H:00:00Z"), "2026-03-04T05:00:00Z");
    assert_eq!(utc_format(t, "%Y-%m"), "2026-03");
    assert_eq!(utc_format(t, "%Y%m%dT%H%M%SZ"), "20260304T050607Z");
    assert_eq!(utc_format(t, "%Y%m%dT%H0000Z"), "20260304T050000Z");
    assert_eq!(date_format(date(2026, 3, 4), "%Y-%m"), "2026-03");
    assert_eq!(date_format(date(2026, 3, 4), "%Y-%m-%d"), "2026-03-04");
    assert_eq!(date_format(date(2026, 3, 4), "%Y%m%d"), "20260304");
    assert_eq!(date_format(date(999, 3, 4), "%Y-%m"), "0999-03");
}

/// Weekdays, ordinals and ISO weeks.
#[test]
fn calendar_fields() {
    assert_eq!(weekday_from_monday(date(2026, 10, 26)), 0); // a Monday
    assert_eq!(weekday_from_monday(date(2026, 11, 1)), 6); // a Sunday
    assert_eq!(weekday_debug(date(2026, 10, 31)), "Sat");
    assert_eq!(ordinal(date(2024, 12, 31)), 366);
    assert_eq!(ordinal(date(2026, 12, 31)), 365);
    assert_eq!(days_from_ce(date(2026, 1, 1)), 739_617);
    assert_eq!(days_from_ce(date(1, 1, 1)), 1);
    assert_eq!(iso_week(date(2027, 1, 1)), (2026, 53));
    assert_eq!(iso_week(date(2024, 12, 30)), (2025, 1));
    assert_eq!(from_iso_week_monday(2026, 53), Some(date(2026, 12, 28)));
    assert_eq!(from_iso_week_monday(2025, 53), None);
    assert_eq!(from_iso_week_monday(2026, 1), Some(date(2025, 12, 29)));
}

/// Zones: names are matched exactly (no case folding), an alias keeps its own name, and an instant is
/// shown with the zone's offset at that instant, to the minute.
#[test]
fn zones_as_before() {
    use crate::tz::parse_zone;
    for bad in [
        "europe/london",
        "EUROPE/LONDON",
        "utc",
        "Utc",
        "America/New York",
        "GMT+1",
    ] {
        assert!(parse_zone(bad).is_err(), "{bad:?}");
    }
    for (name, kept) in [
        ("UTC", "UTC"),
        ("Etc/UTC", "Etc/UTC"),
        ("US/Pacific", "US/Pacific"),
        ("Asia/Calcutta", "Asia/Calcutta"),
        ("Europe/Kyiv", "Europe/Kyiv"),
        ("Etc/GMT+5", "Etc/GMT+5"),
        ("EST", "EST"),
        ("GMT", "GMT"),
    ] {
        assert_eq!(
            parse_zone(name).map(zone_name),
            Ok(kept.to_string()),
            "{name}"
        );
    }
    let t = utc(2026, 10, 2, 23, 30, 0, 0);
    assert_eq!(
        in_zone_rfc3339(t, parse_zone("Pacific/Auckland").unwrap()),
        "2026-10-03T12:30:00+13:00"
    );
    assert_eq!(
        in_zone_rfc3339(t, parse_zone("Asia/Kolkata").unwrap()),
        "2026-10-03T05:00:00+05:30"
    );
    assert_eq!(
        in_zone_rfc3339(t, parse_zone("America/Los_Angeles").unwrap()),
        "2026-10-02T16:30:00-07:00"
    );
    assert_eq!(
        in_zone_rfc3339(t, parse_zone("Etc/GMT+5").unwrap()),
        "2026-10-02T18:30:00-05:00"
    );
    assert_eq!(
        in_zone_rfc3339(t, parse_zone("UTC").unwrap()),
        "2026-10-02T23:30:00+00:00"
    );
    let early = utc(1900, 6, 1, 12, 0, 0, 0);
    assert_eq!(
        in_zone_rfc3339(early, parse_zone("Europe/Amsterdam").unwrap()),
        "1900-06-01T12:00:00+00:00"
    );
    assert_eq!(
        in_zone_rfc3339(early, parse_zone("Africa/Monrovia").unwrap()),
        "1900-06-01T11:16:52-00:43"
    );
}

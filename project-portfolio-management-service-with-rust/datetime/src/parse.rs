//! The readers: how the service has always read a date or an instant from text.
//!
//! A port of the parts of chrono 0.4's parser the service used, kept to the same acceptances, the same
//! order of checks and the same error kinds, because a request body's or a query parameter's error message
//! reaches the client:
//!
//! - `rfc3339_relaxed`: a request body's instant (serde) and `str::parse`. A space or `t` for `T`, spaces
//!   between any two parts, unpadded fields, a signed year of any width, any number of fraction digits (past
//!   nine ignored), `UTC` (any case) or `Z` for a zero offset, an offset with or without its colon, and the
//!   minus sign U+2212.
//! - `rfc3339_strict`: `parse_from_rfc3339` (query parameters): four-digit year, two-digit fields, `T`, `t` or
//!   one space, a colon in the offset.
//! - `date`: a request body's date (serde) and `str::parse`: `year-month-day` with unpadded fields, spaces
//!   around the dashes and a signed year.
//! - `strptime`: the handful of `%` patterns the service reads with (`%Y-%m-%d`, `%d/%m/%Y`, and the
//!   database's text form `%Y-%m-%d %H:%M:%S%.f%#z`).
//!
//! What cannot be kept: a leap second (`:60`) is read as `:59` and the same fraction (jiff has no leap
//! seconds, and neither has Postgres), and a year outside -9999..=9999, or an instant within a day of either
//! end of that range, is out of range (jiff's range).
//!
//! The scanning functions and the two RFC 3339 readers follow chrono 0.4.45's `format/parse.rs` and
//! `format/scan.rs` (copyright (c) 2014--2026 Kang Seonghoon and contributors, MIT OR Apache-2.0), adapted to
//! build jiff values.

use crate::error::{
    BAD_FORMAT, IMPOSSIBLE, INVALID, NOT_ENOUGH, OUT_OF_RANGE, ParseResult, TOO_LONG, TOO_SHORT,
};

/// The fields a read collects before they are checked together, as chrono's `Parsed` held them.
#[derive(Default, Debug)]
pub(crate) struct Parsed {
    pub(crate) year: Option<i32>,
    pub(crate) month: Option<u32>,
    pub(crate) day: Option<u32>,
    pub(crate) hour: Option<u32>,
    pub(crate) minute: Option<u32>,
    pub(crate) second: Option<u32>,
    pub(crate) nanosecond: Option<u32>,
    pub(crate) offset: Option<i32>,
}

fn set_if_consistent<T: PartialEq>(slot: &mut Option<T>, v: T) -> ParseResult<()> {
    match slot {
        Some(old) if *old != v => Err(IMPOSSIBLE),
        _ => {
            *slot = Some(v);
            Ok(())
        }
    }
}

impl Parsed {
    fn set_year(&mut self, v: i64) -> ParseResult<()> {
        set_if_consistent(&mut self.year, i32::try_from(v).map_err(|_| OUT_OF_RANGE)?)
    }

    fn set_month(&mut self, v: i64) -> ParseResult<()> {
        if !(1..=12).contains(&v) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(&mut self.month, u32::try_from(v).map_err(|_| OUT_OF_RANGE)?)
    }

    fn set_day(&mut self, v: i64) -> ParseResult<()> {
        if !(1..=31).contains(&v) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(&mut self.day, u32::try_from(v).map_err(|_| OUT_OF_RANGE)?)
    }

    fn set_hour(&mut self, v: i64) -> ParseResult<()> {
        if !(0..=23).contains(&v) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(&mut self.hour, u32::try_from(v).map_err(|_| OUT_OF_RANGE)?)
    }

    fn set_minute(&mut self, v: i64) -> ParseResult<()> {
        if !(0..=59).contains(&v) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.minute,
            u32::try_from(v).map_err(|_| OUT_OF_RANGE)?,
        )
    }

    fn set_second(&mut self, v: i64) -> ParseResult<()> {
        if !(0..=60).contains(&v) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.second,
            u32::try_from(v).map_err(|_| OUT_OF_RANGE)?,
        )
    }

    fn set_nanosecond(&mut self, v: i64) -> ParseResult<()> {
        if !(0..=999_999_999).contains(&v) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.nanosecond,
            u32::try_from(v).map_err(|_| OUT_OF_RANGE)?,
        )
    }

    fn set_offset(&mut self, v: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.offset,
            i32::try_from(v).map_err(|_| OUT_OF_RANGE)?,
        )
    }

    /// The date the fields name.
    pub(crate) fn to_date(&self) -> ParseResult<jiff::civil::Date> {
        let (Some(year), Some(month), Some(day)) = (self.year, self.month, self.day) else {
            return Err(NOT_ENOUGH);
        };
        civil_date(year, month, day).ok_or(OUT_OF_RANGE)
    }

    /// The time of day the fields name. A leap second (`:60`) is read as `:59` with the same fraction.
    fn to_time(&self) -> ParseResult<jiff::civil::Time> {
        let (Some(hour), Some(minute)) = (self.hour, self.minute) else {
            return Err(NOT_ENOUGH);
        };
        let second = match self.second.unwrap_or(0) {
            v @ 0..=59 => v,
            60 => 59,
            _ => return Err(OUT_OF_RANGE),
        };
        let nano = match self.nanosecond {
            Some(v @ 0..=999_999_999) if self.second.is_some() => v,
            Some(0..=999_999_999) => return Err(NOT_ENOUGH),
            Some(_) => return Err(OUT_OF_RANGE),
            None => 0,
        };
        let narrow = |v: u32| i8::try_from(v).map_err(|_| OUT_OF_RANGE);
        jiff::civil::Time::new(
            narrow(hour)?,
            narrow(minute)?,
            narrow(second)?,
            i32::try_from(nano).map_err(|_| OUT_OF_RANGE)?,
        )
        .map_err(|_| OUT_OF_RANGE)
    }

    /// The instant and the offset (in seconds) the fields name.
    pub(crate) fn to_instant(&self) -> ParseResult<(jiff::Timestamp, i32)> {
        let offset = self.offset.ok_or(NOT_ENOUGH)?;
        let date = self.to_date()?;
        let time = self.to_time()?;
        // An offset of a whole day or more is refused (as `FixedOffset::east_opt` did).
        if offset.unsigned_abs() >= 86_400 {
            return Err(OUT_OF_RANGE);
        }
        let off = jiff::tz::Offset::from_seconds(offset).map_err(|_| OUT_OF_RANGE)?;
        let ts = off
            .to_timestamp(date.to_datetime(time))
            .map_err(|_| OUT_OF_RANGE)?;
        Ok((ts, offset))
    }
}

/// A civil date from wide fields, or `None` outside jiff's range or the calendar.
pub(crate) fn civil_date(year: i32, month: u32, day: u32) -> Option<jiff::civil::Date> {
    jiff::civil::Date::new(
        i16::try_from(year).ok()?,
        i8::try_from(month).ok()?,
        i8::try_from(day).ok()?,
    )
    .ok()
}

/// Digits, at least `min` and at most `max`, as a number (chrono's `scan::number`).
fn number(s: &str, min: usize, max: usize) -> ParseResult<(&str, i64)> {
    let bytes = s.as_bytes();
    if bytes.len() < min {
        return Err(TOO_SHORT);
    }
    let mut n = 0i64;
    for (i, c) in bytes.iter().take(max).copied().enumerate() {
        if !c.is_ascii_digit() {
            if i < min {
                return Err(INVALID);
            }
            return Ok((&s[i..], n));
        }
        n = n
            .checked_mul(10)
            .and_then(|n| n.checked_add(i64::from(c - b'0')))
            .ok_or(OUT_OF_RANGE)?;
    }
    Ok((&s[core::cmp::min(max, bytes.len())..], n))
}

/// A fraction's digits (after the dot) as nanoseconds: up to nine read, any more skipped.
fn nanosecond(s: &str) -> ParseResult<(&str, u32)> {
    const SCALE: [u32; 10] = [
        0,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000,
        10_000,
        1_000,
        100,
        10,
        1,
    ];
    let origlen = s.len();
    let (s, v) = number(s, 1, 9)?;
    let v = u32::try_from(v).map_err(|_| OUT_OF_RANGE)?;
    let consumed = origlen - s.len();
    let v = v.checked_mul(SCALE[consumed]).ok_or(OUT_OF_RANGE)?;
    let s = s.trim_start_matches(|c: char| c.is_ascii_digit());
    Ok((s, v))
}

fn char(s: &str, c1: u8) -> ParseResult<&str> {
    match s.as_bytes().first() {
        Some(&c) if c == c1 => Ok(&s[1..]),
        Some(_) => Err(INVALID),
        None => Err(TOO_SHORT),
    }
}

#[allow(clippy::unnecessary_wraps)] // the same shape as `char`, so either can be passed
fn colon_or_space(s: &str) -> ParseResult<&str> {
    Ok(s.trim_start_matches(|c: char| c == ':' || c.is_whitespace()))
}

/// An offset: `Z` (if allowed), or a sign (`+`, `-`, or U+2212 if allowed), two hour digits, a separator
/// (`consume_colon`), and two minute digits (unless allowed to be missing). In seconds east of UTC.
fn timezone_offset(
    mut s: &str,
    consume_colon: fn(&str) -> ParseResult<&str>,
    allow_zulu: bool,
    allow_missing_minutes: bool,
    allow_tz_minus_sign: bool,
) -> ParseResult<(&str, i32)> {
    const fn digits(s: &str) -> ParseResult<(u8, u8)> {
        let b = s.as_bytes();
        if b.len() < 2 {
            Err(TOO_SHORT)
        } else {
            Ok((b[0], b[1]))
        }
    }
    if allow_zulu && let Some(b'Z' | b'z') = s.as_bytes().first() {
        return Ok((&s[1..], 0));
    }
    let negative = match s.chars().next() {
        Some('+') => {
            s = &s[1..];
            false
        }
        Some('-') => {
            s = &s[1..];
            true
        }
        Some('\u{2212}') => {
            if !allow_tz_minus_sign {
                return Err(INVALID);
            }
            s = &s['\u{2212}'.len_utf8()..];
            true
        }
        Some(_) => return Err(INVALID),
        None => return Err(TOO_SHORT),
    };
    let hours = match digits(s)? {
        (h1 @ b'0'..=b'9', h2 @ b'0'..=b'9') => i32::from((h1 - b'0') * 10 + (h2 - b'0')),
        _ => return Err(INVALID),
    };
    s = &s[2..];
    s = consume_colon(s)?;
    let minutes = if let Ok(ds) = digits(s) {
        match ds {
            (m1 @ b'0'..=b'5', m2 @ b'0'..=b'9') => i32::from((m1 - b'0') * 10 + (m2 - b'0')),
            (b'6'..=b'9', b'0'..=b'9') => return Err(OUT_OF_RANGE),
            _ => return Err(INVALID),
        }
    } else if allow_missing_minutes {
        0
    } else {
        return Err(TOO_SHORT);
    };
    s = match s.len() {
        len if len >= 2 => &s[2..],
        0 => s,
        _ => return Err(TOO_SHORT),
    };
    let seconds = hours * 3600 + minutes * 60;
    Ok((s, if negative { -seconds } else { seconds }))
}

/// One step of a pattern.
#[derive(Clone, Copy, Debug)]
enum Item<'a> {
    /// Exactly this text.
    Literal(&'a str),
    /// Any whitespace, or none.
    Space,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    /// `%.f`: an optional `.` and fraction.
    Nanosecond,
    /// `%#z`: an offset whose minutes may be missing, `Z` allowed.
    OffsetPermissive,
}

/// Run `items` over `s`, collecting fields into `parsed`; the rest of the text is returned.
fn run<'b>(parsed: &mut Parsed, mut s: &'b str, items: &[Item<'_>]) -> ParseResult<&'b str> {
    for item in items {
        match *item {
            Item::Literal(prefix) => {
                if s.len() < prefix.len() {
                    return Err(TOO_SHORT);
                }
                if !s.starts_with(prefix) {
                    return Err(INVALID);
                }
                s = &s[prefix.len()..];
            }
            Item::Space => s = s.trim_start(),
            Item::Year | Item::Month | Item::Day | Item::Hour | Item::Minute | Item::Second => {
                let (width, signed) = if matches!(item, Item::Year) {
                    (4, true)
                } else {
                    (2, false)
                };
                s = s.trim_start();
                let v = if signed && s.starts_with('-') {
                    let (rest, v) = number(&s[1..], 1, usize::MAX)?;
                    s = rest;
                    0i64.checked_sub(v).ok_or(OUT_OF_RANGE)?
                } else if signed && s.starts_with('+') {
                    let (rest, v) = number(&s[1..], 1, usize::MAX)?;
                    s = rest;
                    v
                } else {
                    let (rest, v) = number(s, 1, width)?;
                    s = rest;
                    v
                };
                match item {
                    Item::Year => parsed.set_year(v)?,
                    Item::Month => parsed.set_month(v)?,
                    Item::Day => parsed.set_day(v)?,
                    Item::Hour => parsed.set_hour(v)?,
                    Item::Minute => parsed.set_minute(v)?,
                    _ => parsed.set_second(v)?,
                }
            }
            Item::Nanosecond => {
                if let Some(rest) = s.strip_prefix('.') {
                    let (rest, nano) = nanosecond(rest)?;
                    s = rest;
                    parsed.set_nanosecond(i64::from(nano))?;
                }
            }
            Item::OffsetPermissive => {
                let (rest, offset) =
                    timezone_offset(s.trim_start(), colon_or_space, true, true, true)?;
                s = rest;
                parsed.set_offset(i64::from(offset))?;
            }
        }
    }
    Ok(s)
}

/// The relaxed RFC 3339 reading of an instant (chrono's `DateTime<FixedOffset>: FromStr`): the instant and the
/// offset it was given, in seconds.
pub(crate) fn rfc3339_relaxed(s: &str) -> ParseResult<(jiff::Timestamp, i32)> {
    const DATE_ITEMS: &[Item<'static>] = &[
        Item::Year,
        Item::Space,
        Item::Literal("-"),
        Item::Month,
        Item::Space,
        Item::Literal("-"),
        Item::Day,
    ];
    const TIME_ITEMS: &[Item<'static>] = &[
        Item::Hour,
        Item::Space,
        Item::Literal(":"),
        Item::Minute,
        Item::Space,
        Item::Literal(":"),
        Item::Second,
        Item::Nanosecond,
        Item::Space,
    ];
    let mut parsed = Parsed::default();
    let mut s = run(&mut parsed, s, DATE_ITEMS)?;
    s = match s.as_bytes().first() {
        Some(b't' | b'T' | b' ') => &s[1..],
        Some(_) => return Err(INVALID),
        None => return Err(TOO_SHORT),
    };
    s = run(&mut parsed, s, TIME_ITEMS)?;
    s = s.trim_start();
    let (s, offset) = if s.len() >= 3 && s.as_bytes()[..3].eq_ignore_ascii_case(b"UTC") {
        (&s[3..], 0)
    } else {
        timezone_offset(s, colon_or_space, true, false, true)?
    };
    parsed.set_offset(i64::from(offset))?;
    if !s.trim_start().is_empty() {
        return Err(TOO_LONG);
    }
    parsed.to_instant()
}

fn digit(bytes: &[u8], index: usize) -> ParseResult<u8> {
    if bytes[index].is_ascii_digit() {
        Ok(bytes[index] - b'0')
    } else {
        Err(INVALID)
    }
}

/// The strict RFC 3339 reading of an instant (chrono's `DateTime::parse_from_rfc3339`): the instant and the
/// offset it was given, in seconds.
pub(crate) fn rfc3339_strict(mut s: &str) -> ParseResult<(jiff::Timestamp, i32)> {
    let bytes = s.as_bytes();
    if bytes.len() < 19 {
        return Err(TOO_SHORT);
    }
    let year = u16::from(digit(bytes, 0)?) * 1000
        + u16::from(digit(bytes, 1)?) * 100
        + u16::from(digit(bytes, 2)?) * 10
        + u16::from(digit(bytes, 3)?);
    if bytes[4] != b'-' {
        return Err(INVALID);
    }
    let month = digit(bytes, 5)? * 10 + digit(bytes, 6)?;
    if bytes[7] != b'-' {
        return Err(INVALID);
    }
    let day = digit(bytes, 8)? * 10 + digit(bytes, 9)?;
    let date = civil_date(i32::from(year), u32::from(month), u32::from(day)).ok_or(OUT_OF_RANGE)?;
    if !matches!(bytes[10], b't' | b'T' | b' ') {
        return Err(INVALID);
    }
    let hour = digit(bytes, 11)? * 10 + digit(bytes, 12)?;
    if bytes[13] != b':' {
        return Err(INVALID);
    }
    let min = digit(bytes, 14)? * 10 + digit(bytes, 15)?;
    if bytes[16] != b':' {
        return Err(INVALID);
    }
    let sec = digit(bytes, 17)? * 10 + digit(bytes, 18)?;
    // A leap second is read as :59 (see the module docs).
    let sec = if sec == 60 { 59 } else { sec };
    let nano = if bytes.get(19) == Some(&b'.') {
        let (rest, nano) = nanosecond(&s[20..])?;
        s = rest;
        nano
    } else {
        s = &s[19..];
        0
    };
    let narrow = |v: u8| i8::try_from(v).map_err(|_| OUT_OF_RANGE);
    let time = jiff::civil::Time::new(
        narrow(hour)?,
        narrow(min)?,
        narrow(sec)?,
        i32::try_from(nano).map_err(|_| OUT_OF_RANGE)?,
    )
    .map_err(|_| OUT_OF_RANGE)?;
    let (s, offset) = timezone_offset(s, |s| char(s, b':'), true, false, true)?;
    if !s.is_empty() {
        return Err(TOO_LONG);
    }
    if offset.unsigned_abs() >= 86_400 {
        return Err(OUT_OF_RANGE);
    }
    let off = jiff::tz::Offset::from_seconds(offset).map_err(|_| OUT_OF_RANGE)?;
    let ts = off
        .to_timestamp(date.to_datetime(time))
        .map_err(|_| OUT_OF_RANGE)?;
    Ok((ts, offset))
}

/// A date (chrono's `NaiveDate: FromStr`).
pub(crate) fn date(s: &str) -> ParseResult<jiff::civil::Date> {
    const ITEMS: &[Item<'static>] = &[
        Item::Year,
        Item::Space,
        Item::Literal("-"),
        Item::Month,
        Item::Space,
        Item::Literal("-"),
        Item::Day,
        Item::Space,
    ];
    let mut parsed = Parsed::default();
    if !run(&mut parsed, s, ITEMS)?.is_empty() {
        return Err(TOO_LONG);
    }
    parsed.to_date()
}

/// A pattern's steps. Supported: `%Y %m %d %H %M %S %.f %#z %%`, literal text and whitespace (any run of
/// whitespace in the pattern reads any run, or none, in the text). Anything else is a bad format.
fn items(pattern: &str) -> ParseResult<Vec<Item<'_>>> {
    let mut out = Vec::new();
    let mut rest = pattern;
    while !rest.is_empty() {
        if let Some(spec) = rest.strip_prefix('%') {
            let (item, len) = if spec.starts_with(".f") {
                (Item::Nanosecond, 2)
            } else if spec.starts_with("#z") {
                (Item::OffsetPermissive, 2)
            } else {
                match spec.as_bytes().first() {
                    Some(b'Y') => (Item::Year, 1),
                    Some(b'm') => (Item::Month, 1),
                    Some(b'd') => (Item::Day, 1),
                    Some(b'H') => (Item::Hour, 1),
                    Some(b'M') => (Item::Minute, 1),
                    Some(b'S') => (Item::Second, 1),
                    Some(b'%') => (Item::Literal("%"), 1),
                    _ => return Err(BAD_FORMAT),
                }
            };
            out.push(item);
            rest = &spec[len..];
        } else if rest.starts_with(char::is_whitespace) {
            out.push(Item::Space);
            rest = rest.trim_start();
        } else {
            let end = rest
                .find(|c: char| c == '%' || c.is_whitespace())
                .unwrap_or(rest.len());
            out.push(Item::Literal(&rest[..end]));
            rest = &rest[end..];
        }
    }
    Ok(out)
}

/// Read `s` with a `%` pattern (chrono's `parse_from_str`): every field the pattern names, and nothing left over.
pub(crate) fn strptime(s: &str, pattern: &str) -> ParseResult<Parsed> {
    // The patterns the service reads with, without building their steps each time.
    const YMD: &[Item<'static>] = &[
        Item::Year,
        Item::Literal("-"),
        Item::Month,
        Item::Literal("-"),
        Item::Day,
    ];
    const DMY: &[Item<'static>] = &[
        Item::Day,
        Item::Literal("/"),
        Item::Month,
        Item::Literal("/"),
        Item::Year,
    ];
    let built;
    let items: &[Item<'_>] = match pattern {
        "%Y-%m-%d" => YMD,
        "%d/%m/%Y" => DMY,
        _ => {
            built = items(pattern)?;
            &built
        }
    };
    let mut parsed = Parsed::default();
    if !run(&mut parsed, s, items)?.is_empty() {
        return Err(TOO_LONG);
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fixed steps for the two common patterns are what the general builder makes of them.
    #[test]
    fn the_fixed_patterns_match_the_builder() {
        let general = |s: &str, pattern: &str| {
            let built = items(pattern)?;
            let mut parsed = Parsed::default();
            if !run(&mut parsed, s, &built)?.is_empty() {
                return Err(TOO_LONG);
            }
            parsed.to_date()
        };
        for s in [
            " 2026- 1- 3",
            "3/1/2026",
            "2026 -10-30",
            "2026-10-30 ",
            "30/10/2026",
            "",
            "2026-02-29",
        ] {
            for pattern in ["%Y-%m-%d", "%d/%m/%Y"] {
                let fixed = strptime(s, pattern).and_then(|p| p.to_date());
                assert_eq!(fixed, general(s, pattern), "{pattern} {s:?}");
            }
        }
    }

    #[test]
    fn field_validation_and_scanners_report_each_error_kind() {
        let mut parsed = Parsed::default();
        parsed.set_year(2026).unwrap();
        assert_eq!(parsed.set_year(2027), Err(IMPOSSIBLE));
        assert_eq!(parsed.set_year(i64::MAX), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_month(0), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_day(32), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_hour(24), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_minute(60), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_second(61), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_nanosecond(1_000_000_000), Err(OUT_OF_RANGE));
        assert_eq!(parsed.set_offset(i64::MAX), Err(OUT_OF_RANGE));
        assert_eq!(Parsed::default().to_date(), Err(NOT_ENOUGH));
        assert_eq!(Parsed::default().to_time(), Err(NOT_ENOUGH));

        let fraction_without_second = Parsed {
            hour: Some(1),
            minute: Some(2),
            nanosecond: Some(3),
            ..Parsed::default()
        };
        assert_eq!(fraction_without_second.to_time(), Err(NOT_ENOUGH));
        let invalid_fraction = Parsed {
            hour: Some(1),
            minute: Some(2),
            second: Some(3),
            nanosecond: Some(1_000_000_000),
            ..Parsed::default()
        };
        assert_eq!(invalid_fraction.to_time(), Err(OUT_OF_RANGE));
        let invalid_offset = Parsed {
            year: Some(2026),
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            offset: Some(86_400),
            ..Parsed::default()
        };
        assert_eq!(invalid_offset.to_instant(), Err(OUT_OF_RANGE));

        assert_eq!(number("", 1, 2), Err(TOO_SHORT));
        assert_eq!(number("x", 1, 2), Err(INVALID));
        assert_eq!(number("12x", 1, 2), Ok(("x", 12)));
        assert_eq!(
            number("999999999999999999999999", 1, usize::MAX),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(nanosecond("123456789999x"), Ok(("x", 123_456_789)));
        assert_eq!(char("", b':'), Err(TOO_SHORT));
        assert_eq!(char("x", b':'), Err(INVALID));
        assert_eq!(char(":x", b':'), Ok("x"));
    }

    #[test]
    fn offsets_patterns_and_rfc3339_readers_cover_permissive_and_refused_forms() {
        assert_eq!(
            timezone_offset("Zx", |s| char(s, b':'), true, false, false),
            Ok(("x", 0))
        );
        assert_eq!(
            timezone_offset("+01", |s| char(s, b':'), false, true, false),
            Err(TOO_SHORT)
        );
        assert_eq!(
            timezone_offset("+01", colon_or_space, false, true, false),
            Ok(("", 3600))
        );
        assert_eq!(
            timezone_offset("−0130", colon_or_space, false, false, true),
            Ok(("", -5400))
        );
        assert_eq!(
            timezone_offset("−0130", colon_or_space, false, false, false),
            Err(INVALID)
        );
        assert_eq!(
            timezone_offset("x0130", colon_or_space, false, false, false),
            Err(INVALID)
        );
        assert_eq!(
            timezone_offset("", colon_or_space, false, false, false),
            Err(TOO_SHORT)
        );
        assert_eq!(
            timezone_offset("+x130", colon_or_space, false, false, false),
            Err(INVALID)
        );
        assert_eq!(
            timezone_offset("+0160", colon_or_space, false, false, false),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            timezone_offset("+01x0", colon_or_space, false, false, false),
            Err(INVALID)
        );

        let parsed = strptime(
            "+2026 03 04 05 06 07.123456789999 +0130 %",
            "%Y %m %d %H %M %S%.f %#z %%",
        )
        .unwrap();
        assert_eq!(parsed.year, Some(2026));
        assert_eq!(parsed.nanosecond, Some(123_456_789));
        assert_eq!(parsed.offset, Some(5400));
        assert_eq!(strptime("2026", "%Y-%m").unwrap_err(), TOO_SHORT);
        assert_eq!(strptime("2026/01", "%Y-%m").unwrap_err(), INVALID);
        assert_eq!(strptime("2026", "%Q").unwrap_err(), BAD_FORMAT);
        assert_eq!(strptime("2026x", "%Y").unwrap_err(), TOO_LONG);

        assert!(rfc3339_relaxed("2026-1-2t3:4:5 UTC").is_ok());
        assert!(rfc3339_relaxed("2026-1-2 3:4:5 −0130").is_ok());
        assert_eq!(rfc3339_relaxed("2026-1-2x3:4:5Z"), Err(INVALID));
        assert_eq!(rfc3339_relaxed("2026-1-2"), Err(TOO_SHORT));
        assert_eq!(rfc3339_relaxed("2026-1-2T3:4:5Z trailing"), Err(TOO_LONG));

        assert!(rfc3339_strict("2026-01-02T03:04:05.123456789999+01:30").is_ok());
        for bad in [
            "2026-01-02",
            "x026-01-02T03:04:05Z",
            "2026/01-02T03:04:05Z",
            "2026-01/02T03:04:05Z",
            "2026-01-02X03:04:05Z",
            "2026-01-02T03-04:05Z",
            "2026-01-02T03:04-05Z",
        ] {
            assert!(rfc3339_strict(bad).is_err(), "{bad}");
        }
        assert_eq!(rfc3339_strict("2026-13-02T03:04:05Z"), Err(OUT_OF_RANGE));
        assert_eq!(
            rfc3339_strict("2026-01-02T03:04:05+01:60"),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(rfc3339_strict("2026-01-02T03:04:05Zx"), Err(TOO_LONG));
        assert_eq!(date("2026-01-02 trailing"), Err(TOO_LONG));
    }
}

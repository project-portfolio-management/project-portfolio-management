//! A calendar date with no zone (`date` columns, `YYYY-MM-DD` fields), over `jiff::civil::Date`.
//!
//! Written `YYYY-MM-DD` (four digits from year 0 to 9999, a sign and at least four digits outside), read by
//! [`crate::parse::date`]. Month arithmetic clamps the day to the end of a shorter month; day arithmetic and a
//! [`Duration`]'s whole days (toward zero) move it by days; the difference of two dates is whole days.

use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::str::FromStr;

use jiff::civil;

use crate::duration::{Days, Duration, Months};
use crate::error::{ParseError, ParseResult};
use crate::format::{self, Fields};
use crate::parse;
use crate::weekday::Weekday;

/// A calendar date.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date(pub(crate) civil::Date);

impl Date {
    /// The date as one integer that orders as the date does (year, then month, then day), so a comparison is
    /// one compare rather than three: dates are compared in sets and sorts in the calendar code's hot loops.
    #[inline]
    fn key(self) -> i32 {
        (i32::from(self.0.year()) << 9) | (i32::from(self.0.month()) << 5) | i32::from(self.0.day())
    }
}

impl PartialOrd for Date {
    #[inline]
    fn partial_cmp(&self, other: &Date) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    #[inline]
    fn cmp(&self, other: &Date) -> core::cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

/// The ISO 8601 week a date falls in: its week-numbering year and week (1 to 53).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsoWeek {
    year: i32,
    week: u32,
}

impl IsoWeek {
    /// The week-numbering year, which can differ from the calendar year near New Year.
    #[must_use]
    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// The week, 1 to 53.
    #[must_use]
    #[inline]
    pub const fn week(&self) -> u32 {
        self.week
    }

    /// The week, 0 to 52.
    #[must_use]
    #[inline]
    pub const fn week0(&self) -> u32 {
        self.week - 1
    }
}

impl fmt::Debug for IsoWeek {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (0..=9999).contains(&self.year) {
            write!(f, "{:04}-W{:02}", self.year, self.week)
        } else {
            write!(f, "{:+05}-W{:02}", self.year, self.week)
        }
    }
}

/// A time of day with no zone, to the nanosecond.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NaiveTime(pub(crate) civil::Time);

impl NaiveTime {
    /// Midnight.
    pub const MIN: NaiveTime = NaiveTime(civil::Time::MIN);

    /// `h:m:s`, or `None` outside 0..24, 0..60, 0..60.
    #[must_use]
    #[inline]
    pub fn from_hms_opt(hour: u32, minute: u32, second: u32) -> Option<NaiveTime> {
        civil::Time::new(
            i8::try_from(hour).ok()?,
            i8::try_from(minute).ok()?,
            i8::try_from(second).ok()?,
            0,
        )
        .ok()
        .map(NaiveTime)
    }
}

/// A date and time of day with no zone.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NaiveDateTime(pub(crate) civil::DateTime);

impl NaiveDateTime {
    /// The instant this names in UTC.
    ///
    /// Within a day or so of the ends of the supported range (-9999-01-01 and 9999-12-31), where the instant
    /// does not fit jiff's range, this is the earliest or latest instant instead, so a far boundary such as
    /// `?to=9999-12-31` still means "until the end" rather than failing.
    #[must_use]
    #[inline]
    pub fn and_utc(&self) -> crate::Timestamp {
        match jiff::tz::Offset::UTC.to_timestamp(self.0) {
            Ok(ts) => crate::Timestamp(ts),
            Err(_) if self.0.date().year() < 0 => crate::Timestamp::MIN_UTC,
            Err(_) => crate::Timestamp::MAX_UTC,
        }
    }

    /// The date part.
    #[must_use]
    #[inline]
    pub fn date(&self) -> Date {
        Date(self.0.date())
    }

    /// Moved by a duration, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_signed(self, rhs: Duration) -> Option<NaiveDateTime> {
        self.0.checked_add(rhs.to_jiff()).ok().map(NaiveDateTime)
    }
}

impl Default for NaiveDateTime {
    /// 1970-01-01T00:00:00.
    #[inline]
    fn default() -> NaiveDateTime {
        NaiveDateTime(civil::DateTime::constant(1970, 1, 1, 0, 0, 0, 0))
    }
}

impl Add<Duration> for NaiveDateTime {
    type Output = NaiveDateTime;
    #[inline]
    fn add(self, rhs: Duration) -> NaiveDateTime {
        self.checked_add_signed(rhs)
            .expect("`NaiveDateTime + Duration` overflowed")
    }
}

impl AddAssign<Duration> for NaiveDateTime {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Date {
    /// The earliest date: 1 January of year -9999.
    pub const MIN: Date = Date(civil::Date::MIN);
    /// The latest date: 31 December 9999.
    pub const MAX: Date = Date(civil::Date::MAX);

    /// The date, or `None` if there is no such day (or the year is outside -9999..=9999).
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)] // range-checked first
    #[inline]
    pub const fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Date> {
        if year < -9999 || year > 9999 || month < 1 || month > 12 || day < 1 {
            return None;
        }
        let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let last = match month {
            2 if leap => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        if day > last {
            return None;
        }
        Some(Date(civil::Date::constant(
            year as i16,
            month as i8,
            day as i8,
        )))
    }

    /// The date of the given ISO week's weekday, or `None` if the year has no such week.
    #[must_use]
    #[inline]
    pub fn from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option<Date> {
        // Week 1 is the week with 4 January in it; a year has 53 weeks when it starts on a Thursday, or on a
        // Wednesday in a leap year. Computed on day numbers rather than through jiff's `ISOWeekDate`, which is
        // several times slower here.
        let jan1 = Date::from_ymd_opt(year, 1, 1)?;
        let jan1_wd = jan1.weekday().num_days_from_monday();
        let weeks = if jan1_wd == 3 || (jan1_wd == 2 && jan1.leap_year()) {
            53
        } else {
            52
        };
        if week < 1 || week > weeks {
            return None;
        }
        let jan4 = jan1.epoch_day() + 3;
        let monday_week1 = jan4 - i64::from((jan1_wd + 3) % 7);
        Date::from_epoch_day(
            monday_week1 + i64::from(week - 1) * 7 + i64::from(weekday.num_days_from_monday()),
        )
    }

    /// The date `days` days after 1 January of year 1 counted as day 1, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn from_num_days_from_ce_opt(days: i32) -> Option<Date> {
        Date::CE.add_days(i64::from(days) - 1)
    }

    const CE: Date = Date(civil::Date::constant(1, 1, 1));

    /// The `jiff` date underneath.
    #[must_use]
    #[inline]
    pub const fn to_jiff(self) -> civil::Date {
        self.0
    }

    /// From a `jiff` date.
    #[must_use]
    #[inline]
    pub const fn from_jiff(d: civil::Date) -> Date {
        Date(d)
    }

    /// The year.
    #[must_use]
    #[inline]
    pub fn year(&self) -> i32 {
        i32::from(self.0.year())
    }

    /// The month, 1 to 12.
    #[must_use]
    #[allow(clippy::cast_sign_loss)] // 1..=12
    #[inline]
    pub fn month(&self) -> u32 {
        self.0.month() as u32
    }

    /// The month, 0 to 11.
    #[must_use]
    #[inline]
    pub fn month0(&self) -> u32 {
        self.month() - 1
    }

    /// The day of the month, 1 to 31.
    #[must_use]
    #[allow(clippy::cast_sign_loss)] // 1..=31
    #[inline]
    pub fn day(&self) -> u32 {
        self.0.day() as u32
    }

    /// The day of the month, 0 to 30.
    #[must_use]
    #[inline]
    pub fn day0(&self) -> u32 {
        self.day() - 1
    }

    /// The day of the year, 1 to 366.
    #[must_use]
    #[allow(clippy::cast_sign_loss)] // 1..=366
    #[inline]
    pub fn ordinal(&self) -> u32 {
        self.0.day_of_year() as u32
    }

    /// The day of the week.
    #[must_use]
    #[inline]
    pub fn weekday(&self) -> Weekday {
        // 1970-01-01 was a Thursday (3 days from Monday).
        Weekday::from_monday_offset((self.epoch_day() + 3).rem_euclid(7))
    }

    /// The ISO 8601 week.
    #[must_use]
    #[allow(clippy::cast_sign_loss)] // 1..=53
    #[inline]
    pub fn iso_week(&self) -> IsoWeek {
        let wd = self.0.iso_week_date();
        IsoWeek {
            year: i32::from(wd.year()),
            week: wd.week() as u32,
        }
    }

    /// Whether the year is a leap year.
    #[must_use]
    #[inline]
    pub fn leap_year(&self) -> bool {
        self.0.in_leap_year()
    }

    /// Days since 1 January of year 1, which is day 1.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)] // within ±3.7 million days
    #[inline]
    pub fn num_days_from_ce(&self) -> i32 {
        (self.days_since(Date::CE) + 1) as i32
    }

    /// This date on another day of the month, or `None` if the month has no such day.
    #[must_use]
    #[inline]
    pub fn with_day(&self, day: u32) -> Option<Date> {
        Date::from_ymd_opt(self.year(), self.month(), day)
    }

    /// This date in another month, or `None` if that month has no such day.
    #[must_use]
    #[inline]
    pub fn with_month(&self, month: u32) -> Option<Date> {
        Date::from_ymd_opt(self.year(), month, self.day())
    }

    /// This date in another year, or `None` (29 February in a year that is not a leap year).
    #[must_use]
    #[inline]
    pub fn with_year(&self, year: i32) -> Option<Date> {
        Date::from_ymd_opt(year, self.month(), self.day())
    }

    /// The next day, or `None` after the latest date.
    #[must_use]
    #[inline]
    pub fn succ_opt(&self) -> Option<Date> {
        let (y, m, d) = (self.year(), self.month(), self.day());
        Date::from_ymd_opt(y, m, d + 1)
            .or_else(|| Date::from_ymd_opt(y, m + 1, 1))
            .or_else(|| Date::from_ymd_opt(y + 1, 1, 1))
    }

    /// The day before, or `None` before the earliest date.
    #[must_use]
    #[inline]
    pub fn pred_opt(&self) -> Option<Date> {
        if self.day() > 1 {
            return Date::from_ymd_opt(self.year(), self.month(), self.day() - 1);
        }
        self.add_days(-1)
    }

    /// Days since 1970-01-01 (Howard Hinnant's `days_from_civil`, on plain integers: the calendar arithmetic
    /// below is in hot loops, and this is cheaper than going through jiff's checked arithmetic).
    #[inline]
    fn epoch_day(self) -> i64 {
        let (y, m, d) = (
            i64::from(self.0.year()),
            i64::from(self.0.month()),
            i64::from(self.0.day()),
        );
        let y = if m <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let mp = (m + 9) % 12;
        let doy = (153 * mp + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// The date `z` days after 1970-01-01 (Hinnant's `civil_from_days`), or `None` outside -9999..=9999.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // 1..=12 and 1..=31 by construction
    #[inline]
    fn from_epoch_day(z: i64) -> Option<Date> {
        let z = z + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = yoe + era * 400 + i64::from(m <= 2);
        Date::from_ymd_opt(i32::try_from(y).ok()?, m as u32, d as u32)
    }

    /// Whole days from `other` to `self` (negative if `self` is earlier).
    #[inline]
    fn days_since(self, other: Date) -> i64 {
        self.epoch_day() - other.epoch_day()
    }

    /// `n` days later (earlier if negative), or `None` out of range.
    #[inline]
    fn add_days(self, n: i64) -> Option<Date> {
        // Within the same month (the common step of a day or a week), no day-number round trip.
        let day = i64::from(self.0.day()) + n;
        if day >= 1 && day <= i64::from(self.0.days_in_month()) {
            #[allow(clippy::cast_possible_truncation)] // 1..=31
            return Some(Date(civil::Date::constant(
                self.0.year(),
                self.0.month(),
                day as i8,
            )));
        }
        Date::from_epoch_day(self.epoch_day().checked_add(n)?)
    }

    /// `n` months later (earlier if negative), the day clamped to the end of a shorter month.
    #[inline]
    fn add_months(self, n: i64) -> Option<Date> {
        let months = i64::from(self.year()) * 12 + i64::from(self.month0()) + n;
        let year = i32::try_from(months.div_euclid(12)).ok()?;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)] // 0..12
        let month = months.rem_euclid(12) as u32 + 1;
        let first = Date::from_ymd_opt(year, month, 1)?;
        #[allow(clippy::cast_sign_loss)] // 28..=31
        let last = first.0.days_in_month() as u32;
        Date::from_ymd_opt(year, month, self.day().min(last))
    }

    /// `months` later, the day clamped to the end of a shorter month; `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_months(self, months: Months) -> Option<Date> {
        if months.0 > i32::MAX as u32 {
            return None;
        }
        self.add_months(i64::from(months.0))
    }

    /// `months` earlier, the day clamped to the end of a shorter month; `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_sub_months(self, months: Months) -> Option<Date> {
        if months.0 > i32::MAX as u32 {
            return None;
        }
        self.add_months(-i64::from(months.0))
    }

    /// `days` later, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_days(self, days: Days) -> Option<Date> {
        self.add_days(i64::try_from(days.0).ok()?)
    }

    /// `days` earlier, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_sub_days(self, days: Days) -> Option<Date> {
        self.add_days(-i64::try_from(days.0).ok()?)
    }

    /// Moved by the duration's whole days (toward zero), or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_signed(self, rhs: Duration) -> Option<Date> {
        self.add_days(rhs.num_days())
    }

    /// Moved back by the duration's whole days (toward zero), or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_sub_signed(self, rhs: Duration) -> Option<Date> {
        self.add_days(rhs.num_days().checked_neg()?)
    }

    /// The whole days from `rhs` to `self`, as a duration.
    #[must_use]
    #[inline]
    pub fn signed_duration_since(self, rhs: Date) -> Duration {
        Duration::days(self.days_since(rhs))
    }

    /// Midnight-based date and time, or `None` if the time of day is out of range.
    #[must_use]
    #[inline]
    pub fn and_hms_opt(&self, hour: u32, minute: u32, second: u32) -> Option<NaiveDateTime> {
        NaiveTime::from_hms_opt(hour, minute, second).map(|t| self.and_time(t))
    }

    /// The date at a time of day.
    #[must_use]
    #[inline]
    pub fn and_time(&self, time: NaiveTime) -> NaiveDateTime {
        NaiveDateTime(self.0.to_datetime(time.0))
    }

    /// Written with a `%` pattern (`%Y`, `%m`, `%d`; see [`crate::format`]).
    #[must_use]
    #[inline]
    pub fn format<'a>(&self, pattern: &'a str) -> format::Formatted<'a> {
        format::Formatted::new(Fields::date(self.0), pattern)
    }

    /// Read with a `%` pattern naming at least the year, month and day (`%Y-%m-%d`, `%d/%m/%Y`).
    ///
    /// # Errors
    ///
    /// The text does not match the pattern, names no such day, or the pattern is not supported.
    #[inline]
    pub fn parse_from_str(s: &str, pattern: &str) -> ParseResult<Date> {
        parse::strptime(s, pattern)?.to_date().map(Date)
    }
}

impl Default for Date {
    /// 1 January 1970.
    #[inline]
    fn default() -> Date {
        Date(civil::Date::constant(1970, 1, 1))
    }
}

impl From<civil::Date> for Date {
    #[inline]
    fn from(d: civil::Date) -> Date {
        Date(d)
    }
}

impl From<Date> for civil::Date {
    #[inline]
    fn from(d: Date) -> civil::Date {
        d.0
    }
}

/// `YYYY-MM-DD`; a year outside 0..=9999 takes a sign (`-0001-01-01`).
pub(crate) fn write_date(f: &mut (impl fmt::Write + ?Sized), d: civil::Date) -> fmt::Result {
    let mut b = crate::format::Buf::new();
    b.date(d);
    f.write_str(b.as_str())
}

impl fmt::Debug for Date {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_date(f, self.0)
    }
}

impl fmt::Display for Date {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_date(f, self.0)
    }
}

impl FromStr for Date {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> ParseResult<Date> {
        parse::date(s).map(Date)
    }
}

impl Add<Duration> for Date {
    type Output = Date;
    #[inline]
    fn add(self, rhs: Duration) -> Date {
        self.checked_add_signed(rhs)
            .expect("`Date + Duration` overflowed")
    }
}

impl Sub<Duration> for Date {
    type Output = Date;
    #[inline]
    fn sub(self, rhs: Duration) -> Date {
        self.checked_sub_signed(rhs)
            .expect("`Date - Duration` overflowed")
    }
}

impl AddAssign<Duration> for Date {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl SubAssign<Duration> for Date {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Add<Days> for Date {
    type Output = Date;
    #[inline]
    fn add(self, days: Days) -> Date {
        self.checked_add_days(days)
            .expect("`Date + Days` out of range")
    }
}

impl Sub<Days> for Date {
    type Output = Date;
    #[inline]
    fn sub(self, days: Days) -> Date {
        self.checked_sub_days(days)
            .expect("`Date - Days` out of range")
    }
}

impl Add<Months> for Date {
    type Output = Date;
    #[inline]
    fn add(self, months: Months) -> Date {
        self.checked_add_months(months)
            .expect("`Date + Months` out of range")
    }
}

impl Sub<Months> for Date {
    type Output = Date;
    #[inline]
    fn sub(self, months: Months) -> Date {
        self.checked_sub_months(months)
            .expect("`Date - Months` out of range")
    }
}

impl Sub<Date> for Date {
    type Output = Duration;
    #[inline]
    fn sub(self, rhs: Date) -> Duration {
        self.signed_duration_since(rhs)
    }
}

impl serde::Serialize for Date {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Date;
            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a formatted date string")
            }
            #[inline]
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Date, E> {
                value.parse().map_err(E::custom)
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> Date {
        Date::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn written_with_four_digit_years_and_a_sign_outside() {
        assert_eq!(d(2026, 2, 3).to_string(), "2026-02-03");
        assert_eq!(d(0, 1, 1).to_string(), "0000-01-01");
        assert_eq!(d(-1, 12, 31).to_string(), "-0001-12-31");
        assert_eq!(
            serde_json::to_string(&d(999, 1, 1)).unwrap(),
            "\"0999-01-01\""
        );
    }

    #[test]
    fn a_wrong_json_type_names_what_was_expected() {
        let e = serde_json::from_str::<Date>("5").unwrap_err().to_string();
        assert_eq!(
            e,
            "invalid type: integer `5`, expected a formatted date string at line 1 column 1"
        );
    }

    #[test]
    fn months_clamp_and_days_move() {
        assert_eq!(
            d(2024, 1, 31).checked_add_months(Months::new(1)),
            Some(d(2024, 2, 29))
        );
        assert_eq!(
            d(2024, 3, 31).checked_sub_months(Months::new(1)),
            Some(d(2024, 2, 29))
        );
        assert_eq!(d(9999, 12, 1).checked_add_months(Months::new(1)), None);
        assert_eq!(
            d(2026, 1, 1).checked_add_days(Days::new(365)),
            Some(d(2027, 1, 1))
        );
        assert_eq!(d(9999, 12, 31).succ_opt(), None);
        assert_eq!(d(2026, 3, 1) - d(2026, 2, 1), Duration::days(28));
        assert_eq!(d(2026, 10, 30) + Duration::hours(-36), d(2026, 10, 29));
        assert_eq!(Date::from_num_days_from_ce_opt(1), Some(d(1, 1, 1)));
        assert_eq!(
            Date::from_num_days_from_ce_opt(3_652_059),
            Some(d(9999, 12, 31))
        );
        assert_eq!(Date::from_num_days_from_ce_opt(3_652_060), None);
        let span = (d(9999, 12, 31) - d(-9999, 1, 1)).num_days();
        assert!(span > 7_000_000);
        assert_eq!(
            d(-9999, 1, 1).checked_add_days(Days::new(span.unsigned_abs())),
            Some(d(9999, 12, 31))
        );
        assert_eq!(
            d(9999, 12, 31).checked_sub_days(Days::new(span.unsigned_abs())),
            Some(d(-9999, 1, 1))
        );
        assert_eq!(format!("{:?}", d(769, 4, 8).iso_week()), "0769-W15");
        // The packed order is the calendar's, negative years included.
        let mut all = vec![
            d(2026, 1, 31),
            d(-1, 12, 31),
            d(0, 1, 1),
            d(-9999, 1, 1),
            d(2026, 2, 1),
            d(9999, 12, 31),
        ];
        all.sort();
        assert_eq!(
            all,
            vec![
                d(-9999, 1, 1),
                d(-1, 12, 31),
                d(0, 1, 1),
                d(2026, 1, 31),
                d(2026, 2, 1),
                d(9999, 12, 31)
            ]
        );
        assert!(d(-2, 12, 31) < d(-1, 1, 1));
        // Past jiff's instant range, the far ends saturate rather than fail.
        let end = d(9999, 12, 31).and_hms_opt(23, 59, 59).unwrap().and_utc();
        assert_eq!(end, crate::Timestamp::MAX_UTC);
        let start = d(-9999, 1, 1).and_hms_opt(0, 0, 0).unwrap().and_utc();
        assert_eq!(start, crate::Timestamp::MIN_UTC);
        assert_eq!(d(2026, 1, 1).num_days_from_ce(), 739_617);
        assert_eq!(format!("{:?}", d(2027, 1, 1).iso_week()), "2026-W53");
    }
}

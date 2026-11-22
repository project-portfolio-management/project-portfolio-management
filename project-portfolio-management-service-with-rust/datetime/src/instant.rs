//! Instants: [`Timestamp`] (an instant in UTC, what the service computes with and stores) and
//! [`OffsetDateTime`] (an instant with the fixed offset it was given or shown at; a `timestamptz` column's
//! type, whose offset is zero once read back from the database).
//!
//! The two are separate types because they are written differently where it shows: `Display` is
//! `2026-10-30 09:15:00 UTC` for a `Timestamp` and `2026-10-30 09:15:00 +00:00` for an `OffsetDateTime`, and
//! `Debug` is `…Z` and `…+00:00`. JSON writes `Z` for a zero offset for both, `to_rfc3339` writes `+00:00` for
//! both. Equality, order and hashing compare the instant only, across both types, so `09:15+05:30` equals
//! `03:45Z`.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Sub;
use core::str::FromStr;

use jiff::tz::Offset;

use crate::date::{Date, NaiveDateTime};
use crate::duration::{Days, Duration, Months};
use crate::error::{ParseError, ParseResult};
use crate::format::{self, Fields, SecondsFormat};
use crate::parse;
use crate::weekday::Weekday;

const NANOS_PER_SEC: i128 = 1_000_000_000;

/// An instant, in UTC.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub(crate) jiff::Timestamp);

/// An instant with a fixed offset from UTC (in seconds, less than a day either way).
#[derive(Clone, Copy)]
pub struct OffsetDateTime {
    ts: jiff::Timestamp,
    offset: i32,
}

fn offset_of(seconds: i32) -> Offset {
    // Every offset this crate holds is less than a day, inside jiff's range.
    Offset::from_seconds(seconds).unwrap_or(Offset::UTC)
}

fn from_nanos(n: i128) -> Option<jiff::Timestamp> {
    jiff::Timestamp::from_nanosecond(n).ok()
}

/// Seconds rounded down (so before 1970 a part second counts as the second before).
#[allow(clippy::cast_possible_truncation)] // within jiff's range
fn floor_div(ts: jiff::Timestamp, unit: i128) -> i64 {
    ts.as_nanosecond().div_euclid(unit) as i64
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)] // 0..1e9
fn subsec(ts: jiff::Timestamp) -> u32 {
    ts.as_nanosecond().rem_euclid(NANOS_PER_SEC) as u32
}

macro_rules! instant_methods {
    () => {
        /// Whole seconds since 1970-01-01T00:00:00Z, rounded down.
        #[must_use]
        #[inline]
        pub fn timestamp(&self) -> i64 {
            floor_div(self.instant(), NANOS_PER_SEC)
        }

        /// Whole milliseconds since 1970-01-01T00:00:00Z, rounded down.
        #[must_use]
        #[inline]
        pub fn timestamp_millis(&self) -> i64 {
            floor_div(self.instant(), 1_000_000)
        }

        /// Whole microseconds since 1970-01-01T00:00:00Z, rounded down.
        #[must_use]
        #[inline]
        pub fn timestamp_micros(&self) -> i64 {
            floor_div(self.instant(), 1000)
        }

        /// Nanoseconds since 1970-01-01T00:00:00Z, or `None` outside `i64` (before 1677 or after 2262).
        #[must_use]
        #[inline]
        pub fn timestamp_nanos_opt(&self) -> Option<i64> {
            i64::try_from(self.instant().as_nanosecond()).ok()
        }

        /// The milliseconds past the second (0 to 999).
        #[must_use]
        #[inline]
        pub fn timestamp_subsec_millis(&self) -> u32 {
            subsec(self.instant()) / 1_000_000
        }

        /// The microseconds past the second (0 to 999 999).
        #[must_use]
        #[inline]
        pub fn timestamp_subsec_micros(&self) -> u32 {
            subsec(self.instant()) / 1000
        }

        /// The nanoseconds past the second (0 to 999 999 999).
        #[must_use]
        #[inline]
        pub fn timestamp_subsec_nanos(&self) -> u32 {
            subsec(self.instant())
        }

        /// The civil date and time in UTC.
        #[must_use]
        #[inline]
        pub fn naive_utc(&self) -> NaiveDateTime {
            NaiveDateTime(Offset::UTC.to_datetime(self.instant()))
        }

        /// The civil date and time at this value's own offset (UTC for a [`Timestamp`]).
        #[must_use]
        #[inline]
        pub fn naive_local(&self) -> NaiveDateTime {
            NaiveDateTime(self.local())
        }

        /// The calendar date at this value's own offset (UTC for a [`Timestamp`]).
        #[must_use]
        #[inline]
        pub fn date_naive(&self) -> Date {
            Date(self.local().date())
        }

        /// The year at this value's own offset.
        #[must_use]
        #[inline]
        pub fn year(&self) -> i32 {
            i32::from(self.local().year())
        }

        /// The month (1 to 12) at this value's own offset.
        #[must_use]
        #[allow(clippy::cast_sign_loss)]
        #[inline]
        pub fn month(&self) -> u32 {
            self.local().month() as u32
        }

        /// The day of the month at this value's own offset.
        #[must_use]
        #[allow(clippy::cast_sign_loss)]
        #[inline]
        pub fn day(&self) -> u32 {
            self.local().day() as u32
        }

        /// The day of the week at this value's own offset.
        #[must_use]
        #[inline]
        pub fn weekday(&self) -> Weekday {
            self.date_naive().weekday()
        }

        /// The day of the year at this value's own offset.
        #[must_use]
        #[inline]
        pub fn ordinal(&self) -> u32 {
            self.date_naive().ordinal()
        }

        /// The hour (0 to 23) at this value's own offset.
        #[must_use]
        #[allow(clippy::cast_sign_loss)]
        #[inline]
        pub fn hour(&self) -> u32 {
            self.local().hour() as u32
        }

        /// The minute at this value's own offset.
        #[must_use]
        #[allow(clippy::cast_sign_loss)]
        #[inline]
        pub fn minute(&self) -> u32 {
            self.local().minute() as u32
        }

        /// The second.
        #[must_use]
        #[allow(clippy::cast_sign_loss)]
        #[inline]
        pub fn second(&self) -> u32 {
            self.local().second() as u32
        }

        /// The nanoseconds past the second.
        #[must_use]
        #[allow(clippy::cast_sign_loss)]
        #[inline]
        pub fn nanosecond(&self) -> u32 {
            self.local().subsec_nanosecond() as u32
        }

        /// RFC 3339 with the offset written out (`+00:00`, never `Z`) and the fraction in groups of three.
        #[must_use]
        #[inline]
        pub fn to_rfc3339(&self) -> String {
            self.to_rfc3339_opts(SecondsFormat::AutoSi, false)
        }

        /// RFC 3339 with a chosen fraction, and `Z` for a zero offset if `use_z`.
        #[must_use]
        #[inline]
        pub fn to_rfc3339_opts(&self, secform: SecondsFormat, use_z: bool) -> String {
            let mut s = String::with_capacity(32);
            // Writing to a `String` cannot fail.
            let _ =
                format::write_rfc3339(&mut s, self.local(), self.offset_seconds(), secform, use_z);
            s
        }

        /// Written with a `%` pattern (`%Y %m %d %H %M %S %%`) at this value's own offset.
        #[must_use]
        #[inline]
        pub fn format<'a>(&self, pattern: &'a str) -> format::Formatted<'a> {
            format::Formatted::new(Fields::datetime(self.local()), pattern)
        }

        /// The same instant with offset zero.
        #[must_use]
        #[inline]
        pub fn fixed_offset(&self) -> OffsetDateTime {
            OffsetDateTime {
                ts: self.instant(),
                offset: self.offset_seconds(),
            }
        }

        /// The same instant, in UTC.
        #[must_use]
        #[inline]
        pub fn to_utc(&self) -> Timestamp {
            Timestamp(self.instant())
        }

        /// The same instant shown in a time zone: with the zone's offset at that instant.
        #[must_use]
        #[inline]
        pub fn with_timezone(&self, zone: &crate::Tz) -> OffsetDateTime {
            zone.at(self.to_utc())
        }

        /// The `jiff` instant underneath.
        #[must_use]
        #[inline]
        pub fn to_jiff(&self) -> jiff::Timestamp {
            self.instant()
        }

        /// The whole duration from `rhs` to `self`.
        #[must_use]
        #[inline]
        pub fn signed_duration_since<T: Into<Timestamp>>(&self, rhs: T) -> Duration {
            let rhs: Timestamp = rhs.into();
            duration_between(self.instant(), rhs.0)
        }
    };
}

fn duration_between(a: jiff::Timestamp, b: jiff::Timestamp) -> Duration {
    Duration::of_instant_span(a.duration_since(b))
}

impl Timestamp {
    /// 1970-01-01T00:00:00Z.
    pub const UNIX_EPOCH: Timestamp = Timestamp(jiff::Timestamp::UNIX_EPOCH);
    /// The earliest instant.
    pub const MIN_UTC: Timestamp = Timestamp(jiff::Timestamp::MIN);
    /// The latest instant.
    pub const MAX_UTC: Timestamp = Timestamp(jiff::Timestamp::MAX);

    /// The current instant, from the system clock.
    #[must_use]
    #[inline]
    pub fn now() -> Timestamp {
        Timestamp(jiff::Timestamp::now())
    }

    #[inline]
    fn instant(&self) -> jiff::Timestamp {
        self.0
    }

    #[allow(clippy::unused_self)]
    #[inline]
    fn offset_seconds(&self) -> i32 {
        0
    }

    #[inline]
    fn local(&self) -> jiff::civil::DateTime {
        Offset::UTC.to_datetime(self.0)
    }

    instant_methods!();

    /// From the `jiff` instant.
    #[must_use]
    #[inline]
    pub const fn from_jiff(ts: jiff::Timestamp) -> Timestamp {
        Timestamp(ts)
    }

    /// The instant at this civil date and time in UTC, or `None` if there is no such date or time.
    #[must_use]
    #[inline]
    pub fn with_ymd_and_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Option<Timestamp> {
        Date::from_ymd_opt(year, month, day)?
            .and_hms_opt(hour, min, sec)
            .and_then(|dt| Offset::UTC.to_timestamp(dt.0).ok())
            .map(Timestamp)
    }

    /// `secs` seconds and `nsecs` nanoseconds (below a second) after 1970-01-01T00:00:00Z, or `None` out of
    /// range.
    #[must_use]
    #[inline]
    pub fn from_timestamp(secs: i64, nsecs: u32) -> Option<Timestamp> {
        if i128::from(nsecs) >= NANOS_PER_SEC {
            return None;
        }
        from_nanos(i128::from(secs) * NANOS_PER_SEC + i128::from(nsecs)).map(Timestamp)
    }

    /// `millis` milliseconds after 1970-01-01T00:00:00Z, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn from_timestamp_millis(millis: i64) -> Option<Timestamp> {
        from_nanos(i128::from(millis) * 1_000_000).map(Timestamp)
    }

    /// `micros` microseconds after 1970-01-01T00:00:00Z, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn from_timestamp_micros(micros: i64) -> Option<Timestamp> {
        from_nanos(i128::from(micros) * 1000).map(Timestamp)
    }

    /// `nanos` nanoseconds after 1970-01-01T00:00:00Z. Every `i64` count (1677 to 2262) is in range, so `None` is not
    /// expected; it is returned, not panicked on.
    #[must_use]
    #[inline]
    pub fn from_timestamp_nanos(nanos: i64) -> Option<Timestamp> {
        from_nanos(i128::from(nanos)).map(Timestamp)
    }

    /// Read strictly as RFC 3339 (see [`crate::parse`]), converted to UTC.
    ///
    /// # Errors
    ///
    /// The text is not RFC 3339 or names no such instant.
    #[inline]
    pub fn parse_from_rfc3339(s: &str) -> ParseResult<Timestamp> {
        parse::rfc3339_strict(s).map(|(ts, _)| Timestamp(ts))
    }

    /// Moved by a duration, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_signed(self, rhs: Duration) -> Option<Timestamp> {
        self.0.checked_add(rhs.to_jiff()).ok().map(Timestamp)
    }

    /// Moved back by a duration, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_sub_signed(self, rhs: Duration) -> Option<Timestamp> {
        self.0.checked_sub(rhs.to_jiff()).ok().map(Timestamp)
    }

    /// The same time of day `days` later in UTC, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_days(self, days: Days) -> Option<Timestamp> {
        self.checked_add_signed(Duration::try_days(i64::try_from(days.0).ok()?)?)
    }

    /// The same time of day `days` earlier in UTC, or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_sub_days(self, days: Days) -> Option<Timestamp> {
        self.checked_sub_signed(Duration::try_days(i64::try_from(days.0).ok()?)?)
    }

    /// The same time of day `months` later in UTC (the day clamped to the end of a shorter month).
    #[must_use]
    #[inline]
    pub fn checked_add_months(self, months: Months) -> Option<Timestamp> {
        let local = self.naive_utc();
        let date = local.date().checked_add_months(months)?;
        Offset::UTC
            .to_timestamp(date.0.to_datetime(local.0.time()))
            .ok()
            .map(Timestamp)
    }

    /// The same time of day `months` earlier in UTC (the day clamped to the end of a shorter month).
    #[must_use]
    #[inline]
    pub fn checked_sub_months(self, months: Months) -> Option<Timestamp> {
        let local = self.naive_utc();
        let date = local.date().checked_sub_months(months)?;
        Offset::UTC
            .to_timestamp(date.0.to_datetime(local.0.time()))
            .ok()
            .map(Timestamp)
    }
}

impl OffsetDateTime {
    #[inline]
    fn instant(&self) -> jiff::Timestamp {
        self.ts
    }

    #[inline]
    fn offset_seconds(&self) -> i32 {
        self.offset
    }

    #[inline]
    fn local(&self) -> jiff::civil::DateTime {
        offset_of(self.offset).to_datetime(self.ts)
    }

    instant_methods!();

    /// The instant at `ts` shown at `offset_seconds` east of UTC, or `None` for an offset of a day or more.
    #[must_use]
    #[inline]
    pub fn from_instant(ts: Timestamp, offset_seconds: i32) -> Option<OffsetDateTime> {
        (offset_seconds.unsigned_abs() < 86_400).then_some(OffsetDateTime {
            ts: ts.0,
            offset: offset_seconds,
        })
    }

    /// The offset, in seconds east of UTC.
    #[must_use]
    #[inline]
    pub fn offset_local_minus_utc(&self) -> i32 {
        self.offset
    }

    /// Read strictly as RFC 3339 (see [`crate::parse`]), keeping the offset it gives.
    ///
    /// # Errors
    ///
    /// The text is not RFC 3339 or names no such instant.
    #[inline]
    pub fn parse_from_rfc3339(s: &str) -> ParseResult<OffsetDateTime> {
        parse::rfc3339_strict(s).map(|(ts, offset)| OffsetDateTime { ts, offset })
    }

    /// Read with a `%` pattern that names a date, a time and an offset (`%Y-%m-%d %H:%M:%S%.f%#z`).
    ///
    /// # Errors
    ///
    /// The text does not match the pattern, names no such instant, or the pattern names no offset.
    #[inline]
    pub fn parse_from_str(s: &str, pattern: &str) -> ParseResult<OffsetDateTime> {
        parse::strptime(s, pattern)?
            .to_instant()
            .map(|(ts, offset)| OffsetDateTime { ts, offset })
    }

    /// Moved by a duration (keeping the offset), or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_add_signed(self, rhs: Duration) -> Option<OffsetDateTime> {
        let ts = self.ts.checked_add(rhs.to_jiff()).ok()?;
        Some(OffsetDateTime {
            ts,
            offset: self.offset,
        })
    }

    /// Moved back by a duration (keeping the offset), or `None` out of range.
    #[must_use]
    #[inline]
    pub fn checked_sub_signed(self, rhs: Duration) -> Option<OffsetDateTime> {
        let ts = self.ts.checked_sub(rhs.to_jiff()).ok()?;
        Some(OffsetDateTime {
            ts,
            offset: self.offset,
        })
    }
}

impl Default for Timestamp {
    /// 1970-01-01T00:00:00Z.
    #[inline]
    fn default() -> Timestamp {
        Timestamp::UNIX_EPOCH
    }
}

impl Default for OffsetDateTime {
    /// 1970-01-01T00:00:00+00:00.
    #[inline]
    fn default() -> OffsetDateTime {
        Timestamp::UNIX_EPOCH.fixed_offset()
    }
}

impl From<Timestamp> for OffsetDateTime {
    #[inline]
    fn from(t: Timestamp) -> OffsetDateTime {
        t.fixed_offset()
    }
}

impl From<OffsetDateTime> for Timestamp {
    #[inline]
    fn from(t: OffsetDateTime) -> Timestamp {
        t.to_utc()
    }
}

impl From<jiff::Timestamp> for Timestamp {
    #[inline]
    fn from(t: jiff::Timestamp) -> Timestamp {
        Timestamp(t)
    }
}

impl From<Timestamp> for jiff::Timestamp {
    #[inline]
    fn from(t: Timestamp) -> jiff::Timestamp {
        t.0
    }
}

impl PartialEq for OffsetDateTime {
    #[inline]
    fn eq(&self, other: &OffsetDateTime) -> bool {
        self.ts == other.ts
    }
}

impl Eq for OffsetDateTime {}

impl PartialOrd for OffsetDateTime {
    #[inline]
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OffsetDateTime {
    #[inline]
    fn cmp(&self, other: &OffsetDateTime) -> Ordering {
        self.ts.cmp(&other.ts)
    }
}

impl Hash for OffsetDateTime {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ts.hash(state);
    }
}

impl PartialEq<Timestamp> for OffsetDateTime {
    #[inline]
    fn eq(&self, other: &Timestamp) -> bool {
        self.ts == other.0
    }
}

impl PartialEq<OffsetDateTime> for Timestamp {
    #[inline]
    fn eq(&self, other: &OffsetDateTime) -> bool {
        self.0 == other.ts
    }
}

impl PartialOrd<Timestamp> for OffsetDateTime {
    #[inline]
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        Some(self.ts.cmp(&other.0))
    }
}

impl PartialOrd<OffsetDateTime> for Timestamp {
    #[inline]
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        Some(self.0.cmp(&other.ts))
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Duration;
    #[inline]
    fn sub(self, rhs: Timestamp) -> Duration {
        duration_between(self.0, rhs.0)
    }
}

impl Sub<OffsetDateTime> for OffsetDateTime {
    type Output = Duration;
    #[inline]
    fn sub(self, rhs: OffsetDateTime) -> Duration {
        duration_between(self.ts, rhs.ts)
    }
}

/// `2026-10-30 09:15:00.123400 UTC`.
impl fmt::Display for Timestamp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = self.local();
        crate::date::write_date(f, dt.date())?;
        f.write_str(" ")?;
        format::write_time(f, dt.time())?;
        f.write_str(" UTC")
    }
}

/// `2026-10-30T09:15:00.123400Z`.
impl fmt::Debug for Timestamp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = self.local();
        crate::date::write_date(f, dt.date())?;
        f.write_str("T")?;
        format::write_time(f, dt.time())?;
        f.write_str("Z")
    }
}

/// `2026-10-30 09:15:00.123400 +00:00`.
impl fmt::Display for OffsetDateTime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = self.local();
        crate::date::write_date(f, dt.date())?;
        f.write_str(" ")?;
        format::write_time(f, dt.time())?;
        f.write_str(" ")?;
        format::write_offset_exact(f, self.offset)
    }
}

/// `2026-10-30T09:15:00.123400+00:00`.
impl fmt::Debug for OffsetDateTime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = self.local();
        crate::date::write_date(f, dt.date())?;
        f.write_str("T")?;
        format::write_time(f, dt.time())?;
        format::write_offset_exact(f, self.offset)
    }
}

/// The relaxed reading (see [`crate::parse`]), converted to UTC.
impl FromStr for Timestamp {
    type Err = ParseError;
    #[inline]
    fn from_str(s: &str) -> ParseResult<Timestamp> {
        parse::rfc3339_relaxed(s).map(|(ts, _)| Timestamp(ts))
    }
}

/// The relaxed reading (see [`crate::parse`]), keeping the offset.
impl FromStr for OffsetDateTime {
    type Err = ParseError;
    #[inline]
    fn from_str(s: &str) -> ParseResult<OffsetDateTime> {
        parse::rfc3339_relaxed(s).map(|(ts, offset)| OffsetDateTime { ts, offset })
    }
}

struct Rfc3339Json<'a>(
    jiff::civil::DateTime,
    i32,
    core::marker::PhantomData<&'a ()>,
);

impl fmt::Display for Rfc3339Json<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format::write_rfc3339(f, self.0, self.1, SecondsFormat::AutoSi, true)
    }
}

/// RFC 3339 with `Z` for a zero offset and the fraction in groups of three.
impl serde::Serialize for Timestamp {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&Rfc3339Json(self.local(), 0, core::marker::PhantomData))
    }
}

/// RFC 3339 with `Z` for a zero offset and the fraction in groups of three.
impl serde::Serialize for OffsetDateTime {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&Rfc3339Json(
            self.local(),
            self.offset,
            core::marker::PhantomData,
        ))
    }
}

struct InstantVisitor;

impl serde::de::Visitor<'_> for InstantVisitor {
    type Value = OffsetDateTime;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an RFC 3339 formatted date and time string")
    }

    #[inline]
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<OffsetDateTime, E> {
        value.parse().map_err(E::custom)
    }
}

/// The relaxed reading, converted to UTC.
impl<'de> serde::Deserialize<'de> for Timestamp {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Timestamp, D::Error> {
        deserializer
            .deserialize_str(InstantVisitor)
            .map(|t| t.to_utc())
    }
}

/// The relaxed reading, keeping the offset.
impl<'de> serde::Deserialize<'de> for OffsetDateTime {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<OffsetDateTime, D::Error> {
        deserializer.deserialize_str(InstantVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{DefaultHasher, Hasher};

    #[test]
    fn a_wrong_json_type_names_what_was_expected() {
        let e = serde_json::from_str::<Timestamp>("5")
            .unwrap_err()
            .to_string();
        assert_eq!(
            e,
            "invalid type: integer `5`, expected an RFC 3339 formatted date and time string at line 1 column 1"
        );
    }

    #[test]
    fn both_types_compare_by_instant() {
        let a: OffsetDateTime = "2026-10-30T09:15:00+05:30".parse().unwrap();
        let b: Timestamp = "2026-10-30T03:45:00Z".parse().unwrap();
        assert_eq!(a, b);
        assert_eq!(b, a);
        assert_eq!(a, b.fixed_offset());
        assert_eq!(format!("{a:?}"), "2026-10-30T09:15:00+05:30");
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            "\"2026-10-30T09:15:00+05:30\""
        );
        assert_eq!(a.date_naive(), Date::from_ymd_opt(2026, 10, 30).unwrap());
        assert_eq!(a.to_utc().to_string(), "2026-10-30 03:45:00 UTC");
    }

    #[test]
    fn a_leap_second_is_read_as_the_second_before() {
        let t: Timestamp = "2016-12-31T23:59:60.5Z".parse().unwrap();
        assert_eq!(format!("{t:?}"), "2016-12-31T23:59:59.500Z");
        let t = Timestamp::parse_from_rfc3339("2016-12-31T23:59:60Z").unwrap();
        assert_eq!(format!("{t:?}"), "2016-12-31T23:59:59Z");
    }

    #[test]
    fn months_on_an_instant_keep_the_time_of_day() {
        let t: Timestamp = "2024-01-31T10:00:00Z".parse().unwrap();
        assert_eq!(
            format!("{:?}", t.checked_add_months(Months::new(1)).unwrap()),
            "2024-02-29T10:00:00Z"
        );
    }

    #[test]
    #[expect(clippy::too_many_lines)] // one walk through every accessor and format
    fn timestamp_accessors_constructors_arithmetic_and_formats_are_consistent() {
        let mut t = Timestamp::with_ymd_and_hms(2024, 2, 29, 23, 58, 57).unwrap();
        t = t.checked_add_signed(Duration::milliseconds(654)).unwrap();
        assert_eq!(
            (
                t.year(),
                t.month(),
                t.day(),
                t.hour(),
                t.minute(),
                t.second(),
                t.nanosecond(),
            ),
            (2024, 2, 29, 23, 58, 57, 654_000_000)
        );
        assert_eq!(t.weekday(), Weekday::Thu);
        assert_eq!(t.ordinal(), 60);
        assert_eq!(t.timestamp_subsec_millis(), 654);
        assert_eq!(t.timestamp_subsec_micros(), 654_000);
        assert_eq!(t.timestamp_subsec_nanos(), 654_000_000);
        assert_eq!(t.timestamp_millis(), t.timestamp() * 1000 + 654);
        assert_eq!(t.timestamp_micros(), t.timestamp() * 1_000_000 + 654_000);
        assert_eq!(
            t.timestamp_nanos_opt(),
            Some(t.timestamp() * 1_000_000_000 + 654_000_000)
        );
        assert_eq!(t.naive_utc(), t.naive_local());
        assert_eq!(t.date_naive(), Date::from_ymd_opt(2024, 2, 29).unwrap());
        assert_eq!(t.to_rfc3339(), "2024-02-29T23:58:57.654+00:00");
        assert_eq!(
            t.to_rfc3339_opts(SecondsFormat::Secs, true),
            "2024-02-29T23:58:57Z"
        );
        assert_eq!(
            t.format("%Y %m %d %H %M %S %%").to_string(),
            "2024 02 29 23 58 57 %"
        );
        assert_eq!(t.to_jiff(), Timestamp::from_jiff(t.to_jiff()).to_jiff());
        assert_eq!(
            t.signed_duration_since(Timestamp::UNIX_EPOCH),
            t - Timestamp::UNIX_EPOCH
        );

        let original = t;
        t = t.checked_sub_signed(Duration::milliseconds(654)).unwrap();
        assert_eq!(
            t.checked_add_signed(Duration::milliseconds(654)).unwrap(),
            original
        );
        assert_eq!(
            t.checked_add_days(Days::new(1))
                .unwrap()
                .checked_sub_days(Days::new(1))
                .unwrap(),
            t
        );
        assert_eq!(
            t.checked_add_months(Months::new(1))
                .unwrap()
                .checked_sub_months(Months::new(1))
                .unwrap(),
            t
        );
        assert_eq!(t.checked_add_days(Days::new(1)).unwrap().day(), 1);
        assert_eq!(t.checked_sub_days(Days::new(1)).unwrap().day(), 28);
        assert_eq!(
            Timestamp::MAX_UTC.checked_add_signed(Duration::nanoseconds(1)),
            None
        );
        assert_eq!(
            Timestamp::MIN_UTC.checked_sub_signed(Duration::nanoseconds(1)),
            None
        );
        assert_eq!(Timestamp::MAX_UTC.checked_add_days(Days::new(1)), None);
        assert_eq!(Timestamp::MIN_UTC.checked_sub_days(Days::new(1)), None);
        assert_eq!(Timestamp::MAX_UTC.timestamp_nanos_opt(), None);

        assert_eq!(
            Timestamp::from_timestamp(1, 2)
                .unwrap()
                .timestamp_nanos_opt(),
            Some(1_000_000_002)
        );
        assert_eq!(Timestamp::from_timestamp(0, 1_000_000_000), None);
        assert_eq!(
            Timestamp::from_timestamp_millis(1500)
                .unwrap()
                .timestamp_millis(),
            1500
        );
        assert_eq!(
            Timestamp::from_timestamp_micros(1500)
                .unwrap()
                .timestamp_micros(),
            1500
        );
        assert_eq!(
            Timestamp::from_timestamp_nanos(-1)
                .unwrap()
                .timestamp_nanos_opt(),
            Some(-1)
        );
        assert_eq!(Timestamp::default(), Timestamp::UNIX_EPOCH);
        assert!(Timestamp::now() >= Timestamp::UNIX_EPOCH);
        assert_eq!(Timestamp::with_ymd_and_hms(2024, 2, 30, 0, 0, 0), None);
    }

    #[test]
    fn fixed_offsets_keep_the_instant_across_conversions_order_and_hashing() {
        let utc: Timestamp = "2026-10-30T03:45:00Z".parse().unwrap();
        let mut local = OffsetDateTime::from_instant(utc, 19_800).unwrap();
        assert_eq!(local.offset_local_minus_utc(), 19_800);
        assert_eq!(local.naive_utc(), utc.naive_utc());
        assert_eq!(local.hour(), 9);
        assert_eq!(local.to_rfc3339(), "2026-10-30T09:15:00+05:30");
        assert_eq!(local.fixed_offset(), local);
        assert_eq!(local.to_utc(), utc);
        assert_eq!(Timestamp::from(local), utc);
        assert_eq!(OffsetDateTime::from(utc), utc.fixed_offset());
        assert_eq!(jiff::Timestamp::from(utc), utc.to_jiff());
        assert_eq!(Timestamp::from(utc.to_jiff()), utc);
        assert_eq!(OffsetDateTime::from_instant(utc, 86_400), None);

        let parsed = OffsetDateTime::parse_from_rfc3339("2026-10-30T09:15:00+05:30").unwrap();
        let patterned =
            OffsetDateTime::parse_from_str("2026-10-30 09:15:00+05:30", "%Y-%m-%d %H:%M:%S%#z")
                .unwrap();
        assert_eq!(parsed, patterned);
        assert_eq!(
            serde_json::from_str::<OffsetDateTime>("\"2026-10-30T09:15:00+05:30\"").unwrap(),
            parsed
        );
        assert_eq!(
            serde_json::from_str::<Timestamp>("\"2026-10-30T09:15:00+05:30\"").unwrap(),
            utc
        );

        local = local.checked_add_signed(Duration::seconds(2)).unwrap();
        local = local.checked_sub_signed(Duration::seconds(1)).unwrap();
        assert_eq!(local - parsed, Duration::seconds(1));
        assert_eq!(
            parsed.checked_add_signed(Duration::seconds(1)).unwrap(),
            local
        );
        assert_eq!(
            local.checked_sub_signed(Duration::seconds(1)).unwrap(),
            parsed
        );
        assert_eq!(
            OffsetDateTime::default(),
            Timestamp::UNIX_EPOCH.fixed_offset()
        );
        assert!(parsed.partial_cmp(&utc).is_some());
        assert!(utc.partial_cmp(&parsed).is_some());

        let mut first = DefaultHasher::new();
        let mut second = DefaultHasher::new();
        parsed.hash(&mut first);
        utc.fixed_offset().hash(&mut second);
        assert_eq!(first.finish(), second.finish());
        assert_eq!(utc.with_timezone(&crate::Tz::UTC), utc.fixed_offset());
    }
}

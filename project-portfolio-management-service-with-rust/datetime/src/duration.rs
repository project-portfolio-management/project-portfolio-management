//! A signed length of time, and the day and month counts dates move by.
//!
//! [`Duration`] holds a `jiff::SignedDuration` within the range the service's durations always had: at most
//! `i64::MAX` milliseconds either way (a constructor or sum past that is `None`, or panics where the operator
//! form is used, as before). Its whole-unit counts (`num_days`, `num_hours`, …) truncate toward zero.

use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use jiff::SignedDuration;

use crate::error::OutOfRangeError;

const NANOS_PER_SEC: i128 = 1_000_000_000;
/// `i64::MAX` milliseconds, in nanoseconds: the largest duration either way.
const MAX_NANOS: i128 = i64::MAX as i128 * 1_000_000;

/// A signed duration, to the nanosecond.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Duration(SignedDuration);

impl Duration {
    /// The largest duration: `i64::MAX` milliseconds.
    pub const MAX: Duration = Duration(SignedDuration::new(
        i64::MAX / 1000,
        (i64::MAX % 1000) as i32 * 1_000_000,
    ));
    /// The smallest duration: `-i64::MAX` milliseconds.
    pub const MIN: Duration = Duration(SignedDuration::new(
        -(i64::MAX / 1000),
        -((i64::MAX % 1000) as i32) * 1_000_000,
    ));

    const fn from_total_nanos(nanos: i128) -> Option<Duration> {
        if nanos < -MAX_NANOS || nanos > MAX_NANOS {
            return None;
        }
        Some(Duration(SignedDuration::from_nanos_i128(nanos)))
    }

    fn total_nanos(self) -> i128 {
        self.0.as_nanos()
    }

    const fn of(count: i64, nanos_per_unit: i128) -> Option<Duration> {
        match (count as i128).checked_mul(nanos_per_unit) {
            Some(n) => Duration::from_total_nanos(n),
            None => None,
        }
    }

    /// A duration of no length.
    #[must_use]
    pub const fn zero() -> Duration {
        Duration(SignedDuration::ZERO)
    }

    /// `n` weeks, or `None` past the range.
    #[must_use]
    pub const fn try_weeks(n: i64) -> Option<Duration> {
        Duration::of(n, 7 * 86_400 * NANOS_PER_SEC)
    }

    /// `n` days, or `None` past the range.
    #[must_use]
    pub const fn try_days(n: i64) -> Option<Duration> {
        Duration::of(n, 86_400 * NANOS_PER_SEC)
    }

    /// `n` hours, or `None` past the range.
    #[must_use]
    pub const fn try_hours(n: i64) -> Option<Duration> {
        Duration::of(n, 3600 * NANOS_PER_SEC)
    }

    /// `n` minutes, or `None` past the range.
    #[must_use]
    pub const fn try_minutes(n: i64) -> Option<Duration> {
        Duration::of(n, 60 * NANOS_PER_SEC)
    }

    /// `n` seconds, or `None` past the range.
    #[must_use]
    pub const fn try_seconds(n: i64) -> Option<Duration> {
        Duration::of(n, NANOS_PER_SEC)
    }

    /// `n` milliseconds, or `None` past the range (never, as the range is `i64` milliseconds).
    #[must_use]
    pub const fn try_milliseconds(n: i64) -> Option<Duration> {
        Duration::of(n, 1_000_000)
    }

    /// `n` weeks.
    ///
    /// # Panics
    ///
    /// Past the range.
    #[must_use]
    pub const fn weeks(n: i64) -> Duration {
        Duration::try_weeks(n).expect("Duration::weeks out of bounds")
    }

    /// `n` days.
    ///
    /// # Panics
    ///
    /// Past the range.
    #[must_use]
    pub const fn days(n: i64) -> Duration {
        Duration::try_days(n).expect("Duration::days out of bounds")
    }

    /// `n` hours.
    ///
    /// # Panics
    ///
    /// Past the range.
    #[must_use]
    pub const fn hours(n: i64) -> Duration {
        Duration::try_hours(n).expect("Duration::hours out of bounds")
    }

    /// `n` minutes.
    ///
    /// # Panics
    ///
    /// Past the range.
    #[must_use]
    pub const fn minutes(n: i64) -> Duration {
        Duration::try_minutes(n).expect("Duration::minutes out of bounds")
    }

    /// `n` seconds.
    ///
    /// # Panics
    ///
    /// Past the range.
    #[must_use]
    pub const fn seconds(n: i64) -> Duration {
        Duration::try_seconds(n).expect("Duration::seconds out of bounds")
    }

    /// `n` milliseconds.
    ///
    /// # Panics
    ///
    /// Never: every `i64` count of milliseconds is in range.
    #[must_use]
    pub const fn milliseconds(n: i64) -> Duration {
        Duration::try_milliseconds(n).expect("Duration::milliseconds out of bounds")
    }

    /// `n` microseconds.
    #[must_use]
    pub const fn microseconds(n: i64) -> Duration {
        Duration(SignedDuration::from_micros(n))
    }

    /// `n` nanoseconds.
    #[must_use]
    pub const fn nanoseconds(n: i64) -> Duration {
        Duration(SignedDuration::from_nanos(n))
    }

    /// Whole weeks, toward zero.
    #[must_use]
    pub const fn num_weeks(&self) -> i64 {
        self.num_seconds() / (7 * 86_400)
    }

    /// Whole days, toward zero.
    #[must_use]
    pub const fn num_days(&self) -> i64 {
        self.num_seconds() / 86_400
    }

    /// Whole hours, toward zero.
    #[must_use]
    pub const fn num_hours(&self) -> i64 {
        self.num_seconds() / 3600
    }

    /// Whole minutes, toward zero.
    #[must_use]
    pub const fn num_minutes(&self) -> i64 {
        self.num_seconds() / 60
    }

    /// Whole seconds, toward zero.
    #[must_use]
    pub const fn num_seconds(&self) -> i64 {
        self.0.as_secs()
    }

    /// Whole milliseconds, toward zero.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)] // within ±i64::MAX milliseconds by construction
    pub const fn num_milliseconds(&self) -> i64 {
        self.0.as_millis() as i64
    }

    /// Whole microseconds, toward zero, or `None` past `i64`.
    #[must_use]
    pub fn num_microseconds(&self) -> Option<i64> {
        i64::try_from(self.0.as_micros()).ok()
    }

    /// Nanoseconds, or `None` past `i64`.
    #[must_use]
    pub fn num_nanoseconds(&self) -> Option<i64> {
        i64::try_from(self.0.as_nanos()).ok()
    }

    /// The part below a second, with the duration's sign.
    #[must_use]
    pub const fn subsec_nanos(&self) -> i32 {
        self.0.subsec_nanos()
    }

    /// Fractional seconds.
    #[must_use]
    pub fn as_seconds_f64(self) -> f64 {
        self.0.as_secs_f64()
    }

    /// Whether this is of no length.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// The length, without its sign.
    #[must_use]
    pub const fn abs(&self) -> Duration {
        Duration(self.0.abs())
    }

    /// The sum, or `None` past the range.
    #[must_use]
    pub fn checked_add(&self, rhs: &Duration) -> Option<Duration> {
        Duration::from_total_nanos(self.total_nanos() + rhs.total_nanos())
    }

    /// The difference, or `None` past the range.
    #[must_use]
    pub fn checked_sub(&self, rhs: &Duration) -> Option<Duration> {
        Duration::from_total_nanos(self.total_nanos() - rhs.total_nanos())
    }

    /// The product, or `None` past the range.
    #[must_use]
    pub fn checked_mul(&self, rhs: i32) -> Option<Duration> {
        Duration::from_total_nanos(self.total_nanos().checked_mul(i128::from(rhs))?)
    }

    /// The quotient, or `None` for a divisor of zero. Computed as before, on whole seconds rounded down and the
    /// non-negative nanosecond rest, so a quotient below a nanosecond rounds as it always did.
    #[must_use]
    pub fn checked_div(&self, rhs: i32) -> Option<Duration> {
        if rhs == 0 {
            return None;
        }
        let (s, n) = self.floor_parts();
        let rhs64 = i64::from(rhs);
        let secs = s / rhs64;
        let carry = s % rhs64;
        let extra_nanos = carry * 1_000_000_000 / rhs64;
        // `n / rhs` and `extra_nanos` are each below a second in size, so the sum fits an `i64`.
        let nanos = i64::from(n / rhs) + extra_nanos;
        let total = i128::from(secs) * NANOS_PER_SEC + i128::from(nanos);
        Duration::from_total_nanos(total)
    }

    /// As a standard library duration.
    ///
    /// # Errors
    ///
    /// When this duration is negative.
    pub fn to_std(&self) -> Result<std::time::Duration, OutOfRangeError> {
        std::time::Duration::try_from(self.0).map_err(|_| OutOfRangeError(()))
    }

    /// From a standard library duration.
    ///
    /// # Errors
    ///
    /// When it is longer than [`Duration::MAX`].
    pub fn from_std(d: std::time::Duration) -> Result<Duration, OutOfRangeError> {
        i128::try_from(d.as_nanos())
            .ok()
            .and_then(Duration::from_total_nanos)
            .ok_or(OutOfRangeError(()))
    }

    /// The `jiff` duration underneath.
    #[must_use]
    pub const fn to_jiff(self) -> SignedDuration {
        self.0
    }

    /// From a `jiff` duration, or `None` past the range.
    #[must_use]
    pub fn from_jiff(d: SignedDuration) -> Option<Duration> {
        Duration::from_total_nanos(d.as_nanos())
    }

    /// Seconds rounded down and a non-negative nanosecond part, as the older representation kept them.
    fn floor_parts(self) -> (i64, i32) {
        let n = self.total_nanos();
        // Within ±i64::MAX milliseconds, the floored seconds fit in an `i64`.
        #[allow(clippy::cast_possible_truncation)]
        let secs = n.div_euclid(NANOS_PER_SEC) as i64;
        #[allow(clippy::cast_possible_truncation)]
        let nanos = n.rem_euclid(NANOS_PER_SEC) as i32;
        (secs, nanos)
    }
}

impl Neg for Duration {
    type Output = Duration;
    fn neg(self) -> Duration {
        Duration(-self.0)
    }
}

impl Add for Duration {
    type Output = Duration;
    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(&rhs)
            .expect("`Duration + Duration` overflowed")
    }
}

impl Sub for Duration {
    type Output = Duration;
    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(&rhs)
            .expect("`Duration - Duration` overflowed")
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Duration {
    type Output = Duration;
    fn mul(self, rhs: i32) -> Duration {
        self.checked_mul(rhs).expect("`Duration * i32` overflowed")
    }
}

impl Div<i32> for Duration {
    type Output = Duration;
    fn div(self, rhs: i32) -> Duration {
        self.checked_div(rhs).expect("`i32` is zero")
    }
}

impl core::iter::Sum for Duration {
    fn sum<I: Iterator<Item = Duration>>(iter: I) -> Duration {
        iter.fold(Duration::zero(), |a, b| a + b)
    }
}

impl<'a> core::iter::Sum<&'a Duration> for Duration {
    fn sum<I: Iterator<Item = &'a Duration>>(iter: I) -> Duration {
        iter.fold(Duration::zero(), |a, b| a + *b)
    }
}

/// As the older type's derived `Debug`: seconds rounded down and a non-negative nanosecond part.
impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (secs, nanos) = self.floor_parts();
        f.debug_struct("TimeDelta")
            .field("secs", &secs)
            .field("nanos", &nanos)
            .finish()
    }
}

/// ISO 8601, in seconds: `PT90S`, `PT1.5S`, `-PT3600S`, and `P0D` for zero.
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (secs, _) = self.floor_parts();
        let (abs, sign) = if secs < 0 { (-*self, "-") } else { (*self, "") };
        let (secs, nanos) = abs.floor_parts();
        write!(f, "{sign}P")?;
        if secs == 0 && nanos == 0 {
            return f.write_str("0D");
        }
        write!(f, "T{secs}")?;
        if nanos > 0 {
            let mut figures = 9usize;
            let mut digits = nanos;
            while digits % 10 == 0 {
                digits /= 10;
                figures -= 1;
            }
            write!(f, ".{digits:0figures$}")?;
        }
        f.write_str("S")
    }
}

/// A count of days to move a date by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Days(pub(crate) u64);

impl Days {
    /// `n` days.
    #[must_use]
    pub const fn new(n: u64) -> Days {
        Days(n)
    }
}

/// A count of months to move a date by; the day is clamped to the end of a shorter month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Months(pub(crate) u32);

impl Months {
    /// `n` months.
    #[must_use]
    pub const fn new(n: u32) -> Months {
        Months(n)
    }

    /// The count.
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_truncate_toward_zero() {
        let d = Duration::seconds(-129_570);
        assert_eq!(
            (d.num_days(), d.num_hours(), d.num_minutes()),
            (-1, -35, -2159)
        );
        assert_eq!(Duration::milliseconds(-1500).num_seconds(), -1);
        assert_eq!(Duration::nanoseconds(-1).num_milliseconds(), 0);
    }

    #[test]
    fn range_is_i64_milliseconds() {
        assert!(Duration::try_seconds(i64::MAX / 1000).is_some());
        assert!(Duration::try_seconds(i64::MAX / 1000 + 1).is_none());
        assert!(Duration::try_days(i64::MAX / 86_400_000 + 1).is_none());
        assert!(
            Duration::MAX
                .checked_add(&Duration::nanoseconds(1))
                .is_none()
        );
        assert_eq!(Duration::MIN, -Duration::MAX);
    }

    #[test]
    fn debug_and_display_as_before() {
        assert_eq!(
            format!("{:?}", Duration::milliseconds(-1500)),
            "TimeDelta { secs: -2, nanos: 500000000 }"
        );
        assert_eq!(Duration::milliseconds(1500).to_string(), "PT1.5S");
        assert_eq!(Duration::seconds(-90).to_string(), "-PT90S");
        assert_eq!(Duration::zero().to_string(), "P0D");
        assert!(Duration::seconds(-1).to_std().is_err());
        assert_eq!(
            Duration::seconds(2).to_std().unwrap(),
            std::time::Duration::from_secs(2)
        );
    }
}

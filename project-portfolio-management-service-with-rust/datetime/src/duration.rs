//! A signed length of time, and the day and month counts dates move by.
//!
//! [`Duration`] holds a `jiff::SignedDuration` within the range the service's durations always had: at most
//! `i64::MAX` milliseconds either way (a constructor or sum past that is `None`, or panics where the operator
//! form is used, as before). Its whole-unit counts (`num_days`, `num_hours`, …) truncate toward zero.

use core::fmt;

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

    #[inline]
    const fn from_total_nanos(nanos: i128) -> Option<Duration> {
        if nanos < -MAX_NANOS || nanos > MAX_NANOS {
            return None;
        }
        Some(Duration(SignedDuration::from_nanos_i128(nanos)))
    }

    #[inline]
    fn total_nanos(self) -> i128 {
        self.0.as_nanos()
    }

    /// `count` whole units of `nanos_per_unit` (a whole number of seconds, or milliseconds), on `i64`
    /// seconds where it can be: these constructors are in the calendar code's hot loops.
    #[allow(clippy::cast_possible_truncation)] // a unit of a second or more is a whole number of seconds below i64
    #[inline]
    const fn of(count: i64, nanos_per_unit: i128) -> Option<Duration> {
        if nanos_per_unit % NANOS_PER_SEC == 0 {
            let secs_per_unit = (nanos_per_unit / NANOS_PER_SEC) as i64;
            return match count.checked_mul(secs_per_unit) {
                Some(secs) if secs.unsigned_abs() <= (i64::MAX / 1000).unsigned_abs() => {
                    Some(Duration(SignedDuration::from_secs(secs)))
                }
                _ => None,
            };
        }
        match (count as i128).checked_mul(nanos_per_unit) {
            Some(n) => Duration::from_total_nanos(n),
            None => None,
        }
    }

    /// A duration of no length.
    #[must_use]
    #[inline]
    pub const fn zero() -> Duration {
        Duration(SignedDuration::ZERO)
    }

    /// `n` weeks, or `None` past the range.
    #[must_use]
    #[inline]
    pub const fn try_weeks(n: i64) -> Option<Duration> {
        Duration::of(n, 7 * 86_400 * NANOS_PER_SEC)
    }

    /// `n` days, or `None` past the range.
    #[must_use]
    #[inline]
    pub const fn try_days(n: i64) -> Option<Duration> {
        Duration::of(n, 86_400 * NANOS_PER_SEC)
    }

    /// `n` hours, or `None` past the range.
    #[must_use]
    #[inline]
    pub const fn try_hours(n: i64) -> Option<Duration> {
        Duration::of(n, 3600 * NANOS_PER_SEC)
    }

    /// `n` minutes, or `None` past the range.
    #[must_use]
    #[inline]
    pub const fn try_minutes(n: i64) -> Option<Duration> {
        Duration::of(n, 60 * NANOS_PER_SEC)
    }

    /// `n` seconds, or `None` past the range.
    #[must_use]
    #[inline]
    pub const fn try_seconds(n: i64) -> Option<Duration> {
        Duration::of(n, NANOS_PER_SEC)
    }

    /// `n` milliseconds, or `None` past the range (never, as the range is `i64` milliseconds).
    #[must_use]
    #[inline]
    pub const fn try_milliseconds(n: i64) -> Option<Duration> {
        Duration::of(n, 1_000_000)
    }

    /// `whole` seconds a unit long, `n` of them, for a count that is an `i32`. RCA-EXC-2: `n` is at most 2^31 and a unit
    /// is at most a week (604 800 s), so the product is under 1.4e15, inside `i64` and inside the range `of` allows
    /// (`i64::MAX / 1000`, 9.2e15). It cannot overflow, and no check is needed.
    #[inline]
    const fn of_i32(n: i32, secs_per_unit: i64) -> Duration {
        Duration(SignedDuration::from_secs((n as i64) * secs_per_unit))
    }

    /// `n` weeks. The count is an `i32`, so no count is out of range; for an `i64` count use [`Duration::try_weeks`].
    #[must_use]
    #[inline]
    pub const fn weeks(n: i32) -> Duration {
        Duration::of_i32(n, 7 * 86_400)
    }

    /// `n` days. The count is an `i32`, so no count is out of range; for an `i64` count use [`Duration::try_days`].
    #[must_use]
    #[inline]
    pub const fn days(n: i32) -> Duration {
        Duration::of_i32(n, 86_400)
    }

    /// `n` hours. The count is an `i32`, so no count is out of range; for an `i64` count use [`Duration::try_hours`].
    #[must_use]
    #[inline]
    pub const fn hours(n: i32) -> Duration {
        Duration::of_i32(n, 3600)
    }

    /// `n` minutes. The count is an `i32`, so no count is out of range; for an `i64` count use
    /// [`Duration::try_minutes`].
    #[must_use]
    #[inline]
    pub const fn minutes(n: i32) -> Duration {
        Duration::of_i32(n, 60)
    }

    /// `n` seconds. The count is an `i32`, so no count is out of range; for an `i64` count use
    /// [`Duration::try_seconds`].
    #[must_use]
    #[inline]
    pub const fn seconds(n: i32) -> Duration {
        Duration::of_i32(n, 1)
    }

    /// `n` milliseconds: every `i64` count of milliseconds is in range.
    #[must_use]
    #[inline]
    pub const fn milliseconds(n: i64) -> Duration {
        Duration(SignedDuration::from_millis(n))
    }

    /// A whole number of days between two dates. RCA-EXC-2: a `Date` spans under 3.7 million days, so `n * 86_400` is under
    /// 3.2e11, far inside `i64` and inside the range `of` allows.
    #[inline]
    pub(crate) const fn of_date_span_days(n: i64) -> Duration {
        Duration(SignedDuration::from_secs(n * 86_400))
    }

    /// The span between two instants. RCA-EXC-2: two instants in jiff's range are less than 20 000 years (6.3e11 s) apart,
    /// inside the range `of` allows (`i64::MAX / 1000` seconds, 9.2e15), so the span is a valid `Duration` as it is.
    #[inline]
    pub(crate) const fn of_instant_span(span: SignedDuration) -> Duration {
        Duration(span)
    }

    /// `n` microseconds.
    #[must_use]
    #[inline]
    pub const fn microseconds(n: i64) -> Duration {
        Duration(SignedDuration::from_micros(n))
    }

    /// `n` nanoseconds.
    #[must_use]
    #[inline]
    pub const fn nanoseconds(n: i64) -> Duration {
        Duration(SignedDuration::from_nanos(n))
    }

    /// Whole weeks, toward zero.
    #[must_use]
    #[inline]
    pub const fn num_weeks(&self) -> i64 {
        self.num_seconds() / (7 * 86_400)
    }

    /// Whole days, toward zero.
    #[must_use]
    #[inline]
    pub const fn num_days(&self) -> i64 {
        self.num_seconds() / 86_400
    }

    /// Whole hours, toward zero.
    #[must_use]
    #[inline]
    pub const fn num_hours(&self) -> i64 {
        self.num_seconds() / 3600
    }

    /// Whole minutes, toward zero.
    #[must_use]
    #[inline]
    pub const fn num_minutes(&self) -> i64 {
        self.num_seconds() / 60
    }

    /// Whole seconds, toward zero.
    #[must_use]
    #[inline]
    pub const fn num_seconds(&self) -> i64 {
        self.0.as_secs()
    }

    /// Whole milliseconds, toward zero.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)] // within ±i64::MAX milliseconds by construction
    #[inline]
    pub const fn num_milliseconds(&self) -> i64 {
        self.0.as_millis() as i64
    }

    /// Whole microseconds, toward zero, or `None` past `i64`.
    #[must_use]
    #[inline]
    pub fn num_microseconds(&self) -> Option<i64> {
        i64::try_from(self.0.as_micros()).ok()
    }

    /// Nanoseconds, or `None` past `i64`.
    #[must_use]
    #[inline]
    pub fn num_nanoseconds(&self) -> Option<i64> {
        i64::try_from(self.0.as_nanos()).ok()
    }

    /// The part below a second, with the duration's sign.
    #[must_use]
    #[inline]
    pub const fn subsec_nanos(&self) -> i32 {
        self.0.subsec_nanos()
    }

    /// Fractional seconds.
    #[must_use]
    #[inline]
    pub fn as_seconds_f64(self) -> f64 {
        self.0.as_secs_f64()
    }

    /// Whether this is of no length.
    #[must_use]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// The length, without its sign.
    #[must_use]
    #[inline]
    pub const fn abs(&self) -> Duration {
        Duration(self.0.abs())
    }

    /// The sum, or `None` past the range.
    #[must_use]
    #[inline]
    pub fn checked_add(&self, rhs: &Duration) -> Option<Duration> {
        Duration::from_total_nanos(self.total_nanos() + rhs.total_nanos())
    }

    /// The difference, or `None` past the range.
    #[must_use]
    #[inline]
    pub fn checked_sub(&self, rhs: &Duration) -> Option<Duration> {
        Duration::from_total_nanos(self.total_nanos() - rhs.total_nanos())
    }

    /// The negation, or `None` past the range.
    #[must_use]
    #[inline]
    pub fn checked_neg(&self) -> Option<Duration> {
        self.0.checked_neg().map(Duration)
    }

    /// Times `rhs`, or `None` past the range.
    #[must_use]
    #[inline]
    pub fn checked_mul(&self, rhs: i32) -> Option<Duration> {
        Duration::from_total_nanos(self.total_nanos().checked_mul(i128::from(rhs))?)
    }

    /// The quotient, or `None` for a divisor of zero. Computed as before, on whole seconds rounded down and the
    /// non-negative nanosecond rest, so a quotient below a nanosecond rounds as it always did.
    #[must_use]
    #[inline]
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
    #[inline]
    pub fn to_std(&self) -> Result<std::time::Duration, OutOfRangeError> {
        std::time::Duration::try_from(self.0).map_err(|_| OutOfRangeError(()))
    }

    /// From a standard library duration.
    ///
    /// # Errors
    ///
    /// When it is longer than [`Duration::MAX`].
    #[inline]
    pub fn from_std(d: std::time::Duration) -> Result<Duration, OutOfRangeError> {
        i128::try_from(d.as_nanos())
            .ok()
            .and_then(Duration::from_total_nanos)
            .ok_or(OutOfRangeError(()))
    }

    /// The `jiff` duration underneath.
    #[must_use]
    #[inline]
    pub const fn to_jiff(self) -> SignedDuration {
        self.0
    }

    /// From a `jiff` duration, or `None` past the range.
    #[must_use]
    #[inline]
    pub fn from_jiff(d: SignedDuration) -> Option<Duration> {
        Duration::from_total_nanos(d.as_nanos())
    }

    /// Seconds rounded down and a non-negative nanosecond part, as the older representation kept them.
    #[inline]
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

/// As the older type's derived `Debug`: seconds rounded down and a non-negative nanosecond part.
impl fmt::Debug for Duration {
    #[inline]
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
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (secs, _) = self.floor_parts();
        let (abs, sign) = if secs < 0 {
            let Some(abs) = self.checked_neg() else {
                return f.write_str("out of range");
            };
            (abs, "-")
        } else {
            (*self, "")
        };
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
    #[inline]
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
    #[inline]
    pub const fn new(n: u32) -> Months {
        Months(n)
    }

    /// The count.
    #[must_use]
    #[inline]
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
        assert_eq!(Duration::MAX.checked_neg(), Some(Duration::MIN));
        // The range is symmetric (`MIN` is `-MAX`), so negation never leaves it.
        assert_eq!(Duration::MIN.checked_neg(), Some(Duration::MAX));
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

    #[test]
    fn constructors_counts_checked_arithmetic_and_operators_agree() {
        let d = [
            Duration::weeks(2),
            Duration::days(3),
            Duration::hours(4),
            Duration::minutes(5),
            Duration::seconds(6),
            Duration::milliseconds(7),
            Duration::microseconds(8),
            Duration::nanoseconds(9),
        ]
        .iter()
        .try_fold(Duration::zero(), |sum, part| sum.checked_add(part))
        .unwrap();
        assert_eq!(d.num_weeks(), 2);
        assert_eq!(d.num_days(), 17);
        assert_eq!(d.num_hours(), 412);
        assert_eq!(d.num_minutes(), 24_725);
        assert_eq!(d.num_seconds(), 1_483_506);
        assert_eq!(d.num_milliseconds(), 1_483_506_007);
        assert_eq!(d.num_microseconds(), Some(1_483_506_007_008));
        assert_eq!(d.num_nanoseconds(), Some(1_483_506_007_008_009));
        assert_eq!(d.subsec_nanos(), 7_008_009);
        assert!((d.as_seconds_f64() - 1_483_506.007_008_009).abs() < f64::EPSILON);
        assert!(!d.is_zero());
        assert!(Duration::zero().is_zero());
        assert_eq!(d.checked_neg().unwrap().abs(), d);

        assert_eq!(
            Duration::seconds(4).checked_add(&Duration::seconds(3)),
            Some(Duration::seconds(7))
        );
        assert_eq!(
            Duration::seconds(4).checked_sub(&Duration::seconds(3)),
            Some(Duration::seconds(1))
        );
        assert_eq!(
            Duration::seconds(4).checked_mul(3),
            Some(Duration::seconds(12))
        );
        assert_eq!(
            Duration::seconds(5).checked_div(2),
            Some(Duration::milliseconds(2500))
        );
        assert_eq!(
            Duration::milliseconds(-1500).checked_div(2),
            Some(Duration::milliseconds(-750))
        );
        assert_eq!(Duration::seconds(1).checked_div(0), None);
        assert_eq!(Duration::MAX.checked_mul(2), None);
        assert_eq!(Duration::MIN.checked_sub(&Duration::nanoseconds(1)), None);

        let assigned = Duration::seconds(10)
            .checked_add(&Duration::seconds(2))
            .and_then(|d| d.checked_sub(&Duration::seconds(1)));
        assert_eq!(assigned, Some(Duration::seconds(11)));
        assert_eq!(
            Duration::seconds(3).checked_mul(4),
            Some(Duration::seconds(12))
        );
        assert_eq!(
            Duration::seconds(9).checked_div(3),
            Some(Duration::seconds(3))
        );

        assert_eq!(
            Duration::from_std(std::time::Duration::from_millis(1500)).unwrap(),
            Duration::milliseconds(1500)
        );
        assert!(Duration::from_std(std::time::Duration::MAX).is_err());
        assert_eq!(
            Duration::from_jiff(Duration::seconds(4).to_jiff()),
            Some(Duration::seconds(4))
        );
        assert_eq!(Days::new(8), Days(8));
        assert_eq!(Months::new(13).as_u32(), 13);
    }

    #[test]
    fn every_try_constructor_refuses_values_past_the_duration_range() {
        assert_eq!(Duration::try_weeks(1), Some(Duration::weeks(1)));
        assert_eq!(Duration::try_days(1), Some(Duration::days(1)));
        assert_eq!(Duration::try_hours(1), Some(Duration::hours(1)));
        assert_eq!(Duration::try_minutes(1), Some(Duration::minutes(1)));
        assert_eq!(Duration::try_seconds(1), Some(Duration::seconds(1)));
        assert_eq!(Duration::try_milliseconds(i64::MAX), Some(Duration::MAX));
        for constructor in [
            Duration::try_weeks,
            Duration::try_days,
            Duration::try_hours,
            Duration::try_minutes,
            Duration::try_seconds,
        ] {
            assert!(constructor(i64::MAX).is_none());
            assert!(constructor(i64::MIN).is_none());
        }
        assert_eq!(Duration::MAX.num_microseconds(), None);
        assert_eq!(Duration::MAX.num_nanoseconds(), None);
        assert_eq!(Duration::milliseconds(1).to_string(), "PT0.001S");
    }
}

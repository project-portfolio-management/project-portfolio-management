//! The day of the week, written `Mon` … `Sun` (`Debug` and `Display`) as before.

use core::fmt;

/// A day of the week.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Weekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}

impl Weekday {
    const ALL: [Weekday; 7] = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];

    /// Monday is 0, Sunday 6.
    #[must_use]
    #[inline]
    pub const fn num_days_from_monday(self) -> u32 {
        self as u32
    }

    /// Sunday is 0, Saturday 6.
    #[must_use]
    #[inline]
    pub const fn num_days_from_sunday(self) -> u32 {
        (self as u32 + 1) % 7
    }

    /// Monday is 1, Sunday 7.
    #[must_use]
    #[inline]
    pub const fn number_from_monday(self) -> u32 {
        self as u32 + 1
    }

    /// Sunday is 1, Saturday 7.
    #[must_use]
    #[inline]
    pub const fn number_from_sunday(self) -> u32 {
        self.num_days_from_sunday() + 1
    }

    /// The next day.
    #[must_use]
    #[inline]
    pub const fn succ(self) -> Weekday {
        Self::ALL[(self as usize + 1) % 7]
    }

    /// The day before.
    #[must_use]
    #[inline]
    pub const fn pred(self) -> Weekday {
        Self::ALL[(self as usize + 6) % 7]
    }

    /// Monday is 0; `n` must be 0..=6.
    #[inline]
    pub(crate) fn from_monday_offset(n: i64) -> Weekday {
        Self::ALL[usize::try_from(n).unwrap_or(0) % 7]
    }

    /// The same day as jiff's weekday.
    #[inline]
    #[must_use]
    pub fn to_jiff(self) -> jiff::civil::Weekday {
        // `self as i8` is 0..=6, always a valid offset.
        jiff::civil::Weekday::from_monday_zero_offset(self as i8)
            .unwrap_or(jiff::civil::Weekday::Monday)
    }
}

impl fmt::Display for Weekday {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_number_neighbour_conversion_and_name_is_pinned() {
        let days = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        let names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        for (index, day) in days.into_iter().enumerate() {
            assert_eq!(day.num_days_from_monday(), index as u32);
            assert_eq!(day.number_from_monday(), index as u32 + 1);
            assert_eq!(day.num_days_from_sunday(), (index as u32 + 1) % 7);
            assert_eq!(day.number_from_sunday(), (index as u32 + 1) % 7 + 1);
            assert_eq!(day.succ().pred(), day);
            assert_eq!(day.pred().succ(), day);
            assert_eq!(Weekday::from_monday_offset(index as i64), day);
            assert_eq!(day.to_string(), names[index]);
            assert_eq!(day.to_jiff().to_monday_zero_offset(), index as i8);
        }
        assert_eq!(Weekday::from_monday_offset(-1), Weekday::Mon);
        assert_eq!(Weekday::from_monday_offset(7), Weekday::Mon);
        assert_eq!(format!("{:>5}", Weekday::Wed), "  Wed");
    }
}

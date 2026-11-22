//! Totals that have a declared bound (service spec, "Figures: bounds and fallbacks", FIG-1 to FIG-3).
//!
//! A total of money, minutes, milliseconds or counts that would leave its type is reported as the declared bound, not
//! wrapped and not a panic. The bound is written as `checked_*` plus `unwrap_or(BOUND)` (spec/rust-checked-arithmetic,
//! RCA-INT-4, RCA-NONE-2), here once, so the declaration is cited in one place and tested in one place. Code that can
//! refuse an out-of-range figure instead of bounding it should use `checked_*` and `ok_or` directly.

/// Arithmetic whose overflow has a declared bound (FIG-1 to FIG-3).
pub trait Figure: Copy {
    /// `self + other`; past the type's range, the bound on that side (FIG-1).
    #[must_use]
    fn total_add(self, other: Self) -> Self;
    /// `self - other`; past the type's range, the bound on that side (FIG-1); an unsigned count below zero is zero (FIG-2).
    #[must_use]
    fn total_sub(self, other: Self) -> Self;
    /// `self * other`; past the type's range, the bound on the side of the product's sign (FIG-1).
    #[must_use]
    fn total_mul(self, other: Self) -> Self;
}

macro_rules! signed {
    ($($t:ty),*) => {$(
        impl Figure for $t {
            fn total_add(self, other: Self) -> Self {
                // FIG-1: a total past the type's range is reported at the bound on its side.
                self.checked_add(other).unwrap_or(if other < 0 { Self::MIN } else { Self::MAX })
            }
            fn total_sub(self, other: Self) -> Self {
                // FIG-1: as above.
                self.checked_sub(other).unwrap_or(if other < 0 { Self::MAX } else { Self::MIN })
            }
            fn total_mul(self, other: Self) -> Self {
                // FIG-1: as above; a product of mixed signs is below zero.
                self.checked_mul(other)
                    .unwrap_or(if (self < 0) == (other < 0) { Self::MAX } else { Self::MIN })
            }
        }
    )*};
}

macro_rules! unsigned {
    ($($t:ty),*) => {$(
        impl Figure for $t {
            fn total_add(self, other: Self) -> Self {
                // FIG-1: a total past the type's range is reported at the maximum.
                self.checked_add(other).unwrap_or(Self::MAX)
            }
            fn total_sub(self, other: Self) -> Self {
                // FIG-2: a count taken from a smaller count is zero (nothing remains), never negative.
                self.checked_sub(other).unwrap_or(0)
            }
            fn total_mul(self, other: Self) -> Self {
                // FIG-1: as for addition.
                self.checked_mul(other).unwrap_or(Self::MAX)
            }
        }
    )*};
}

signed!(i32, i64);
unsigned!(u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::Figure;

    #[test]
    fn a_total_past_the_range_is_at_the_bound_on_its_side() {
        assert_eq!(i64::MAX.total_add(1), i64::MAX);
        assert_eq!(i64::MIN.total_add(-1), i64::MIN);
        assert_eq!(i64::MIN.total_sub(1), i64::MIN);
        assert_eq!(i64::MAX.total_sub(-1), i64::MAX);
        assert_eq!(i64::MAX.total_mul(2), i64::MAX);
        assert_eq!(i64::MAX.total_mul(-2), i64::MIN);
        assert_eq!(i64::MIN.total_mul(-1), i64::MAX);
        assert_eq!(u64::MAX.total_add(1), u64::MAX);
        assert_eq!(usize::MAX.total_mul(2), usize::MAX);
    }

    #[test]
    fn a_total_inside_the_range_is_exact() {
        assert_eq!(2_i64.total_add(3), 5);
        assert_eq!((-2_i64).total_sub(3), -5);
        assert_eq!(6_i32.total_mul(-7), -42);
        assert_eq!(7_usize.total_sub(3), 4);
    }

    #[test]
    fn a_count_taken_from_a_smaller_count_is_zero() {
        assert_eq!(3_usize.total_sub(7), 0);
        assert_eq!(0_u64.total_sub(1), 0);
    }
}

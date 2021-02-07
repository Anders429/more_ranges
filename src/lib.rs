#![no_std]

#[cfg(test)]
#[macro_use]
extern crate claim;

use core::ops::{
    Bound::{self, Excluded, Included, Unbounded},
    RangeBounds,
};

/// A range only bounded exclusively below.
///
/// The `RangeFromExclusive` contains all values with `x > start`.
///
/// *Note*: Overflow in the [`Iterator`] implementation (when the contained data type reaches its
/// numerical limit) is allowed to panic, wrap, or saturate. This behavior is defined by the
/// implementation of the [`Step`] trait. For primitive integers, this follows the normal rules, and
/// respects the overflow checks profile (panic in debug, wrap in release). Note also that overflow
/// happens earlier than you might assume: the overflow happens in the call to next that yields the
/// maximum value, as the range must be set to a state to yield the next value.
///
/// [`Iterator`]: core::iter::Iterator
/// [`Step`]: core::iter::Step
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusive<Idx> {
    /// The lower bound of the range (exclusive).
    pub start: Idx,
}

impl<T> RangeBounds<T> for RangeFromExclusive<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<'a, T> RangeBounds<T> for RangeFromExclusive<&'a T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(self.start)
    }
    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

/// A range bounded exclusively below and inclusively above.
///
/// The `RangeFromExclusiveToInclusive` contains all values with `x > start` and `x <= end`. It is
/// empty unless `start < end`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusiveToInclusive<Idx> {
    /// The lower bound of the range (exclusive).
    pub start: Idx,
    /// The upper bound of the range (inclusive).
    pub end: Idx,
}

impl<T> RangeBounds<T> for RangeFromExclusiveToInclusive<T> {
    #[inline]
    #[must_use]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    #[inline]
    #[must_use]
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

impl<'a, T> RangeBounds<T> for RangeFromExclusiveToInclusive<&'a T> {
    #[inline]
    #[must_use]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(self.start)
    }
    #[inline]
    #[must_use]
    fn end_bound(&self) -> Bound<&T> {
        Included(self.end)
    }
}

/// A range bounded exclusively below and above.
///
/// The `RangeFromExclusiveToExclusive` contains all values with `x > start` and x < end`. It is
/// empty unless `start < end + 1`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusiveToExclusive<Idx> {
    /// The lower bound of the range (exclusive).
    pub start: Idx,
    /// The upper bound of the range (exclusive).
    pub end: Idx,
}

impl<T> RangeBounds<T> for RangeFromExclusiveToExclusive<T> {
    #[inline]
    #[must_use]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    #[inline]
    #[must_use]
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

impl<'a, T> RangeBounds<T> for RangeFromExclusiveToExclusive<&'a T> {
    #[inline]
    #[must_use]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(self.start)
    }
    #[inline]
    #[must_use]
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    };
    use {RangeFromExclusive, RangeFromExclusiveToExclusive, RangeFromExclusiveToInclusive};

    #[test]
    fn range_from_exclusive_range_bounds() {
        let range = RangeFromExclusive { start: 1 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Unbounded);
    }

    #[test]
    fn range_from_exclusive_range_bounds_borrowed() {
        let range = RangeFromExclusive { start: &1 };

        assert_matches!(RangeBounds::<usize>::start_bound(&range), Excluded(1));
        assert_matches!(RangeBounds::<usize>::end_bound(&range), Unbounded);
    }

    #[test]
    fn range_from_exclusive_to_exclusive_range_bounds() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 3 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Excluded(3));
    }

    #[test]
    fn range_from_exclusive_to_exclusive_range_bounds_borrowed() {
        let range = RangeFromExclusiveToExclusive { start: &1, end: &3 };

        assert_matches!(RangeBounds::<usize>::start_bound(&range), Excluded(1));
        assert_matches!(RangeBounds::<usize>::end_bound(&range), Excluded(3));
    }

    #[test]
    fn range_from_exclusive_to_inclusive_range_bounds() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 3 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Included(3));
    }

    #[test]
    fn range_from_exclusive_to_inclusive_range_bounds_borrowed() {
        let range = RangeFromExclusiveToInclusive { start: &1, end: &3 };

        assert_matches!(RangeBounds::<usize>::start_bound(&range), Excluded(1));
        assert_matches!(RangeBounds::<usize>::end_bound(&range), Included(3));
    }
}

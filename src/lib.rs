#![no_std]

#[cfg(test)]
#[macro_use]
extern crate claim;

use core::ops::{
    Bound::{self, Excluded, Included, Unbounded},
    RangeBounds,
};

pub struct RangeFromExclusive<Idx> {
    pub start: Idx,
}

impl<T> RangeBounds<T> for RangeFromExclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

pub struct RangeFromExclusiveToInclusive<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<T> RangeBounds<T> for RangeFromExclusiveToInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

pub struct RangeFromExclusiveToExclusive<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<T> RangeBounds<T> for RangeFromExclusiveToExclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::{RangeFromExclusive, RangeFromExclusiveToExclusive, RangeFromExclusiveToInclusive};
    use core::ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    };

    #[test]
    fn range_from_exclusive_range_bounds() {
        let range = RangeFromExclusive { start: 1 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Unbounded);
    }

    #[test]
    fn range_from_exclusive_to_exclusive_range_bounds() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 3 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Excluded(3));
    }

    #[test]
    fn range_from_exclusive_to_inclusive_range_bounds() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 3 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Included(3));
    }
}

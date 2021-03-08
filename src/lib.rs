//! Range types that are not included in the standard library.
//!
//! Specifically, these are ranges which are bounded exclusively below.
//!
//! These ranges currently do not function as [`Iterator`]s and cannot be used in indexing.
//!
//! # Example
//! While each range type in the standard library is either bounded inclusively below or unbounded
//! below, each range type provided in this crate is bounded exclusively below. Compare, for
//! example, [`RangeFrom`] with [`RangeFromExclusive`].
//!
//! ```
//! use std::ops::{Bound, RangeBounds, RangeFrom};
//! use more_ranges::RangeFromExclusive;
//!
//! let range_inclusive = RangeFrom {
//!     start: 1,
//! };
//! let range_exclusive = RangeFromExclusive {
//!     start: 1,
//! };
//!
//! // The inclusive range is inclusively bound.
//! assert_eq!(range_inclusive.start_bound(), Bound::Included(&1));
//!
//! // Contrastingly, the exclusive range is exclusively bound.
//! assert_eq!(range_exclusive.start_bound(), Bound::Excluded(&1));
//! ```
//!
//! [`Iterator`]: core::iter::Iterator
//! [`RangeFrom`]: core::ops::RangeFrom
#![cfg_attr(
    all(rustc_channel_nightly, impl_iterator),
    feature(step_trait, step_trait_ext, unchecked_math)
)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate claim;

use core::ops::{
    Bound::{self, Excluded, Included, Unbounded},
    RangeBounds,
};
#[cfg(impl_iterator)]
use core::{
    iter::{FusedIterator, Step},
    mem,
};

/// A range only bounded exclusively below.
///
/// The `RangeFromExclusive` contains all values with `x > start`.
///
/// *Note*: Overflow in the [`Iterator`] implementation (when the contained data type reaches its
/// numerical limit) is allowed to panic, wrap, or saturate. This behavior is defined by the
/// implementation of the [`Step`] trait. For primitive integers, this follows the normal rules, and
/// respects the overflow checks profile (panic in debug, wrap in release).
///
/// # Example
/// `RangeFromExclusive`s can be created directly, as follows:
///
/// ```
/// use more_ranges::RangeFromExclusive;
///
/// let range = RangeFromExclusive {
///     start: 1,
/// };
/// ```
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

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> Iterator for RangeFromExclusive<T>
where
    T: Step,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.start = Step::forward(self.start.clone(), 1);
        Some(self.start.clone())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<T> {
        self.start = Step::forward(self.start.clone(), n + 1);
        Some(self.start.clone())
    }

    #[inline]
    fn min(mut self) -> Option<T> {
        self.next()
    }
}

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> FusedIterator for RangeFromExclusive<T> where T: Step {}

/// A range bounded exclusively below and inclusively above.
///
/// The `RangeFromExclusiveToInclusive` contains all values with `x > start` and `x <= end`. It is
/// empty unless `start < end`.
///
/// # Example
/// `RangeFromExclusiveToInclusive`s can be created directly, as follows:
///
/// ```
/// use more_ranges::RangeFromExclusiveToInclusive;
///
/// let range = RangeFromExclusiveToInclusive {
///     start: 1,
///     end: 4,
/// };
/// ```
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

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> Iterator for RangeFromExclusiveToInclusive<T>
where
    T: Step,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.start < self.end {
            self.start = unsafe {
                // SAFETY: Just checked that start < end, which means stepping forward by one will
                // always succeed.
                Step::forward_unchecked(self.start.clone(), 1)
            };
            Some(self.start.clone())
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.start < self.end {
            let hint = Step::steps_between(&self.start, &self.end);
            (hint.unwrap_or(usize::MAX), hint)
        } else {
            (0, Some(0))
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<T> {
        if self.start == self.end {
            return None;
        }

        if let Some(plus_n) = Step::forward_checked(self.start.clone(), n) {
            if plus_n < self.end {
                self.start = unsafe {
                    // SAFETY: Just checked that plus_n < end, which means stepping forward by one
                    // will always succeed.
                    Step::forward_unchecked(plus_n.clone(), 1)
                };
                return Some(self.start.clone());
            }
        }

        self.start = self.end.clone();
        None
    }

    #[inline]
    fn last(mut self) -> Option<T> {
        self.next_back()
    }

    #[inline]
    fn min(mut self) -> Option<T> {
        self.next()
    }

    #[inline]
    fn max(mut self) -> Option<T> {
        self.next_back()
    }
}

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> DoubleEndedIterator for RangeFromExclusiveToInclusive<T>
where
    T: Step,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.start < self.end {
            let n = unsafe {
                // SAFETY: Just checked that start < end, which means stepping backward by one will
                // always succeed.
                Step::backward_unchecked(self.end.clone(), 1)
            };
            Some(mem::replace(&mut self.end, n))
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<T> {
        if self.start >= self.end {
            return None;
        }

        if let Some(minus_n) = Step::backward_checked(self.end.clone(), n) {
            if self.start < minus_n {
                self.end = unsafe {
                    // SAFETY: Just checked that start < minus_n, which means stepping backward by
                    // one will always succeed.
                    Step::backward_unchecked(minus_n.clone(), 1)
                };
                return Some(minus_n);
            }
        }

        self.end = self.start.clone();
        None
    }
}

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> FusedIterator for RangeFromExclusiveToInclusive<T> where T: Step {}

/// A range bounded exclusively below and above.
///
/// The `RangeFromExclusiveToExclusive` contains all values with `x > start` and x < end`. It is
/// empty unless `start < end + 1`.
///
/// # Example
/// `RangeFromExclusiveToExclusive`s can be created directly, as follows:
///
/// ```
/// use more_ranges::RangeFromExclusiveToExclusive;
///
/// let range = RangeFromExclusiveToExclusive {
///     start: 1,
///     end: 4,
/// };
/// ```
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

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> Iterator for RangeFromExclusiveToExclusive<T>
where
    T: Step,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if Step::steps_between(&self.start, &self.end) > Some(1) {
            self.start = unsafe {
                // SAFETY: Just checked that start < end, which means stepping forward by one will
                // always succeed.
                Step::forward_unchecked(self.start.clone(), 1)
            };
            Some(self.start.clone())
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(hint) = Step::steps_between(&self.start, &self.end) {
            if hint > 1 {
                (hint - 1, Some(hint - 1))
            } else {
                (0, Some(0))
            }
        } else if self.start >= self.end {
            (0, Some(0))
        } else {
            (usize::MAX, None)
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<T> {
        if Step::steps_between(&self.start, &self.end) <= Some(1) {
            return None;
        }

        if let Some(plus_n) = Step::forward_checked(self.start.clone(), n) {
            if Step::steps_between(&plus_n, &self.end) > Some(1) {
                self.start = unsafe {
                    // SAFETY: Just checked that plus_n < end, which means stepping forward by one
                    // will always succeed.
                    Step::forward_unchecked(plus_n.clone(), 1)
                };
                return Some(self.start.clone());
            }
        }

        self.start = unsafe {
            // SAFETY: It is guaranteed that end can step backwards, since there are at least 2
            // steps between end and start.
            Step::backward_unchecked(self.end.clone(), 1)
        };
        None
    }

    #[inline]
    fn last(mut self) -> Option<T> {
        self.next_back()
    }

    #[inline]
    fn min(mut self) -> Option<T> {
        self.next()
    }

    #[inline]
    fn max(mut self) -> Option<T> {
        self.next_back()
    }
}

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> DoubleEndedIterator for RangeFromExclusiveToExclusive<T>
where
    T: Step,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if Step::steps_between(&self.start, &self.end) > Some(1) {
            self.end = unsafe {
                // SAFETY: Just checked that there are steps between start and end, which means
                // stepping backward by one will always succeed.
                Step::backward_unchecked(self.end.clone(), 1)
            };
            Some(self.end.clone())
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<T> {
        if Step::steps_between(&self.start, &self.end) <= Some(1) {
            return None;
        }

        if let Some(minus_n) = Step::backward_checked(self.end.clone(), n) {
            if Step::steps_between(&self.start, &minus_n) > Some(1) {
                self.end = unsafe {
                    // SAFETY: Just checked that start < minus_n, which means stepping backward by
                    // one will always succeed.
                    Step::backward_unchecked(minus_n.clone(), 1)
                };
                return Some(self.end.clone());
            }
        }

        self.end = unsafe {
            // SAFETY: It is guaranteed that start can step forwards, since there are at least 2
            // steps between end and start.
            Step::forward_unchecked(self.start.clone(), 1)
        };
        None
    }
}

#[cfg(impl_iterator)]
#[cfg_attr(
    feature = "doc_item",
    doc_item::docbox(
        content = "Only available on <b><code>nightly</code></b>.",
        class = "nightly"
    )
)]
impl<T> FusedIterator for RangeFromExclusiveToExclusive<T> where T: Step {}

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

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_iterator_next() {
        let mut range = RangeFromExclusive { start: 1 };

        assert_some_eq!(range.next(), 2);
        assert_some_eq!(range.next(), 3);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_iterator_size_hint() {
        let range = RangeFromExclusive { start: 1 };

        assert_eq!(range.size_hint(), (usize::MAX, None));
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_iterator_nth() {
        let mut range = RangeFromExclusive { start: 1 };

        assert_some_eq!(range.nth(0), 2);
        assert_some_eq!(range.nth(5), 8);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_iterator_min() {
        assert_some_eq!(RangeFromExclusive { start: 1 }.min(), 2);
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

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_iterator_next() {
        let mut range = RangeFromExclusiveToInclusive { start: 1, end: 3 };

        assert_some_eq!(range.next(), 2);
        assert_some_eq!(range.next(), 3);
        assert_none!(range.next());
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_iterator_size_hint() {
        assert_eq!(
            RangeFromExclusiveToInclusive { start: 1, end: 3 }.size_hint(),
            (2, Some(2))
        );
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_iterator_nth() {
        let mut range = RangeFromExclusiveToInclusive { start: 1, end: 5 };

        assert_some_eq!(range.nth(0), 2);
        assert_some_eq!(range.nth(2), 5);
        assert_none!(range.nth(0));
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_iterator_last() {
        assert_some_eq!(RangeFromExclusiveToInclusive { start: 1, end: 3 }.last(), 3);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_iterator_min() {
        assert_some_eq!(RangeFromExclusiveToInclusive { start: 1, end: 3 }.min(), 2);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_iterator_max() {
        assert_some_eq!(RangeFromExclusiveToInclusive { start: 1, end: 3 }.max(), 3);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_double_ended_iterator_next_back() {
        let mut range = RangeFromExclusiveToInclusive { start: 1, end: 3 };

        assert_some_eq!(range.next_back(), 3);
        assert_some_eq!(range.next_back(), 2);
        assert_none!(range.next_back());
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_inclusive_double_ended_iterator_nth_back() {
        let mut range = RangeFromExclusiveToInclusive { start: 1, end: 6 };

        assert_some_eq!(range.nth_back(0), 6);
        assert_some_eq!(range.nth_back(3), 2);
        assert_none!(range.nth_back(0));
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

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_iterator_next() {
        let mut range = RangeFromExclusiveToExclusive { start: 1, end: 4 };

        assert_some_eq!(range.next(), 2);
        assert_some_eq!(range.next(), 3);
        assert_none!(range.next());
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_iterator_size_hint() {
        assert_eq!(
            RangeFromExclusiveToExclusive { start: 1, end: 3 }.size_hint(),
            (1, Some(1))
        );
        assert_eq!(
            RangeFromExclusiveToExclusive { start: 1, end: 2 }.size_hint(),
            (0, Some(0))
        );
        assert_eq!(
            RangeFromExclusiveToExclusive { start: 1, end: 1 }.size_hint(),
            (0, Some(0))
        );
        assert_eq!(
            RangeFromExclusiveToExclusive { start: 3, end: 1 }.size_hint(),
            (0, Some(0))
        );
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_iterator_nth() {
        let mut range = RangeFromExclusiveToExclusive { start: 1, end: 6 };

        assert_some_eq!(range.nth(0), 2);
        assert_some_eq!(range.nth(2), 5);
        assert_none!(range.nth(0));
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_iterator_last() {
        assert_some_eq!(RangeFromExclusiveToExclusive { start: 1, end: 4 }.last(), 3);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_iterator_min() {
        assert_some_eq!(RangeFromExclusiveToExclusive { start: 1, end: 3 }.min(), 2);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_iterator_max() {
        assert_some_eq!(RangeFromExclusiveToExclusive { start: 1, end: 4 }.max(), 3);
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_double_ended_iterator_next_back() {
        let mut range = RangeFromExclusiveToExclusive { start: 1, end: 4 };

        assert_some_eq!(range.next_back(), 3);
        assert_some_eq!(range.next_back(), 2);
        assert_none!(range.next_back());
    }

    #[cfg(impl_iterator)]
    #[test]
    fn range_from_exclusive_to_exclusive_double_ended_iterator_nth_back() {
        let mut range = RangeFromExclusiveToExclusive { start: 1, end: 7 };

        assert_some_eq!(range.nth_back(0), 6);
        assert_some_eq!(range.nth_back(3), 2);
        assert_none!(range.nth_back(0));
    }
}

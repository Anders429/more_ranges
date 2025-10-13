use core::ops::{
    Bound::{self, Excluded, Included},
    RangeBounds, RangeInclusive,
};

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
pub struct RangeFromExclusiveToInclusive<T> {
    /// The lower bound of the range (exclusive).
    pub start: T,
    /// The upper bound of the range (inclusive).
    pub end: T,
}

impl<T> RangeBounds<T> for RangeFromExclusiveToInclusive<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

impl<'a, T> RangeBounds<T> for RangeFromExclusiveToInclusive<&'a T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(self.start)
    }
    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Included(self.end)
    }
}

impl<T> IntoIterator for RangeFromExclusiveToInclusive<T>
where
    RangeInclusive<T>: Iterator<Item = T>,
{
    type IntoIter = IterRangeFromExclusiveToInclusive<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        let mut inner = RangeInclusive::new(self.start, self.end);
        // Advance by one so we don't include the first value.
        inner.next();

        IterRangeFromExclusiveToInclusive { inner }
    }
}

pub struct IterRangeFromExclusiveToInclusive<T> {
    inner: RangeInclusive<T>,
}

impl<T> Iterator for IterRangeFromExclusiveToInclusive<T>
where
    RangeInclusive<T>: Iterator<Item = T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n)
    }

    #[inline]
    fn last(self) -> Option<T> {
        self.inner.last()
    }

    #[inline]
    fn min(self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.min()
    }

    #[inline]
    fn max(self) -> Option<Self::Item>
    where
        T: Ord,
    {
        self.inner.max()
    }

    #[inline]
    fn is_sorted(self) -> bool
    where
        T: PartialOrd,
    {
        self.inner.is_sorted()
    }
}

#[cfg(test)]
mod tests {
    use super::RangeFromExclusiveToInclusive;
    use claims::{assert_matches, assert_none, assert_some_eq};
    use core::ops::{
        Bound::{Excluded, Included},
        RangeBounds,
    };

    #[test]
    fn range_bounds() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 3 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Included(3));
    }

    #[test]
    fn range_bounds_borrowed() {
        let range = RangeFromExclusiveToInclusive { start: &1, end: &3 };

        assert_matches!(RangeBounds::<usize>::start_bound(&range), Excluded(1));
        assert_matches!(RangeBounds::<usize>::end_bound(&range), Included(3));
    }

    #[test]
    fn iter_next() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 3 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.next(), 2);
        assert_some_eq!(iter.next(), 3);
        assert_none!(iter.next());
    }

    #[test]
    fn iter_size_hint() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    fn iter_count() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_eq!(iter.count(), 3);
    }

    #[test]
    fn iter_nth() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 250 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.nth(42), 44);
        assert_some_eq!(iter.nth(100), 145);
        assert_some_eq!(iter.nth(104), 250);
        assert_none!(iter.nth(0));
    }

    #[test]
    fn iter_last() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_some_eq!(iter.last(), 4);
    }

    #[test]
    fn iter_last_empty() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 1 };
        let iter = range.into_iter();

        assert_none!(iter.last());
    }

    #[test]
    fn iter_min() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_some_eq!(iter.min(), 2);
    }

    #[test]
    fn iter_max() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_some_eq!(iter.max(), 4);
    }

    #[test]
    fn iter_is_sorted() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert!(iter.is_sorted());
    }
}

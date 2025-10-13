use core::ops::{
    Bound::{self, Excluded},
    Range, RangeBounds,
};

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
pub struct RangeFromExclusiveToExclusive<T> {
    /// The lower bound of the range (exclusive).
    pub start: T,
    /// The upper bound of the range (exclusive).
    pub end: T,
}

impl<T> RangeBounds<T> for RangeFromExclusiveToExclusive<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }
    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

impl<'a, T> RangeBounds<T> for RangeFromExclusiveToExclusive<&'a T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Excluded(self.start)
    }
    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}

impl<T> IntoIterator for RangeFromExclusiveToExclusive<T>
where
    Range<T>: Iterator<Item = T>,
{
    type IntoIter = IterRangeFromExclusiveToExclusive<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        let mut inner = Range {
            start: self.start,
            end: self.end,
        };
        // Advance by one so we don't include the first value.
        inner.next();
        IterRangeFromExclusiveToExclusive { inner }
    }
}

pub struct IterRangeFromExclusiveToExclusive<T> {
    inner: Range<T>,
}

impl<T> Iterator for IterRangeFromExclusiveToExclusive<T>
where
    Range<T>: Iterator<Item = T>,
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
    fn max(self) -> Option<T>
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
    use super::RangeFromExclusiveToExclusive;
    use claims::{assert_matches, assert_none, assert_some_eq};
    use core::ops::{Bound::Excluded, RangeBounds};

    #[test]
    fn range_bounds() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 3 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Excluded(3));
    }

    #[test]
    fn range_bounds_borrowed() {
        let range = RangeFromExclusiveToExclusive { start: &1, end: &3 };

        assert_matches!(RangeBounds::<usize>::start_bound(&range), Excluded(1));
        assert_matches!(RangeBounds::<usize>::end_bound(&range), Excluded(3));
    }

    #[test]
    fn iter_next() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.next(), 2);
        assert_some_eq!(iter.next(), 3);
        assert_none!(iter.next());
    }

    #[test]
    fn iter_size_hint() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_eq!(iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn iter_count() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn iter_nth() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 250 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.nth(42), 44);
        assert_some_eq!(iter.nth(100), 145);
        assert_none!(iter.nth(104));
    }

    #[test]
    fn iter_last() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_some_eq!(iter.last(), 3);
    }

    #[test]
    fn iter_last_empty() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 2 };
        let iter = range.into_iter();

        assert_none!(iter.last());
    }

    #[test]
    fn iter_min() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_some_eq!(iter.min(), 2);
    }

    #[test]
    fn iter_max() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert_some_eq!(iter.max(), 3);
    }

    #[test]
    fn iter_is_sorted() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let iter = range.into_iter();

        assert!(iter.is_sorted());
    }
}

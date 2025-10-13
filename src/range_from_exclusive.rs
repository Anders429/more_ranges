use core::ops::{
    Bound::{self, Excluded, Unbounded},
    RangeBounds, RangeFrom,
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
pub struct RangeFromExclusive<T> {
    /// The lower bound of the range (exclusive).
    pub start: T,
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

impl<T> IntoIterator for RangeFromExclusive<T>
where
    RangeFrom<T>: Iterator<Item = T>,
{
    type IntoIter = IterRangeFromExclusive<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        let mut inner = RangeFrom { start: self.start };
        // Advance by one so we don't include the first value.
        inner.next();

        IterRangeFromExclusive { inner }
    }
}

pub struct IterRangeFromExclusive<T> {
    inner: RangeFrom<T>,
}

impl<T> Iterator for IterRangeFromExclusive<T>
where
    RangeFrom<T>: Iterator<Item = T>,
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
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n)
    }
}

#[cfg(test)]
mod tests {
    use super::RangeFromExclusive;
    use claims::{assert_matches, assert_some_eq};
    use core::ops::{
        Bound::{Excluded, Unbounded},
        RangeBounds,
    };

    #[test]
    fn range_bounds() {
        let range = RangeFromExclusive { start: 1 };

        assert_matches!(range.start_bound(), Excluded(1));
        assert_matches!(range.end_bound(), Unbounded);
    }

    #[test]
    fn range_bounds_borrowed() {
        let range = RangeFromExclusive { start: &1 };

        assert_matches!(RangeBounds::<usize>::start_bound(&range), Excluded(1));
        assert_matches!(RangeBounds::<usize>::end_bound(&range), Unbounded);
    }

    #[test]
    fn iter_next() {
        let range = RangeFromExclusive { start: 1 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.next(), 2);
        assert_some_eq!(iter.next(), 3);
    }

    #[test]
    fn iter_size_hint() {
        let range = RangeFromExclusive { start: 1 };
        let iter = range.into_iter();

        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }

    #[test]
    fn iter_nth() {
        let range = RangeFromExclusive { start: 1 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.nth(42), 44);
        assert_some_eq!(iter.nth(100), 145);
    }
}

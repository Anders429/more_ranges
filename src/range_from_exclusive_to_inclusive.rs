#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{
    iter::FusedIterator,
    ops::{
        Bound::{self, Excluded, Included},
        Index, IndexMut, RangeBounds, RangeInclusive,
    },
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusiveToInclusive<T> {
    /// The lower bound of the range (exclusive).
    pub start: T,
    /// The upper bound of the range (inclusive).
    pub end: T,
}

impl<T> RangeFromExclusiveToInclusive<T> {
    /// Since implementations for many standard library traits for built-in range types rely on
    /// nightly features, we implement those traits here by converting into standard library range
    /// types. This allows these traits to be implemented without enabling nightly features.
    fn into_range_inclusive(self) -> RangeInclusive<T>
    where
        RangeInclusive<T>: Iterator,
    {
        let mut range_inclusive = RangeInclusive::new(self.start, self.end);
        // Advance by one so we don't include the first value.
        range_inclusive.next();
        range_inclusive
    }
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

impl<T> Index<RangeFromExclusiveToInclusive<usize>> for [T] {
    type Output = <[T] as Index<RangeInclusive<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &Self::Output {
        self.index(index.into_range_inclusive())
    }
}

impl<T> IndexMut<RangeFromExclusiveToInclusive<usize>> for [T] {
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_inclusive())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> Index<RangeFromExclusiveToInclusive<usize>> for Vec<T> {
    type Output = <Vec<T> as Index<RangeInclusive<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &Self::Output {
        self.index(index.into_range_inclusive())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> IndexMut<RangeFromExclusiveToInclusive<usize>> for Vec<T> {
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_inclusive())
    }
}

impl Index<RangeFromExclusiveToInclusive<usize>> for str {
    type Output = <str as Index<RangeInclusive<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &Self::Output {
        self.index(index.into_range_inclusive())
    }
}

impl IndexMut<RangeFromExclusiveToInclusive<usize>> for str {
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_inclusive())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl Index<RangeFromExclusiveToInclusive<usize>> for String {
    type Output = <String as Index<RangeInclusive<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &Self::Output {
        self.index(index.into_range_inclusive())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl IndexMut<RangeFromExclusiveToInclusive<usize>> for String {
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_inclusive())
    }
}

impl<T> IntoIterator for RangeFromExclusiveToInclusive<T>
where
    RangeInclusive<T>: Iterator<Item = T>,
{
    type IntoIter = IterRangeFromExclusiveToInclusive<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IterRangeFromExclusiveToInclusive {
            inner: self.into_range_inclusive(),
        }
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

impl<T> DoubleEndedIterator for IterRangeFromExclusiveToInclusive<T>
where
    RangeInclusive<T>: DoubleEndedIterator<Item = T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n)
    }
}

impl<T> FusedIterator for IterRangeFromExclusiveToInclusive<T> where
    RangeInclusive<T>: Iterator<Item = T>
{
}

#[cfg(test)]
mod tests {
    use super::RangeFromExclusiveToInclusive;
    #[cfg(feature = "alloc")]
    use alloc::{borrow::ToOwned, vec};
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
    fn index_slice() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let slice = [0, 1, 2, 3];

        assert_eq!(slice[range], [1, 2]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_vec() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let vec = vec![0, 1, 2, 3];

        assert_eq!(vec[range], [1, 2]);
    }

    #[test]
    fn index_str() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let str = "abcd";

        assert_eq!(&str[range], "bc");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_string() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let string = "abcd".to_owned();

        assert_eq!(&string[range], "bc");
    }

    #[test]
    fn index_mut_slice() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let mut slice = [0, 1, 2, 3];

        slice[range][0] = 4;
        slice[range][1] = 5;

        assert_eq!(slice, [0, 4, 5, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_vec() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let mut vec = vec![0, 1, 2, 3];

        vec[range][0] = 4;
        vec[range][1] = 5;

        assert_eq!(vec, [0, 4, 5, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_str() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let mut string = "abcd".to_owned();
        let str: &mut str = string.as_mut_str();

        str[range].make_ascii_uppercase();

        assert_eq!(string, "aBCd");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_string() {
        let range = RangeFromExclusiveToInclusive { start: 0, end: 2 };
        let mut string = "abcd".to_owned();

        string[range].make_ascii_uppercase();

        assert_eq!(string, "aBCd");
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

    #[test]
    fn iter_next_back() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 3 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.next_back(), 3);
        assert_some_eq!(iter.next_back(), 2);
        assert_none!(iter.next_back());
    }

    #[test]
    fn iter_nth_back() {
        let range = RangeFromExclusiveToInclusive { start: 1, end: 250 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.nth_back(42), 208);
        assert_some_eq!(iter.nth_back(100), 107);
        assert_some_eq!(iter.nth_back(104), 2);
        assert_none!(iter.nth_back(0));
    }
}

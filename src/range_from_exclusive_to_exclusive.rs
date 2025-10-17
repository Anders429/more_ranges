#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{
    iter::FusedIterator,
    ops::{
        Bound::{self, Excluded},
        Index, IndexMut, Range, RangeBounds,
    },
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusiveToExclusive<T> {
    /// The lower bound of the range (exclusive).
    pub start: T,
    /// The upper bound of the range (exclusive).
    pub end: T,
}

impl<T> RangeFromExclusiveToExclusive<T> {
    /// Since implementations for many standard library traits for built-in range types rely on
    /// nightly features, we implement those traits here by converting into standard library range
    /// types. This allows these traits to be implemented without enabling nightly features.
    fn into_range(self) -> Range<T>
    where
        Range<T>: Iterator,
    {
        let mut range = Range {
            start: self.start,
            end: self.end,
        };
        // Advance by one so we don't include the first value.
        range.next();
        range
    }
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

impl<T> Index<RangeFromExclusiveToExclusive<usize>> for [T] {
    type Output = <[T] as Index<Range<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

impl<T> IndexMut<RangeFromExclusiveToExclusive<usize>> for [T] {
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> Index<RangeFromExclusiveToExclusive<usize>> for Vec<T> {
    type Output = <Vec<T> as Index<Range<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> IndexMut<RangeFromExclusiveToExclusive<usize>> for Vec<T> {
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

impl Index<RangeFromExclusiveToExclusive<usize>> for str {
    type Output = <str as Index<Range<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

impl IndexMut<RangeFromExclusiveToExclusive<usize>> for str {
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl Index<RangeFromExclusiveToExclusive<usize>> for String {
    type Output = <String as Index<Range<usize>>>::Output;

    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl IndexMut<RangeFromExclusiveToExclusive<usize>> for String {
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

impl<T> IntoIterator for RangeFromExclusiveToExclusive<T>
where
    Range<T>: Iterator<Item = T>,
{
    type IntoIter = IterRangeFromExclusiveToExclusive<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IterRangeFromExclusiveToExclusive {
            inner: self.into_range(),
        }
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

impl<T> FusedIterator for IterRangeFromExclusiveToExclusive<T> where Range<T>: Iterator<Item = T> {}

#[cfg(test)]
mod tests {
    use super::RangeFromExclusiveToExclusive;
    #[cfg(feature = "alloc")]
    use alloc::{borrow::ToOwned, vec};
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
    fn index_slice() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let slice = [0, 1, 2, 3];

        assert_eq!(slice[range], [1, 2]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_vec() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let vec = vec![0, 1, 2, 3];

        assert_eq!(vec[range], [1, 2]);
    }

    #[test]
    fn index_str() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let str = "abcd";

        assert_eq!(&str[range], "bc");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_string() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let string = "abcd".to_owned();

        assert_eq!(&string[range], "bc");
    }

    #[test]
    fn index_mut_slice() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let mut slice = [0, 1, 2, 3];

        slice[range][0] = 4;
        slice[range][1] = 5;

        assert_eq!(slice, [0, 4, 5, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_vec() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let mut vec = vec![0, 1, 2, 3];

        vec[range][0] = 4;
        vec[range][1] = 5;

        assert_eq!(vec, [0, 4, 5, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_str() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let mut string = "abcd".to_owned();
        let str: &mut str = string.as_mut_str();

        str[range].make_ascii_uppercase();

        assert_eq!(string, "aBCd");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_string() {
        let range = RangeFromExclusiveToExclusive { start: 0, end: 3 };
        let mut string = "abcd".to_owned();

        string[range].make_ascii_uppercase();

        assert_eq!(string, "aBCd");
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

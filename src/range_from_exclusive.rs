#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{
    ffi::CStr,
    iter::FusedIterator,
    ops::{
        Bound::{self, Excluded, Unbounded},
        Index, IndexMut, RangeBounds, RangeFrom,
    },
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusive<T> {
    /// The lower bound of the range (exclusive).
    pub start: T,
}

impl<T> RangeFromExclusive<T> {
    /// Since implementations for many standard library traits for built-in range types rely on
    /// nightly features, we implement those traits here by converting into standard library range
    /// types. This allows these traits to be implemented without enabling nightly features.
    fn into_range_from(self) -> RangeFrom<T>
    where
        RangeFrom<T>: Iterator,
    {
        let mut range_from = RangeFrom { start: self.start };
        // Advance by one so we don't include the first value.
        range_from.next();
        range_from
    }
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

impl<T> Index<RangeFromExclusive<usize>> for [T] {
    type Output = <[T] as Index<RangeFrom<usize>>>::Output;

    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

impl<T> IndexMut<RangeFromExclusive<usize>> for [T] {
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> Index<RangeFromExclusive<usize>> for Vec<T> {
    type Output = <Vec<T> as Index<RangeFrom<usize>>>::Output;

    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> IndexMut<RangeFromExclusive<usize>> for Vec<T> {
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

impl Index<RangeFromExclusive<usize>> for str {
    type Output = <str as Index<RangeFrom<usize>>>::Output;

    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

impl IndexMut<RangeFromExclusive<usize>> for str {
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl Index<RangeFromExclusive<usize>> for String {
    type Output = <String as Index<RangeFrom<usize>>>::Output;

    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl IndexMut<RangeFromExclusive<usize>> for String {
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

impl Index<RangeFromExclusive<usize>> for CStr {
    type Output = <CStr as Index<RangeFrom<usize>>>::Output;

    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

impl<T> IntoIterator for RangeFromExclusive<T>
where
    RangeFrom<T>: Iterator<Item = T>,
{
    type IntoIter = IterRangeFromExclusive<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IterRangeFromExclusive {
            inner: self.into_range_from(),
        }
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

impl<T> FusedIterator for IterRangeFromExclusive<T> where RangeFrom<T>: Iterator<Item = T> {}

#[cfg(test)]
mod tests {
    use super::RangeFromExclusive;
    #[cfg(feature = "alloc")]
    use alloc::{borrow::ToOwned, vec};
    use claims::{assert_matches, assert_ok, assert_some_eq};
    use core::{
        ffi::CStr,
        ops::{
            Bound::{Excluded, Unbounded},
            RangeBounds,
        },
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
    fn index_slice() {
        let range = RangeFromExclusive { start: 1 };
        let slice = [0, 1, 2, 3];

        assert_eq!(slice[range], [2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_vec() {
        let range = RangeFromExclusive { start: 1 };
        let vec = vec![0, 1, 2, 3];

        assert_eq!(vec[range], [2, 3]);
    }

    #[test]
    fn index_str() {
        let range = RangeFromExclusive { start: 1 };
        let str = "abcd";

        assert_eq!(&str[range], "cd");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_string() {
        let range = RangeFromExclusive { start: 1 };
        let string = "abcd".to_owned();

        assert_eq!(&string[range], "cd");
    }

    #[test]
    fn index_mut_slice() {
        let range = RangeFromExclusive { start: 1 };
        let mut slice = [0, 1, 2, 3];

        slice[range][0] = 4;
        slice[range][1] = 5;

        assert_eq!(slice, [0, 1, 4, 5]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_vec() {
        let range = RangeFromExclusive { start: 1 };
        let mut vec = vec![0, 1, 2, 3];

        vec[range][0] = 4;
        vec[range][1] = 5;

        assert_eq!(vec, [0, 1, 4, 5]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_str() {
        let range = RangeFromExclusive { start: 1 };
        let mut string = "abcd".to_owned();
        let str: &mut str = string.as_mut_str();

        str[range].make_ascii_uppercase();

        assert_eq!(string, "abCD");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn index_mut_string() {
        let range = RangeFromExclusive { start: 1 };
        let mut string = "abcd".to_owned();

        string[range].make_ascii_uppercase();

        assert_eq!(string, "abCD");
    }

    #[test]
    fn index_cstr() {
        let range = RangeFromExclusive { start: 1 };
        let str = assert_ok!(CStr::from_bytes_with_nul(b"abcd\x00"));

        assert_eq!(&str[range].to_bytes(), b"cd");
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

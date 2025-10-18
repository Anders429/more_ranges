#[cfg(feature = "serde")]
use crate::string;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "serde")]
use core::{fmt, fmt::Formatter, marker::PhantomData};
use core::{
    iter::FusedIterator,
    ops::{
        Bound::{self, Excluded},
        Index, IndexMut, Range, RangeBounds,
    },
};
#[cfg(feature = "serde")]
use serde_core::{
    de,
    de::{Deserialize, Deserializer, Error as _, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
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
    #[inline]
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

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

impl<T> IndexMut<RangeFromExclusiveToExclusive<usize>> for [T] {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> Index<RangeFromExclusiveToExclusive<usize>> for Vec<T> {
    type Output = <Vec<T> as Index<Range<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> IndexMut<RangeFromExclusiveToExclusive<usize>> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

impl Index<RangeFromExclusiveToExclusive<usize>> for str {
    type Output = <str as Index<Range<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

impl IndexMut<RangeFromExclusiveToExclusive<usize>> for str {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl Index<RangeFromExclusiveToExclusive<usize>> for String {
    type Output = <String as Index<Range<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &Self::Output {
        self.index(index.into_range())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl IndexMut<RangeFromExclusiveToExclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<T> Serialize for RangeFromExclusiveToExclusive<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RangeFromExclusiveToExclusive", 2)?;
        state.serialize_field("start", &self.start)?;
        state.serialize_field("end", &self.end)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de, T> Deserialize<'de> for RangeFromExclusiveToExclusive<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["start"];

        enum Field {
            Start,
            End,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                        formatter.write_str("`start` or `end`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            "end" => Ok(Field::End),
                            _ => Err(E::unknown_field(value, FIELDS)),
                        }
                    }

                    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            b"start" => Ok(Field::Start),
                            b"end" => Ok(Field::End),
                            _ => Err(E::unknown_field(string::from_utf8_lossy(value), FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RangeFromExclusiveToExclusiveVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for RangeFromExclusiveToExclusiveVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = RangeFromExclusiveToExclusive<T>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct RangeFromExclusiveToExclusive")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let start = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(0, &self))?;
                let end = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(1, &self))?;
                Ok(RangeFromExclusiveToExclusive { start, end })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Start => {
                            if start.is_some() {
                                return Err(A::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                        Field::End => {
                            if end.is_some() {
                                return Err(A::Error::duplicate_field("end"));
                            }
                            end = Some(map.next_value()?);
                        }
                    }
                }

                Ok(RangeFromExclusiveToExclusive {
                    start: start.ok_or_else(|| A::Error::missing_field("start"))?,
                    end: end.ok_or_else(|| A::Error::missing_field("end"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RangeFromExclusiveToExclusive",
            FIELDS,
            RangeFromExclusiveToExclusiveVisitor {
                marker: PhantomData,
            },
        )
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

impl<T> DoubleEndedIterator for IterRangeFromExclusiveToExclusive<T>
where
    Range<T>: DoubleEndedIterator<Item = T>,
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

impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<usize> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<u8> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<u16> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<u32> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(target_pointer_width = "64")]
impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<u64> {
    #[inline]
    fn len(&self) -> usize {
        // Note that this is guaranteed to be a correct bound when the pointer width is 64 bits.
        //
        // Therefore, the lower bound should always be correct. It will always either match the
        // upper bound, or be 0 if the start and end aren't iterable (as in, the end is below or
        // equal to the start).
        self.inner.size_hint().0
    }
}

impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<isize> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<i8> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<i16> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<i32> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(target_pointer_width = "64")]
impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<i64> {
    #[inline]
    fn len(&self) -> usize {
        // Note that this is guaranteed to be a correct bound when the pointer width is 64 bits.
        //
        // Therefore, the lower bound should always be correct. It will always either match the
        // upper bound, or be 0 if the start and end aren't iterable (as in, the end is below or
        // equal to the start).
        self.inner.size_hint().0
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl ExactSizeIterator for IterRangeFromExclusiveToExclusive<char> {
    #[inline]
    fn len(&self) -> usize {
        // Note that this is guaranteed to be a correct bound when the pointer width is 32 bits or
        // larger.
        //
        // Therefore, the lower bound should always be correct. It will always either match the
        // upper bound, or be 0 if the start and end aren't iterable (as in, the end is below or
        // equal to the start).
        self.inner.size_hint().0
    }
}

impl<T> FusedIterator for IterRangeFromExclusiveToExclusive<T> where Range<T>: Iterator<Item = T> {}

#[cfg(test)]
mod tests {
    use super::RangeFromExclusiveToExclusive;
    #[cfg(feature = "alloc")]
    use alloc::{borrow::ToOwned, vec};
    #[cfg(all(feature = "alloc", feature = "serde"))]
    use claims::{assert_err_eq, assert_ok, assert_ok_eq};
    use claims::{assert_matches, assert_none, assert_some_eq};
    use core::ops::{Bound::Excluded, RangeBounds};
    #[cfg(all(feature = "alloc", feature = "serde"))]
    use serde_assert::{de, Deserializer, Serializer, Token};
    #[cfg(all(feature = "alloc", feature = "serde"))]
    use serde_core::{Deserialize, Serialize};

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

    #[test]
    fn iter_next_back() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 4 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.next_back(), 3);
        assert_some_eq!(iter.next_back(), 2);
        assert_none!(iter.next_back());
    }

    #[test]
    fn iter_nth_back() {
        let range = RangeFromExclusiveToExclusive { start: 1, end: 250 };
        let mut iter = range.into_iter();

        assert_some_eq!(iter.nth_back(42), 207);
        assert_some_eq!(iter.nth_back(100), 106);
        assert_none!(iter.nth_back(104));
    }

    #[test]
    fn iter_usize_len() {
        let range = RangeFromExclusiveToExclusive {
            start: 1usize,
            end: 4usize,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn iter_usize_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1usize,
            end: 1usize,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn iter_usize_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: usize::MIN,
            end: usize::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), usize::MAX - 1);
    }

    #[test]
    fn iter_u8_len() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u8,
            end: 4u8,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn iter_u8_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u8,
            end: 1u8,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn iter_u8_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: u8::MIN,
            end: u8::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u8::MAX - 1) as usize);
    }

    #[test]
    fn iter_u16_len() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u16,
            end: 4u16,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn iter_u16_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u16,
            end: 1u16,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn iter_u16_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: u16::MIN,
            end: u16::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u16::MAX - 1) as usize);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_u32_len() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u32,
            end: 4u32,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 2);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_u32_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u32,
            end: 1u32,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_u32_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: u32::MIN,
            end: u32::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u32::MAX - 1) as usize);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn iter_u64_len() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u64,
            end: 4u64,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 2);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn iter_u64_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u64,
            end: 1u64,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn iter_u64_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: u64::MIN,
            end: u64::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u64::MAX - 1) as usize);
    }

    #[test]
    fn iter_isize_len() {
        let range = RangeFromExclusiveToExclusive {
            start: -1isize,
            end: 4isize,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 4);
    }

    #[test]
    fn iter_isize_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1isize,
            end: 1isize,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn iter_isize_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: isize::MIN,
            end: isize::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), usize::MAX - 1);
    }

    #[test]
    fn iter_i8_len() {
        let range = RangeFromExclusiveToExclusive {
            start: -1i8,
            end: 4i8,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 4);
    }

    #[test]
    fn iter_i8_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1i8,
            end: 1i8,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn iter_i8_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: i8::MIN,
            end: i8::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u8::MAX - 1) as usize);
    }

    #[test]
    fn iter_i16_len() {
        let range = RangeFromExclusiveToExclusive {
            start: -1i16,
            end: 4i16,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 4);
    }

    #[test]
    fn iter_i16_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1i16,
            end: 1i16,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn iter_i16_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: i16::MIN,
            end: i16::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u16::MAX - 1) as usize);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_i32_len() {
        let range = RangeFromExclusiveToExclusive {
            start: -1i32,
            end: 4i32,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 4);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_i32_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1i32,
            end: 1i32,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_i32_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: i32::MIN,
            end: i32::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u32::MAX - 1) as usize);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn iter_i64_len() {
        let range = RangeFromExclusiveToExclusive {
            start: -1i64,
            end: 4i64,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 4);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn iter_i64_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 1i64,
            end: 1i64,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn iter_i64_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: i64::MIN,
            end: i64::MAX,
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), (u64::MAX - 1) as usize);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_char_len() {
        let range = RangeFromExclusiveToExclusive {
            start: 'a',
            end: 'd',
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 2);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_char_len_empty() {
        let range = RangeFromExclusiveToExclusive {
            start: 'a',
            end: 'a',
        };
        let iter = range.into_iter();

        assert_eq!(iter.len(), 0);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn iter_char_len_max() {
        let range = RangeFromExclusiveToExclusive {
            start: char::MIN,
            end: char::MAX,
        };
        let iter = range.into_iter();

        // This is guaranteed to never change. Unicode has a fixed number of code points.
        assert_eq!(iter.len(), 1_112_062);
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn serialize() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u32,
            end: 4u32,
        };

        let serializer = Serializer::builder().build();
        assert_ok_eq!(
            range.serialize(&serializer),
            [
                Token::Struct {
                    name: "RangeFromExclusiveToExclusive",
                    len: 2
                },
                Token::Field("start"),
                Token::U32(1),
                Token::Field("end"),
                Token::U32(4),
                Token::StructEnd,
            ]
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusiveToExclusive",
                len: 2,
            },
            Token::Field("start"),
            Token::I8(-5),
            Token::Field("end"),
            Token::I8(27),
            Token::StructEnd,
        ])
        .build();

        assert_ok_eq!(
            RangeFromExclusiveToExclusive::deserialize(&mut deserializer),
            RangeFromExclusiveToExclusive {
                start: -5i8,
                end: 27,
            }
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_unknown_field() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusiveToExclusive",
                len: 1,
            },
            Token::Field("invalid"),
            Token::I8(-5),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusiveToExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::UnknownField("invalid".to_owned(), &["start"])
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_duplicate_field_start() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusiveToExclusive",
                len: 3,
            },
            Token::Field("start"),
            Token::I8(-5),
            Token::Field("end"),
            Token::I8(27),
            Token::Field("start"),
            Token::I8(42),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusiveToExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::DuplicateField("start")
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_duplicate_field_end() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusiveToExclusive",
                len: 3,
            },
            Token::Field("start"),
            Token::I8(-5),
            Token::Field("end"),
            Token::I8(27),
            Token::Field("end"),
            Token::I8(42),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusiveToExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::DuplicateField("end")
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_missing_field_start() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusiveToExclusive",
                len: 2,
            },
            Token::Field("end"),
            Token::I8(27),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusiveToExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::MissingField("start")
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_missing_field_end() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusiveToExclusive",
                len: 2,
            },
            Token::Field("start"),
            Token::I8(-5),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusiveToExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::MissingField("end")
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn serde_roundtrip() {
        let range = RangeFromExclusiveToExclusive {
            start: 1u32,
            end: 4u32,
        };

        let serializer = Serializer::builder().build();
        let mut deserializer =
            Deserializer::builder(assert_ok!(range.serialize(&serializer))).build();
        assert_ok_eq!(
            RangeFromExclusiveToExclusive::deserialize(&mut deserializer),
            range
        );
    }
}

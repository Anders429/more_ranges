#[cfg(feature = "serde")]
use crate::string;
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
#[cfg(feature = "serde")]
use core::{fmt, fmt::Formatter, marker::PhantomData};
#[cfg(feature = "serde")]
use serde_core::{
    de,
    de::{Deserialize, Deserializer, Error as _, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
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
    #[inline]
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

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

impl<T> IndexMut<RangeFromExclusive<usize>> for [T] {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> Index<RangeFromExclusive<usize>> for Vec<T> {
    type Output = <Vec<T> as Index<RangeFrom<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> IndexMut<RangeFromExclusive<usize>> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

impl Index<RangeFromExclusive<usize>> for str {
    type Output = <str as Index<RangeFrom<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

impl IndexMut<RangeFromExclusive<usize>> for str {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl Index<RangeFromExclusive<usize>> for String {
    type Output = <String as Index<RangeFrom<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
impl IndexMut<RangeFromExclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut Self::Output {
        self.index_mut(index.into_range_from())
    }
}

impl Index<RangeFromExclusive<usize>> for CStr {
    type Output = <CStr as Index<RangeFrom<usize>>>::Output;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &Self::Output {
        self.index(index.into_range_from())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<T> Serialize for RangeFromExclusive<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RangeFromExclusive", 1)?;
        state.serialize_field("start", &self.start)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de, T> Deserialize<'de> for RangeFromExclusive<T>
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
                        formatter.write_str("`start`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            _ => Err(E::unknown_field(value, FIELDS)),
                        }
                    }

                    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            b"start" => Ok(Field::Start),
                            _ => Err(E::unknown_field(string::from_utf8_lossy(value), FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RangeFromExclusiveVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for RangeFromExclusiveVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = RangeFromExclusive<T>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct RangeFromExclusive")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let start = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(0, &self))?;
                Ok(RangeFromExclusive { start })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut start = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Start => {
                            if start.is_some() {
                                return Err(A::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                    }
                }

                Ok(RangeFromExclusive {
                    start: start.ok_or_else(|| A::Error::missing_field("start"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RangeFromExclusive",
            FIELDS,
            RangeFromExclusiveVisitor {
                marker: PhantomData,
            },
        )
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

/// Iterator for [`RangeFromExclusive`].
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
    #[cfg(all(feature = "alloc", feature = "serde"))]
    use claims::{assert_err_eq, assert_ok_eq};
    use claims::{assert_matches, assert_ok, assert_some_eq};
    use core::{
        ffi::CStr,
        ops::{
            Bound::{Excluded, Unbounded},
            RangeBounds,
        },
    };
    #[cfg(all(feature = "alloc", feature = "serde"))]
    use serde_assert::{de, Deserializer, Serializer, Token};
    #[cfg(all(feature = "alloc", feature = "serde"))]
    use serde_core::{Deserialize, Serialize};

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

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn serialize() {
        let range = RangeFromExclusive { start: 1u32 };

        let serializer = Serializer::builder().build();
        assert_ok_eq!(
            range.serialize(&serializer),
            [
                Token::Struct {
                    name: "RangeFromExclusive",
                    len: 1
                },
                Token::Field("start"),
                Token::U32(1),
                Token::StructEnd,
            ]
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusive",
                len: 1,
            },
            Token::Field("start"),
            Token::I8(-5),
            Token::StructEnd,
        ])
        .build();

        assert_ok_eq!(
            RangeFromExclusive::deserialize(&mut deserializer),
            RangeFromExclusive { start: -5i8 }
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_unknown_field() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusive",
                len: 1,
            },
            Token::Field("invalid"),
            Token::I8(-5),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::UnknownField("invalid".to_owned(), &["start"])
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_duplicate_field() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusive",
                len: 2,
            },
            Token::Field("start"),
            Token::I8(-5),
            Token::Field("start"),
            Token::I8(42),
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::DuplicateField("start")
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn deserialize_missing_field() {
        let mut deserializer = Deserializer::builder([
            Token::Struct {
                name: "RangeFromExclusive",
                len: 0,
            },
            Token::StructEnd,
        ])
        .build();

        assert_err_eq!(
            RangeFromExclusive::<i8>::deserialize(&mut deserializer),
            de::Error::MissingField("start")
        );
    }

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[test]
    fn serde_roundtrip() {
        let range = RangeFromExclusive { start: 1u32 };

        let serializer = Serializer::builder().build();
        let mut deserializer =
            Deserializer::builder(assert_ok!(range.serialize(&serializer))).build();
        assert_ok_eq!(RangeFromExclusive::deserialize(&mut deserializer), range);
    }
}

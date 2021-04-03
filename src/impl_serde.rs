use core::fmt;
use core::marker::PhantomData;
#[cfg(feature = "doc_item")]
use doc_item::since;
use serde::de;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use RangeFromExclusive;
use RangeFromExclusiveToExclusive;
use RangeFromExclusiveToInclusive;

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<T> Serialize for RangeFromExclusive<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = try!(serializer.serialize_struct("RangeFromExclusive", 1));
        try!(state.serialize_field("start", &self.start));
        state.end()
    }
}

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de, T> Deserialize<'de> for RangeFromExclusive<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Start,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`start`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RangeFromExclusiveVisitor<T> {
            phantom: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for RangeFromExclusiveVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = RangeFromExclusive<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RangeFromExclusive")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let start = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };
                Ok(RangeFromExclusive { start: start })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut start = None;
                while let Some(key) = try!(map.next_key()) {
                    match key {
                        Field::Start => {
                            if start.is_some() {
                                return Err(de::Error::duplicate_field("start"));
                            }
                            start = Some(try!(map.next_value()))
                        }
                    }
                }
                let start = match start {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("start")),
                };
                Ok(RangeFromExclusive { start: start })
            }
        }

        const FIELDS: &'static [&'static str] = &["start"];
        deserializer.deserialize_struct(
            "RangeFromExclusive",
            FIELDS,
            RangeFromExclusiveVisitor {
                phantom: PhantomData,
            },
        )
    }
}

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<T> Serialize for RangeFromExclusiveToInclusive<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = try!(serializer.serialize_struct("RangeFromExclusiveToInclusive", 2));
        try!(state.serialize_field("start", &self.start));
        try!(state.serialize_field("end", &self.end));
        state.end()
    }
}

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de, T> Deserialize<'de> for RangeFromExclusiveToInclusive<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`start`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            "end" => Ok(Field::End),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RangeFromExclusiveToInclusiveVisitor<T> {
            phantom: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for RangeFromExclusiveToInclusiveVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = RangeFromExclusiveToInclusive<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RangeFromExclusiveToInclusive")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let start = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };
                let end = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(1, &self)),
                };
                Ok(RangeFromExclusiveToInclusive {
                    start: start,
                    end: end,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;
                while let Some(key) = try!(map.next_key()) {
                    match key {
                        Field::Start => {
                            if start.is_some() {
                                return Err(de::Error::duplicate_field("start"));
                            }
                            start = Some(try!(map.next_value()))
                        }
                        Field::End => {
                            if end.is_some() {
                                return Err(de::Error::duplicate_field("end"));
                            }
                            end = Some(try!(map.next_value()))
                        }
                    }
                }
                let start = match start {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("start")),
                };
                let end = match end {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("end")),
                };
                Ok(RangeFromExclusiveToInclusive {
                    start: start,
                    end: end,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["start", "end"];
        deserializer.deserialize_struct(
            "RangeFromExclusiveToInclusive",
            FIELDS,
            RangeFromExclusiveToInclusiveVisitor {
                phantom: PhantomData,
            },
        )
    }
}

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<T> Serialize for RangeFromExclusiveToExclusive<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = try!(serializer.serialize_struct("RangeFromExclusiveToExclusive", 2));
        try!(state.serialize_field("start", &self.start));
        try!(state.serialize_field("end", &self.end));
        state.end()
    }
}

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de, T> Deserialize<'de> for RangeFromExclusiveToExclusive<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`start`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            "end" => Ok(Field::End),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RangeFromExclusiveToExclusiveVisitor<T> {
            phantom: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for RangeFromExclusiveToExclusiveVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = RangeFromExclusiveToExclusive<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RangeFromExclusiveToExclusive")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let start = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };
                let end = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(1, &self)),
                };
                Ok(RangeFromExclusiveToExclusive {
                    start: start,
                    end: end,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;
                while let Some(key) = try!(map.next_key()) {
                    match key {
                        Field::Start => {
                            if start.is_some() {
                                return Err(de::Error::duplicate_field("start"));
                            }
                            start = Some(try!(map.next_value()))
                        }
                        Field::End => {
                            if end.is_some() {
                                return Err(de::Error::duplicate_field("end"));
                            }
                            end = Some(try!(map.next_value()))
                        }
                    }
                }
                let start = match start {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("start")),
                };
                let end = match end {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("end")),
                };
                Ok(RangeFromExclusiveToExclusive {
                    start: start,
                    end: end,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["start", "end"];
        deserializer.deserialize_struct(
            "RangeFromExclusiveToExclusive",
            FIELDS,
            RangeFromExclusiveToExclusiveVisitor {
                phantom: PhantomData,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_test::assert_tokens;
    use serde_test::Token;
    use RangeFromExclusive;
    use RangeFromExclusiveToExclusive;
    use RangeFromExclusiveToInclusive;

    #[test]
    fn range_from_exclusive_ser_de() {
        assert_tokens(
            &RangeFromExclusive::<u8> { start: 1 },
            &[
                Token::Struct {
                    name: "RangeFromExclusive",
                    len: 1,
                },
                Token::Str("start"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn range_from_exclusive_to_inclusive_ser_de() {
        assert_tokens(
            &RangeFromExclusiveToInclusive::<u8> { start: 1, end: 3 },
            &[
                Token::Struct {
                    name: "RangeFromExclusiveToInclusive",
                    len: 2,
                },
                Token::Str("start"),
                Token::U8(1),
                Token::Str("end"),
                Token::U8(3),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn range_from_exclusive_to_exclusive_ser_de() {
        assert_tokens(
            &RangeFromExclusiveToExclusive::<u8> { start: 1, end: 3 },
            &[
                Token::Struct {
                    name: "RangeFromExclusiveToExclusive",
                    len: 2,
                },
                Token::Str("start"),
                Token::U8(1),
                Token::Str("end"),
                Token::U8(3),
                Token::StructEnd,
            ],
        )
    }
}

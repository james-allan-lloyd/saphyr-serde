use std::str::FromStr;

use regex::RegexSet;
use saphyr_parser::Event;
use serde::{
    Deserialize,
    de::{IntoDeserializer, Visitor},
};

use crate::{
    error::{DeserializeError, Result},
    mapping::YamlMapping,
    seq::YamlSequence,
    variant::Enum,
};

pub struct YamlDeserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    yaml: saphyr_parser::Parser<'de, saphyr_parser::StrInput<'de>>,
}

impl<'de> YamlDeserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_str(input: &'de str) -> Self {
        let yaml = saphyr_parser::Parser::new_from_str(input);

        YamlDeserializer { yaml }
    }

    pub fn next_event(&mut self) -> Result<(Event<'de>, saphyr_parser::Span)> {
        let next = self.yaml.next_event();
        println!("next: {:?}", next);
        Ok(next.ok_or(DeserializeError::EarlyTermination)??)
    }

    pub fn peek_event(&mut self) -> Option<&(Event<'_>, saphyr_parser::Span)> {
        let peek = self.yaml.peek();
        println!("peek: {:?}", peek);
        // TODO:
        peek.and_then(|r| r.ok())
    }

    pub fn parse_scalar<T>(&mut self) -> Result<T>
    where
        T: FromStr,
    {
        let (s, span) = self.read_scalar_string()?;
        let parse_result = s.parse::<T>();
        parse_result
            .map_err(|_e| DeserializeError::number_parse_failure(&s, span, "parse_unsigned"))
    }

    pub fn read_scalar_string(
        &mut self,
    ) -> Result<(std::borrow::Cow<'_, str>, saphyr_parser::Span)> {
        match self.next_event()? {
            (saphyr_parser::Event::Scalar(s, _, _, _), span) => Ok((s, span)),
            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_str",
            )),
        }
    }

    pub fn peek_scalar_string(
        &mut self,
    ) -> Option<(std::borrow::Cow<'_, str>, saphyr_parser::Span)> {
        match self.peek_event()? {
            (saphyr_parser::Event::Scalar(s, _, _, _), span) => Some((s.clone(), span.to_owned())),
            _ => None,
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut YamlDeserializer<'de> {
    type Error = crate::error::DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_event()? {
            (saphyr_parser::Event::Scalar(value, _, _, _), _span) => visitor.visit_str(&value),
            (saphyr_parser::Event::MappingStart(_map, _), _span) => {
                visitor.visit_map(YamlMapping::new(self))
            }
            // 'n' => self.deserialize_unit(visitor),
            // 't' | 'f' => self.deserialize_bool(visitor),
            // '"' => self.deserialize_str(visitor),
            // '0'..='9' => self.deserialize_u64(visitor),
            // '-' => self.deserialize_i64(visitor),
            (saphyr_parser::Event::SequenceStart(_, _), _span) => {
                visitor.visit_seq(YamlSequence::new(self))
            }
            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_any",
            )),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // FIXME: don't instantiate here
        let set = RegexSet::new([
            r"^(y|Y|yes|Yes|YES|true|True|TRUE|on|On|ON|)$",
            r"^(n|N|no|No|NO|false|False|FALSE|off|Off|OFF)$",
        ])
        .unwrap();

        let (s, span) = self.read_scalar_string()?;

        let matches = set.matches(&s);
        if matches.matched(0) {
            visitor.visit_bool(true)
        } else if matches.matched(1) {
            visitor.visit_bool(false)
        } else {
            Err(DeserializeError::not_a_bool(&s, span))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_scalar()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_scalar()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_scalar()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_scalar()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_scalar()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_scalar()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_scalar()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_scalar()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_scalar()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_scalar()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => visitor.visit_str(&key),
            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_str",
            )),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => visitor.visit_str(&key),
            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_string",
            )),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek_scalar_string().map(|(s, _span)| s == "null") {
            Some(true) => {
                self.next_event()?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::SequenceStart(_size, _option_tag), _span) => {
                let value = visitor.visit_seq(YamlSequence::new(self))?;
                Ok(value)
            }
            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_seq",
            )),
        }
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next() {
            Some(event) => match event {
                Ok((saphyr_parser::Event::MappingStart(_size, _option_tag), _span)) => {
                    let value = visitor.visit_map(YamlMapping::new(self))?;
                    Ok(value)
                }
                Ok((event, _span)) => Err(DeserializeError::unexpected(
                    &event,
                    _span,
                    "deserialize_map",
                )),
                _ => todo!(),
            },
            None => Err(DeserializeError::TypeError),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => {
                let s = key.to_string();
                visitor.visit_enum(s.into_deserializer())
            }
            (saphyr_parser::Event::MappingStart(_, _), _span) => {
                let value = visitor.visit_enum(Enum::new(self))?;

                match self.yaml.next().unwrap().unwrap() {
                    (saphyr_parser::Event::MappingEnd, _span) => Ok(value),
                    (event, span) => Err(DeserializeError::unexpected(
                        &event,
                        span,
                        "deserialize_identifier",
                    )),
                }
            }

            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_enum",
            )),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => visitor.visit_str(&key),
            (event, span) => Err(DeserializeError::unexpected(
                &event,
                span,
                "deserialize_identifier",
            )),
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
        // _visitor.visit_none()
    }
}

#[allow(dead_code)]
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = YamlDeserializer::from_str(s);
    let _stream_start = deserializer.yaml.next_event().unwrap().unwrap();
    let _doc_start = deserializer.yaml.next_event().unwrap().unwrap();
    let t = T::deserialize(&mut deserializer)?;
    match deserializer.yaml.next_event().unwrap().unwrap() {
        (Event::DocumentEnd, _span) => Ok(t),
        (event, span) => Err(DeserializeError::unexpected(&event, span, "from_str")),
    }
}

#[cfg(test)]
mod test {

    use std::f32;

    use serde::Deserialize;
    use serde_json::json;

    use crate::deserialize::from_str;

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    const POINT_YAML_STR: &str = r###"
x: 10
y: 45
"###;

    #[test]
    fn it_deserializes_yaml() {
        let result: Point = from_str(POINT_YAML_STR).expect("Should deserialize");

        assert_eq!(result, Point { x: 10, y: 45 });
    }

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Address {
        street: String,
        state: String,
    }

    const ADDRESS_YAML_STR: &str = r###"
street: Kerkstraat
state: Noord Holland
"###;

    #[test]
    fn it_deserializes_strings() {
        let result: Address = from_str(ADDRESS_YAML_STR).expect("Should deserialize");

        assert_eq!(
            result,
            Address {
                street: String::from("Kerkstraat"),
                state: String::from("Noord Holland")
            }
        );
    }

    #[test]
    fn it_reads_json_values() {
        let result: serde_json::Value = from_str(ADDRESS_YAML_STR).expect("Should deserialize");

        assert_eq!(
            result,
            json!({"street": "Kerkstraat", "state": "Noord Holland"})
        );
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct NestedAddress {
        address: Address,
    }

    const NESTED_ADDRESS_YAML_STR: &str = r###"
address:
    street: Kerkstraat
    state: Noord Holland
"###;

    #[test]
    fn it_reads_nested_values() {
        let result: serde_json::Value =
            from_str(NESTED_ADDRESS_YAML_STR).expect("Should deserialize");

        assert_eq!(
            result,
            json!({"address": {"street": "Kerkstraat", "state": "Noord Holland"}})
        );

        let address: NestedAddress = from_str(NESTED_ADDRESS_YAML_STR).expect("Should deserialize");

        assert_eq!(
            address,
            NestedAddress {
                address: Address {
                    street: String::from("Kerkstraat"),
                    state: String::from("Noord Holland")
                }
            }
        );
    }

    const SEQUENCE_ADDRESS_YAML_STR: &str = r###"
- street: Kerkstraat
  state: Noord Holland
- street: Main Street
  state: New York
"###;

    #[test]
    fn it_reads_sequences() {
        let result: serde_json::Value =
            from_str(SEQUENCE_ADDRESS_YAML_STR).expect("Should deserialize");

        assert_eq!(
            result,
            json!([
                {"street": "Kerkstraat", "state": "Noord Holland"},
                {"street": "Main Street", "state": "New York"},
            ])
        );

        let address: Vec<Address> =
            from_str(SEQUENCE_ADDRESS_YAML_STR).expect("Should deserialize");

        assert_eq!(
            address,
            vec![
                Address {
                    street: String::from("Kerkstraat"),
                    state: String::from("Noord Holland")
                },
                Address {
                    street: String::from("Main Street"),
                    state: String::from("New York")
                },
            ]
        );
    }

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    enum TestEnum {
        ValueA,
        ValueB,
    }

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct StructWithEnum {
        value: TestEnum,
    }

    const STRUCT_WITH_ENUM_YAML_STR: &str = r###"
value: ValueA
"###;

    #[test]
    fn it_reads_enums() {
        let result: StructWithEnum =
            from_str(STRUCT_WITH_ENUM_YAML_STR).expect("Should deserialize");

        assert_eq!(
            result,
            StructWithEnum {
                value: TestEnum::ValueA
            }
        );
    }

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    enum TestExternallyTaggedEnum {
        ValueA { id: String, method: String },
        ValueB { id: String, result: String },
    }

    const EXTERNALLY_TAGGED_ENUM_YAML_STR: &str = r###"
- ValueA:
    id: foo
    method: bar
- ValueB:
    id: baz
    result: passed
"###;

    #[test]
    fn it_reads_externally_tagged_enums() {
        let result: Vec<TestExternallyTaggedEnum> =
            from_str(EXTERNALLY_TAGGED_ENUM_YAML_STR).expect("Should deserialize");

        assert_eq!(
            result,
            vec![
                TestExternallyTaggedEnum::ValueA {
                    id: String::from("foo"),
                    method: String::from("bar")
                },
                TestExternallyTaggedEnum::ValueB {
                    id: String::from("baz"),
                    result: String::from("passed")
                }
            ],
        );
    }

    #[test]
    fn it_reads_all_the_int_formats() {
        #[derive(Deserialize, PartialEq, Eq, Debug)]
        struct TestInts {
            sbyte: i8,
            ubyte: u8,
            sshort: i16,
            ushort: u16,
            slong: i32,
            ulong: u32,
            slonglong: i64,
            ulonglong: u64,
        }

        const TEST_INTS_YAML: &str = r###"
sbyte: -1
ubyte: 2
sshort: -3
ushort: 4
slong: -5
ulong: 6
slonglong: -7
ulonglong: 8
"###;
        let result: TestInts = from_str(TEST_INTS_YAML).expect("Should deserialize");

        assert_eq!(
            result,
            TestInts {
                sbyte: -1,
                ubyte: 2,
                sshort: -3,
                ushort: 4,
                slong: -5,
                ulong: 6,
                slonglong: -7,
                ulonglong: 8,
            }
        );
    }

    #[test]
    fn it_reads_all_both_floats() {
        #[derive(Deserialize, Debug)]
        struct TestFloats {
            single: f32,
            double: f64,
        }

        const TEST_YAML: &str = r###"
single: 0.123
double: 0.123
"###;
        let result: TestFloats = from_str(TEST_YAML).expect("Should deserialize");

        fn are_nearly_equal<T: Into<f64>>(a: T, b: T, epsilon: T) -> bool {
            let a = a.into();
            let b = b.into();
            let epsilon = epsilon.into();

            (a - b).abs() < epsilon
        }

        assert!(are_nearly_equal(result.single, 0.123, f32::EPSILON));
        assert!(are_nearly_equal(result.double, 0.123, f64::EPSILON));
    }

    #[test]
    fn it_reads_chars() {
        #[derive(Deserialize, Debug)]
        struct Test {
            c: char,
        }

        from_str::<Test>(
            r###"
c: ab
"###,
        )
        .expect_err("Should not deserialize");

        let result: Test = from_str(
            r###"
c: a
"###,
        )
        .expect("Should deserialize");
        assert_eq!(result.c, 'a');
    }

    #[test]
    fn it_reads_bools() {
        #[derive(Deserialize, Debug)]
        struct Test {
            b: bool,
        }

        from_str::<Test>("b: not_a_boolean").expect_err("Should not deserialize");

        let result: Test = from_str("b: True").expect("Should deserialize");
        assert!(result.b);

        from_str::<Test>("b: tRUE").expect_err("Should not deserialize");
    }

    #[test]
    fn it_reads_options() {
        #[derive(Deserialize, Debug)]
        struct Test {
            opt: Option<String>,
        }

        let result: Test = from_str("opt: foo").expect("Should deserialize");
        assert_eq!(result.opt, Some(String::from("foo")));

        let result: Test = from_str("opt: null").expect("Should deserialize");
        assert_eq!(result.opt, None);
    }
}

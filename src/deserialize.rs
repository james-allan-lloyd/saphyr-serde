use std::{
    ops::{AddAssign, MulAssign, Neg},
    str::FromStr,
};

use saphyr_parser::Event;
use serde::{Deserialize, de::Visitor};

use crate::{
    error::{DeserializeError, Result},
    mapping::YamlMapping,
    seq::YamlSequence,
};

pub struct YamlDeserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    pub yaml: saphyr_parser::Parser<'de, saphyr_parser::StrInput<'de>>,
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

    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8> + FromStr<Err = DeserializeError>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(value, _, _, _), _span) => value.parse::<T>(),
            e => Err(DeserializeError::UnexpectedElement(format!("{:?}", e))),
        }
    }

    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + FromStr,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(value, _, _, _), _span) => match value.parse::<T>() {
                Ok(value) => Ok(value),
                Err(_) => todo!(),
            },
            e => Err(DeserializeError::UnexpectedElement(format!("{:?}", e))),
        }
    }
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut YamlDeserializer<'de> {
    type Error = crate::error::DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(value, _, _, _), span) => visitor.visit_str(&value),
            (saphyr_parser::Event::MappingStart(map, _), span) => {
                visitor.visit_map(YamlMapping::new(self))
            }
            // 'n' => self.deserialize_unit(visitor),
            // 't' | 'f' => self.deserialize_bool(visitor),
            // '"' => self.deserialize_str(visitor),
            // '0'..='9' => self.deserialize_u64(visitor),
            // '-' => self.deserialize_i64(visitor),
            (saphyr_parser::Event::SequenceStart(_, _), span) => {
                visitor.visit_seq(YamlSequence::new(self))
            }
            // '[' => self.deserialize_seq(visitor),
            // '{' => self.deserialize_map(visitor),
            (event, span) => Err(DeserializeError::UnexpectedElement(format!("{:?}", event))),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => visitor.visit_str(&key),
            e => Err(DeserializeError::UnexpectedElement(format!("{:?}", e))),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => visitor.visit_str(&key),
            e => Err(DeserializeError::UnexpectedElement(format!("{:?}", e))),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
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
        todo!()
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
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
                Ok((saphyr_parser::Event::MappingStart(size, option_tag), span)) => {
                    println!("Size {}", size);
                    let value = visitor.visit_map(YamlMapping::new(self))?;
                    Ok(value)
                }
                Ok((event, span)) => {
                    Err(DeserializeError::UnexpectedElement(format!("{:?}", event)))
                }
                _ => todo!(),
            },
            None => Err(DeserializeError::TypeError),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.yaml.next().unwrap().unwrap() {
            (saphyr_parser::Event::Scalar(key, _, _, _), _span) => visitor.visit_str(&key),
            e => Err(DeserializeError::UnexpectedElement(format!("{:?}", e))),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = YamlDeserializer::from_str(s);
    let stream_start = deserializer.yaml.next_event().unwrap().unwrap();
    let doc_start = deserializer.yaml.next_event().unwrap().unwrap();
    let t = T::deserialize(&mut deserializer)?;
    match deserializer.yaml.next_event().unwrap().unwrap() {
        (Event::DocumentEnd, span) => Ok(t),
        (event, span) => Err(DeserializeError::UnexpectedElement(format!("{:?}", event))),
    }
}

#[cfg(test)]
mod test {

    use std::ops::Add;

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
addresses:
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
            json!({"addresses": [
                {"street": "Kerkstraat", "state": "Noord Holland"},
                {"street": "Main Street", "state": "New York"},
            ]})
        );

        // let address: NestedAddress = from_str(NESTED_ADDRESS_YAML_STR).expect("Should deserialize");
        //
        // assert_eq!(
        //     address,
        //     NestedAddress {
        //         address: Address {
        //             street: String::from("Kerkstraat"),
        //             state: String::from("Noord Holland")
        //         }
        //     }
        // );
    }
}

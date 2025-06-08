use serde::de::{DeserializeSeed, EnumAccess, VariantAccess};

use crate::{deserialize::YamlDeserializer, error::DeserializeError};

pub(crate) struct Enum<'a, 'de: 'a> {
    de: &'a mut YamlDeserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    pub fn new(de: &'a mut YamlDeserializer<'de>) -> Self {
        Enum { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = DeserializeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> std::result::Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        // match self.de.yaml.next().unwrap().unwrap() {
        //     (saphyr_parser::Event::Scalar(value, _, _, _), _span) =>
        // }

        Ok((val, self))

        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        // let val = seed.deserialize(&mut *self.de)?;
        // // Parse the colon separating map key from value.
        // if self.de.next_char()? == ':' {
        //     Ok((val, self))
        // } else {
        //     Err(De::ExpectedMapColon)
        // }
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = DeserializeError;

    fn unit_variant(self) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> std::result::Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        todo!()
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_map(self.de, visitor)
    }
}

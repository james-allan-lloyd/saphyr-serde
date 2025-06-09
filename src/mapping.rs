use saphyr_parser::Event;
use serde::de::{DeserializeSeed, MapAccess};

use crate::{deserialize::YamlDeserializer, error::DeserializeError};

pub struct YamlMapping<'a, 'de: 'a> {
    de: &'a mut YamlDeserializer<'de>,
}
impl<'a, 'de> YamlMapping<'a, 'de> {
    pub(crate) fn new(de: &'a mut YamlDeserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> MapAccess<'de> for YamlMapping<'a, 'de> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.de.peek_event() {
            Some((Event::MappingEnd, _span)) => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

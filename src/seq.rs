use saphyr_parser::Event;
use serde::de::{DeserializeSeed, SeqAccess};

use crate::{deserialize::YamlDeserializer, error::DeserializeError};

pub struct YamlSequence<'a, 'de: 'a> {
    de: &'a mut YamlDeserializer<'de>,
}

impl<'a, 'de> YamlSequence<'a, 'de> {
    pub(crate) fn new(de: &'a mut YamlDeserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for YamlSequence<'a, 'de> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.yaml.peek().unwrap().unwrap().0 == Event::SequenceEnd {
            self.de.yaml.next();
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

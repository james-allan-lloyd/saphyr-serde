use crate::error::SerializeError;
use serde::{Serialize, ser};

pub struct Serializer {
    output: String,
}

pub type Result<T> = std::result::Result<T, SerializeError>;

pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = SerializeError;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, _v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, _value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_key<T>(&mut self, _key: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

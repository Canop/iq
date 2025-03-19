use {
    crate::errors::IqInternalError,
    serde::{
        Serialize,
        ser,
    },
};

/// "serialize" a value by counting it.
///
/// Eg, if it's a map/struct, count the number of keys.
pub(crate) struct Sizer {
    count: usize,
}
impl Sizer {
    pub fn new() -> Self {
        Self { count: 0 }
    }
    fn finish(&mut self) -> Result<(), IqInternalError> {
        Err(IqInternalError::Count(self.count))
    }
    fn uncountable(&mut self) -> Result<(), IqInternalError> {
        Err(IqInternalError::NoCount)
    }
    pub fn count<T: Serialize>(value: T) -> Option<usize> {
        let mut sizer = Self::new();
        match value.serialize(&mut sizer) {
            Ok(()) => {
                // not really expected
                Some(sizer.count)
            }
            Err(IqInternalError::Count(count)) => Some(count),
            Err(IqInternalError::NoCount) => None,
            Err(_) => None, // not expected to happen unless object isn't serializable
        }
    }
}
impl ser::Serializer for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn serialize_bool(
        self,
        _v: bool,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_i8(
        self,
        _v: i8,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_i16(
        self,
        _v: i16,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_i32(
        self,
        _v: i32,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_i64(
        self,
        _v: i64,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_u8(
        self,
        _v: u8,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_u16(
        self,
        _v: u16,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_u32(
        self,
        _v: u32,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_u64(
        self,
        _v: u64,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_f32(
        self,
        _v: f32,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_f64(
        self,
        _v: f64,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_char(
        self,
        _v: char,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_str(
        self,
        v: &str,
    ) -> Result<(), IqInternalError> {
        self.count = v.chars().count();
        self.finish()
    }
    fn serialize_bytes(
        self,
        _v: &[u8],
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_none(self) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_some<T>(
        self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self) // Some(array) should be the same as array
    }
    fn serialize_unit(self) -> Result<(), IqInternalError> {
        self.count = 0; // it's simpler to see it as an empty tuple
        self.finish()
    }
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), IqInternalError> {
        self.uncountable()
    }
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        // TODO I'm not sure what this is for
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, IqInternalError> {
        if let Some(len) = len {
            self.count = len;
            self.finish()?;
        }
        Ok(self)
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple, IqInternalError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, IqInternalError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, IqInternalError> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, IqInternalError> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, IqInternalError> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, IqInternalError> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}
impl ser::SerializeSeq for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_element<T>(
        &mut self,
        _value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.count += 1;
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        self.finish()
    }
}

impl ser::SerializeTuple for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_element<T>(
        &mut self,
        _value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.count += 1;
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        self.finish()
    }
}

impl ser::SerializeTupleStruct for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;

    fn serialize_field<T>(
        &mut self,
        _value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.count += 1;
        Ok(())
    }

    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

// TODO not sure I correctly handled this thing
impl ser::SerializeTupleVariant for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;

    fn serialize_field<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_key<T>(
        &mut self,
        _key: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.count += 1;
        Ok(())
    }
    fn serialize_value<T>(
        &mut self,
        _value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        self.finish()
    }
}

impl ser::SerializeStruct for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.count += 1;
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        self.finish()
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl ser::SerializeStructVariant for &mut Sizer {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.count += 1;
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        self.finish()
    }
}

#[test]
fn test_sizer() {
    assert_eq!(Sizer::count([1, 2, 3]), Some(3));
    assert_eq!(Sizer::count("abcdefg"), Some(7));
    assert_eq!(Sizer::count(()), Some(0));
    assert_eq!(Sizer::count((0, 1)), Some(2));
    assert_eq!(Sizer::count(5), None);
    assert_eq!(Sizer::count(&None::<String>), None);
}

use {
    crate::{
        errors::IqInternalError,
        *,
    },
    serde::{
        Serialize,
        ser,
    },
};

/// The thing wich dives into a Serialize value and goes directly
/// to the searched value.
pub(crate) struct Diver<'p> {
    keys: &'p [&'p str],
    next_token: usize,
    requested_seq_idx: usize,
    current_seq_idx: usize,
    return_next_primitive: bool,
    accept_next_map_value: bool,
    return_next_map_value: bool,
    format: IqFormat,
}
impl<'p> Diver<'p> {
    pub fn new(
        keys: &'p [&'p str],
        format: IqFormat,
    ) -> Self {
        Self {
            keys,
            next_token: 0,
            requested_seq_idx: 0,
            current_seq_idx: 0,
            return_next_primitive: false,
            return_next_map_value: false,
            accept_next_map_value: false,
            format,
        }
    }
    fn has_next_token(
        &self,
        key: &str,
    ) -> bool {
        self.next_token < self.keys.len() && key == self.keys[self.next_token]
    }
    fn on_found_with_value<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        match self.format {
            IqFormat::Primitive => {
                self.return_next_primitive = true;
                Ok(())
            }
            IqFormat::Json => {
                let json = serde_json::to_string(value)?;
                Err(IqInternalError::Found(json))
            }
            IqFormat::JsonPretty => {
                let json = serde_json::to_string_pretty(value)?;
                Err(IqInternalError::Found(json))
            }
            IqFormat::Size => match Sizer::count(value) {
                Some(count) => Err(IqInternalError::Count(count)),
                None => Err(IqInternalError::NoCount),
            },
        }
    }
    fn incr_next_token_with_value<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        self.next_token += 1;
        if self.next_token >= self.keys.len() {
            return self.on_found_with_value(value);
        }
        Ok(())
    }
}
impl ser::Serializer for &mut Diver<'_> {
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
        v: bool,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_i8(
        self,
        v: i8,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_i16(
        self,
        v: i16,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_i32(
        self,
        v: i32,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_i64(
        self,
        v: i64,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_u8(
        self,
        v: u8,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_u16(
        self,
        v: u16,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_u32(
        self,
        v: u32,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_u64(
        self,
        v: u64,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_f32(
        self,
        v: f32,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_f64(
        self,
        v: f64,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_char(
        self,
        v: char,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(format!("{}", v)));
        }
        Ok(())
    }
    fn serialize_str(
        self,
        v: &str,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(v.to_string()));
        }
        Ok(())
    }
    fn serialize_bytes(
        self,
        _v: &[u8],
    ) -> Result<(), IqInternalError> {
        // FIXME not yet implemented
        Ok(())
    }
    fn serialize_none(self) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found("none".to_string()));
        }
        Ok(())
    }
    fn serialize_some<T>(
        self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found("unit".to_string()));
        }
        Ok(())
    }
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<(), IqInternalError> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), IqInternalError> {
        if self.return_next_primitive {
            return Err(IqInternalError::Found(variant.to_string()));
        }
        self.serialize_str(variant)
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
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, IqInternalError> {
        self.requested_seq_idx = self
            .keys
            .get(self.next_token)
            .ok_or(IqInternalError::OutOfBounds)?
            .parse()
            .map_err(|_| IqInternalError::IndexExpected)?;
        self.current_seq_idx = 0;
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
impl ser::SerializeSeq for &mut Diver<'_> {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_element<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        if self.current_seq_idx == self.requested_seq_idx {
            self.incr_next_token_with_value(value)?;
            value.serialize(&mut **self)?;
        }
        self.current_seq_idx += 1;
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Diver<'_> {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_element<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        if self.current_seq_idx == self.requested_seq_idx {
            self.incr_next_token_with_value(value)?;
            value.serialize(&mut **self)?;
        }
        self.current_seq_idx += 1;
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Diver<'_> {
    type Ok = ();
    type Error = IqInternalError;

    fn serialize_field<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        if self.current_seq_idx == self.requested_seq_idx {
            self.incr_next_token_with_value(value)?;
            value.serialize(&mut **self)?;
        }
        self.current_seq_idx += 1;
        Ok(())
    }

    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

// TODO not sure I correctly handled this thing
impl ser::SerializeTupleVariant for &mut Diver<'_> {
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

impl ser::SerializeMap for &mut Diver<'_> {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_key<T>(
        &mut self,
        _key: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        // The key can be anything. For the purpose of comparing with the path,
        // we'll take the JSON representation of the key, with quotes removed.
        // For complex composite keys, a specific query language might be needed.
        let key = serde_json::to_string(_key)?;
        self.accept_next_map_value =
            key.trim_matches('"') == self.keys[self.next_token].trim_matches('"');
        if self.accept_next_map_value {
            self.next_token += 1;
            if self.next_token >= self.keys.len() {
                self.return_next_map_value = true;
            }
        }
        Ok(())
    }
    fn serialize_value<T>(
        &mut self,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        if !self.accept_next_map_value {
            return Ok(());
        }
        if self.return_next_map_value {
            self.on_found_with_value(value)?;
        }
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Diver<'_> {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        if self.has_next_token(key) {
            self.incr_next_token_with_value(value)?;
            value.serialize(&mut **self)?;
        }
        Ok(())
    }
    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl ser::SerializeStructVariant for &mut Diver<'_> {
    type Ok = ();
    type Error = IqInternalError;
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), IqInternalError>
    where
        T: ?Sized + Serialize,
    {
        if self.has_next_token(key) {
            self.incr_next_token_with_value(value)?;
        }
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), IqInternalError> {
        Ok(())
    }
}

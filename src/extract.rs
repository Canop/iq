use {
    crate::{
        diver::Diver,
        errors::IqInternalError,
        *,
    },
    serde::{
        Serialize,
        de::DeserializeOwned,
    },
};

/// Format for the extracted value
///
/// Warning: this enum is expected to change. Prefer to use the public
/// functions to extract values.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IqFormat {
    /// Exctract as Display, but only if the value is a "primitive"
    Primitive,
    /// Extract as JSON
    Json,
    /// Extract as JSON, pretty
    JsonPretty,
    /// Extract the size of the array/map/struct/tupple/string at the end of the path
    Size,
}

/// Extract a string from a structure at a given path, with a given format.
///
/// If the path is not found, or empty, return None.
///
/// May theorethically return an error (eg if structure serialization fails),
/// but most users should probably use one of the simpler other functions.
pub fn extract_string_checked<T: Serialize, P: IqPath>(
    source: &T,
    path: P,
    format: IqFormat,
) -> Result<Option<String>, IqError> {
    let keys: Vec<&str> = path.keys().collect();
    let mut diver = Diver::new(&keys, format);
    match source.serialize(&mut diver) {
        Ok(()) => Ok(None), // Not found
        Err(IqInternalError::Found(json)) => Ok(Some(json)),
        Err(IqInternalError::Message(msg)) => Err(IqError::Serde(msg)),
        Err(IqInternalError::Json(err)) => Err(IqError::Json(err)),
        Err(IqInternalError::IndexExpected) => Ok(None), // path doesn't match
        Err(IqInternalError::OutOfBounds) => Ok(None),   // path doesn't match
        Err(IqInternalError::Count(n)) => Ok(Some(n.to_string())),
        Err(IqInternalError::NoCount) => Ok(None),
    }
}
/// Extract a string from a structure at a given path, with a given format.
///
/// If the path is not found, or empty, return None.
///
/// This function also returns None if the the `Serialize` implementation fails,
/// which should not happen with a standard implementation.
pub fn extract_string<T: Serialize, P: IqPath>(
    source: &T,
    path: P,
    format: IqFormat,
) -> Option<String> {
    extract_string_checked(source, path, format).unwrap_or(None)
}

/// Extract a value as JSON
pub fn extract_json<T: Serialize, P: IqPath>(
    source: &T,
    path: P,
) -> Option<String> {
    extract_string(source, path, IqFormat::Json)
}

/// Extract a value as JSON, pretty
pub fn extract_json_pretty<T: Serialize, P: IqPath>(
    source: &T,
    path: P,
) -> Option<String> {
    extract_string(source, path, IqFormat::JsonPretty)
}

/// Extract a "primitive" value (including strings, simple enum variants, etc)
/// as a string using the `Display` implementation of the deep value.
pub fn extract_primitive<T: Serialize, P: IqPath>(
    source: &T,
    path: P,
) -> Option<String> {
    extract_string(source, path, IqFormat::Primitive)
}

/// Extract a value, which must implement `Deserialize`, from a value, at
/// the given path.
///
/// This function uses a JSON representation of the deep value as intermediate
/// step, which adds some (usually light) overload but also allows to extract
/// in a different type than the real type of the deep value.
pub fn extract_value<T: Serialize, P: IqPath, V: DeserializeOwned>(
    source: &T,
    path: P,
) -> Result<Option<V>, IqError> {
    let json = extract_string_checked(source, path, IqFormat::Json)?;
    let value = json.map(|json| serde_json::from_str(&json)).transpose()?;
    Ok(value)
}

/// Extract the size of the array/map/struct/tupple/string at the end of the path
pub fn extract_size<T: Serialize, P: IqPath>(
    source: &T,
    path: P,
) -> Option<usize> {
    let keys: Vec<&str> = path.keys().collect();
    if keys.first().map_or(true, |s| s.is_empty()) {
        return Sizer::count(source);
    }
    let mut diver = Diver::new(&keys, IqFormat::Size);
    match source.serialize(&mut diver) {
        Err(IqInternalError::Count(n)) => Some(n),
        Err(IqInternalError::NoCount) => None, // not countable
        _ => None,                             // not found, or object not serializable
    }
}

/// Extract the size of the array/map/struct/tupple/string of the given value
pub fn size_of<T: Serialize>(source: &T) -> Option<usize> {
    Sizer::count(source)
}

#[test]
fn test_extract_size(){
    #[derive(Debug, PartialEq, Serialize)]
    struct Thing {
        pub coord: (&'static str, i16),
        pub name: String,
        pub v: Vec<i16>,
    }
    let thing = Thing {
        coord: ("Earth", 4),
        name: "some name".to_string(),
        v: vec![1, 2, 3, 4],
    };
    assert_eq!(extract_size(&thing, "coord").unwrap(), 2);
    assert_eq!(extract_size(&thing, "coord.0").unwrap(), 5);
    assert_eq!(extract_size(&thing, vec!["coord", "0"]).unwrap(), 5);
    assert_eq!(extract_size(&thing, "name").unwrap(), 9);
    assert_eq!(extract_size(&thing, "v").unwrap(), 4);
    assert_eq!(extract_size(&thing, "").unwrap(), 3);
    assert_eq!(extract_size(&thing, vec![]).unwrap(), 3);
}

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
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IqFormat {
    /// Exctract as Display, but only if the value is a "primitive"
    Primitive,
    /// Extract as JSON
    Json,
    /// Extract as JSON, pretty
    JsonPretty,
}

/// Extract a string from a structure at a given path, with a given format.
///
/// If the path is not found, or empty, return None.
///
/// May theorethically return an error (eg if structure serialization fails),
/// but most users should probably use one of the simpler other functions.
pub fn extract_string_checked<T: Serialize, P: Into<IqPath>>(
    source: &T,
    path: P,
    format: IqFormat,
) -> Result<Option<String>, IqError> {
    let path = path.into();
    if path.keys.is_empty() {
        // we can't return the complete structure because we
        // would need to determine if it's a primitive
        return Ok(None);
    }
    let mut diver = Diver::new(path, format);
    match source.serialize(&mut diver) {
        Ok(()) => Ok(None), // Not found
        Err(IqInternalError::Found(json)) => Ok(Some(json)),
        Err(IqInternalError::Message(msg)) => Err(IqError::Serde(msg)),
        Err(IqInternalError::Json(err)) => Err(IqError::Json(err)),
        Err(IqInternalError::IndexExpected) => Ok(None), // path doesn't match
    }
}
/// Extract a string from a structure at a given path, with a given format.
///
/// If the path is not found, or empty, return None.
///
/// This function also returns None if the the Serialize implementation fails,
/// which should not happen with a standard implementation.
pub fn extract_string<T: Serialize, P: Into<IqPath>>(
    source: &T,
    path: P,
    format: IqFormat,
) -> Option<String> {
    extract_string_checked(source, path, format).unwrap_or(None)
}

/// Extract a value as JSON
pub fn extract_json<T: Serialize, P: Into<IqPath>>(
    source: &T,
    path: P,
) -> Option<String> {
    extract_string(source, path, IqFormat::Json)
}

/// Extract a value as JSON, pretty
pub fn extract_json_pretty<T: Serialize, P: Into<IqPath>>(
    source: &T,
    path: P,
) -> Option<String> {
    extract_string(source, path, IqFormat::JsonPretty)
}

/// Extract a "primitive" value (including strings, simple enum variants, etc)
/// as a string using the Display implementation of the deep value.
pub fn extract_primitive<T: Serialize, P: Into<IqPath>>(
    source: &T,
    path: P,
) -> Option<String> {
    extract_string(source, path, IqFormat::Primitive)
}

/// Extract a value, which must implement Deserialize, from a value, at
/// the given path.
///
/// This function uses a JSON representation of the deep value as intermediate
/// step, which adds some (usually light) overload but also allows to extract
/// in a different type than the real type of the deep value.
pub fn extract_value<T: Serialize, P: Into<IqPath>, V: DeserializeOwned>(
    source: &T,
    path: P,
) -> Result<Option<V>, IqError> {
    let json = extract_string_checked(source, path, IqFormat::Json)?;
    let value = json.map(|json| serde_json::from_str(&json)).transpose()?;
    Ok(value)
}

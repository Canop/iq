use {
    serde::ser,
    std::fmt,
};

/// Internal error type for the extract operation.
#[derive(Debug)]
pub(crate) enum IqInternalError {
    Message(String),
    Json(serde_json::Error),
    Found(String),
    IndexExpected,
}
impl std::error::Error for IqInternalError {}
impl ser::Error for IqInternalError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
impl From<serde_json::Error> for IqInternalError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
impl fmt::Display for IqInternalError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Self::Message(msg) => write!(formatter, "IQ Error: {}", msg),
            Self::Json(err) => write!(formatter, "IQ Error: JSON: {}", err),
            Self::IndexExpected => write!(formatter, "IQ Error: Index expected"),
            Self::Found(_) => write!(formatter, "IQ Error: Found"),
        }
    }
}

/// Error
#[derive(Debug)]
pub enum IqError {
    Serde(String),
    Json(serde_json::Error),
}
impl std::error::Error for IqError {}
impl From<serde_json::Error> for IqError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
impl fmt::Display for IqError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Self::Serde(msg) => write!(formatter, "Serde Error: {}", msg),
            Self::Json(err) => write!(formatter, "JSON error: {}", err),
        }
    }
}
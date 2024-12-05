/// A path defining a deep destination into a value.
///
/// Searching with an empty path will always return None.
#[derive(Debug)]
pub struct IqPath {
    pub keys: Vec<String>,
}
impl From<&str> for IqPath {
    fn from(path: &str) -> Self {
        Self {
            keys: path.split('.').map(|s| s.to_string()).collect(),
        }
    }
}

impl<S> From<Vec<S>> for IqPath
where
    S: Into<String>,
{
    fn from(path: Vec<S>) -> Self {
        Self {
            keys: path.into_iter().map(|s| s.into()).collect(),
        }
    }
}

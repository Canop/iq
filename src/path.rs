/// A path defining a deep destination into a value.
///
/// Searching with an empty path will always return None.
pub trait IqPath {
    fn keys(&self) -> impl Iterator<Item = &str>;

    /// Build from any implementation of the trait a canonical parsed
    /// version which is a little faster to use (for when you need to
    /// use the same iq_path multiple times).
    fn iq_path(&self) -> Vec<String> {
        self.keys().map(|s| s.to_string()).collect()
    }
}

impl IqPath for &Vec<String> {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.iter().map(|s| s.as_str())
    }
}
impl IqPath for &Vec<&str> {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.iter().copied()
    }
}
impl IqPath for &[String] {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.iter().map(|s| s.as_str())
    }
}
impl<const N: usize> IqPath for &[&str; N] {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.iter().copied()
    }
}
impl IqPath for &[&str] {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.iter().copied()
    }
}
impl IqPath for Vec<&str> {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.iter().copied()
    }
}
impl IqPath for &str {
    fn keys(&self) -> impl Iterator<Item = &str> {
        self.split('.')
    }
}

use {
    crate::*,
    serde::{
        Serialize,
        de::DeserializeOwned,
    },
};

/// A trait to import if you want extract function on any `Serialize` type.
pub trait IQ {
    /// Extract a "primitive" value (including strings, simple enum variants, etc)
    /// as a string using the Display implementation of the deep value.
    fn extract_primitive<P: IqPath>(
        &self,
        path: P,
    ) -> Option<String>;

    /// Extract a value as JSON
    fn extract_json<P: IqPath>(
        &self,
        path: P,
    ) -> Option<String>;

    /// Extract a value as JSON, pretty
    fn extract_json_pretty<P: IqPath>(
        &self,
        path: P,
    ) -> Option<String>;

    /// Extract a value in a type which must implement `Deserialize`, from a value, at
    /// the given path.
    ///
    /// This function uses a JSON representation of the deep value as intermediate
    /// step, which adds some (usually light) overload but also allows to extract
    /// in a different type than the real type of the deep value.
    fn extract_value<P: IqPath, V: DeserializeOwned>(
        &self,
        path: P,
    ) -> Result<Option<V>, IqError>;

    /// Extract the size of the string/tuple/map/vec/struct at the given path
    fn extract_size<P: IqPath>(
        &self,
        path: P,
    ) -> Option<usize>;
}

impl<T> IQ for T
where
    T: Serialize,
{
    fn extract_primitive<P: IqPath>(
        &self,
        path: P,
    ) -> Option<String> {
        extract_primitive(self, path)
    }

    fn extract_json<P: IqPath>(
        &self,
        path: P,
    ) -> Option<String> {
        extract_json(self, path)
    }

    fn extract_json_pretty<P: IqPath>(
        &self,
        path: P,
    ) -> Option<String> {
        extract_json(self, path)
    }

    fn extract_value<P: IqPath, V: DeserializeOwned>(
        &self,
        path: P,
    ) -> Result<Option<V>, IqError> {
        extract_value(self, path)
    }

    fn extract_size<P: IqPath>(
        &self,
        path: P,
    ) -> Option<usize> {
        extract_size(self, path)
    }
}

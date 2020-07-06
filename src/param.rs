use thiserror::Error;

/// Trait for a single request parameter.
pub trait Param {
    /// Return a tuple of this parameter as a key-value pair.
    fn as_tuple(&self) -> (String, String) {
        self.try_as_tuple().unwrap()
    }

    /// Try to return a tuple of this parameter as a key-value pair.
    fn try_as_tuple(&self) -> Result<(String, String), ParamError>;
}

/// Wrapper around possible errors that might be encountered when parsing
/// `Param` implementors.
#[derive(Debug, Error)]
pub enum ParamError {
    /// A parameter could not be parsed into a JSON string.
    #[error("Parameter '{0}' contains invalid JSON: {1}")]
    InvalidJson(String, serde_json::error::Error),
}

/// Trait for a list of request parameters.
///
/// Each implementor of this should have an internal list of parameters.
pub trait ParamList {
    type ParamType: Param;

    /// Add a new `Param` to the parameter list.
    fn add(self, param: Self::ParamType) -> Self;
}

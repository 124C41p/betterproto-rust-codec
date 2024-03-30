use pyo3::{exceptions::PyRuntimeError, PyErr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InteropError {
    #[error("Given object is not a valid betterproto message.")]
    NoBetterprotoMessage(#[from] PyErr),
    #[error("Unsupported value type `{0}`.")]
    UnsupportedValueType(String),
    #[error("Unsupported key type `{0:?}`.")]
    UnsupportedKeyType(String),
    #[error("Unsupported wrapped type `{0:?}`.")]
    UnsupportedWrappedType(String),
    #[error("Given object is not a valid betterproto message.")]
    IncompleteMetadata,
    #[error("Offset-naive datetime {0} is invalid for the current local timezone.")]
    OffsetNaiveDateTimeDoesNotMap(chrono::NaiveDateTime),
}

pub type InteropResult<T> = Result<T, InteropError>;

impl From<InteropError> for PyErr {
    fn from(value: InteropError) -> Self {
        PyRuntimeError::new_err(value.to_string())
    }
}

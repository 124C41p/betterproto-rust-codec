use crate::betterproto_interop::InteropError;
use pyo3::{exceptions::PyRuntimeError, DowncastError, PyErr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error(transparent)]
    PythonInteropFailed(#[from] PyErr),
    #[error("Given object is not a valid betterproto message.")]
    DowncastFailed,
    #[error(transparent)]
    Interop(#[from] InteropError),
    #[error("Given object is not a valid betterproto message.")]
    ProstEncode(#[from] prost::EncodeError),
    #[error("Offset-naive datetime {0} is invalid for the current local timezone.")]
    OffsetNaiveDateTimeDoesNotMap(chrono::NaiveDateTime),
}

pub type EncodeResult<T> = Result<T, EncodeError>;

impl From<DowncastError<'_, '_>> for EncodeError {
    fn from(_: DowncastError) -> Self {
        Self::DowncastFailed
    }
}

impl From<EncodeError> for PyErr {
    fn from(value: EncodeError) -> Self {
        if let EncodeError::PythonInteropFailed(pyerr) = value {
            pyerr
        } else {
            PyRuntimeError::new_err(value.to_string())
        }
    }
}

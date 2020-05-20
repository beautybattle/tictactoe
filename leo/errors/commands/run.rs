use crate::errors::ManifestError;

use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("main file {:?} does not exist", _0)]
    MainFileDoesNotExist(OsString),

    #[error("{}", _0)]
    ManifestError(ManifestError),
}

impl From<ManifestError> for RunError {
    fn from(error: ManifestError) -> Self {
        RunError::ManifestError(error)
    }
}
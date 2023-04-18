//! Errors

use std::{
    fmt::{self, Display},
    io,
};

/// It handles internal, I/O and formatter errors
#[non_exhaustive]
#[derive(Debug)]
pub enum MoshError {
    /// Data format is not supported.
    DecodingError(png::DecodingError),
    /// i.e. wrong data size/formatter failure.
    EncodingError(png::EncodingError),
    /// I/O errors.
    IoError(io::Error),
    /// Allocation failed.
    OutOfMemory,
    /// Unsupported color type.
    UnsupportedColorType,
}

impl std::error::Error for MoshError {}

impl Display for MoshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodingError(e) => Display::fmt(e, f),
            Self::EncodingError(e) => Display::fmt(e, f),
            Self::IoError(e) => Display::fmt(e, f),
            Self::OutOfMemory => f.write_str("Out of memory"),
            Self::UnsupportedColorType => f.write_str("Unsupported color type"),
        }
    }
}

impl From<io::Error> for MoshError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<png::DecodingError> for MoshError {
    fn from(e: png::DecodingError) -> Self {
        Self::DecodingError(e)
    }
}

impl From<png::EncodingError> for MoshError {
    fn from(e: png::EncodingError) -> Self {
        Self::EncodingError(e)
    }
}

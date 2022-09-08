use std::{fmt, io};

#[derive(Debug)]
pub enum MoshError {
    DecodingError(String),
    EncodingError(String),
    InvalidParameters,
    IoError(String),
    OutOfMemory,
    UnsupportedColorType,
}

impl std::error::Error for MoshError {}

impl fmt::Display for MoshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::DecodingError(e) | Self::EncodingError(e) | Self::IoError(e) => e,
            Self::InvalidParameters => "Invalid parameters",
            Self::OutOfMemory => "Out of memory",
            Self::UnsupportedColorType => "Unsupported color type",
        })
    }
}

impl From<io::Error> for MoshError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

impl From<png::DecodingError> for MoshError {
    fn from(e: png::DecodingError) -> Self {
        Self::DecodingError(e.to_string())
    }
}

impl From<png::EncodingError> for MoshError {
    fn from(e: png::EncodingError) -> Self {
        Self::EncodingError(e.to_string())
    }
}

impl From<resize::Error> for MoshError {
    fn from(e: resize::Error) -> Self {
        use resize::Error::{InvalidParameters, OutOfMemory};
        match e {
            InvalidParameters => Self::InvalidParameters,
            OutOfMemory => Self::OutOfMemory,
        }
    }
}

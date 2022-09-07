use std::fmt;

#[derive(Debug)]
pub enum MoshError {
    InvalidParameters,
    OutOfMemory,
    UnsupportedColorType,
}

impl std::error::Error for MoshError {}

impl fmt::Display for MoshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidParameters => "Invalid parameters",
            Self::OutOfMemory => "Out of memory",
            Self::UnsupportedColorType => "Unsupported color type",
        })
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

//! PostScript (CFF and CFF2) common tables.

use std::fmt;

mod blend;
mod index;

include!("../../generated/generated_postscript.rs");

pub use blend::BlendState;
pub use index::Index;

/// Errors that are specific to PostScript processing.
#[derive(Clone, Debug)]
pub enum Error {
    /// The `off_size` field in an INDEX contained an invalid value.
    InvalidIndexOffsetSize(u8),
    /// An INDEX contained a zero offset.
    ZeroOffsetInIndex,
    /// Variation store index referenced an invalid variation region.
    InvalidVariationStoreIndex(u16),
    /// Underlying parsing error.
    Read(ReadError),
}

impl From<ReadError> for Error {
    fn from(value: ReadError) -> Self {
        Self::Read(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIndexOffsetSize(size) => {
                write!(f, "invalid offset size of {size} for INDEX (expected 1-4)")
            }
            Self::ZeroOffsetInIndex => {
                write!(f, "invalid offset of 0 in INDEX (must be >= 1)")
            }
            Self::InvalidVariationStoreIndex(index) => {
                write!(
                    f,
                    "variation store index {index} referenced an invalid variation region"
                )
            }
            Self::Read(err) => write!(f, "{err}"),
        }
    }
}

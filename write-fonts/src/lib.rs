//! Raw types for compiling opentype tables

mod collections;
mod font_builder;
pub mod from_obj;
mod graph;
pub mod layout;
mod offsets;
pub mod tables;
pub mod validate;
mod write;

#[cfg(test)]
mod codegen_test;
#[cfg(test)]
mod hex_diff;

pub use font_builder::FontBuilder;
pub use offsets::{NullableOffsetMarker, OffsetMarker};
pub use write::{dump_table, FontWrite, TableWriter};

/// types used in autogenerated code.
pub(crate) mod codegen_prelude {
    use std::num::TryFromIntError;

    pub use super::from_obj::{FromObjRef, FromTableRef, ToOwnedObj, ToOwnedTable};
    pub use super::offsets::{NullableOffsetMarker, OffsetMarker, WIDTH_16, WIDTH_24, WIDTH_32};
    pub use super::validate::{Validate, ValidationCtx};
    pub use super::write::{FontWrite, TableWriter};
    pub use font_types::*;
    pub use std::collections::BTreeSet;

    pub use read_fonts::{
        FontData, FontRead, FontReadWithArgs, ReadArgs, ReadError, ResolveOffset,
    };

    /// checked conversion to u16
    pub fn array_len<T: super::collections::HasLen>(s: &T) -> Result<u16, TryFromIntError> {
        s.checked_len()
    }

    pub fn plus_one(val: &usize) -> Result<u16, TryFromIntError> {
        val.saturating_add(1).try_into()
    }
}

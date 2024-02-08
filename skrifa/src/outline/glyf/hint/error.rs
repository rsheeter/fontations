//! Hinting error definitions.

use read_fonts::tables::glyf::bytecode::{DecodeError, Opcode};

use super::code_state::ProgramKind;
use crate::GlyphId;

/// Errors that may occur when interpreting TrueType bytecode.
#[derive(Clone, Debug)]
pub enum HintErrorKind {
    UnexpectedEndOfBytecode,
    UnhandledOpcode(Opcode),
    DefinitionInGlyphProgram,
    NestedDefinition,
    InvalidDefintionIndex(usize),
    ValueStackOverflow,
    ValueStackUnderflow,
    CallStackOverflow,
    CallStackUnderflow,
    InvalidStackValue(i32),
    InvalidPointIndex(usize),
    InvalidPointRange(usize, usize),
    InvalidContourIndex(usize),
    InvalidCvtIndex(usize),
    InvalidStorageIndex(usize),
    DivideByZero,
    InvalidZoneIndex(i32),
    NegativeLoopCounter,
    InvalidJump,
    ExceededExecutionBudget,
}

impl core::fmt::Display for HintErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfBytecode => write!(f, "unexpected end of bytecode"),
            Self::UnhandledOpcode(opcode) => write!(f, "unhandled instruction opcode {opcode}"),
            Self::DefinitionInGlyphProgram => {
                write!(f, "FDEF or IDEF instruction present in glyph program")
            }
            Self::NestedDefinition => write!(
                f,
                "FDEF or IDEF instruction present in another FDEF or IDEF block"
            ),
            Self::InvalidDefintionIndex(index) => write!(
                f,
                "invalid function or instruction definition index {index}"
            ),
            Self::ValueStackOverflow => write!(f, "value stack overflow"),
            Self::ValueStackUnderflow => write!(f, "value stack underflow"),
            Self::CallStackOverflow => write!(f, "call stack overflow"),
            Self::CallStackUnderflow => write!(f, "call stack underflow"),
            Self::InvalidStackValue(value) => write!(
                f,
                "stack value {value} was invalid for the current operation"
            ),
            Self::InvalidPointIndex(index) => write!(f, "point index {index} was out of bounds"),
            Self::InvalidPointRange(start, end) => {
                write!(f, "point range {start}..{end} was out of bounds")
            }
            Self::InvalidContourIndex(index) => {
                write!(f, "contour index {index} was out of bounds")
            }
            Self::InvalidCvtIndex(index) => write!(f, "cvt index {index} was out of bounds"),
            Self::InvalidStorageIndex(index) => {
                write!(f, "storage area index {index} was out of bounds")
            }
            Self::DivideByZero => write!(f, "attempt to divide by 0"),
            Self::InvalidZoneIndex(index) => write!(
                f,
                "zone index {index} was invalid (only 0 or 1 are permitted)"
            ),
            Self::NegativeLoopCounter => {
                write!(f, "attempt to set the loop counter to a negative value")
            }
            Self::InvalidJump => write!(f, "the target of a jump instruction was invalid"),
            Self::ExceededExecutionBudget => write!(f, "too many instructions executed"),
        }
    }
}

impl From<DecodeError> for HintErrorKind {
    fn from(_: DecodeError) -> Self {
        Self::UnexpectedEndOfBytecode
    }
}

/// Hinting error with additional context.
#[derive(Clone, Debug)]
pub struct HintError {
    pub program: ProgramKind,
    pub glyph_id: Option<GlyphId>,
    pub pc: usize,
    pub kind: HintErrorKind,
}

impl core::fmt::Display for HintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.program {
            ProgramKind::ControlValue => write!(f, "prep")?,
            ProgramKind::Font => write!(f, "fpgm")?,
            ProgramKind::Glyph => {
                write!(f, "glyf[")?;
                if let Some(glyph_id) = self.glyph_id {
                    write!(f, "{}", glyph_id.to_u16())?;
                } else {
                    write!(f, "?")?;
                }
                write!(f, "]")?
            }
        }
        write!(f, "+{}: {}", self.pc, self.kind)
    }
}

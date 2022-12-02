//! common test data.
//!
//! Much of this is taken from the OpenType spec, although some is hand-written.
//!
//! This is in a module so that it can be shared between crates.

pub mod gpos {
    use crate::FontData;

    #[rustfmt::skip]
    pub static SINGLEPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x08, 0x00, 0x02, 0xFF, 0xB0, 0x00, 0x02, 0x00,
        0x01, 0x01, 0xB3, 0x01, 0xBC, 0x00, 0x00,
    ]);

    #[rustfmt::skip]
    pub static SINGLEPOSFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x14, 0x00, 0x05, 0x00, 0x03, 0x00, 0x32, 0x00,
        0x32, 0x00, 0x19, 0x00, 0x19, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x01,
        0x00, 0x03, 0x00, 0x4F, 0x01, 0x25, 0x01, 0x29,
    ]);

    #[rustfmt::skip]
    pub static PAIRPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x1E, 0x00, 0x04, 0x00, 0x01, 0x00, 0x02, 0x00,
        0x0E, 0x00, 0x16, 0x00, 0x01, 0x00, 0x59, 0xFF, 0xE2, 0xFF, 0xEC,
        0x00, 0x01, 0x00, 0x59, 0xFF, 0xD8, 0xFF, 0xE7, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x2D, 0x00, 0x31,
    ]);

    #[rustfmt::skip]
    pub static PAIRPOSFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x18, 0x00, 0x04, 0x00, 0x00, 0x00, 0x22, 0x00,
        0x32, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xCE, 0x00, 0x01, 0x00, 0x03, 0x00, 0x46, 0x00, 0x47, 0x00,
        0x49, 0x00, 0x02, 0x00, 0x02, 0x00, 0x46, 0x00, 0x47, 0x00, 0x01,
        0x00, 0x49, 0x00, 0x49, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x6A, 0x00, 0x6B, 0x00, 0x01,
    ]);

    #[rustfmt::skip]
    pub static CURSIVEPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0E, 0x00, 0x02, 0x00, 0x16, 0x00, 0x1C, 0x00,
        0x22, 0x00, 0x28, 0x00, 0x01, 0x00, 0x02, 0x02, 0x03, 0x02, 0x7E,
        0x00, 0x01, 0x05, 0xDC, 0x00, 0x2C, 0x00, 0x01, 0x00, 0x00, 0xFF,
        0xEC, 0x00, 0x01, 0x05, 0xDC, 0x00, 0x2C, 0x00, 0x01, 0x00, 0x00,
        0xFF, 0xEC,
    ]);

    #[rustfmt::skip]
    pub static MARKBASEPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0C, 0x00, 0x14, 0x00, 0x02, 0x00, 0x1A, 0x00,
        0x30, 0x00, 0x01, 0x00, 0x02, 0x03, 0x33, 0x03, 0x3F, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x90, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0A, 0x00,
        0x01, 0x00, 0x10, 0x00, 0x01, 0x01, 0x5A, 0xFF, 0x9E, 0x00, 0x01,
        0x01, 0x05, 0x00, 0x58, 0x00, 0x01, 0x00, 0x06, 0x00, 0x0C, 0x00,
        0x01, 0x03, 0x3E, 0x06, 0x40, 0x00, 0x01, 0x03, 0x3E, 0xFF, 0xAD,
    ]);

    #[rustfmt::skip]
    pub static MARKLIGPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0C, 0x00, 0x14, 0x00, 0x02, 0x00, 0x1A, 0x00,
        0x30, 0x00, 0x01, 0x00, 0x02, 0x03, 0x3C, 0x03, 0x3F, 0x00, 0x01,
        0x00, 0x01, 0x02, 0x34, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0A, 0x00,
        0x01, 0x00, 0x10, 0x00, 0x01, 0x01, 0x5A, 0xFF, 0x9E, 0x00, 0x01,
        0x01, 0x05, 0x01, 0xE8, 0x00, 0x01, 0x00, 0x04, 0x00, 0x03, 0x00,
        0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x02, 0x71, 0x07, 0x08, 0x00, 0x01, 0x01, 0x78, 0xFE,
        0x90,
    ]);

    #[rustfmt::skip]
    pub static MARKMARKPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0C, 0x00, 0x12, 0x00, 0x01, 0x00, 0x18, 0x00,
        0x24, 0x00, 0x01, 0x00, 0x01, 0x02, 0x96, 0x00, 0x01, 0x00, 0x01,
        0x02, 0x89, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00,
        0xBD, 0xFF, 0x99, 0x00, 0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0xDD,
        0x01, 0x2D,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUALPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x01, 0x00,
        0x01, 0x02, 0xA6, 0x00, 0x01, 0x00, 0x04, 0x00, 0x03, 0x00, 0x01,
        0x02, 0xDD, 0x02, 0xC6, 0x00, 0x02, 0x00, 0x01,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUALPOSFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x12, 0x00, 0x20, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x60, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x05,
        0x00, 0x29, 0x00, 0x33, 0x00, 0x37, 0x00, 0x39, 0x00, 0x3A, 0x00,
        0x02, 0x00, 0x0A, 0x00, 0x29, 0x00, 0x29, 0x00, 0x02, 0x00, 0x33,
        0x00, 0x33, 0x00, 0x02, 0x00, 0x37, 0x00, 0x37, 0x00, 0x01, 0x00,
        0x39, 0x00, 0x3A, 0x00, 0x01, 0x00, 0x42, 0x00, 0x42, 0x00, 0x03,
        0x00, 0x46, 0x00, 0x46, 0x00, 0x03, 0x00, 0x4A, 0x00, 0x4A, 0x00,
        0x03, 0x00, 0x51, 0x00, 0x51, 0x00, 0x03, 0x00, 0x56, 0x00, 0x56,
        0x00, 0x03, 0x00, 0xF5, 0x00, 0xF6, 0x00, 0x04, 0x00, 0x01, 0x00,
        0x04, 0x00, 0x03, 0x00, 0x01, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x03, 0x00, 0x01, 0x00,
        0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUALPOSFORMAT3: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x00, 0x10, 0x00, 0x3C, 0x00,
        0x44, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x14, 0x00, 0x33,
        0x00, 0x35, 0x00, 0x37, 0x00, 0x39, 0x00, 0x3B, 0x00, 0x3C, 0x00,
        0x3F, 0x00, 0x40, 0x00, 0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x44,
        0x00, 0x45, 0x00, 0x46, 0x00, 0x47, 0x00, 0x48, 0x00, 0x49, 0x00,
        0x4A, 0x00, 0x4B, 0x00, 0x4C, 0x00, 0x01, 0x00, 0x02, 0x01, 0x1E,
        0x01, 0x2D, 0x00, 0x02, 0x00, 0x01, 0x00, 0x33, 0x00, 0x4C, 0x00,
        0x00,
    ]);

    #[rustfmt::skip]
    pub static SEQUENCELOOKUPRECORD: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x01
    ]);

    #[rustfmt::skip]
    pub static VALUEFORMATTABLE: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0E, 0x00, 0x99, 0x00, 0x50, 0x00, 0xD2,
        0x00, 0x18, 0x00, 0x20, 0x00, 0x02, 0x00, 0x01, 0x00, 0xC8,
        0x00, 0xD1, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x0F, 0x00, 0x01,
        0x55, 0x40, 0x00, 0x0B, 0x00, 0x0F, 0x00, 0x01, 0x55, 0x40,
    ]);

    #[rustfmt::skip]
    pub static ANCHORFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0xBD, 0xFF, 0x99
    ]);

    #[rustfmt::skip]
    pub static ANCHORFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x01, 0x42, 0x03, 0x84, 0x00, 0x0D
    ]);

    #[rustfmt::skip]
    pub static ANCHORFORMAT3: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x01, 0x17, 0x05, 0x15, 0x00, 0x0A, 0x00, 0x14,
        0x00, 0x0C, 0x00, 0x11, 0x00, 0x02, 0x11, 0x11, 0x22, 0x00,
        0x00, 0x0C, 0x00, 0x11, 0x00, 0x02, 0x11, 0x11, 0x22, 0x00,
    ]);
}

pub mod layout {
    use crate::FontData;

    #[rustfmt::skip]
    pub static SCRIPTS: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x68, 0x61, 0x6E, 0x69, 0x00, 0x14, 0x6B, 0x61, 0x6E,
        0x61, 0x00, 0x18, 0x6C, 0x61, 0x74, 0x6E, 0x00, 0x1C,
    ]);

    #[rustfmt::skip]
    pub static SCRIPTS_AND_LANGUAGES: FontData<'static> = FontData::new(&[
        0x00, 0x0A, 0x00, 0x01, 0x55, 0x52, 0x44, 0x20, 0x00, 0x16, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x02,
    ]);

    #[rustfmt::skip]
    pub static FEATURELIST_AND_FEATURE: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x6C, 0x69, 0x67, 0x61, 0x00, 0x14, 0x6C, 0x69, 0x67,
        0x61, 0x00, 0x1A, 0x6C, 0x69, 0x67, 0x61, 0x00, 0x22, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
    ]);
}

pub mod gdef {
    use crate::FontData;

    #[rustfmt::skip]
    pub static GDEF_HEADER: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x26, 0x00, 0x40, 0x00, 0x5A,
    ]);

    #[rustfmt::skip]
    pub static GLYPHCLASSDEF_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x04, 0x00, 0x24, 0x00, 0x24, 0x00, 0x01, 0x00, 0x9F,
        0x00, 0x9F, 0x00, 0x02, 0x00, 0x58, 0x00, 0x58, 0x00, 0x03, 0x01, 0x8F,
        0x01, 0x8F, 0x00, 0x04,
    ]);

    #[rustfmt::skip]
    pub static ATTACHLIST_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x12, 0x00, 0x02, 0x00, 0x08, 0x00, 0x0C, 0x00, 0x01, 0x00, 0x12,
        0x00, 0x02, 0x00, 0x0E, 0x00, 0x17, 0x00, 0x01, 0x00, 0x02, 0x00, 0x1C,
        0x00, 0x20,
    ]);

    #[rustfmt::skip]
    pub static LIGCARETLIST_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x08, 0x00, 0x02, 0x00, 0x10, 0x00, 0x14, 0x00, 0x01, 0x00, 0x02,
        0x00, 0x9F, 0x00, 0xA5, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x02, 0x00, 0x06,
        0x00, 0x0E, 0x00, 0x01, 0x02, 0x5B, 0x00, 0x01, 0x02, 0x5B, 0x00, 0x01,
        0x04, 0xB6,
    ]);

    #[rustfmt::skip]
    pub static CARETVALUEFORMAT3_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x04, 0xB6, 0x00, 0x06, 0x00, 0x0C, 0x00, 0x11, 0x00, 0x02,
        0x11, 0x11, 0x22, 0x00,
    ]);

    #[rustfmt::skip]
    pub static MARKATTACHCLASSDEF_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x04, 0x02, 0x68, 0x02, 0x6A, 0x00, 0x01, 0x02, 0x70,
        0x02, 0x72, 0x00, 0x01, 0x02, 0x8C, 0x02, 0x8F, 0x00, 0x02, 0x02, 0x95,
        0x02, 0x95, 0x00, 0x02,
    ]);
}

pub mod gsub {
    use crate::FontData;

    #[rustfmt::skip]
    pub static SINGLESUBSTFORMAT1_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x06, 0x00, 0xC0, 0x00, 0x02, 0x00, 0x01, 0x00, 0x4E,
        0x00, 0x58, 0x00, 0x00,
    ]);

    #[rustfmt::skip]
    pub static SINGLESUBSTFORMAT2_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x0E, 0x00, 0x04, 0x01, 0x31, 0x01, 0x35, 0x01, 0x3E,
        0x01, 0x43, 0x00, 0x01, 0x00, 0x04, 0x00, 0x3C, 0x00, 0x40, 0x00, 0x4B,
        0x00, 0x4F,
    ]);

    #[rustfmt::skip]
    pub static MULTIPLESUBSTFORMAT1_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x01, 0x00, 0x01,
        0x00, 0xF1, 0x00, 0x03, 0x00, 0x1A, 0x00, 0x1A, 0x00, 0x1D,
    ]);

    #[rustfmt::skip]
    pub static ALTERNATESUBSTFORMAT1_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x01, 0x00, 0x01,
        0x00, 0x3A, 0x00, 0x02, 0x00, 0xC9, 0x00, 0xCA,
    ]);

    #[rustfmt::skip]
    pub static LIGATURESUBSTFORMAT1_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0A, 0x00, 0x02, 0x00, 0x14, 0x00, 0x20, 0x00, 0x02,
        0x00, 0x01, 0x00, 0x19, 0x00, 0x1A, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04,
        0x01, 0x5B, 0x00, 0x03, 0x00, 0x28, 0x00, 0x17, 0x00, 0x02, 0x00, 0x06,
        0x00, 0x0E, 0x00, 0xF1, 0x00, 0x03, 0x00, 0x1A, 0x00, 0x1D, 0x00, 0xF0,
        0x00, 0x02, 0x00, 0x1D,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUAL_SUBSTITUTION_FORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0A, 0x00, 0x02, 0x00, 0x12, 0x00, 0x20, 0x00, 0x01,
        0x00, 0x02, 0x00, 0x28, 0x00, 0x5D, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02,
        0x00, 0x01, 0x00, 0x5D, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x04,
        0x00, 0x02, 0x00, 0x01, 0x00, 0x28, 0x00, 0x01, 0x00, 0x01,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUAL_SUBSTITUTION_FORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x10, 0x00, 0x1C, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x32, 0x00, 0x40, 0x00, 0x01, 0x00, 0x04, 0x00, 0x30, 0x00, 0x31,
        0x00, 0x40, 0x00, 0x41, 0x00, 0x02, 0x00, 0x03, 0x00, 0x30, 0x00, 0x31,
        0x00, 0x02, 0x00, 0x40, 0x00, 0x41, 0x00, 0x03, 0x00, 0xD2, 0x00, 0xD3,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x02,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUAL_SUBSTITUTION_FORMAT3: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x14, 0x00, 0x30, 0x00, 0x52,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x0C,
        0x00, 0x33, 0x00, 0x35, 0x00, 0x37, 0x00, 0x38, 0x00, 0x39, 0x00, 0x3B,
        0x00, 0x3C, 0x00, 0x3D, 0x00, 0x41, 0x00, 0x42, 0x00, 0x45, 0x00, 0x4A,
        0x00, 0x01, 0x00, 0x0F, 0x00, 0x32, 0x00, 0x34, 0x00, 0x36, 0x00, 0x3A,
        0x00, 0x3E, 0x00, 0x3F, 0x00, 0x40, 0x00, 0x43, 0x00, 0x44, 0x00, 0x45,
        0x00, 0x46, 0x00, 0x47, 0x00, 0x48, 0x00, 0x49, 0x00, 0x4B, 0x00, 0x01,
        0x00, 0x05, 0x00, 0x38, 0x00, 0x3B, 0x00, 0x41, 0x00, 0x42, 0x00, 0x4A,
    ]);

    #[rustfmt::skip]
    pub static REVERSECHAINSINGLESUBSTFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x26,
        0x00, 0x0C, 0x00, 0xA7, 0x00, 0xB9, 0x00, 0xC5, 0x00, 0xD4, 0x00, 0xEA,
        0x00, 0xF2, 0x00, 0xFD, 0x01, 0x0D, 0x01, 0x1B, 0x01, 0x2B, 0x01, 0x3B,
        0x01, 0x41, 0x00, 0x01, 0x00, 0x1F, 0x00, 0xA5, 0x00, 0xA9, 0x00, 0xAA,
        0x00, 0xE2, 0x01, 0x67, 0x01, 0x68, 0x01, 0x69, 0x01, 0x6D, 0x01, 0x6E,
        0x01, 0x70, 0x01, 0x83, 0x01, 0x84, 0x01, 0x85, 0x01, 0x89, 0x01, 0x8A,
        0x01, 0x8C, 0x01, 0x9F, 0x01, 0xA0, 0x01, 0xA1, 0x01, 0xA2, 0x01, 0xA3,
        0x01, 0xA4, 0x01, 0xA5, 0x01, 0xA6, 0x01, 0xA7, 0x01, 0xA8, 0x01, 0xA9,
        0x01, 0xAA, 0x01, 0xAB, 0x01, 0xAC, 0x01, 0xEC, 0x00, 0x01, 0x00, 0x0C,
        0x00, 0xA6, 0x00, 0xB7, 0x00, 0xC3, 0x00, 0xD2, 0x00, 0xE9, 0x00, 0xF1,
        0x00, 0xFC, 0x01, 0x0C, 0x01, 0x19, 0x01, 0x29, 0x01, 0x3A, 0x01, 0x40,
    ]);
}

pub mod post {
    use crate::FontData;

    #[rustfmt::skip]
    pub static SIMPLE: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x00, // version 2.0
        0x00, 0x00, 0x00, 0x00, // italic angle
        0xFF, 0xb5,             // underlinePosition -75
        0x00, 0x32,             // underlineThickness 50
        0x00, 0x00, 0x00, 0x00, // fixedpitch
        0x00, 0x00, 0x00, 0x00, // min/max mem:
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x0A,             // numGlyphs 10
                                // glyph name index:
        0x00, 0x00,              // glyph 0 -> name 0
        0x00, 0x00,             // glyph 1 -> name 0
        0x00, 0x03,              // glyph 2 -> name 3 ('space')
        0x00, 0x04,              // glyph 3 -> name 4 ('exclam')
        0x00, 0x06,
        0x00, 0x07,
        0x00, 0x08,
        0x01, 0x02,             // glyph 7 -> name 258 first custom
        0x01, 0x03,             // glyph 8 -> name 258 first custom
        0x01, 0x04,             // glyph 9 -> name 258 first custom
        0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, // 5, h e l l o
        0x02, 0x68, 0x69, // 2, h i
        0x4, 0x68, 0x6f, 0x6c, 0x61, // 4, h o l a
    ]);
}

pub mod cpal {
    use crate::{tables::cpal::Cpal, FontRef, TableProvider};

    use super::test_fonts::COLR_GRADIENT_RECT;

    pub fn sample() -> Cpal<'static> {
        return FontRef::new(COLR_GRADIENT_RECT).unwrap().cpal().unwrap();
    }
}

pub mod os2 {
    use crate::{tables::os2::Os2, FontRef, TableProvider};
    pub fn sample() -> Os2<'static> {
        FontRef::new(super::test_fonts::SIMPLE_GLYF)
            .unwrap()
            .os2()
            .unwrap()
    }
}

pub mod stat {
    use crate::{tables::stat::Stat, FontRef, TableProvider};
    pub fn vazirmatn() -> Stat<'static> {
        FontRef::new(super::test_fonts::VAZIRMATN_VAR)
            .unwrap()
            .stat()
            .unwrap()
    }
}

pub mod test_fonts {
    //! statically bundled test fonts.
    //!
    //! These files are only present if this crate is built from the repo root,
    //! and not when the crate is packaged.
    //!
    //! To add new files, you will need to add the file in resources/test_fonts,
    //! and then update read-fonts/build.rs.

    pub static COLR_GRADIENT_RECT: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/linear_gradient_rect_colr_1.ttf"));

    pub static VAZIRMATN_VAR: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/vazirmatn_var_trimmed.ttf"));

    pub static SIMPLE_GLYF: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/simple_glyf.ttf"));
}

//! test data shared between various fontations crates.

pub mod gdef;
pub mod gpos;
pub mod gsub;
pub mod layout;

pub static CMAP12_FONT1: &[u8] = include_bytes!("../test_data/ttf/cmap12_font1.ttf");

pub static CMAP14_FONT1: &[u8] = include_bytes!("../test_data/ttf/cmap14_font1.ttf");

pub static CMAP4_SYMBOL_PUA: &[u8] = include_bytes!("../test_data/ttf/cmap4_symbol_pua.ttf");

pub static COLR_GRADIENT_RECT: &[u8] =
    include_bytes!("../test_data/ttf/linear_gradient_rect_colr_1.ttf");

pub static VAZIRMATN_VAR: &[u8] = include_bytes!("../test_data/ttf/vazirmatn_var_trimmed.ttf");

pub static NAMES_ONLY: &[u8] = include_bytes!("../test_data/ttf/names_only.ttf");

pub static VAZIRMATN_VAR_GLYPHS: &str =
    include_str!("../test_data/extracted/vazirmatn_var_trimmed-glyphs.txt");

pub static SIMPLE_GLYF: &[u8] = include_bytes!("../test_data/ttf/simple_glyf.ttf");

pub static NOTO_SERIF_DISPLAY_TRIMMED: &[u8] =
    include_bytes!("../test_data/ttf/noto_serif_display_trimmed.ttf");

pub static CANTARELL_VF_TRIMMED: &[u8] =
    include_bytes!("../test_data/ttf/cantarell_vf_trimmed.ttf");

pub static CHARSTRING_PATH_OPS: &[u8] = include_bytes!("../test_data/ttf/charstring_path_ops.ttf");

pub mod post {

    #[rustfmt::skip]
    pub static SIMPLE: &[u8] = &[
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
    ];
}

pub mod cff2 {
    /// CFF2 example table
    /// <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#appendix-a-example-cff2-font>
    pub static EXAMPLE: &[u8] = &[
        0x02, 0x00, 0x05, 0x00, 0x07, 0xCF, 0x0C, 0x24, 0xC3, 0x11, 0x9B, 0x18, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x26, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1C,
        0x00, 0x01, 0x00, 0x02, 0xC0, 0x00, 0xE0, 0x00, 0x00, 0x00, 0xC0, 0x00, 0xC0, 0x00, 0xE0,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
        0x01, 0x01, 0x03, 0x05, 0x20, 0x0A, 0x20, 0x0A, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x05,
        0xF7, 0x06, 0xDA, 0x12, 0x77, 0x9F, 0xF8, 0x6C, 0x9D, 0xAE, 0x9A, 0xF4, 0x9A, 0x95, 0x9F,
        0xB3, 0x9F, 0x8B, 0x8B, 0x8B, 0x8B, 0x85, 0x9A, 0x8B, 0x8B, 0x97, 0x73, 0x8B, 0x8B, 0x8C,
        0x80, 0x8B, 0x8B, 0x8B, 0x8D, 0x8B, 0x8B, 0x8C, 0x8A, 0x8B, 0x8B, 0x97, 0x17, 0x06, 0xFB,
        0x8E, 0x95, 0x86, 0x9D, 0x8B, 0x8B, 0x8D, 0x17, 0x07, 0x77, 0x9F, 0xF8, 0x6D, 0x9D, 0xAD,
        0x9A, 0xF3, 0x9A, 0x95, 0x9F, 0xB3, 0x9F, 0x08, 0xFB, 0x8D, 0x95, 0x09, 0x1E, 0xA0, 0x37,
        0x5F, 0x0C, 0x09, 0x8B, 0x0C, 0x0B, 0xC2, 0x6E, 0x9E, 0x8C, 0x17, 0x0A, 0xDB, 0x57, 0xF7,
        0x02, 0x8C, 0x17, 0x0B, 0xB3, 0x9A, 0x77, 0x9F, 0x82, 0x8A, 0x8D, 0x17, 0x0C, 0x0C, 0xDB,
        0x95, 0x57, 0xF7, 0x02, 0x85, 0x8B, 0x8D, 0x17, 0x0C, 0x0D, 0xF7, 0x06, 0x13, 0x00, 0x00,
        0x00, 0x01, 0x01, 0x01, 0x1B, 0xBD, 0xBD, 0xEF, 0x8C, 0x10, 0x8B, 0x15, 0xF8, 0x88, 0x27,
        0xFB, 0x5C, 0x8C, 0x10, 0x06, 0xF8, 0x88, 0x07, 0xFC, 0x88, 0xEF, 0xF7, 0x5C, 0x8C, 0x10,
        0x06,
    ];
}

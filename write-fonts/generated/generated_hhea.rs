// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [hhea](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea) Horizontal Header Table
#[derive(Clone, Debug, Default)]
pub struct Hhea {
    /// Typographic ascent.
    pub ascender: FWord,
    /// Typographic descent.
    pub descender: FWord,
    /// Typographic line gap. Negative LineGap values are treated as
    /// zero in some legacy platform implementations.
    pub line_gap: FWord,
    /// Maximum advance width value in 'hmtx' table.
    pub advance_width_max: UfWord,
    /// Minimum left sidebearing value in 'hmtx' table for glyphs with
    /// contours (empty glyphs should be ignored).
    pub min_left_side_bearing: FWord,
    /// Minimum right sidebearing value; calculated as min(aw - (lsb +
    /// xMax - xMin)) for glyphs with contours (empty glyphs should be ignored).
    pub min_right_side_bearing: FWord,
    /// Max(lsb + (xMax-xMin))
    pub x_max_extent: FWord,
    /// Used to calculate the slope of the cursor (rise/run); 1 for
    /// vertical caret, 0 for horizontal.
    pub caret_slope_rise: i16,
    /// 0 for vertical caret, 1 for horizontal.
    pub caret_slope_run: i16,
    /// The amount by which a slanted highlight on a glyph needs to be
    /// shifted to produce the best appearance. Set to 0 for
    /// non-slanted fonts
    pub caret_offset: i16,
    /// Number of LongMetric entries in 'hmtx'/'vmtx' table
    pub number_of_long_metrics: u16,
}

impl Hhea {
    /// Construct a new `Hhea`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ascender: FWord,
        descender: FWord,
        line_gap: FWord,
        advance_width_max: UfWord,
        min_left_side_bearing: FWord,
        min_right_side_bearing: FWord,
        x_max_extent: FWord,
        caret_slope_rise: i16,
        caret_slope_run: i16,
        caret_offset: i16,
        number_of_long_metrics: u16,
    ) -> Self {
        Self {
            ascender,
            descender,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            number_of_long_metrics,
        }
    }
}

impl FontWrite for Hhea {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (MajorMinor::VERSION_1_0 as MajorMinor).write_into(writer);
        self.ascender.write_into(writer);
        self.descender.write_into(writer);
        self.line_gap.write_into(writer);
        self.advance_width_max.write_into(writer);
        self.min_left_side_bearing.write_into(writer);
        self.min_right_side_bearing.write_into(writer);
        self.x_max_extent.write_into(writer);
        self.caret_slope_rise.write_into(writer);
        self.caret_slope_run.write_into(writer);
        self.caret_offset.write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        self.number_of_long_metrics.write_into(writer);
    }
    fn name(&self) -> &'static str {
        "Hhea"
    }
}

impl Validate for Hhea {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl TopLevelTable for Hhea {
    const TAG: Tag = Tag::new(b"hhea");
}

impl<'a> FromObjRef<read_fonts::tables::hhea::Hhea<'a>> for Hhea {
    fn from_obj_ref(obj: &read_fonts::tables::hhea::Hhea<'a>, _: FontData) -> Self {
        Hhea {
            ascender: obj.ascender(),
            descender: obj.descender(),
            line_gap: obj.line_gap(),
            advance_width_max: obj.advance_width_max(),
            min_left_side_bearing: obj.min_left_side_bearing(),
            min_right_side_bearing: obj.min_right_side_bearing(),
            x_max_extent: obj.x_max_extent(),
            caret_slope_rise: obj.caret_slope_rise(),
            caret_slope_run: obj.caret_slope_run(),
            caret_offset: obj.caret_offset(),
            number_of_long_metrics: obj.number_of_long_metrics(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::hhea::Hhea<'a>> for Hhea {}

impl<'a> FontRead<'a> for Hhea {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::hhea::Hhea as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

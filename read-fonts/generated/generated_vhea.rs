// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [vhea](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea) Vertical Header Table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct VheaMarker {}

impl VheaMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn ascender_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + FWord::RAW_BYTE_LEN
    }
    fn descender_byte_range(&self) -> Range<usize> {
        let start = self.ascender_byte_range().end;
        start..start + FWord::RAW_BYTE_LEN
    }
    fn line_gap_byte_range(&self) -> Range<usize> {
        let start = self.descender_byte_range().end;
        start..start + FWord::RAW_BYTE_LEN
    }
    fn advance_height_max_byte_range(&self) -> Range<usize> {
        let start = self.line_gap_byte_range().end;
        start..start + UfWord::RAW_BYTE_LEN
    }
    fn min_top_side_bearing_byte_range(&self) -> Range<usize> {
        let start = self.advance_height_max_byte_range().end;
        start..start + FWord::RAW_BYTE_LEN
    }
    fn min_bottom_side_bearing_byte_range(&self) -> Range<usize> {
        let start = self.min_top_side_bearing_byte_range().end;
        start..start + FWord::RAW_BYTE_LEN
    }
    fn y_max_extent_byte_range(&self) -> Range<usize> {
        let start = self.min_bottom_side_bearing_byte_range().end;
        start..start + FWord::RAW_BYTE_LEN
    }
    fn caret_slope_rise_byte_range(&self) -> Range<usize> {
        let start = self.y_max_extent_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn caret_slope_run_byte_range(&self) -> Range<usize> {
        let start = self.caret_slope_rise_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn caret_offset_byte_range(&self) -> Range<usize> {
        let start = self.caret_slope_run_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn reserved1_byte_range(&self) -> Range<usize> {
        let start = self.caret_offset_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn reserved2_byte_range(&self) -> Range<usize> {
        let start = self.reserved1_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn reserved3_byte_range(&self) -> Range<usize> {
        let start = self.reserved2_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn reserved4_byte_range(&self) -> Range<usize> {
        let start = self.reserved3_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn metric_data_format_byte_range(&self) -> Range<usize> {
        let start = self.reserved4_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn number_of_long_ver_metrics_byte_range(&self) -> Range<usize> {
        let start = self.metric_data_format_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
}

impl TopLevelTable for Vhea<'_> {
    /// `vhea`
    const TAG: Tag = Tag::new(b"vhea");
}

impl<'a> FontRead<'a> for Vhea<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<FWord>();
        cursor.advance::<FWord>();
        cursor.advance::<FWord>();
        cursor.advance::<UfWord>();
        cursor.advance::<FWord>();
        cursor.advance::<FWord>();
        cursor.advance::<FWord>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<u16>();
        cursor.finish(VheaMarker {})
    }
}

/// The [vhea](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea) Vertical Header Table
pub type Vhea<'a> = TableRef<'a, VheaMarker>;

impl<'a> Vhea<'a> {
    /// The major/minor version (1, 0)
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Typographic ascent.
    pub fn ascender(&self) -> FWord {
        let range = self.shape.ascender_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Typographic descent.
    pub fn descender(&self) -> FWord {
        let range = self.shape.descender_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Typographic line gap. Negative LineGap values are treated as
    /// zero in some legacy platform implementations.
    pub fn line_gap(&self) -> FWord {
        let range = self.shape.line_gap_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum advance height value in 'vmtx' table.
    pub fn advance_height_max(&self) -> UfWord {
        let range = self.shape.advance_height_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum top sidebearing value in 'vmtx' table for glyphs with
    /// contours (empty glyphs should be ignored).
    pub fn min_top_side_bearing(&self) -> FWord {
        let range = self.shape.min_top_side_bearing_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum bottom sidebearing value
    pub fn min_bottom_side_bearing(&self) -> FWord {
        let range = self.shape.min_bottom_side_bearing_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Defined as max( tsb + (yMax-yMin)).
    pub fn y_max_extent(&self) -> FWord {
        let range = self.shape.y_max_extent_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Used to calculate the slope of the cursor (rise/run); 1 for
    /// vertical caret, 0 for horizontal.
    pub fn caret_slope_rise(&self) -> i16 {
        let range = self.shape.caret_slope_rise_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// 0 for vertical caret, 1 for horizontal.
    pub fn caret_slope_run(&self) -> i16 {
        let range = self.shape.caret_slope_run_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The amount by which a slanted highlight on a glyph needs to be
    /// shifted to produce the best appearance. Set to 0 for
    /// non-slanted fonts
    pub fn caret_offset(&self) -> i16 {
        let range = self.shape.caret_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// 0 for current format.
    pub fn metric_data_format(&self) -> i16 {
        let range = self.shape.metric_data_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of LongMetric entries in 'hmtx'/'vmtx' table
    pub fn number_of_long_ver_metrics(&self) -> u16 {
        let range = self.shape.number_of_long_ver_metrics_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Vhea<'a> {
    fn type_name(&self) -> &str {
        "Vhea"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("ascender", self.ascender())),
            2usize => Some(Field::new("descender", self.descender())),
            3usize => Some(Field::new("line_gap", self.line_gap())),
            4usize => Some(Field::new("advance_height_max", self.advance_height_max())),
            5usize => Some(Field::new(
                "min_top_side_bearing",
                self.min_top_side_bearing(),
            )),
            6usize => Some(Field::new(
                "min_bottom_side_bearing",
                self.min_bottom_side_bearing(),
            )),
            7usize => Some(Field::new("y_max_extent", self.y_max_extent())),
            8usize => Some(Field::new("caret_slope_rise", self.caret_slope_rise())),
            9usize => Some(Field::new("caret_slope_run", self.caret_slope_run())),
            10usize => Some(Field::new("caret_offset", self.caret_offset())),
            11usize => Some(Field::new("metric_data_format", self.metric_data_format())),
            12usize => Some(Field::new(
                "number_of_long_ver_metrics",
                self.number_of_long_ver_metrics(),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Vhea<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

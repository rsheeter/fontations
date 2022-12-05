// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [`maxp`](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct MaxpMarker {
    max_points_byte_start: Option<usize>,
    max_contours_byte_start: Option<usize>,
    max_composite_points_byte_start: Option<usize>,
    max_composite_contours_byte_start: Option<usize>,
    max_zones_byte_start: Option<usize>,
    max_twilight_points_byte_start: Option<usize>,
    max_storage_byte_start: Option<usize>,
    max_function_defs_byte_start: Option<usize>,
    max_instruction_defs_byte_start: Option<usize>,
    max_stack_elements_byte_start: Option<usize>,
    max_size_of_instructions_byte_start: Option<usize>,
    max_component_elements_byte_start: Option<usize>,
    max_component_depth_byte_start: Option<usize>,
}

impl MaxpMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + Version16Dot16::RAW_BYTE_LEN
    }
    fn num_glyphs_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn max_points_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_points_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_contours_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_contours_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_composite_points_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_composite_points_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_composite_contours_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_composite_contours_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_zones_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_zones_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_twilight_points_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_twilight_points_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_storage_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_storage_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_function_defs_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_function_defs_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_instruction_defs_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_instruction_defs_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_stack_elements_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_stack_elements_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_size_of_instructions_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_size_of_instructions_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_component_elements_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_component_elements_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
    fn max_component_depth_byte_range(&self) -> Option<Range<usize>> {
        let start = self.max_component_depth_byte_start?;
        Some(start..start + u16::RAW_BYTE_LEN)
    }
}

impl TopLevelTable for Maxp<'_> {
    /// `maxp`
    const TAG: Tag = Tag::new(b"maxp");
}

impl<'a> FontRead<'a> for Maxp<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: Version16Dot16 = cursor.read()?;
        cursor.advance::<u16>();
        let max_points_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_contours_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_composite_points_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_composite_contours_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_zones_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_twilight_points_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_storage_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_function_defs_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_instruction_defs_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_stack_elements_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_size_of_instructions_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_component_elements_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        let max_component_depth_byte_start = version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(Version16Dot16::VERSION_1_0)
            .then(|| cursor.advance::<u16>());
        cursor.finish(MaxpMarker {
            max_points_byte_start,
            max_contours_byte_start,
            max_composite_points_byte_start,
            max_composite_contours_byte_start,
            max_zones_byte_start,
            max_twilight_points_byte_start,
            max_storage_byte_start,
            max_function_defs_byte_start,
            max_instruction_defs_byte_start,
            max_stack_elements_byte_start,
            max_size_of_instructions_byte_start,
            max_component_elements_byte_start,
            max_component_depth_byte_start,
        })
    }
}

/// [`maxp`](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp)
pub type Maxp<'a> = TableRef<'a, MaxpMarker>;

impl<'a> Maxp<'a> {
    /// The version: 0x00005000 for version 0.5, 0x00010000 for version 1.0.
    pub fn version(&self) -> Version16Dot16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of glyphs in the font.
    pub fn num_glyphs(&self) -> u16 {
        let range = self.shape.num_glyphs_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum points in a non-composite glyph.
    pub fn max_points(&self) -> Option<u16> {
        let range = self.shape.max_points_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum contours in a non-composite glyph.
    pub fn max_contours(&self) -> Option<u16> {
        let range = self.shape.max_contours_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum points in a composite glyph.
    pub fn max_composite_points(&self) -> Option<u16> {
        let range = self.shape.max_composite_points_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum contours in a composite glyph.
    pub fn max_composite_contours(&self) -> Option<u16> {
        let range = self.shape.max_composite_contours_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// 1 if instructions do not use the twilight zone (Z0), or 2 if
    /// instructions do use Z0; should be set to 2 in most cases.
    pub fn max_zones(&self) -> Option<u16> {
        let range = self.shape.max_zones_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum points used in Z0.
    pub fn max_twilight_points(&self) -> Option<u16> {
        let range = self.shape.max_twilight_points_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Number of Storage Area locations.
    pub fn max_storage(&self) -> Option<u16> {
        let range = self.shape.max_storage_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Number of FDEFs, equal to the highest function number + 1.
    pub fn max_function_defs(&self) -> Option<u16> {
        let range = self.shape.max_function_defs_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Number of IDEFs.
    pub fn max_instruction_defs(&self) -> Option<u16> {
        let range = self.shape.max_instruction_defs_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum stack depth across Font Program ('fpgm' table), CVT
    /// Program ('prep' table) and all glyph instructions (in the
    /// 'glyf' table).
    pub fn max_stack_elements(&self) -> Option<u16> {
        let range = self.shape.max_stack_elements_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum byte count for glyph instructions.
    pub fn max_size_of_instructions(&self) -> Option<u16> {
        let range = self.shape.max_size_of_instructions_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum number of components referenced at “top level” for
    /// any composite glyph.
    pub fn max_component_elements(&self) -> Option<u16> {
        let range = self.shape.max_component_elements_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Maximum levels of recursion; 1 for simple components.
    pub fn max_component_depth(&self) -> Option<u16> {
        let range = self.shape.max_component_depth_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Maxp<'a> {
    fn type_name(&self) -> &str {
        "Maxp"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("num_glyphs", self.num_glyphs())),
            2usize if version.compatible(Version16Dot16::VERSION_1_0) => {
                Some(Field::new("max_points", self.max_points().unwrap()))
            }
            3usize if version.compatible(Version16Dot16::VERSION_1_0) => {
                Some(Field::new("max_contours", self.max_contours().unwrap()))
            }
            4usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_composite_points",
                self.max_composite_points().unwrap(),
            )),
            5usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_composite_contours",
                self.max_composite_contours().unwrap(),
            )),
            6usize if version.compatible(Version16Dot16::VERSION_1_0) => {
                Some(Field::new("max_zones", self.max_zones().unwrap()))
            }
            7usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_twilight_points",
                self.max_twilight_points().unwrap(),
            )),
            8usize if version.compatible(Version16Dot16::VERSION_1_0) => {
                Some(Field::new("max_storage", self.max_storage().unwrap()))
            }
            9usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_function_defs",
                self.max_function_defs().unwrap(),
            )),
            10usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_instruction_defs",
                self.max_instruction_defs().unwrap(),
            )),
            11usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_stack_elements",
                self.max_stack_elements().unwrap(),
            )),
            12usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_size_of_instructions",
                self.max_size_of_instructions().unwrap(),
            )),
            13usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_component_elements",
                self.max_component_elements().unwrap(),
            )),
            14usize if version.compatible(Version16Dot16::VERSION_1_0) => Some(Field::new(
                "max_component_depth",
                self.max_component_depth().unwrap(),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Maxp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

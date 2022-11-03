// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [CPAL (Color Palette Table)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-table-header) table
#[derive(Clone, Debug, Default)]
pub struct Cpal {
    /// Number of palette entries in each palette.
    pub num_palette_entries: u16,
    /// Number of palettes in the table.
    pub num_palettes: u16,
    /// Total number of color records, combined for all palettes.
    pub num_color_records: u16,
    /// Offset from the beginning of CPAL table to the first
    /// ColorRecord.
    pub color_records_array_offset: NullableOffsetMarker<Vec<ColorRecord>, WIDTH_32>,
    /// Index of each palette’s first color record in the combined
    /// color record array.
    pub color_record_indices: Vec<u16>,
    /// Offset from the beginning of CPAL table to the [Palette Types Array][].
    ///
    /// This is an array of 32-bit flag fields that describe properties of each palette.
    ///
    /// [Palette Types Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-type-array
    pub palette_types_array_offset: NullableOffsetMarker<Vec<u32>, WIDTH_32>,
    /// Offset from the beginning of CPAL table to the [Palette Labels Array][].
    ///
    /// This is an array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with  each palette.
    /// Use 0xFFFF if no name ID is provided for a palette.
    ///
    /// [Palette Labels Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-labels-array
    pub palette_labels_array_offset: NullableOffsetMarker<Vec<u16>, WIDTH_32>,
    /// Offset from the beginning of CPAL table to the [Palette Entry Labels Array][].
    ///
    /// This is an array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with  each palette
    /// entry, e.g. “Outline”, “Fill”. This set of palette entry labels applies
    /// to all palettes in the font. Use  0xFFFF if no name ID is provided for a
    /// palette entry.
    ///
    /// [Palette Entry Labels Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entry-label-array
    pub palette_entry_labels_array_offset: NullableOffsetMarker<Vec<u16>, WIDTH_32>,
}

impl FontWrite for Cpal {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = 0 as u16;
        version.write_into(writer);
        self.num_palette_entries.write_into(writer);
        self.num_palettes.write_into(writer);
        self.num_color_records.write_into(writer);
        self.color_records_array_offset.write_into(writer);
        self.color_record_indices.write_into(writer);
        version
            .compatible(1)
            .then(|| self.palette_types_array_offset.write_into(writer));
        version
            .compatible(1)
            .then(|| self.palette_labels_array_offset.write_into(writer));
        version
            .compatible(1)
            .then(|| self.palette_entry_labels_array_offset.write_into(writer));
    }
}

impl Validate for Cpal {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Cpal", |ctx| {
            ctx.in_field("color_records_array_offset", |ctx| {
                self.color_records_array_offset.validate_impl(ctx);
            });
            ctx.in_field("color_record_indices", |ctx| {
                if self.color_record_indices.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::cpal::Cpal<'a>> for Cpal {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::Cpal<'a>, _: FontData) -> Self {
        Cpal {
            num_palette_entries: obj.num_palette_entries(),
            num_palettes: obj.num_palettes(),
            num_color_records: obj.num_color_records(),
            color_records_array_offset: obj.color_records_array().into(),
            color_record_indices: obj.color_record_indices().iter().map(|x| x.get()).collect(),
            palette_types_array_offset: obj.palette_types_array().into(),
            palette_labels_array_offset: obj.palette_labels_array().into(),
            palette_entry_labels_array_offset: obj.palette_entry_labels_array().into(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::cpal::Cpal<'a>> for Cpal {}

impl<'a> FontRead<'a> for Cpal {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::cpal::Cpal as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [CPAL (Color Record)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entries-and-color-records) record
#[derive(Clone, Debug, Default)]
pub struct ColorRecord {
    /// Blue value (B0).
    pub blue: u8,
    /// Green value (B1).
    pub green: u8,
    ///     Red value (B2).
    pub red: u8,
    /// Alpha value (B3).
    pub alpha: u8,
}

impl FontWrite for ColorRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.blue.write_into(writer);
        self.green.write_into(writer);
        self.red.write_into(writer);
        self.alpha.write_into(writer);
    }
}

impl Validate for ColorRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::cpal::ColorRecord> for ColorRecord {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::ColorRecord, _: FontData) -> Self {
        ColorRecord {
            blue: obj.blue(),
            green: obj.green(),
            red: obj.red(),
            alpha: obj.alpha(),
        }
    }
}

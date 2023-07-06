// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [hmtx (Horizontal Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hmtx {
    /// Paired advance width/height and left/top side bearing values for each
    /// glyph. Records are indexed by glyph ID.
    pub h_metrics: Vec<LongMetric>,
    /// Leading (left/top) side bearings for glyph IDs greater than or equal to
    /// numberOfLongMetrics.
    pub left_side_bearings: Vec<i16>,
}

impl Hmtx {
    /// Construct a new `Hmtx`
    pub fn new(h_metrics: Vec<LongMetric>, left_side_bearings: Vec<i16>) -> Self {
        Self {
            h_metrics: h_metrics.into_iter().map(Into::into).collect(),
            left_side_bearings: left_side_bearings.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for Hmtx {
    fn write_into(&self, writer: &mut TableWriter) {
        self.h_metrics.write_into(writer);
        self.left_side_bearings.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Hmtx::TAG)
    }
}

impl Validate for Hmtx {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Hmtx", |ctx| {
            ctx.in_field("h_metrics", |ctx| {
                if self.h_metrics.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.h_metrics.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Hmtx {
    const TAG: Tag = Tag::new(b"hmtx");
}

impl<'a> FromObjRef<read_fonts::tables::hmtx::Hmtx<'a>> for Hmtx {
    fn from_obj_ref(obj: &read_fonts::tables::hmtx::Hmtx<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Hmtx {
            h_metrics: obj.h_metrics().to_owned_obj(offset_data),
            left_side_bearings: obj.left_side_bearings().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::hmtx::Hmtx<'a>> for Hmtx {}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LongMetric {
    /// Advance width/height, in font design units.
    pub advance: u16,
    /// Glyph leading (left/top) side bearing, in font design units.
    pub side_bearing: i16,
}

impl LongMetric {
    /// Construct a new `LongMetric`
    pub fn new(advance: u16, side_bearing: i16) -> Self {
        Self {
            advance,
            side_bearing,
        }
    }
}

impl FontWrite for LongMetric {
    fn write_into(&self, writer: &mut TableWriter) {
        self.advance.write_into(writer);
        self.side_bearing.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("LongMetric")
    }
}

impl Validate for LongMetric {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::hmtx::LongMetric> for LongMetric {
    fn from_obj_ref(obj: &read_fonts::tables::hmtx::LongMetric, _: FontData) -> Self {
        LongMetric {
            advance: obj.advance(),
            side_bearing: obj.side_bearing(),
        }
    }
}

// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [avar (Axis Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/avar) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct AvarMarker {
    axis_segment_maps_byte_len: usize,
}

impl AvarMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn _reserved_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn axis_count_byte_range(&self) -> Range<usize> {
        let start = self._reserved_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn axis_segment_maps_byte_range(&self) -> Range<usize> {
        let start = self.axis_count_byte_range().end;
        start..start + self.axis_segment_maps_byte_len
    }
}

impl TopLevelTable for Avar<'_> {
    /// `avar`
    const TAG: Tag = Tag::new(b"avar");
}

impl<'a> FontRead<'a> for Avar<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        let axis_segment_maps_byte_len = cursor.remaining_bytes();
        cursor.advance_by(axis_segment_maps_byte_len);
        cursor.finish(AvarMarker {
            axis_segment_maps_byte_len,
        })
    }
}

/// The [avar (Axis Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/avar) table
pub type Avar<'a> = TableRef<'a, AvarMarker>;

impl<'a> Avar<'a> {
    /// Major version number of the axis variations table — set to 1.
    /// Minor version number of the axis variations table — set to 0.
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of variation axes for this font. This must be the same number as axisCount in the 'fvar' table.
    pub fn axis_count(&self) -> u16 {
        let range = self.shape.axis_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The segment maps array — one segment map for each axis, in the order of axes specified in the 'fvar' table.
    pub fn axis_segment_maps(&self) -> VarLenArray<'a, SegmentMaps<'a>> {
        let range = self.shape.axis_segment_maps_byte_range();
        VarLenArray::read(self.data.split_off(range.start).unwrap()).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Avar<'a> {
    fn type_name(&self) -> &str {
        "Avar"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("axis_count", self.axis_count())),
            2usize => Some(Field::new(
                "axis_segment_maps",
                traversal::FieldType::var_array(
                    "SegmentMaps",
                    self.axis_segment_maps(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Avar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// [SegmentMaps](https://learn.microsoft.com/en-us/typography/opentype/spec/avar#table-formats) record
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SegmentMaps<'a> {
    /// The number of correspondence pairs for this axis.
    pub position_map_count: BigEndian<u16>,
    /// The array of axis value map records for this axis.
    pub axis_value_maps: &'a [AxisValueMap],
}

impl<'a> SegmentMaps<'a> {
    /// The number of correspondence pairs for this axis.
    pub fn position_map_count(&self) -> u16 {
        self.position_map_count.get()
    }

    /// The array of axis value map records for this axis.
    pub fn axis_value_maps(&self) -> &'a [AxisValueMap] {
        self.axis_value_maps
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for SegmentMaps<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "SegmentMaps",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("position_map_count", self.position_map_count())),
                1usize => Some(Field::new(
                    "axis_value_maps",
                    traversal::FieldType::array_of_records(
                        stringify!(AxisValueMap),
                        self.axis_value_maps(),
                        _data,
                    ),
                )),
                _ => None,
            }),
            data,
        }
    }
}

/// [AxisValueMap](https://learn.microsoft.com/en-us/typography/opentype/spec/avar#table-formats) record
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct AxisValueMap {
    /// A normalized coordinate value obtained using default normalization.
    pub from_coordinate: BigEndian<F2Dot14>,
    /// The modified, normalized coordinate value.
    pub to_coordinate: BigEndian<F2Dot14>,
}

impl AxisValueMap {
    /// A normalized coordinate value obtained using default normalization.
    pub fn from_coordinate(&self) -> F2Dot14 {
        self.from_coordinate.get()
    }

    /// The modified, normalized coordinate value.
    pub fn to_coordinate(&self) -> F2Dot14 {
        self.to_coordinate.get()
    }
}

impl FixedSize for AxisValueMap {
    const RAW_BYTE_LEN: usize = F2Dot14::RAW_BYTE_LEN + F2Dot14::RAW_BYTE_LEN;
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for AxisValueMap {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "AxisValueMap",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("from_coordinate", self.from_coordinate())),
                1usize => Some(Field::new("to_coordinate", self.to_coordinate())),
                _ => None,
            }),
            data,
        }
    }
}

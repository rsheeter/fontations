// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [fvar (Font Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/fvar) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct FvarMarker {}

impl FvarMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn axis_instance_arrays_offset_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn _reserved_byte_range(&self) -> Range<usize> {
        let start = self.axis_instance_arrays_offset_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn axis_count_byte_range(&self) -> Range<usize> {
        let start = self._reserved_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn axis_size_byte_range(&self) -> Range<usize> {
        let start = self.axis_count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn instance_count_byte_range(&self) -> Range<usize> {
        let start = self.axis_size_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn instance_size_byte_range(&self) -> Range<usize> {
        let start = self.instance_count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
}

impl TopLevelTable for Fvar<'_> {
    /// `fvar`
    const TAG: Tag = Tag::new(b"fvar");
}

impl<'a> FontRead<'a> for Fvar<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<Offset16>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.finish(FvarMarker {})
    }
}

/// The [fvar (Font Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/fvar) table
pub type Fvar<'a> = TableRef<'a, FvarMarker>;

impl<'a> Fvar<'a> {
    /// Major version number of the font variations table — set to 1.
    /// Minor version number of the font variations table — set to 0.
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset in bytes from the beginning of the table to the start of the VariationAxisRecord array. The
    /// InstanceRecord array directly follows.
    pub fn axis_instance_arrays_offset(&self) -> Offset16 {
        let range = self.shape.axis_instance_arrays_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`axis_instance_arrays_offset`][Self::axis_instance_arrays_offset].
    pub fn axis_instance_arrays(&self) -> Result<AxisInstanceArrays<'a>, ReadError> {
        let data = self.data;
        let args = (
            self.axis_count(),
            self.instance_count(),
            self.instance_size(),
        );
        self.axis_instance_arrays_offset()
            .resolve_with_args(data, &args)
    }

    /// The number of variation axes in the font (the number of records in the axes array).
    pub fn axis_count(&self) -> u16 {
        let range = self.shape.axis_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The size in bytes of each VariationAxisRecord — set to 20 (0x0014) for this version.
    pub fn axis_size(&self) -> u16 {
        let range = self.shape.axis_size_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of named instances defined in the font (the number of records in the instances array).
    pub fn instance_count(&self) -> u16 {
        let range = self.shape.instance_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The size in bytes of each InstanceRecord — set to either axisCount * sizeof(Fixed) + 4, or to axisCount * sizeof(Fixed) + 6.
    pub fn instance_size(&self) -> u16 {
        let range = self.shape.instance_size_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Fvar<'a> {
    fn type_name(&self) -> &str {
        "Fvar"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new(
                "axis_instance_arrays_offset",
                FieldType::offset(
                    self.axis_instance_arrays_offset(),
                    self.axis_instance_arrays(),
                ),
            )),
            2usize => Some(Field::new("axis_count", self.axis_count())),
            3usize => Some(Field::new("axis_size", self.axis_size())),
            4usize => Some(Field::new("instance_count", self.instance_count())),
            5usize => Some(Field::new("instance_size", self.instance_size())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Fvar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Shim table to handle combined axis and instance arrays.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct AxisInstanceArraysMarker {
    axis_count: u16,
    instance_size: u16,
    axes_byte_len: usize,
    instances_byte_len: usize,
}

impl AxisInstanceArraysMarker {
    fn axes_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + self.axes_byte_len
    }
    fn instances_byte_range(&self) -> Range<usize> {
        let start = self.axes_byte_range().end;
        start..start + self.instances_byte_len
    }
}

impl ReadArgs for AxisInstanceArrays<'_> {
    type Args = (u16, u16, u16);
}

impl<'a> FontReadWithArgs<'a> for AxisInstanceArrays<'a> {
    fn read_with_args(data: FontData<'a>, args: &(u16, u16, u16)) -> Result<Self, ReadError> {
        let (axis_count, instance_count, instance_size) = *args;
        let mut cursor = data.cursor();
        let axes_byte_len = axis_count as usize * VariationAxisRecord::RAW_BYTE_LEN;
        cursor.advance_by(axes_byte_len);
        let instances_byte_len = instance_count as usize
            * <InstanceRecord as ComputeSize>::compute_size(&(axis_count, instance_size));
        cursor.advance_by(instances_byte_len);
        cursor.finish(AxisInstanceArraysMarker {
            axis_count,
            instance_size,
            axes_byte_len,
            instances_byte_len,
        })
    }
}

impl<'a> AxisInstanceArrays<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(
        data: FontData<'a>,
        axis_count: u16,
        instance_count: u16,
        instance_size: u16,
    ) -> Result<Self, ReadError> {
        let args = (axis_count, instance_count, instance_size);
        Self::read_with_args(data, &args)
    }
}

/// Shim table to handle combined axis and instance arrays.
pub type AxisInstanceArrays<'a> = TableRef<'a, AxisInstanceArraysMarker>;

impl<'a> AxisInstanceArrays<'a> {
    /// Variation axis record array.
    pub fn axes(&self) -> &'a [VariationAxisRecord] {
        let range = self.shape.axes_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Instance record array.
    pub fn instances(&self) -> ComputedArray<'a, InstanceRecord<'a>> {
        let range = self.shape.instances_byte_range();
        self.data
            .read_with_args(range, &(self.axis_count(), self.instance_size()))
            .unwrap()
    }

    pub(crate) fn axis_count(&self) -> u16 {
        self.shape.axis_count
    }

    pub(crate) fn instance_size(&self) -> u16 {
        self.shape.instance_size
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for AxisInstanceArrays<'a> {
    fn type_name(&self) -> &str {
        "AxisInstanceArrays"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new(
                "axes",
                traversal::FieldType::array_of_records(
                    stringify!(VariationAxisRecord),
                    self.axes(),
                    self.offset_data(),
                ),
            )),
            1usize => Some(Field::new(
                "instances",
                traversal::FieldType::computed_array(
                    "InstanceRecord",
                    self.instances(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for AxisInstanceArrays<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [VariationAxisRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#variationaxisrecord)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[repr(packed)]
pub struct VariationAxisRecord {
    /// Tag identifying the design variation for the axis.
    pub axis_tag: BigEndian<Tag>,
    /// The minimum coordinate value for the axis.
    pub min_value: BigEndian<Fixed>,
    /// The default coordinate value for the axis.
    pub default_value: BigEndian<Fixed>,
    /// The maximum coordinate value for the axis.
    pub max_value: BigEndian<Fixed>,
    /// Axis qualifiers — see details below.
    pub flags: BigEndian<u16>,
    /// The name ID for entries in the 'name' table that provide a display name for this axis.
    pub axis_name_id: BigEndian<NameId>,
}

impl VariationAxisRecord {
    /// Tag identifying the design variation for the axis.
    pub fn axis_tag(&self) -> Tag {
        self.axis_tag.get()
    }

    /// The minimum coordinate value for the axis.
    pub fn min_value(&self) -> Fixed {
        self.min_value.get()
    }

    /// The default coordinate value for the axis.
    pub fn default_value(&self) -> Fixed {
        self.default_value.get()
    }

    /// The maximum coordinate value for the axis.
    pub fn max_value(&self) -> Fixed {
        self.max_value.get()
    }

    /// Axis qualifiers — see details below.
    pub fn flags(&self) -> u16 {
        self.flags.get()
    }

    /// The name ID for entries in the 'name' table that provide a display name for this axis.
    pub fn axis_name_id(&self) -> NameId {
        self.axis_name_id.get()
    }
}

impl FixedSize for VariationAxisRecord {
    const RAW_BYTE_LEN: usize = Tag::RAW_BYTE_LEN
        + Fixed::RAW_BYTE_LEN
        + Fixed::RAW_BYTE_LEN
        + Fixed::RAW_BYTE_LEN
        + u16::RAW_BYTE_LEN
        + NameId::RAW_BYTE_LEN;
}

impl sealed::Sealed for VariationAxisRecord {}

/// SAFETY: see the [`FromBytes`] trait documentation.
unsafe impl FromBytes for VariationAxisRecord {
    fn this_trait_should_only_be_implemented_in_generated_code() {}
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for VariationAxisRecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "VariationAxisRecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("axis_tag", self.axis_tag())),
                1usize => Some(Field::new("min_value", self.min_value())),
                2usize => Some(Field::new("default_value", self.default_value())),
                3usize => Some(Field::new("max_value", self.max_value())),
                4usize => Some(Field::new("flags", self.flags())),
                5usize => Some(Field::new("axis_name_id", self.axis_name_id())),
                _ => None,
            }),
            data,
        }
    }
}
